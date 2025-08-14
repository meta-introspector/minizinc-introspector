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
    MiniZinc::Flattener* new_flattener = new MiniZinc::Flattener(std::cout, std::cerr, stdlib_path);

    std::cerr << "DEBUG: minizinc_env_new - Created Flattener at: " << new_flattener << std::endl;
    if (new_flattener) {
        std::cerr << "DEBUG: minizinc_env_new - Flattener->getEnv() returns: " << new_flattener->getEnv() << std::endl;
    } else {
        std::cerr << "DEBUG: minizinc_env_new - Flattener creation failed (nullptr)." << std::endl;
    }

    return reinterpret_cast<Flattener*>(new_flattener);
}

// Function to free a MiniZinc environment
void minizinc_env_free(Flattener* env) {
    delete reinterpret_cast<MiniZinc::Flattener*>(env);
}

// Function to parse a MiniZinc model from a string
MiniZincModel* minizinc_parse_model(Flattener* env_ptr, const char* model_str, const char* filename) {
    MiniZinc::Flattener* flattener = reinterpret_cast<MiniZinc::Flattener*>(env_ptr);
    
    std::string model_s(model_str);
    std::string filename_s = "/tmp/" + std::string(filename); // Prepend dummy absolute path
    std::vector<std::string> include_paths;
    // The standard library path is now handled by the Flattener constructor
    bool is_flatzinc = false;
    bool ignore_stdlib = false;
    bool parse_doc_comments = false;
    bool verbose = true; // Changed to true

    std::stringstream ss_err; // Create a stringstream for error capture
    std::ostream& err = ss_err; // Redirect error output to stringstream

    try {
        // Call flatten to initialize the Env
        flattener->flatten(model_s, filename_s);
        MiniZinc::Env& env = *(flattener->getEnv()); // Get the Env from Flattener

        MiniZinc::Model* model = MiniZinc::parse_from_string(env, model_s, filename_s,
                                                               include_paths, is_flatzinc,
                                                               ignore_stdlib, parse_doc_comments,
                                                               verbose, err);
        return reinterpret_cast<MiniZincModel*>(model);
    } catch (const MiniZinc::Exception& e) {
        std::cerr << "MiniZinc parsing error (captured): " << ss_err.str() << " | Exception: " << e.what() << std::endl;
        return nullptr;
    } catch (const std::exception& e) {
        std::cerr << "Standard exception (captured): " << ss_err.str() << " | Exception: " << e.what() << std::endl;
        return nullptr;
    } catch (...) {
        std::cerr << "Unknown exception during parsing (captured): " << ss_err.str() << std::endl;
        return nullptr;
    }
}

// Function to parse DZN data into a MiniZinc model
int minizinc_parse_data_from_string(Flattener* env_ptr, MiniZincModel* model_ptr, const char* data_str,
       const char* filename) {
    MiniZinc::Flattener* flattener = reinterpret_cast<MiniZinc::Flattener*>(env_ptr);
    // Call flatten to initialize the Env
    flattener->flatten(); // Call flatten without arguments to initialize Env
    MiniZinc::Env& env = *(flattener->getEnv()); // Dereference the pointer
    MiniZinc::Model* model = reinterpret_cast<MiniZinc::Model*>(model_ptr);
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
        std::endl;
        return -1;
    }
    temp_data_file << data_s;
    temp_data_file.close();

    std::vector<std::string> data_files;
    data_files.push_back(temp_data_filename);

    try {
        MiniZinc::parse_data(env, model, data_files, include_paths, is_flatzinc,
                              ignore_stdlib, parse_doc_comments, verbose, err);

        // Clean up temporary file
        std::remove(temp_data_filename.c_str()); // Use C-style remove for broader compatibility

        return 0; // Success
    } catch (const MiniZinc::Exception& e) {
        std::cerr << "MiniZinc data parsing error (captured): " << ss_err.str() << " | Exception: " << e.what() << std::endl;
        std::remove(temp_data_filename.c_str());
        return -1; // Failure
    } catch (const std::exception& e) {
        std::cerr << "Standard exception (captured): " << ss_err.str() << " | Exception: " << e.what() << std::endl;
        std::remove(temp_data_filename.c_str());
        return -1; // Failure
    } catch (...) {
        std::cerr << "Unknown exception during data parsing (captured): " << ss_err.str() << std::endl;
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