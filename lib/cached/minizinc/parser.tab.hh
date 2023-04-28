/* A Bison parser, made by GNU Bison 3.7.6.  */

/* Bison interface for Yacc-like parsers in C

   Copyright (C) 1984, 1989-1990, 2000-2015, 2018-2021 Free Software Foundation,
   Inc.

   This program is free software: you can redistribute it and/or modify
   it under the terms of the GNU General Public License as published by
   the Free Software Foundation, either version 3 of the License, or
   (at your option) any later version.

   This program is distributed in the hope that it will be useful,
   but WITHOUT ANY WARRANTY; without even the implied warranty of
   MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
   GNU General Public License for more details.

   You should have received a copy of the GNU General Public License
   along with this program.  If not, see <https://www.gnu.org/licenses/>.  */

/* As a special exception, you may create a larger work that contains
   part or all of the Bison parser skeleton and distribute that work
   under terms of your choice, so long as that work isn't itself a
   parser generator using the skeleton or a modified version thereof
   as a parser skeleton.  Alternatively, if you modify or redistribute
   the parser skeleton itself, you may (at your option) remove this
   special exception, which will cause the skeleton and the resulting
   Bison output files to be licensed under the GNU General Public
   License without this special exception.

   This special exception was added by the Free Software Foundation in
   version 2.2 of Bison.  */

/* DO NOT RELY ON FEATURES THAT ARE NOT DOCUMENTED in the manual,
   especially those whose name start with YY_ or yy_.  They are
   private implementation details that can be changed or removed.  */

#ifndef YY_MZN_YY_USERS_GUIDOT_PROGRAMMING_MINIZINC_LIBMZN_BUILD_XCODE_INCLUDE_MINIZINC_PARSER_TAB_HH_INCLUDED
# define YY_MZN_YY_USERS_GUIDOT_PROGRAMMING_MINIZINC_LIBMZN_BUILD_XCODE_INCLUDE_MINIZINC_PARSER_TAB_HH_INCLUDED
/* Debug traces.  */
#ifndef YYDEBUG
# define YYDEBUG 0
#endif
#if YYDEBUG
extern int mzn_yydebug;
#endif

/* Token kinds.  */
#ifndef YYTOKENTYPE
# define YYTOKENTYPE
  enum yytokentype
  {
    YYEMPTY = -2,
    END = 0,                       /* "end of file"  */
    YYerror = 256,                 /* error  */
    YYUNDEF = 257,                 /* "invalid token"  */
    MZN_INTEGER_LITERAL = 258,     /* "integer literal"  */
    MZN_BOOL_LITERAL = 259,        /* "bool literal"  */
    MZN_FLOAT_LITERAL = 260,       /* "float literal"  */
    MZN_IDENTIFIER = 261,          /* "identifier"  */
    MZN_QUOTED_IDENTIFIER = 262,   /* "quoted identifier"  */
    MZN_STRING_LITERAL = 263,      /* "string literal"  */
    MZN_STRING_QUOTE_START = 264,  /* "interpolated string start"  */
    MZN_STRING_QUOTE_MID = 265,    /* "interpolated string middle"  */
    MZN_STRING_QUOTE_END = 266,    /* "interpolated string end"  */
    MZN_TI_IDENTIFIER = 267,       /* "type-inst identifier"  */
    MZN_TI_ENUM_IDENTIFIER = 268,  /* "type-inst enum identifier"  */
    MZN_DOC_COMMENT = 269,         /* "documentation comment"  */
    MZN_DOC_FILE_COMMENT = 270,    /* "file-level documentation comment"  */
    MZN_FIELD_TAIL = 271,          /* "field access"  */
    MZN_VAR = 272,                 /* "var"  */
    MZN_PAR = 273,                 /* "par"  */
    MZN_ABSENT = 274,              /* "<>"  */
    MZN_ANN = 275,                 /* "ann"  */
    MZN_ANNOTATION = 276,          /* "annotation"  */
    MZN_ANY = 277,                 /* "any"  */
    MZN_ARRAY = 278,               /* "array"  */
    MZN_BOOL = 279,                /* "bool"  */
    MZN_CASE = 280,                /* "case"  */
    MZN_CONSTRAINT = 281,          /* "constraint"  */
    MZN_DEFAULT = 282,             /* "default"  */
    MZN_ELSE = 283,                /* "else"  */
    MZN_ELSEIF = 284,              /* "elseif"  */
    MZN_ENDIF = 285,               /* "endif"  */
    MZN_ENUM = 286,                /* "enum"  */
    MZN_FLOAT = 287,               /* "float"  */
    MZN_FUNCTION = 288,            /* "function"  */
    MZN_IF = 289,                  /* "if"  */
    MZN_INCLUDE = 290,             /* "include"  */
    MZN_INFINITY = 291,            /* "infinity"  */
    MZN_INT = 292,                 /* "int"  */
    MZN_LET = 293,                 /* "let"  */
    MZN_LIST = 294,                /* "list"  */
    MZN_MAXIMIZE = 295,            /* "maximize"  */
    MZN_MINIMIZE = 296,            /* "minimize"  */
    MZN_OF = 297,                  /* "of"  */
    MZN_OPT = 298,                 /* "opt"  */
    MZN_SATISFY = 299,             /* "satisfy"  */
    MZN_OUTPUT = 300,              /* "output"  */
    MZN_PREDICATE = 301,           /* "predicate"  */
    MZN_RECORD = 302,              /* "record"  */
    MZN_SET = 303,                 /* "set"  */
    MZN_SOLVE = 304,               /* "solve"  */
    MZN_STRING = 305,              /* "string"  */
    MZN_TEST = 306,                /* "test"  */
    MZN_THEN = 307,                /* "then"  */
    MZN_TUPLE = 308,               /* "tuple"  */
    MZN_TYPE = 309,                /* "type"  */
    MZN_UNDERSCORE = 310,          /* "_"  */
    MZN_VARIANT_RECORD = 311,      /* "variant_record"  */
    MZN_WHERE = 312,               /* "where"  */
    MZN_LEFT_BRACKET = 313,        /* "["  */
    MZN_LEFT_2D_BRACKET = 314,     /* "[|"  */
    MZN_RIGHT_BRACKET = 315,       /* "]"  */
    MZN_RIGHT_2D_BRACKET = 316,    /* "|]"  */
    QUOTED_IDENTIFIER = 317,       /* QUOTED_IDENTIFIER  */
    MZN_INVALID_INTEGER_LITERAL = 318, /* "invalid integer literal"  */
    MZN_INVALID_FLOAT_LITERAL = 319, /* "invalid float literal"  */
    MZN_UNTERMINATED_STRING = 320, /* "unterminated string"  */
    MZN_END_OF_LINE_IN_STRING = 321, /* "end of line inside string literal"  */
    MZN_INVALID_NULL = 322,        /* "null character"  */
    MZN_INVALID_STRING_LITERAL = 323, /* "invalid string literal"  */
    MZN_EQUIV = 324,               /* "<->"  */
    MZN_IMPL = 325,                /* "->"  */
    MZN_RIMPL = 326,               /* "<-"  */
    MZN_OR = 327,                  /* "\\/"  */
    MZN_XOR = 328,                 /* "xor"  */
    MZN_AND = 329,                 /* "/\\"  */
    MZN_LE = 330,                  /* "<"  */
    MZN_GR = 331,                  /* ">"  */
    MZN_LQ = 332,                  /* "<="  */
    MZN_GQ = 333,                  /* ">="  */
    MZN_EQ = 334,                  /* "="  */
    MZN_NQ = 335,                  /* "!="  */
    MZN_WEAK_EQ = 336,             /* "~="  */
    MZN_WEAK_NQ = 337,             /* "~!="  */
    MZN_IN = 338,                  /* "in"  */
    MZN_SUBSET = 339,              /* "subset"  */
    MZN_SUPERSET = 340,            /* "superset"  */
    MZN_UNION = 341,               /* "union"  */
    MZN_DIFF = 342,                /* "diff"  */
    MZN_SYMDIFF = 343,             /* "symdiff"  */
    MZN_DOTDOT = 344,              /* ".."  */
    MZN_DOTDOT_LE = 345,           /* "..<"  */
    MZN_LE_DOTDOT = 346,           /* "<.."  */
    MZN_LE_DOTDOT_LE = 347,        /* "<..<"  */
    MZN_PLUS = 348,                /* "+"  */
    MZN_MINUS = 349,               /* "-"  */
    MZN_WEAK_PLUS = 350,           /* "~+"  */
    MZN_WEAK_MINUS = 351,          /* "~-"  */
    MZN_MULT = 352,                /* "*"  */
    MZN_DIV = 353,                 /* "/"  */
    MZN_IDIV = 354,                /* "div"  */
    MZN_MOD = 355,                 /* "mod"  */
    MZN_WEAK_DIV = 356,            /* "~/"  */
    MZN_WEAK_IDIV = 357,           /* "~div"  */
    MZN_INTERSECT = 358,           /* "intersect"  */
    MZN_WEAK_MULT = 359,           /* "~*"  */
    MZN_POW = 360,                 /* "^"  */
    MZN_POW_MINUS1 = 361,          /* "^-1"  */
    MZN_NOT = 362,                 /* "not"  */
    MZN_PLUSPLUS = 363,            /* "++"  */
    MZN_COLONCOLON = 364,          /* "::"  */
    PREC_ANNO = 365,               /* PREC_ANNO  */
    MZN_EQUIV_QUOTED = 366,        /* "'<->'"  */
    MZN_IMPL_QUOTED = 367,         /* "'->'"  */
    MZN_RIMPL_QUOTED = 368,        /* "'<-'"  */
    MZN_OR_QUOTED = 369,           /* "'\\/'"  */
    MZN_XOR_QUOTED = 370,          /* "'xor'"  */
    MZN_AND_QUOTED = 371,          /* "'/\\'"  */
    MZN_LE_QUOTED = 372,           /* "'<'"  */
    MZN_GR_QUOTED = 373,           /* "'>'"  */
    MZN_LQ_QUOTED = 374,           /* "'<='"  */
    MZN_GQ_QUOTED = 375,           /* "'>='"  */
    MZN_EQ_QUOTED = 376,           /* "'='"  */
    MZN_NQ_QUOTED = 377,           /* "'!='"  */
    MZN_IN_QUOTED = 378,           /* "'in'"  */
    MZN_SUBSET_QUOTED = 379,       /* "'subset'"  */
    MZN_SUPERSET_QUOTED = 380,     /* "'superset'"  */
    MZN_UNION_QUOTED = 381,        /* "'union'"  */
    MZN_DIFF_QUOTED = 382,         /* "'diff'"  */
    MZN_SYMDIFF_QUOTED = 383,      /* "'symdiff'"  */
    MZN_DOTDOT_QUOTED = 384,       /* "'..'"  */
    MZN_LE_DOTDOT_QUOTED = 385,    /* "'<..'"  */
    MZN_DOTDOT_LE_QUOTED = 386,    /* "'..<'"  */
    MZN_LE_DOTDOT_LE_QUOTED = 387, /* "'<..<'"  */
    MZN_PLUS_QUOTED = 388,         /* "'+'"  */
    MZN_MINUS_QUOTED = 389,        /* "'-'"  */
    MZN_MULT_QUOTED = 390,         /* "'*'"  */
    MZN_DIV_QUOTED = 391,          /* "'/'"  */
    MZN_IDIV_QUOTED = 392,         /* "'div'"  */
    MZN_MOD_QUOTED = 393,          /* "'mod'"  */
    MZN_INTERSECT_QUOTED = 394,    /* "'intersect'"  */
    MZN_POW_QUOTED = 395,          /* "'^'"  */
    MZN_NOT_QUOTED = 396,          /* "'not'"  */
    MZN_COLONCOLON_QUOTED = 397,   /* "'::'"  */
    MZN_PLUSPLUS_QUOTED = 398      /* "'++'"  */
  };
  typedef enum yytokentype yytoken_kind_t;
#endif

/* Value type.  */
#if ! defined YYSTYPE && ! defined YYSTYPE_IS_DECLARED
union YYSTYPE
{
 long long int iValue; char* sValue; bool bValue; double dValue;
         MiniZinc::Item* item;
         MiniZinc::VarDecl* vardeclexpr;
         std::vector<MiniZinc::VarDecl*>* vardeclexprs;
         MiniZinc::TypeInst* tiexpr;
         std::vector<MiniZinc::TypeInst*>* tiexprs;
         MiniZinc::Expression* expression;
         std::vector<MiniZinc::Expression*>* expressions1d;
         std::vector<std::vector<MiniZinc::Expression*> >* expressions2d;
         std::vector<std::pair<std::vector<MiniZinc::Expression*>,std::vector<MiniZinc::Expression*>>>* indexedexpressions2d;
         std::pair<std::vector<MiniZinc::Expression*>,std::vector<MiniZinc::Expression*>>* indexedexpression2d;
         std::vector<std::vector<std::vector<MiniZinc::Expression*> > >* expressions3d;
         MiniZinc::Generator* generator;
         std::vector<MiniZinc::Generator>* generators;
         std::vector<std::string>* strings;
         std::vector<std::pair<MiniZinc::Expression*,MiniZinc::Expression*> >* expressionPairs;
         MiniZinc::Generators* generatorsPointer;
       


};
typedef union YYSTYPE YYSTYPE;
# define YYSTYPE_IS_TRIVIAL 1
# define YYSTYPE_IS_DECLARED 1
#endif

/* Location type.  */
#if ! defined YYLTYPE && ! defined YYLTYPE_IS_DECLARED
typedef struct YYLTYPE YYLTYPE;
struct YYLTYPE
{
  int first_line;
  int first_column;
  int last_line;
  int last_column;
};
# define YYLTYPE_IS_DECLARED 1
# define YYLTYPE_IS_TRIVIAL 1
#endif



int mzn_yyparse (void *parm);

#endif /* !YY_MZN_YY_USERS_GUIDOT_PROGRAMMING_MINIZINC_LIBMZN_BUILD_XCODE_INCLUDE_MINIZINC_PARSER_TAB_HH_INCLUDED  */
