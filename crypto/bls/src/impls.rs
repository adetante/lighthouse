mod blst;
mod fake_crypto;
mod milagro;

macro_rules! define_mod {
    ($name: ident, $mod: path) => {
        pub mod $name {
            use $mod as bls_variant;

            pub use bls_variant::{verify_signature_sets, SignatureSet};

            pub type PublicKey = crate::public_key::PublicKey<bls_variant::PublicKey>;
            pub type AggregatePublicKey =
                crate::aggregate_public_key::AggregatePublicKey<bls_variant::AggregatePublicKey>;
            pub type PublicKeyBytes =
                crate::public_key_bytes::PublicKeyBytes<bls_variant::PublicKey>;
            pub type Signature =
                crate::signature::Signature<bls_variant::PublicKey, bls_variant::Signature>;
            pub type AggregateSignature = crate::aggregate_signature::AggregateSignature<
                bls_variant::PublicKey,
                bls_variant::AggregatePublicKey,
                bls_variant::Signature,
                bls_variant::AggregateSignature,
            >;
            pub type SignatureBytes = crate::signature_bytes::SignatureBytes<
                bls_variant::PublicKey,
                bls_variant::Signature,
            >;
            pub type SecretKey = crate::secret_key::SecretKey<
                bls_variant::Signature,
                bls_variant::PublicKey,
                bls_variant::SecretKey,
            >;
            pub type Keypair = crate::keypair::Keypair<
                bls_variant::PublicKey,
                bls_variant::SecretKey,
                bls_variant::Signature,
            >;
        }
    };
}

define_mod!(milagro_implementations, super::milagro::types);
define_mod!(blst_implementations, super::blst::types);
define_mod!(fake_crypto_implementations, super::fake_crypto::types);

#[cfg(all(
    feature = "milagro",
    not(feature = "fake_crypto"),
    not(feature = "supranatural")
))]
pub use milagro_implementations::*;

#[cfg(all(
    feature = "supranatural",
    not(feature = "fake_crypto"),
    not(feature = "milagro")
))]
pub use blst_implementations::*;

#[cfg(feature = "fake_crypto")]
pub use fake_crypto_implementations::*;
