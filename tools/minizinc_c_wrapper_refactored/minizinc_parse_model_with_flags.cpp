#include "minizinc_opaque_types.h"
#include "minizinc_ffi_helpers.h" // Include helper functions
#include <minizinc/solver.hh> // Include MznSolver (though not directly used for parsing now)
#include <minizinc/model.hh>
#include <minizinc/parser.hh> // Include Parser (for MiniZinc::parse_from_string function)
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

MiniZincModel* minizinc_parse_model_with_flags(MiniZincEnvWrapper* wrapper_ptr, const char* model_str, const char* filename, bool is_model_string_flag) {
    MiniZinc::GCLock lock; // Acquire GC lock for this function

    std::cerr << "[minizinc_parse_model_with_flags] Starting parse process." << std::endl; std::cerr.flush();
    std::cerr << "[minizinc_parse_model_with_flags] model_str: " << (model_str ? model_str : "(null)") << std::endl; std::cerr.flush();
    std::cerr << "[minizinc_parse_model_with_flags] filename (initial const char*): " << (filename ? filename : "(null)") << std::endl; std::cerr.flush();
    std::cerr << "[minizinc_parse_model_with_flags] is_model_string_flag: " << (is_model_string_flag ? "true" : "false") << std::endl; std::cerr.flush();

    std::string model_s(model_str);
    std::string filename_s(filename ? filename : "<string>"); // Use provided filename or default to "<string>"

    try {
        // Use the MiniZinc::Env from the wrapper_ptr
        // MiniZinc::Env& env = *wrapper_ptr->env; // This line is commented out as Env is not directly used by ParserState
        // std::cerr << "[minizinc_parse_model_with_flags] MiniZinc::Env from wrapper used." << std::endl; std::cerr.flush();
        
        MiniZinc::Model* model = new MiniZinc::Model();
        model->setFilename(filename_s); // Set filename for the model

        // Create a ParserState directly for string parsing
        std::vector<std::string> includePaths_vec; // Empty
        std::vector<MiniZinc::ParseWorkItem> files_vec; // Empty
        std::map<std::string, MiniZinc::Model*> seenModels_map; // Empty

        MiniZinc::ParserState pp(filename_s, model_s, std::cerr, includePaths_vec, files_vec, seenModels_map, model, false, false, false, false);

        mzn_yylex_init(&pp.yyscanner);
        mzn_yyset_extra(&pp, pp.yyscanner);
        mzn_yyparse(&pp);

        if (pp.yyscanner != nullptr) {
            mzn_yylex_destroy(pp.yyscanner);
        }
        if (pp.hadError) {
            std::cerr << "[minizinc_parse_model_with_flags] Parser had errors (pp.hadError is true)." << std::endl; std::cerr.flush();
            std::cerr << "[minizinc_parse_model_with_flags] DEALLOC: Deleting model due to parse error." << std::endl; std::cerr.flush();
            delete model; // Clean up allocated model
            return nullptr; // Return nullptr on error
        }

        std::cerr << "[minizinc_parse_model_with_flags] Model parsed successfully." << std::endl; std::cerr.flush();
        return reinterpret_cast<MiniZincModel*>(model);

    } catch (const MiniZinc::Exception& e) {
        std::cerr << "[minizinc_parse_model_with_flags] MiniZinc parsing error (captured): ";
        e.print(std::cerr); std::cerr.flush();
        return nullptr;
    } catch (const std::exception& e) {
        std::cerr << "[minizinc_parse_model_with_flags] Standard exception (captured): " << e.what() << std::endl; std::cerr.flush();
        return nullptr;
    } catch (...) {
        std::cerr << "[minizinc_parse_model_with_flags] Unknown exception during parsing (captured)." << std::endl; std::cerr.flush();
        return nullptr;
    }
}

} // extern "C"