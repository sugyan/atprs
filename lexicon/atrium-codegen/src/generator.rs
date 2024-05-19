use crate::fs::find_dirs;
use crate::schema::find_ref_unions;
use crate::token_stream::{client, collection, modules, record_enum, ref_unions, user_type};
use atrium_lex::lexicon::LexUserType;
use atrium_lex::LexiconDoc;
use heck::ToSnakeCase;
use itertools::Itertools;
use proc_macro2::TokenStream;
use quote::quote;
use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::fs::{create_dir_all, read_dir, File};
use std::io::Write;
use std::path::{Path, PathBuf};

const HEADER: &str = "// This file is generated by atrium-codegen. DO NOT EDIT.";

pub(crate) fn generate_schemas(
    schema: &LexiconDoc,
    outdir: &Path,
) -> Result<Vec<PathBuf>, Box<dyn Error>> {
    let mut results = Vec::new();
    let mut paths = schema.id.split('.').collect::<Vec<_>>();
    if let Some(basename) = paths.pop() {
        let mut tokens = Vec::new();
        let mut names = Vec::new();
        for (name, def) in &schema.defs {
            // NSID (for XrpcSubscription only)
            if let LexUserType::XrpcSubscription(_) = &def {
                let nsid = schema.id.clone();
                tokens.push(quote! {
                    pub const NSID: &str = #nsid;
                });
            }
            // main def
            if name == "main" {
                tokens.push(user_type(def, basename, true)?);
            } else {
                names.push(name);
            }
        }
        // other defs
        for &name in names.iter().sorted() {
            tokens.push(user_type(&schema.defs[name], name, false)?);
        }
        // ref unions
        tokens.push(ref_unions(&schema.id, &find_ref_unions(&schema.defs))?);

        let documentation = {
            let doc = format!("Definitions for the `{}` namespace.", schema.id);
            let description = if let Some(description) = &schema.description {
                quote!(#![doc = #description])
            } else {
                quote!()
            };
            quote! {
                #![doc = #doc]
                #description
            }
        };
        let content = quote! {
            #documentation
            #(#tokens)*
        };
        let dir = outdir.join(paths.join("/"));
        create_dir_all(&dir)?;
        let mut filename = PathBuf::from(basename.to_snake_case());
        filename.set_extension("rs");
        let path = dir.join(filename);
        write_to_file(File::create(&path)?, content)?;
        results.push(path);
    }
    Ok(results)
}

pub(crate) fn generate_records(
    outdir: &Path,
    schemas: &[LexiconDoc],
    namespaces: &[(&str, Option<&str>)],
) -> Result<PathBuf, Box<dyn Error>> {
    let records = schemas
        .iter()
        .filter_map(|schema| {
            if let Some(LexUserType::Record(_)) = schema.defs.get("main") {
                Some(schema.id.clone())
            } else {
                None
            }
        })
        .sorted()
        .collect_vec();
    let tokens = record_enum(&records, "KnownRecord", None, namespaces)?;
    let content = quote! {
        #![doc = "A collection of ATP repository record types."]
        #[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq)]
        #[serde(untagged)]
        pub enum Record {
            Known(KnownRecord),
            Unknown(crate::types::UnknownData),
        }
        #tokens
    };
    let path = outdir.join("records.rs");
    write_to_file(File::create(&path)?, content)?;
    Ok(path)
}

pub(crate) fn generate_client(
    outdir: &Path,
    schemas: &[LexiconDoc],
    namespaces: &[(&str, Option<&str>)],
) -> Result<PathBuf, Box<dyn Error>> {
    let mut schema_map = HashMap::new();
    let mut tree = HashMap::new();
    for schema in schemas {
        if let Some(def) = schema.defs.get("main") {
            if matches!(
                def,
                LexUserType::XrpcQuery(_) | LexUserType::XrpcProcedure(_)
            ) {
                schema_map.insert(schema.id.clone(), def);
                let mut parts = schema.id.split('.').collect_vec();
                let mut is_leaf = true;
                while let Some(part) = parts.pop() {
                    let key = parts.join(".");
                    tree.entry(key)
                        .or_insert_with(HashSet::new)
                        .insert((part, is_leaf));
                    is_leaf = false;
                }
            }
        }
    }
    let tokens = client(&tree, &schema_map, namespaces)?;
    let content = quote! {
        #![doc = r#"Structs for ATP client, implements all HTTP APIs of XRPC."#]
        #tokens
    };
    let path = outdir.join("client.rs");
    write_to_file(File::create(&path)?, content)?;
    Ok(path)
}

pub(crate) fn generate_modules(
    outdir: &Path,
    schemas: &[LexiconDoc],
    namespaces: &[(&str, Option<&str>)],
) -> Result<Vec<PathBuf>, Box<dyn Error>> {
    let mut paths = find_dirs(outdir)?;
    paths.reverse();
    paths.retain(|p| {
        p.as_ref() != outdir
            && p.as_ref()
                .strip_prefix(outdir)
                .map_or(true, |p| !p.starts_with("agent") && !p.starts_with("types"))
    });
    let mut files = Vec::with_capacity(paths.len());
    // generate ".rs" files names
    for path in &paths {
        let mut p = path.as_ref().to_path_buf();
        p.set_extension("rs");
        files.push(p);
    }
    // write "mod" statements
    for (path, filepath) in paths.iter().zip(&files) {
        let names = read_dir(path)?
            .filter_map(Result::ok)
            .filter(|entry| entry.path().is_file())
            .filter_map(|entry| {
                entry
                    .path()
                    .file_stem()
                    .map(|s| s.to_string_lossy().into_owned())
            })
            .sorted()
            .collect_vec();
        let relative = path.as_ref().strip_prefix(outdir)?;
        let modules = modules(
            &names,
            &relative
                .components()
                .filter_map(|c| c.as_os_str().to_str())
                .collect_vec(),
            namespaces,
        )?;
        let (documentation, collections) = {
            let ns = relative.to_string_lossy().replace(['/', '\\'], ".");
            let doc = format!("Definitions for the `{}` namespace.", ns);
            let collections = names
                .iter()
                .filter_map(|name| {
                    let nsid = format!("{}.{}", ns, name);
                    schemas
                        .iter()
                        .find(|schema| {
                            schema
                                .defs
                                .get("main")
                                .map(|def| {
                                    schema.id == nsid && matches!(def, LexUserType::Record(_))
                                })
                                .unwrap_or(false)
                        })
                        .map(|_| collection(name, &nsid))
                })
                .collect_vec();
            (quote!(#![doc = #doc]), collections)
        };
        let content = quote! {
            #documentation
            #modules
            #(#collections)*
        };
        write_to_file(File::create(filepath)?, content)?;
    }
    Ok(files)
}

fn write_to_file(mut file: impl Write, content: TokenStream) -> Result<(), Box<dyn Error>> {
    let parsed = syn::parse_file(&content.to_string())?;
    writeln!(file, "{HEADER}")?;
    write!(file, "{}", prettyplease::unparse(&parsed))?;
    Ok(())
}
