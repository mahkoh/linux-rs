use proc_macro::{TokenStream};
use quote::{quote};
use syn::{parse_macro_input, LitStr, Error};

pub fn transform(input: TokenStream, internal: bool) -> TokenStream {
    let input: LitStr = parse_macro_input!(input as LitStr);
    let mut val = input.value();
    if val.contains("\0") {
        return TokenStream::from(
            Error::new(input.span(), "Literal contains a 0 byte").to_compile_error()
        );
    }
    val.push('\0');
    let crat = if internal { quote!(crate) } else { quote!(linux) };
    let tokens = quote! {
        unsafe {
            static S: &str = #val;
            #crat::util::kstr::KStr::new(S.as_ptr() as *const _)
        }
    };
    TokenStream::from(tokens)
}