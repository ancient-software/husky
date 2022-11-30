use crate::*;
use place::SingleAssignPlace;

pub struct VfsJar(
    <PathBufItd as salsa::storage::IngredientsFor>::Ingredients,
    <SourceFile as salsa::storage::IngredientsFor>::Ingredients,
    <path_class as salsa::storage::IngredientsFor>::Ingredients,
    HuskyFileCache,
    SingleAssignPlace<VfsWatcher>,
);

impl VfsJar {
    pub(crate) fn husky_file_cache(&self) -> &HuskyFileCache {
        &self.3
    }

    pub(crate) fn watcher_place(&self) -> &SingleAssignPlace<VfsWatcher> {
        &self.4
    }

    pub(crate) fn watcher_place_mut(&mut self) -> &mut SingleAssignPlace<VfsWatcher> {
        &mut self.4
    }
}

impl salsa::storage::HasIngredientsFor<PathBufItd> for VfsJar {
    fn ingredient(&self) -> &<PathBufItd as salsa::storage::IngredientsFor>::Ingredients {
        &self.0
    }
    fn ingredient_mut(
        &mut self,
    ) -> &mut <PathBufItd as salsa::storage::IngredientsFor>::Ingredients {
        &mut self.0
    }
}
impl salsa::storage::HasIngredientsFor<SourceFile> for VfsJar {
    fn ingredient(&self) -> &<SourceFile as salsa::storage::IngredientsFor>::Ingredients {
        &self.1
    }
    fn ingredient_mut(
        &mut self,
    ) -> &mut <SourceFile as salsa::storage::IngredientsFor>::Ingredients {
        &mut self.1
    }
}
impl salsa::storage::HasIngredientsFor<path_class> for VfsJar {
    fn ingredient(&self) -> &<path_class as salsa::storage::IngredientsFor>::Ingredients {
        &self.2
    }
    fn ingredient_mut(
        &mut self,
    ) -> &mut <path_class as salsa::storage::IngredientsFor>::Ingredients {
        &mut self.2
    }
}
impl<'salsa_db> salsa::jar::Jar<'salsa_db> for VfsJar {
    type DynDb = dyn VfsDb + 'salsa_db;
    fn create_jar<DB>(routes: &mut salsa::routes::Routes<DB>) -> Self
    where
        DB: salsa::storage::JarFromJars<Self> + salsa::storage::DbWithJar<Self>,
    {
        let i0 = <PathBufItd as salsa::storage::IngredientsFor>::create_ingredients(routes);
        let i1 = <SourceFile as salsa::storage::IngredientsFor>::create_ingredients(routes);
        let i2 = <path_class as salsa::storage::IngredientsFor>::create_ingredients(routes);
        Self(i0, i1, i2, Default::default(), Default::default())
    }
}
