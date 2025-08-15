#pragma once

#include <minizinc/flattener.hh>
#include <minizinc/solver_config.hh>
#include <iostream>

namespace MiniZinc {

class MznSolver; // Forward declaration

class MiniZincEnvWrapper {
private:
    std::ostream& _os;
    std::ostream& _log;
    SolverConfigs _solverConfigs;
    Flattener _flattener;
    MznSolver* _mznSolver; // Pointer to MznSolver
    bool _verbose; // New member for verbose flag

public:
    MiniZincEnvWrapper(std::ostream& os, std::ostream& log, const std::string& stdlibDir, bool verbose = false);

    Flattener* getFlattener();
    SolverConfigs& getSolverConfigs();
    void setMznSolver(MznSolver* solver); // Setter for MznSolver
};

} // namespace MiniZinc
