#include "minizinc_opaque_types.h"
#include <minizinc/solver.hh> // Include MznSolver
#include <minizinc/model.hh>
#include <minizinc/parser.hh>
#include <iostream>
#include <fstream>
#include <cstdio>
#include <sstream>
#include <vector> // For std::vector

extern "C" {

MiniZincModel* minizinc_parse_model(MiniZinc::MznSolver* solver_ptr, const char* model_str, const char* filename) {
    std::cerr << "Starting MiniZinc parse process (via MznSolver::run)" << std::endl; std::cerr.flush();

    std::string model_s(model_str);
    std::string filename_s(filename);

    // Construct arguments for MznSolver::run()
    std::vector<std::string> args;
    // The first argument is typically the executable name, which MznSolver::run handles
    // The model content is passed directly to run()
    // The filename is also passed to run()

    // MznSolver::run expects the model content as a string, and the filename.
    // It will internally handle parsing and flattening.

    try {
        // Call MznSolver::run()
        // The model content is passed as the second argument (model string)
        // The filename is passed as the fourth argument (model name)
        MiniZinc::SolverInstance::Status status = solver_ptr->run(args, model_s, "minizinc", filename_s);

        if (status == MiniZinc::SolverInstance::ERROR) {
            std::cerr << "Error: MznSolver::run() returned an error status." << std::endl;
            return nullptr;
        }

        // After run() completes, the parsed model should be available in the solver's environment
        MiniZinc::Env* mzn_env_ptr = solver_ptr->getFlt().getEnv(); // Access Flattener from MznSolver, then Env
        std::cerr << "DEBUG: mzn_env_ptr: " << mzn_env_ptr << std::endl; std::cerr.flush();

        if (!mzn_env_ptr) {
            std::cerr << "Error: MznSolver's Flattener->getEnv() returned nullptr after run." << std::endl;
            return nullptr;
        }
        MiniZinc::Env& env = *mzn_env_ptr;

        MiniZinc::Model* model = env.model();
        std::cerr << "DEBUG: model: " << model << std::endl; std::cerr.flush();
        if (!model) {
            std::cerr << "Error: Env->model() returned nullptr after run." << std::endl;
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
