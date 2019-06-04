extern crate proc_macro;

mod pod;
mod kstr;

use proc_macro::{TokenStream};

macro_rules! pod {
    ($fn:ident, $name:ident, $internal:expr) => {
        #[proc_macro_derive($name)]
        pub fn $fn(input: TokenStream) -> TokenStream {
            pod::derive(input, $internal)
        }
    }
}

pod!(pod, Pod, true);
pod!(pod_pub, PodPub, false);

macro_rules! kstr {
    ($name:ident, $internal:expr) => {
        #[proc_macro]
        pub fn $name(input: TokenStream) -> TokenStream {
            kstr::transform(input, $internal)
        }
    }
}

kstr!(kstr, true);
kstr!(kstr_pub, false);
