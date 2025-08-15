#include "minizinc_opaque_types.h"
#include <minizinc/solver.hh> // Include MznSolver (though not directly used for parsing now)
#include <minizinc/model.hh>
#include <minizinc/parser.hh> // Include Parser (for MiniZinc::parse function)
#include <iostream>
#include <fstream>
#include <cstdio>
#include <sstream>
#include <vector> // For std::vector
#include <unordered_set> // For std::unordered_set

extern "C" {

MiniZincModel* minizinc_parse_model(MiniZinc::MznSolver* solver_ptr, const char* model_str, const char* filename) {
    std::cerr << "Starting MiniZinc parse process (via MiniZinc::parse function with all arguments)" << std::endl; std::cerr.flush();

    std::string model_s(model_str);
    std::string filename_s(filename);

    // Default values for arguments not directly provided by the FFI
    std::vector<std::string> filenames_vec; // Empty for parsing from string
    std::vector<std::string> datafiles_vec; // Empty for parsing from string
    std::vector<std::string> includePaths_vec; // Empty for now
    std::unordered_set<std::string> globalInc_set; // Empty for now
    bool isFlatZinc = false;
    bool ignoreStdlib = false;
    bool parseDocComments = false;
    bool verbose = false;
    std::ostream& err_stream = std::cerr; // Use cerr for errors

    try {
        MiniZinc::Env env; // Create an environment object

        // Call the MiniZinc::parse function with all required arguments
        MiniZinc::Model* model = MiniZinc::parse(env,
                                                 filenames_vec,
                                                 datafiles_vec,
                                                 model_s,
                                                 filename_s,
                                                 includePaths_vec,
                                                 globalInc_set,
                                                 isFlatZinc,
                                                 ignoreStdlib,
                                                 parseDocComments,
                                                 verbose,
                                                 err_stream);

        std::cerr << "DEBUG: model: " << model << std::endl; std::cerr.flush();
        if (!model) {
            std::cerr << "Error: MiniZinc::parse returned nullptr." << std::endl;
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