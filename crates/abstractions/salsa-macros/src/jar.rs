use proc_macro2::Literal;
use syn::punctuated::Punctuated;
use syn::{Field, FieldsUnnamed, Ident, ItemStruct, Token};

// Source:
//
// #[salsa::jar]
// pub struct Jar0(Entity0, Ty0, EntityComponent0, my_func);

pub(crate) fn jar(
    args: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let input = syn::parse_macro_input!(input as ItemStruct);
    jar_struct_and_friends(&input).into()
}

struct Jar;

impl crate::options::AllowedOptions for Jar {
    const RETURN_REF: bool = false;

    const SPECIFY: bool = false;

    const NO_EQ: bool = false;

    const SINGLETON: bool = false;

    const JAR: bool = false;

    const DATA: bool = false;

    const DB: bool = false;

    const RECOVERY_FN: bool = false;

    const LRU: bool = false;

    const CONSTRUCTOR: bool = false;

    const OVERRIDE_DEBUG: bool = false;
}

pub(crate) fn jar_struct_and_friends(input: &ItemStruct) -> proc_macro2::TokenStream {
    let output_struct = jar_struct(input);

    let jar_struct = &input.ident;

    // for each field, we need to generate an impl of `HasIngredientsFor`
    let has_ingredients_for_impls: Vec<_> = input
        .fields
        .iter()
        .zip(0..)
        .map(|(field, index)| has_ingredients_for_impl(jar_struct, field, index))
        .collect();

    let jar_impl = jar_impl(jar_struct, input);

    quote! {
        #output_struct

        #(#has_ingredients_for_impls)*

        #jar_impl
    }
}

pub(crate) fn has_ingredients_for_impl(
    jar_struct: &Ident,
    field: &Field,
    index: u32,
) -> proc_macro2::TokenStream {
    let field_ty = &field.ty;
    let index = Literal::u32_unsuffixed(index);
    quote! {
        impl salsa::storage::HasIngredientsFor<#field_ty> for #jar_struct {
            fn ingredient(&self) -> &<#field_ty as salsa::storage::IngredientsFor>::Ingredients {
                &self.#index
            }

            fn ingredient_mut(&mut self) -> &mut <#field_ty as salsa::storage::IngredientsFor>::Ingredients {
                &mut self.#index
            }
        }
    }
}

pub(crate) fn jar_impl(jar_struct: &Ident, input: &ItemStruct) -> proc_macro2::TokenStream {
    let field_initializations: Vec<_> = input
        .fields
        .iter()
        .enumerate()
        .map(|(i, f)| {
            let index: syn::Index = i.into();
            let field_ty = &f.ty;
            quote! {
                unsafe{
                    std::mem::transmute::<_, &mut std::mem::MaybeUninit<
                        <#field_ty as salsa::storage::IngredientsFor>::Ingredients
                    >>(&mut self.#index)
                    .write(
                        <#field_ty as salsa::storage::IngredientsFor>::create_ingredients(routes)
                    )
                };
            }
        })
        .collect();
    quote! {
        impl ::salsa::jar::HasJarIndex for #jar_struct {
            const JAR_INDEX: ::salsa::jar::JarIndex = ::salsa::jar::JarIndex::#jar_struct;
        }

        impl ::salsa::jar::Jar for #jar_struct {
            fn initialize(&mut self, routes: &mut salsa::routes::Routes)
            {
                #(#field_initializations)*
            }
        }
    }
    // ANCHOR_END: initialize
}

pub(crate) fn jar_struct(input: &ItemStruct) -> ItemStruct {
    let mut output_struct = input.clone();
    output_struct.fields = generate_fields(input).into();
    if output_struct.semi_token.is_none() {
        output_struct.semi_token = Some(Token![;](input.struct_token.span));
    }
    output_struct
}

fn generate_fields(input: &ItemStruct) -> FieldsUnnamed {
    // Generate the
    let mut output_fields = Punctuated::new();
    for field in input.fields.iter() {
        let mut field = field.clone();

        // Convert to anonymous fields
        field.ident = None;
        let field_ty = &field.ty;
        field.ty =
            syn::parse2(quote!(< #field_ty as salsa::storage::IngredientsFor >::Ingredients))
                .unwrap();
        output_fields.push(field);
    }

    let paren_token = match &input.fields {
        syn::Fields::Named(f) => syn::token::Paren {
            span: f.brace_token.span,
        },
        syn::Fields::Unnamed(f) => f.paren_token,
        syn::Fields::Unit => syn::token::Paren::default(),
    };

    FieldsUnnamed {
        paren_token,
        unnamed: output_fields,
    }
}
