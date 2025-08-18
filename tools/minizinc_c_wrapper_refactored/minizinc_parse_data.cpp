#include "minizinc_opaque_types.h"
#include "minizinc_ffi_declarations.h"
#include <minizinc/parser.hh>
#include <minizinc/model.hh>
#include <minizinc/solver.hh>
#include <iostream>
#include <vector>
#include <string>
#include <fstream>
#include <sstream>
#include <cstdio>   // For remove
#include <cstdlib>  // For mkstemp
#include <unistd.h> // For close
#include <sys/stat.h> // For mkdir

extern "C" {

int minizinc_parse_data_from_string(MiniZinc::MznSolver* env_ptr, MiniZincModel* model_ptr, const char* data_str, const char* filename) {
    MiniZinc::GCLock lock;

    MiniZinc::Env env(nullptr, std::cerr, std::cerr);
    MiniZinc::Model* model = reinterpret_cast<MiniZinc::Model*>(model_ptr);

    std::string data_s(data_str);
    std::string filename_s(filename ? filename : "");

    std::vector<std::string> datafiles_vec;

    // Define a temporary directory within the project's build directory or a known writable location
    // For Termux, /data/data/com.termux/files/home/storage/github/libminizinc/temp/ is a good candidate
    std::string temp_dir = "/data/data/com.termux/files/home/storage/github/libminizinc/temp/";
    // Create the temporary directory if it doesn't exist
    mkdir(temp_dir.c_str(), 0700); // 0700 for owner read/write/execute

    std::string temp_dzn_path_template = temp_dir + "minizinc_data_XXXXXX.dzn"; // Template for mkstemp
    std::vector<char> temp_dzn_path_buffer(temp_dzn_path_template.begin(), temp_dzn_path_template.end());
    temp_dzn_path_buffer.push_back('\0'); // Null-terminate

    int fd = mkstemp(temp_dzn_path_buffer.data());
    if (fd == -1) {
        std::cerr << "Error: Could not create temporary file for MiniZinc data (mkstemp failed). Errno: " << errno << std::endl;
        return -1;
    }
    std::string temp_dzn_path(temp_dzn_path_buffer.data());

    // Write data to the temporary file
    std::ofstream temp_ofs(temp_dzn_path);
    if (!temp_ofs.is_open()) {
        std::cerr << "Error: Could not open temporary file for writing." << std::endl;
        close(fd);
        remove(temp_dzn_path.c_str());
        return -1;
    }
    temp_ofs << data_s;
    temp_ofs.close();
    close(fd); // Close the file descriptor

    datafiles_vec.push_back(temp_dzn_path);

    std::vector<std::string> includePaths_vec;
    bool isFlatZinc = false;
    bool ignoreStdlib = false;
    bool parseDocComments = false;
    bool verbose = true;
    std::ostream& err_stream = std::cerr;

    int result = 0;
    try {
        MiniZinc::parse_data(env, model, datafiles_vec, includePaths_vec, isFlatZinc, ignoreStdlib, parseDocComments, verbose, err_stream);
    } catch (const MiniZinc::Exception& e) {
        std::cerr << "MiniZinc data parsing error (captured): ";
        e.print(std::cerr);
        result = -1;
    } catch (const std::exception& e) {
        std::cerr << "Standard exception during data parsing (captured): " << e.what() << std::endl;
        result = -1;
    } catch (...) {
        std::cerr << "Unknown exception during data parsing (captured)." << std::endl;
        result = -1;
    }

    // Clean up the temporary file
    remove(temp_dzn_path.c_str());

    return result;
}

} // extern "C"
