#include "minizinc_c_wrapper.h"
#include "minizinc_env_wrapper.h"
#include <minizinc/model.hh>
#include <minizinc/parser.hh>
#include <iostream>
#include <fstream>
#include <cstdio>
#include <sstream>

extern "C" {

MiniZincModel* minizinc_parse_model(MiniZinc::Flattener* env_ptr, const char* model_str, const char* filename) {
    std::cerr << "Starting MiniZinc parse process" << std::endl; std::cerr.flush();
    MiniZinc::MiniZincEnvWrapper* wrapper = reinterpret_cast<MiniZinc::MiniZincEnvWrapper*>(env_ptr);
    MiniZinc::Flattener* flattener = wrapper->getFlattener();
    std::cerr << "DEBUG: minizinc_parse_model - Flattener address: " << flattener << std::endl; std::cerr.flush();
    
    std::string model_s(model_str);
    std::string filename_s = "/data/data/com.termux/files/home/storage/github/libminizinc/dummy_model.mzn"; // Use a dummy filename in project directory
    std::vector<std::string> include_paths;

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

    } catch (const MiniZinc::Exception& e) {
        std::cerr << "MiniZinc parsing error (captured): ";
        e.print(std::cerr); std::cerr.flush();
        return nullptr;
    } catch (const std::exception& e) {
        std::cerr << "Standard exception (captured): " << e.what() << std::endl; std::cerr.flush();
        return nullptr;
    } catch (...) {
        std::cerr << "Unknown exception during parsing (captured)." << std::endl; std::cerr.flush();
        return nullptr;
    }
}

} // extern "C"
