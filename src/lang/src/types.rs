use std::collections::{BTreeSet, HashSet};
use lasso::Spur;


#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Type {
    Unresolved,
    Integer,
    Float,
    Boolean,
    Vector(Box<Type>),
    String,
    Pair,
    Nil,
    Quote,
    Object(ObjType),
    Lambda(FuncType),
}


#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ObjType {
    pub super_types: Vec<Type>,
    pub name: Spur,
}


#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FuncType {
    pub return_type: Box<Type>,
    pub arg_typ: Vec<Type>,
}