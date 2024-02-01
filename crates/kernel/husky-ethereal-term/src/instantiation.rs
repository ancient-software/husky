use crate::*;
use maybe_result::*;
use vec_like::SmallVecPairMap;

#[salsa::debug_with_db]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct EtherealInstantiation {
    symbol_map: SmallVecPairMap<SymbolEtherealTerm, EtherealTerm, 4>,
    /// indicates the separation for associated item template instantiation
    separator: Option<u8>,
}

impl EtherealInstantiation {
    pub fn symbol_map(&self) -> &[(SymbolEtherealTerm, EtherealTerm)] {
        self.symbol_map.as_ref()
    }

    pub fn separator(&self) -> Option<u8> {
        self.separator
    }

    /// assume that symbol is in symbol_map
    /// panic otherwise
    pub fn symbol_instantiation(&self, symbol: SymbolEtherealTerm) -> EtherealTerm {
        *self
            .symbol_map
            .get_value(symbol)
            .expect("symbol should be in symbol_map")
    }

    pub fn symbol_map_splitted(
        &self,
    ) -> (
        &[(SymbolEtherealTerm, EtherealTerm)],
        Option<&[(SymbolEtherealTerm, EtherealTerm)]>,
    ) {
        let symbol_map: &[_] = self.symbol_map.as_ref();
        match self.separator {
            Some(separator) => {
                let separator = separator as usize;
                (&symbol_map[0..separator], Some(&symbol_map[separator..]))
            }
            None => (symbol_map, None),
        }
    }
}

pub trait EtherealTermInstantiate: Copy {
    type Output;

    fn instantiate(self, db: &::salsa::Db, instantiation: &EtherealInstantiation) -> Self::Output;
}

impl<T> EtherealTermInstantiate for Option<T>
where
    T: EtherealTermInstantiate,
{
    type Output = Option<T::Output>;

    fn instantiate(self, db: &salsa::Db, instantiation: &EtherealInstantiation) -> Self::Output {
        self.map(|slf| slf.instantiate(db, instantiation))
    }
}

impl<T> EtherealTermInstantiate for &[T]
where
    T: EtherealTermInstantiate,
{
    type Output = Vec<T::Output>;

    fn instantiate(self, db: &salsa::Db, instantiation: &EtherealInstantiation) -> Self::Output {
        self.iter()
            .copied()
            .map(|elem| elem.instantiate(db, instantiation))
            .collect()
    }
}

pub trait EtherealTermInstantiateRef {
    type Target;

    fn instantiate(&self, db: &::salsa::Db, instantiation: &EtherealInstantiation) -> Self::Target;
}

#[salsa::debug_with_db]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct EtherealInstantiationBuilder {
    symbol_map: SmallVecPairMap<SymbolEtherealTerm, Option<EtherealTerm>, 4>,
    /// indicates the separation for associated item template instantiation
    separator: Option<u8>,
}

impl EtherealInstantiationBuilder {
    /// symbols must be unique
    pub(crate) fn new(
        symbols: impl Iterator<Item = SymbolEtherealTerm>,
        is_associated: bool,
    ) -> Self {
        let symbol_map: SmallVecPairMap<SymbolEtherealTerm, Option<EtherealTerm>, 4> =
            symbols.map(|symbol| (symbol, None)).collect();
        Self {
            separator: is_associated.then_some(symbol_map.len().try_into().unwrap()),
            symbol_map,
        }
    }

    /// `JustOk(())` means rule is added and everything is compatible.
    ///
    /// `Nothing` means something is incompatible.
    ///
    /// `JustErr(e)` means something is wrong.
    pub fn try_add_rules_from_application(
        &mut self,
        src: EtherealTerm,
        dst_arguments: &[EtherealTerm],
        db: &::salsa::Db,
    ) -> EtherealTermMaybeResult<()> {
        let src_application_expansion = src.application_expansion(db);
        if src_application_expansion.arguments(db).len() != dst_arguments.len() {
            todo!()
        }
        std::iter::zip(
            src_application_expansion.arguments(db).iter().copied(),
            dst_arguments.iter().copied(),
        )
        .try_for_each(|(src, dst)| self.try_add_rule(src, dst.into(), db))?;
        JustOk(())
    }

    /// `JustOk(())` means rule is added and everything is compatible.
    ///
    /// `Nothing` means something is incompatible.
    ///
    /// `JustErr(e)` means something is wrong.
    pub fn try_add_rule(
        &mut self,
        src: EtherealTerm,
        dst: EtherealTerm,
        db: &::salsa::Db,
    ) -> EtherealTermMaybeResult<()> {
        if src == dst {
            return JustOk(());
        }
        match src {
            EtherealTerm::Symbol(symbol) => self.try_add_symbol_rule(symbol, dst),
            EtherealTerm::Application(_) => {
                let src_application_expansion = src.application_expansion(db);
                let dst_application_expansion = dst.application_expansion(db);
                if src_application_expansion.function() != dst_application_expansion.function() {
                    p!(
                        src_application_expansion.function().debug(db),
                        dst_application_expansion.function().debug(db)
                    );
                    todo!()
                }
                let src_application_arguments = src_application_expansion.arguments(db);
                let dst_application_arguments = dst_application_expansion.arguments(db);
                if src_application_arguments.len() != dst_application_arguments.len() {
                    todo!()
                }
                for (&src_arg, &dst_arg) in
                    std::iter::zip(src_application_arguments, dst_application_arguments)
                {
                    self.try_add_rule(src_arg, dst_arg, db)?
                }
                JustOk(())
            }
            EtherealTerm::Literal(_)
            | EtherealTerm::Rune(_)
            | EtherealTerm::EntityPath(_)
            | EtherealTerm::Category(_)
            | EtherealTerm::Universe(_) => Nothing,
            EtherealTerm::Curry(_) => todo!(),
            EtherealTerm::Ritchie(_) => todo!(),
            EtherealTerm::Abstraction(_) => todo!(),
            EtherealTerm::TypeAsTraitItem(_) => todo!(),
            EtherealTerm::TraitConstraint(_) => todo!(),
        }
    }

    pub fn try_add_symbol_rule(
        &mut self,
        symbol: SymbolEtherealTerm,
        dst: EtherealTerm,
    ) -> EtherealTermMaybeResult<()> {
        if let Some((_, opt_dst0)) = self.symbol_map.get_entry_mut(symbol) {
            match opt_dst0 {
                Some(dst0) => {
                    if dst != *dst0 {
                        todo!()
                    } else {
                        return JustOk(());
                    }
                }
                None => {
                    *opt_dst0 = Some(dst);
                    JustOk(())
                }
            }
        } else {
            Nothing
        }
    }

    pub fn try_into_instantiation(&self) -> Option<EtherealInstantiation> {
        let mut symbol_map = SmallVecPairMap::<SymbolEtherealTerm, EtherealTerm, 4>::default();
        for (symbol, mapped) in self.symbol_map.iter() {
            let mapped = (*mapped)?;
            unsafe { symbol_map.insert_new_unchecked((*symbol, mapped)) }
        }
        Some(EtherealInstantiation {
            symbol_map,
            separator: self.separator,
        })
    }

    pub fn merge_with_item_template_parameters(
        &self,
        template_parameters: &EtherealTemplateParameters,
    ) -> Self {
        let mut symbol_map = self.symbol_map.clone();
        let len = symbol_map.len().try_into().unwrap();
        for param in template_parameters.iter() {
            unsafe { symbol_map.insert_new_unchecked((param.symbol(), None)) }
        }
        Self {
            symbol_map,
            separator: Some(len),
        }
    }
}
