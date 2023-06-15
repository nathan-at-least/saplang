use sappho_unparse::{Stream, Unparse};

/// A query definition, ie `query $x`.
#[derive(Clone, Debug, PartialEq, derive_more::Constructor)]
pub struct QueryDef<QueryExpr> {
    /// The `QueryExpr` definition, ie the `$x` in `query $x`.
    pub body: Box<QueryExpr>,
}

impl<X> QueryDef<X> {
    pub fn transform_into<Y>(self) -> QueryDef<Y>
    where
        Y: From<X>,
    {
        QueryDef {
            body: Box::new(Y::from(*self.body)),
        }
    }
}

impl<X> Unparse for QueryDef<X>
where
    X: Unparse,
{
    fn unparse_into(&self, s: &mut Stream) {
        s.write("query ");
        s.write(&self.body);
    }
}
