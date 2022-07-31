mod rust_code;

use husky_c_code_build::build_single_file_to_lib;
use husky_c_code_repr::*;
use rust_code::write_rust_code;
use std::io::prelude::*;
use std::process::Command;
use std::{fs::File, path::Path};

pub static PRIMITIVE_TYPES: &'static [&'static str] =
    &["void", "bool", "i32", "i64", "b32", "b64", "f32", "f64"];

pub static NONPRIMITIVE_BUILTIN_TYPES: &'static [&'static str] = &["__VirtualFunction"];

pub fn gen_vm_interface_code(c_code_gen_dir: &str) {
    let c_header_path = format!("{}/husky_vm_interface.h", c_code_gen_dir);
    let c_source_path = format!("{}/husky_vm_interface.c", c_code_gen_dir);
    // let husky_dir = std::env::var("HUSKY_DIR").expect("HUSKY_DIR is not set");
    let husky_dir = "/home/xiyuzhai/Documents/husky";
    let rust_path = format!(
        "{}/core/crates/vm/husky-vm-interface/src/__rust_code_gen__.rs",
        husky_dir
    );
    write_c_header(&c_header_path).unwrap();
    write_c_source(&c_source_path).unwrap();
    write_rust_code(&rust_path).unwrap();
    build_single_file_to_lib(&c_code_gen_dir, "husky_vm_interface");
}

pub fn write_c_header(c_header_path: &str) -> std::io::Result<()> {
    eprintln!("c_header_path: {}", c_header_path);
    let mut buffer = File::create(c_header_path).unwrap();
    write!(
        buffer,
        r#"// this is generated by husky_vm_interface_code_gen::c_header::write_c_header
// do not modify by hand
        
#pragma once
#include <stdbool.h>
#include <stdint.h>

typedef struct unit {{
}} unit;
typedef union __RegisterData {{
    unit as_void;
    bool as_bool;
    int32_t as_i32;
    int64_t as_i64;
    uint32_t as_b32;
    uint64_t as_b64;
    float as_f32;
    double as_f64;
    void *as_opt_ptr;
}} __RegisterData;

typedef struct __Register __Register;

typedef bool (*__primitive_value_to_bool_t)(__RegisterData);
typedef void *(*__primitive_value_to_box_t)(__RegisterData);
typedef void *(*__clone_t)(void const*);
typedef void (*__drop_t)(void const*);
typedef bool (*__eq_t)(void const*, void const*);
typedef void (*__assign_t)(__Register *);

typedef struct __RegisterVTable {{
    char const *typename_str;
    __primitive_value_to_bool_t primitive_value_to_bool;
    __primitive_value_to_box_t primitive_value_to_box;
    __clone_t clone;
    __drop_t drop;
    __eq_t eq;
    __assign_t assign;
}} __RegisterVTable;
    
// handles of primitive types are provided by Rust
"#
    )?;
    for ty in PRIMITIVE_TYPES {
        write!(buffer, "{}", CPrimitiveTypeRegistrationHeader { ty })?
    }
    for ty in NONPRIMITIVE_BUILTIN_TYPES {
        write!(buffer, "{}", CNonPrimitiveTypeRegistrationHeader { ty })?
    }
    Ok(())
}

pub fn write_c_source(c_source_path: &str) -> std::io::Result<()> {
    use std::fmt::Display;

    let mut buffer = File::create(c_source_path).unwrap();
    write!(
        buffer,
        r#"#include "husky_vm_interface.h"
"#
    );

    for ty in PRIMITIVE_TYPES {
        write!(buffer, "{}", CPrimitiveTypeRegistrationSource { ty })?
    }

    for ty in NONPRIMITIVE_BUILTIN_TYPES {
        write!(buffer, "{}", CNonPrimitiveTypeRegistrationSource { ty })?
    }
    Ok(())
}

#[test]
fn try_gen_vm_interface_csrc() {
    std::env::set_var("RUST_BACKTRACE", "0");
    // this only works on my comuter
    std::env::set_var("HUSKY_DIR", "/home/xiyuzhai/Documents/husky");
    gen_vm_interface_code("/home/xiyuzhai/Documents/husky/core/__c_code_gen_test__")
}
