#include "minizinc_opaque_types.h"
#include <minizinc/model.hh>
#include <minizinc/parser.hh>
#include <minizinc/astexception.hh>
#include <iostream>
#include <fstream>
#include <cstdio>
#include <sstream>
#include <vector> // For std::vector
#include <unordered_set> // For std::unordered_set

// Include the new FFI declarations header
#include "minizinc_ffi_declarations_v2.h"

// Forward declarations for lexer/parser functions
    int mzn_yylex_init(void** scanner);
    void mzn_yyset_extra(void* user_defined, void* yyscanner);
    int mzn_yyparse(void*);
    int mzn_yylex_destroy(void* scanner);

extern "C" {

MiniZincModel* minizinc_parse_string_only(MiniZinc::MznSolver* solver_ptr, const char* model_str) {
    std::cerr << "[minizinc_parse_string_only] Starting parse process." << std::endl; std::cerr.flush();
    std::cerr << "[minizinc_parse_string_only] model_str: " << (model_str ? model_str : "(null)") << std::endl; std::cerr.flush();

    std::string model_s(model_str);
    std::string dummy_filename = "<string>"; // Use a dummy filename for ParserState

    try {
        MiniZinc::Env env; // Create an environment object
        std::cerr << "[minizinc_parse_string_only] MiniZinc::Env created." << std::endl; std::cerr.flush();

        MiniZinc::Model* model = new MiniZinc::Model();
        model->setFilename(dummy_filename); // Set filename for the model

        // Create a ParserState directly for string parsing
        // Arguments: filename, buffer, error_stream, includePaths, files, seenModels, model, isDatafile, isFlatZinc, isSTDLib, parseDocComments
        // For string parsing, filenames, datafiles, includePaths, globalInc, isFlatZinc, ignoreStdlib, parseDocComments can be default/empty/false
        std::vector<std::string> includePaths_vec; // Empty
        std::vector<MiniZinc::ParseWorkItem> files_vec; // Empty
        std::map<std::string, MiniZinc::Model*> seenModels_map; // Empty

        MiniZinc::ParserState pp(dummy_filename, model_s, std::cerr, includePaths_vec, files_vec, seenModels_map, model, false, false, false, false);

        mzn_yylex_init(&pp.yyscanner);
        mzn_yyset_extra(&pp, pp.yyscanner);
        mzn_yyparse(&pp);

        if (pp.yyscanner != nullptr) {
            mzn_yylex_destroy(pp.yyscanner);
        }
        if (pp.hadError) {
            std::cerr << "[minizinc_parse_string_only] Parser had errors (pp.hadError is true)." << std::endl; std::cerr.flush();
            std::cerr << "[minizinc_parse_string_only] DEALLOC: Deleting model due to parse error." << std::endl; std::cerr.flush();
            delete model; // Clean up allocated model
            return nullptr; // Return nullptr on error
        }

        std::cerr << "[minizinc_parse_string_only] Model parsed successfully." << std::endl; std::cerr.flush();
        std::cerr << "[minizinc_parse_string_only] DEBUG: Model pointer after mzn_yyparse: " << model << std::endl; std::cerr.flush();
        std::cerr << "[minizinc_parse_string_only] DEBUG: Model filename after mzn_yyparse: " << model->filename() << std::endl; std::cerr.flush();
        std::cerr << "[minizinc_parse_string_only] DEBUG: Model pointer is nullptr after mzn_yyparse? " << (model == nullptr ? "true" : "false") << std::endl; std::cerr.flush();

        return reinterpret_cast<MiniZincModel*>(model);

    } catch (const MiniZinc::Exception& e) {
        std::cerr << "[minizinc_parse_string_only] MiniZinc parsing error (captured): ";
        e.print(std::cerr); std::cerr.flush();
        return nullptr;
    } catch (const std::exception& e) {
        std::cerr << "[minizinc_parse_string_only] Standard exception (captured): " << e.what() << std::endl; std::cerr.flush();
        return nullptr;
    } catch (...) {
        std::cerr << "[minizinc_parse_string_only] Unknown exception during parsing (captured)." << std::endl; std::cerr.flush();
        return nullptr;
    }
}

} // extern "C"