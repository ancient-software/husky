use std::collections::{BTreeMap, BTreeSet};

use syn::Token;

pub(crate) fn db(
    args: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let args = syn::parse_macro_input!(args as Args);
    let input = syn::parse_macro_input!(input as syn::ItemStruct);
    let ident = &input.ident;
    let vis = &input.vis;
    check_jar_paths(args.jar_paths.iter());
    let initialization: proc_macro2::TokenStream = initialization(args);

    quote! {
        #vis struct #ident(::salsa::Db);

        impl std::ops::Deref for #ident {
            type Target = ::salsa::Db;

            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }

        impl std::ops::DerefMut for #ident {
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.0
            }
        }

        impl Default for #ident {
            fn default() -> Self {
                Self(::salsa::Db::new(|jars, routes| {
                    *jars = ::salsa::jar::Jars::default();
                    #initialization
                }))
            }
        }

        impl ::salsa::snapshot::SnapshotClone for #ident {
            fn snapshot_clone(&self) -> Self {
                Self(self.0.snapshot_clone())
            }
        }
    }
    .into()
}

fn initialization(args: Args) -> proc_macro2::TokenStream {
    if !args.jar_paths.is_empty() {
        args.jar_paths
            .iter()
            .map(|jar_path| {
                quote! {
                    jars.initialize_jar::<#jar_path>(routes);
                }
            })
            .collect()
    } else {
        todo!()
    }
}

fn check_jar_paths<'a>(jar_paths: impl Iterator<Item = &'a syn::Path>) {
    let jar_idents: Vec<String> = jar_paths
        .map(|jar_path| jar_path.segments.last().unwrap().ident.to_string())
        .collect();
    let jar_tree = decode_jar_tree();
    for jar_ident in &jar_idents {
        use convert_case::Case;
        use convert_case::Casing;

        assert!(jar_ident.ends_with("Jar"));
        if jar_ident == "Jar" {
            continue;
        }
        let jar_package_name = format!(
            "husky-{}",
            jar_ident.split("Jar").next().unwrap().to_case(Case::Kebab)
        )
        .replace("-type", "-ty");
        let Some(deps) = jar_tree.get(&jar_package_name) else {
            panic!("{jar_package_name} is not present in jar tree");
        };
        for dep in deps {
            let kebab = dep.split("husky-").last().unwrap();
            let dep_jar_ident = format!("{}Jar", kebab.to_case(Case::Pascal)).replace("Ty", "Type");
            assert!(
                jar_idents.contains(&dep_jar_ident),
                "expect `{dep_jar_ident}` to be included as a dependency for `{jar_ident}`"
            )
        }
    }

    fn decode_jar_tree() -> BTreeMap<String, BTreeSet<String>> {
        let cargo_manifest_dir: std::path::PathBuf =
            std::env::var("CARGO_MANIFEST_DIR").unwrap().into();
        let path = cargo_manifest_dir
            .join("../../utils/husky-jar-utils/expect-files/husky_lang_jar_tree.json");
        assert!(path.exists());
        let text = std::fs::read_to_string(path).unwrap();
        serde_json::from_str(&text).unwrap()
    }

    fn jar_deps(jar_ident: &str) -> &[&str] {
        // todo: update this list
        match jar_ident {
            "Jar" => &[],
            // comptime
            "RustTranspilationJar" => &[],
            // devtime
            "TraceJar" => &["TextJar"],
            // fs
            "CorgiConfigJar" => &[],
            "ManifestJar" => &[],
            "ToolchainConfigJar" => &[],
            "VfsJar" => &[],
            // hir
            "HirDeclJar" => &[],
            "HirDefnJar" => &[],
            "HirEagerExprJar" => &["TextJar"],
            "HirExprJar" => &[],
            "HirLazyExprJar" => &[],
            "HirPreludeJar" => &[],
            "HirTypeJar" => &[],
            // ide
            "CompletionJar" => &[],
            "DiagnosticsJar" => &[],
            "DocumentationJar" => &[],
            "FoldingRangeJar" => &[],
            "HoverJar" => &[],
            "SemanticTokenJar" => &[],
            "IdeFmtJar" => &[],
            "TokenInfoJar" => &[],
            // kernel
            "CowordJar" => &[],
            "DecSignatureJar" => &[],
            "DecTermJar" => &[],
            "DecTypeJar" => &[],
            "EntityPathJar" => &[],
            "EthSignatureJar" => &[],
            "EthTermJar" => &["DecTermJar"],
            "FlyTermJar" => &["EthTermJar"],
            "TermPreludeJar" => &[],
            "PlaceJar" => &[],
            // lex
            "TextJar" => &[],
            "TokenDataJar" => &[],
            "TokenJar" => &["TokenDataJar"],
            "TomlTokenJar" => &[],
            // linkage
            "JavelinJar" => &[],
            "LinkageJar" => &[],
            // semantics
            "SemExprJar" => &["TextJar"],
            "SemDepsJar" => &["SemExprJar"],
            "SemExprDepsJar" => &["SemDepsJar"],
            "SemVarDepsJar" => &["SemExprDepsJar"],
            "SemPlaceContractJar" => &[],
            // super
            "SuperNodeJar" => &[],
            // syntax
            "AstJar" => &["TokenJar"],
            "EntityTreeJar" => &["AstJar"],
            "ManifestAstJar" => &[],
            "SynDeclJar" => &[],
            "SynDefnJar" => &[],
            "SynExprJar" => &[],
            "TomlAstJar" => &[],
            "CorgiConfigAstJar" => &[],
            // val
            "KiJar" => &[],
            "KiReprJar" => &["KiJar"],
            // vm
            "VmirJar" => &[],
            other => panic!("unknown jar ident `{other}`"),
        }
    }
}

pub struct Args {
    jar_paths: syn::punctuated::Punctuated<syn::Path, Token![,]>,
}

impl syn::parse::Parse for Args {
    fn parse(input: syn::parse::ParseStream<'_>) -> syn::Result<Self> {
        Ok(Self {
            jar_paths: syn::punctuated::Punctuated::parse_terminated(input)?,
        })
    }
}

#[test]
fn jar_deps_works() {
    // let export_info = husky_cargo_utils::metadata::output_metadata(&dev_paths.dev_root().join("Cargo.toml"));
    // let packages=export_info.
}
