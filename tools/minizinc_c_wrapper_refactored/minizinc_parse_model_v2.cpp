#include "minizinc_opaque_types.h"
#include "minizinc_ffi_helpers.h" // Include helper functions
#include <minizinc/solver.hh> // Include MznSolver (though not directly used for parsing now)
#include <minizinc/model.hh>
#include <minizinc/parser.hh> // Include Parser (for MiniZinc::parse_from_string function)
#include <iostream>
#include <fstream>
#include <cstdio>
#include <sstream>
#include <vector> // For std::vector
#include <unordered_set> // For std::unordered_set

extern "C" {

MiniZincModel* minizinc_parse_model(MiniZinc::MznSolver* wrapper_ptr, const char* model_str, const char* filename) {
    std::cerr << "[minizinc_parse_model] Starting parse process." << std::endl; std::cerr.flush();
    std::cerr << "[minizinc_parse_model] model_str: " << (model_str ? model_str : "(null)") << std::endl; std::cerr.flush();
    std::cerr << "[minizinc_parse_model] filename (initial const char*): " << (filename ? filename : "(null)") << std::endl; std::cerr.flush();

    std::string model_s(model_str);
    std::string filename_s(filename ? filename : "");

    std::cerr << "[minizinc_parse_model] DEBUG: filename_s after std::string conversion: \"" << filename_s  << std::endl; std::cerr.flush();
    std::cerr << "[minizinc_parse_model] DEBUG: filename_s.empty() after conversion: " << (filename_s.empty() ? "true" : "false") << std::endl; std::cerr.flush();

    // Determine the filename to pass to MiniZinc::parse_from_string
    std::string filename_to_pass;
    if (filename_s.empty()) {
        filename_to_pass = "<string>";
    } else {
        filename_to_pass = filename_s;
    }
    std::cerr << "[minizinc_parse_model]   filename_to_pass (after conditional, now always empty): \"" << filename_to_pass  << std::endl; std::cerr.flush();

    // Default values for arguments not directly provided by the FFI
    std::vector<std::string> includePaths_vec; // Empty for now
    bool isFlatZinc = false;
    bool ignoreStdlib = false;
    bool parseDocComments = false;
    bool verbose = true; // <--- Set verbose to true here
    std::ostream& err_stream = std::cerr; // Use cerr for errors

    try {
        MiniZinc::Env& env = *(wrapper_ptr->_flt.getEnv()); // Use the environment from the wrapper
        std::cerr << "[minizinc_parse_model] MiniZinc::Env created." << std::endl; std::cerr.flush();

        MiniZinc::Model* model = MiniZinc::parse_from_string(env,
                                                             model_s,
                                                             filename_to_pass,
                                                             includePaths_vec,
                                                             isFlatZinc,
                                                             ignoreStdlib,
                                                             parseDocComments,
                                                             verbose,
                                                             err_stream);
        std::cerr << "[minizinc_parse_model] MiniZinc::parse_from_string returned." << std::endl; std::cerr.flush();

        std::cerr << "[minizinc_parse_model] DEBUG: model: " << model << std::endl; std::cerr.flush();

        if (model == nullptr) {
            std::cerr << "[minizinc_parse_model] Error: MiniZinc::parse_from_string returned nullptr." << std::endl; std::cerr.flush();
            return nullptr; // Return nullptr on error
        }

        std::cerr << "[minizinc_parse_model] Model parsed successfully." << std::endl; std::cerr.flush();
        return reinterpret_cast<MiniZincModel*>(model);

    } catch (const MiniZinc::Exception& e) {
        std::cerr << "[minizinc_parse_model] MiniZinc parsing error (captured): ";
        e.print(std::cerr); std::cerr.flush();
        return nullptr;
    } catch (const std::exception& e) {
        std::cerr << "[minizinc_parse_model] Standard exception (captured): " << e.what() << std::endl; std::cerr.flush();
        return nullptr;
    } catch (...) {
        std::cerr << "[minizinc_parse_model] Unknown exception during parsing (captured)." << std::endl; std::cerr.flush();
        return nullptr;
    }
}
  
MiniZincModel* minizinc_parse_model2(MiniZinc::MznSolver* wrapper_ptr, const char* model_str, const char* filename) {
    std::cerr << "[minizinc_parse_model] Starting parse process." << std::endl;
    std::cerr.flush();
    std::cerr << "[minizinc_parse_model] model_str: " << (model_str ? model_str : "(null)") << std::endl; std::cerr.flush();
    std::cerr << "[minizinc_parse_model] filename (initial const char*): " << (filename ? filename : "(null)") << std::endl; std::cerr.flush();

    std::string model_s(model_str);
    std::string filename_s(filename);

    std::cerr << "[minizinc_parse_model] DEBUG: filename_s after std::string conversion: \"" << filename_s << "\"" << std::endl; std::cerr.flush();
    std::cerr << "[minizinc_parse_model] DEBUG: filename_s.empty() after conversion: " << (filename_s.empty() ? "true" : "false") << std::endl; std::cerr.flush();

    // Default values for arguments not directly provided by the FFI
    std::vector<std::string> filenames_vec; // Keep empty for parsing from string
    std::vector<std::string> datafiles_vec; // Empty for parsing from string
    std::vector<std::string> includePaths_vec; // Empty for now
    std::unordered_set<std::string> globalInc_set; // Empty for now
    bool isFlatZinc = false;
    bool ignoreStdlib = false;
    bool parseDocComments = false;
    bool verbose = true; // <--- Set verbose to true here
    std::ostream& err_stream = std::cerr; // Use cerr for errors

    std::cerr << "[minizinc_parse_model] Calling MiniZinc::parse_from_string with:" << std::endl; std::cerr.flush();
    std::cerr << "[minizinc_parse_model]   model_s: " << model_s << std::endl; std::cerr.flush();
    std::cerr << "[minizinc_parse_model]   filename_s (before conditional): \"" << filename_s << "\"" << std::endl; std::cerr.flush();
    std::cerr << "[minizinc_parse_model]   filename_s.empty() (before conditional): " << (filename_s.empty() ? "true" : "false") << std::endl; std::cerr.flush();

    // Always pass an empty string for the filename argument to MiniZinc::parse_from_string
    // when parsing from a model string, as it seems to trigger file open operations.
    std::string filename_to_pass = ""; 
    std::cerr << "[minizinc_parse_model]   filename_to_pass (after conditional, now always empty): \"" << filename_to_pass << "\"" << std::endl; std::cerr.flush();

    try {
        MiniZinc::Env& env = *(wrapper_ptr->_flt.getEnv()); // Use the environment from the wrapper
        

        // Call the MiniZinc::parse_from_string function
        MiniZinc::Model* model = MiniZinc::parse_from_string(env,
                                                             model_s,
                                                             filename_to_pass,
                                                             includePaths_vec,
                                                             isFlatZinc,
                                                             ignoreStdlib,
                                                             parseDocComments,
                                                             verbose,
                                                             err_stream);
        std::cerr << "[minizinc_parse_model] MiniZinc::parse_from_string returned." << std::endl; std::cerr.flush();

        std::cerr << "[minizinc_parse_model] DEBUG: model: " << model << std::endl; std::cerr.flush();
        if (!model) {
            std::cerr << "[minizinc_parse_model] Error: MiniZinc::parse_from_string returned nullptr." << std::endl; std::cerr.flush();
            return nullptr;
        }
        std::cerr << "[minizinc_parse_model] Model parsed successfully." << std::endl; std::cerr.flush();
        return reinterpret_cast<MiniZincModel*>(model);

    } catch (const MiniZinc::Exception& e) {
        std::cerr << "[minizinc_parse_model] MiniZinc parsing error (captured): ";
        e.print(std::cerr); std::cerr.flush();
        return nullptr;
    } catch (const std::exception& e) {
        std::cerr << "[minizinc_parse_model] Standard exception (captured): " << e.what() << std::endl; std::cerr.flush();
        return nullptr;
    } catch (...) {
        std::cerr << "[minizinc_parse_model] Unknown exception during parsing (captured)." << std::endl; std::cerr.flush();
        return nullptr;
    }
}

} // extern "C"
