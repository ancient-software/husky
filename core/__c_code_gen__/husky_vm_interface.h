// this is generated by husky_vm_interface_code_gen::c_header::write_c_header
// do not modify by hand
        
#pragma once
#include <stdbool.h>
#include <stdint.h>

typedef struct unit {
} unit;
typedef union __RegisterData {
    unit as_void;
    bool as_bool;
    int32_t as_i32;
    int64_t as_i64;
    uint32_t as_b32;
    uint64_t as_b64;
    float as_f32;
    double as_f64;
    void *as_opt_ptr;
} __RegisterData;

typedef bool (*__primitive_value_to_bool_t)(__RegisterData);

typedef void *(*__primitive_value_to_box_t)(__RegisterData);

typedef void (*__drop_t)(void *);

typedef struct __RegisterVTable {
    char const *typename_str;
    __primitive_value_to_bool_t primitive_value_to_bool;
    __primitive_value_to_box_t primitive_value_to_box;
    __drop_t drop;
} __RegisterVTable;
    
// handles of primitive types are provided by Rust

// void
extern bool __void_primitive_value_to_bool(__RegisterData data);
extern void *__void_primitive_value_to_box(__RegisterData data);
extern void __void_drop(void*);
extern const __RegisterVTable __VOID_VTABLE;
        
// bool
extern bool __bool_primitive_value_to_bool(__RegisterData data);
extern void *__bool_primitive_value_to_box(__RegisterData data);
extern void __bool_drop(void*);
extern const __RegisterVTable __BOOL_VTABLE;
        
// i32
extern bool __i32_primitive_value_to_bool(__RegisterData data);
extern void *__i32_primitive_value_to_box(__RegisterData data);
extern void __i32_drop(void*);
extern const __RegisterVTable __I32_VTABLE;
        
// i64
extern bool __i64_primitive_value_to_bool(__RegisterData data);
extern void *__i64_primitive_value_to_box(__RegisterData data);
extern void __i64_drop(void*);
extern const __RegisterVTable __I64_VTABLE;
        
// b32
extern bool __b32_primitive_value_to_bool(__RegisterData data);
extern void *__b32_primitive_value_to_box(__RegisterData data);
extern void __b32_drop(void*);
extern const __RegisterVTable __B32_VTABLE;
        
// b64
extern bool __b64_primitive_value_to_bool(__RegisterData data);
extern void *__b64_primitive_value_to_box(__RegisterData data);
extern void __b64_drop(void*);
extern const __RegisterVTable __B64_VTABLE;
        
// f32
extern bool __f32_primitive_value_to_bool(__RegisterData data);
extern void *__f32_primitive_value_to_box(__RegisterData data);
extern void __f32_drop(void*);
extern const __RegisterVTable __F32_VTABLE;
        
// f64
extern bool __f64_primitive_value_to_bool(__RegisterData data);
extern void *__f64_primitive_value_to_box(__RegisterData data);
extern void __f64_drop(void*);
extern const __RegisterVTable __F64_VTABLE;
        