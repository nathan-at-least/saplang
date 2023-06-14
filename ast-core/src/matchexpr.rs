mod clause;

use sappho_unparse::{Stream, Unparse};

pub use self::clause::MatchClause;

/// A `match` expression, ie: `match x { 3 -> 0, y -> y }`.
#[derive(Clone, Debug, PartialEq, derive_more::Constructor)]
pub struct MatchExpr<Pattern, Expr> {
    /// The match target, ie: `x` in `match x { 3 -> 0, y -> y }`.
    pub target: Box<Expr>,

    /// The match clauses, ie: `3 -> 0` and `y -> y` in `match x { 3 -> 0, y -> y }`.
    pub clauses: Vec<MatchClause<Pattern, Expr>>,
}

impl<P, X> MatchExpr<P, X> {
    pub fn transform_into<PD, XD>(self) -> MatchExpr<PD, XD>
    where
        PD: From<P>,
        XD: From<X>,
    {
        MatchExpr {
            target: Box::new(XD::from(*self.target)),
            clauses: self
                .clauses
                .into_iter()
                .map(|c| c.transform_into())
                .collect(),
        }
    }
}

impl<P, X> Unparse for MatchExpr<P, X>
where
    P: Unparse,
    X: Unparse,
{
    fn unparse_into(&self, s: &mut Stream) {
        use sappho_unparse::{Brackets::Squiggle, Break::OptSpace};

        s.write("match ");
        s.write(&self.target);
        s.write(" ");
        s.bracketed(Squiggle, |subs| {
            for clause in &self.clauses {
                subs.write(&OptSpace);
                subs.write(clause);
                subs.write(",");
            }
        });
    }
}
