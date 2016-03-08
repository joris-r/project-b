
//
//             MBAST: Mini B-language Abstract Syntax Tree
//

#![allow(dead_code)]

enum Mbast {

    Component
    {   kind: ComponentKind
    ,   name: String
    ,   formal_param: Vec<String>
    ,   refines: Option<String>

    ,   imports: Vec< ( String, Option<Box<Mbast>> ) >
    ,   includes: Vec< ( String, Option<Box<Mbast>> ) >
    ,   extends: Vec< ( String, Option<Box<Mbast>> ) >

    ,   sees: Vec<String>
    ,   promotes: Vec<String>
    ,   uses: Vec<String>

    ,   abstract_constants: Vec<String>
    ,   abstract_variables: Vec<String>
    ,   concrete_constants: Vec<String>
    ,   concrete_variables: Vec<String>
    ,   constants: Vec<String>
    ,   variables: Vec<String>
    
    ,   constraints: Option<Box<Mbast>>
    ,   properties: Option<Box<Mbast>>
    ,   invariant: Option<Box<Mbast>>
    
    ,   sets: Vec<Box<Mbast>>
    ,   values: Vec<Box<Mbast>>
    ,   assertions: Vec<Box<Mbast>>
    
    ,   initialisation: Option<Box<Mbast>>
    ,   operations: Vec<Box<Mbast>>
    ,   local_operations: Vec<Box<Mbast>>
    },
    
    Operation(Vec<String>, String, Vec<String>, Box<Mbast>),
    
    SubBin(Box<Mbast>, SubBinOpe, Box<Mbast>),
    
    SubBegin( Box<Mbast> ),
    
    SubPreAssert( SubPreAssertOpe, Box<Mbast>, Box<Mbast> ),
    
    SubAnyLet( SubAnyLet, Vec<String>, Box<Mbast>, Box<Mbast> ),
    
    SubVar( Vec<String>, Box<Mbast> ),
    
    SubChoice( Vec<Box<Mbast>> ),
    
    SubIfSelect( SubIfSelect, Vec<( Box<Mbast>, Box<Mbast> )>, Option<Box<Mbast>> ),
    
    SubCase( Box<Mbast>, Vec<( Box<Mbast>, Box<Mbast> )>, Option<Box<Mbast>> ),
    
    SubWhile( Box<Mbast>, Box<Mbast>, Box<Mbast>, Box<Mbast> ),
    
    ExprUnary( ExprUnaryOpe, Box<Mbast> ),
    
    ExprStruct( Vec< ( String, Box<Mbast> ) > ),
    ExprRec( Vec< ( Option<String>, Box<Mbast> ) > ),
    
    ExprBin( Box<Mbast>, ExprBinOpe, Box<Mbast> ),
    
    ExprId(String),
    ExprNum(u32),
    
    ExprQPred( ExprQPredOpe, Vec<String>, Box<Mbast> ),
    ExprQuant( ExprQuantOpe, Vec<String>, Box<Mbast>, Box<Mbast> ),
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