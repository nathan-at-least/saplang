use crate::{AstFxFor, FromFx, GenExpr, Identifier};
use sappho_ast as ast;
use std::fmt;

#[derive(Debug, PartialEq)]
pub struct Lookup<Effects> {
    pub target: Box<GenExpr<Effects>>,
    pub field: Identifier,
}

impl<FX> From<ast::Lookup<AstFxFor<FX>>> for Lookup<FX>
where
    FX: FromFx,
{
    fn from(lookup: ast::Lookup<AstFxFor<FX>>) -> Self {
        let ast::Lookup { target, field } = lookup;

        Lookup {
            target: Box::new(GenExpr::from(*target)),
            field: field,
        }
    }
}

impl<FX> fmt::Display for Lookup<FX>
where
    FX: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "(")?;
        self.target.fmt(f)?;
        write!(f, ").{}", self.field)?;
        Ok(())
    }
}
