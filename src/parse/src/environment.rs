use ahash::{AHashMap, HashMap};
use intmap::IntMap;
use lasso::{Key, Spur};
use lang::types::{ObjType, Type};
use crate::util::SCACHE;


pub struct Binding {
    pub name: String,
    pub value: LiteralValue,
    pub mutable: bool,
    pub private: bool,
}


enum LiteralValue {
    Integer(i64),
    Float(f64),
    String(String),
    Nil,
    Quote,
    Object,
    Vector,
    Pair,
    Lambda,
}


pub struct SymbolCtx {
    pub scope: u32,
    pub depth: u32,
    pub typ: Type,
}


pub struct Context {
    curr_scope: u32,
    curr_depth: u32,
    active_scopes: Vec<u32>,
    symbols: AHashMap<u32, IntMap<SymbolCtx>>,
    globals: IntMap<SymbolCtx>,
    types: IntMap<Type>,
    unresolved: Type
}


impl Default for Context {
    fn default() -> Self {
        let mut types = IntMap::<Type>::with_capacity(50);
        types.insert(SCACHE.const_int.into_usize() as u64, Type::Integer);
        types.insert(SCACHE.const_float.into_usize() as u64, Type::Float);
        types.insert(SCACHE.const_bool.into_usize() as u64, Type::Boolean);
        types.insert(SCACHE.const_string.into_usize() as u64, Type::String);
        types.insert(SCACHE.const_nil.into_usize() as u64, Type::Nil);
        
        Context {
            curr_scope: 0,
            curr_depth: 0,
            active_scopes: Vec::<u32>::new(),
            symbols: AHashMap::<u32, IntMap<SymbolCtx>>::with_capacity(50),
            globals: IntMap::<SymbolCtx>::with_capacity(50),
            types,
            unresolved: Type::Unresolved
        }
    }
}


impl Context {
    pub fn add_symbol(&mut self, symbol: Spur, typ: Type) -> Result<(), String> {
        let s_int = symbol.into_usize() as u64;
        let data = SymbolCtx { scope: self.curr_scope, depth: self.curr_depth, typ };

        if self.curr_depth == 0 {
            self.globals.insert(s_int, data);
            return Ok(());
        }

        if let Some(existing) = self.symbols.get_mut(&self.curr_scope) {
            return match existing.insert_checked(s_int, data) {
                true => Ok(()),
                false => Err(format!("Redefinition of existing binding: {}", SCACHE.resolve(&symbol)))
            }
        }

        let mut scope_table = IntMap::<SymbolCtx>::new();
        scope_table.insert(s_int, data);
        self.symbols.insert(self.curr_scope, scope_table);
        Ok(())
    }

    pub fn get_symbol_type(&self, symbol: Spur) -> &Type {
        let s_int = symbol.into_usize() as u64;

        for &scope_id in self.active_scopes.iter().rev() {
            if let Some(scope_symbols) = self.symbols.get(&scope_id) {
                if let Some(symbol_ctx) = scope_symbols.get(s_int) {
                    return &symbol_ctx.typ;
                }
            }
        }
        
        if let Some(global_symbol) = self.globals.get(s_int) {
            return &global_symbol.typ;
        }
        
        &self.unresolved
    }

    pub fn validate_type(&self, spur: Spur) -> Type {
        let found = self.types.get(spur.into_usize() as u64);
        if found.is_none() { return Type::Unresolved; }

        match found.unwrap() {
            Type::Unresolved | Type::Integer | Type::Float | Type::String | Type::Boolean |
            Type::Vector(_) | Type::Nil | Type::Pair => found.unwrap().clone(),
            Type::Quote => todo!(),
            Type::Object(obj) => {
                for typ in &obj.super_types {
                    if let Type::Object(obj_type) = typ {
                        if obj_type.name == spur { return typ.clone(); }
                    }
                } // Object will always have a spur value
                Type::Unresolved
            }
            Type::Lambda(_) => todo!(),
        }
    }

    pub fn add_type(&mut self, spur: Spur, typ: Type) {
        self.types.insert_checked(spur.into_usize() as u64, typ);
    }

    pub fn pushScope(&mut self) {
        self.curr_scope += 1;
        self.curr_depth += 1;
        self.active_scopes.push(self.curr_scope)
    }


    pub fn popScope(&mut self) {
        self.curr_depth -= 1;
        self.active_scopes.pop().expect("Fatal: Popped global scope");
    }

    pub fn get_scope_tup(&self) -> (u32, u32) {
        (self.curr_scope, self.curr_depth)
    }
}
