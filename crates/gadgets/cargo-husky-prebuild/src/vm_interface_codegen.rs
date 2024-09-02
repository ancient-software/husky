use crate::*;

pub(crate) fn write_vm_interface_codegen() {
    diff_write(
        &PathBuf::from("crates/vm/husky-vm-interface/src/__rust_code_gen__.rs"),
        &gen_rust_code().unwrap(),
        true,
    );
}

pub fn gen_rust_code() -> Result<String, std::fmt::Error> {
    let mut code = String::new();
    w!(code; BuildCodeGenStart);
    for ty in PRIMITIVE_TYPES {
        w!(code; PrimitiveTypeRegistration { ty })
    }
    for ty in NONPRIMITIVE_BUILTIN_TYPES {
        w!(code; NonPrimitiveTypeRegistration { ty })
    }
    for nargs in 0..10 {
        w!(code; ImplFp { nargs })
    }
    Ok(code)
}

pub static PRIMITIVE_TYPES: &'static [&'static str] =
    &["void", "bool", "i32", "i64", "r32", "b64", "f32", "f64"];

pub static NONPRIMITIVE_BUILTIN_TYPES: &'static [&'static str] =
    &["__VirtualFunction", "__VirtualEnum"];

pub struct ImplFp {
    nargs: usize,
}

impl std::fmt::Display for ImplFp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use itertools::Itertools;

        let arg_types_decl = (0..self.nargs)
            .into_iter()
            .map(|i| -> String { format!("A{i}: __ThawedInfo, ") })
            .join("");
        let static_arg_types_decl = (0..self.nargs)
            .into_iter()
            .map(|i| -> String { format!("A{i}: __ThawedInfo, ") })
            .join("");
        let arg_types = (0..self.nargs)
            .into_iter()
            .map(|i| -> String { format!(r#"A{i}"#) })
            .join(", ");
        let arg_types_with_comma = (0..self.nargs)
            .into_iter()
            .map(|i| -> String { format!(r#"A{i}, "#) })
            .join("");
        let static_arg_types_with_comma = (0..self.nargs)
            .into_iter()
            .map(|i| -> String {
                format!(
                    r#"
        <A{i} as __ThawedInfo>::__ThawedSelf, "#
                )
            })
            .join("");
        f.write_fmt(format_args!(
            r#"
// base

#[cfg(feature = "thin_fp")]
#[rustfmt::skip]
impl<{static_arg_types_decl}Output: __ThawedInfo> __ThawedInfo for fn({arg_types}
) -> Output {{
    type __ThawedSelf = fn({static_arg_types_with_comma}
    ) -> <Output as __ThawedInfo>::__ThawedSelf;

    fn __Thawed_typename() -> std::borrow::Cow<'static, str> {{
        todo!()
    }}

    unsafe fn __transmute_static(self) -> Self::__ThawedSelf {{
        todo!()
    }}
}}

#[cfg(feature = "thin_fp")]
#[rustfmt::skip]
impl<{arg_types_decl}Output: __ThawedInfo> const ThinFp
    for fn({arg_types}) -> Output {{
    fn __to_void_pointer(self) -> *const c_void {{
        self as *const c_void
    }}
}}

#[cfg(feature = "thin_fp")]
#[rustfmt::skip]
impl<{arg_types_decl}Output: __ThawedInfo> const __BaseThinFp
    for fn({arg_types}) -> Output {{
    type __CtxThinFp = fn(
        {static_arg_types_with_comma}&dyn __EvalContext
    ) -> Output;
}}

// ctx

#[cfg(feature = "thin_fp")]
#[rustfmt::skip]
impl<{static_arg_types_decl}Output: __ThawedInfo> __ThawedInfo
    for fn(
        {arg_types_with_comma}&dyn __EvalContext
    ) -> Output {{
    type __ThawedSelf = fn({static_arg_types_with_comma}
    ) -> <Output as __ThawedInfo>::__ThawedSelf;

    fn __Thawed_typename() -> std::borrow::Cow<'static, str> {{
        todo!()
    }}

    unsafe fn __transmute_static(self) -> Self::__ThawedSelf {{
        todo!()
    }}
}}

#[cfg(feature = "thin_fp")]
#[rustfmt::skip]
impl<{arg_types_decl}Output: __ThawedInfo> const ThinFp
    for fn(
        {arg_types_with_comma}&dyn __EvalContext
    ) -> Output {{
    fn __to_void_pointer(self) -> *const c_void {{
        self as *const c_void
    }}
}}

#[cfg(feature = "thin_fp")]
#[rustfmt::skip]
impl<{arg_types_decl}Output: __ThawedInfo> const __CtxThinFp
    for fn(
        {arg_types_with_comma}&dyn __EvalContext
    ) -> Output {{}}
"#,
        ))
    }
}
