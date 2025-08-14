#include "minizinc_env_wrapper.h"

namespace MiniZinc {

MiniZincEnvWrapper::MiniZincEnvWrapper(std::ostream& os, std::ostream& log, std::string stdlibDir)
    : _solverConfigs(log),
      _flattener(os, log, _solverConfigs.mznlibDir()) {
    std::cerr << "DEBUG: MiniZincEnvWrapper - mznlibDir: " << _solverConfigs.mznlibDir() << std::endl; std::cerr.flush();
}

MiniZinc::Flattener* MiniZincEnvWrapper::getFlattener() {
    return &_flattener;
}

} // namespace MiniZinc
