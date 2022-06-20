use crate::error::BareError;
use crate::error::Span;
use sappho_ast::{
    ApplicationExpr, GenExpr, LetExpr, ListForm, Lookup, ProcEffects, PureEffects, QueryEffects,
    QueryExpr,
};

pub(crate) trait Restrict<S>: Sized {
    fn restrict(src: S, span: Span) -> Result<Self, BareError>;
}

impl Restrict<ProcEffects> for PureEffects {
    fn restrict(src: ProcEffects, span: Span) -> Result<Self, BareError> {
        use ProcEffects::*;

        Err(BareError::custom(
            span,
            format!(
                "pure expressions cannot contain {}",
                match src {
                    Inquire(_) => "inquiry effects, e.g. `$…`",
                    Evoke(_) => "evoke effects, e.g. `!…`",
                }
            ),
        ))
    }
}

impl Restrict<ProcEffects> for QueryEffects {
    fn restrict(src: ProcEffects, span: Span) -> Result<Self, BareError> {
        match src {
            ProcEffects::Inquire(x) => {
                Box::<QueryExpr>::restrict(x, span).map(QueryEffects::Inquire)
            }
            ProcEffects::Evoke(_) => Err(BareError::custom(
                span,
                "query expressions cannot contain evoke effects, e.g. `!…`".to_string(),
            )),
        }
    }
}

impl<FXS, FXD> Restrict<GenExpr<FXS>> for GenExpr<FXD>
where
    FXD: Restrict<FXS>,
{
    fn restrict(src: GenExpr<FXS>, span: Span) -> Result<Self, BareError> {
        use sappho_ast::Lookup as LookupExpr;
        use GenExpr::*;

        match src {
            Lit(x) => Ok(Lit(x)),
            Ref(x) => Ok(Ref(x)),
            Func(x) => Ok(Func(x)),
            Query(x) => Ok(Query(x)),
            Object(x) => Ok(Object(x)),
            List(x) => Ok(List(
                x.into_iter()
                    .map(|subx| GenExpr::<FXD>::restrict(subx, span.clone()))
                    .collect::<Result<ListForm<_>, BareError>>()?,
            )),
            Let(x) => LetExpr::restrict(x, span).map(Let),
            Apply(x) => ApplicationExpr::restrict(x, span).map(Apply),
            Lookup(x) => LookupExpr::restrict(x, span).map(Lookup),
            Effect(x) => FXD::restrict(x, span).map(Effect),
        }
    }
}

impl<FXS, FXD> Restrict<LetExpr<FXS>> for LetExpr<FXD>
where
    FXD: Restrict<FXS>,
{
    fn restrict(src: LetExpr<FXS>, span: Span) -> Result<Self, BareError> {
        Ok(LetExpr {
            binding: src.binding,
            bindexpr: Box::new(GenExpr::<FXD>::restrict(*src.bindexpr, span.clone())?),
            tail: Box::new(GenExpr::<FXD>::restrict(*src.tail, span)?),
        })
    }
}

impl<FXS, FXD> Restrict<ApplicationExpr<FXS>> for ApplicationExpr<FXD>
where
    FXD: Restrict<FXS>,
{
    fn restrict(src: ApplicationExpr<FXS>, span: Span) -> Result<Self, BareError> {
        Ok(ApplicationExpr {
            target: Box::new(GenExpr::<FXD>::restrict(*src.target, span.clone())?),
            argument: Box::new(GenExpr::<FXD>::restrict(*src.argument, span)?),
        })
    }
}

impl<FXS, FXD> Restrict<Lookup<FXS>> for Lookup<FXD>
where
    FXD: Restrict<FXS>,
{
    fn restrict(src: Lookup<FXS>, span: Span) -> Result<Self, BareError> {
        Ok(Lookup {
            target: Box::new(GenExpr::<FXD>::restrict(*src.target, span)?),
            attr: src.attr,
        })
    }
}

impl<S, D> Restrict<Box<S>> for Box<D>
where
    D: Restrict<S>,
{
    fn restrict(src: Box<S>, span: Span) -> Result<Self, BareError> {
        let d = D::restrict(*src, span)?;
        Ok(Box::new(d))
    }
}
