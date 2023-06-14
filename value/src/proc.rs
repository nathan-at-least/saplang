use crate::{GenThunk, ScopeRef};
use sappho_ast_core::{EffectExpr, ProcEffects};
use sappho_ast_reduced::{ObjectDef, ProcClause, ProcExpr};
use sappho_unparse::{Stream, Unparse};

#[derive(Debug, derive_more::Constructor)]
pub struct Proc {
    pdef: ProcClause,
    defscope: ScopeRef,
}

impl Proc {
    pub fn as_thunk(&self) -> GenThunk<ProcEffects> {
        // FIXME: This is ugly: GenThunk requires an `Expr` so we synthsize `!proc { ... }` around
        // the proc definition.
        GenThunk::new(
            ProcExpr::new(EffectExpr::new(
                ProcEffects::Invoke,
                Box::new(ProcExpr::new(ObjectDef::new_proc(self.pdef.clone()))),
            )),
            self.defscope.clone(),
        )
    }
}

impl Unparse for Proc {
    fn unparse_into(&self, s: &mut Stream) {
        s.write(&self.pdef);
    }
}
