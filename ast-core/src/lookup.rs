use crate::Identifier;
use sappho_unparse::{Stream, Unparse};

/// An attribute lookup expression, ie: `x.foo`.
#[derive(Clone, Debug, PartialEq, derive_more::Constructor)]
pub struct LookupExpr<Expr> {
    /// The target expression of the lookup, ie `x` in `x.foo`.
    pub target: Box<Expr>,

    /// An attribute name, ie: `foo` in `x.foo`.
    pub attr: Identifier,
}

impl<X> LookupExpr<X> {
    pub fn transform_into<Y>(self) -> LookupExpr<Y>
    where
        Y: From<X>,
    {
        LookupExpr {
            target: Box::new(Y::from(*self.target)),
            attr: self.attr,
        }
    }
}

impl<X> Unparse for LookupExpr<X>
where
    X: Unparse,
{
    fn unparse_into(&self, s: &mut Stream) {
        s.write(&self.target);
        s.write(".");
        s.write(&self.attr);
    }
}
