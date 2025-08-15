/* -*- mode: C++; c-basic-offset: 2; indent-tabs-mode: nil -*- */

/*
 *  Main authors:
 *     Guido Tack <guido.tack@monash.edu>
 */

/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

/* This (main) file coordinates flattening and solving.
 * The corresponding modules are flexibly plugged in
 * as derived classes, prospectively from DLLs.
 * A flattening module should provide MinZinc::GetFlattener()
 * A solving module should provide an object of a class derived from SolverFactory.
 * Need to get more flexible for multi-pass & multi-solving stuff  TODO
 */

#include <minizinc/file_utils.hh>
#include <minizinc/flatten_internal.hh>
#include <minizinc/json_parser.hh>
#include <minizinc/parser.hh>
#include <minizinc/prettyprinter.hh>

#include <fstream>
#include <regex>

// New includes for refactoring
#include "parser_refactored/parser_utils.h"
#include "parser_refactored/parse_from_buffer.h"

using namespace std;

int mzn_yylex_init(void** scanner);
void mzn_yyset_extra(void* user_defined, void* yyscanner);
int mzn_yylex_destroy(void* scanner);

namespace MiniZinc {

// Moved from anonymous namespace
std::string get_file_contents(std::ifstream& in) {
  if (in) {
    std::string contents;
    in.seekg(0, std::ios::end);
    contents.resize(static_cast<unsigned int>(in.tellg()));
    in.seekg(0, std::ios::beg);
    // Warning assume editability of the string underlying storage
    in.read(const_cast<char*>(contents.data()), static_cast<long>(contents.size()));
    in.close();
    if (!contents.empty() && contents[0] == '@') {
      contents = MiniZinc::FileUtils::decode_base64(contents);
      MiniZinc::FileUtils::inflate_string(contents);
    }
    return (contents);
  }
  throw(errno);
}

std::string ParserState::canonicalFilename(const std::string& f) const {
  if (FileUtils::is_absolute(f) || std::string(filename).empty()) {
    return f;
  }
  for (const auto& ip : includePaths) {
    std::string fullname = FileUtils::file_path(ip + "/" + f);
    if (FileUtils::file_exists(fullname)) {
      return fullname;
    }
  }
  std::string parentPath = FileUtils::dir_name(filename);
  if (parentPath.empty()) {
    parentPath = ".";
  }
  std::string fullname = FileUtils::file_path(parentPath + "/" + f);
  if (FileUtils::file_exists(fullname)) {
    return fullname;
  }
  return f;
}

std::unordered_set<std::string> global_includes(const std::string& stdlib) {
  GCLock lock;
  // Check if globals.mzn file can be found
  if (!FileUtils::file_exists(stdlib + "/std/globals.mzn")) {
    // Otherwise act as if there are no bad files
    return {};
  }

  // Read globals file
  std::ifstream ifs(FileUtils::file_path(stdlib + "/std/globals.mzn"));
  std::string content((std::istreambuf_iterator<char>(ifs)), (std::istreambuf_iterator<char>()));

  // Regular expression to find include items (assumes no include statements in comments)
  std::regex include_item("include[[:space:]]+\"([^\"]+)\"", std::regex_constants::egrep);

  // Collect all files directly included in globals.mzn
  std::unordered_set<std::string> files;
  for (auto inc = std::sregex_token_iterator(content.begin(), content.end(), include_item, 1);
       inc != std::sregex_token_iterator(); ++inc) {
    files.emplace(inc->str());
  }
  return files;
}

void parse(Env& env, Model*& model, const vector<string>& filenames,
           const vector<string>& datafiles, const std::string& modelString,
           const std::string& modelStringName, const vector<string>& ip,
           std::unordered_set<std::string> globalInc, bool isFlatZinc, bool ignoreStdlib,
           bool parseDocComments, bool verbose, ostream& err) {
  // Instrumentation: Entry point
  std::cerr << "Entering parse function..." << std::endl;
  std::cerr << "  Arguments:" << std::endl;
  std::cerr << "    filenames: [";
  for (const auto& s : filenames) {
    std::cerr << "\"" << s << "\", ";
  }
  std::cerr << "]" << std::endl;
  std::cerr << "    datafiles: [";
  for (const auto& s : datafiles) {
    std::cerr << "\"" << s << "\", ";
  }
  std::cerr << "]" << std::endl;
  std::cerr << "    modelString: \"" << modelString << "\"" << std::endl;
  std::cerr << "    modelStringName: \"" << modelStringName << "\"" << std::endl;
  std::cerr << "    ip (includePaths): [";
  for (const auto& s : ip) {
    std::cerr << "\"" << s << "\", ";
  }
  std::cerr << "]" << std::endl;
  std::cerr << "    globalInc: [";
  for (const auto& s : globalInc) {
    std::cerr << "\"" << s << "\", ";
  }
  std::cerr << "]" << std::endl;
  std::cerr << "    isFlatZinc: " << (isFlatZinc ? "true" : "false") << std::endl;
  std::cerr << "    ignoreStdlib: " << (ignoreStdlib ? "true" : "false") << std::endl;
  std::cerr << "    parseDocComments: " << (parseDocComments ? "true" : "false") << std::endl;
  std::cerr << "    verbose: " << (verbose ? "true" : "false") << std::endl;

  // DEBUG: Added prints for filename flow
  std::cerr << "DEBUG_PARSE: filenames.empty(): " << (filenames.empty() ? "true" : "false") << std::endl;
  std::cerr << "DEBUG_PARSE: modelString.empty(): " << (modelString.empty() ? "true" : "false") << std::endl;
  std::cerr << "DEBUG_PARSE: modelStringName: \"" << modelStringName << "\"" << std::endl;

  // Note: 'err' is is an ostream, not directly printable in this manner.
  vector<string> includePaths;
  for (const auto& i : ip) {
    includePaths.push_back(i);
  }

  // Refactored: Call parse_from_buffer if modelString is provided
  if (!modelString.empty()) {
    parse_from_buffer(env, model, modelString, modelStringName, includePaths, std::move(globalInc),
                      isFlatZinc, ignoreStdlib, parseDocComments, verbose, err);
  } else if (!filenames.empty()) {
    // Original file parsing logic (simplified for now)
    GCLock lock;
    auto rootFileName = FileUtils::file_path(filenames[0], FileUtils::working_directory());
    model->setFilename(rootFileName);
    // TODO: Implement proper file parsing logic here, potentially calling a new load_model_from_file
    // For now, this path is not fully refactored.
    std::cerr << "WARNING: File parsing path is not fully refactored yet." << std::endl;
  } else {
    throw Error("No model given.");
  }

  // Datafile processing remains for now
  for (const auto& f : datafiles) {
    GCLock lock;
    if (f.size() >= 6 && f.substr(f.size() - 5, string::npos) == ".json") {
      JSONParser jp(env.envi());
      jp.parse(model, f, true);
    } else if (f.size() >= 6 && f.substr(0, 6) == "json:/") {
      JSONParser jp(env.envi());
      jp.parseFromString(model, f.substr(6), true);
    } else {
      string s;
      if (f.size() > 5 && f.substr(0, 5) == "cmd:/") {
        s = f.substr(5);
      } else {
        std::ifstream file(FILE_PATH(f), std::ios::binary);
        if (!FileUtils::file_exists(f) || !file.is_open()) {
          throw Error("Cannot open data file '" + f + "'.");
        }
        if (verbose) {
          std::cerr << "processing data file '" << f << "'" << endl;
        }
        s = get_file_contents(file);
      }

      ParserState pp(f, s, err, includePaths, files, seenModels, model, true, false, false,
                     parseDocComments);
      mzn_yylex_init(&pp.yyscanner);
      mzn_yyset_extra(&pp, pp.yyscanner);
      mzn_yyparse(&pp);
      if (pp.yyscanner != nullptr) {
        mzn_yylex_destroy(pp.yyscanner);
      }
      if (pp.hadError) {
        throw MultipleErrors<SyntaxError>(pp.syntaxErrors);
      }
    }
  }
  // Instrumentation: Exit point
  std::cerr << "Exiting parse function." << std::endl;
}

Model* parse(Env& env, const vector<string>& filenames, const vector<string>& datafiles,
             const string& textModel, const string& textModelName,
             const vector<string>& includePaths, std::unordered_set<std::string> globalInc,
             bool isFlatZinc, bool ignoreStdlib,
             bool parseDocComments, bool verbose, ostream& err) {
  if (filenames.empty() && textModel.empty()) {
    throw Error("No model given.");
  }

  Model* model;
  {
    GCLock lock;
    model = new Model();
  }
  try {
    parse(env, model, filenames, datafiles, textModel, textModelName, includePaths,
          std::move(globalInc), isFlatZinc, ignoreStdlib, parseDocComments, verbose, err);
  } catch (Exception& e) {
    // Instrumentation: Log caught exception
    std::cerr << "Caught parsing exception: " << e.what() << std::endl;
    delete model;
    throw;
  }
  return model;
}

Model* parse_data(Env& env, Model* model, const vector<string>& datafiles,
                  const vector<string>& includePaths, bool isFlatZinc, bool ignoreStdlib,
                  bool parseDocComments, bool verbose, ostream& err) {
  vector<string> filenames;
  parse(env, model, filenames, datafiles, "", "", includePaths, {}, isFlatZinc, ignoreStdlib,
        parseDocComments, verbose, err);
  return model;
}

Model* parse_from_string(Env& env, const string& text, const string& filename,
                         const vector<string>& includePaths, bool isFlatZinc, bool ignoreStdlib,
                         bool parseDocComments, bool verbose, ostream& err) {
  vector<string> filenames;
  vector<string> datafiles;
  Model* model;
  {
    GCLock lock;
    model = new Model();
  }
  try {
    parse(env, model, filenames, datafiles, text, filename, includePaths, {}, isFlatZinc,
          ignoreStdlib, parseDocComments, verbose, err);
  } catch (Exception& /*e*/) {
    delete model;
    throw;
  }
  return model;
}

}  // namespace MiniZinc
