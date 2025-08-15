#include <minizinc/file_utils.hh>
#include <minizinc/parser.hh>
#include <minizinc/astexception.hh>
#include <minizinc/model.hh>

#include <fstream>
#include <iostream>
#include <sstream>
#include <vector>
#include <map>

namespace MiniZinc {

// Forward declaration for get_file_contents (defined in parser.cpp)
extern std::string get_file_contents(std::ifstream& in);

void handleFileParsing(Env& env, Model* m, ParseWorkItem& np, const std::vector<std::string>& includePaths, std::map<std::string, Model*>& seenModels, std::vector<ParseWorkItem>& files, std::string& s, std::string& fullname, std::string& basename, bool& isFzn, bool verbose, std::ostream& err) {
    std::string f(np.fileName);

    // DEBUG: Inside !isModelString block
    std::cerr << "DEBUG_WHILE: Inside !isModelString block (refactored)" << std::endl;
    std::cerr << "DEBUG_WHILE: f (before file operations): \"" << f << "\"" << std::endl;

    for (Model* p = m->parent(); p != nullptr; p = p->parent()) {
        if (p->filename() == f) {
            std::vector<ASTString> cycle;
            for (Model* pe = m; pe != nullptr; pe = pe->parent()) {
                cycle.push_back(pe->filename());
            }
            throw CyclicIncludeError(cycle);
        }
    }
    ifstream file;
    if (FileUtils::is_absolute(f)) {
        fullname = f;
        basename = FileUtils::base_name(fullname);
        if (FileUtils::file_exists(fullname)) {
            file.open(FILE_PATH(fullname), std::ios::binary);
        }
    }
    // DEBUG: After file open attempt
    std::cerr << "DEBUG_WHILE: fullname (after path resolution): \"" << fullname << "\"" << std::endl;
    std::cerr << "DEBUG_WHILE: FileUtils::file_exists(fullname): " << (FileUtils::file_exists(fullname) ? "true" : "false") << std::endl;
    std::cerr << "DEBUG_WHILE: file.is_open(): " << (file.is_open() ? "true" : "false") << std::endl;

    if (file.is_open() &&
        FileUtils::file_path(FileUtils::dir_name(fullname)) != FileUtils::file_path(FileUtils::working_directory()) &&
        FileUtils::file_exists(FileUtils::working_directory() + "/" + basename)) {
        std::ostringstream w;
        w << "file \"" << basename
          << "\" included from library, but also exists in current working directory.";
        env.envi().addWarning(w.str());
    } else if (file.is_open() && globalInc.find(basename) != globalInc.end() &&
               fullname.find(includePaths.back()) == std::string::npos) {
        std::ostringstream w;
        w << "included file \"" << basename
          << "\" overrides a global constraint file from the standard library. This is "
             "deprecated. For a solver-specific redefinition of a global constraint, override "
             "\"fzn_<global>.mzn\" instead.";
        env.envi().addWarning(w.str());
    }
    for (const auto& includePath : includePaths) {
        std::string deprecatedName = includePath + "/" + basename + ".deprecated.mzn";
        if (FileUtils::file_exists(deprecatedName)) {
            string deprecatedFullPath = FileUtils::file_path(deprecatedName);
            string deprecatedDirName = FileUtils::dir_name(deprecatedFullPath);
            auto* includedModel = new Model;
            includedModel->setFilename(deprecatedName);
            files.emplace_back(includedModel, nullptr, "", deprecatedName, np.isSTDLib, false);
            seenModels.insert(pair<string, Model*>(deprecatedName, includedModel));
            Location loc(ASTString(deprecatedName), 0, 0, 0, 0);
            auto* inc = new IncludeI(loc, includedModel->filename());
            inc->m(includedModel, true);
            m->addItem(inc);
            files.emplace_back(includedModel, inc, deprecatedDirName, deprecatedFullPath, np.isSTDLib,
                               false);
        }
    }
    if (!file.is_open()) {
        if (np.ii != nullptr) {
            throw IncludeError(env.envi(), np.ii->loc(), "Cannot open file '" + f + "'.");
        }
        throw Error("Cannot open file '" + f + "'.");
    }
    std::cerr << "Processing file: " << f << std::endl;
    if (verbose) {
        std::cerr << "processing file '" << fullname << "'" << endl;
    }
    s = get_file_contents(file);

    if (m->filepath().empty()) {
        m->setFilepath(fullname);
    }
    isFzn = (fullname.compare(fullname.length() - 4, 4, ".fzn") == 0);
}

} // namespace MiniZinc
