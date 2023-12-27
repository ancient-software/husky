use super::*;

pub(super) fn enum_value_conversion(item: syn::ItemEnum) -> TokenStream {
    let syn::ItemEnum {
        ref attrs,
        ref vis,
        enum_token,
        ref ident,
        ref generics,
        brace_token,
        ref variants,
    } = item;
    let self_ty = self_ty(ident, generics);
    let impl_weak_static_generic_constraints = impl_weak_static_generic_constraints(generics);
    let impl_weak_static_associated_ty_static =
        impl_weak_static_associated_ty_static(ident, generics);
    let impl_static_generic_constraints = impl_static_generic_constraints(generics);
    let impl_static_associated_ty_frozen = impl_static_associated_ty_frozen(ident, generics);
    let impl_frozen_generic_constraints = impl_frozen_generic_constraints(generics);
    let impl_frozen_associated_ty_static = impl_frozen_associated_ty_static(ident, generics);
    let impl_from_value_generic_constraints = impl_from_value_generic_constraints(generics);
    let impl_into_value_generic_constraints = impl_into_value_generic_constraints(generics);
    let is_trivial = variants.iter().all(|variant| match variant.fields {
        syn::Fields::Unit => true,
        syn::Fields::Named(_) | syn::Fields::Unnamed(_) => false,
    });
    if is_trivial {
        quote::quote! {
            #item

            impl #generics __WeakStatic for #self_ty where #impl_weak_static_generic_constraints {
                type Static = #impl_weak_static_associated_ty_static;

                unsafe fn into_static(self) -> Self::Static {
                    self
                }
            }

            impl #generics __Static for #self_ty where #impl_static_generic_constraints {
                type Frozen = #impl_static_associated_ty_frozen;

                unsafe fn freeze(&self) -> Self::Frozen {
                    // MutFrozen::new(*self)
                    todo!()
                }
            }

            impl #generics __Frozen for #self_ty where #impl_frozen_generic_constraints {
                type Static = #impl_frozen_associated_ty_static;

                type Stand = ();

                fn revive(&self) -> (Option<Self::Stand>, Self::Static) {
                    todo!()
                }
            }

            impl #generics __FromValue for #self_ty {
                fn from_value_aux(value: __Value, _value_stands: Option<&mut __ValueStands>) -> Self {
                    let __Value::EnumU8(index_raw) = value else {
                        unreachable!()
                    };
                    unsafe {
                        std::mem::transmute(index_raw)
                    }
                }
            }

            impl #generics __IntoValue for #self_ty {
                fn into_value(self) -> __Value {
                    __Value::EnumU8(unsafe {
                        std::mem::transmute(self)
                    })
                }
            }
        }
        .into()
    } else {
        quote::quote! {
            #item

            impl #generics __WeakStatic for #self_ty where #impl_weak_static_generic_constraints {
                type Static = #impl_weak_static_associated_ty_static;

                unsafe fn into_static(self) -> Self::Static {
                    self
                }
            }

            impl #generics __Static for #self_ty where #impl_static_generic_constraints {
                type Frozen = #impl_static_associated_ty_frozen;

                unsafe fn freeze(&self) -> Self::Frozen {
                    // MutFrozen::new(*self)
                    todo!()
                }
            }

            impl #generics __Frozen for #self_ty where #impl_frozen_generic_constraints {
                type Static = #impl_frozen_associated_ty_static;

                type Stand = ();

                fn revive(&self) -> (Option<Self::Stand>, Self::Static) {
                    todo!()
                }
            }

            impl #generics __FromValue for #self_ty where #impl_from_value_generic_constraints {
                fn from_value_aux(value: __Value, _: Option<&mut __ValueStands>) -> Self {
                    value.into_owned()
                }
            }

            impl #generics __IntoValue for #self_ty where #impl_into_value_generic_constraints {
                fn into_value(self) -> __Value {
                    __Value::from_owned(self)
                }
            }
        }
        .into()
    }
}
