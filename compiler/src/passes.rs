//! Compiler Passes - Memory Annotation & Task Partitioning

use crate::parser::{AST, App, Function, Expr, Statement};

/// Pass 1: Automatic Memory Annotation
pub fn annotate_memory(ast: &mut AST) {
    for app in &mut ast.apps {
        for member in &mut app.members {
            if let crate::parser::AppMember::Function(func) = member {
                analyze_function_memory(func);
            }
        }
    }
}

fn analyze_function_memory(func: &mut Function) {
    // Estimate memory usage
    let estimated_size = estimate_memory_usage(&func.body);
    
    // Add annotation if not present and size > threshold
    if estimated_size > 512 * 1024 && !has_memory_annotation(func) {
        func.annotations.push(crate::parser::Annotation {
            name: "memory".to_string(),
            args: vec![
                ("budget".to_string(), format!("{}.kb", estimated_size / 1024)),
                ("distributed".to_string(), "true".to_string()),
            ],
        });
    }
}

fn estimate_memory_usage(block: &crate::parser::Block) -> usize {
    // Simple heuristic: count allocations
    let mut size = 0;
    for stmt in &block.statements {
        match stmt {
            Statement::Let { value, .. } => {
                size += estimate_expr_size(value);
            }
            _ => {}
        }
    }
    size
}

fn estimate_expr_size(expr: &Expr) -> usize {
    match expr {
        Expr::String(s) => s.len(),
        Expr::Call { .. } => 1024, // Assume 1KB per call
        _ => 8, // Primitive types
    }
}

fn has_memory_annotation(func: &Function) -> bool {
    func.annotations.iter().any(|a| a.name == "memory")
}

/// Pass 2: Task Partitioning (DAG-based)
pub fn partition_tasks(ast: &mut AST) {
    for app in &mut ast.apps {
        for member in &mut app.members {
            if let crate::parser::AppMember::Function(func) = member {
                if func.is_distributed || has_distributed_annotation(func) {
                    create_partition_plan(func);
                }
            }
        }
    }
}

fn has_distributed_annotation(func: &Function) -> bool {
    func.annotations.iter().any(|a| a.name == "distributed")
}

fn create_partition_plan(func: &mut Function) {
    // Analyze function body for parallelizable sections
    // In v1.0: Build DAG and identify critical path
    
    // For now: Mark function as distributable
    if !func.is_distributed {
        func.is_distributed = true;
    }
}

/// Pass 3: Optimization
pub fn optimize(ast: &mut AST) {
    for app in &mut ast.apps {
        for member in &mut app.members {
            if let crate::parser::AppMember::Function(func) = member {
                optimize_function(func);
            }
        }
    }
}

fn optimize_function(_func: &mut Function) {
    // TODO: Implement optimizations
    // - Constant folding
    // - Dead code elimination
    // - Loop unrolling
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::*;

    #[test]
    fn test_memory_annotation() {
        let mut ast = AST {
            apps: vec![App {
                name: "Test".to_string(),
                annotations: vec![],
                members: vec![AppMember::Function(Function {
                    name: "test".to_string(),
                    annotations: vec![],
                    is_distributed: false,
                    params: vec![],
                    return_type: None,
                    body: Block {
                        statements: vec![
                            Statement::Let {
                                name: "data".to_string(),
                                value: Expr::String("x".repeat(1000000)),
                            }
                        ],
                    },
                })],
            }],
        };
        
        annotate_memory(&mut ast);
        
        if let AppMember::Function(func) = &ast.apps[0].members[0] {
            assert!(func.annotations.iter().any(|a| a.name == "memory"));
        }
    }
}
