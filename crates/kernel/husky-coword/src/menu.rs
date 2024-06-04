use salsa::Db;

use crate::*;

#[derive(Debug, PartialEq, Eq)]
pub struct CowordMenu {
    std_name: Kebab,
    std_ident: Ident,
    core_name: Kebab,
    core_ident: Ident,
    unit_ident: Ident,
    never_ident: Ident,
    bool_ident: Ident,
    i8_ident: Ident,
    i16_ident: Ident,
    i32_ident: Ident,
    i64_ident: Ident,
    i128_ident: Ident,
    isize_ident: Ident,
    u8_ident: Ident,
    u16_ident: Ident,
    u32_ident: Ident,
    u64_ident: Ident,
    u128_ident: Ident,
    usize_ident: Ident,
    r8_ident: Ident,
    r16_ident: Ident,
    r32_ident: Ident,
    r64_ident: Ident,
    r128_ident: Ident,
    rsize_ident: Ident,
    f32_ident: Ident,
    f64_ident: Ident,
    /// `Trait`
    trai_ty_ident: Ident,
    /// `Lifetime`
    lifetime_ty_ident: Ident,
    place_ty_ident: Ident,
    task_ty_ident: Ident,
    module_ident: Ident,
    crate_ident: Ident,
    camel_case_output_ident: Ident,
    snake_case_unveil_ident: Ident,
    /// # attrs
    affect_ident: Ident,
    backprop_ident: Ident,
    deps_ident: Ident,
    derive_ident: Ident,
    task_ident: Ident,
    test_ident: Ident,
}

impl CowordMenu {
    pub(crate) fn new(db: &::salsa::Db) -> Self {
        Self {
            core_name: Kebab::from_ref(db, "core").unwrap(),
            core_ident: Ident::from_ref(db, "core").unwrap(),
            std_name: Kebab::from_ref(db, "std").unwrap(),
            std_ident: Ident::from_ref(db, "std").unwrap(),
            unit_ident: Ident::from_ref(db, "unit").unwrap(),
            never_ident: Ident::from_ref(db, "never").unwrap(),
            bool_ident: Ident::from_ref(db, "bool").unwrap(),
            i8_ident: Ident::from_ref(db, "i8").unwrap(),
            i16_ident: Ident::from_ref(db, "i16").unwrap(),
            i32_ident: Ident::from_ref(db, "i32").unwrap(),
            i64_ident: Ident::from_ref(db, "i64").unwrap(),
            i128_ident: Ident::from_ref(db, "i128").unwrap(),
            isize_ident: Ident::from_ref(db, "isize").unwrap(),
            u8_ident: Ident::from_ref(db, "u8").unwrap(),
            u16_ident: Ident::from_ref(db, "u16").unwrap(),
            u32_ident: Ident::from_ref(db, "u32").unwrap(),
            u64_ident: Ident::from_ref(db, "u64").unwrap(),
            u128_ident: Ident::from_ref(db, "u128").unwrap(),
            usize_ident: Ident::from_ref(db, "usize").unwrap(),
            r8_ident: Ident::from_ref(db, "r8").unwrap(),
            r16_ident: Ident::from_ref(db, "r16").unwrap(),
            r32_ident: Ident::from_ref(db, "r32").unwrap(),
            r64_ident: Ident::from_ref(db, "r64").unwrap(),
            r128_ident: Ident::from_ref(db, "r128").unwrap(),
            rsize_ident: Ident::from_ref(db, "rsize").unwrap(),
            f32_ident: Ident::from_ref(db, "f32").unwrap(),
            f64_ident: Ident::from_ref(db, "f64").unwrap(),
            trai_ty_ident: Ident::from_ref(db, "Trait").unwrap(),
            module_ident: Ident::from_ref(db, "Module").unwrap(),
            crate_ident: Ident::from_ref(db, "crate").unwrap(),
            lifetime_ty_ident: Ident::from_ref(db, "Lifetime").unwrap(),
            place_ty_ident: Ident::from_ref(db, "Place").unwrap(),
            task_ty_ident: Ident::from_ref(db, "Task").unwrap(),
            camel_case_output_ident: Ident::from_ref(db, "Output").unwrap(),
            snake_case_unveil_ident: Ident::from_ref(db, "unveil").unwrap(),
            affect_ident: Ident::from_ref(db, "affect").unwrap(),
            backprop_ident: Ident::from_ref(db, "backprop").unwrap(),
            deps_ident: Ident::from_ref(db, "deps").unwrap(),
            derive_ident: Ident::from_ref(db, "derive").unwrap(),
            task_ident: Ident::from_ref(db, "task").unwrap(),
            test_ident: Ident::from_ref(db, "test").unwrap(),
        }
    }

    pub fn core_name(&self) -> Kebab {
        self.core_name
    }

    pub fn core_ident(&self) -> Ident {
        self.core_ident
    }

    pub fn std_name(&self) -> Kebab {
        self.std_name
    }

    pub fn std_ident(&self) -> Ident {
        self.std_ident
    }

    pub fn i8_ident(&self) -> Ident {
        self.i8_ident
    }

    pub fn i16_ident(&self) -> Ident {
        self.i16_ident
    }

    pub fn i32_ident(&self) -> Ident {
        self.i32_ident
    }

    pub fn i64_ident(&self) -> Ident {
        self.i64_ident
    }

    pub fn i128_ident(&self) -> Ident {
        self.i128_ident
    }

    pub fn isize_ident(&self) -> Ident {
        self.isize_ident
    }

    pub fn r8_ident(&self) -> Ident {
        self.r8_ident
    }

    pub fn r16_ident(&self) -> Ident {
        self.r16_ident
    }

    pub fn r32_ident(&self) -> Ident {
        self.r32_ident
    }

    pub fn r64_ident(&self) -> Ident {
        self.r64_ident
    }

    pub fn r128_ident(&self) -> Ident {
        self.r128_ident
    }

    pub fn rsize_ident(&self) -> Ident {
        self.rsize_ident
    }

    pub fn f32_ident(&self) -> Ident {
        self.f32_ident
    }

    pub fn f64_ident(&self) -> Ident {
        self.f64_ident
    }

    pub fn unit_ident(&self) -> Ident {
        self.unit_ident
    }

    pub fn bool_ident(&self) -> Ident {
        self.bool_ident
    }

    pub fn u8_ident(&self) -> Ident {
        self.u8_ident
    }

    pub fn u16_ident(&self) -> Ident {
        self.u16_ident
    }

    pub fn u32_ident(&self) -> Ident {
        self.u32_ident
    }

    pub fn u64_ident(&self) -> Ident {
        self.u64_ident
    }

    pub fn u128_ident(&self) -> Ident {
        self.u128_ident
    }

    pub fn usize_ident(&self) -> Ident {
        self.usize_ident
    }

    pub fn trai_ty_ident(&self) -> Ident {
        self.trai_ty_ident
    }

    pub fn module_ident(&self) -> Ident {
        self.module_ident
    }

    pub fn crate_ident(&self) -> Ident {
        self.crate_ident
    }

    pub fn lifetime_ty_ident(&self) -> Ident {
        self.lifetime_ty_ident
    }

    pub fn place_ty_ident(&self) -> Ident {
        self.place_ty_ident
    }

    pub fn task_ty_ident(&self) -> Ident {
        self.task_ty_ident
    }

    pub fn never_ident(&self) -> Ident {
        self.never_ident
    }

    /// `backprop`
    pub fn backprop_ident(&self) -> Ident {
        self.backprop_ident
    }

    /// `affect`
    pub fn affect_ident(&self) -> Ident {
        self.affect_ident
    }

    /// `deps`
    pub fn deps_ident(&self) -> Ident {
        self.deps_ident
    }

    /// `derive`
    pub fn derive_ident(&self) -> Ident {
        self.derive_ident
    }

    /// `task`
    pub fn task_ident(&self) -> Ident {
        self.task_ident
    }

    /// `test`
    pub fn test_ident(&self) -> Ident {
        self.test_ident
    }

    pub fn camel_case_output_ident(&self) -> Ident {
        self.camel_case_output_ident
    }

    pub fn snake_case_unveil_ident(&self) -> Ident {
        self.snake_case_unveil_ident
    }
}

// #[salsa::tracked(jar = Jar, return_ref)]
// pub(crate) fn coword_menu(db: &::salsa::Db,) -> CowordMenu {
//     CowordMenu::new(db)
// }

#[allow(non_camel_case_types)]
pub(crate) struct coword_menu {
    intern_map: salsa::interned::IdentityInterner<()>,
    function: salsa::function::FunctionIngredient<Self>,
}
impl salsa::function::Configuration for coword_menu {
    type Jar = Jar;
    type SalsaStruct = salsa::salsa_struct::Singleton;
    type Key = ();
    type Value = CowordMenu;
    const CYCLE_STRATEGY: salsa::cycle::CycleRecoveryStrategy =
        salsa::cycle::CycleRecoveryStrategy::Panic;
    fn should_backdate_value(v1: &Self::Value, v2: &Self::Value) -> bool {
        salsa::function::should_backdate_value(v1, v2)
    }
    fn execute(__db: &::salsa::Db, __id: Self::Key) -> Self::Value {
        pub(crate) fn __fn(db: &::salsa::Db) -> CowordMenu {
            CowordMenu::new(db)
        }
        let (__jar, __runtime) = __db.jar::<Jar>();
        let __ingredients =
            <_ as salsa::storage::HasIngredientsFor<coword_menu>>::ingredient(__jar);
        let __key = __ingredients.intern_map.data(__runtime, __id).clone();
        __fn(__db)
    }
    fn recover_from_cycle(
        _db: &::salsa::Db,
        _cycle: &salsa::Cycle,
        _key: Self::Key,
    ) -> Self::Value {
        panic!()
    }
}
impl salsa::storage::IngredientsFor for coword_menu {
    type Ingredients = Self;
    type Jar = Jar;
    fn create_ingredients(routes: &mut salsa::routes::Routes) -> Self::Ingredients {
        Self {
            intern_map: salsa::interned::IdentityInterner::new(),
            function: {
                let index = routes.push(
                    |jars| {
                        let jar = jars.jar::<Self::Jar>();
                        let ingredients = <_ as salsa::storage::HasIngredientsFor<
                            Self::Ingredients,
                        >>::ingredient(jar);
                        &ingredients.function
                    },
                    |jars| {
                        let jar = jars.jar_mut::<Self::Jar>();
                        let ingredients = <_ as salsa::storage::HasIngredientsFor<
                            Self::Ingredients,
                        >>::ingredient_mut(jar);
                        &mut ingredients.function
                    },
                );
                let ingredient = salsa::function::FunctionIngredient::new(index, "coword_menu");
                ingredient.set_capacity(0usize);
                ingredient
            },
        }
    }
}
impl coword_menu {
    #[allow(dead_code, clippy::needless_lifetimes)]
    pub(crate) fn get<'__db>(db: &'__db Db) -> &'__db CowordMenu {
        let (__jar, __runtime) = db.jar::<Jar>();
        let __ingredients =
            <_ as salsa::storage::HasIngredientsFor<coword_menu>>::ingredient(__jar);
        let __key = __ingredients.intern_map.intern(__runtime, ());
        __ingredients.function.fetch(db, __key)
    }
    #[allow(dead_code, clippy::needless_lifetimes)]
    pub(crate) fn set(db: &mut Db, __value: CowordMenu) {
        let (__jar, __runtime) = db.jar_mut::<Jar>();
        let __ingredients =
            <_ as salsa::storage::HasIngredientsFor<coword_menu>>::ingredient_mut(__jar);
        let __key = __ingredients.intern_map.intern(__runtime, ());
        __ingredients
            .function
            .store(__runtime, __key, __value, salsa::Durability::LOW)
    }
    #[allow(dead_code, clippy::needless_lifetimes)]
    pub(crate) fn accumulated<'__db, __A: salsa::accumulator::Accumulator>(
        db: &'__db Db,
    ) -> Vec<<__A as salsa::accumulator::Accumulator>::Data> {
        let (__jar, __runtime) = db.jar::<Jar>();
        let __ingredients =
            <_ as salsa::storage::HasIngredientsFor<coword_menu>>::ingredient(__jar);
        let __key = __ingredients.intern_map.intern(__runtime, ());
        __ingredients.function.accumulated::<__A>(db, __key)
    }
}
#[allow(clippy::needless_lifetimes)]
pub fn coword_menu<'__db>(db: &'__db Db) -> &'__db CowordMenu {
    coword_menu::get(db)
}
