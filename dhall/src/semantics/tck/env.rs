use crate::semantics::{AlphaVar, NameEnv, Nir, NzEnv, NzVar, Type, ValEnv};
use crate::syntax::Label;

/// Environment for indexing variables.
#[derive(Debug, Clone, Copy, Default)]
pub struct VarEnv {
    size: usize,
}

/// Environment for typing expressions.
#[derive(Debug, Clone)]
pub struct TyEnv {
    names: NameEnv,
    items: ValEnv<Type>,
}

impl VarEnv {
    pub fn new() -> Self {
        VarEnv::default()
    }
    pub fn from_size(size: usize) -> Self {
        VarEnv { size }
    }
    pub fn size(self) -> usize {
        self.size
    }
    pub fn insert(self) -> Self {
        VarEnv {
            size: self.size + 1,
        }
    }
    pub fn lookup(self, var: NzVar) -> AlphaVar {
        self.lookup_fallible(var).unwrap()
    }
    pub fn lookup_fallible(self, var: NzVar) -> Option<AlphaVar> {
        let idx = self.size.checked_sub(var.idx() + 1)?;
        Some(AlphaVar::new(idx))
    }
}

impl TyEnv {
    pub fn new() -> Self {
        TyEnv {
            names: NameEnv::new(),
            items: ValEnv::new(),
        }
    }
    pub fn as_varenv(&self) -> VarEnv {
        self.names.as_varenv()
    }
    pub fn to_nzenv(&self) -> NzEnv {
        self.items.discard_types()
    }
    pub fn as_nameenv(&self) -> &NameEnv {
        &self.names
    }

    pub fn insert_type(&self, x: &Label, ty: Type) -> Self {
        TyEnv {
            names: self.names.insert(x),
            items: self.items.insert_type(ty),
        }
    }
    pub fn insert_value(&self, x: &Label, e: Nir, ty: Type) -> Self {
        TyEnv {
            names: self.names.insert(x),
            items: self.items.insert_value(e, ty),
        }
    }
    pub fn lookup(&self, var: AlphaVar) -> Type {
        self.items.lookup_ty(var)
    }
}

impl<'a> From<&'a TyEnv> for NzEnv {
    fn from(x: &'a TyEnv) -> Self {
        x.to_nzenv()
    }
}
