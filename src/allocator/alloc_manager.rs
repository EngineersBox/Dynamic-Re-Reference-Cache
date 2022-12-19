use cty::{c_void, size_t};

extern "C" {
    pub fn am_malloc(size: size_t) -> *c_void;
    pub fn am_calloc(count: size_t, size: size_t) -> *c_void;
    pub fn am_memalign(align: size_t, size: size_t) -> *c_void;
    pub fn am_realloc(ptr: *mut c_void, size: size_t) -> *c_void;
    pub fn am_free(ptr: *mut c_void);
}