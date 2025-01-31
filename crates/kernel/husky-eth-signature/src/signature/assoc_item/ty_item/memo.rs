use super::*;
use crate::signature::impl_block::ty_impl_block::TypeImplBlockEthTemplate;
use helpers::trai_for_ty::is_ty_term_always_copyable;
use husky_dec_signature::signature::assoc_item::ty_item::memo::TypeMemoizedFieldDecTemplate;
use package::PackageEthSignatureData;

#[salsa::interned]
pub struct TypeMemoizedFieldEthTemplate {
    pub path: TypeItemPath,
    pub impl_block: TypeImplBlockEthTemplate,
    pub return_ty: EthTerm,
    pub expr_ty: EthTerm,
}

impl TypeMemoizedFieldEthTemplate {
    pub fn self_ty(self, db: &::salsa::Db) -> EthTerm {
        self.impl_block(db).self_ty(db)
    }
}

impl TypeMemoizedFieldEthTemplate {
    pub(super) fn from_dec(
        db: &::salsa::Db,
        path: TypeItemPath,
        declarative_signature: TypeMemoizedFieldDecTemplate,
    ) -> EthSignatureResult<TypeMemoizedFieldEthTemplate> {
        let impl_block = path.impl_block(db).eth_template(db)?;
        let return_ty = EthTerm::ty_from_dec(db, declarative_signature.return_ty(db))?;
        let expr_ty = if is_ty_term_always_copyable(return_ty, db)?
            .expect("should be valid to check copyable")
        {
            return_ty
        } else {
            return_ty.leashed(db)
        };
        Ok(TypeMemoizedFieldEthTemplate::new(
            db, path, impl_block, return_ty, expr_ty,
        ))
    }
}

impl TypeMemoizedFieldEthTemplate {
    fn try_instantiate<'db>(
        self,
        target_self_ty_arguments: &[EthTerm],
        ctx: EthTermContextRef,
        db: &'db ::salsa::Db,
    ) -> EthSignatureMaybeResult<TypeMemoizedFieldEthSignature> {
        let self_ty = self.impl_block(db).self_ty(db);
        let mut builder = self
            .impl_block(db)
            .template_parameters(db)
            .empty_instantiation_builder(self.path(db).into(), true, ctx);
        builder.try_add_rules_from_application(self_ty, target_self_ty_arguments, db)?;
        let instantiation = builder.try_into_instantiation().expect("business done");
        debug_assert!(instantiation.separator().is_some());
        let self_ty = self.self_ty(db).instantiate(&instantiation, ctx, db);
        let return_ty = self.return_ty(db).instantiate(&instantiation, ctx, db);
        let expr_ty = self.expr_ty(db).instantiate(&instantiation, ctx, db);
        JustOk(TypeMemoizedFieldEthSignature {
            path: self.path(db),
            self_ty,
            instantiation,
            return_ty,
            expr_ty,
        })
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct TypeMemoizedFieldEthSignature {
    path: TypeItemPath,
    instantiation: EthInstantiation,
    self_ty: EthTerm,
    return_ty: EthTerm,
    expr_ty: EthTerm,
}

impl TypeMemoizedFieldEthSignature {
    pub fn path(&self) -> TypeItemPath {
        self.path
    }

    pub fn instantiation(&self) -> &EthInstantiation {
        &self.instantiation
    }

    pub fn return_ty(&self) -> EthTerm {
        self.return_ty
    }

    pub fn expr_ty(&self) -> EthTerm {
        self.expr_ty
    }
}

pub trait HasTypeMemoizedFieldEthTemplates: Copy {
    fn ty_memo_field_eth_templates_map<'a>(
        self,
        db: &'a ::salsa::Db,
    ) -> EthSignatureResult<
        &'a [(
            Ident,
            EthSignatureResult<SmallVecImpl<TypeMemoizedFieldEthTemplate>>,
        )],
    >;

    fn ty_memo_field_eth_templates<'a>(
        self,
        db: &'a ::salsa::Db,
        ident: Ident,
    ) -> EthSignatureMaybeResult<&'a [TypeMemoizedFieldEthTemplate]> {
        use vec_like::VecMapGetEntry;
        match self.ty_memo_field_eth_templates_map(db)?.get_entry(ident) {
            Some((_, Ok(templates))) => JustOk(templates),
            Some((_, Err(e))) => JustErr(*e),
            None => Nothing,
        }
    }
}

pub trait HasTypeMemoizedFieldEthSignature: Copy {
    fn ty_memo_field_ethereal_signature<'db>(
        self,
        arguments: &'db [EthTerm],
        ident: Ident,
        ctx: EthTermContextRef,
        db: &'db ::salsa::Db,
    ) -> EthSignatureMaybeResult<TypeMemoizedFieldEthSignature>;
}

impl HasTypeMemoizedFieldEthSignature for TypePath {
    fn ty_memo_field_ethereal_signature<'db>(
        self,
        arguments: &'db [EthTerm],
        ident: Ident,
        ctx: EthTermContextRef,
        db: &'db ::salsa::Db,
    ) -> EthSignatureMaybeResult<TypeMemoizedFieldEthSignature> {
        let TypeItemEthTemplates::MemoizedField(tmpls) = self.ty_item_eth_templates(db, ident)?
        else {
            return Nothing;
        };
        for tmpl in tmpls {
            if let Some(signature) = tmpl
                .try_instantiate(arguments, ctx, db)
                .into_result_option()?
            {
                return JustOk(signature);
            }
        }
        Nothing
    }
}
