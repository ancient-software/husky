use super::*;

#[derive(Debug, PartialEq, Eq)]
pub struct VfsPathMenu {
    core_package: PackagePath,
    std_package: PackagePath,
    core_library: CratePath,
    std_library: CratePath,
    /// core
    core_root: ModulePath,
    std_root: ModulePath,
    core_array: SubmodulePath,
    core_basic: SubmodulePath,
    core_backend: SubmodulePath,
    core_clone: SubmodulePath,
    core_cmp: SubmodulePath,
    core_default: SubmodulePath,
    core_frontend: SubmodulePath,
    core_fmt: SubmodulePath,
    core_vec: SubmodulePath,
    core_marker: SubmodulePath,
    core_mem: SubmodulePath,
    /// core::num
    core_num: SubmodulePath,
    core_slice: SubmodulePath,
    core_str: SubmodulePath,
    core_ops: SubmodulePath,
    core_option: SubmodulePath,
    core_prelude: SubmodulePath,
    core_raw_bits: SubmodulePath,
    core_result: SubmodulePath,
    core_visual: SubmodulePath,
}

#[salsa::tracked(return_ref)]
pub(crate) fn vfs_path_menu(db: &::salsa::Db, toolchain: Toolchain) -> VfsPathMenu {
    VfsPathMenu::new(db, toolchain)
}

impl VfsPathMenu {
    fn new(db: &::salsa::Db, toolchain: Toolchain) -> Self {
        let coword_menu = coword_menu(db);
        let core_package =
            PackagePath::new_toolchain_package(db, toolchain, coword_menu.core_name());
        let std_package = PackagePath::new_toolchain_package(db, toolchain, coword_menu.std_name());
        let core_library =
            CratePath::new(core_package, CrateKind::Lib, db).expect("should be valid");
        let std_library = CratePath::new(std_package, CrateKind::Lib, db).expect("should be valid");
        let core_root = core_library.root_module_path(db);
        let std_root = std_library.root_module_path(db);
        // todo: refactor with it_ident_borrowed_uncheck or Ident::from_borrowed_unchecked
        let core_array = ModulePath::new_child(
            db,
            core_root,
            Ident::from_ref(db, "array").expect("should be valid identifier"),
        )
        .expect("should be valid");
        let core_basic = ModulePath::new_child(
            db,
            core_root,
            Ident::from_ref(db, "basic").expect("should be valid identifier"),
        )
        .expect("should be valid");
        let core_backend = ModulePath::new_child(
            db,
            core_root,
            Ident::from_ref(db, "backend").expect("should be valid identifier"),
        )
        .expect("should be valid");
        let core_clone = ModulePath::new_child(
            db,
            core_root,
            Ident::from_ref(db, "clone").expect("should be valid identifier"),
        )
        .expect("should be valid");
        let core_cmp = ModulePath::new_child(
            db,
            core_root,
            Ident::from_ref(db, "cmp").expect("should be valid identifier"),
        )
        .expect("should be valid");
        let core_default = ModulePath::new_child(
            db,
            core_root,
            Ident::from_ref(db, "default").expect("should be valid identifier"),
        )
        .expect("should be valid");
        let core_fmt = ModulePath::new_child(
            db,
            core_root,
            Ident::from_ref(db, "fmt").expect("should be valid identifier"),
        )
        .expect("should be valid");
        let core_frontend = ModulePath::new_child(
            db,
            core_root,
            Ident::from_ref(db, "frontend").expect("should be valid identifier"),
        )
        .expect("should be valid");
        let core_vec = ModulePath::new_child(
            db,
            core_root,
            Ident::from_ref(db, "vec").expect("should be valid identifier"),
        )
        .expect("should be valid");
        let core_marker = ModulePath::new_child(
            db,
            core_root,
            Ident::from_ref(db, "marker").expect("should be valid identifier"),
        )
        .expect("should be valid");
        let core_mem = ModulePath::new_child(
            db,
            core_root,
            Ident::from_ref(db, "mem").expect("should be valid identifier"),
        )
        .expect("should be valid");
        let core_num = ModulePath::new_child(
            db,
            core_root,
            Ident::from_ref(db, "num").expect("should be valid identifier"),
        )
        .expect("should be valid");
        let core_ops = ModulePath::new_child(
            db,
            core_root,
            Ident::from_ref(db, "ops").expect("should be valid identifier"),
        )
        .expect("should be valid");
        let core_option = ModulePath::new_child(
            db,
            core_root,
            Ident::from_ref(db, "option").expect("should be valid identifier"),
        )
        .expect("should be valid");
        let core_prelude = ModulePath::new_child(
            db,
            core_root,
            Ident::from_ref(db, "prelude").expect("should be valid identifier"),
        )
        .expect("should be valid");
        let core_raw_bits = ModulePath::new_child(
            db,
            core_root,
            Ident::from_ref(db, "raw_bits").expect("should be valid identifier"),
        )
        .expect("should be valid");
        let core_result = ModulePath::new_child(
            db,
            core_root,
            Ident::from_ref(db, "result").expect("should be valid identifier"),
        )
        .expect("should be valid");
        let core_slice = ModulePath::new_child(
            db,
            core_root,
            Ident::from_ref(db, "slice").expect("should be valid identifier"),
        )
        .expect("should be valid");
        let core_str = ModulePath::new_child(
            db,
            core_root,
            Ident::from_ref(db, "str").expect("should be valid identifier"),
        )
        .expect("should be valid");
        let core_visual = ModulePath::new_child(
            db,
            core_root,
            Ident::from_ref(db, "visual").expect("should be valid identifier"),
        )
        .expect("should be valid");
        Self {
            core_package,
            std_package,
            core_library,
            std_library,
            core_root,
            std_root,
            core_array,
            core_basic,
            core_backend,
            core_clone,
            core_cmp,
            core_default,
            core_fmt,
            core_frontend,
            core_marker,
            core_mem,
            core_num,
            core_ops,
            core_option,
            core_prelude,
            core_raw_bits,
            core_result,
            core_slice,
            core_str,
            core_vec,
            core_visual,
        }
    }

    pub fn core_package(&self) -> PackagePath {
        self.core_package
    }

    pub fn std_package(&self) -> PackagePath {
        self.std_package
    }

    pub fn core_library(&self) -> CratePath {
        self.core_library
    }

    pub fn core_root(&self) -> ModulePath {
        self.core_root
    }

    pub fn core_basic(&self) -> SubmodulePath {
        self.core_basic
    }

    pub fn core_clone(&self) -> SubmodulePath {
        self.core_clone
    }

    pub fn core_cmp(&self) -> SubmodulePath {
        self.core_cmp
    }

    pub fn core_fmt(&self) -> SubmodulePath {
        self.core_fmt
    }

    pub fn core_default(&self) -> SubmodulePath {
        self.core_default
    }

    /// core::marker
    pub fn core_marker(&self) -> SubmodulePath {
        self.core_marker
    }

    /// core::mem
    pub fn core_mem(&self) -> SubmodulePath {
        self.core_mem
    }

    /// core::num
    pub fn core_num(&self) -> SubmodulePath {
        self.core_num
    }

    pub fn core_prelude(&self) -> SubmodulePath {
        self.core_prelude
    }

    /// core::raw_bits
    pub fn core_raw_bits(&self) -> SubmodulePath {
        self.core_raw_bits
    }

    pub fn core_result(&self) -> SubmodulePath {
        self.core_result
    }

    /// core::vec
    pub fn core_vec(&self) -> SubmodulePath {
        self.core_vec
    }

    /// core::array
    pub fn core_array(&self) -> SubmodulePath {
        self.core_array
    }

    /// core::visual
    pub fn core_visual(&self) -> SubmodulePath {
        self.core_visual
    }

    pub fn core_backend(&self) -> SubmodulePath {
        self.core_backend
    }

    pub fn core_frontend(&self) -> SubmodulePath {
        self.core_frontend
    }

    pub fn std_root(&self) -> ModulePath {
        self.std_root
    }

    pub fn core_ops(&self) -> SubmodulePath {
        self.core_ops
    }

    pub fn core_option(&self) -> SubmodulePath {
        self.core_option
    }

    pub fn core_slice(&self) -> SubmodulePath {
        self.core_slice
    }

    pub fn core_str(&self) -> SubmodulePath {
        self.core_str
    }
}

#[test]
fn vfs_path_menu_works() {
    let db = DB::default();
    let db = &*db;
    let toolchain = db.dev_toolchain().unwrap();
    let menu = db.vfs_path_menu(toolchain);
    assert_eq!(menu.core_root().to_string_with_db(db), "core");
    assert_eq!(menu.std_root().to_string_with_db(db), "std");
    assert_eq!(menu.core_basic().to_string_with_db(db), "core::basic");
    assert_eq!(menu.core_default().to_string_with_db(db), "core::default");
    assert_eq!(menu.core_mem().to_string_with_db(db), "core::mem");
    assert_eq!(menu.core_num().to_string_with_db(db), "core::num");
    assert_eq!(menu.core_slice().to_string_with_db(db), "core::slice");
    assert_eq!(menu.core_str().to_string_with_db(db), "core::str");
    assert_eq!(menu.core_ops().to_string_with_db(db), "core::ops");
    assert_eq!(menu.core_option().to_string_with_db(db), "core::option");
    assert_eq!(menu.core_prelude().to_string_with_db(db), "core::prelude");
    assert_eq!(menu.core_raw_bits().to_string_with_db(db), "core::raw_bits");
    assert_eq!(menu.core_vec().to_string_with_db(db), "core::vec");
    assert_eq!(menu.core_visual().to_string_with_db(db), "core::visual");
}
