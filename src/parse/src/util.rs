use std::cell::{RefCell, UnsafeCell};
use std::ops::Deref;
use std::sync::{Mutex, RwLock, RwLockReadGuard};
use lasso::{Rodeo, Spur};
use lazy_static::lazy_static;


pub struct SCache {
    pub cache: Mutex<Rodeo>,
    pub const_init: Spur,
    pub const_param: Spur,
    pub const_var: Spur,
    pub const_func: Spur,
    pub const_pre: Spur,
    pub const_post: Spur,
    pub const_final: Spur,
    pub const_validate: Spur,
    pub const_int: Spur,
    pub const_float: Spur,
    pub const_bool: Spur,
    pub const_string: Spur,
    pub const_nil: Spur,
}


impl Default for SCache {
    fn default() -> Self {
        let mut cache = Rodeo::new();
        let const_init = cache.get_or_intern_static("init");
        let const_param = cache.get_or_intern_static("param");
        let const_var = cache.get_or_intern_static("var");
        let const_func = cache.get_or_intern_static("func");
        let const_pre = cache.get_or_intern_static("pre");
        let const_post = cache.get_or_intern_static("post");
        let const_final = cache.get_or_intern_static("final");
        let const_validate = cache.get_or_intern_static("validate");
        let const_int = cache.get_or_intern_static("int");
        let const_float = cache.get_or_intern_static("float");
        let const_bool = cache.get_or_intern_static("bool");
        let const_string = cache.get_or_intern_static("string");
        let const_nil = cache.get_or_intern_static("nil");

        SCache {
            cache: Mutex::new(cache),
            const_init,
            const_param,
            const_var,
            const_func,
            const_pre,
            const_post,
            const_final,
            const_validate,
            const_int,
            const_float,
            const_bool,
            const_string,
            const_nil
            
        }
    }
}


impl SCache {
    pub fn intern(&self, s: &str) -> Spur {
        self.cache.lock().unwrap().get_or_intern(s)
    }

    // Unsafe: values are never dropped from the cache, due to the need for a 
    // rw lock the value is owned by the lock. As such this function transmutes
    // the pointer to static as to be able to return it. These reference are never 
    // stored externally and are just used for temporary access.
    pub fn resolve(&self, spur: &Spur) -> &str {
        let rodeo = self.cache.lock().unwrap();
        let string_ref: &str = rodeo.resolve(&spur);
        unsafe {
            std::mem::transmute::<&str, &'static str>(string_ref)
        }
    }
}

lazy_static! {pub static ref SCACHE : SCache = SCache::default();}



pub fn get_wide_bytes(val: u16) -> (u8, u8) {
    ((val & 0xFF) as u8, ((val >> 8) & 0xFF) as u8)
}