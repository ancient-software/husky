use super::PRIMITIVE_TYPES;
use crate::NONPRIMITIVE_BUILTIN_TYPES;
use husky_print_utils::p;
use husky_rust_code_repr::{NonPrimitiveTypeRegistration, PrimitiveTypeRegistration};
use std;
use std::fs::File;
use std::io::prelude::*;

pub(crate) fn write_rust_code(rust_path: &str) -> std::io::Result<()> {
    let mut buffer =
        File::create(rust_path).expect(&format!("rust path {rust_path} doesn't exist"));
    write!(
        buffer,
        r#"// this is generated by husky_vm_interface_code_gen::rust_code::write_rust_code
// do not modify by hand

use crate::*;

type void = ();
type b32 = u32;
type b64 = u64;
"#
    );

    for ty in PRIMITIVE_TYPES {
        write!(buffer, "{}", PrimitiveTypeRegistration { ty })?
    }

    for ty in NONPRIMITIVE_BUILTIN_TYPES {
        write!(buffer, "{}", NonPrimitiveTypeRegistration { ty })?
    }
    Ok(())
}
