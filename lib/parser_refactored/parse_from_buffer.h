#ifndef MINIZINC_PARSE_FROM_BUFFER_H
#define MINIZINC_PARSE_FROM_BUFFER_H

#include <minizinc/parser.hh>
#include <string>
#include <vector>
#include <unordered_set>
#include <iostream>

namespace MiniZinc {

void parse_from_buffer(Env& env, Model*& model, const std::string& modelString,
                       const std::string& modelStringName, const std::vector<std::string>& includePaths,
                       std::unordered_set<std::string> globalInc, bool isFlatZinc, bool ignoreStdlib,
                       bool parseDocComments, bool verbose, std::ostream& err);

} // namespace MiniZinc

#endif // MINIZINC_PARSE_FROM_BUFFER_H
