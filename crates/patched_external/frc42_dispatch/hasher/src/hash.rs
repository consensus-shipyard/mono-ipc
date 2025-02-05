// Copyright 2022-2024 Protocol Labs
// SPDX-License-Identifier: MIT
//
// Forked from https://github.com/filecoin-project/actors-utils with assumed MIT license
// as per Cargo.toml: https://github.com/filecoin-project/actors-utils/blob/7628cd8d39dafcc6035f28e350cdb0cccbea5ab4/frc42_dispatch/Cargo.toml#L5
//
// License headers added post-fork.
use thiserror::Error;

/// Minimal interface for a hashing function.
///
/// [`Hasher::hash()`] must return a digest that is at least 4 bytes long so that it can be cast to
/// a [`u32`].
pub trait Hasher {
    /// For an input of bytes return a digest that is at least 4 bytes long.
    fn hash(&self, bytes: &[u8]) -> Vec<u8>;
}

/// Hasher that uses the blake2b hash syscall provided by the FVM.
#[cfg(feature = "use_sdk")]
#[derive(Default)]
pub struct Blake2bSyscall {}

#[cfg(feature = "use_sdk")]
impl Hasher for Blake2bSyscall {
    // fvm_sdk dependence can be removed by setting default-features to false
    fn hash(&self, bytes: &[u8]) -> Vec<u8> {
        use fvm_shared::crypto::hash::SupportedHashes;
        fvm_sdk::crypto::hash_owned(SupportedHashes::Blake2b512, bytes)
    }
}

/// Uses an underlying hashing function (blake2b by convention) to generate method numbers from
/// method names.
#[derive(Default)]
pub struct MethodResolver<T: Hasher> {
    hasher: T,
}

#[derive(Error, PartialEq, Eq, Debug)]
pub enum MethodNameErr {
    #[error("empty method name provided")]
    EmptyString,
    #[error("method name does not conform to the FRC-0042 convention {0}")]
    IllegalName(#[from] IllegalNameErr),
    #[error("unable to calculate method id, choose a another method name")]
    IndeterminableId,
}

#[derive(Error, PartialEq, Eq, Debug)]
pub enum IllegalNameErr {
    #[error("method name doesn't start with capital letter or _")]
    NotValidStart,
    #[error("method name contains letters outside [a-zA-Z0-9_]")]
    IllegalCharacters,
}

impl<T: Hasher> MethodResolver<T> {
    const CONSTRUCTOR_METHOD_NAME: &'static str = "Constructor";
    const CONSTRUCTOR_METHOD_NUMBER: u64 = 1_u64;
    const FIRST_METHOD_NUMBER: u64 = 1 << 24;
    const DIGEST_CHUNK_LENGTH: usize = 4;

    /// Creates a [`MethodResolver`] with an instance of a hasher (blake2b by convention).
    pub fn new(hasher: T) -> Self {
        Self { hasher }
    }

    /// Generates a standard FRC-0042 compliant method number.
    ///
    /// The method number is calculated as the first four bytes of `hash(method-name)`.
    /// The name `Constructor` is always hashed to 1 and other method names that hash to
    /// 0 or 1 are avoided via rejection sampling.
    pub fn method_number(&self, method_name: &str) -> Result<u64, MethodNameErr> {
        check_method_name(method_name)?;

        if method_name == Self::CONSTRUCTOR_METHOD_NAME {
            return Ok(Self::CONSTRUCTOR_METHOD_NUMBER);
        }

        let method_name = format!("1|{method_name}");
        let digest = self.hasher.hash(method_name.as_bytes());

        for chunk in digest.chunks(Self::DIGEST_CHUNK_LENGTH) {
            if chunk.len() < Self::DIGEST_CHUNK_LENGTH {
                // last chunk may be smaller than 4 bytes
                break;
            }

            let method_id = as_u32(chunk) as u64;
            // Method numbers below FIRST_METHOD_NUMBER are reserved for other use
            if method_id >= Self::FIRST_METHOD_NUMBER {
                return Ok(method_id);
            }
        }

        Err(MethodNameErr::IndeterminableId)
    }
}

/// Checks that a method name is valid and compliant with the FRC-0042 standard recommendations.
///
/// - Only ASCII characters in `[a-zA-Z0-9_]` are allowed.
/// - Starts with a character in `[A-Z_]`.
fn check_method_name(method_name: &str) -> Result<(), MethodNameErr> {
    if method_name.is_empty() {
        return Err(MethodNameErr::EmptyString);
    }

    // Check starts with capital letter
    let first_letter = method_name.chars().next().unwrap(); // safe because we checked for empty string
    if !(first_letter.is_ascii_uppercase() || first_letter == '_') {
        return Err(IllegalNameErr::NotValidStart.into());
    }

    // Check that all characters are legal
    if !method_name
        .chars()
        .all(|c| c.is_ascii_alphanumeric() || c == '_')
    {
        return Err(IllegalNameErr::IllegalCharacters.into());
    }

    Ok(())
}

/// Takes a byte array and interprets it as a u32 number.
///
/// Using big-endian order interperets the first four bytes to an int.
///
/// The slice passed to this must be at least length 4.
fn as_u32(bytes: &[u8]) -> u32 {
    u32::from_be_bytes(
        bytes[0..4]
            .try_into()
            .expect("bytes was not at least length 4"),
    )
}

#[cfg(test)]
mod tests {

    use super::{Hasher, IllegalNameErr, MethodNameErr, MethodResolver};

    #[derive(Clone, Copy)]
    struct FakeHasher {}
    impl Hasher for FakeHasher {
        fn hash(&self, bytes: &[u8]) -> Vec<u8> {
            bytes.to_vec()
        }
    }

    #[test]
    fn constructor_is_1() {
        let method_hasher = MethodResolver::new(FakeHasher {});
        assert_eq!(method_hasher.method_number("Constructor").unwrap(), 1);
    }

    #[test]
    fn normal_method_is_hashed() {
        let fake_hasher = FakeHasher {};
        let method_hasher = MethodResolver::new(fake_hasher);
        // note that the method hashing prepends each name with "1|" as a domain separator
        assert_eq!(
            method_hasher.method_number("NormalMethod").unwrap(),
            super::as_u32(&fake_hasher.hash(b"1|NormalMethod")) as u64
        );

        assert_eq!(
            method_hasher.method_number("NormalMethod2").unwrap(),
            super::as_u32(&fake_hasher.hash(b"1|NormalMethod2")) as u64
        );
    }

    #[test]
    fn disallows_invalid_method_names() {
        let method_hasher = MethodResolver::new(FakeHasher {});
        assert_eq!(
            method_hasher.method_number("Invalid|Method").unwrap_err(),
            MethodNameErr::IllegalName(IllegalNameErr::IllegalCharacters)
        );
        assert_eq!(
            method_hasher.method_number("").unwrap_err(),
            MethodNameErr::EmptyString
        );
        assert_eq!(
            method_hasher.method_number("invalidMethod").unwrap_err(),
            MethodNameErr::IllegalName(IllegalNameErr::NotValidStart)
        );
    }

    /// Fake hasher that always returns a digest beginning with b"\0\0\0\0".
    #[derive(Clone, Copy)]
    struct FakeHasher0 {}
    impl Hasher for FakeHasher0 {
        fn hash(&self, bytes: &[u8]) -> Vec<u8> {
            let mut hash: Vec<u8> = vec![0, 0, 0, 0];
            let mut suffix = bytes.to_vec();
            hash.append(suffix.as_mut());
            hash
        }
    }

    /// Fake hasher that always returns a digest beginning with b"\0\0\0\1".
    #[derive(Clone, Copy)]
    struct FakeHasher1 {}
    impl Hasher for FakeHasher1 {
        fn hash(&self, bytes: &[u8]) -> Vec<u8> {
            let mut hash: Vec<u8> = vec![0, 0, 0, 1];
            let mut suffix = bytes.to_vec();
            hash.append(suffix.as_mut());
            hash
        }
    }

    #[test]
    fn avoids_disallowed_method_numbers() {
        let hasher_0 = FakeHasher0 {};
        let method_hasher_0 = MethodResolver::new(hasher_0);

        // This simulates a method name that would hash to 0
        let contrived_0 = "MethodName";
        let contrived_0_digest = hasher_0.hash(contrived_0.as_bytes());
        assert_eq!(super::as_u32(&contrived_0_digest), 0);
        // But the method number is not a collision
        assert_ne!(method_hasher_0.method_number(contrived_0).unwrap(), 0);

        let hasher_1 = FakeHasher1 {};
        let method_hasher_1 = MethodResolver::new(hasher_1);
        // This simulates a method name that would hash to 1
        let contrived_1 = "MethodName";
        let contrived_1_digest = hasher_1.hash(contrived_1.as_bytes());
        assert_eq!(super::as_u32(&contrived_1_digest), 1);
        // But the method number is not a collision
        assert_ne!(method_hasher_1.method_number(contrived_1).unwrap(), 1);
    }
}
