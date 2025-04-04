//! # Clowncopterize
//! `clowncopterize` is a tool to make setting all `clowntown` cli arguments easier.
//!
//! ## Background
//! The `clowntown` command line argument is a well-known reliability feature that allows to hide risky features behind a flag. As reliability
//! is taken more and more seriously, the sprawling of `clowntown` flags is becoming an issue for our savvy engineer who are ending up writing
//! command lines that goes beyond our cherrished 80-char limits.
//!
//! To make people's life easier, here comes `clowncopterize`! With a single line added to your program, you can achieve the apex of reliability
//! by providing the almighty `--clowncopterize` argument which will set all your `--clowntown-X` flags to true.
//!
//! ## Requirements
//! `clowncopterize` is an attribute macro to apply to a `clap` struct to make it easier to set all those --clowntown-X flags with a single `--clowncopterize` flag.
//!
//! ## Usage
//! Wrap you clap Parser struct with
//! ```ignore
//! #[clowncopterize::clowncopterize]
//! ```
//!
//! This macro must be above the derive one.
//!
//! ### Example
//! ```ignore
//! #[clowncopterize::clowncopterize]
//! #[derive(Parser, Debug)]
//! struct Cli {
//!     /// Optional name to operate on
//!     name: Option<String>,
//!
//!     /// Turn debugging information on
//!     #[arg(long)]
//!     clowntown_this: bool,
//!
//!     /// lists test values
//!     #[arg(long)]
//!     clowntown_that: bool,
//! }
//! ```
//!

use proc_macro::TokenStream;
use proc_macro2::{Ident, Span};
use quote::ToTokens;

const CLOWNCOPTERIZE_PREFIX: &str = "clowntown";
const CLOWNCOPTERIZE_FLAG: &str = "clowncopterize";

// used to parse Named Fields from a TokenStream so we can inject our ew --clowncopterize parameter
// https://github.com/dtolnay/syn/issues/651#issuecomment-503771863
struct ParsableNamedField {
    pub field: syn::Field,
}

impl syn::parse::Parse for ParsableNamedField {
    fn parse(input: syn::parse::ParseStream<'_>) -> syn::parse::Result<Self> {
        let field = syn::Field::parse_named(input)?;

        Ok(ParsableNamedField { field })
    }
}

/// Clowncopterize the underlying clap parser
///
/// # Example
///
/// ```
/// use clap::Parser;
///
/// // by default, uses `--clowncopterize` flag:
///
/// #[clowncopterize::clowncopterize]
/// #[derive(Parser, Debug)]
/// struct Cli {
///     /// Optional name to operate on
///     name: Option<String>,
///
///     /// Turn debugging information on
///     #[arg(long)]
///     clowntown_this: bool,
///
///     /// lists test values
///     #[arg(long)]
///     clowntown_that: bool,
/// }
///
///
/// let cli = Cli::try_parse_from(vec!["prog", "--clowncopterize"]).unwrap();
///
/// println!("Cli! {:#?}", cli);
/// assert!(cli.clowntown_this);
///
/// // but can be customized with the `clowncopterizer` attribute:
///
/// #[clowncopterize::clowncopterize(clowncopterizer = "i-live-in-clowntown")]
/// #[derive(Parser, Debug)]
/// struct CliCustom {
///     /// Optional name to operate on
///     name: Option<String>,
///
///     /// Turn debugging information on
///     #[arg(long)]
///     clowntown_this: bool,
///
///     /// lists test values
///     #[arg(long)]
///     clowntown_that: bool,
/// }
///
///
/// let cli = CliCustom::try_parse_from(vec!["prog", "--i-live-in-clowntown"]).unwrap();
///
/// println!("Cli! {:#?}", cli);
/// assert!(cli.clowntown_this);
/// ```
#[proc_macro_attribute]
pub fn clowncopterize(attr: TokenStream, item: TokenStream) -> TokenStream {
    let clowncopterizer = syn::parse_macro_input!(attr as Clowncopterize);

    let item_struct: syn::ItemStruct = syn::parse_macro_input!(item);
    let out = clowncopterizer.clowncopterize_struct(item_struct);
    proc_macro::TokenStream::from(out.to_token_stream())
}

#[derive(Debug)]
struct Clowncopterize {
    clowncopterizer: String,
}

impl Default for Clowncopterize {
    fn default() -> Self {
        Clowncopterize {
            clowncopterizer: CLOWNCOPTERIZE_FLAG.to_string(),
        }
    }
}

impl syn::parse::Parse for Clowncopterize {
    fn parse(input: syn::parse::ParseStream) -> syn::parse::Result<Self> {
        if input.is_empty() {
            return Ok(Clowncopterize {
                ..Default::default()
            });
        }
        let attr_name = input.parse::<syn::Ident>()?;
        assert_eq!(
            attr_name, "clowncopterizer",
            "Unexpected attribute {}",
            attr_name
        );
        input.parse::<syn::Token![=]>()?;
        let attr_value = input.parse::<syn::LitStr>()?;

        Ok(Clowncopterize {
            clowncopterizer: attr_value.value().replace("-", "_"),
        })
    }
}

impl Clowncopterize {
    fn clowncopterize_struct(&self, mut ast: syn::ItemStruct) -> syn::ItemStruct {
        let mut is_clown = false;
        if let syn::Fields::Named(ref mut fields) = ast.fields {
            // iterate over each fields and modify any fields that start with `clowntown` and is a boolean.
            fields.named =
                syn::punctuated::Punctuated::from_iter(fields.named.iter_mut().map(|field| {
                    if let syn::Type::Path(type_path) = &field.ty {
                        if type_path.path.is_ident("bool") {
                            if let Some(ref ident) = field.ident {
                                if ident.to_string().starts_with(CLOWNCOPTERIZE_PREFIX) {
                                    is_clown = true;
                                    return self.clowncopterize_field(field);
                                }
                            }
                        }
                    }
                    field.clone()
                }));
            // There is at least 1 clowntown flag, add our clowncopterize flag.
            if is_clown {
                let clowncopterizer = Ident::new(&self.clowncopterizer, Span::call_site());
                let punctuated_fields: syn::punctuated::Punctuated<
                    ParsableNamedField,
                    syn::Token![,],
                > = syn::parse_quote! {
                    /// Turns all the clowntown flags on
                    #[arg(long)]
                    #clowncopterizer: bool
                };
                for punctuated_field in punctuated_fields {
                    fields.named.push(punctuated_field.field);
                }
            }
        }
        ast
    }

    fn clowncopterize_field(&self, ast: &mut syn::Field) -> syn::Field {
        for attr in ast.attrs.iter_mut() {
            if attr.path().is_ident("arg") {
                let meta = attr.meta.require_list().unwrap();
                let mut tokens = meta.tokens.clone();
                let clowncopterizer = &self.clowncopterizer;
                let ext = quote::quote! {
                    , default_value_if(#clowncopterizer, "true", "true")
                };
                tokens.extend(ext);
                attr.meta = syn::Meta::List(syn::MetaList {
                    path: meta.path.clone(),
                    delimiter: meta.delimiter.clone(),
                    tokens,
                });
            }
        }
        ast.clone()
    }
}
