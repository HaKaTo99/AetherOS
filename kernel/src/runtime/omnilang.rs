//! OmniLang Native Runtime (Source: https://github.com/HaKaTo99/OmniLang)
//! Official Integration Layer for AetherOS v7.0 Kernel

use alloc::string::String;
use alloc::vec::Vec;
use alloc::format;
use alloc::collections::BTreeMap;

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Int(i64),
    String(String),
}

impl Value {
    pub fn as_int(&self) -> i64 {
        match self {
            Value::Int(n) => *n,
            Value::String(_) => 0,
        }
    }
    
    pub fn as_bool(&self) -> bool {
        match self {
            Value::Int(n) => *n != 0,
            Value::String(s) => !s.is_empty(),
        }
    }
}

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
    And, Or, Not, NotEq, // &&, ||, !, !=
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
                Some(b'&') => {
                    self.advance();
                    if self.peek() == Some(b'&') { self.advance(); tokens.push(Token::And); }
                    else { /* Bitwise and not supported yet, skip or return error */ }
                }
                Some(b'|') => {
                    self.advance();
                    if self.peek() == Some(b'|') { self.advance(); tokens.push(Token::Or); }
                    else { /* Bitwise or not supported yet */ }
                }
                Some(b'!') => {
                    self.advance();
                    if self.peek() == Some(b'=') { self.advance(); tokens.push(Token::NotEq); }
                    else { tokens.push(Token::Not); }
                }
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
            // Allow dotted identifiers so namespaced builtins like System.input stay intact
            if ch.is_ascii_alphanumeric() || ch == b'_' || ch == b'.' {
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
    UnaryOp(UnaryOp, Vec<Expr>),
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum BinOp { Add, Sub, Mul, Div, Eq, NotEq, Lt, Gt, And, Or }

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum UnaryOp { Not }

#[derive(Debug, Clone)]
pub enum Stmt {
    Let(String, Expr),
    Return(Expr),
    Expr(Expr),
    While(Expr, Vec<Stmt>),
    If(Expr, Vec<Stmt>, Option<Vec<Stmt>>),
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
            Token::If => {
                self.advance(); // consume 'if'
                if *self.peek() == Token::LParen { self.advance(); }
                let cond = self.parse_expr()?;
                if *self.peek() == Token::RParen { self.advance(); }
                
                let then_body = self.parse_block();
                
                let mut else_body = None;
                if *self.peek() == Token::Else {
                    self.advance();
                    // Handle 'else if' or just 'else'
                    if *self.peek() == Token::If {
                         if let Some(s) = self.parse_stmt() {
                             else_body = Some(vec![s]);
                         }
                    } else {
                         else_body = Some(self.parse_block());
                    }
                }
                Some(Stmt::If(cond, then_body, else_body))
            },
            Token::While => {
                self.advance(); // consume 'while'
                if *self.peek() == Token::LParen { self.advance(); }
                let cond = self.parse_expr()?;
                if *self.peek() == Token::RParen { self.advance(); }
                
                let body = self.parse_block();
                Some(Stmt::While(cond, body))
            },
            Token::Ident(_) => {
                // assume expression statement
                let expr = self.parse_expr()?;
                if *self.peek() == Token::Semi { self.advance(); }
                Some(Stmt::Expr(expr))
            },
            _ => None
        }
    }

    fn parse_block(&mut self) -> Vec<Stmt> {
        let mut body = Vec::new();
        if *self.peek() == Token::LBrace {
            self.advance();
            while *self.peek() != Token::RBrace && *self.peek() != Token::Eof {
                 if let Some(stmt) = self.parse_stmt() {
                     body.push(stmt);
                 } else {
                     self.advance();
                 }
            }
            if *self.peek() == Token::RBrace { self.advance(); }
        } else {
            // Single statement block
            if let Some(stmt) = self.parse_stmt() {
                body.push(stmt);
            }
        }
        body
    }

    fn parse_expr(&mut self) -> Option<Expr> {
        self.parse_logical_or()
    }

    fn parse_logical_or(&mut self) -> Option<Expr> {
        let mut left = self.parse_logical_and()?;
        while *self.peek() == Token::Or {
            self.advance();
            let right = self.parse_logical_and()?;
            left = Expr::BinOp(vec![left], BinOp::Or, vec![right]);
        }
        Some(left)
    }

    fn parse_logical_and(&mut self) -> Option<Expr> {
        let mut left = self.parse_equality()?;
        while *self.peek() == Token::And {
            self.advance();
            let right = self.parse_equality()?;
            left = Expr::BinOp(vec![left], BinOp::And, vec![right]);
        }
        Some(left)
    }

    fn parse_equality(&mut self) -> Option<Expr> {
        let mut left = self.parse_comparison()?;
        while matches!(self.peek(), Token::EqEq | Token::NotEq) {
            let op = match self.advance() {
                Token::EqEq => BinOp::Eq,
                Token::NotEq => BinOp::NotEq,
                _ => unreachable!(),
            };
            let right = self.parse_comparison()?;
            left = Expr::BinOp(vec![left], op, vec![right]);
        }
        Some(left)
    }

    fn parse_comparison(&mut self) -> Option<Expr> {
        let mut left = self.parse_term()?;
        while matches!(self.peek(), Token::Lt | Token::Gt) {
            let op = match self.advance() {
                Token::Lt => BinOp::Lt,
                Token::Gt => BinOp::Gt,
                _ => unreachable!(),
            };
            let right = self.parse_term()?;
            left = Expr::BinOp(vec![left], op, vec![right]);
        }
        Some(left)
    }

    fn parse_term(&mut self) -> Option<Expr> {
        let mut left = self.parse_factor()?;
        while matches!(self.peek(), Token::Plus | Token::Minus) {
            let op = match self.advance() {
                Token::Plus => BinOp::Add,
                Token::Minus => BinOp::Sub,
                _ => unreachable!(),
            };
            let right = self.parse_factor()?;
            left = Expr::BinOp(vec![left], op, vec![right]);
        }
        Some(left)
    }

    fn parse_factor(&mut self) -> Option<Expr> {
        let mut left = self.parse_unary()?;
        while matches!(self.peek(), Token::Star | Token::Slash) {
            let op = match self.advance() {
                Token::Star => BinOp::Mul,
                Token::Slash => BinOp::Div,
                _ => unreachable!(),
            };
            let right = self.parse_unary()?;
            left = Expr::BinOp(vec![left], op, vec![right]);
        }
        Some(left)
    }

    fn parse_unary(&mut self) -> Option<Expr> {
        if matches!(self.peek(), Token::Not | Token::Minus) {
            let tok = self.advance();
            let right = self.parse_unary()?;
            match tok {
                Token::Not => Some(Expr::UnaryOp(UnaryOp::Not, vec![right])),
                Token::Minus => Some(Expr::BinOp(vec![Expr::IntLit(0)], BinOp::Sub, vec![right])),
                _ => unreachable!(),
            }
        } else {
            self.parse_primary()
        }
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
                    while *self.peek() != Token::RParen && *self.peek() != Token::Eof {
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
            Token::LParen => {
                self.advance();
                let expr = self.parse_expr()?;
                if *self.peek() == Token::RParen { self.advance(); }
                Some(expr)
            }
            _ => None
        }
    }
}

// ===========================
// Runtime (Interpreter)
// ===========================

pub struct OmniRuntime {
    pub last_output: String,
    pub variables: BTreeMap<String, Value>,
}

impl OmniRuntime {
    pub fn new() -> Self {
        Self { 
            last_output: String::new(),
            variables: BTreeMap::new(),
        }
    }

    pub fn execute(&mut self, source: &str) {
        let mut lexer = Lexer::new(source);
        let tokens = lexer.tokenize();
        let mut parser = Parser::new(tokens);
        let module = parser.parse_module();

        for func in module.functions {
            if func.name == "main" {
                self.eval_body(&func.body);
            }
        }
    }

    fn eval_body(&mut self, body: &[Stmt]) {
        for stmt in body {
            match stmt {
                Stmt::Expr(expr) => { self.eval_expr(expr); },
                Stmt::Let(name, expr) => {
                    let val = self.eval_expr(expr);
                    self.variables.insert(name.clone(), val);
                },
                Stmt::While(cond, inner_body) => {
                    while self.eval_expr(cond).as_bool() {
                        self.eval_body(inner_body);
                    }
                },
                Stmt::If(cond, then_body, else_body) => {
                    if self.eval_expr(cond).as_bool() {
                        self.eval_body(then_body);
                    } else if let Some(eb) = else_body {
                        self.eval_body(eb);
                    }
                },
                Stmt::Return(expr) => {
                    self.eval_expr(expr);
                    return;
                },
                _ => {}
            }
        }
    }

    fn eval_expr(&mut self, expr: &Expr) -> Value {
        match expr {
            Expr::IntLit(n) => Value::Int(*n),
            Expr::StringLit(s) => Value::String(s.clone()),
            Expr::Ident(n) => {
                self.variables.get(n).cloned().unwrap_or(Value::Int(0))
            },
            Expr::Call(name, args) => {
                let eval_args: Vec<Value> = args.iter().map(|a| self.eval_expr(a)).collect();
                self.eval_builtin(name, &eval_args)
            },
            Expr::BinOp(left, op, right) => {
                let l = left.first().map(|e| self.eval_expr(e)).unwrap_or(Value::Int(0));
                let r = right.first().map(|e| self.eval_expr(e)).unwrap_or(Value::Int(0));
                match (l, r) {
                    (Value::Int(lv), Value::Int(rv)) => {
                        match op {
                            BinOp::Add => Value::Int(lv + rv),
                            BinOp::Sub => Value::Int(lv - rv),
                            BinOp::Mul => Value::Int(lv * rv),
                            BinOp::Div => Value::Int(if rv != 0 { lv / rv } else { 0 }),
                            BinOp::Eq => Value::Int(if lv == rv { 1 } else { 0 }),
                            BinOp::NotEq => Value::Int(if lv != rv { 1 } else { 0 }),
                            BinOp::Lt => Value::Int(if lv < rv { 1 } else { 0 }),
                            BinOp::Gt => Value::Int(if lv > rv { 1 } else { 0 }),
                            BinOp::And => Value::Int(if lv != 0 && rv != 0 { 1 } else { 0 }),
                            BinOp::Or => Value::Int(if lv != 0 || rv != 0 { 1 } else { 0 }),
                        }
                    },
                    (Value::String(ls), Value::String(rs)) => {
                        match op {
                            BinOp::Eq => Value::Int(if ls == rs { 1 } else { 0 }),
                            BinOp::NotEq => Value::Int(if ls != rs { 1 } else { 0 }),
                            BinOp::And => Value::Int(if !ls.is_empty() && !rs.is_empty() { 1 } else { 0 }),
                            BinOp::Or => Value::Int(if !ls.is_empty() || !rs.is_empty() { 1 } else { 0 }),
                            _ => Value::Int(0),
                        }
                    },
                    _ => Value::Int(0),
                }
            },
            Expr::UnaryOp(op, expr) => {
                let v = expr.first().map(|e| self.eval_expr(e)).unwrap_or(Value::Int(0));
                match op {
                    UnaryOp::Not => Value::Int(if !v.as_bool() { 1 } else { 0 }),
                }
            },
            Expr::If(cond, then_body, else_body) => {
                let c = cond.first().map(|e| self.eval_expr(e)).unwrap_or(Value::Int(0));
                if c.as_bool() {
                    self.eval_body(then_body);
                } else if let Some(eb) = else_body {
                    self.eval_body(eb);
                }
                Value::Int(0)
            },
        }
    }

    fn eval_builtin(&mut self, name: &str, args: &[Value]) -> Value {
        if name == "print" {
            for arg in args {
                let platform = crate::hal::get_platform();
                match arg {
                    Value::Int(n) => {
                        let s = format!("{}", n);
                        platform.puts(&s);
                    },
                    Value::String(s) => {
                        // Standardize terminal output for escapes
                        let processed = s.replace("\\n", "\n").replace("\\t", "\t");
                        platform.puts(&processed);
                    }
                }
            }
            Value::Int(0)
        } else if name == "System.shutdown" {
            crate::hal::get_platform().shutdown();
            Value::Int(0)
        } else if name == "System.input" {
            let mut input = String::new();
            let platform = crate::hal::get_platform();
            
            // Diamond Grade: Aggressive buffer clearing before starting input
            platform.clear();

            loop {
                // platform.get_char() blocks until data is available in UART
                let c = platform.get_char();
                
                // Noise filter (Null or UART Phantom bytes)
                if c == 0 || c == 0xFF { continue; } 
                
                if c == b'\r' || c == b'\n' {
                     platform.puts("\r\n");
                     
                     // Strict CRLF Sync: if \r, consume the potential following \n
                     if c == b'\r' {
                         for _ in 0..10000 {
                             if platform.has_data() {
                                 let next = platform.get_char();
                                 if next == b'\n' { /* consumed */ }
                                 break;
                             }
                             core::hint::spin_loop();
                         }
                     }
                     break;
                } else if c == 8 || c == 127 { // Backspace
                     if !input.is_empty() {
                         input.pop();
                         platform.puts("\x08 \x08"); 
                     }
                } else {
                     input.push(c as char);
                     platform.put_char(c);
                }
            }
            Value::String(input)
        } else {
            Value::Int(0)
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
