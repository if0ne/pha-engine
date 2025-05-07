#[derive(Clone, Copy, Debug)]
pub enum Ty {
    Int,
    String,
    Float,
}

#[derive(Debug)]
pub struct MemberField {
    pub name: &'static str,
    pub ty: Ty,
    pub offset: usize,
}

impl MemberField {
    pub const fn new(name: &'static str, ty: Ty, offset: usize) -> Self {
        Self { name, ty, offset }
    }
}

#[derive(Debug)]
pub struct UserDefinedType {
    pub fields: &'static [MemberField],
}

impl UserDefinedType {
    pub const fn new(fields: &'static [MemberField]) -> Self {
        Self { fields }
    }
}

pub trait Reflect {
    fn reflect(&self) -> &'static UserDefinedType;
}
