#include "minizinc_c_wrapper.h"
#include <minizinc/model.hh>
#include <minizinc/parser.hh>
#include <minizinc/flattener.hh> // Include Flattener header
#include <minizinc/solver.hh> // Include Solver header for SolverInitialiser

#include <iostream> // For debugging
#include <fstream>  // For temporary file
#include <cstdio>   // For remove (C-style file deletion)
#include <sstream>  // For std::stringstream

// Function to create a new MiniZinc environment
Flattener* minizinc_env_new() {
    // Create a static SolverInitialiser to ensure global MiniZinc setup
    static MiniZinc::SolverInitialiser solver_initialiser;

    // MiniZinc::Flattener constructor takes ostream& os, ostream& log, string stdlibDir
    // The stdlibDir is crucial for parsing.
    std::string stdlib_path = "/data/data/com.termux/files/home/storage/github/libminizinc/install/share/minizinc";
    std::cerr << "DEBUG: minizinc_env_new - Using stdlib_path: " << stdlib_path << std::endl; std::cerr.flush();
    MiniZinc::Flattener* new_flattener = new MiniZinc::Flattener(std::cout, std::cerr, stdlib_path);

    std::cerr << "DEBUG: minizinc_env_new - Created Flattener at: " << new_flattener << std::endl; std::cerr.flush();
    // Removed: if (new_flattener) { std::cerr << "DEBUG: minizinc_env_new - Flattener->getEnv() returns: " << new_flattener->getEnv() << std::endl; } else { std::cerr << "DEBUG: minizinc_env_new - Flattener creation failed (nullptr)." << std::endl; }

    return reinterpret_cast<Flattener*>(new_flattener);
}

// Function to free a MiniZinc environment
void minizinc_env_free(Flattener* env) {
    std::cerr << "DEBUG: minizinc_env_free - Freeing Flattener at: " << env << std::endl; std::cerr.flush();
    delete reinterpret_cast<MiniZinc::Flattener*>(env);
}

// Function to parse a MiniZinc model from a string
MiniZincModel* minizinc_parse_model(Flattener* env_ptr, const char* model_str, const char* filename) {
    std::cerr << "Starting MiniZinc parse process" << std::endl; std::cerr.flush();
    MiniZinc::Flattener* flattener = reinterpret_cast<MiniZinc::Flattener*>(env_ptr);
    
    std::string model_s(model_str);
    // Create a temporary file for the model content
    std::string temp_model_filename = "/data/data/com.termux/files/home/storage/github/libminizinc/temp_model_" + std::string(filename);
    std::ofstream temp_model_file(temp_model_filename);
    if (!temp_model_file.is_open()) {
        std::cerr << "Error: Could not open temporary model file for writing: " << temp_model_filename << std::endl; std::cerr.flush();
        return nullptr;
    }
    temp_model_file << model_s;
    temp_model_file.close();

    std::string filename_s = temp_model_filename; // Use the temporary file path
    std::vector<std::string> include_paths;
    // The standard library path is now handled by the Flattener constructor
    bool is_flatzinc = false;
    bool ignore_stdlib = false;
    bool parse_doc_comments = false;
    bool verbose = true; // Changed to true

    std::stringstream ss_err; // Create a stringstream for error capture
    std::ostream& err = ss_err; // Redirect error output to stringstream

    try {
        std::cerr << "DEBUG: Before flattener->flatten()" << std::endl; std::cerr.flush();
        // Call flatten to initialize the Env
        flattener->flatten(model_s, filename_s);
        std::cerr << "DEBUG: After flattener->flatten()" << std::endl; std::cerr.flush();
        MiniZinc::Env* mzn_env_ptr = flattener->getEnv(); // Get the Env pointer from Flattener
        std::cerr << "DEBUG: mzn_env_ptr: " << mzn_env_ptr << std::endl; std::cerr.flush();
        if (!mzn_env_ptr) {
            std::cerr << "Error: Flattener->getEnv() returned nullptr after flatten in minizinc_parse_model." << std::endl;
            return nullptr;
        }
        MiniZinc::Env& env = *mzn_env_ptr; // Dereference the pointer

        // After flattening, the Flattener's internal Env should contain the parsed model.
        // We retrieve the model directly from the Env.
        MiniZinc::Model* model = env.model();
        std::cerr << "DEBUG: model: " << model << std::endl; std::cerr.flush();
        if (!model) {
            std::cerr << "Error: Env->model() returned nullptr after flatten in minizinc_parse_model." << std::endl;
            return nullptr;
        }
        return reinterpret_cast<MiniZincModel*>(model);
        std::remove(temp_model_filename.c_str()); // Clean up temporary file
    } catch (const MiniZinc::Exception& e) {
        std::cerr << "MiniZinc parsing error (captured): " << e.msg() << std::endl; std::cerr.flush();
        std::remove(temp_model_filename.c_str());
        return nullptr;
    } catch (const std::exception& e) {
        std::cerr << "Standard exception (captured): " << e.what() << std::endl; std::cerr.flush();
        std::remove(temp_model_filename.c_str());
        return nullptr;
    } catch (...) {
        std::cerr << "Unknown exception during parsing (captured)." << std::endl; std::cerr.flush();
        std::remove(temp_model_filename.c_str());
        return nullptr;
    }
}

// Function to parse DZN data into a MiniZinc model
int minizinc_parse_data_from_string(Flattener* env_ptr, MiniZincModel* model_ptr, const char* data_str,
       const char* filename) {
    std::cerr << "DEBUG: minizinc_parse_data_from_string - Entry" << std::endl; std::cerr.flush();
    MiniZinc::Flattener* flattener = reinterpret_cast<MiniZinc::Flattener*>(env_ptr);
    std::cerr << "DEBUG: minizinc_parse_data_from_string - Before flatten() - flattener: " << flattener << std::endl; std::cerr.flush();
    // Call flatten to initialize the Env
    flattener->flatten(); // Call flatten without arguments to initialize Env
    std::cerr << "DEBUG: minizinc_parse_data_from_string - After flatten()" << std::endl; std::cerr.flush();

    std::cerr << "DEBUG: minizinc_parse_data_from_string - Before getEnv()" << std::endl; std::cerr.flush();
    MiniZinc::Env* mzn_env_ptr = flattener->getEnv(); // Get the Env pointer from Flattener
    std::cerr << "DEBUG: minizinc_parse_data_from_string - After getEnv() - mzn_env_ptr: " << mzn_env_ptr << std::endl; std::cerr.flush();

    if (!mzn_env_ptr) {
        std::cerr << "Error: Flattener->getEnv() returned nullptr after flatten in minizinc_parse_data_from_string." << std::endl; std::cerr.flush();
        return -1;
    }
    MiniZinc::Env& env = *mzn_env_ptr; // Dereference the pointer
    MiniZinc::Model* model = reinterpret_cast<MiniZinc::Model*>(model_ptr);
    std::cerr << "DEBUG: minizinc_parse_data_from_string - Model* model_ptr: " << model_ptr << std::endl; std::cerr.flush();
    std::string data_s(data_str);
    std::string filename_s = "/tmp/" + std::string(filename); // Prepend dummy absolute path
    std::vector<std::string> include_paths;
    // The standard library path is now handled by the Flattener constructor
    bool is_flatzinc = false;
    bool ignore_stdlib = false;
    bool parse_doc_comments = false;
    bool verbose = true; // Changed to true

    std::stringstream ss_err; // Create a stringstream for error capture
    std::ostream& err = ss_err; // Redirect error output to stringstream

    // Workaround: MiniZinc::parse_data expects a vector of filenames, not a string directly.
    // Write the data string to a temporary file and pass its name.
    std::string temp_data_filename = filename_s + "_temp_data.dzn";
    std::ofstream temp_data_file(temp_data_filename);
    if (!temp_data_file.is_open()) {
        std::cerr << "Error: Could not open temporary data file for writing: " << temp_data_filename <<
        std::endl; std::cerr.flush();
        return -1;
    }
    temp_data_file << data_s;
    temp_data_file.close();

    std::vector<std::string> data_files;
    data_files.push_back(temp_data_filename);

    try {
        std::cerr << "DEBUG: minizinc_parse_data_from_string - Before parse_data()" << std::endl; std::cerr.flush();
        MiniZinc::parse_data(env, model, data_files, include_paths, is_flatzinc,
                              ignore_stdlib, parse_doc_comments, verbose, err);
        std::cerr << "DEBUG: minizinc_parse_data_from_string - After parse_data()" << std::endl; std::cerr.flush();

        // Clean up temporary file
        std::remove(temp_data_filename.c_str()); // Use C-style remove for broader compatibility

        std::cerr << "DEBUG: minizinc_parse_data_from_string - Exit Success" << std::endl; std::cerr.flush();
        return 0; // Success
    } catch (const MiniZinc::Exception& e) {
        std::cerr << "MiniZinc data parsing error (captured): " << e.what() << std::endl; std::cerr.flush();
        std::remove(temp_data_filename.c_str());
        return -1; // Failure
    } catch (const std::exception& e) {
        std::cerr << "Standard exception (captured): " << e.what() << std::endl; std::cerr.flush();
        std::remove(temp_data_filename.c_str());
        return -1; // Failure
    } catch (...) {
        std::cerr << "Unknown exception during data parsing (captured)." << std::endl; std::cerr.flush();
        std::remove(temp_data_filename.c_str());
        return -1; // Failure
    }
}

// Function to free a MiniZinc model
void minizinc_model_free(MiniZincModel* model) {
    delete reinterpret_cast<MiniZinc::Model*>(model);
}

// Function to get version string (for testing FFI)
const char* minizinc_get_version_string() {
    return "2.9.4-introspector";
}
