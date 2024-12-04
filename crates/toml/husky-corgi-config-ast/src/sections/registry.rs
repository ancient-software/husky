use super::*;

#[derive(Debug, PartialEq, Eq)]
pub struct CorgiConfigRegistrySectionAst {
    path: Option<CorgiConfigAstResult<VirtualPath>>,
}

impl CorgiConfigRegistrySectionAst {
    pub fn path(&self) -> Option<CorgiConfigAstResultRef<VirtualPath>> {
        self.path.as_ref().map(|s| s.as_ref().copied())
    }
}

impl TransformFromTomlAst<CorgiConfigAstTransformContext> for CorgiConfigRegistrySectionAst {
    type Ast = TomlSection;

    fn transform_from<'a, 'b>(
        tf: TomlTransformer<'a, 'b, CorgiConfigAstTransformContext, Self::Ast>,
    ) -> CorgiConfigAstResult<Self> {
        let key = tf.menu().path_coword();
        Ok(CorgiConfigRegistrySectionAst {
            path: tf.transform_value(key),
        })
    }
}

impl TransformFromTomlParentKeyed<CorgiConfigAstTransformContext>
    for CorgiConfigRegistrySectionAst
{
    fn key(
        menu: &<CorgiConfigAstTransformContext as TomlDeserializeContext>::Menu,
    ) -> husky_coword::BaseCoword {
        menu.registry_coword()
    }
}
