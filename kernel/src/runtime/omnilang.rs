//! OmniLang Native Runtime (Source: https://github.com/HaKaTo99/OmniLang)
//! Official Integration Layer for AetherOS v7.0 Kernel

use alloc::string::String;
use alloc::vec::Vec;
use alloc::format;

// ===========================
// Lexer
// ===========================

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    // Keywords
    Fn, Let, If, Else, Return, While, For, Struct,
    // Annotations
    AtMemory,       // @memory
    AtDistributed,  // @distributed
    AtRealtime,     // @realtime
    // Literals
    IntLit(i64),
    FloatLit(i64), // Fixed-point (no f64 in no_std)
    StringLit(String),
    Ident(String),
    // Operators
    Plus, Minus, Star, Slash, Eq, EqEq, Lt, Gt, Arrow,
    // Delimiters
    LParen, RParen, LBrace, RBrace, Comma, Colon, Semi,
    // Special
    Eof,
}

pub struct Lexer {
    source: Vec<u8>,
    pos: usize,
}

impl Lexer {
    pub fn new(source: &str) -> Self {
        Self { source: source.bytes().collect(), pos: 0 }
    }

    fn peek(&self) -> Option<u8> {
        self.source.get(self.pos).copied()
    }

    fn advance(&mut self) -> Option<u8> {
        let ch = self.source.get(self.pos).copied();
        self.pos += 1;
        ch
    }

    fn skip_whitespace(&mut self) {
        while let Some(ch) = self.peek() {
            if ch == b' ' || ch == b'\n' || ch == b'\r' || ch == b'\t' {
                self.advance();
            } else { break; }
        }
    }

    pub fn tokenize(&mut self) -> Vec<Token> {
        let mut tokens = Vec::new();
        loop {
            self.skip_whitespace();
            match self.peek() {
                None => { tokens.push(Token::Eof); break; }
                Some(b'+') => { self.advance(); tokens.push(Token::Plus); }
                Some(b'-') => {
                    self.advance();
                    if self.peek() == Some(b'>') { self.advance(); tokens.push(Token::Arrow); }
                    else { tokens.push(Token::Minus); }
                }
                Some(b'*') => { self.advance(); tokens.push(Token::Star); }
                Some(b'/') => { self.advance(); tokens.push(Token::Slash); }
                Some(b'=') => {
                    self.advance();
                    if self.peek() == Some(b'=') { self.advance(); tokens.push(Token::EqEq); }
                    else { tokens.push(Token::Eq); }
                }
                Some(b'<') => { self.advance(); tokens.push(Token::Lt); }
                Some(b'>') => { self.advance(); tokens.push(Token::Gt); }
                Some(b'(') => { self.advance(); tokens.push(Token::LParen); }
                Some(b')') => { self.advance(); tokens.push(Token::RParen); }
                Some(b'{') => { self.advance(); tokens.push(Token::LBrace); }
                Some(b'}') => { self.advance(); tokens.push(Token::RBrace); }
                Some(b',') => { self.advance(); tokens.push(Token::Comma); }
                Some(b':') => { self.advance(); tokens.push(Token::Colon); }
                Some(b';') => { self.advance(); tokens.push(Token::Semi); }
                Some(b'@') => {
                    self.advance();
                    let word = self.read_ident();
                    match word.as_str() {
                        "memory" => tokens.push(Token::AtMemory),
                        "distributed" => tokens.push(Token::AtDistributed),
                        "realtime" => tokens.push(Token::AtRealtime),
                        _ => tokens.push(Token::Ident(word)),
                    }
                }
                Some(b'"') => {
                    let s = self.read_string();
                    tokens.push(Token::StringLit(s));
                }
                Some(ch) if ch.is_ascii_digit() => {
                    let n = self.read_number();
                    tokens.push(Token::IntLit(n));
                }
                Some(ch) if ch.is_ascii_alphabetic() || ch == b'_' => {
                    let word = self.read_ident();
                    match word.as_str() {
                        "fn" => tokens.push(Token::Fn),
                        "let" => tokens.push(Token::Let),
                        "if" => tokens.push(Token::If),
                        "else" => tokens.push(Token::Else),
                        "return" => tokens.push(Token::Return),
                        "while" => tokens.push(Token::While),
                        "for" => tokens.push(Token::For),
                        "struct" => tokens.push(Token::Struct),
                        _ => tokens.push(Token::Ident(word)),
                    }
                }
                Some(_) => { self.advance(); } // Skip unknown
            }
        }
        tokens
    }

    fn read_ident(&mut self) -> String {
        let mut s = String::new();
        while let Some(ch) = self.peek() {
            if ch.is_ascii_alphanumeric() || ch == b'_' {
                s.push(ch as char);
                self.advance();
            } else { break; }
        }
        s
    }

    fn read_number(&mut self) -> i64 {
        let mut n: i64 = 0;
        while let Some(ch) = self.peek() {
            if ch.is_ascii_digit() {
                n = n * 10 + (ch - b'0') as i64;
                self.advance();
            } else { break; }
        }
        n
    }

    fn read_string(&mut self) -> String {
        self.advance(); // skip opening quote
        let mut s = String::new();
        while let Some(ch) = self.peek() {
            if ch == b'"' {
                self.advance();
                break;
            }
            s.push(ch as char);
            self.advance();
        }
        s
    }
}

// ===========================
// AST (Abstract Syntax Tree)
// ===========================

#[derive(Debug, Clone)]
pub enum Expr {
    IntLit(i64),
    StringLit(String),
    Ident(String),
    BinOp(Vec<Expr>, BinOp, Vec<Expr>), // Vec<Expr> with 1 element instead of Box
    Call(String, Vec<Expr>),
    If(Vec<Expr>, Vec<Stmt>, Option<Vec<Stmt>>), // condition as Vec<Expr>
}

#[derive(Debug, Clone, Copy)]
pub enum BinOp { Add, Sub, Mul, Div, Eq, Lt, Gt }

#[derive(Debug, Clone)]
pub enum Stmt {
    Let(String, Expr),
    Return(Expr),
    Expr(Expr),
    While(Expr, Vec<Stmt>),
}

#[derive(Debug, Clone)]
pub struct Function {
    pub name: String,
    pub params: Vec<(String, String)>, // (name, type)
    pub return_type: Option<String>,
    pub body: Vec<Stmt>,
    pub annotations: Vec<Annotation>,
}

#[derive(Debug, Clone)]
pub enum Annotation { Memory, Distributed, Realtime }

#[derive(Debug, Clone)]
pub struct Module {
    pub functions: Vec<Function>,
}

// ===========================
// Parser (simplified)
// ===========================

pub struct Parser {
    tokens: Vec<Token>,
    pos: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, pos: 0 }
    }

    fn peek(&self) -> &Token {
        self.tokens.get(self.pos).unwrap_or(&Token::Eof)
    }

    fn advance(&mut self) -> Token {
        let tok = self.tokens.get(self.pos).cloned().unwrap_or(Token::Eof);
        self.pos += 1;
        tok
    }

    pub fn parse_module(&mut self) -> Module {
        let mut functions = Vec::new();
        while *self.peek() != Token::Eof {
            if let Some(func) = self.parse_function() {
                functions.push(func);
            } else {
                self.advance(); // Skip unknown
            }
        }
        Module { functions }
    }

    fn parse_function(&mut self) -> Option<Function> {
        // Collect annotations
        let mut annotations = Vec::new();
        while matches!(self.peek(), Token::AtMemory | Token::AtDistributed | Token::AtRealtime) {
            match self.advance() {
                Token::AtMemory => annotations.push(Annotation::Memory),
                Token::AtDistributed => annotations.push(Annotation::Distributed),
                Token::AtRealtime => annotations.push(Annotation::Realtime),
                _ => {}
            }
        }

        if *self.peek() != Token::Fn { return None; }
        self.advance(); // consume 'fn'

        let name = match self.advance() {
            Token::Ident(n) => n,
            _ => return None,
        };

        // Parse params
        if *self.peek() == Token::LParen { self.advance(); }
        let mut params = Vec::new();
        while *self.peek() != Token::RParen && *self.peek() != Token::Eof {
            if let Token::Ident(pname) = self.advance() {
                if *self.peek() == Token::Colon { self.advance(); }
                if let Token::Ident(ptype) = self.advance() {
                    params.push((pname, ptype));
                }
            }
            if *self.peek() == Token::Comma { self.advance(); }
        }
        if *self.peek() == Token::RParen { self.advance(); }

        // Return type
        let return_type = if *self.peek() == Token::Arrow {
            self.advance();
            match self.advance() {
                Token::Ident(t) => Some(t),
                _ => None,
            }
        } else { None };

        // Body (simplified: parse until matching brace)
        if *self.peek() == Token::LBrace { self.advance(); }
        
        // Very simplified body parsing for demo
        // In a real implementation, we would call parse_stmt recursively
        let mut body = Vec::new();
        
        while *self.peek() != Token::RBrace && *self.peek() != Token::Eof {
             if let Some(stmt) = self.parse_stmt() {
                 body.push(stmt);
             } else {
                 self.advance(); // Skip to next
             }
        }
        
        if *self.peek() == Token::RBrace { self.advance(); }

        Some(Function { name, params, return_type, body, annotations })
    }

    fn parse_stmt(&mut self) -> Option<Stmt> {
        match self.peek() {
            Token::Let => {
                self.advance();
                let name = match self.advance() {
                    Token::Ident(n) => n,
                    _ => return None,
                };
                if *self.peek() == Token::Eq { self.advance(); }
                let expr = self.parse_expr()?;
                if *self.peek() == Token::Semi { self.advance(); }
                Some(Stmt::Let(name, expr))
            },
            Token::Return => {
                self.advance();
                let expr = self.parse_expr()?;
                if *self.peek() == Token::Semi { self.advance(); }
                Some(Stmt::Return(expr))
            },
             Token::Ident(_) => {
                // assume expression statement
                let expr = self.parse_expr()?;
                if *self.peek() == Token::Semi { self.advance(); }
                Some(Stmt::Expr(expr))
             }
            _ => None
        }
    }

    fn parse_expr(&mut self) -> Option<Expr> {
        self.parse_primary()
    }

    fn parse_primary(&mut self) -> Option<Expr> {
        match self.peek() {
            Token::IntLit(n) => {
                let val = *n;
                self.advance();
                Some(Expr::IntLit(val))
            },
            Token::StringLit(s) => {
                 let val = s.clone();
                 self.advance();
                 Some(Expr::StringLit(val))
            }
            Token::Ident(name) => {
                let n = name.clone();
                self.advance();
                if *self.peek() == Token::LParen {
                    self.advance();
                    // Parse args
                    let mut args = Vec::new();
                    while *self.peek() != Token::RParen {
                         if let Some(arg) = self.parse_expr() {
                             args.push(arg);
                         }
                         if *self.peek() == Token::Comma { self.advance(); }
                         else { break; }
                    }
                    if *self.peek() == Token::RParen { self.advance(); }
                    Some(Expr::Call(n, args))
                } else {
                    Some(Expr::Ident(n))
                }
            },
            _ => None
        }
    }
}

// ===========================
// Runtime (Interpreter)
// ===========================

pub struct OmniRuntime {
    pub last_output: String,
}

impl OmniRuntime {
    pub fn new() -> Self {
        Self { last_output: String::new() }
    }

    pub fn execute(&mut self, source: &str) -> String {
        let mut lexer = Lexer::new(source);
        let tokens = lexer.tokenize();
        let mut parser = Parser::new(tokens);
        let module = parser.parse_module();

        self.last_output.clear();

        // Interpreter
        for func in module.functions {
            if func.name == "main" {
                self.eval_body(&func.body);
            }
        }

        if self.last_output.is_empty() {
            String::from("Success (No Output)")
        } else {
            self.last_output.clone()
        }
    }

    fn eval_body(&mut self, body: &[Stmt]) {
        for stmt in body {
            match stmt {
                Stmt::Expr(expr) => {
                    self.eval_expr(expr);
                },
                Stmt::Let(_, expr) => {
                    self.eval_expr(expr);
                },
                _ => {}
            }
        }
    }

    fn eval_expr(&mut self, expr: &Expr) -> i64 {
        match expr {
            Expr::IntLit(n) => *n,
            Expr::StringLit(_) => 0, // Strings are 0 for now in this toy interpreter
            Expr::Call(name, args) => {
                if name == "print" {
                    if let Some(first) = args.first() {
                        match first {
                            Expr::StringLit(s) => self.last_output.push_str(s),
                            Expr::IntLit(n) => self.last_output.push_str(&format!("{}", n)),
                             _ => {}
                        }
                    }
                    0
                } else if name == "version" {
                     100 // v1.0.0
                } else if name == "System.shutdown" {
                    crate::enterprise::lifecycle::shutdown();
                    0
                } else if name == "System.logout" {
                    // Simulation of logout
                    self.last_output = String::from("User logged out.");
                    0
                } else if name == "System.input" {
                    // In a real environment, this would wait for keyboard input.
                    // For the kernel boot simulation, we return a mock value.
                    self.last_output = String::from("root"); 
                    0
                } else {
                    0
                }
            },
            _ => 0,
        }
    }

    /// Compile OmniLang to WebAssembly (Phase 27.3)
    pub fn compile_to_wasm(&self, _source: &str) -> Vec<u8> {
        let mut wasm_binary = Vec::new();
        // Magic Numbers for WASM (\0asm)
        wasm_binary.extend_from_slice(&[0x00, 0x61, 0x73, 0x6D]);
        wasm_binary.extend_from_slice(&[0x01, 0x00, 0x00, 0x00]);
        // Mock compilation logic
        wasm_binary
    }

    /// Compile OmniLang to JVM Bytecode (Phase 27.3)
    pub fn compile_to_jvm(&self, _source: &str) -> Vec<u8> {
        let mut jvm_binary = Vec::new();
        // Magic Number for JVM (CAFEBABE)
        jvm_binary.extend_from_slice(&[0xCA, 0xFE, 0xBA, 0xBE]);
        jvm_binary
    }
}
