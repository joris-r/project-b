
//
//             MBAST: Mini B-language Abstract Syntax Tree
//
// It's a "mini" B-AST because the types allow more than legal B in order to
// simplify and factorize.

#![allow(dead_code)]

pub struct Component {
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
    
pub struct Ope{
    param_out: Vec<String>,
    name: String,
    param_in: Vec<String>,
    sub: Box<Sub>,
}
    
pub enum Sub {
    Bin(Box<Sub>, SubBinOpe, Box<Sub>),
    
    Begin( Box<Sub> ),
    
    PreAssert( SubPreAssertOpe, Box<Expr>, Box<Sub> ),
    
    AnyLet( SubAnyLet, Vec<String>, Box<Expr>, Box<Sub> ),
    
    Var( Vec<String>, Box<Sub> ),
    
    Choice( Vec<Box<Sub>> ),
    
    IfSelect( SubIfSelect, Vec<( Box<Expr>, Box<Sub> )>, Option<Box<Sub>> ),
    
    Case( Box<Expr>, Vec<( Box<Expr>, Box<Sub> )>, Option<Box<Sub>> ),
    
    While( Box<Expr>, Box<Sub>, Box<Expr>, Box<Expr> ),
}
    
pub enum Expr {
    Unary( ExprUnaryOpe, Box<Expr> ),
    
    Struct( Vec< ( String, Box<Expr> ) > ),
    Rec( Vec< ( Option<String>, Box<Expr> ) > ),
    
    Bin( Box<Expr>, ExprBinOpe, Box<Expr> ),
    
    Id(String),
    Num(String),
    
    QPred( ExprQPredOpe, Vec<String>, Box<Expr> ),
    Quant( ExprQuantOpe, Vec<String>, Box<Expr>, Box<Expr> ),
}

pub enum ComponentKind{ Machine, Refinement, Implementation }

pub enum SubBinOpe{
    Sequence,       // sub ; sub
    Parallel,       // sub || sub
    Assign,         // expr := expr
    BecomeIn,       // expr :: expr
    BecomeSuchThat, // expr : ( expr )
    OperationCall,  // idList <-- id(idList)  (or variants)
}

pub enum SubPreAssertOpe{ Pre, Assert }
pub enum SubAnyLet{ Any, Let }
pub enum SubIfSelect{ If, Select }

pub enum ExprUnaryOpe
{   Minus               // -x
,   Inverse             // x~
,   Paren               // (x)
,   SetExtension        // {x}
,   ListExtension       // [x]
}

pub enum ExprQPredOpe{ Forall, Exist }
pub enum ExprQuantOpe{ Lambda, Sigma, Pi, Union, Inter }

pub enum ExprBinOpe
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