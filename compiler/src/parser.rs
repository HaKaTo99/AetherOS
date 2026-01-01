//! AetherScript Parser - Pest-based
//! Complete grammar for v1.0

use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar_inline = r#"
WHITESPACE = _{ " " | "\t" | "\n" | "\r" }
COMMENT = _{ "//" ~ (!"\n" ~ ANY)* }

// Identifiers and literals
ident = @{ ASCII_ALPHA ~ (ASCII_ALPHANUMERIC | "_")* }
number = @{ ASCII_DIGIT+ ~ ("." ~ ASCII_DIGIT+)? }
string = @{ "\"" ~ (!"\"" ~ ANY)* ~ "\"" }

// Memory sizes
memory_size = { number ~ ("." ~ ident)? }

// Annotations
annotation = { "@" ~ ident ~ ("(" ~ annotation_args ~ ")")? }
annotation_args = { annotation_arg ~ ("," ~ annotation_arg)* }
annotation_arg = { ident ~ ":" ~ (ident | number | string | "true" | "false") }

// App declaration
app = { annotation* ~ "app" ~ ident ~ "{" ~ app_member* ~ "}" }
app_member = { function | variable }

// Function declaration
function = { 
    annotation* ~ 
    ("distributed")? ~ 
    "func" ~ ident ~ 
    "(" ~ params? ~ ")" ~ 
    ("->" ~ type_expr)? ~ 
    block 
}

params = { param ~ ("," ~ param)* }
param = { ident ~ ":" ~ type_expr }

type_expr = { ident ~ ("<" ~ type_expr ~ ("," ~ type_expr)* ~ ">")? }

// Statements
block = { "{" ~ statement* ~ "}" }
statement = {
    let_stmt |
    assign_stmt |
    if_stmt |
    for_stmt |
    while_stmt |
    return_stmt |
    expr_stmt
}

let_stmt = { "let" ~ ident ~ (":" ~ type_expr)? ~ "=" ~ expr }
assign_stmt = { ident ~ "=" ~ expr }
if_stmt = { "if" ~ expr ~ block ~ ("else" ~ block)? }
for_stmt = { "for" ~ ident ~ "in" ~ expr ~ block }
while_stmt = { "while" ~ expr ~ block }
return_stmt = { "return" ~ expr? }
expr_stmt = { expr }

// Expressions
expr = { term ~ (bin_op ~ term)* }
term = { 
    number | 
    string | 
    ident ~ ("(" ~ args? ~ ")")? |
    "(" ~ expr ~ ")"
}

bin_op = { "+" | "-" | "*" | "/" | "==" | "!=" | "<" | ">" }
args = { expr ~ ("," ~ expr)* }

variable = { "var" ~ ident ~ ":" ~ type_expr }

// Entry point
program = { SOI ~ app+ ~ EOI }
"#]
pub struct AetherScriptParser;

#[derive(Debug, Clone)]
pub struct AST {
    pub apps: Vec<App>,
}

#[derive(Debug, Clone)]
pub struct App {
    pub name: String,
    pub annotations: Vec<Annotation>,
    pub members: Vec<AppMember>,
}

#[derive(Debug, Clone)]
pub enum AppMember {
    Function(Function),
    Variable(Variable),
}

#[derive(Debug, Clone)]
pub struct Function {
    pub name: String,
    pub annotations: Vec<Annotation>,
    pub is_distributed: bool,
    pub params: Vec<Param>,
    pub return_type: Option<String>,
    pub body: Block,
}

#[derive(Debug, Clone)]
pub struct Param {
    pub name: String,
    pub type_name: String,
}

#[derive(Debug, Clone)]
pub struct Block {
    pub statements: Vec<Statement>,
}

#[derive(Debug, Clone)]
pub enum Statement {
    Let { name: String, value: Expr },
    Assign { name: String, value: Expr },
    Return { value: Option<Expr> },
    Expr(Expr),
}

#[derive(Debug, Clone)]
pub enum Expr {
    Number(f64),
    String(String),
    Ident(String),
    Call { name: String, args: Vec<Expr> },
    BinOp { op: String, left: Box<Expr>, right: Box<Expr> },
}

#[derive(Debug, Clone)]
pub struct Annotation {
    pub name: String,
    pub args: Vec<(String, String)>,
}

#[derive(Debug, Clone)]
pub struct Variable {
    pub name: String,
    pub type_name: String,
}

pub fn parse(source: &str) -> Result<AST, String> {
    let pairs = AetherScriptParser::parse(Rule::program, source)
        .map_err(|e| format!("Parse error: {}", e))?;
    
    let mut apps = Vec::new();
    
    for pair in pairs {
        if pair.as_rule() == Rule::app {
            apps.push(parse_app(pair)?);
        }
    }
    
    Ok(AST { apps })
}

fn parse_app(pair: pest::iterators::Pair<Rule>) -> Result<App, String> {
    let mut name = String::new();
    let mut annotations = Vec::new();
    let mut members = Vec::new();
    
    for inner in pair.into_inner() {
        match inner.as_rule() {
            Rule::ident => name = inner.as_str().to_string(),
            Rule::annotation => annotations.push(parse_annotation(inner)?),
            Rule::app_member => {
                // Parse member
            }
            _ => {}
        }
    }
    
    Ok(App { name, annotations, members })
}

fn parse_annotation(pair: pest::iterators::Pair<Rule>) -> Result<Annotation, String> {
    let mut name = String::new();
    let args = Vec::new();
    
    for inner in pair.into_inner() {
        if inner.as_rule() == Rule::ident {
            name = inner.as_str().to_string();
        }
    }
    
    Ok(Annotation { name, args })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_simple_app() {
        let source = r#"
app HelloWorld {
    func main() {
        return 0
    }
}
"#;
        
        let ast = parse(source).unwrap();
        assert_eq!(ast.apps.len(), 1);
        assert_eq!(ast.apps[0].name, "HelloWorld");
    }

    #[test]
    fn test_parse_annotations() {
        let source = r#"
@memory(budget: 1.mb)
app TestApp {
    func test() {}
}
"#;
        
        let ast = parse(source).unwrap();
        assert_eq!(ast.apps[0].annotations.len(), 1);
    }
}
