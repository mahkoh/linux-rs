use proc_macro2::TokenStream;
use quote::{quote, quote_spanned};
use syn::{parse_macro_input, DeriveInput, Data, Fields, spanned::Spanned};

pub fn derive(input: proc_macro::TokenStream, internal: bool) -> proc_macro::TokenStream {
    let ast: DeriveInput = parse_macro_input!(input as DeriveInput);

    let name = ast.ident;

    let (impl_generics, type_generics, where_clause) = ast.generics.split_for_impl();
    let asserts = get_asserts(&ast.data);
    let pod = if internal { quote!(crate::util::pod::Pod) } else { quote!(linux::Pod) };

    let tokens = quote! {
        unsafe impl #impl_generics #pod for #name #type_generics #where_clause {
            fn _assert_pod() {
                fn assert<T: crate::util::pod::Pod>() { }
                #asserts
            }
        }
    };

    proc_macro::TokenStream::from(tokens)
}

fn get_asserts(data: &Data) -> TokenStream {
    let fields = match data {
        Data::Struct(ref data) => {
            match data.fields {
                Fields::Unnamed(ref fields) => &fields.unnamed,
                Fields::Named(ref fields) => &fields.named,
                Fields::Unit => return TokenStream::new(),
            }
        },
        Data::Union(ref data) => &data.fields.named,
        _ => unimplemented!(),
    };
    let recurse = fields.iter().map(|f| {
        let ty = &f.ty;
        quote_spanned! {
            ty.span() => assert::<#ty>();
        }
    });
    quote!(#(#recurse)*)
}