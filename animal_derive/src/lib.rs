use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

/// Derive Macro to add traits to Animals
///
/// # Example
///
/// ```
///     #[derive(Animal, Debug)]
///     struct SkunkApe {
///         color: String,
///         weight: i32
///     }
/// ```
#[proc_macro_derive(Animal)]
pub fn animal_derive(input: TokenStream) -> TokenStream {
    // Construct a representation of Rust code as a syntax tree
    // that we can manipulate
    let ast = parse_macro_input!(input as DeriveInput);

    // get the identifier of the code input
    let name = &ast.ident;
    // create code at runtime
    let expanded = quote! {
        impl #name {
            fn new(color: &str, weight: i32) -> Self {
                #name {
                    color: String::from(color),
                    weight,
                }
            }
        }

        #[typetag::serde]
        impl Animal for #name {}

        impl HasWeight for #name {
            fn weight(&self) -> i32 {
                self.weight
            }
        }

        impl HasColor for #name {
            fn color(&self) -> &str {
                &self.color
            }
        }
    };
    // Build the trait implementation
    expanded.into()
}
