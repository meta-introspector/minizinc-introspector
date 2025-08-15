#include <minizinc/file_utils.hh>
#include <minizinc/parser.hh>
#include <minizinc/astexception.hh>
#include <minizinc/model.hh>

#include <fstream>
#include <iostream>
#include <sstream>
#include <vector>
#include <map>
#include <unordered_set>

namespace MiniZinc {

// This function will parse a MiniZinc model from a string buffer.
// It will NOT perform any file I/O.
void parse_from_buffer(Env& env, Model*& model, const std::string& modelString,
                       const std::string& modelStringName, const std::vector<std::string>& includePaths,
                       std::unordered_set<std::string> globalInc, bool isFlatZinc, bool ignoreStdlib,
                       bool parseDocComments, bool verbose, std::ostream& err) {

    std::cerr << "Entering parse_from_buffer function..." << std::endl;
    std::cerr << "  Arguments:" << std::endl;
    std::cerr << "    modelString: \"" << modelString << "\"" << std::endl;
    std::cerr << "    modelStringName: \"" << modelStringName << "\"" << std::endl;
    std::cerr << "    verbose: " << (verbose ? "true" : "false") << std::endl;

    std::vector<ParseWorkItem> files;
    std::map<std::string, Model*> seenModels;

    // In this function, we always parse from a string, so isModelString is always true
    // and we don't need to worry about file loading.
    GCLock lock;
    model->setFilename(modelStringName);
    files.emplace_back(model, nullptr, modelString, modelStringName, false, true); // isModelString = true

    // DEBUG: Check ParseWorkItem after emplace_back
    std::cerr << "DEBUG_PARSE_FROM_BUFFER: files.back().isModelString (after emplace_back): " << (files.back().isModelString ? "true" : "false") << std::endl;

    auto include_file = [&](const std::string& libname, bool builtin) {
        GCLock lock;
        auto* lib = new Model;
        std::string fullname;
        // In parse_from_buffer, we assume includes are also from strings or already handled.
        // For now, we'll just use the libname as fullname.
        fullname = libname;

        lib->setFilename(fullname);
        files.emplace_back(lib, nullptr, "./", fullname, builtin, true); // isModelString = true
        seenModels.insert(std::pair<std::string, Model*>(fullname, lib));
        Location libloc(ASTString(model->filename()), 0, 0, 0, 0);
        auto* libinc = new IncludeI(libloc, ASTString(libname));
        libinc->m(lib, true);
        model->addItem(libinc);
    };

    if (!ignoreStdlib) {
        include_file("solver_redefinitions.mzn", false);
        include_file("stdlib.mzn", true);  // Added last, so it is processed first
    }

    while (!files.empty()) {
        GCLock lock;
        ParseWorkItem& np = files.back();
        // DEBUG: Check np.isModelString immediately after getting reference
        std::cerr << "DEBUG_PARSE_FROM_BUFFER_WHILE: np.isModelString (after getting reference): " << (np.isModelString ? "true" : "false") << std::endl;

        std::string parentPath = np.dirName;
        Model* m_current = np.m;
        bool isModelString_np = np.isModelString; // Use a new variable to avoid confusion
        bool isSTDLib_np = np.isSTDLib;
        IncludeI* np_ii = np.ii;
        std::string f(np.fileName);

        // DEBUG: Check files.back().isModelString before pop_back
        std::cerr << "DEBUG_PARSE_FROM_BUFFER_WHILE: files.back().isModelString (before pop_back): " << (files.back().isModelString ? "true" : "false") << std::endl;
        files.pop_back();

        std::string s;
        std::string fullname;
        bool isFzn;

        // In parse_from_buffer, we always treat content as string, so this branch is always taken
        std::cerr << "DEBUG_PARSE_FROM_BUFFER_WHILE: Inside isModelString block" << std::endl;
        isFzn = false;
        fullname = f;
        s = np.modelString; // Get model string from ParseWorkItem
        std::cerr << "DEBUG_PARSE_FROM_BUFFER_WHILE: fullname (in isModelString block): \"" << fullname << "\"" << std::endl;

        ParserState pp(fullname, s, err, includePaths, files, seenModels, m_current, false, isFzn, isSTDLib_np,
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
    std::cerr << "Exiting parse_from_buffer function." << std::endl;
}

} // namespace MiniZinc
