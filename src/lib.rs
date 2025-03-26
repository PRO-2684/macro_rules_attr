#[cfg(feature = "log")]
use log::debug;
use proc_macro::{Delimiter, Group, Punct, Spacing, TokenStream, TokenTree};

/// Emit a compile error with the given message
macro_rules! error {
    ($msg:expr) => {{
        let ts: ::proc_macro::TokenStream = ::core::stringify! {
            ::core::compile_error!($msg)
        }
        .parse()
        .unwrap();
        ts
    }};
}

// apply: apply(my_macro), apply(my_macro, additional_args)
#[proc_macro_attribute]
pub fn apply(attrs: TokenStream, input: TokenStream) -> TokenStream {
    // Parse `the_macro` and `macro_append` from `attrs`
    let mut tts = attrs.into_iter();

    let macro_name = match tts.next() {
        Some(tt) => tt,
        None => return error!("Expected at least 1 token tree"),
    };
    #[cfg(feature = "log")]
    debug!("macro_name: {:?}", macro_name);

    let macro_append = tts.collect();
    #[cfg(feature = "log")]
    debug!("macro_append: {:?}", macro_append);

    // Call `macro_rules_attr_impl`
    macro_rules_attr_impl(macro_name.into(), macro_append, input)
}

fn macro_rules_attr_impl(
    macro_name: TokenStream,
    macro_append: TokenStream,
    input: TokenStream,
) -> TokenStream {
    // Adapted from https://github.com/danielhenrymantilla/macro_rules_attribute-rs/blob/fa6f120939c7757dec23589e2687d5e8480fa1ce/src/proc_macro/mod.rs#L27-L53

    let mut ret = macro_name;
    let mut macro_input = input;
    ret.extend([TokenTree::Punct(Punct::new('!', Spacing::Alone))]);

    // Append `macro_append` to `macro_input` (With delimiter `,`)
    if !macro_append.is_empty() {
        // Append `macro_append`
        macro_input.extend(macro_append);
    }

    // Wrap with `{}`
    ret.extend([TokenTree::Group(Group::new(Delimiter::Brace, macro_input))]);

    #[cfg(feature = "log")]
    debug!("ret: {ret:?}");
    ret
}
