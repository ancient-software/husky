use entity_route::{EntityKind, GenericArgument};
use vm::*;
use word::RootIdentifier;

use crate::{convexity::Convexity, symbol::SymbolContext, *};

#[derive(Debug)]
pub(crate) struct AtomStack {
    pub(crate) atoms: Vec<Atom>,
}

impl Into<Vec<Atom>> for AtomStack {
    fn into(self) -> Vec<Atom> {
        self.atoms
    }
}

// get
impl AtomStack {
    pub fn new() -> Self {
        Self { atoms: Vec::new() }
    }

    pub(crate) fn convexity(&self) -> Convexity {
        if let Some(atom) = self.atoms.last() {
            convexity::right_side_convexity(&atom.kind)
        } else {
            Convexity::Concave
        }
    }

    pub(crate) fn is_convex(&self) -> bool {
        self.convexity() == Convexity::Convex
    }

    pub(crate) fn is_concave(&self) -> bool {
        self.convexity() == Convexity::Concave
    }
}

// push
impl AtomStack {
    pub(crate) fn push(&mut self, atom: Atom) -> AtomResult<()> {
        if convexity::compatible(self.convexity(), convexity::left_side_convexity(&atom.kind)) {
            self.atoms.push(atom);
            Ok(())
        } else {
            err!("convexity not compatible", atom.text_range())
        }
    }

    pub(crate) fn end_list(&mut self, ket: Bracket, attr: ListEndAttr, ket_range: TextRange) {
        if self.is_convex() {
            self.push(Atom::new(ket_range.clone(), AtomVariant::ListItem))
                .unwrap();
        }
        self.push(Atom::new(ket_range, AtomVariant::ListEnd(ket, attr)))
            .unwrap();
    }

    pub(crate) fn end_list_or_make_type(
        &mut self,
        ket: Bracket,
        attr: ListEndAttr,
        mut tail: TextRange,
        symbol_context: &SymbolContext,
    ) -> AtomResult<()> {
        match (ket, self.atoms.last()) {
            (
                Bracket::Par,
                Some(Atom {
                    kind:
                        AtomVariant::EntityRoute {
                            kind: EntityKind::Type(_),
                            ..
                        },
                    ..
                }),
            ) => {
                let (attr, mut generics) = self.pop_par_list_of_types(&mut tail)?;
                let ident = match attr {
                    ListStartAttr::None => RootIdentifier::Tuple,
                    ListStartAttr::Attach => {
                        generics.push(EntityRoutePtr::Root(RootIdentifier::Void).into());
                        self.func_generic(attr)?
                    }
                };
                self.push(symbol_context.builtin_type_atom(ident, generics, tail))
            }
            _ => Ok(self.end_list(ket, attr, tail)),
        }
    }

    pub(crate) fn start_list(&mut self, bra: Bracket, text_range: TextRange) {
        self.push(Atom::new(
            text_range,
            AtomVariant::ListStart(
                bra,
                if self.is_convex() {
                    ListStartAttr::Attach
                } else {
                    ListStartAttr::None
                }
                .into(),
            ),
        ))
        .unwrap();
    }

    fn func_generic(&mut self, attr: ListStartAttr) -> AtomResult<RootIdentifier> {
        let expectation = "expect Fp, Fn, FnMut, FnOnce";

        match attr {
            ListStartAttr::None => Ok(word::default_func_type()),
            ListStartAttr::Attach => {
                let last_atom = self.atoms.pop().unwrap();
                match last_atom.kind {
                    AtomVariant::EntityRoute {
                        route: EntityRoutePtr::Root(ident),
                        ..
                    } => match ident {
                        RootIdentifier::Fp
                        | RootIdentifier::Fn
                        | RootIdentifier::FnMut
                        | RootIdentifier::FnOnce => Ok(ident),
                        _ => err!(expectation, last_atom.text_range()),
                    },
                    _ => err!(expectation, last_atom.text_range()),
                }
            }
        }
    }

    fn pop(&mut self, follower: &mut TextRange) -> AtomResult<Atom> {
        let atom = self
            .atoms
            .pop()
            .ok_or(error!("something before it", follower.clone()))?;
        *follower = atom.to(follower);
        Ok(atom)
    }

    fn pop_par_list_of_types(
        &mut self,
        tail: &mut TextRange,
    ) -> AtomResult<(ListStartAttr, Vec<GenericArgument>)> {
        let mut types = Vec::new();
        match self.pop(tail)?.kind {
            AtomVariant::ListStart(Bracket::Par, attr) => return Ok((attr, Vec::new())),
            AtomVariant::EntityRoute {
                route: scope,
                kind: EntityKind::Type(_),
            } => types.push(scope.into()),
            _ => err!("left parenthesis or type", *tail)?,
        };
        loop {
            match self.pop(tail)?.kind {
                AtomVariant::ListStart(Bracket::Par, attr) => {
                    types.reverse();
                    return Ok((attr, types));
                }
                AtomVariant::ListItem => (),
                _ => err!("left parenthesis or comma", *tail)?,
            }
            match self.pop(tail)?.kind {
                AtomVariant::EntityRoute {
                    route: scope,
                    kind: EntityKind::Type(_),
                } => types.push(scope.into()),
                _ => err!("type", *tail)?,
            }
        }
    }

    pub(crate) fn make_func_type(
        &mut self,
        symbol_context: &SymbolContext,
        output: EntityRoutePtr,
        mut tail: TextRange,
    ) -> AtomResult<()> {
        let (attr, mut generics) = self.pop_par_list_of_types(&mut tail)?;
        generics.push(output.into());
        let func_type = self.func_generic(attr)?;
        self.push(symbol_context.builtin_type_atom(func_type, generics, tail))
    }
}
