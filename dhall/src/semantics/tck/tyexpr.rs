#![allow(dead_code)]
use crate::error::{TypeError, TypeMessage};
use crate::semantics::core::context::TyCtx;
use crate::semantics::core::var::AlphaVar;
use crate::semantics::phase::typecheck::rc;
use crate::semantics::phase::Normalized;
use crate::semantics::phase::{NormalizedExpr, ToExprOptions};
use crate::semantics::Value;
use crate::syntax::{ExprKind, Label, Span, V};

pub(crate) type Type = Value;

// An expression with inferred types at every node and resolved variables.
#[derive(Debug, Clone)]
pub(crate) struct TyExpr {
    kind: Box<TyExprKind>,
    ty: Option<Type>,
    span: Span,
}

#[derive(Debug, Clone)]
pub(crate) enum TyExprKind {
    Var(AlphaVar),
    // Forbidden ExprKind variants: Var, Import, Embed
    Expr(ExprKind<TyExpr, Normalized>),
}

impl TyExpr {
    pub fn new(kind: TyExprKind, ty: Option<Type>, span: Span) -> Self {
        TyExpr {
            kind: Box::new(kind),
            ty,
            span,
        }
    }

    pub fn kind(&self) -> &TyExprKind {
        &*self.kind
    }
    pub fn get_type(&self) -> Result<Type, TypeError> {
        match &self.ty {
            Some(t) => Ok(t.clone()),
            None => Err(TypeError::new(&TyCtx::new(), TypeMessage::Sort)),
        }
    }

    /// Converts a value back to the corresponding AST expression.
    pub fn to_expr(&self, opts: ToExprOptions) -> NormalizedExpr {
        tyexpr_to_expr(self, opts, &mut Vec::new())
    }
    // TODO: temporary hack
    pub fn to_value(&self) -> Value {
        todo!()
    }
}

fn tyexpr_to_expr<'a>(
    tyexpr: &'a TyExpr,
    opts: ToExprOptions,
    ctx: &mut Vec<&'a Label>,
) -> NormalizedExpr {
    rc(match tyexpr.kind() {
        TyExprKind::Var(v) if opts.alpha => {
            ExprKind::Var(V("_".into(), v.idx()))
        }
        TyExprKind::Var(v) => {
            let name = ctx[ctx.len() - 1 - v.idx()];
            let idx = ctx
                .iter()
                .rev()
                .take(v.idx())
                .filter(|l| **l == name)
                .count();
            ExprKind::Var(V(name.clone(), idx))
        }
        TyExprKind::Expr(e) => {
            let e = e.map_ref_maybe_binder(|l, tye| {
                if let Some(l) = l {
                    ctx.push(l);
                }
                let e = tyexpr_to_expr(tye, opts, ctx);
                if let Some(_) = l {
                    ctx.pop();
                }
                e
            });

            match e {
                ExprKind::Lam(_, t, e) if opts.alpha => {
                    ExprKind::Lam("_".into(), t, e)
                }
                ExprKind::Pi(_, t, e) if opts.alpha => {
                    ExprKind::Pi("_".into(), t, e)
                }
                e => e,
            }
        }
    })
}
