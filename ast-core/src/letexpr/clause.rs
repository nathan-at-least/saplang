use sappho_unparse::{Stream, Unparse};

#[derive(Clone, Debug, PartialEq, derive_more::Constructor)]
pub struct LetClause<Pattern, Expr> {
    /// The binding pattern, ie: the first `x` in `let x = 42; f x`.
    pub binding: Pattern,

    /// The expression to bind, ie: `42` in `let x = 42; f x`.
    pub bindexpr: Box<Expr>,
}

impl<P, X> LetClause<P, X> {
    pub fn transform_into<PD, XD>(self) -> LetClause<PD, XD>
    where
        PD: From<P>,
        XD: From<X>,
    {
        LetClause {
            binding: PD::from(self.binding),
            bindexpr: Box::new(XD::from(*self.bindexpr)),
        }
    }
}

impl<P, X> Unparse for LetClause<P, X>
where
    P: Unparse,
    X: Unparse,
{
    fn unparse_into(&self, s: &mut Stream) {
        s.write("let ");
        s.write(&self.binding);
        s.write(" = ");
        s.write(&self.bindexpr);
    }
}
