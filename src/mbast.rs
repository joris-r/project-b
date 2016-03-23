
//
//             MBAST: Mini B-language Abstract Syntax Tree
//
// It's a "mini" B-AST because the types allow more than legal B in order to
// simplify and factorize.

#![allow(dead_code)]

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Component {
    pub kind: ComponentKind,
    pub name: String,
    pub formal_param: Vec<String>,
    pub refines: Option<String>,

    pub imports: Vec< ( String, Option<Box<Expr>> ) >,
    pub includes: Vec< ( String, Option<Box<Expr>> ) >,
    pub extends: Vec< ( String, Option<Box<Expr>> ) >,

    pub sees: Vec<String>,
    pub promotes: Vec<String>,
    pub uses: Vec<String>,

    pub abstract_constants: Vec<String>,
    pub abstract_variables: Vec<String>,
    pub concrete_constants: Vec<String>,
    pub concrete_variables: Vec<String>,
    pub constants: Vec<String>,
    pub variables: Vec<String>,

    pub constraints: Option<Box<Expr>>,
    pub properties: Option<Box<Expr>>,
    pub invariant: Option<Box<Expr>>,

    pub sets: Vec<Box<Expr>>,
    pub values: Vec<Box<Expr>>,
    pub assertions: Vec<Box<Expr>>,

    pub initialisation: Option<Box<Sub>>,
    pub operations: Vec<Box<Ope>>,
    pub local_operations: Vec<Box<Ope>>,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Ope{
    pub param_out: Vec<String>,
    pub name: String,
    pub param_in: Vec<String>,
    pub sub: Box<Sub>,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Sub {
    Begin( Box<Sub> ),

    Bin(Box<Sub>, SubBinOpe, Box<Sub>),
    Simple(Box<Expr>, SubSimple, Box<Expr>),
    OpeCall(Vec<String>, String, Vec<Box<Expr>>),

    PreAssert( SubPreAssertOpe, Box<Expr>, Box<Sub> ),

    AnyLet( SubAnyLet, Vec<String>, Box<Expr>, Box<Sub> ),

    Var( Vec<String>, Box<Sub> ),

    Choice( Vec<Box<Sub>> ),

    IfSelect( SubIfSelect, Vec<( Box<Expr>, Box<Sub> )>, Option<Box<Sub>> ),

    Case( Box<Expr>, Vec<( Box<Expr>, Box<Sub> )>, Option<Box<Sub>> ),

    While( Box<Expr>, Box<Sub>, Box<Expr>, Box<Expr> ),
}

#[derive(Debug, PartialEq, Eq, Clone)]
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

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum ComponentKind{ Machine, Refinement, Implementation }

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum SubSimple{
    Assign,         // expr := expr
    BecomeIn,       // expr :: expr
    BecomeSuchThat, // expr : ( expr )
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum SubBinOpe{
    Sequence,       // sub ; sub
    Parallel,       // sub || sub
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum SubPreAssertOpe{ Pre, Assert }
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum SubAnyLet{ Any, Let }
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum SubIfSelect{ If, Select }

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum ExprUnaryOpe
{   Minus               // -x
,   Inverse             // x~
,   Negate              // not(x)
,   Paren               // (x)
,   SetExtension        // {x}
,   ListExtension       // [x]
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum ExprQPredOpe{ Forall, Exist }
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum ExprQuantOpe{ Lambda, Sigma, Pi, Union, Inter }

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum ExprBinOpe
{   Apply         // f(x)
,   Image         // r[x]
,   SetCompreh    // { x | P }
,   Mod           // x mod y
,   Or            // x or y
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
,   Sinter        // /\
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

pub struct ComponentBuilder {
    pub refines: Option<String>,

    pub imports: Vec< ( String, Option<Box<Expr>> ) >,
    pub includes: Vec< ( String, Option<Box<Expr>> ) >,
    pub extends: Vec< ( String, Option<Box<Expr>> ) >,

    pub sees: Vec<String>,
    pub promotes: Vec<String>,
    pub uses: Vec<String>,

    pub abstract_constants: Vec<String>,
    pub abstract_variables: Vec<String>,
    pub concrete_constants: Vec<String>,
    pub concrete_variables: Vec<String>,
    pub constants: Vec<String>,
    pub variables: Vec<String>,

    pub constraints: Option<Box<Expr>>,
    pub properties: Option<Box<Expr>>,
    pub invariant: Option<Box<Expr>>,

    pub sets: Vec<Box<Expr>>,
    pub values: Vec<Box<Expr>>,
    pub assertions: Vec<Box<Expr>>,

    pub initialisation: Option<Box<Sub>>,
    pub operations: Vec<Box<Ope>>,
    pub local_operations: Vec<Box<Ope>>,
}

impl ComponentBuilder {
    pub fn new() -> ComponentBuilder {
        ComponentBuilder {
            refines: None,

            imports: vec![],
            includes: vec![],
            extends: vec![],

            sees: vec![],
            promotes: vec![],
            uses: vec![],

            abstract_constants: vec![],
            abstract_variables: vec![],
            concrete_constants: vec![],
            concrete_variables: vec![],
            constants: vec![],
            variables: vec![],

            constraints: None,
            properties : None,
            invariant  : None,

            sets: vec![],
            values: vec![],
            assertions: vec![],

            initialisation: None,
            operations: vec![],
            local_operations: vec![],
        }
    }

    pub fn finalize(self,
                    kind: ComponentKind,
                    name: String,
                    formal_param: Vec<String> ) -> Component {
        Component
        {   kind               : kind
        ,   name               : name
        ,   formal_param       : formal_param
        ,   refines            : self.refines
        ,   imports            : self.imports
        ,   includes           : self.includes
        ,   extends            : self.extends
        ,   sees               : self.sees
        ,   promotes           : self.promotes
        ,   uses               : self.uses
        ,   abstract_constants : self.abstract_constants
        ,   abstract_variables : self.abstract_variables
        ,   concrete_constants : self.concrete_constants
        ,   concrete_variables : self.concrete_variables
        ,   constants          : self.constants
        ,   variables          : self.variables
        ,   constraints        : self.constraints
        ,   properties         : self.properties
        ,   invariant          : self.invariant
        ,   sets               : self.sets
        ,   values             : self.values
        ,   assertions         : self.assertions
        ,   initialisation     : self.initialisation
        ,   operations         : self.operations
        ,   local_operations   : self.local_operations
        }
    }
}
