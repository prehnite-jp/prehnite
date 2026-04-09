use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(CreateRecord)]
pub fn derive_c(input: TokenStream) -> TokenStream {
    let _ = parse_macro_input!(input as DeriveInput);
    unimplemented!()
}

#[proc_macro_derive(ReadRecord)]
pub fn derive_r(input: TokenStream) -> TokenStream {
    let _ = parse_macro_input!(input as DeriveInput);
    unimplemented!()
}

#[proc_macro_derive(UpdateRecord)]
pub fn derive_u(input: TokenStream) -> TokenStream {
    let _ = parse_macro_input!(input as DeriveInput);
    unimplemented!()
}

#[proc_macro_derive(DeleteRecord)]
pub fn derive_d(input: TokenStream) -> TokenStream {
    let _ = parse_macro_input!(input as DeriveInput);
    unimplemented!()
}
