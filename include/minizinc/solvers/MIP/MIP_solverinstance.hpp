#include <chrono>

namespace MiniZinc {

  template<class MIPWrapper>
  MIP_SolverFactory<MIPWrapper>::MIP_SolverFactory(void) {
    SolverConfig sc(getId(), MIPWrapper::getVersion());
    sc.name(MIPWrapper::getName());
    sc.mznlib(MIPWrapper::getMznLib());
    sc.mznlibVersion(1);
    sc.supportsMzn(true);
    sc.description("MiniZinc MIP solver plugin");
    sc.requiredFlags(MIPWrapper::getRequiredFlags());
    sc.tags(MIPWrapper::getTags());
    sc.stdFlags(MIPWrapper::getStdFlags());
    SolverConfigs::registerBuiltinSolver(sc);
  }
  
  template<class MIPWrapper>
  bool MIP_SolverFactory<MIPWrapper>::processOption(SolverInstanceBase::Options* opt, int& i, std::vector<std::string>& argv) {
    if (argv[i]=="--verbose-solving") {
      opt->verbose = true;
      return true;
    } else if (argv[i]=="--solver-statistics") {
      opt->printStatistics = true;
      return true;
    } else {
      return static_cast<typename MIPWrapper::Options&>(*opt).processOption(i, argv);
    }
  }
  
  template<class MIPWrapper>
  std::string MIP_SolverFactory<MIPWrapper>::getDescription(SolverInstanceBase::Options* opt)
  {
    std::string v = "MIP solver plugin, compiled " __DATE__ ", using: "
      + MIPWrapper::getDescription(opt);
    return v;
  }

  template<class MIPWrapper>
  std::string MIP_SolverFactory<MIPWrapper>::getVersion(SolverInstanceBase::Options* opt)
  {
    return MIPWrapper::getVersion(opt);
  }

  template<class MIPWrapper>
  std::string MIP_SolverFactory<MIPWrapper>::getId()
  {
    return "org.minizinc.mip."+MIPWrapper::getId();
  }

  template<class MIPWrapper>
  MIP_solver::Variable
  MIP_solverinstance<MIPWrapper>::exprToVar(Expression* arg) {
    if (Id* ident = arg->dyn_cast<Id>()) {
      return _variableMap.get(ident->decl()->id());
    } else
      return mip_wrap->addLitVar( exprToConst( arg ) );
  }

  template<class MIPWrapper>
  void
  MIP_solverinstance<MIPWrapper>::exprToVarArray(Expression* arg, std::vector<VarId> &vars) {
    ArrayLit* al = eval_array_lit(getEnv()->envi(), arg);
    vars.clear();
    vars.reserve(al->size());
    for (unsigned int i=0; i<al->size(); i++)
      vars.push_back(exprToVar((*al)[i]));
  }

  template<class MIPWrapper>
  std::pair<double,bool>
  MIP_solverinstance<MIPWrapper>::exprToConstEasy(Expression* e) {
    std::pair<double, bool> res { 0.0, true };
    if (IntLit* il = e->dyn_cast<IntLit>()) {
      res.first = ( static_cast<double>(il->v().toInt()) );
    } else if (FloatLit* fl = e->dyn_cast<FloatLit>()) {
      res.first = ( fl->v().toDouble() );
    } else if (BoolLit* bl = e->dyn_cast<BoolLit>()) {
      res.first = ( bl->v() );
    } else {
      res.second = false;
    }
    return res;
  }

  template<class MIPWrapper>
  double
  MIP_solverinstance<MIPWrapper>::exprToConst(Expression* e) {
    const auto e2ce = exprToConstEasy( e );
    if ( !e2ce.second ) {
      std::ostringstream oss;
      oss << "ExprToConst: expected a numeric/bool literal, getting " << *e;
      throw InternalError( oss.str() );
    }
    return e2ce.first;
  }
  
  template<class MIPWrapper>
  void
  MIP_solverinstance<MIPWrapper>::exprToArray(Expression* arg, std::vector<double> &vals) {
    ArrayLit* al = eval_array_lit(getEnv()->envi(), arg);
    vals.clear();
    vals.reserve(al->size());
    for (unsigned int i=0; i<al->size(); i++) {
      vals.push_back( exprToConst( (*al)[i] ) );
    }
  }

  template<class MIPWrapper>
  void
  MIP_solverinstance<MIPWrapper>::processSearchAnnotations(const Annotation& ann) {
    if ( 1==getMIPWrapper()->getFreeSearch() )
      return;
    std::vector<Expression*> aAnns;
    flattenSearchAnnotations( ann, aAnns );
    std::vector<MIP_solverinstance::VarId> vars;
    std::vector<int> aPri;                                  // priorities
    int nArrayAnns = 0;
    for ( auto iA=0; iA<aAnns.size(); ++iA ) {
      const auto& pE = aAnns[iA];
      if( pE->isa<Call>() ) {
        Call* pC = pE->cast<Call>();
        const auto cId = pC->id().str();
        if ( "int_search"==cId || "float_search"==cId ) {
          ArrayLit* alV = nullptr;
          if ( !pC->n_args() || nullptr == (alV = eval_array_lit(_env.envi(),pC->arg(0))) ) {
            std::cerr << "  SEARCH ANN: '" << (*pC)
            << "'  is unknown. " << std::endl;
            continue;
          }
          ++nArrayAnns;
          for (unsigned int i=0; i<alV->size(); i++) {
            if (Id* ident = (*alV)[i]->dyn_cast<Id>()) {
              vars.push_back( exprToVar( ident ) );
              aPri.push_back( static_cast<int>(aAnns.size())-iA );            // level search by default
            } // else ignore
          }
        }
      }
    }
    if ( vars.size() ) {
      if ( 2==getMIPWrapper()->getFreeSearch() ) {
        for ( int i=0; i<vars.size(); ++i )
          aPri[i] = 1; //vars.size()-i;                                    // descending
      }
      if ( !getMIPWrapper()->addSearch( vars, aPri ) )
        std::cerr << "\nWARNING: MIP backend seems to ignore search strategy." << std::endl;
      else
        std::cerr << "  MIP: added " << vars.size() << " variable branching priorities from "
        << nArrayAnns << " arrays." << std::endl;
    }
  }
  
  template<class MIPWrapper>
  void
  MIP_solverinstance<MIPWrapper>::processWarmstartAnnotations(const Annotation& ann) {
    int nVal = 0;
    for(ExpressionSetIter i = ann.begin(); i != ann.end(); ++i) {
      Expression* e = *i;
      if ( e->isa<Call>() ) {
        Call* c = e->cast<Call>();
        if ( c->id().str() == "warm_start_array" || c->id().str() == "seq_search" ) {
          ArrayLit* anns = c->arg(0)->cast<ArrayLit>();
          for(unsigned int i=0; i<anns->size(); i++) {
            Annotation subann;
            subann.add((*anns)[i]);
            processWarmstartAnnotations( subann );
          }
        } else
          if ( c->id().str() == "warm_start" ) {
            MZN_ASSERT_HARD_MSG( c->n_args()>=2, "ERROR: warm_start needs 2 array args" );
            std::vector<double> coefs;
            std::vector<MIP_solverinstance::VarId> vars;
            
            /// Process coefs & vars together to eliminate literals (problem with Gurobi's updatemodel()'s)
            ArrayLit* alC = eval_array_lit(_env.envi(), c->arg(1));
            MZN_ASSERT_HARD_MSG( 0!=alC, "ERROR: warm_start needs 2 array args" );
            coefs.reserve(alC->size());
            ArrayLit* alV = eval_array_lit(_env.envi(), c->arg(0));
            MZN_ASSERT_HARD_MSG( 0!=alV, "ERROR: warm_start needs 2 array args" );
            vars.reserve(alV->size());
            for (unsigned int i=0; i<alV->size() && i<alC->size(); i++) {
              const auto e2c = exprToConstEasy( (*alC)[i] );
              /// Check if it is not an opt int etc. and a proper variable
              if (e2c.second)
                if (Id* ident = (*alV)[i]->dyn_cast<Id>()) {
                  coefs.push_back( e2c.first );
                  vars.push_back( exprToVar( ident ) );
                } // else ignore
            }
            assert(coefs.size() == vars.size());
            nVal += static_cast<int>(coefs.size());
            if ( coefs.size() && !getMIPWrapper()->addWarmStart( vars, coefs ) ) {
              std::cerr << "\nWARNING: MIP backend seems to ignore warm starts" << std::endl;
              return;
            }
          }
      }
    }
    if ( nVal && getMIPWrapper()->fVerbose ) {
      std::cerr << "  MIP: added " << nVal << " MIPstart values..." << std::flush;
    }
  }
  
  template<class MIPWrapper>
  void
  MIP_solverinstance<MIPWrapper>::processFlatZinc(void) {
    mip_wrap->fVerbose = _options->verbose;
    
    SolveI* solveItem = getEnv()->flat()->solveItem();
    VarDecl* objVd = NULL;
    
    if (solveItem->st() != SolveI::SolveType::ST_SAT) {
      if(Id* id = solveItem->e()->dyn_cast<Id>()) {
        objVd = id->decl();
      } else {
        std::cerr << "Objective must be Id: " << solveItem->e() << std::endl;
        throw InternalError("Objective must be Id");
      }
    }
    
    for (VarDeclIterator it = getEnv()->flat()->begin_vardecls(); it != getEnv()->flat()->end_vardecls(); ++it) {
      if (it->removed()) {
        continue;
      }
      VarDecl* vd = it->e();
      if(!vd->ann().isEmpty()) {
        if(vd->ann().containsCall(constants().ann.output_array.aststr()) ||
           vd->ann().contains(constants().ann.output_var)
           ) {
          _varsWithOutput.push_back(vd);
          //         std::cerr << (*vd);
          //         if ( vd->e() )
          //           cerr << " = " << (*vd->e());
          //         cerr << endl;
        }
      }
      if (vd->type().dim() == 0 && it->e()->type().isvar() && !it->removed()) {
        MiniZinc::TypeInst* ti = it->e()->ti();
        MIP_wrapper::VarType vType = MIP_wrapper::VarType::REAL;     // fInt = false;
        if (ti->type().isvarint() || ti->type().isint())
          vType = MIP_wrapper::VarType::INT;
        else if (ti->type().isvarbool() || ti->type().isbool()) {
          vType = MIP_wrapper::VarType::BINARY;
        } else if (ti->type().isvarfloat() || ti->type().isfloat()) {
        } else {
          std::stringstream ssm;
          ssm << "This type of var is not handled by MIP: " << *it << std::endl;
          ssm << "  VarDecl flags (ti, bt, st, ot): "
          << ti->type().ti()
          << ti->type().bt()
          << ti->type().st()
          << ti->type().ot()
          << ", dim == " << ti->type().dim()
          << "\nRemove the variable or add a constraint so it is redefined."
          << std::endl;
          throw InternalError(ssm.str());
        }
        double lb=0.0, ub=1.0;  // for bool
        if (ti->domain()) {
          if (MIP_wrapper::VarType::REAL == vType) {
            FloatBounds fb = compute_float_bounds(getEnv()->envi(), it->e()->id());
            if (fb.valid) {
              lb = fb.l.toDouble();
              ub = fb.u.toDouble();
            } else {
              lb = 1.0;
              ub = 0.0;
            }
          } else if (MIP_wrapper::VarType::INT == vType) {
            IntBounds ib = compute_int_bounds(getEnv()->envi(), it->e()->id());
            if (ib.valid) {  // Normally should be
              lb = static_cast<double>(ib.l.toInt());
              ub = static_cast<double>(ib.u.toInt());
            } else {
              lb = 1;
              ub = 0;
            }
          }
        } else if (MIP_wrapper::VarType::BINARY != vType) {
          lb = -getMIPWrapper()->getInfBound();  // if just 1 bound inf, using MZN's default?  TODO
          ub = -lb;
        }
        
        //       IntSetVal* dom = eval_intset(env,vdi->e()->ti()->domain());
        //       if (dom->size() > 1)
        //         throw runtime_error("MIP_solverinstance: domains with holes ! supported, use --MIPdomains");
        
        VarId res;
        Id* id = it->e()->id();
        MZN_ASSERT_HARD( id == id->decl()->id() );   // Assume all unified
        MZN_ASSERT_HARD( it->e() == id->decl() );    // Assume all unified
        double obj = vd==objVd ? 1.0 : 0.0;
        auto decl00 = follow_id_to_decl( it->e() );
        MZN_ASSERT_HARD ( decl00->isa<VarDecl>() );
        {
          auto vd00 = decl00->dyn_cast<VarDecl>();
          if ( 0!=vd00->e() ) {
            // Should be a const
            auto dRHS = exprToConst( vd00->e() );
            lb = std::max( lb, dRHS );
            ub = std::min( ub, dRHS );
          }
          if ( it->e()!=vd00 ) {    // A different vardecl
            res = exprToVar( vd00->id() );                 // Assume FZN is sorted.
            MZN_ASSERT_HARD( !getMIPWrapper()->fPhase1Over ); // Still can change colUB, colObj
            /// Tighten the ini-expr's bounds
            lb = getMIPWrapper()->colLB.at( res ) = std::max( getMIPWrapper()->colLB.at( res ), lb );
            ub = getMIPWrapper()->colUB.at( res ) = std::min( getMIPWrapper()->colUB.at( res ), ub );
            if ( 0.0!=obj ) {
              getMIPWrapper()->colObj.at( res ) = obj;
            }
          } else {
            res = getMIPWrapper()->addVar(obj, lb, ub, vType, id->str().c_str());
          }
        }
        /// Test infeasibility
        if ( lb>ub ) {
          _status = SolverInstance::UNSAT;
          if ( getMIPWrapper()->fVerbose )
            std::cerr << "  VarDecl '" << *(it->e())
            << "' seems infeasible: computed bounds [" << lb << ", " << ub << ']'
            << std::endl;
        }
        if ( 0.0!=obj ) {
          dObjVarLB = lb;
          dObjVarUB = ub;
          getMIPWrapper()->output.nObjVarIndex = res;
          if ( getMIPWrapper()->fVerbose )
            std::cerr << "  MIP: objective variable index (0-based): " << res << std::endl;
        }
        _variableMap.insert(id, res);
        assert( res == _variableMap.get(id) );
      }
    }
    if (mip_wrap->fVerbose && mip_wrap->sLitValues.size())
      std::cerr << "  MIP_solverinstance: during Phase 1,  "
      << mip_wrap->nLitVars << " literals with "
      << mip_wrap-> sLitValues.size() << " values used." << std::endl;
    if (! getMIPWrapper()->fPhase1Over)
      getMIPWrapper()->addPhase1Vars();
    
    if (mip_wrap->fVerbose)
      std::cerr << "  MIP_solverinstance: adding constraints..." << std::flush;
    
    for (ConstraintIterator it = getEnv()->flat()->begin_constraints(); it != getEnv()->flat()->end_constraints(); ++it) {
      if (!it->removed()) {
        if (Call* c = it->e()->dyn_cast<Call>()) {
          _constraintRegistry.post(c);
        }
      }
    }
    
    if (mip_wrap->fVerbose) {
      std::cerr << " done, " << mip_wrap->getNRows() << " rows && "
      << mip_wrap->getNCols() << " columns in total.";
      if (mip_wrap->nIndicatorConstr)
        std::cerr << "  " << mip_wrap->nIndicatorConstr << " indicator constraints." << std::endl;
      std::cerr  << std::endl;
      if (mip_wrap->sLitValues.size())
        std::cerr << "  MIP_solverinstance: overall,  "
        << mip_wrap->nLitVars << " literals with "
        << mip_wrap-> sLitValues.size() << " values used." << std::endl;
    }
    
    processSearchAnnotations( solveItem->ann() );
    
    processWarmstartAnnotations( solveItem->ann() );
  }  // processFlatZinc
  
  template<class MIPWrapper>
  Expression*
  MIP_solverinstance<MIPWrapper>::getSolutionValue(Id* id) {
    id = id->decl()->id();
    
    if(id->type().isvar()) {
      MIP_solver::Variable var = exprToVar(id);
      double val = getMIPWrapper()->getValues()[var];
      switch (id->type().bt()) {
        case Type::BT_INT: return IntLit::a(round_to_longlong(val));
        case Type::BT_FLOAT: return FloatLit::a(val);
        case Type::BT_BOOL: return new BoolLit(Location(), round_to_longlong(val) != 0);
        default: return NULL;
      }
    } else {
      return id->decl()->e();
    }
  }
  
  template<class MIPWrapper>
  void
  MIP_solverinstance<MIPWrapper>::genCuts(const MIP_wrapper::Output& slvOut,
                                          MIP_wrapper::CutInput& cutsIn, bool fMIPSol) {
    for ( auto& pCG : cutGenerators ) {
      if ( !fMIPSol || pCG->getMask()&MIP_wrapper::MaskConsType_Lazy )
        pCG->generate( slvOut, cutsIn );
    }
    /// Select some most violated? TODO
  }

  template<class MIPWrapper>
  void
  MIP_solverinstance<MIPWrapper>::printStatisticsLine(bool fLegend)
  {
    //   auto nn = std::chrono::system_clock::now();
    //   auto n_c = std::chrono::system_clock::to_time_t( nn );
    {
      std::ios oldState(nullptr);
      oldState.copyfmt(_log);
      _log.precision(12);
      _log << "  % MIP Status: " << mip_wrap->getStatusName() << std::endl;
      if (fLegend)
        _log << "  % obj, bound, time wall/CPU, nodes (left): ";
      _log << mip_wrap->getObjValue() << ",  ";
      _log << mip_wrap->getBestBound() << ",  ";
      _log.setf( std::ios::fixed );
      _log.precision( 1 );
      _log << mip_wrap->getWallTimeElapsed() << "/";
      _log << mip_wrap->getCPUTime() << ",  ";
      _log << mip_wrap->getNNodes();
      if (mip_wrap->getNOpen())
        _log << " ( " << mip_wrap->getNOpen() << " )";
      //       _log << "    " << std::ctime( &n_c );
      //  ctime already adds EOL.     os << endl;
      _log << std::endl;
      _log.copyfmt( oldState );
    }
  }
  
  template<class MIPWrapper>
  void
  MIP_solverinstance<MIPWrapper>::printStatistics(bool fLegend)
  {
    //   auto nn = std::chrono::system_clock::now();
    //   auto n_c = std::chrono::system_clock::to_time_t( nn );
    {
      EnvI& env = getEnv()->envi();
      
      std::ios oldState(nullptr);
      oldState.copyfmt(env.outstream);
      env.outstream.precision(12);
      env.outstream << "%%%mzn-stat objective=" << mip_wrap->getObjValue() << std::endl;;
      env.outstream << "%%%mzn-stat objectiveBound=" << mip_wrap->getBestBound() << std::endl;;
      env.outstream << "%%%mzn-stat nodes=" << mip_wrap->getNNodes() << std::endl;;
      if (mip_wrap->getNOpen())
        env.outstream << "%%%mzn-stat openNodes=" << mip_wrap->getNOpen() << std::endl;;
      env.outstream.setf( std::ios::fixed );
      env.outstream.precision( 4 );
      env.outstream << "%%%mzn-stat solveTime=" << mip_wrap->getWallTimeElapsed() << std::endl;;
      env.outstream.copyfmt( oldState );

      env.outstream << "%%%mzn-stat-end" << std::endl;
    }
  }

  
  template<class MIPWrapper>
  void HandleSolutionCallback(const MIP_wrapper::Output& out, void* pp) {
    // multi-threading? TODO
    MIP_solverinstance<MIPWrapper>* pSI = static_cast<MIP_solverinstance<MIPWrapper>*>( pp );
    assert(pSI);
    /// Not for -a:
    //   if (fabs(pSI->lastIncumbent - out.objVal) > 1e-12*(1.0 + fabs(out.objVal))) {
    pSI->lastIncumbent = out.objVal;
    
    try {     /// Sometimes the intermediate output is wrong, especially in SCIP
      pSI->printSolution();            // The solution in [out] is not used  TODO
    } catch (const Exception& e) {
      std::cerr << std::endl;
      std::cerr << "  Error when evaluating an intermediate solution:  " << e.what() << ": " << e.msg() << std::endl;
    }
    catch (const std::exception& e) {
      std::cerr << std::endl;
      std::cerr << "  Error when evaluating an intermediate solution:  " << e.what() << std::endl;
    }
    catch (...) {
      std::cerr << std::endl;
      std::cerr << "  Error when evaluating an intermediate solution:  " << "  UNKNOWN EXCEPTION." << std::endl;
    }
    //   }
  }
  
  template<class MIPWrapper>
  void HandleCutCallback(const MIP_wrapper::Output& out, MIP_wrapper::CutInput& in,
                         void* pp, bool fMIPSol) {
    // multi-threading? TODO
    MIP_solverinstance<MIPWrapper>* pSI = static_cast<MIP_solverinstance<MIPWrapper>*>( pp );
    assert(pSI);
    assert(&out);
    assert(&in);
    pSI->genCuts( out, in, fMIPSol );
  }
  
  template<class MIPWrapper>
  SolverInstance::Status
  MIP_solverinstance<MIPWrapper>::solve(void) {
    SolveI* solveItem = getEnv()->flat()->solveItem();
    int nProbType=0;
    if (solveItem->st() != SolveI::SolveType::ST_SAT) {
      if (solveItem->st() == SolveI::SolveType::ST_MAX) {
        getMIPWrapper()->setObjSense(1);
        getMIPWrapper()->setProbType(1);
        nProbType=1;
        if (mip_wrap->fVerbose)
          std::cerr << "    MIP_solverinstance: this is a MAXimization problem." << std::endl;
      } else {
        getMIPWrapper()->setObjSense(-1);
        getMIPWrapper()->setProbType(-1);
        nProbType=-1;
        if (mip_wrap->fVerbose)
          std::cerr << "    MIP_solverinstance: this is a MINimization problem." << std::endl;
      }
      if (mip_wrap->fVerbose) {
        std::cerr << "    MIP_solverinstance: bounds for the objective function: "
        << dObjVarLB << ", " << dObjVarUB << std::endl;
      }
    } else {
      getMIPWrapper()->setProbType(0);
      if (mip_wrap->fVerbose)
        std::cerr << "    MIP_solverinstance: this is a SATisfiability problem." << std::endl;
    }
    
    
    lastIncumbent = 1e200;                  // for callbacks
    MIP_wrapper::Status sw;
    if ( SolverInstance::UNSAT == _status )     // already deduced - exit now
      return _status;
    if ( getMIPWrapper()->getNCols() ) {     // If any variables, we need to run solver just to get values?
      getMIPWrapper()->provideSolutionCallback(HandleSolutionCallback<MIPWrapper>, this);
      if ( cutGenerators.size() )  // only then, can modify presolve
        getMIPWrapper()->provideCutCallback(HandleCutCallback<MIPWrapper>, this);
      ////////////// clean up envi /////////////////
      {
        /// Removing for now - need access to output variables  TODO
        //       cleanupForNonincrementalSolving();
        if (GC::locked() && mip_wrap->fVerbose)
          std::cerr << "WARNING: GC is locked before SolverInstance::solve()! Wasting memory." << std::endl;
        // GCLock lock;
        GC::trigger();
      }
      getMIPWrapper()->solve();
      //   printStatistics(cout, 1);   MznSolver does this (if it wants)
      sw = getMIPWrapper()->getStatus();
    } else {
      if ( mip_wrap->fVerbose )
        std::cerr << "  MIP_solverinstance: no constraints - skipping actual solution phase." << std::endl;
      sw = MIP_wrapper::Status::OPT;
      printSolution();
    }
    SolverInstance::Status s = SolverInstance::UNKNOWN;
    switch(sw) {
      case MIP_wrapper::Status::OPT:
        if ( 0!=nProbType ) {
          s = SolverInstance::OPT;
        } else {
          s = SolverInstance::SAT;    // For SAT problems, just say SAT unless we know it's complete
        }
        break;
      case MIP_wrapper::Status::SAT:
        s = SolverInstance::SAT;
        break;
      case MIP_wrapper::Status::UNSAT:
        s = SolverInstance::UNSAT;
        break;
      case MIP_wrapper::Status::UNBND:
        s = SolverInstance::UNBND;
        break;
      case MIP_wrapper::Status::UNSATorUNBND:
        s = SolverInstance::UNSATorUNBND;
        break;
      case MIP_wrapper::Status::UNKNOWN:
        s = SolverInstance::UNKNOWN;
        break;
      default:
        s = SolverInstance::ERROR;
    }
    return s;
  }
  
  namespace SCIPConstraints {
    
    bool CheckAnnUserCut(const Call* call);
    bool CheckAnnLazyConstraint(const Call* call);
    int GetMaskConsType(const Call* call);

    /// Create constraint name
    /// Input: a prefix, a counter, and the original call.
    /// If the call has a path annotation, that is used,
    /// otherwise pfx << cnt.
    inline
    std::string makeConstrName(const char* pfx, int cnt, const Expression* cOrig=nullptr) {
      Call* mznp;
      if (nullptr!=cOrig && (mznp=cOrig->ann().getCall(constants().ann.mzn_path))) {
        assert(1==mznp->n_args());
        auto strp = mznp->arg(0)->dyn_cast<StringLit>();
        assert(strp);
        return strp->v().str().substr(0, 255);    // Gurobi 8.1 has <=255 characters
      }
      std::ostringstream ss;
      ss << pfx << cnt;
      return ss.str();
    }

    /// Gurobi 8.1.0 complains about duplicates, CPLEX 12.8.0 just ignores repeats
    /// An example for duplicated indices was on 72a9b64f with two floats equated
    template <class Idx>
    void removeDuplicates(std::vector<Idx>& rmi, std::vector<double>& rmv) {
      std::unordered_map<Idx, double> linExp;
      for (int i=rmi.size(); i--; )
        linExp[rmi[i]] += rmv[i];
      if (rmi.size()==linExp.size())
        return;
      rmi.resize(linExp.size());
      rmv.resize(linExp.size());
      int i=0;
      for (const auto& iv: linExp) {
        rmi[i] = iv.first;
        rmv[i] = iv.second;
        ++i;
      }
    }
    
    template<class MIPWrapper>
    void p_lin(SolverInstanceBase& si, const Call* call, MIP_wrapper::LinConType lt) {
      MIP_solverinstance<MIPWrapper>& gi = dynamic_cast<MIP_solverinstance<MIPWrapper>&>( si );
      Env& _env = gi.env();
      //     ArrayLit* al = eval_array_lit(_env.envi(), args[0]);
      //     int nvars = al->v().size();
      std::vector<double> coefs;
      //     gi.exprToArray(args[0], coefs);
      std::vector<typename MIP_solverinstance<MIPWrapper>::VarId> vars;
      //     gi.exprToVarArray(args[1], vars);
      IntVal ires;
      FloatVal fres;
      
      double rhs;
      if(call->arg(2)->type().isint()) {
        ires = eval_int(_env.envi(), call->arg(2));
        rhs = static_cast<double>(ires.toInt());
      } else if(call->arg(2)->type().isfloat()) {
        fres = eval_float(_env.envi(), call->arg(2));
        rhs = fres.toDouble();
      } else {
        throw InternalError("p_lin: rhs unknown type");
      }
      
      /// Process coefs & vars together to eliminate literals (problem with Gurobi's updatemodel()'s)
      ArrayLit* alC = eval_array_lit(_env.envi(), call->arg(0));
      coefs.reserve(alC->size());
      ArrayLit* alV = eval_array_lit(_env.envi(), call->arg(1));
      vars.reserve(alV->size());
      for (unsigned int i=0; i<alV->size(); i++) {
        const double dCoef = gi.exprToConst( (*alC)[i] );
        if (Id* ident = (*alV)[i]->dyn_cast<Id>()) {
          coefs.push_back( dCoef );
          vars.push_back( gi.exprToVar( ident ) );
        } else
          rhs -= dCoef*gi.exprToConst( (*alV)[i] );
      }
      assert(coefs.size() == vars.size());
      
      /// Check feas-ty
      if ( coefs.empty() ) {
        if ( (MIP_wrapper::LinConType::EQ==lt && 1e-5 < fabs( rhs ))
            || (MIP_wrapper::LinConType::LQ==lt && -1e-5 > ( rhs ))
            || (MIP_wrapper::LinConType::GQ==lt && 1e-5 < ( rhs ))
            ) {
          si._status = SolverInstance::UNSAT;
          if ( gi.getMIPWrapper()->fVerbose )
            std::cerr << "  Constraint '" << *call
            << "' seems infeasible: simplified to 0 (rel) " << rhs
            << std::endl;
        }
      } else {
        removeDuplicates(vars, coefs);
        // See if the solver adds indexation itself: no.
        gi.getMIPWrapper()->addRow(static_cast<int>(coefs.size()), &vars[0], &coefs[0], lt, rhs,
                                   GetMaskConsType(call),
            makeConstrName("p_lin_", (gi.getMIPWrapper()->nAddedRows++), call));
      }
    }
    
    template<class MIPWrapper>
    void p_int_lin_le(SolverInstanceBase& si, const Call* call) {
      p_lin<MIPWrapper>(si, call, MIP_wrapper::LQ);
    }
    template<class MIPWrapper>
    void p_int_lin_eq(SolverInstanceBase& si, const Call* call) {
      p_lin<MIPWrapper>(si, call, MIP_wrapper::EQ);
    }
    template<class MIPWrapper>
    void p_float_lin_le(SolverInstanceBase& si, const Call* call) {
      p_lin<MIPWrapper>(si, call, MIP_wrapper::LQ);
    }
    template<class MIPWrapper>
    void p_float_lin_eq(SolverInstanceBase& si, const Call* call) {
      p_lin<MIPWrapper>(si, call, MIP_wrapper::EQ);
    }
    
    // The non-_lin constraints happen in a failed model || in a non-optimized one:
    template<class MIPWrapper>
    void p_non_lin(SolverInstanceBase& si, const Call* call, MIP_wrapper::LinConType nCmp) {
      MIP_solverinstance<MIPWrapper>& gi = dynamic_cast<MIP_solverinstance<MIPWrapper>&>( si );
      std::vector<double> coefs;
      std::vector<MIP_solver::Variable> vars;
      double rhs = 0.0;
      if ( call->arg(0)->isa<Id>() ) {
        coefs.push_back( 1.0 );
        vars.push_back( gi.exprToVar(call->arg(0)) );
      } else
        rhs -= gi.exprToConst(call->arg(0));
      if ( call->arg(1)->isa<Id>() ) {
        coefs.push_back( -1.0 );
        vars.push_back( gi.exprToVar(call->arg(1)) );
      } else
        rhs += gi.exprToConst(call->arg(1));
      /// Check feas-ty
      if ( coefs.empty() ) {
        if ( (MIP_wrapper::LinConType::EQ==nCmp && 1e-5 < fabs( rhs ))
            || (MIP_wrapper::LinConType::LQ==nCmp && -1e-5 > ( rhs ))
            || (MIP_wrapper::LinConType::GQ==nCmp && 1e-5 < ( rhs ))
            ) {
          si._status = SolverInstance::UNSAT;
          if ( gi.getMIPWrapper()->fVerbose )
            std::cerr << "  Constraint '" << *call
            << "' seems infeasible: simplified to 0 (rel) " << rhs
            << std::endl;
        }
      } else {
        removeDuplicates(vars, coefs);
        gi.getMIPWrapper()->addRow(static_cast<int>(vars.size()), &vars[0], &coefs[0], nCmp, rhs,
                                   GetMaskConsType(call),
            makeConstrName("p_eq_", (gi.getMIPWrapper()->nAddedRows++), call));
      }
    }
    template<class MIPWrapper>
    void p_eq(SolverInstanceBase& si, const Call* call) {
      p_non_lin<MIPWrapper>( si, call, MIP_wrapper::EQ );
    }
    template<class MIPWrapper>
    void p_le(SolverInstanceBase& si, const Call* call) {
      p_non_lin<MIPWrapper>( si, call, MIP_wrapper::LQ );
    }
    
    /// var1<=0 if var2==0
    template<class MIPWrapper>
    void p_indicator_le0_if0(SolverInstanceBase& si, const Call* call) {
      MIP_solverinstance<MIPWrapper>& gi = dynamic_cast<MIP_solverinstance<MIPWrapper>&>( si );
      /// Looking at the bounded variable and the flag
      bool f1const=0, f2const=0;
      double val1, val2;
      MIP_solver::Variable var1, var2;
      if ( call->arg(0)->isa<Id>() ) {
        var1 = gi.exprToVar(call->arg(0));
      } else {
        f1const = 1;
        val1 = gi.exprToConst(call->arg(0));
      }
      if ( call->arg(1)->isa<Id>() ) {
        var2 = gi.exprToVar(call->arg(1));
      } else {
        f2const = 1;
        val2 = gi.exprToConst(call->arg(1));
      }
      /// Check feas-ty. 1e-6 ?????????????   TODO
      if ( f1const && f2const ) {
        if ( val1>1e-6 && val2<1e-6 ) {
          si._status = SolverInstance::UNSAT;
          if ( gi.getMIPWrapper()->fVerbose )
            std::cerr << "  Constraint '" << *call
            << "' seems infeasible: " << val2 << "==0 -> " << val1 << "<=0"
            << std::endl;
        }
      } else if ( f1const ) {
        if ( val1>1e-6 ) // so  var2==1
          gi.getMIPWrapper()->setVarBounds( var2, 1.0, 1.0 );
      } else if ( f2const ) {
        if ( val2<1e-6 )           // so  var1<=0
          gi.getMIPWrapper()->setVarUB( var1, 0.0 );
      } else {
        double coef = 1.0;
        gi.getMIPWrapper()->addIndicatorConstraint( var2, 0, 1, &var1, &coef,
                                                   MIP_wrapper::LinConType::LQ, 0.0,
                 makeConstrName("p_ind_", (gi.getMIPWrapper()->nAddedRows++), call));
        ++gi.getMIPWrapper()->nIndicatorConstr;
      }
    }
    
    /// var1==var2 if var3==1
    template<class MIPWrapper>
    void p_indicator_eq_if1(SolverInstanceBase& si, const Call* call) {
      MIP_solverinstance<MIPWrapper>& gi = dynamic_cast<MIP_solverinstance<MIPWrapper>&>( si );
      std::vector<double> coefs;
      std::vector<MIP_solver::Variable> vars;
      double rhs = 0.0;
      /// Looking at the bounded variables and the flag
      bool f1const=0, f2const=0, fBconst=0;
      double val1, val2, valB;
      MIP_solver::Variable var1, var2, varB;
      if ( call->arg(0)->isa<Id>() ) {
        var1 = gi.exprToVar(call->arg(0));
        coefs.push_back( 1.0 );
        vars.push_back( var1 );
      } else {
        f1const = 1;
        val1 = gi.exprToConst(call->arg(0));
        rhs -= val1;
      }
      if ( call->arg(1)->isa<Id>() ) {
        var2 = gi.exprToVar(call->arg(1));
        coefs.push_back( -1.0 );
        vars.push_back( var2 );
      } else {
        f2const = 1;
        val2 = gi.exprToConst(call->arg(1));
        rhs += val2;
      }
      if ( call->arg(2)->isa<Id>() ) {
        varB = gi.exprToVar(call->arg(2));
      } else {
        fBconst = 1;
        valB = gi.exprToConst(call->arg(2));
      }
      /// Check feas-ty. 1e-6 ?????????????   TODO
      if ( f1const && f2const && fBconst ) {
        if ( fabs(val1-val2)>1e-6 && val2>0.999999 ) {
          si._status = SolverInstance::UNSAT;
          if ( gi.getMIPWrapper()->fVerbose )
            std::cerr << "  Constraint '" << *call
            << "' seems infeasible: " << valB << "==0 -> " << val1 << "==" << val2
            << std::endl;
        }
      } else if ( f1const && f2const ) {
        if ( fabs(val1-val2)>1e-6 ) // so  varB=0
          gi.getMIPWrapper()->setVarBounds( varB, 0.0, 0.0 );
      } else if ( fBconst ) {
        if ( val2>0.999999 ) {          // so  var1<=0
          removeDuplicates(vars, coefs);
          gi.getMIPWrapper()->addRow(static_cast<int>(vars.size()), &vars[0], &coefs[0], MIP_wrapper::LinConType::EQ, rhs,
                                     MIP_wrapper::MaskConsType_Normal,
              makeConstrName("p_eq_", (gi.getMIPWrapper()->nAddedRows++), call));
        }
      } else {
        std::ostringstream ss;
        ss << "p_ind_" << (gi.getMIPWrapper()->nAddedRows++);
        gi.getMIPWrapper()->addIndicatorConstraint( varB, 1, static_cast<int>(coefs.size()), vars.data(), coefs.data(),
                                                   MIP_wrapper::LinConType::EQ, rhs,
                                                    makeConstrName("p_ind_", (gi.getMIPWrapper()->nAddedRows++), call));
        ++gi.getMIPWrapper()->nIndicatorConstr;
      }
    }
    
    /// Cumulative
    template<class MIPWrapper>
    void p_cumulative(SolverInstanceBase& si, const Call* call) {
      MIP_solverinstance<MIPWrapper>& gi = dynamic_cast<MIP_solverinstance<MIPWrapper>&>( si );

      std::unique_ptr<SECCutGen> pCG( new SECCutGen( gi.getMIPWrapper() ) );

      assert( call->n_args()==4 );

      std::vector<MIP_solver::Variable> startTimes;
      gi.exprToVarArray(call->arg(0), startTimes);
      std::vector<double> durations, demands;
      gi.exprToArray(call->arg(1), durations);
      gi.exprToArray(call->arg(2), demands);
      double b = gi.exprToConst(call->arg(3));

      gi.getMIPWrapper()->addCumulative(startTimes.size(), startTimes.data(), durations.data(), demands.data(), b,
                                        makeConstrName("p_cumulative_", (gi.getMIPWrapper()->nAddedRows++), call));
    }

    /// The XBZ cut generator
    template<class MIPWrapper>
    void p_XBZ_cutgen(SolverInstanceBase& si, const Call* call) {
      MIP_solverinstance<MIPWrapper>& gi = dynamic_cast<MIP_solverinstance<MIPWrapper>&>( si );
      
      //     auto pCG = make_unique<XBZCutGen>();
      std::unique_ptr<XBZCutGen> pCG( new XBZCutGen( gi.getMIPWrapper() ) );
      
      assert( call->n_args()==3 );
      gi.exprToVarArray(call->arg(0), pCG->varX);
      gi.exprToVarArray(call->arg(1), pCG->varB);
      assert(pCG->varX.size() == pCG->varB.size());
      pCG->varZ = gi.exprToVar(call->arg(2));
      //     cout << "  NEXT_CUTGEN" << endl;
      //     pCG->print( cout );
      
      gi.registerCutGenerator( move( pCG ) );
    }

    /// Initialize the SEC cut generator
    template<class MIPWrapper>
    void p_SEC_cutgen(SolverInstanceBase& si, const Call* call) {
      MIP_solverinstance<MIPWrapper>& gi = dynamic_cast<MIP_solverinstance<MIPWrapper>&>( si );
      
      std::unique_ptr<SECCutGen> pCG( new SECCutGen( gi.getMIPWrapper() ) );
      
      assert( call->n_args()==1 );
      gi.exprToVarArray(call->arg(0), pCG->varXij);            // WHAT ABOUT CONSTANTS?
      const double dN = sqrt( pCG->varXij.size() );
      MZN_ASSERT_HARD( fabs( dN - round(dN) ) < 1e-6 );   // should be a square matrix
      pCG->nN = round(dN);
      const auto sVld = pCG->validate();
      MZN_ASSERT_HARD_MSG(sVld.empty(), "ERROR(s): " << sVld);
  //     cout << "  NEXT_CUTGEN" << endl;
  //     pCG->print( cout );
      
      gi.registerCutGenerator( move( pCG ) );
    }
    
    /// SCIP's bound disj
    template<class MIPWrapper>
    void p_bounds_disj(SolverInstanceBase& si, const Call* call) {
      MIP_solverinstance<MIPWrapper>& gi = dynamic_cast<MIP_solverinstance<MIPWrapper>&>( si );
      assert(6==call->n_args());
      std::vector<double> fUB, fUBF, bnd, bndF;
      std::vector<MIP_solver::Variable> vars, varsF;
      gi.exprToArray(call->arg(0), fUB);
      gi.exprToArray(call->arg(3), fUBF);
      gi.exprToArray(call->arg(1), bnd);
      gi.exprToArray(call->arg(4), bndF);
      gi.exprToVarArray(call->arg(2), vars);
      gi.exprToVarArray(call->arg(5), varsF);
      double coef = 1.0;
      gi.getMIPWrapper()->addBoundsDisj( fUB.size(), fUB.data(), bnd.data(), vars.data(),
                                         fUBF.size(), fUBF.data(), bndF.data(), varsF.data(),
                                         makeConstrName("p_bounds_disj_", (gi.getMIPWrapper()->nAddedRows++), call));
    }

  }

  

  template<class MIPWrapper>
  void
  MIP_solverinstance<MIPWrapper>::registerConstraints() {
    GCLock lock;
    _constraintRegistry.add("int2float",    SCIPConstraints::p_eq<MIPWrapper>);
    _constraintRegistry.add("bool_eq",      SCIPConstraints::p_eq<MIPWrapper>);   // for inconsistency reported in fzn
    _constraintRegistry.add("int_eq",       SCIPConstraints::p_eq<MIPWrapper>);
    _constraintRegistry.add("int_le",       SCIPConstraints::p_le<MIPWrapper>);
    _constraintRegistry.add("int_lin_eq",   SCIPConstraints::p_int_lin_eq<MIPWrapper>);
    _constraintRegistry.add("int_lin_le",   SCIPConstraints::p_int_lin_le<MIPWrapper>);
    //   _constraintRegistry.add("int_plus",     SCIPConstraints::p_plus<MIPWrapper>);
    //   _constraintRegistry.add("bool2int",     SCIPConstraints::p_eq<MIPWrapper>);
    _constraintRegistry.add("float_eq",     SCIPConstraints::p_eq<MIPWrapper>);
    _constraintRegistry.add("float_le",     SCIPConstraints::p_le<MIPWrapper>);
    _constraintRegistry.add("float_lin_eq", SCIPConstraints::p_float_lin_eq<MIPWrapper>);
    _constraintRegistry.add("float_lin_le", SCIPConstraints::p_float_lin_le<MIPWrapper>);
    //   _constraintRegistry.add("float_plus",   SCIPConstraints::p_plus<MIPWrapper>);
    
    /// Indicators, if supported by the solver
    _constraintRegistry.add("aux_int_le_zero_if_0__IND", SCIPConstraints::p_indicator_le0_if0<MIPWrapper>);
    _constraintRegistry.add("aux_float_le_zero_if_0__IND", SCIPConstraints::p_indicator_le0_if0<MIPWrapper>);
    _constraintRegistry.add("aux_float_eq_if_1__IND", SCIPConstraints::p_indicator_eq_if1<MIPWrapper>);
    
    _constraintRegistry.add("fzn_cumulative", SCIPConstraints::p_cumulative<MIPWrapper>);

    /// XBZ cut generator
    _constraintRegistry.add("array_var_float_element__XBZ_lb__cutgen",
                            SCIPConstraints::p_XBZ_cutgen<MIPWrapper>);
    _constraintRegistry.add("circuit__SECcuts", SCIPConstraints::p_SEC_cutgen<MIPWrapper>);

    _constraintRegistry.add("bounds_disj", SCIPConstraints::p_bounds_disj<MIPWrapper>);
  }
  


}
