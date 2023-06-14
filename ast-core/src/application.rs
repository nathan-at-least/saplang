use sappho_unparse::{Stream, Unparse};

/// Function application, ie `f x`.
#[derive(Clone, Debug, PartialEq, derive_more::Constructor)]
pub struct ApplicationExpr<Expr> {
    /// The target of application, ie `f` in `f x`.
    pub target: Box<Expr>,

    /// The argument of application, ie `x` in `f x`.
    pub argument: Box<Expr>,
}

impl<X> ApplicationExpr<X> {
    pub fn transform_into<Y>(self) -> ApplicationExpr<Y>
    where
        Y: From<X>,
    {
        ApplicationExpr {
            target: Box::new(Y::from(*self.target)),
            argument: Box::new(Y::from(*self.argument)),
        }
    }
}

impl<Expr> Unparse for ApplicationExpr<Expr>
where
    Expr: Unparse,
{
    fn unparse_into(&self, s: &mut Stream) {
        use sappho_unparse::{
            Brackets::Parens,
            Break::{Opt, OptSpace},
        };

        s.bracketed(Parens, |subs| {
            subs.write(&Opt);
            subs.write(&self.target);
            subs.write(&OptSpace);
            subs.write(&self.argument);
        });
    }
}
