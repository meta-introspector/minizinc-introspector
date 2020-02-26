// * -*- mode: C++; c-basic-offset: 2; indent-tabs-mode: nil -*- */

/*
 *  Main authors:
 *     Gleb Belov <gleb.belov@monash.edu>
 */

/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

#ifdef _MSC_VER 
#define _CRT_SECURE_NO_WARNINGS
#endif

#include <iostream>
#include <fstream>
#include <sstream>
#include <iomanip>
#include <string>
#include <cstring>
#include <cmath>
#include <stdexcept>

#include <minizinc/config.hh>
#include <minizinc/utils.hh>
#include <minizinc/file_utils.hh>

#ifdef CPLEX_PLUGIN
#ifdef HAS_DLFCN_H
#include <dlfcn.h>
#elif defined HAS_WINDOWS_H
#include <Windows.h>
#endif
#endif

using namespace std;

#include <minizinc/solvers/MIP/MIP_cplex_wrap.hh>

#ifdef CPLEX_PLUGIN

namespace {
  void* dll_open(const std::string& file) {
#ifdef HAS_DLFCN_H
    if (MiniZinc::FileUtils::is_absolute(file)) {
      return dlopen( file.c_str(), RTLD_NOW);
    }
    if (void* so = dlopen( ("lib"+file+".so").c_str(), RTLD_NOW)) {
      return so;
    }
    return dlopen( ("lib"+file+".jnilib").c_str(), RTLD_NOW);
#else
    if (MiniZinc::FileUtils::is_absolute(file)) {
      return LoadLibrary(file.c_str());
    } else {
      return LoadLibrary((file+".dll").c_str());
    }
#endif
  }
  void* dll_sym(void* dll, const char* sym) {
#ifdef HAS_DLFCN_H
    void* ret = dlsym(dll, sym);
#else
    void* ret = GetProcAddress((HMODULE)dll, sym);
#endif
    if (ret==NULL)
      throw MiniZinc::InternalError("cannot load symbol "+string(sym)+" from CPLEX dll");
    return ret;
  }
  void dll_close(void* dll) {
#ifdef HAS_DLFCN_H
    dlclose(dll);
#else
    FreeLibrary((HMODULE)dll);
#endif
  }
}

#endif

const vector<string>& CPLEXDLLs(void) {
  static const vector<string> sCPLEXDLLs = { "cplex12100", "cplex1290", "cplex1280", "cplex1270" };
  return sCPLEXDLLs;
}

void MIP_cplex_wrapper::checkDLL() {
#ifdef CPLEX_PLUGIN
  _cplex_dll = NULL;
  if ( options->sCPLEXDLL.size() ) {
    _cplex_dll = dll_open( options->sCPLEXDLL.c_str() );
  } else {
    for( const auto& s: CPLEXDLLs() ) {
      _cplex_dll = dll_open( s.c_str() );
      if ( NULL!=_cplex_dll ) {
        break;
      }
    }
  }
  
  if (_cplex_dll==NULL) {
    if (options->sCPLEXDLL.empty()) {
      throw MiniZinc::InternalError("cannot load cplex dll, specify --cplex-dll");
    } else {
      throw MiniZinc::InternalError("cannot load cplex dll `"+options->sCPLEXDLL+"'");
    }
  }

  *(void**)(&dll_CPXaddfuncdest) = dll_sym(_cplex_dll, "CPXaddfuncdest");
  *(void**)(&dll_CPXaddindconstr) = dll_sym(_cplex_dll, "CPXaddindconstr");
  *(void**)(&dll_CPXaddlazyconstraints) = dll_sym(_cplex_dll, "CPXaddlazyconstraints");
  *(void**)(&dll_CPXaddmipstarts) = dll_sym(_cplex_dll, "CPXaddmipstarts");
  *(void**)(&dll_CPXaddrows) = dll_sym(_cplex_dll, "CPXaddrows");
  *(void**)(&dll_CPXaddusercuts) = dll_sym(_cplex_dll, "CPXaddusercuts");
  *(void**)(&dll_CPXchgbds) = dll_sym(_cplex_dll, "CPXchgbds");
  *(void**)(&dll_CPXchgmipstarts) = dll_sym(_cplex_dll, "CPXchgmipstarts");
  *(void**)(&dll_CPXchgobjsen) = dll_sym(_cplex_dll, "CPXchgobjsen");
  *(void**)(&dll_CPXcloseCPLEX) = dll_sym(_cplex_dll, "CPXcloseCPLEX");
  *(void**)(&dll_CPXcreateprob) = dll_sym(_cplex_dll, "CPXcreateprob");
  *(void**)(&dll_CPXcutcallbackadd) = dll_sym(_cplex_dll, "CPXcutcallbackadd");
  *(void**)(&dll_CPXfreeprob) = dll_sym(_cplex_dll, "CPXfreeprob");
  *(void**)(&dll_CPXgetbestobjval) = dll_sym(_cplex_dll, "CPXgetbestobjval");
  *(void**)(&dll_CPXgetcallbackincumbent) = dll_sym(_cplex_dll, "CPXgetcallbackincumbent");
  *(void**)(&dll_CPXgetcallbackinfo) = dll_sym(_cplex_dll, "CPXgetcallbackinfo");
  *(void**)(&dll_CPXgetcallbacknodeinfo) = dll_sym(_cplex_dll, "CPXgetcallbacknodeinfo");
  *(void**)(&dll_CPXgetcallbacknodex) = dll_sym(_cplex_dll, "CPXgetcallbacknodex");
  *(void**)(&dll_CPXgetchannels) = dll_sym(_cplex_dll, "CPXgetchannels");
  *(void**)(&dll_CPXgetdettime) = dll_sym(_cplex_dll, "CPXgetdettime");
  *(void**)(&dll_CPXgeterrorstring) = dll_sym(_cplex_dll, "CPXgeterrorstring");
  *(void**)(&dll_CPXgetmipstartindex) = dll_sym(_cplex_dll, "CPXgetmipstartindex");
  *(void**)(&dll_CPXgetnodecnt) = dll_sym(_cplex_dll, "CPXgetnodecnt");
  *(void**)(&dll_CPXgetnodeleftcnt) = dll_sym(_cplex_dll, "CPXgetnodeleftcnt");
  *(void**)(&dll_CPXgetnumcols) = dll_sym(_cplex_dll, "CPXgetnumcols");
  *(void**)(&dll_CPXgetnumrows) = dll_sym(_cplex_dll, "CPXgetnumrows");
  *(void**)(&dll_CPXgetobjsen) = dll_sym(_cplex_dll, "CPXgetobjsen");
  *(void**)(&dll_CPXgetobjval) = dll_sym(_cplex_dll, "CPXgetobjval");
  *(void**)(&dll_CPXgetsolnpoolnumsolns) = dll_sym(_cplex_dll, "CPXgetsolnpoolnumsolns");
  *(void**)(&dll_CPXgetstat) = dll_sym(_cplex_dll, "CPXgetstat");
  *(void**)(&dll_CPXgetstatstring) = dll_sym(_cplex_dll, "CPXgetstatstring");
  *(void**)(&dll_CPXgettime) = dll_sym(_cplex_dll, "CPXgettime");
  *(void**)(&dll_CPXgetx) = dll_sym(_cplex_dll, "CPXgetx");
  *(void**)(&dll_CPXmipopt) = dll_sym(_cplex_dll, "CPXmipopt");
  *(void**)(&dll_CPXnewcols) = dll_sym(_cplex_dll, "CPXnewcols");
  *(void**)(&dll_CPXopenCPLEX) = dll_sym(_cplex_dll, "CPXopenCPLEX");
  *(void**)(&dll_CPXreadcopyparam) = dll_sym(_cplex_dll, "CPXreadcopyparam");
  *(void**)(&dll_CPXsetdblparam) = dll_sym(_cplex_dll, "CPXsetdblparam");
  *(void**)(&dll_CPXsetinfocallbackfunc) = dll_sym(_cplex_dll, "CPXsetinfocallbackfunc");
  *(void**)(&dll_CPXsetintparam) = dll_sym(_cplex_dll, "CPXsetintparam");
  *(void**)(&dll_CPXsetstrparam) = dll_sym(_cplex_dll, "CPXsetstrparam");
  *(void**)(&dll_CPXsetlazyconstraintcallbackfunc) = dll_sym(_cplex_dll, "CPXsetlazyconstraintcallbackfunc");
  *(void**)(&dll_CPXsetusercutcallbackfunc) = dll_sym(_cplex_dll, "CPXsetusercutcallbackfunc");
  *(void**)(&dll_CPXversion) = dll_sym(_cplex_dll, "CPXversion");
  *(void**)(&dll_CPXwriteparam) = dll_sym(_cplex_dll, "CPXwriteparam");
  *(void**)(&dll_CPXwriteprob) = dll_sym(_cplex_dll, "CPXwriteprob");
  
#else
  
  dll_CPXaddfuncdest = CPXaddfuncdest;
  dll_CPXaddindconstr = CPXaddindconstr;
  dll_CPXaddlazyconstraints = CPXaddlazyconstraints;
  dll_CPXaddmipstarts = CPXaddmipstarts;
  dll_CPXaddrows = CPXaddrows;
  dll_CPXaddusercuts = CPXaddusercuts;
  dll_CPXchgbds = CPXchgbds;
  dll_CPXchgmipstarts = CPXchgmipstarts;
  dll_CPXchgobjsen = CPXchgobjsen;
  dll_CPXcloseCPLEX = CPXcloseCPLEX;
  dll_CPXcreateprob = CPXcreateprob;
  dll_CPXcutcallbackadd = CPXcutcallbackadd;
  dll_CPXfreeprob = CPXfreeprob;
  dll_CPXgetbestobjval = CPXgetbestobjval;
  dll_CPXgetcallbackincumbent = CPXgetcallbackincumbent;
  dll_CPXgetcallbackinfo = CPXgetcallbackinfo;
  dll_CPXgetcallbacknodeinfo = CPXgetcallbacknodeinfo;
  dll_CPXgetcallbacknodex = CPXgetcallbacknodex;
  dll_CPXgetchannels = CPXgetchannels;
  dll_CPXgetdettime = CPXgetdettime;
  dll_CPXgeterrorstring = CPXgeterrorstring;
  dll_CPXgetmipstartindex = CPXgetmipstartindex;
  dll_CPXgetnodecnt = CPXgetnodecnt;
  dll_CPXgetnodeleftcnt = CPXgetnodeleftcnt;
  dll_CPXgetnumcols = CPXgetnumcols;
  dll_CPXgetnumrows = CPXgetnumrows;
  dll_CPXgetobjsen = CPXgetobjsen;
  dll_CPXgetobjval = CPXgetobjval;
  dll_CPXgetsolnpoolnumsolns = CPXgetsolnpoolnumsolns;
  dll_CPXgetstat = CPXgetstat;
  dll_CPXgetstatstring = CPXgetstatstring;
  dll_CPXgettime = CPXgettime;
  dll_CPXgetx = CPXgetx;
  dll_CPXmipopt = CPXmipopt;
  dll_CPXnewcols = CPXnewcols;
  dll_CPXopenCPLEX = CPXopenCPLEX;
  dll_CPXreadcopyparam = CPXreadcopyparam;
  dll_CPXsetdblparam = CPXsetdblparam;
  dll_CPXsetinfocallbackfunc = CPXsetinfocallbackfunc;
  dll_CPXsetintparam = CPXsetintparam;
  dll_CPXsetstrparam = CPXsetstrparam;
  dll_CPXsetlazyconstraintcallbackfunc = CPXsetlazyconstraintcallbackfunc;
  dll_CPXsetusercutcallbackfunc = CPXsetusercutcallbackfunc;
  dll_CPXversion = CPXversion;
  dll_CPXwriteparam = CPXwriteparam;
  dll_CPXwriteprob = CPXwriteprob;
  
#endif
}


string MIP_cplex_wrapper::getDescription(MiniZinc::SolverInstanceBase::Options* opt) {
  string v = "MIP wrapper for IBM ILOG CPLEX  ";
  int status;
  Options def_options;
  Options* options = opt ? static_cast<Options*>(opt) : &def_options;
  try {
    MIP_cplex_wrapper mcw(options);
    CPXENVptr env = mcw.dll_CPXopenCPLEX (&status);
    if (env) {
      v += mcw.dll_CPXversion (env);
      status = mcw.dll_CPXcloseCPLEX (&env);
    } else
      v += "[?? ...cannot open CPLEX env to query version]";
  } catch (MiniZinc::InternalError&) {
    v += "[?? ...cannot open CPLEX env to query version]";
  }
  v += "  Compiled  " __DATE__ "  " __TIME__;
  return v;
}

string MIP_cplex_wrapper::getVersion(MiniZinc::SolverInstanceBase::Options* opt) {
  string v;
  int status;
  Options def_options;
  Options* options = opt ? static_cast<Options*>(opt) : &def_options;
  try {
    MIP_cplex_wrapper mcw(options);
    CPXENVptr env = mcw.dll_CPXopenCPLEX (&status);
    if (env) {
      v += mcw.dll_CPXversion (env);
      status = mcw.dll_CPXcloseCPLEX (&env);
    } else {
      v += "<unknown version>";
    }
  } catch (MiniZinc::InternalError&) {
    v += "<unknown version>";
  }
  return v;
}

vector<string> MIP_cplex_wrapper::getRequiredFlags(void) {
  int status;
  Options options;
  try {
    MIP_cplex_wrapper mcw(&options);
    CPXENVptr env = mcw.dll_CPXopenCPLEX (&status);
    if (env) {
      return {};
    }
  } catch (MiniZinc::InternalError&) {
  }
  return { "--cplex-dll" };
}

string MIP_cplex_wrapper::getId() {
  return "cplex";
}

string MIP_cplex_wrapper::getName() {
  return "CPLEX";
}

vector<string> MIP_cplex_wrapper::getTags() {
  return {"mip","float","api"};
}

vector<string> MIP_cplex_wrapper::getStdFlags() {
  return {"-a", "-n", "-p", "-s", "-v"};
}

void MIP_cplex_wrapper::Options::printHelp(ostream& os) {
  os
  << "IBM ILOG CPLEX  MIP wrapper options:" << std::endl
  // -s                  print statistics
  //            << "  --readParam <file>  read CPLEX parameters from file
  //               << "--writeParam <file> write CPLEX parameters to file
  //               << "--tuneParam         instruct CPLEX to tune parameters instead of solving
  << "  --mipfocus <n>\n    1: feasibility, 2: optimality, 3: move bound (default is 0, balanced)" << std::endl
  << "  -a\n    print intermediate solutions (use for optimization problems only TODO)" << std::endl
  << "  -p <N>\n    use N threads, default: 1" << std::endl
//   << "  --nomippresolve     disable MIP presolving   NOT IMPL" << std::endl
  << "  --solver-time-limit <N>\n    stop search after N milliseconds wall time" << std::endl
  << "  -n <N>, --num-solutions <N>\n"
     "    stop search after N solutions" << std::endl
  << "  -r <N>, --random-seed <N>\n"
     "    random seed, integer" << std::endl
  << "  --workmem <N>, --nodefilestart <N>\n"
     "    maximal RAM for working memory used before writing to node file, GB, default: 0.5" << std::endl
  << "  --nodefiledir <path>\n"
     "    nodefile directory" << std::endl
  << "  --writeModel <file>\n    write model to <file> (.lp, .mps, .sav, ...)" << std::endl
  << "  --readParam <file>\n    read CPLEX parameters from file" << std::endl
  << "  --writeParam <file>\n    write CPLEX parameters to file" << std::endl
//   << "  --tuneParam         instruct CPLEX to tune parameters instead of solving   NOT IMPL"

  << "  --absGap <n>\n    absolute gap |primal-dual| to stop" << std::endl
  << "  --relGap <n>\n    relative gap |primal-dual|/<solver-dep> to stop. Default 1e-8, set <0 to use backend's default" << std::endl
  << "  --intTol <n>\n    integrality tolerance for a variable. Default 1e-8" << std::endl
  << "\n  --cplex-dll <file> or <basename>\n    CPLEX DLL, or base name, such as cplex1280, when using plugin. Default range tried: "
  << CPLEXDLLs().front() << " .. " << CPLEXDLLs().back() << std::endl
//   << "  --objDiff <n>       objective function discretization. Default 1.0" << std::endl

  << std::endl;
}

bool MIP_cplex_wrapper::Options::processOption(int& i, std::vector<std::string>& argv) {
  MiniZinc::CLOParser cop( i, argv );
  if ( string(argv[i])=="-a"
      || string(argv[i])=="--all"
      || string(argv[i])=="--all-solutions" ) {
    flag_all_solutions = true;
  } else if (string(argv[i])=="-f") {
//     std::cerr << "  Flag -f: ignoring fixed strategy anyway." << std::endl;
  } else if ( cop.get( "--mipfocus --mipFocus --MIPFocus --MIPfocus", &nMIPFocus ) ) {
  } else if ( cop.get( "--writeModel", &sExportModel ) ) {
  } else if ( cop.get( "-p", &nThreads ) ) {
  } else if ( cop.get( "--solver-time-limit", &nTimeout ) ) {
  } else if ( cop.get( "-n --num-solutions", &nSolLimit ) ) {
  } else if ( cop.get( "-r --random-seed", &nSeed ) ) {
  } else if ( cop.get( "--workmem --nodefilestart", &nWorkMemLimit ) ) {
  } else if ( cop.get( "--nodefiledir --NodefileDir", &sNodefileDir ) ) {
  } else if ( cop.get( "--readParam", &sReadParams ) ) {
  } else if ( cop.get( "--writeParam", &sWriteParams ) ) {
  } else if ( cop.get( "--absGap", &absGap ) ) {
  } else if ( cop.get( "--relGap", &relGap ) ) {
  } else if ( cop.get( "--intTol", &intTol ) ) {
  } else if ( cop.get( "--cplex-dll", &sCPLEXDLL ) ) {
//   } else if ( cop.get( "--objDiff", &objDiff ) ) {
  } else
    return false;
  return true;
}

void MIP_cplex_wrapper::wrap_assert(bool cond, string msg, bool fTerm)
{
   if ( !cond ) {
      strcpy(cplex_buffer, "[NO ERROR STRING GIVEN]");
      dll_CPXgeterrorstring (env, status, cplex_buffer);
      string msgAll = ("  MIP_cplex_wrapper runtime error:  " + msg + "  " + cplex_buffer);
      cerr << msgAll << endl;
      if (fTerm) {
        cerr << "TERMINATING." << endl;
        throw runtime_error(msgAll);
      }
   }
}

void MIP_cplex_wrapper::openCPLEX()
{
  checkDLL();
  cbui.wrapper = this;
  /// Cleanup first.
//   cleanup();
   /* Initialize the CPLEX environment */
   env = dll_CPXopenCPLEX (&status);
   /* If an error occurs, the status value indicates the reason for
      failure.  A call to CPXgeterrorstring will produce the text of
      the error message.  Note that CPXopenCPLEX produces no output,
      so the only way to see the cause of the error is to use
      CPXgeterrorstring.  For other CPLEX routines, the errors will
      be seen if the CPXPARAM_ScreenOutput indicator is set to CPX_ON.  */
   wrap_assert ( env, "Could not open CPLEX environment." );
   /* Create the problem. */
   lp = dll_CPXcreateprob (env, &status, "MIP_cplex_wrapper");
   /* A returned pointer of NULL may mean that not enough memory
      was available or there was some other problem.  In the case of
      failure, an error message will have been written to the error
      channel from inside CPLEX.  In this example, the setting of
      the parameter CPXPARAM_ScreenOutput causes the error message to
      appear on stdout.  */
   wrap_assert ( lp, "Failed to create LP." );
}

void MIP_cplex_wrapper::closeCPLEX()
{
  /// Freeing the problem can be slow both in C and C++, see IBM forums. Skipping.
     /* Free up the problem as allocated by CPXcreateprob, if necessary */
//    if ( lp != NULL ) {
//       status = CPXfreeprob (env, &lp);
//       cplex_wrap_assert ( !status, "CPXfreeprob failed." );
//    }
  lp = 0;
   /* Free up the CPLEX environment, if necessary */
   if ( env != NULL ) {
      status = dll_CPXcloseCPLEX (&env);
      wrap_assert ( !status, "Could not close CPLEX environment." );
   }
  /// and at last:
//   MIP_wrapper::cleanup();
#ifdef CPLEX_PLUGIN
//  dll_close(cplex_dll);
#endif

}

void MIP_cplex_wrapper::doAddVars
  (size_t n, double* obj, double* lb, double* ub, MIP_wrapper::VarType* vt, string *names)
{
  /// Convert var types:
  vector<char> ctype(n);
  vector<char*> pcNames(n);
  for (size_t i=0; i<n; ++i) {
    pcNames[i] = (char*)names[i].c_str();
    switch (vt[i]) {
      case REAL:
        ctype[i] = CPX_CONTINUOUS;
        break;
      case INT:
        ctype[i] = CPX_INTEGER;
        break;
      case BINARY:
        ctype[i] = CPX_BINARY;
        break;
      default:
        throw runtime_error("  MIP_wrapper: unknown variable type");
    }
  }
  status = dll_CPXnewcols (env, lp, n, obj, lb, ub, &ctype[0], &pcNames[0]);
  wrap_assert( !status,  "Failed to declare variables." );
}

static char getCPLEXConstrSense(MIP_wrapper::LinConType sense) {
    switch (sense) {
      case MIP_wrapper::LQ:
        return 'L';
      case MIP_wrapper::EQ:
        return 'E';
      case MIP_wrapper::GQ:
        return 'G';
      default:
        throw runtime_error("  MIP_cplex_wrapper: unknown constraint type");
    }
}

void MIP_cplex_wrapper::addRow
  (int nnz, int* rmatind, double* rmatval, MIP_wrapper::LinConType sense,
   double rhs, int mask, string rowName)
{
  /// Convert var types:
  char ssense=getCPLEXConstrSense(sense);
  const int ccnt=0;
  const int rcnt=1;
  const int rmatbeg[] = { 0 };
  char * pRName = (char*)rowName.c_str();
  if (MaskConsType_Normal & mask) {
    status = dll_CPXaddrows (env, lp, ccnt, rcnt, nnz, &rhs,
        &ssense, rmatbeg, rmatind, rmatval,
        NULL, &pRName);
    wrap_assert( !status,  "Failed to add constraint." );
  }
  if (MaskConsType_Usercut & mask) {
    status = dll_CPXaddusercuts (env, lp, rcnt, nnz, &rhs,
        &ssense, rmatbeg, rmatind, rmatval,
        &pRName);
    wrap_assert( !status,  "Failed to add usercut." );
  }
  if (MaskConsType_Lazy & mask) {
    status = dll_CPXaddlazyconstraints (env, lp, rcnt, nnz, &rhs,
        &ssense, rmatbeg, rmatind, rmatval,
        &pRName);
    wrap_assert( !status,  "Failed to add lazy constraint." );
  }
}

void MIP_cplex_wrapper::addIndicatorConstraint(
  int iBVar, int bVal, int nnz, int* rmatind, double* rmatval,
  MIP_wrapper::LinConType sense, double rhs, string rowName)
{
  wrap_assert( 0<=bVal && 1>=bVal, "mzn-cplex: addIndicatorConstraint: bVal not 0/1" );
  char ssense=getCPLEXConstrSense(sense);
  status = dll_CPXaddindconstr (env, lp, iBVar, 1-bVal, nnz, rhs,
      ssense, rmatind, rmatval, rowName.c_str());
  wrap_assert( !status,  "Failed to add indicator constraint." );
}

bool MIP_cplex_wrapper::addWarmStart( const std::vector<VarId>& vars, const std::vector<double> vals ) {
  assert( vars.size()==vals.size() );
  const char* sMSName = "MZNMS";
  int msindex=-1;
  const int beg=0;
  /// Check if we already added a start
  status = dll_CPXgetmipstartindex (env, lp, sMSName, &msindex);
  if ( status ) {      // not existent
    // status = dll_CPXaddmipstarts (env, lp, mcnt, nzcnt, beg, varindices,
    //                            values, effortlevel, mipstartname);
    status = dll_CPXaddmipstarts (env, lp, 1, vars.size(), &beg, vars.data(),
                                 vals.data(), nullptr, (char**)&sMSName);
    wrap_assert( !status,  "Failed to add warm start." );
  } else {
    // status = dll_CPXchgmipstarts (env, lp, mcnt, mipstartindices, nzcnt, beg, varindices, values, effortlevel);
    status = dll_CPXchgmipstarts (env, lp, 1, &msindex, vars.size(), &beg, vars.data(),
                                 vals.data(), nullptr);
    wrap_assert( !status,  "Failed to extend warm start." );
  }
  return true;
}

void MIP_cplex_wrapper::setVarBounds(int iVar, double lb, double ub)
{
  wrap_assert( lb<=ub, "mzn-cplex: setVarBounds: lb>ub" );
  char cl = 'L', cu = 'U';
  status = dll_CPXchgbds (env, lp, 1, &iVar, &cl, &lb);
  wrap_assert( !status,  "Failed to set lower bound." );
  status = dll_CPXchgbds (env, lp, 1, &iVar, &cu, &ub);
  wrap_assert( !status,  "Failed to set upper bound." );
}

void MIP_cplex_wrapper::setVarLB(int iVar, double lb)
{
  char cl = 'L';
  status = dll_CPXchgbds (env, lp, 1, &iVar, &cl, &lb);
  wrap_assert( !status,  "Failed to set lower bound." );
}

void MIP_cplex_wrapper::setVarUB(int iVar, double ub)
{
  char cu = 'U';
  status = dll_CPXchgbds (env, lp, 1, &iVar, &cu, &ub);
  wrap_assert( !status,  "Failed to set upper bound." );
}



/// SolutionCallback ------------------------------------------------------------------------
/// CPLEX ensures thread-safety
static int CPXPUBLIC
solcallback (CPXCENVptr env, void *cbdata, int wherefrom, void *cbhandle)
{
   int status = 0;

   MIP_wrapper::CBUserInfo *info = (MIP_wrapper::CBUserInfo*) cbhandle;
   MIP_cplex_wrapper* cw = static_cast<MIP_cplex_wrapper*>(info->wrapper);
   int        hasincumbent = 0;
   int        newincumbent = 0;
   double objVal;

   status = cw->dll_CPXgetcallbackinfo (env, cbdata, wherefrom,
                                CPX_CALLBACK_INFO_NODE_COUNT, &info->pOutput->nNodes);
   if ( status )  goto TERMINATE;

   status = cw->dll_CPXgetcallbackinfo (env, cbdata, wherefrom,
                                CPX_CALLBACK_INFO_NODES_LEFT, &info->pOutput->nOpenNodes);
   if ( status )  goto TERMINATE;

   status = cw->dll_CPXgetcallbackinfo (env, cbdata, wherefrom,
                                CPX_CALLBACK_INFO_MIP_FEAS, &hasincumbent);
   if ( status )  goto TERMINATE;

   if ( hasincumbent ) {
      status = cw->dll_CPXgetcallbackinfo (env, cbdata, wherefrom,
                                   CPX_CALLBACK_INFO_BEST_INTEGER, &objVal);
      if ( status )  goto TERMINATE;
      
      if ( fabs(info->pOutput->objVal - objVal) > 1e-12*(1.0 + fabs(objVal)) ) {
         newincumbent = 1;
         info->pOutput->objVal = objVal;
        info->pOutput->status = MIP_wrapper::SAT;
        info->pOutput->statusName = "feasible from a callback";

      }
   }

//    if ( nodecnt >= info->lastlog + 100  ||  newincumbent ) {
//       double walltime;
//       double dettime;

      status = cw->dll_CPXgetcallbackinfo (env, cbdata, wherefrom,
                                   CPX_CALLBACK_INFO_BEST_REMAINING, &info->pOutput->bestBound);
//       if ( status )  goto TERMINATE;

//       status = dll_CPXgettime (env, &walltime);
//       if ( status )  goto TERMINATE;
// 
//       status = dll_CPXgetdettime (env, &dettime);
//       if ( status )  goto TERMINATE;
// 
//    }

   if ( newincumbent ) {
      assert(info->pOutput->x);
      status = cw->dll_CPXgetcallbackincumbent (env, cbdata, wherefrom,
                                        (double*)info->pOutput->x,
                                        0, info->pOutput->nCols-1);
      if ( status )  goto TERMINATE;

      info->pOutput->dWallTime = std::chrono::duration<double>(
         std::chrono::steady_clock::now() - info->pOutput->dWallTime0).count();
      info->pOutput->dCPUTime = double(std::clock() - info->pOutput->cCPUTime0) / CLOCKS_PER_SEC;

      /// Call the user function:
      if (info->solcbfn)
          (*info->solcbfn)(*info->pOutput, info->psi);
     info->printed = true;
   }
   

TERMINATE:
   return (status);

} /* END logcallback */
// end SolutionCallback ---------------------------------------------------------------------

/// Cut callbacks, mostly copied from admipex5.c, CPLEX 12.6.3
/* The following macro defines the smallest improvement 
   on the value of the objective function that is required
   when adding user cuts from within a callback.
   If the improvement on the value of the ojective function
   is not large enough, the callback will abort the cut loop. */

#define EPSOBJ 0.1

/* The following structure will hold the information we need to 
   pass to the cut callback function */

struct cutinfo {
   CPXLPptr lp;
   int      numcols;
   int      num;
   double   *x;
   int      *beg;
   int      *ind; 
   double   *val;
   double   *rhs;
   int      nodeid;
   double   nodeobjval;
   int      objsen;
   MIP_wrapper::CBUserInfo *info=0;
};
typedef struct cutinfo CUTINFO, *CUTINFOptr;

/* Init information on the node objval for the user cut callback */

static void 
initnodeobjvalinfo (MIP_cplex_wrapper* cw, CPXENVptr env, CPXLPptr lp, CUTINFOptr cutinfo)
{
   cutinfo->nodeid = -1;
   cutinfo->nodeobjval = 0.0;
   cutinfo->objsen = cw->dll_CPXgetobjsen (env, lp);
   if ( cutinfo->objsen == CPX_MIN )
      cutinfo->objsen = 1;
   else
      cutinfo->objsen = -1;
} /* END initnodeobjvalinfo */



static int CPXPUBLIC 
myusercutcallback (CPXCENVptr env,
               void       *cbdata,
               int        wherefrom,
               void       *cbhandle,
               int        *useraction_p)
{
   int status = 0;

   CUTINFOptr cutinfo = (CUTINFOptr) cbhandle;

//   int      numcols  = cutinfo->numcols;
//   int      numcuts  = cutinfo->num;
//    double   *x       = cutinfo->x;
//    int      *beg     = cutinfo->beg;
//    int      *ind     = cutinfo->ind;
//    double   *val     = cutinfo->val;
//    double   *rhs     = cutinfo->rhs;
//    int      *cutind  = NULL;
//    double   *cutval  = NULL;
//   double   cutvio;
   int      addedcuts = 0;
//   int      i, j, k; //, cutnz;
   MIP_wrapper::CBUserInfo *info = cutinfo->info;
   MIP_cplex_wrapper* cw = static_cast<MIP_cplex_wrapper*>(info->wrapper);
//    double   *x       = info->pCutOutput->x;

   *useraction_p = CPX_CALLBACK_DEFAULT; 

   /* If we are called as a user cut callback, decide
      first if we want to add cuts or abort the cut loop.
      When adding user cuts with purgeable flag set to 
      CPX_USECUT_PURGE or CPX_USECUT_FILTER, it is important 
      to avoid the possibility of an infinite cut loop, where 
      the same cuts are added to the LP and then immediately 
      purged at every cut pass. Such a situation can be avoided,
      for instance, by applying a tailing off criterion and aborting 
      the cut loop where no progress in the objval is observed.
      Note, however, that the same approach must not be applied
      with lazy constraints. In this case, if lazy constraints are
      added with purgeable flag set to CPX_USECUT_PURGE, adding
      the same lazy constraint more than once could be required
      to ensure the correctness of the final result. */

   bool fMIPSol=true;
   if ( wherefrom == CPX_CALLBACK_MIP_CUT_LOOP ||
        wherefrom == CPX_CALLBACK_MIP_CUT_LAST   ) {
      int    oldnodeid     = cutinfo->nodeid;
      double oldnodeobjval = cutinfo->nodeobjval;

      fMIPSol = false;
   
      /* Retrieve nodeid and node objval of the current node */

      status = cw->dll_CPXgetcallbacknodeinfo (env, cbdata, wherefrom, 0,
                                       CPX_CALLBACK_INFO_NODE_SEQNUM,
                                       &cutinfo->nodeid);
      if ( status ) {
         fprintf(stderr, "Failed to get node id.\n");
         goto TERMINATE;
      }

      status = cw->dll_CPXgetcallbacknodeinfo (env, cbdata, wherefrom, 0,
                                       CPX_CALLBACK_INFO_NODE_OBJVAL,
                                       &cutinfo->nodeobjval);
      if ( status ) {
         fprintf(stderr, "Failed to get node objval.\n");
         goto TERMINATE;
      }

      /* Abort the cut loop if we are stuck at the same node
         as before and there is no progress in the node objval */

      if ( oldnodeid == cutinfo->nodeid ) {
         double objchg = (cutinfo->nodeobjval - oldnodeobjval);
         /* Multiply objchg by objsen to normalize 
            the change in the objective function to 
            the case of a minimization problem */
         objchg *= cutinfo->objsen;
         if ( objchg <= EPSOBJ ) {
            *useraction_p = CPX_CALLBACK_ABORT_CUT_LOOP;
            goto TERMINATE;
         }
      }
   }

   /* If we reached this point, we are 
      .. in a lazyconstraint callback, or 
      .. in a user cut callback, and cuts seem to help 
         improving the node objval. 
      In both cases, we retrieve the x solution and 
      look for violated cuts. */

    if ( info->cutcbfn ) {    // if cut handler given
      MIP_wrapper::Output outpRlx;
      outpRlx.x = info->pOutput->x;  // using the sol output storage  TODO?
      outpRlx.nCols = info->pOutput->nCols;
      assert( outpRlx.x && outpRlx.nCols );
      status = cw->dll_CPXgetcallbacknodex (env, cbdata, wherefrom, (double*)outpRlx.x,
                                    0, outpRlx.nCols-1); 
      if ( status ) {
          fprintf(stderr, "Cut callback: failed to get node solution.\n");
          goto TERMINATE;
      }
      MIP_wrapper::CutInput cutInput;
      info->cutcbfn( outpRlx, cutInput, info->psi, fMIPSol );
      static int nCuts=0;
      nCuts += cutInput.size();
      // if ( cutInput.size() )
      //  cerr << "\n   N CUTS:  " << nCuts << endl;
      for ( auto& cd : cutInput ) {
        if ( ! ( cd.mask &
          (MIP_wrapper::MaskConsType_Usercut|MIP_wrapper::MaskConsType_Lazy) ) )
          throw runtime_error( "Cut callback: should be user/lazy" );
        /* Use a cut violation tolerance of 0.01 */
        if ( true ) {  //cutvio > 0.01 ) { 
          status = cw->dll_CPXcutcallbackadd (env, cbdata, wherefrom,
                       cd.rmatind.size(), cd.rhs, getCPLEXConstrSense(cd.sense),
                                      cd.rmatind.data(), cd.rmatval.data(),
                                      CPX_USECUT_FORCE);   // PURGE?
          if ( status ) {
              fprintf (stderr, "CPLEX callback: failed to add cut.\n");
              goto TERMINATE;
          }
          addedcuts++;
        }
      }
   }

   /* Tell CPLEX that cuts have been created */ 
   if ( addedcuts > 0 ) {
      *useraction_p = CPX_CALLBACK_SET; 
   }

TERMINATE:

   return (status);

} /* END myusercutcallback */

// ----------------- END Cut callbacks ------------------


MIP_cplex_wrapper::Status MIP_cplex_wrapper::convertStatus(int cplexStatus)
{
  Status s = Status::UNKNOWN;
   /* Converting the status. */
   switch(cplexStatus) {
     case CPXMIP_OPTIMAL:
       s = Status::OPT;
       wrap_assert(dll_CPXgetsolnpoolnumsolns(env, lp), "Optimality reported but pool empty?", false);
       break;
     case CPXMIP_INFEASIBLE:
       s = Status::UNSAT;
       break;
//      case CPXMIP_OPTIMAL_INFEAS:
     case CPXMIP_INForUNBD:
       s = Status::UNSATorUNBND;
       break;
     case CPXMIP_SOL_LIM:
     case CPXMIP_NODE_LIM_FEAS:
     case CPXMIP_TIME_LIM_FEAS:
     case CPXMIP_FAIL_FEAS:
     case CPXMIP_MEM_LIM_FEAS:
     case CPXMIP_ABORT_FEAS:
     case CPXMIP_FAIL_FEAS_NO_TREE:
       s = Status::SAT;
       wrap_assert(dll_CPXgetsolnpoolnumsolns(env, lp), "Feasibility reported but pool empty?", false);
       break;
     case CPXMIP_UNBOUNDED:
       s = Status::UNBND;
       break;
//      case CPXMIP_ABORT_INFEAS:
     case CPXMIP_FAIL_INFEAS:
       s = Status::__ERROR;
       break;
     default:
//      case CPXMIP_OPTIMAL_TOL:
//      case CPXMIP_ABORT_RELAXATION_UNBOUNDED:
       if (dll_CPXgetsolnpoolnumsolns (env, lp))
         s = Status::SAT;
       else
        s = Status::UNKNOWN;
   }
   return s;
}

void msgfunction(void *handle, const char *msg_string)
{
  cerr << msg_string << flush;
}

void MIP_cplex_wrapper::solve() {  // Move into ancestor?

  /////////////// Last-minute solver options //////////////////
  if ( options->flag_all_solutions && 0==nProbType )
    cerr << "WARNING. --all-solutions for SAT problems not implemented." << endl;
  // Before all manual params ???
  if (options->sReadParams.size()) {
    status = dll_CPXreadcopyparam (env, options->sReadParams.c_str());
    wrap_assert(!status, "Failed to read CPLEX parameters.", false);
  }
  
  /* Turn on output to the screen */
   if (fVerbose) {
     CPXCHANNELptr chnl[4];
     dll_CPXgetchannels(env, &chnl[0], &chnl[1], &chnl[2], &chnl[3]);
     for (int i = 0; i < 3; ++i) {
       status = dll_CPXaddfuncdest(env, chnl[i], nullptr, msgfunction);
     }
//     status = dll_CPXsetintparam(env, CPXPARAM_ScreenOutput,
//       fVerbose ? CPX_ON : CPX_OFF);  // also when flag_all_solutions?  TODO
//     wrap_assert(!status, "  CPLEX Warning: Failure to switch screen indicator.", false);
   }
   status = dll_CPXsetintparam (env, CPXPARAM_MIP_Display,
                            fVerbose ? 2 : 0);  // also when flag_all_solutions?  TODO
   wrap_assert(!status, "  CPLEX Warning: Failure to switch logging.", false);
   /// Make it wall time by default, 12.8
//    status =  dll_CPXsetintparam (env, CPXPARAM_ClockType, 1);            // CPU time
//    wrap_assert(!status, "  CPLEX Warning: Failure to measure CPU time.", false);
   status =  dll_CPXsetintparam (env, CPX_PARAM_MIPCBREDLP, CPX_OFF);    // Access original model
   wrap_assert(!status, "  CPLEX Warning: Failure to set access original model in callbacks.", false);
   if (options->sExportModel.size()) {
     status = dll_CPXwriteprob (env, lp, options->sExportModel.c_str(), NULL);
     wrap_assert(!status, "Failed to write LP to disk.", false);
   }

   /// TODO
//     if(all_solutions && obj.getImpl()) {
//       IloNum lastObjVal = (obj.getSense() == IloObjective::Minimize ) ?
//       _ilocplex->use(SolutionCallback(_iloenv, lastObjVal, *this));
      // Turn off CPLEX logging

   if (options->nThreads>0) {
     status =  dll_CPXsetintparam (env, CPXPARAM_Threads, options->nThreads);
     wrap_assert(!status, "Failed to set CPXPARAM_Threads.", false);
   }

   if (options->nTimeout>0) {
     status =  dll_CPXsetdblparam (env, CPXPARAM_TimeLimit, static_cast<double>(options->nTimeout)/1000.0);
     wrap_assert(!status, "Failed to set CPXPARAM_TimeLimit.", false);
   }
   if (options->nSolLimit>0) {
     status =  dll_CPXsetintparam (env, CPXPARAM_MIP_Limits_Solutions, options->nSolLimit);
     wrap_assert(!status, "Failed to set CPXPARAM_MIP_Limits_Solutions.", false);
   }
   if (options->nSeed>=0) {
     status =  dll_CPXsetintparam (env, CPXPARAM_RandomSeed, options->nSeed);
     wrap_assert(!status, "Failed to set CPXPARAM_RandomSeed.", false);
   }
   if (options->nMIPFocus>0) {
     status =  dll_CPXsetintparam (env, CPXPARAM_Emphasis_MIP, options->nMIPFocus);
     wrap_assert(!status, "Failed to set CPXPARAM_Emphasis_MIP.", false);
   }
   
    
    if (options->nWorkMemLimit>0) {
     status =  dll_CPXsetintparam (env, CPXPARAM_MIP_Strategy_File, 3);
     wrap_assert(!status, "Failed to set CPXPARAM_MIP_Strategy_File.", false);
     status =  dll_CPXsetdblparam (env, CPXPARAM_WorkMem, 1024.0 * options->nWorkMemLimit);   // MB in CPLEX
     wrap_assert(!status, "Failed to set CPXPARAM_WorkMem.", false);
    }

    if (options->sNodefileDir.size()>0) {
     status =  dll_CPXsetstrparam (env, CPXPARAM_WorkDir, options->sNodefileDir.c_str());
     wrap_assert(!status, "Failed to set CPXPARAM_WorkDir.", false);
    }

   if ( options->absGap>=0.0 ) {
    status =  dll_CPXsetdblparam (env, CPXPARAM_MIP_Tolerances_AbsMIPGap, options->absGap);
    wrap_assert(!status, "Failed to set CPXPARAM_MIP_Tolerances_AbsMIPGap.", false);
   }
   if (options->relGap>=0.0) {
    status =  dll_CPXsetdblparam (env, CPXPARAM_MIP_Tolerances_MIPGap, options->relGap);
    wrap_assert(!status, "Failed to set CPXPARAM_MIP_Tolerances_MIPGap.", false);
   }
   if (options->intTol>=0.0) {
    status =  dll_CPXsetdblparam (env, CPXPARAM_MIP_Tolerances_Integrality, options->intTol);
    wrap_assert(!status, "Failed to set CPXPARAM_MIP_Tolerances_Integrality.", false);
   }

//    status =  dll_CPXsetdblparam (env, CPXPARAM_MIP_Tolerances_ObjDifference, objDiff);
//    wrap_assert(!status, "Failed to set CPXPARAM_MIP_Tolerances_ObjDifference.", false);

    
   /// Solution callback
   output.nCols = colObj.size();
   x.resize(output.nCols);
   output.x = &x[0];
   if (options->flag_all_solutions && cbui.solcbfn) {
      status = dll_CPXsetinfocallbackfunc (env, solcallback, &cbui);
      wrap_assert(!status, "Failed to set solution callback", false);
   }
  if ( cbui.cutcbfn ) {
    assert( cbui.cutMask & (MaskConsType_Usercut|MaskConsType_Lazy) );
    if ( cbui.cutMask & MaskConsType_Usercut ) {
      // For user cuts, needs to keep some info after presolve
      if ( fVerbose )
        cerr << "  MIP_cplex_wrapper: user cut callback enabled, setting params" << endl;
      CUTINFO usercutinfo;  // THREADS?  TODO
      usercutinfo.info = &cbui;
      /* Init information on the node objval for the user cut callback */
      initnodeobjvalinfo (this, env, lp, &usercutinfo);
      /* Assure linear mappings between the presolved and original
          models */
      status = dll_CPXsetintparam (env, CPXPARAM_Preprocessing_Linear, 0);
      wrap_assert ( !status, "CPLEX: setting prepro_linear" );
      /* Turn on traditional search for use with control callbacks */
      status = dll_CPXsetintparam (env, CPXPARAM_MIP_Strategy_Search,
                                CPX_MIPSEARCH_TRADITIONAL);
      wrap_assert ( !status, "CPLEX: setting traditional search" );
      /* Let MIP callbacks work on the original model */
      status = dll_CPXsetintparam (env, CPXPARAM_MIP_Strategy_CallbackReducedLP,
                                CPX_OFF);
      wrap_assert ( !status, "CPLEX: setting callbacks to work on orig model" );
      /// And
      /* Set up to use MIP usercut callback */

      status = dll_CPXsetusercutcallbackfunc (env, myusercutcallback, &usercutinfo);
      wrap_assert ( !status, "CPLEX: setting user cut callback" );
    }
    if ( cbui.cutMask & MaskConsType_Lazy ) {
      if ( fVerbose )
        cerr << "  MIP_cplex_wrapper: lazy cut callback enabled, setting params" << endl;
      CUTINFO lazyconinfo;
      lazyconinfo.info = &cbui;
      /* Init information on the node objval for the user cut callback.
          No need to initialize the information on the node objval,
          for the lazy constraint callback, because those information are
          used only in the user cut callback. */
      initnodeobjvalinfo (this, env, lp, &lazyconinfo);
      /* Assure linear mappings between the presolved and original
          models */
      status = dll_CPXsetintparam (env, CPXPARAM_Preprocessing_Linear, 0);
      wrap_assert ( !status, "CPLEX: setting prepro_linear" );
      /* Turn on traditional search for use with control callbacks */
//       status = dll_CPXsetintparam (env, CPXPARAM_MIP_Strategy_Search,
//                                 CPX_MIPSEARCH_TRADITIONAL);
//       wrap_assert ( !status, "CPLEX: setting traditional search" );
      /* Let MIP callbacks work on the original model */
      status = dll_CPXsetintparam (env, CPXPARAM_MIP_Strategy_CallbackReducedLP,
                                CPX_OFF);
      wrap_assert ( !status, "CPLEX: setting callbacks to work on orig model" );
      /* Set up to use MIP lazyconstraint callback. The callback funtion
        * registered is the same, but the data will be different. */

      status = dll_CPXsetlazyconstraintcallbackfunc (env, myusercutcallback, &lazyconinfo);
      wrap_assert ( !status, "CPLEX: setting lazy cut callback" );
    }
  }

  /// after all modifs
    if (options->sWriteParams.size()) {
     status = dll_CPXwriteparam (env, options->sWriteParams.c_str());
     wrap_assert(!status, "Failed to write CPLEX parameters.", false);
    }
    
   // status = dll_CPXgettime (env, &output.dCPUTime);
   // wrap_assert(!status, "Failed to get time stamp.", false);
   cbui.pOutput->dWallTime0 = output.dWallTime0 =
     std::chrono::steady_clock::now();
   cbui.pOutput->cCPUTime0 = std::clock();

   /* Optimize the problem and obtain solution. */
   status = dll_CPXmipopt (env, lp);
   wrap_assert( !status,  "Failed to optimize MIP." );

   output.dWallTime = std::chrono::duration<double>(
     std::chrono::steady_clock::now() - output.dWallTime0).count();
   double tmNow = std::clock();
   // status = dll_CPXgettime (env, &tmNow);   Buggy in 12.7.1.0
   wrap_assert(!status, "Failed to get time stamp.", false);
   output.dCPUTime = (tmNow - cbui.pOutput->cCPUTime0) / CLOCKS_PER_SEC;

   int solstat = dll_CPXgetstat (env, lp);
   output.status = convertStatus(solstat);
   output.statusName = dll_CPXgetstatstring (env, solstat, cplex_status_buffer);

   /// Continuing to fill the output object:
   if (Status::OPT == output.status || Status::SAT ==output.status) {
      status = dll_CPXgetobjval (env, lp, &output.objVal);
      wrap_assert( !status, "No MIP objective value available." );

      /* The size of the problem should be obtained by asking CPLEX what
          the actual size is, rather than using what was passed to CPXcopylp.
          cur_numrows and cur_numcols store the current number of rows and
          columns, respectively.  */   // ?????????????? TODO

    //    int cur_numrows = dll_CPXgetnumrows (env, lp);
      int cur_numcols = dll_CPXgetnumcols (env, lp);
      assert(cur_numcols == colObj.size());
      
      x.resize(cur_numcols);
      output.x = &x[0];
      status = dll_CPXgetx (env, lp, &x[0], 0, cur_numcols-1);
      wrap_assert(!status, "Failed to get variable values.");
      if (cbui.solcbfn /*&& (!options->flag_all_solutions || !cbui.printed)*/) {
        cbui.solcbfn(output, cbui.psi);
      }
   }
   output.bestBound = 1e308;
   status = dll_CPXgetbestobjval (env, lp, &output.bestBound);
   wrap_assert(!status, "Failed to get the best bound.", false);
   output.nNodes = dll_CPXgetnodecnt (env, lp);
   output.nOpenNodes = dll_CPXgetnodeleftcnt (env, lp);
}

void MIP_cplex_wrapper::setObjSense(int s)
{
  status = dll_CPXchgobjsen (env, lp, -s);  // +1 for min in CPLEX
  wrap_assert(!status, "Failed to set obj sense.");
}


