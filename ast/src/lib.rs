//! This Abstract Syntax Tree corresponds to the textual grammar of `sappho`. Some of the grammar
//! is short-hand convenience for a simpler grammar used in evaluation, which is embodied in the
//! `sappho-east` (aka "eval ast") crate. For example:
//!
//! `fn x -> x` is AST short-hand for EAST `{ fn x -> x }`.
//!
//! The top-level expression for evaluation is [PureExpr], which is a type alias to a general
//! expression type over all effects, [GenExpr]. The three bespoke effects are [PureEffects],
//! [QueryEffects], and [ProcEffects].

mod effects;
mod expr;
mod func;
mod matchexpr;
mod object;
mod query;

pub use sappho_gast::{Identifier, Literal};
pub type ApplicationExpr<FX> = sappho_gast::ApplicationExpr<GenExpr<FX>>;
pub type LetExpr<FX> = sappho_gast::LetExpr<GenExpr<FX>>;
pub type LetClause<FX> = sappho_gast::LetClause<GenExpr<FX>>;
pub type LookupExpr<FX> = sappho_gast::LookupExpr<GenExpr<FX>>;

pub use self::effects::{ProcEffects, ProcExpr, PureEffects, PureExpr, QueryEffects, QueryExpr};
pub use self::expr::GenExpr;
pub use self::func::FuncDef;
pub use self::matchexpr::{MatchClause, MatchExpr};
pub use self::object::ObjectDef;
pub use self::query::QueryDef;
