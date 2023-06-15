use sappho_unparse::{Stream, Unparse};

#[derive(Clone, Debug, PartialEq, derive_more::Constructor)]
pub struct EffectExpr<FX, Expr> {
    pub effect: FX,
    pub expr: Box<Expr>,
}

impl<FX, X> EffectExpr<FX, X> {
    pub fn transform_into<Y>(self) -> EffectExpr<FX, Y>
    where
        Y: From<X>,
    {
        EffectExpr {
            effect: self.effect,
            expr: Box::new(Y::from(*self.expr)),
        }
    }
}

impl<FX, X> Unparse for EffectExpr<FX, X>
where
    FX: Unparse,
    X: Unparse,
{
    fn unparse_into(&self, s: &mut Stream) {
        s.write(&self.effect);
        s.write(&self.expr);
    }
}
