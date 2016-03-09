
//
//             MBAST: Mini B-language Abstract Syntax Tree
//
// It's a "mini" B-AST because the types allow more than legal B in order to
// simplify and factorize.

#![allow(dead_code)]

struct Component {
    kind: ComponentKind,
    name: String,
    formal_param: Vec<String>,
    refines: Option<String>,
 
    imports: Vec< ( String, Option<Box<Expr>> ) >,
    includes: Vec< ( String, Option<Box<Expr>> ) >,
    extends: Vec< ( String, Option<Box<Expr>> ) >,
 
    sees: Vec<String>,
    promotes: Vec<String>,
    uses: Vec<String>,
 
    abstract_constants: Vec<String>,
    abstract_variables: Vec<String>,
    concrete_constants: Vec<String>,
    concrete_variables: Vec<String>,
    constants: Vec<String>,
    variables: Vec<String>,
 
    constraints: Option<Box<Expr>>,
    properties: Option<Box<Expr>>,
    invariant: Option<Box<Expr>>,
 
    sets: Vec<Box<Expr>>,
    values: Vec<Box<Expr>>,
    assertions: Vec<Box<Expr>>,
 
    initialisation: Option<Box<Sub>>,
    operations: Vec<Box<Ope>>,
    local_operations: Vec<Box<Ope>>,
}
    
struct Ope{
    param_out: Vec<String>,
    name: String,
    param_in: Vec<String>,
    sub: Box<Sub>,
}
    
enum Sub {
    SubBin(Box<Sub>, SubBinOpe, Box<Sub>),
    
    SubBegin( Box<Sub> ),
    
    SubPreAssert( SubPreAssertOpe, Box<Expr>, Box<Sub> ),
    
    SubAnyLet( SubAnyLet, Vec<String>, Box<Expr>, Box<Sub> ),
    
    SubVar( Vec<String>, Box<Sub> ),
    
    SubChoice( Vec<Box<Sub>> ),
    
    SubIfSelect( SubIfSelect, Vec<( Box<Expr>, Box<Sub> )>, Option<Box<Sub>> ),
    
    SubCase( Box<Expr>, Vec<( Box<Expr>, Box<Sub> )>, Option<Box<Sub>> ),
    
    SubWhile( Box<Expr>, Box<Sub>, Box<Expr>, Box<Expr> ),
}
    
enum Expr {
    ExprUnary( ExprUnaryOpe, Box<Expr> ),
    
    ExprStruct( Vec< ( String, Box<Expr> ) > ),
    ExprRec( Vec< ( Option<String>, Box<Expr> ) > ),
    
    ExprBin( Box<Expr>, ExprBinOpe, Box<Expr> ),
    
    ExprId(String),
    ExprNum(u32),
    
    ExprQPred( ExprQPredOpe, Vec<String>, Box<Expr> ),
    ExprQuant( ExprQuantOpe, Vec<String>, Box<Expr>, Box<Expr> ),
}

enum ComponentKind{ Machine, Refinement, Implementation }

enum SubBinOpe{
    Sequence,       // sub ; sub
    Parallel,       // sub || sub
    Assign,         // expr := expr
    BecomeIn,       // expr :: expr
    BecomeSuchThat, // expr : ( expr )
    OperationCall,  // idList <-- id(idList)  (or variants)
}

enum SubPreAssertOpe{ Pre, Assert }
enum SubAnyLet{ Any, Let }
enum SubIfSelect{ If, Select }

enum ExprUnaryOpe
{   Minus               // -x
,   Inverse             // x~
,   Paren               // (x)
,   SetExtension        // {x}
,   ListExtension       // [x]
}

enum ExprQPredOpe{ Forall, Exist }
enum ExprQuantOpe{ Lambda, Sigma, Pi, Union, Inter }

enum ExprBinOpe
{   Apply         // f(x)
,   Image         // r[x]
,   SetCompreh    // { x | P }
,   Mod           // x mod y
,   Comma         // ,
,   Maplet        // |->
,   Plus          // +
,   Minus         // -
,   Star          // *
,   Slash         // /
,   Pow           // **
,   And           // &
,   Imply         // =>
,   Equiv         // <=>
,   Barbar        // ||
,   Semicolon     // ;
,   Equal         // =
,   Neq           // /=
,   Lt            // <
,   Leq           // <=
,   Gt            // >
,   Geq           // >=
,   Colon         // :
,   Nin           // /:
,   Incl          // <:
,   Nincl         // /<:
,   Inclstr       // <<:
,   Ninclstr      // /<<:
,   Pfun          // +->
,   Pinj          // >+>
,   Psur          // +->>
,   Pbij          // >+>>
,   Tfun          // -->
,   Tinj          // >->
,   Tsur          // -->>
,   Tbij          // >->>
,   Sinter        // \/
,   Sunion        // \/
,   Overw         // <+
,   Rel           // <->
,   Reldprod      // ><
,   Interval      // ..
,   Domrestr      // <|
,   Domsub        // <<|
,   Ranrestr      // |>
,   Ransub        // |>>
,   Larw          // <-
,   Rarw          // ->
,   Uparrow       // /|\
,   Lowarrow      // \|/
,   Caret         // ^
}