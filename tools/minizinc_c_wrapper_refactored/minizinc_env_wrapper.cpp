#include "minizinc_env_wrapper.h"
#include <minizinc/solver.hh> // For MznSolver

namespace MiniZinc {

MiniZincEnvWrapper::MiniZincEnvWrapper(std::ostream& os, std::ostream& log, const std::string& stdlibDir, bool verbose)
    : _os(os),
      _log(log),
      _solverConfigs(log),
      _flattener(os, log, _solverConfigs.mznlibDir()),
      _mznSolver(nullptr), // Initialize to nullptr
      _verbose(verbose)
{
    // The stdlibDir passed here is currently unused by Flattener, as it gets it from SolverConfigs
    // We might need to explicitly set it on SolverConfigs if it's not picked up from env var
    std::cerr << "DEBUG: MiniZincEnvWrapper constructor - mznlibDir from SolverConfigs: " << _solverConfigs.mznlibDir() << std::endl; std::cerr.flush();
    if (_verbose) {
        std::cerr << "DEBUG: MiniZincEnvWrapper constructor - Verbose mode enabled." << std::endl; std::cerr.flush();
    }
}

Flattener* MiniZincEnvWrapper::getFlattener() {
    return &_flattener;
}

SolverConfigs& MiniZincEnvWrapper::getSolverConfigs() {
    return _solverConfigs;
}

void MiniZincEnvWrapper::setMznSolver(MznSolver* solver) {
    _mznSolver = solver;
}

} // namespace MiniZinc