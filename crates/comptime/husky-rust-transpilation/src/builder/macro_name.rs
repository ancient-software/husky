use super::*;

pub enum RustMacroName {
    Vec,
    Println,
    Todo,
    Unreachable,
    Panic,
    Require,
    Assert,
    Matches,
    Unveil,
    LinkageImpls,
    FnLinkageImpl,
    DestructorFnLinkageImpl,
    UnveilFnLinkageImpl,
    GnLinkageImpl,
    StructFieldLinkageImpl,
    TypeDefault,
    EnumU8Presenter,
    HtmlTag(husky_coword::Ident),
}

impl RustMacroName {}

impl<'a, 'b, E> RustTranspilationBuilder<'a, 'b, E> {
    pub(crate) fn macro_name(&mut self, macro_name: RustMacroName) {
        let db = self.db;
        if self.result.ends_with(|c: char| c.is_alphabetic()) {
            self.write_str(" ")
        }
        self.write_str(match macro_name {
            RustMacroName::Vec => "vec!",
            RustMacroName::Println => "println!",
            RustMacroName::Todo => "todo!",
            RustMacroName::Unreachable => "unreachable!",
            RustMacroName::Panic => "panic!",
            RustMacroName::Require => "require!",
            RustMacroName::Assert => "assert!",
            RustMacroName::Matches => "matches!",
            RustMacroName::Unveil => "unveil!",
            RustMacroName::LinkageImpls => "linkage_impls!",
            RustMacroName::FnLinkageImpl => "fn_linkage_impl!",
            RustMacroName::DestructorFnLinkageImpl => "destructor_fn_linkage_impl!",
            RustMacroName::UnveilFnLinkageImpl => "unveil_fn_linkage_impl!",
            RustMacroName::GnLinkageImpl => "gn_linkage_impl!",
            RustMacroName::StructFieldLinkageImpl => "struct_field_linkage_impl!",
            RustMacroName::TypeDefault => "ty_default_linkage_impl!",
            RustMacroName::EnumU8Presenter => "enum_u8_presenter_linkage_impl!",
            RustMacroName::HtmlTag(ident) => {
                self.result += ident.data(db);
                self.result += "!";
                return;
            }
        })
    }

    pub(crate) fn macro_call(&mut self, macro_name: RustMacroName, f: impl FnOnce(&mut Self)) {
        self.macro_name(macro_name);
        self.bracketed(RustDelimiter::Par, f)
    }
}
