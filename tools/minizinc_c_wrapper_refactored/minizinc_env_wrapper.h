#pragma once

#include <minizinc/flattener.hh>
#include <minizinc/solver.hh>
#include <iostream>
#include <string>

namespace MiniZinc {

class MiniZincEnvWrapper {
public:
    MiniZinc::SolverInitialiser _solverInitialiser;
    MiniZinc::SolverConfigs _solverConfigs;
    MiniZinc::Flattener _flattener;

    MiniZincEnvWrapper(std::ostream& os, std::ostream& log, std::string stdlibDir);

    MiniZinc::Flattener* getFlattener();
};

} // namespace MiniZinc
