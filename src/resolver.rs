use std::collections::HashMap;
use crate::{ast::{Expr, Stmt}, interpreter::Interpreter, position::{Diagnostic, Span}, program::Program};




/// Resolves all variables in a single pass
pub struct Resolver<'i> {
    interpreter: &'i mut Interpreter,
    scopes: Vec<HashMap<String, bool>>,
    diagnostics: Vec<Diagnostic>,
}


impl<'i> Resolver<'i> {
    pub fn new(interpreter: &'i mut Interpreter) -> Self {
        Self {
            interpreter: interpreter,
            scopes: Vec::new(),
            diagnostics: Vec::new()
        }
    }

    pub fn run(&mut self, program: &Program) -> Result<(), Vec<Diagnostic>> {
        self.resolve_stmts(&program.statements);
        if self.diagnostics.len() > 0 {
            return Err(self.diagnostics.clone())
        } else {
            return Ok(())
        }
    }


    fn resolve_stmts(&mut self, stmts: &Vec<Stmt>) {
        for stmt in stmts {
            self.resolve_stmt(stmt);
        }
    }

    fn resolve_stmt(&mut self, stmt: &Stmt) {
        match stmt {
            Stmt::Block(stmts) => {
                self.scoped(|this| {
                    this.resolve_stmts(stmts);
                });
            }
            Stmt::Var(id, expr) => {
                self.declare(id);
                self.resolve_expr(expr);
                self.define(id);
            }
            Stmt::Function(name, vars, stmts) => {
                self.declare(name); //TODO: remove this line
                self.define(name);
                self.scoped(|this| {
                    for var in vars {
                        this.declare(var); //TODO: remove this line
                        this.define(var);
                    }
                    this.resolve_stmt(stmts);
                });
            }
            Stmt::Expression(expr) => self.resolve_expr(expr),
            Stmt::If(cond, first, second) => {
                self.resolve_expr(cond);
                self.resolve_stmt(first);
                if let Some(stmt) = second {
                    self.resolve_stmt(stmt);
                }
            }
            Stmt::Print(expr) => self.resolve_expr(expr),
            Stmt::Return(expr) => self.resolve_expr(expr),
            Stmt::While(cond, body) => {
                self.resolve_expr(cond);
                self.resolve_stmt(body);
            }
        }
    }

    fn resolve_expr(&mut self, expr: &Expr) {
        match expr {
            Expr::Call(expr, exprs) => {
                self.resolve_expr(expr);
                for expr in exprs {
                    self.resolve_expr(expr);
                }
            },
            Expr::Variable(id) => {
                if let Some(scope) = self.scopes.last_mut() {
                    if scope.get(id) == Some(&false) {
                        self.diagnostics.push(Diagnostic {
                            span: Span::empty(),
                            message: format!("Can't read local variable '{}' in its own initializer.", id)
                        });
                    }
                }
                self.resolve_local(id);
            }
            Expr::Assignment(id, inner_expr) => {
                self.resolve_expr(inner_expr);
                self.resolve_local(id);
            }
            Expr::LogicalOr(left, right) => {
                self.resolve_expr(left);
                self.resolve_expr(right);
            }
            Expr::LogicalAnd(left, right) => {
                self.resolve_expr(left);
                self.resolve_expr(right);
            }
            Expr::Grouping(expr) => self.resolve_expr(expr),
            Expr::Binary(expr1, _, expr2) => {
                self.resolve_expr(expr1);
                self.resolve_expr(expr2);
            }
            Expr::Unary(_, expr) => self.resolve_expr(expr),
            _ => {}
        }
    }


    fn resolve_local(&mut self, id: &str) {
        let len = self.scopes.len();
        for depth in 0..len {
            let i = len - depth - 1;
            let scope = &self.scopes[i];
            if scope.contains_key(id) {
                self.interpreter.resolve_local(id, depth);
                return
            }
        }
    }

    fn scoped<I>(&mut self, inner: I)
    where
        I: FnOnce(&mut Self),
    {
        self.begin_scope();
        let res = inner(self);
        self.end_scope();
        res
    }

    /// One should ideally use `scoped`. Callers of `begin_scope` must also call `end_scope`.
    #[inline]
    fn begin_scope(&mut self) {
        self.scopes.push(HashMap::new());
    }

    #[inline]
    fn end_scope(&mut self) {
        self.scopes.pop();
    }

    fn declare(&mut self, id: &str) {
        if let Some(scope) = self.scopes.last_mut() {
            scope.insert(id.to_string(), false);
        }
    }

    fn define(&mut self, id: &str) {
        if let Some(scope) = self.scopes.last_mut() {
            scope.insert(id.to_string(), true);
        }
    }


}