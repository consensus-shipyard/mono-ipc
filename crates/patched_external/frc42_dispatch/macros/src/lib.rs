// Copyright 2022-2024 Protocol Labs
// SPDX-License-Identifier: MIT
//
// Forked from https://github.com/filecoin-project/actors-utils with assumed MIT license
// as per Cargo.toml: https://github.com/filecoin-project/actors-utils/blob/7628cd8d39dafcc6035f28e350cdb0cccbea5ab4/frc42_dispatch/Cargo.toml#L5
//
// License headers added post-fork.
use frc42_hasher::hash::MethodResolver;
use proc_macro::TokenStream;
use quote::quote;
use syn::parse::{Parse, ParseStream};
use syn::{parse_macro_input, LitStr, Result};

mod hash;
use crate::hash::Blake2bHasher;

struct MethodName(LitStr);

impl MethodName {
    /// Hash the method name.
    fn hash(&self) -> u64 {
        let resolver = MethodResolver::new(Blake2bHasher {});
        resolver.method_number(&self.0.value()).unwrap()
    }
}

impl Parse for MethodName {
    fn parse(input: ParseStream) -> Result<Self> {
        let lookahead = input.lookahead1();

        if lookahead.peek(LitStr) {
            input.parse().map(MethodName)
        } else {
            Err(lookahead.error())
        }
    }
}

#[proc_macro]
pub fn method_hash(input: TokenStream) -> TokenStream {
    let name: MethodName = parse_macro_input!(input);
    let hash = name.hash();
    quote!(#hash).into()
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let t = trybuild::TestCases::new();
        t.pass("tests/build-success.rs");
    }

    #[test]
    fn empty_names() {
        let t = trybuild::TestCases::new();
        // NOTE: these need to live in a separate directory under `tests`
        // otherwise cargo tries to build them every time and everything breaks
        t.compile_fail("tests/naming/empty-name-string.rs");
        t.compile_fail("tests/naming/missing-name.rs");
    }

    #[test]
    fn bad_names() {
        let t = trybuild::TestCases::new();
        t.compile_fail("tests/naming/illegal-chars.rs");
        t.compile_fail("tests/naming/non-capital-start.rs");
    }
}
