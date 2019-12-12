 
/* -*- mode: C++; c-basic-offset: 2; indent-tabs-mode: nil -*- */

/*
 *  Main authors:
 *     Gleb Belov <gleb.belov@monash.edu>
 */

/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

#ifndef __MIP_GUROBI_WRAPPER_H__
#define __MIP_GUROBI_WRAPPER_H__

#include <minizinc/solvers/MIP/MIP_wrap.hh>
#include <minizinc/solver_instance_base.hh>

extern "C" {
  #include <gurobi_c.h>     // need GUROBI_HOME defined
}

class MIP_gurobi_wrapper : public MIP_wrapper {
    GRBenv        * env = 0;
    GRBmodel      * model = 0;
#ifdef GUROBI_PLUGIN
    void          * gurobi_dll;
#endif
    int             error;
    std::string          gurobi_buffer;   // [GRB_MESSAGEBUFSIZE];
    std::string          gurobi_status_buffer; // [GRB_MESSAGEBUFSIZE];
    
    std::vector<double> x;

  public:

    class Options : public MiniZinc::SolverInstanceBase::Options {
    public:
      int nMIPFocus=0;
      int nFreeSearch=1;
      int nThreads=1;
      std::string sExportModel;
      int nTimeout1000=-1;
      int nTimeoutFeas1000=-1;
      long int nSolLimit = -1;
      int nSeed = -1;
      double nWorkMemLimit=0.5;
      std::string sNodefileDir;
      std::string sReadParams;
      std::string sWriteParams;
      bool flag_all_solutions = false;
      
      double absGap=-1;
      double relGap=1e-8;
      double feasTol=1e-8;
      double intTol=1e-8;
      double objDiff=1.0;
      std::string sGurobiDLL;
      bool processOption(int& i, std::vector<std::string>& argv);
      static void printHelp(std::ostream& );
    };
  private:
    Options* options=nullptr;
  public:
  
    void (__stdcall *dll_GRBversion) (int*, int*, int*);
    
    int (__stdcall *dll_GRBaddconstr) (GRBmodel *model, int numnz, int *cind, double *cval,
                             char sense, double rhs, const char *constrname);

    int (__stdcall *dll_GRBaddgenconstrIndicator) (  GRBmodel  *model, const char  *name, int binvar,
        int binval, int nvars, const int*  ind, const double* val, char  sense, double  rhs );
    
    int (__stdcall *dll_GRBaddvars) (GRBmodel *model, int numvars, int numnz,
                           int *vbeg, int *vind, double *vval,
                           double *obj, double *lb, double *ub, char *vtype,
                           char **varnames);

    int (__stdcall *dll_GRBcbcut) (void *cbdata, int cutlen, const int *cutind, const double *cutval,
                         char cutsense, double cutrhs);

    int (__stdcall *dll_GRBcbget) (void *cbdata, int where, int what, void *resultP);

    int (__stdcall *dll_GRBcblazy) (void *cbdata, int lazylen, const int *lazyind,
                          const double *lazyval, char lazysense, double lazyrhs);

    void (__stdcall *dll_GRBfreeenv) (GRBenv *env);

    int (__stdcall *dll_GRBfreemodel) (GRBmodel *model);

    int (__stdcall *dll_GRBgetdblattr) (GRBmodel *model, const char *attrname, double *valueP);

    int (__stdcall *dll_GRBgetdblattrarray) (GRBmodel *model, const char *attrname,
                                   int first, int len, double *values);

    GRBenv * (__stdcall *dll_GRBgetenv) (GRBmodel *model);

    const char * (__stdcall *dll_GRBgeterrormsg) (GRBenv *env);

    int (__stdcall *dll_GRBgetintattr) (GRBmodel *model, const char *attrname, int *valueP);

    int (__stdcall *dll_GRBloadenv) (GRBenv **envP, const char *logfilename);

    int (__stdcall *dll_GRBnewmodel) (GRBenv *env, GRBmodel **modelP, const char *Pname, int numvars,
                            double *obj, double *lb, double *ub, char *vtype,
                            char **varnames);

    int (__stdcall *dll_GRBoptimize) (GRBmodel *model);

    int (__stdcall *dll_GRBreadparams) (GRBenv *env, const char *filename);

    int (__stdcall *dll_GRBsetcallbackfunc) (GRBmodel *model,
                                   int (__stdcall *cb)(CB_ARGS),
                                   void  *usrdata);

    int (__stdcall *dll_GRBsetdblparam) (GRBenv *env, const char *paramname, double value);

    int (__stdcall *dll_GRBsetintparam) (GRBenv *env, const char *paramname, int value);

    int (__stdcall *dll_GRBsetintattr) (GRBmodel *model, const char *attrname, int newvalue);

    int (__stdcall *dll_GRBsetdblattrelement) (GRBmodel *model, const char *attrname, int iv, double v);

    int (__stdcall *dll_GRBsetintattrlist) (GRBmodel *model, const char *attrname,
                    int len, int *ind, int *newvalues);

    int (__stdcall *dll_GRBsetdblattrlist) (GRBmodel *model, const char *attrname,
                    int len, int *ind, double *newvalues);

    int (__stdcall *dll_GRBsetstrparam) (GRBenv *env, const char *paramname, const char *value);

    void (__stdcall *dll_GRBterminate) (GRBmodel* model);

    int (__stdcall *dll_GRBupdatemodel) (GRBmodel *model);

    int (__stdcall *dll_GRBwrite) (GRBmodel *model, const char *filename);

    int (__stdcall *dll_GRBwriteparams) (GRBenv *env, const char *filename);

    int (__stdcall *dll_GRBgetintparam) (GRBenv *env, const char *paramname, int *valueP);
    
  public:
    MIP_gurobi_wrapper(Options* opt) : options(opt) {
      if (opt)
        openGUROBI();
    }
    virtual ~MIP_gurobi_wrapper() { closeGUROBI(); }

    static std::string getDescription(MiniZinc::SolverInstanceBase::Options* opt=NULL);
    static std::string getVersion(MiniZinc::SolverInstanceBase::Options* opt=NULL);
    static std::string getId(void);
    static std::string getName(void);
    static std::vector<std::string> getTags(void);
    static std::vector<std::string> getStdFlags(void);
    static std::string needDllFlag(void);
//       Statistics& getStatistics() { return _statistics; }

//      IloConstraintArray *userCuts, *lazyConstraints;

    /// derived should overload and call the ancestor
//     virtual void cleanup();
    
    void checkDLL();
    void openGUROBI();
    void closeGUROBI();
    
    /// actual adding new variables to the solver
    virtual void doAddVars(size_t n, double *obj, double *lb, double *ub,
      VarType *vt, std::string *names);

    /// adding a linear constraint
    virtual void addRow(int nnz, int *rmatind, double* rmatval,
                        LinConType sense, double rhs,
                        int mask = MaskConsType_Normal,
                        std::string rowName = "");
    virtual void setVarBounds( int iVar, double lb, double ub );
    virtual void setVarLB( int iVar, double lb );
    virtual void setVarUB( int iVar, double ub );
    /// Indicator constraint: x[iBVar]==bVal -> lin constr
    virtual void addIndicatorConstraint(int iBVar, int bVal, int nnz, int *rmatind, double* rmatval,
                        LinConType sense, double rhs,
                        std::string rowName = "");
    virtual int getFreeSearch();
    virtual bool addSearch( const std::vector<VarId>& vars, const std::vector<int> pri );
    virtual bool addWarmStart( const std::vector<VarId>& vars, const std::vector<double> vals );
    int nRows=0;    // to count rows in order tp notice lazy constraints
    std::vector<int> nLazyIdx;
    std::vector<int> nLazyValue;
    /// adding an implication
//     virtual void addImpl() = 0;
    virtual void setObjSense(int s);   // +/-1 for max/min
    
    virtual double getInfBound() { return GRB_INFINITY; }
                        
    virtual int getNCols() {
      dll_GRBupdatemodel(model);
      int cols; error = dll_GRBgetintattr(model, GRB_INT_ATTR_NUMVARS, &cols); return cols;
    }
    virtual int getNRows() {
      dll_GRBupdatemodel(model);
      int cols; error = dll_GRBgetintattr(model, GRB_INT_ATTR_NUMCONSTRS, &cols); return cols;
    }
                        
//     void setObjUB(double ub) { objUB = ub; }
//     void addQPUniform(double c) { qpu = c; } // also sets problem type to MIQP unless c=0

    virtual void solve(); 
    
    /// OUTPUT:
    virtual const double* getValues() { return output.x; }
    virtual double getObjValue() { return output.objVal; }
    virtual double getBestBound() { return output.bestBound; }
    virtual double getCPUTime() { return output.dCPUTime; }
    
    virtual Status getStatus()  { return output.status; }
    virtual std::string getStatusName() { return output.statusName; }

     virtual int getNNodes() { return output.nNodes; }
     virtual int getNOpen() { return output.nOpenNodes; }

//     virtual int getNNodes() = 0;
//     virtual double getTime() = 0;
    
  protected:
    void wrap_assert(bool , std::string , bool fTerm=true);
    
    /// Need to consider the 100 status codes in GUROBI and change with every version? TODO
    Status convertStatus(int gurobiStatus);
};

#endif  // __MIP_GUROBI_WRAPPER_H__
