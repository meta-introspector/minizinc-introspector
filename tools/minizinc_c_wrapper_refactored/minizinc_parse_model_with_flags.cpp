#include "minizinc_opaque_types.h"
#include <minizinc/solver.hh> // Include MznSolver (though not directly used for parsing now)
#include <minizinc/model.hh>
#include <minizinc/parser.hh> // Include Parser (for MiniZinc::parse_from_string function)
#include <iostream>
#include <fstream>
#include <cstdio>
#include <sstream>
#include <vector> // For std::vector
#include <unordered_set> // For std::unordered_set

// Include the new FFI declarations header
#include "minizinc_ffi_declarations_v2.h"

extern "C" {

// MiniZinc::MznSolver* solver = reinterpret_cast<MiniZinc::MznSolver*>(solver_ptr);
    // We don't need the MznSolver here, as parsing is done via MiniZinc::parse
    // and the Env is created locally.
    // If we need to pass the MznSolver's Env, we would need to access it from the wrapper.
    // For now, we create a local Env for parsing.

    std::cerr << "[minizinc_parse_model_with_flags] Starting parse process." << std::endl; std::cerr.flush();
    std::cerr << "[minizinc_parse_model_with_flags] model_str: " << (model_str ? model_str : "(null)") << std::endl; std::cerr.flush();
    std::cerr << "[minizinc_parse_model_with_flags] filename (initial const char*): " << (filename ? filename : "(null)") << std::endl; std::cerr.flush();
    std::cerr << "[minizinc_parse_model_with_flags] is_model_string_flag: " << (is_model_string_flag ? "true" : "false") << std::endl; std::cerr.flush();

    std::string model_s(model_str);
    std::string filename_s(filename);

    std::cerr << "[minizinc_parse_model_with_flags] DEBUG: filename_s after std::string conversion: \"" << filename_s << "\"" << std::endl; std::cerr.flush();
    std::cerr << "[minizinc_parse_model_with_flags] DEBUG: filename_s.empty() after conversion: " << (filename_s.empty() ? "true" : "false") << std::endl; std::cerr.flush();

    // Default values for arguments not directly provided by the FFI
    std::vector<std::string> filenames_vec; // Keep empty for parsing from string
    std::vector<std::string> datafiles_vec; // Empty for parsing from string
    std::vector<std::string> includePaths_vec; // Empty for now
    std::unordered_set<std::string> globalInc_set; // Empty for now
    // bool isFlatZinc = false; // Repurposing this for is_model_string_flag
    bool ignoreStdlib = false;
    bool parseDocComments = false;
    bool verbose = true; // Keep verbose true for debugging
    std::ostream& err_stream = std::cerr; // Use cerr for errors

    std::cerr << "[minizinc_parse_model_with_flags] Calling MiniZinc::parse with:" << std::endl; std::cerr.flush();
    std::cerr << "[minizinc_parse_model_with_flags]   model_s: " << model_s << std::endl; std::cerr.flush();
    std::cerr << "[minizinc_parse_model_with_flags]   filename_s (before conditional): \"" << filename_s << "\"" << std::endl; std::cerr.flush();
    std::cerr << "[minizinc_parse_model_with_flags]   filename_s.empty() (before conditional): " << (filename_s.empty() ? "true" : "false") << std::endl; std::cerr.flush();

    // Always pass an empty string for the filename argument to MiniZinc::parse_from_string
    // when parsing from a model string, as it seems to trigger file open operations.
    // The actual filename (if provided) is now only for metadata/logging.
    std::string filename_to_pass = ""; 
    std::cerr << "[minizinc_parse_model_with_flags]   filename_to_pass (after conditional, now always empty): \"" << filename_to_pass << "\"" << std::endl; std::cerr.flush();

    try {
        MiniZinc::Env env; // Create an environment object
        

        // Directly call the parse function
        MiniZinc::Model* model = parse(env, filenames_vec, datafiles_vec, model_s, filename_to_pass,
                                       includePaths_vec, globalInc_set,
                                       is_model_string_flag, // Repurposing isFlatZinc to pass our flag
                                       ignoreStdlib, parseDocComments, verbose, err_stream);

        std::cerr << "[minizinc_parse_model_with_flags] MiniZinc::parse returned." << std::endl; std::cerr.flush();

        std::cerr << "[minizinc_parse_model_with_flags] DEBUG: model: " << model << std::endl; std::cerr.flush();
        if (!model) {
            std::cerr << "[minizinc_parse_model_with_flags] Error: MiniZinc::parse returned nullptr." << std::endl; std::cerr.flush();
            return nullptr;
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
    // MiniZinc::MznSolver* solver = reinterpret_cast<MiniZinc::MznSolver*>(solver_ptr);
    // We don't need the MznSolver here, as parsing is done via MiniZinc::parse
    // and the Env is created locally.
    // If we need to pass the MznSolver's Env, we would need to access it from the wrapper.
    // For now, we create a local Env for parsing.

    std::cerr << "[minizinc_parse_model_with_flags] Starting parse process." << std::endl; std::cerr.flush();
    std::cerr << "[minizinc_parse_model_with_flags] model_str: " << (model_str ? model_str : "(null)") << std::endl; std::cerr.flush();
    std::cerr << "[minizinc_parse_model_with_flags] filename (initial const char*): " << (filename ? filename : "(null)") << std::endl; std::cerr.flush();
    std::cerr << "[minizinc_parse_model_with_flags] is_model_string_flag: " << (is_model_string_flag ? "true" : "false") << std::endl; std::cerr.flush();

    std::string model_s(model_str);
    std::string filename_s(filename);

    std::cerr << "[minizinc_parse_model_with_flags] DEBUG: filename_s after std::string conversion: \"" << filename_s << "\"\"" << std::endl; std::cerr.flush();
    std::cerr << "[minizinc_parse_model_with_flags] DEBUG: filename_s.empty() after conversion: " << (filename_s.empty() ? "true" : "false") << std::endl; std::cerr.flush();

    // Default values for arguments not directly provided by the FFI
    std::vector<std::string> filenames_vec; // Keep empty for parsing from string
    std::vector<std::string> datafiles_vec; // Empty for parsing from string
    std::vector<std::string> includePaths_vec; // Empty for now
    std::unordered_set<std::string> globalInc_set; // Empty for now
    // bool isFlatZinc = false; // Repurposing this for is_model_string_flag
    bool ignoreStdlib = false;
    bool parseDocComments = false;
    bool verbose = true; // Keep verbose true for debugging
    std::ostream& err_stream = std::cerr; // Use cerr for errors

    std::cerr << "[minizinc_parse_model_with_flags] Calling MiniZinc::parse with:" << std::endl; std::cerr.flush();
    std::cerr << "[minizinc_parse_model_with_flags]   model_s: " << model_s << std::endl; std::cerr.flush();
    std::cerr << "[minizinc_parse_model_with_flags]   filename_s (before conditional): \"" << filename_s << "\"\"" << std::endl; std::cerr.flush();
    std::cerr << "[minizinc_parse_model_with_flags]   filename_s.empty() (before conditional): " << (filename_s.empty() ? "true" : "false") << std::endl; std::cerr.flush();

    // Always pass an empty string for the filename argument to MiniZinc::parse_from_string
    // when parsing from a model string, as it seems to trigger file open operations.
    // The actual filename (if provided) is now only for metadata/logging.
    std::string filename_to_pass = ""; 
    std::cerr << "[minizinc_parse_model_with_flags]   filename_to_pass (after conditional, now always empty): \"" << filename_to_pass << "\"\"" << std::endl; std::cerr.flush();

    try {
        MiniZinc::Env env; // Create an environment object
        

        // Directly call the parse function
        MiniZinc::Model* model = parse(env, filenames_vec, datafiles_vec, model_s, filename_to_pass,
                                       includePaths_vec, globalInc_set,
                                       is_model_string_flag, // Repurposing isFlatZinc to pass our flag
                                       ignoreStdlib, parseDocComments, verbose, err_stream);

        std::cerr << "[minizinc_parse_model_with_flags] MiniZinc::parse returned." << std::endl; std::cerr.flush();

        std::cerr << "[minizinc_parse_model_with_flags] DEBUG: model: " << model << std::endl; std::cerr.flush();
        if (!model) {
            std::cerr << "[minizinc_parse_model_with_flags] Error: MiniZinc::parse returned nullptr." << std::endl; std::cerr.flush();
            return nullptr;
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
