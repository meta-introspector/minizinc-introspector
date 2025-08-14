#include "minizinc_opaque_types.h"
#include "minizinc_env_wrapper.h"
#include <minizinc/model.hh>
#include <iostream>
#include <fstream>
#include <cstdio>
#include <sstream>

extern "C" {

int minizinc_parse_data_from_string(MiniZinc::Flattener* env_ptr, MiniZincModel* model_ptr, const char* data_str,
       const char* filename) {
    std::cerr << "DEBUG: minizinc_parse_data_from_string - Entry" << std::endl; std::cerr.flush();
    MiniZinc::MiniZincEnvWrapper* wrapper = reinterpret_cast<MiniZinc::MiniZincEnvWrapper*>(env_ptr);
    MiniZinc::Flattener* flattener = wrapper->getFlattener();
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

        std::cerr << "DEBUG: minizinc_parse_data_from_string - Exit Success"; std::cerr.flush();
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

} // extern "C"
