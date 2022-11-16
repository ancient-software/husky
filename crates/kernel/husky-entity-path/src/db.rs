use crate::*;
use husky_word::InternWord;
use std::sync::Arc;

#[salsa::query_group(EntityPathDbStorage)]
pub trait EntityPathDb: InternEntityPath + InternWord {
    fn entity_path_menu(&self) -> Arc<EntityPathMenu>;
}

fn entity_path_menu(db: &dyn EntityPathDb) -> Arc<EntityPathMenu> {
    Arc::new(EntityPathMenu::new(db))
}

impl dyn EntityPathDb + '_ {
    pub(crate) fn it_root_entity_path(&self, ident: &str) -> EntityPathItd {
        self.it_entity_path(EntityPath::root(self.it_ident(ident)))
    }
    pub(crate) fn it_child_entity_path(&self, parent: EntityPathItd, ident: &str) -> EntityPathItd {
        self.it_entity_path(EntityPath {
            ident: self.it_ident(ident),
            variant: EntityPathVariant::Childpath { parent },
        })
    }
}
