#include "minizinc_c_wrapper.h"
#include <minizinc/model.hh>
#include <minizinc/parser.hh>
// #include <minizinc/version.hh> // For version string

#include <iostream> // For debugging
#include <fstream>  // For temporary file
#include <cstdio>   // For remove (C-style file deletion)

// Function to create a new MiniZinc environment
MiniZincEnv* minizinc_env_new() {
    // MiniZinc::Env constructor takes optional Model*, ostream&, ostream&
    // For simplicity, we'll use default streams for now.
    return reinterpret_cast<MiniZincEnv*>(new MiniZinc::Env());
}

// Function to free a MiniZinc environment
void minizinc_env_free(MiniZincEnv* env) {
    delete reinterpret_cast<MiniZinc::Env*>(env);
}

// Function to parse a MiniZinc model from a string
MiniZincModel* minizinc_parse_model_from_string(MiniZincEnv* env_ptr, const char* model_str, const char* filename) {
    MiniZinc::Env* env = reinterpret_cast<MiniZinc::Env*>(env_ptr);
    std::string model_s(model_str);
    std::string filename_s(filename);
    std::vector<std::string> include_paths;
    // Add MiniZinc standard library path
    include_paths.push_back("/data/data/com.termux/files/home/storage/github/libminizinc/install/share/minizinc");
    bool is_flatzinc = false;
    bool ignore_stdlib = false;
    bool parse_doc_comments = false;
    bool verbose = false;
    std::ostream& err = std::cerr; // Use cerr for errors

    try {
        MiniZinc::Model* model = MiniZinc::parse_from_string(*env, model_s, filename_s,
                                                               include_paths, is_flatzinc,
                                                               ignore_stdlib, parse_doc_comments,
                                                               verbose, err);
        return reinterpret_cast<MiniZincModel*>(model);
    } catch (const MiniZinc::Exception& e) {
        std::cerr << "MiniZinc parsing error: " << e.what() << std::endl;
        return nullptr;
    } catch (const std::exception& e) {
        std::cerr << "Standard exception: " << e.what() << std::endl;
        return nullptr;
    } catch (...) {
        std::cerr << "Unknown exception during parsing." << std::endl;
        return nullptr;
    }
}

// Function to parse DZN data into a MiniZinc model
int minizinc_parse_data_from_string(MiniZincEnv* env_ptr, MiniZincModel* model_ptr, const char* data_str,
       const char* filename) {
    MiniZinc::Env* env = reinterpret_cast<MiniZinc::Env*>(env_ptr);
    MiniZinc::Model* model = reinterpret_cast<MiniZinc::Model*>(model_ptr);
    std::string data_s(data_str);
    std::string filename_s(filename);
    std::vector<std::string> include_paths;
    // Add MiniZinc standard library path
    include_paths.push_back("/data/data/com.termux/files/home/storage/github/libminizinc/install/share/minizinc");
    bool is_flatzinc = false;
    bool ignore_stdlib = false;
    bool parse_doc_comments = false;
    bool verbose = false;
    std::ostream& err = std::cerr; // Use cerr for errors

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
        MiniZinc::parse_data(*env, model, data_files, include_paths, is_flatzinc,
                              ignore_stdlib, parse_doc_comments, verbose, err);

        // Clean up temporary file
        std::remove(temp_data_filename.c_str()); // Use C-style remove for broader compatibility

        return 0; // Success
    } catch (const MiniZinc::Exception& e) {
        std::cerr << "MiniZinc data parsing error: " << e.what() << std::endl;
        std::remove(temp_data_filename.c_str());
        return -1; // Failure
    } catch (const std::exception& e) {
        std::cerr << "Standard exception: " << e.what() << std::endl;
        std::remove(temp_data_filename.c_str());
        return -1; // Failure
    } catch (...) {
        std::cerr << "Unknown exception during data parsing." << std::endl;
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