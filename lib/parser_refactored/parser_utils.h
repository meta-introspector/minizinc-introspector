#ifndef MINIZINC_PARSER_UTILS_H
#define MINIZINC_PARSER_UTILS_H

#include <string>
#include <fstream>

namespace MiniZinc {

// Function to read file contents into a string
std::string get_file_contents(std::ifstream& in);

} // namespace MiniZinc

#endif // MINIZINC_PARSER_UTILS_H
