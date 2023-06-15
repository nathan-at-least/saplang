mod clause;

use sappho_unparse::{Stream, Unparse};

pub use self::clause::LetClause;

/// A `let` expression for local definitions, ie: `let x = 42; f x`.
#[derive(Clone, Debug, PartialEq, derive_more::Constructor)]
pub struct LetExpr<Pattern, Expr> {
    /// The let clauses:
    pub clauses: Vec<LetClause<Pattern, Expr>>,

    /// The expression to evaluate with the binding in-scope, ie: `f x` in `let x = 42; f x`.
    pub tail: Box<Expr>,
}

impl<P, X> LetExpr<P, X> {
    pub fn transform_into<PD, XD>(self) -> LetExpr<PD, XD>
    where
        PD: From<P>,
        XD: From<X>,
    {
        LetExpr {
            clauses: self
                .clauses
                .into_iter()
                .map(|c| c.transform_into())
                .collect(),
            tail: Box::new(XD::from(*self.tail)),
        }
    }
}

impl<P, X> Unparse for LetExpr<P, X>
where
    P: Unparse,
    X: Unparse,
{
    fn unparse_into(&self, s: &mut Stream) {
        use sappho_unparse::{Brackets::Parens, Break::Mandatory};

        let unparse_clauses = |s: &mut Stream| {
            for (ix, clause) in self.clauses.iter().enumerate() {
                if s.depth() > 0 || ix > 0 {
                    s.write(&Mandatory);
                }
                s.write(clause);
                s.write(";");
            }
            s.write(&Mandatory);
            s.write(&self.tail);
        };

        if s.depth() == 0 {
            unparse_clauses(s);
        } else {
            s.bracketed(Parens, unparse_clauses);
        }
    }
}
