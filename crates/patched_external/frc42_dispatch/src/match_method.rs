// Copyright 2022-2024 Protocol Labs
// SPDX-License-Identifier: MIT
//
// Forked from https://github.com/filecoin-project/actors-utils with assumed MIT license
// as per Cargo.toml: https://github.com/filecoin-project/actors-utils/blob/7628cd8d39dafcc6035f28e350cdb0cccbea5ab4/frc42_dispatch/Cargo.toml#L5
//
// License headers added post-fork.
#[macro_export]
macro_rules! match_method {
    ($method:expr, {$($body:tt)*}) => {
        match_method!{@match $method, {}, $($body)*}
    };
    (@match $method:expr, {$($body:tt)*}, $(,)*) => {
        match $method {
            $($body)*
        }
    };
    // matches block with comma
    (@match $method:expr, {$($body:tt)*}, $p:literal => $e:expr, $($tail:tt)*) => {
        match_method! {
            @match
            $method,
            {
                $($body)*
                $crate::method_hash!($p) => $e,
            },
            $($tail)*
        }
    };
    // matches block without comma
    (@match $method:expr, {$($body:tt)*}, $p:literal => $e:block $($tail:tt)*) => {
        match_method! {
            @match
            $method,
            {
                $($body)*
                $crate::method_hash!($p) => $e,
            },
            $($tail)*
        }
    };
    // matches _ with a trailing comma
    (@match $method:expr, {$($body:tt)*}, _ => $e:expr, $($tail:tt)*) => {
        match_method! {
            @match
            $method,
            {
                $($body)*
                _ => $e,
            },
            $($tail)*
        }
    };
    // matches _ without a trailing comma (common if it's the last item)
    (@match $method:expr, {$($body:tt)*}, _ => $e:expr) => {
        match_method! {
            @match
            $method,
            {
                $($body)*
                _ => $e,
            },
        }
    };
}

#[cfg(test)]
mod tests {
    #[test]
    fn handle_constructor() {
        let method_num = 1u64; // constructor should always hash to 1
        let ret = match_method!(method_num, {
            "Constructor" => Some(1),
            _ => None,
        });

        assert_eq!(ret, Some(1));
    }

    #[test]
    fn handle_unknown_method() {
        let method_num = 12345u64; // not a method we know about
        let ret = match_method!(method_num, {
            "Constructor" => Some(1),
            _ => None,
        });

        assert_eq!(ret, None);
    }

    #[test]
    fn handle_user_method() {
        let method_num = crate::method_hash!("TokensReceived");
        let ret = match_method!(method_num, {
            "Constructor" => Some(1),
            "TokensReceived" => Some(2),
            _ => None,
        });

        assert_eq!(ret, Some(2));
    }

    #[test]
    fn handle_optional_commas() {
        let method_num = crate::method_hash!("TokensReceived");
        let ret = match_method!(method_num, {
            "Constructor" => Some(1),
            "TokensReceived" => {
                Some(2)
            }
            _ => {
                None
            }
        });

        assert_eq!(ret, Some(2));
    }
}
