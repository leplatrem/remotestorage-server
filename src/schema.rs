#![allow(proc_macro_derive_resolution_fallback)]

table! {
    documents (name) {
        name -> Text,
        folder -> Text,
    }
}
