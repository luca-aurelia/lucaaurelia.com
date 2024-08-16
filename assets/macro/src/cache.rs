use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use serde::{de::DeserializeOwned, Serialize};
use std::fs;
use syn::{parse_str, Item};

pub fn cache_macro_output<OutputStruct: Serialize + DeserializeOwned + ToTokens>(
    macro_name: &str,
    need_to_run_macro: bool,
    run_macro: impl FnOnce() -> Result<OutputStruct, TokenStream>,
) -> TokenStream {
    match get_cached_output::<OutputStruct>(macro_name, need_to_run_macro) {
        Some(cached_output) => cached_output,
        None => match run_macro() {
            Ok(output) => {
                let cached_output_path = paths::cached_macro_output_path(macro_name);

                if let Some(parent) = cached_output_path.parent() {
                    fs::create_dir_all(parent).expect("Error creating cached output directory.");
                }

                let serialized = serde_json::to_string(&output).expect("Error serializing output.");
                fs::write(cached_output_path, serialized).expect("Error writing cached output.");

                let tokens = quote! {
                    #output
                };

                tokens.into()
            }
            Err(error) => error,
        },
    }
}

fn get_cached_output<OutputStruct: Serialize + DeserializeOwned + ToTokens>(
    macro_name: &str,
    need_to_run_macro: bool,
) -> Option<TokenStream> {
    if need_to_run_macro {
        return None;
    }

    let cached_output_path = paths::cached_macro_output_path(macro_name);
    let serialized = match std::fs::read_to_string(cached_output_path) {
        Ok(serialized) => serialized,
        Err(err) => {
            eprintln!("Error reading cached output: {:?}", err);
            return None;
        }
    };

    let deserialized = match serde_json::from_str::<OutputStruct>(&serialized) {
        Ok(deserialized) => deserialized,
        Err(err) => {
            eprintln!("Error deserializing cached output: {:?}", err);
            return None;
        }
    };

    let tokens = quote! {
        #deserialized
    };

    Some(tokens.into())
}
