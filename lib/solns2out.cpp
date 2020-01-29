/* -*- mode: C++; c-basic-offset: 2; indent-tabs-mode: nil -*- */

/*
 *  Main authors:
 *     Guido Tack <guido.tack@monash.edu>
 *     Gleb Belov <gleb.belov@monash.edu>
 */

/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was ! distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */


#ifdef _MSC_VER
#define _CRT_SECURE_NO_WARNINGS
#endif

#include <minizinc/solns2out.hh>
#include <minizinc/solver.hh>
#include <fstream>

using namespace std;
using namespace MiniZinc;

void Solns2Out::printHelp(ostream& os)
{
  os
  << "Solution output options:" << std::endl
  << "  --ozn-file <file>\n    Read output specification from ozn file." << std::endl
  << "  -o <file>, --output-to-file <file>\n    Filename for generated output." << std::endl
  << "  -i <n>, --ignore-lines <n>, --ignore-leading-lines <n>\n    Ignore the first <n> lines in the FlatZinc solution stream." << std::endl
  << "  --soln-sep <s>, --soln-separator <s>, --solution-separator <s>\n    Specify the string printed after each solution (as a separate line).\n    The default is to use the same as FlatZinc, \"----------\"." << std::endl
  << "  --soln-comma <s>, --solution-comma <s>\n    Specify the string used to separate solutions.\n    The default is the empty string." << std::endl
  << "  --unsat-msg (--unsatisfiable-msg), --unbounded-msg, --unsatorunbnd-msg,\n"
  "        --unknown-msg, --error-msg, --search-complete-msg <msg>\n"
  "    Specify solution status messages. The defaults:\n"
  "    \"=====UNSATISFIABLE=====\", \"=====UNSATorUNBOUNDED=====\", \"=====UNBOUNDED=====\",\n"
  "    \"=====UNKNOWN=====\", \"=====ERROR=====\", \"==========\", respectively." << std::endl
  << "  --non-unique\n    Allow duplicate solutions.\n"
  << "  -c, --canonicalize\n    Canonicalize the output solution stream (i.e., buffer and sort).\n"
  << "  --output-non-canonical <file>\n    Non-buffered solution output file in case of canonicalization.\n"
  << "  --output-raw <file>\n    File to dump the solver's raw output (not for hard-linked solvers)\n"
  // Unclear how to exit then:
//   << "  --number-output <n>\n    Maximal number of different solutions printed." << std::endl
  << "  --no-output-comments\n    Do not print comments in the FlatZinc solution stream." << std::endl
  << "  --output-time\n    Print timing information in the FlatZinc solution stream." << std::endl
  << "  --no-flush-output\n    Don't flush output stream after every line." << std::endl
  ;
}

bool Solns2Out::processOption(int& i, std::vector<std::string>& argv)
{
  CLOParser cop( i, argv );
  std::string oznfile;
  if ( cop.getOption( "--ozn-file", &oznfile) ) {
    initFromOzn(oznfile);
  } else if ( cop.getOption( "-o --output-to-file", &_opt.flag_output_file) ) {
  } else if ( cop.getOption( "--no-flush-output" ) ) {
    _opt.flag_output_flush = false;
  } else if ( cop.getOption( "--no-output-comments" ) ) {
    _opt.flag_output_comments = false;
  } else if ( cop.getOption( "-i --ignore-lines --ignore-leading-lines", &_opt.flag_ignore_lines ) ) {
  } else if ( cop.getOption( "--output-time" ) ) {
    _opt.flag_output_time = true;
  } else if ( cop.getOption( "--soln-sep --soln-separator --solution-separator", &_opt.solution_separator ) ) {
  } else if ( cop.getOption( "--soln-comma --solution-comma", &_opt.solution_comma ) ) {
  } else if ( cop.getOption( "--unsat-msg --unsatisfiable-msg", &_opt.unsatisfiable_msg ) ) {
  } else if ( cop.getOption( "--unbounded-msg", &_opt.unbounded_msg ) ) {
  } else if ( cop.getOption( "--unsatorunbnd-msg", &_opt.unsatorunbnd_msg ) ) {
  } else if ( cop.getOption( "--unknown-msg", &_opt.unknown_msg ) ) {
  } else if ( cop.getOption( "--error-msg", &_opt.error_msg ) ) {
  } else if ( cop.getOption( "--search-complete-msg", &_opt.search_complete_msg ) ) {
  } else if ( cop.getOption( "--unique") ) {
    _opt.flag_unique = true;
  } else if ( cop.getOption( "--non-unique") ) {
    _opt.flag_unique = false;
  } else if ( cop.getOption( "-c --canonicalize") ) {
    _opt.flag_canonicalize = true;
  } else if ( cop.getOption( "--output-non-canonical --output-non-canon", &_opt.flag_output_noncanonical) ) {
  } else if ( cop.getOption( "--output-raw", &_opt.flag_output_raw) ) {
//   } else if ( cop.getOption( "--number-output", &_opt.flag_number_output ) ) {
  } else if ( _opt.flag_standaloneSolns2Out ) {
    std::string oznfile(argv[i]);
    if (oznfile.length()<=4) {
      return false;
    }
    size_t last_dot = oznfile.find_last_of('.');
    if (last_dot == string::npos) {
      return false;
    }
    std::string extension = oznfile.substr(last_dot,string::npos);
    if (extension == ".ozn") {
      initFromOzn(oznfile);
      return true;
    }
    return false;
  } else {
    return false;
  }
  return true;
}

bool Solns2Out::initFromEnv(Env* pE) {
  assert(pE); pEnv=pE;
  init();
  return true;
}

void Solns2Out::initFromOzn(const std::string& filename) {
  std::vector<string> filenames( 1, filename );
  
  includePaths.push_back(stdlibDir+"/std/");
  
  for (unsigned int i=0; i<includePaths.size(); i++) {
    if (!FileUtils::directory_exists(includePaths[i])) {
      std::cerr << "solns2out: cannot access include directory " << includePaths[i] << "\n";
      std::exit(EXIT_FAILURE);
    }
  }
  
  {
    pEnv = new Env();
    std::stringstream errstream;
    if ((pOutput = parse(*pEnv, filenames, std::vector<std::string>(), "", "", includePaths, false, false, false,
                         errstream))) {
      std::vector<TypeError> typeErrors;
      pEnv->model(pOutput);
      MZN_ASSERT_HARD_MSG( pEnv, "solns2out: could not allocate Env" );
      pEnv_guard.reset( pEnv );
      MiniZinc::typecheck(*pEnv,pOutput,typeErrors,false,false);
      MiniZinc::registerBuiltins(*pEnv);
      pEnv->envi().swap_output();
      init();
    } else {
      throw Error(errstream.str());
    }
  }
}

Solns2Out::DE& Solns2Out::findOutputVar( ASTString id ) {
  declNewOutput();
  auto it = declmap.find( id.str() );
  MZN_ASSERT_HARD_MSG( declmap.end()!=it,
                       "solns2out_base: unexpected id in output: " << id );
  return it->second;
}

void Solns2Out::restoreDefaults() {
  for (unsigned int i=0; i<getModel()->size(); i++) {
    if (VarDeclI* vdi = (*getModel())[i]->dyn_cast<VarDeclI>()) {
      if (vdi->e()->id()->idn()!=-1 || vdi->e()->id()->v()!="_mzn_solution_checker") {
        GCLock lock;
        auto& de = findOutputVar(vdi->e()->id()->str());
        vdi->e()->e(de.second());
        vdi->e()->evaluated(false);
      }
    }
  }
  fNewSol2Print = false;
}

void Solns2Out::parseAssignments(string& solution) {
  std::vector<SyntaxError> se;
  unique_ptr<Model> sm(
    parseFromString(*pEnv, solution, "solution received from solver", includePaths, true, false, false, log, se) );
  if (sm.get()==NULL)
    throw Error("solns2out_base: could not parse solution");
  solution = "";
  for (unsigned int i=0; i<sm->size(); i++) {
    if (AssignI* ai = (*sm)[i]->dyn_cast<AssignI>()) {
      auto& de = findOutputVar(ai->id());
      ai->e()->type(de.first->type());
      ai->decl(de.first);
      typecheck(*pEnv, getModel(), ai);
      if (Call* c = ai->e()->dyn_cast<Call>()) {
        // This is an arrayXd call, make sure we get the right builtin
        assert(c->arg(c->n_args()-1)->isa<ArrayLit>());
        for (unsigned int i=0; i<c->n_args(); i++)
          c->arg(i)->type(Type::parsetint());
        c->arg(c->n_args()-1)->type(de.first->type());
        c->decl(getModel()->matchFn(pEnv->envi(), c, false));
      }
      de.first->e(ai->e());
    }
  }
  declNewOutput();
}

void Solns2Out::declNewOutput() {
  fNewSol2Print=true;
  status = SolverInstance::SAT;
}

bool Solns2Out::evalOutput( const string& s_ExtraInfo ) {
  if ( !fNewSol2Print )
    return true;
  ostringstream oss;
  if ( !checkerModel.empty() ) {
    auto& checkerStream = pEnv->envi().checker_output;
    checkerStream.clear();
    checkerStream.str("");
    checkSolution(checkerStream);
  }
  if (!__evalOutput( oss )) {
    return false;
  }
  {
    auto& checkerStream = pEnv->envi().checker_output;
    checkerStream.flush();
    std::string line;
    if (std::getline(checkerStream, line)) {
      os << "% Solution checker report:\n";
      os << "% " << line << "\n";
      while (std::getline(checkerStream, line)) {
        os << "% " << line << "\n";
      }
    }
  }
  bool fNew=true;
  if ( _opt.flag_unique || _opt.flag_canonicalize ) {
    auto res = sSolsCanon.insert( oss.str() );
    if ( !res.second )            // repeated solution
      fNew = false;
  }
  if ( fNew ) {
    ++nSolns;
    if ( _opt.flag_canonicalize ) {
      if ( pOfs_non_canon.get() )
        if ( pOfs_non_canon->good() ) {
          (*pOfs_non_canon) << oss.str();
          (*pOfs_non_canon) << comments;
          if ( s_ExtraInfo.size() ) {
            (*pOfs_non_canon) << s_ExtraInfo;
            if ( '\n'!=s_ExtraInfo.back() )                 /// TODO is this enough to check EOL?
              (*pOfs_non_canon) << '\n';
          }
          if (_opt.flag_output_time)
            (*pOfs_non_canon) << "% time elapsed: " << starttime.stoptime() << "\n";
          if (!_opt.solution_separator.empty())
            (*pOfs_non_canon) << _opt.solution_separator << '\n';
          if ( _opt.flag_output_flush )
            pOfs_non_canon->flush();
        }
    } else {
      if ( _opt.solution_comma.size() && nSolns>1 )
        getOutput() << _opt.solution_comma << '\n';
      getOutput() << oss.str();
    }
  }
  getOutput() << comments;                          // print them now ????
  comments = "";
  if ( s_ExtraInfo.size() ) {
    getOutput() << s_ExtraInfo;
    if ( '\n'!=s_ExtraInfo.back() )                 /// TODO is this enough to check EOL?
      getOutput() << '\n';
  }
  if ( fNew && _opt.flag_output_time)
    getOutput() << "% time elapsed: " << starttime.stoptime() << "\n";
  if ( fNew && !_opt.flag_canonicalize && !_opt.solution_separator.empty())
    getOutput() << _opt.solution_separator << '\n';
  if ( _opt.flag_output_flush )
    getOutput().flush();
  restoreDefaults();     // cleans data. evalOutput() should not be called again w/o assigning new data.
  return true;
}

void Solns2Out::checkSolution(std::ostream& oss) {
#ifdef HAS_GECODE

  std::ostringstream checker;
  checker << checkerModel;
  {
    GCLock lock;
    for (unsigned int i=0; i<getModel()->size(); i++) {
      if (VarDeclI* vdi = (*getModel())[i]->dyn_cast<VarDeclI>()) {
        if (vdi->e()->ann().contains(constants().ann.mzn_check_var)) {
          checker << vdi->e()->id()->str() << " = ";
          Expression* e = eval_par(getEnv()->envi(),vdi->e()->e());
          ArrayLit* al = e->dyn_cast<ArrayLit>();
          std::vector<Id*> enumids;
          if (Call* cev = vdi->e()->ann().getCall(constants().ann.mzn_check_enum_var)) {
            ArrayLit* enumIdsAl = cev->arg(0)->cast<ArrayLit>();
            for (int j=0; j<enumIdsAl->size(); j++) {
              enumids.push_back((*enumIdsAl)[j]->dyn_cast<Id>());
            }
          }
          
          if (al) {
            checker << "array" << al->dims() << "d(";
            for (int i=0; i<al->dims(); i++) {
              if (enumids.size() > 0 && enumids[i] != nullptr) {
                checker << "to_enum(" << *enumids[i] << ",";
              }
              checker << al->min(i) << ".." << al->max(i);
              if (enumids.size() > 0 && enumids[i] != nullptr) {
                checker << ")";
              }
              checker << ",";
            }
          }
          if (enumids.size() > 0 && enumids.back() != nullptr) {
            checker << "to_enum(" << *enumids.back() << "," << *e << ")";
          } else {
            checker << *e;
          }
          if (al) {
            checker << ")";
          }
          checker << ";\n";
        }
      }
    }
  }

  MznSolver slv(oss,oss);
  slv.s2out._opt.solution_separator = "";
  try {
    std::vector<std::string> args({"--solver","org.minizinc.gecode_presolver"});
    slv.run(args, checker.str(), "minizinc", "checker.mzc");
  } catch (const LocationException& e) {
    oss << e.loc() << ":" << std::endl;
    oss << e.what() << ": " << e.msg() << std::endl;
  } catch (const Exception& e) {
    std::string what = e.what();
    oss << what << (what.empty() ? "" : ": ") <<e.msg() << std::endl;
  }
  catch (const exception& e) {
    oss << e.what() << std::endl;
  }
  catch (...) {
    oss << "  UNKNOWN EXCEPTION." << std::endl;
  }
  
#else
  oss << "% solution checking not supported (need built-in Gecode)" << std::endl;
#endif
}

bool Solns2Out::__evalOutput( ostream& fout ) {
  if ( 0!=outputExpr ) {
    pEnv->envi().evalOutput( fout );
  }
  return true;
}

bool Solns2Out::evalStatus( SolverInstance::Status status ) {
  if ( _opt.flag_canonicalize )
    __evalOutputFinal( _opt.flag_output_flush );
  __evalStatusMsg( status );
  fStatusPrinted = 1;
  return true;
}

bool Solns2Out::__evalOutputFinal( bool ) {
  /// Print the canonical list
  for ( auto& sol : sSolsCanon ) {
    if ( _opt.solution_comma.size() && &sol != &*sSolsCanon.begin() )
      getOutput() << _opt.solution_comma << '\n';
    getOutput() << sol;
    if (!_opt.solution_separator.empty())
      getOutput() << _opt.solution_separator << '\n';
  }
  return true;
}

bool Solns2Out::__evalStatusMsg( SolverInstance::Status status ) {
  std::map<SolverInstance::Status, string> stat2msg;
  stat2msg[ SolverInstance::OPT ] = _opt.search_complete_msg;
  stat2msg[ SolverInstance::UNSAT ] = _opt.unsatisfiable_msg;
  stat2msg[ SolverInstance::UNBND ] = _opt.unbounded_msg;
  stat2msg[ SolverInstance::UNSATorUNBND ] = _opt.unsatorunbnd_msg;
  stat2msg[ SolverInstance::UNKNOWN ] = _opt.unknown_msg;
  stat2msg[ SolverInstance::ERROR ] = _opt.error_msg;
  stat2msg[ SolverInstance::NONE ] = "";
  auto it=stat2msg.find(status);
  if ( stat2msg.end()!=it ) {
    getOutput() << comments;
    if (!it->second.empty())
      getOutput() << it->second << '\n';
    if ( _opt.flag_output_time)
      getOutput() << "% time elapsed: " << starttime.stoptime() << "\n";
    if ( _opt.flag_output_flush )
      getOutput().flush();
    Solns2Out::status = status;
  }
  else {
    getOutput() << comments;
    if ( _opt.flag_output_flush )
      getOutput().flush();
    MZN_ASSERT_HARD_MSG( SolverInstance::SAT==status,    // which is ignored
                         "solns2out_base: undefined solution status code " << status );
    Solns2Out::status = SolverInstance::SAT;
  }
  comments = "";
  return true;
}

void Solns2Out::init() {

  declmap.clear();
  for (unsigned int i=0; i<getModel()->size(); i++) {
    if (OutputI* oi = (*getModel())[i]->dyn_cast<OutputI>()) {
      outputExpr = oi->e();
    } else if (VarDeclI* vdi = (*getModel())[i]->dyn_cast<VarDeclI>()) {
      if (vdi->e()->id()->idn()==-1 && vdi->e()->id()->v()=="_mzn_solution_checker") {
        checkerModel = eval_string(getEnv()->envi(), vdi->e()->e());
        if (checkerModel.size() > 0 && checkerModel[0]=='@') {
          checkerModel = FileUtils::decodeBase64(checkerModel);
          FileUtils::inflateString(checkerModel);
        }
      } else {
        GCLock lock;
        declmap.insert(pair<std::string,DE>(vdi->e()->id()->str().str(),DE(vdi->e(),vdi->e()->e())));
      }
    }
  }

  /// Main output file
  if ( 0==pOut ) {
    if ( _opt.flag_output_file.size() ) {
      pOut.reset( new ofstream( _opt.flag_output_file ) );
      MZN_ASSERT_HARD_MSG( pOut.get(),
        "solns2out_base: could not allocate stream object for file output into "
        << _opt.flag_output_file );
      checkIOStatus( pOut->good(), _opt.flag_output_file);
    }
  }
  /// Non-canonical output
  if ( _opt.flag_canonicalize && _opt.flag_output_noncanonical.size() ) {
    pOfs_non_canon.reset( new ofstream( _opt.flag_output_noncanonical ) );
    MZN_ASSERT_HARD_MSG( pOfs_non_canon.get(),
                         "solns2out_base: could not allocate stream object for non-canon output" );
    checkIOStatus( pOfs_non_canon->good(), _opt.flag_output_noncanonical, 0);
  }
  /// Raw output
  if ( _opt.flag_output_raw.size() ) {
    pOfs_raw.reset( new ofstream( _opt.flag_output_raw ) );
    MZN_ASSERT_HARD_MSG( pOfs_raw.get(),
                         "solns2out_base: could not allocate stream object for raw output" );
    checkIOStatus( pOfs_raw->good(), _opt.flag_output_raw, 0);
  }
  /// Assume all options are set before
  nLinesIgnore = _opt.flag_ignore_lines;
}

Solns2Out::Solns2Out(std::ostream& os0, std::ostream& log0, const std::string& stdlibDir0) : os(os0), log(log0), stdlibDir(stdlibDir0) {}

Solns2Out::~Solns2Out() {
  getOutput() << comments;
  if ( _opt.flag_output_flush )
    getOutput() << flush;
}

ostream& Solns2Out::getOutput() {
  return (( pOut.get() && pOut->good() ) ? *pOut : os);
}

ostream& Solns2Out::getLog() {
  return log;
}

bool Solns2Out::feedRawDataChunk(const char* data) {
  istringstream solstream( data );
  while (solstream.good()) {
    string line;
    getline(solstream, line);
    if (line_part.size()) {
      line = line_part + line;
      line_part.clear();
    }
    if (solstream.eof()) {  // wait next chunk
      line_part = line;
      break;  // to get to raw output
    }
    if (line.size())
      if ('\r' == line.back())
        line.pop_back();       // For WIN files
    if ( nLinesIgnore > 0 ) {
      --nLinesIgnore;
      continue;
    }
    if ( mapInputStatus.empty() )
      createInputMap();
    auto it = mapInputStatus.find( line );
    if ( mapInputStatus.end()!=it ) {
      if ( SolverInstance::SAT==it->second ) {
        parseAssignments( solution );
        evalOutput();
      } else {
        evalStatus( it->second );
      }
    } else {
      solution += line + '\n';
      if ( _opt.flag_output_comments ) {
        std::istringstream iss( line );
        char c='_';
        iss >> skipws >> c;
        if ( iss.good() && '%'==c) {
          // Feed comments directly
          getOutput() << line << '\n';
          if ( _opt.flag_output_flush )
            getOutput().flush();
          if ( pOfs_non_canon.get() )
            if ( pOfs_non_canon->good() )
              ( *pOfs_non_canon ) << line << '\n';
        }
      }
    }
  }
  if ( pOfs_raw.get() ) {
    (*pOfs_raw.get()) << data;
    if (_opt.flag_output_flush)
      pOfs_raw->flush();
  }
  return true;
}

void Solns2Out::createInputMap() {
  mapInputStatus[ _opt.search_complete_msg_00 ] = SolverInstance::OPT;
  mapInputStatus[ _opt.solution_separator_00 ] = SolverInstance::SAT;
  mapInputStatus[ _opt.unsatisfiable_msg_00 ] = SolverInstance::UNSAT;
  mapInputStatus[ _opt.unbounded_msg_00 ] = SolverInstance::UNBND;
  mapInputStatus[ _opt.unsatorunbnd_msg_00 ] = SolverInstance::UNSATorUNBND;
  mapInputStatus[ _opt.unknown_msg_00 ] = SolverInstance::UNKNOWN;
  mapInputStatus[ _opt.error_msg ] = SolverInstance::ERROR;
}

void Solns2Out::printStatistics(ostream& os)
{
  os << "%%%mzn-stat: nSolutions=" << nSolns << "\n";
}
