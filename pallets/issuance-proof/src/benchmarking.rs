
#![cfg(feature = "runtime-benchmarks")]
use super::*;

#[allow(unused)]
use crate::{Config, Pallet as Proof}; 
use frame_benchmarking::v2::*;
use frame_support::traits::fungible::Mutate;
use frame_system::RawOrigin;
use frame::runtime::{prelude::*};
use scale_info::prelude::vec::Vec;

fn create_user_account<T: Config>(seed: u32) -> T::AccountId {
	let entropy = (b"probo", seed).using_encoded(blake2_256);
	Decode::decode(&mut TrailingZeroInput::new(entropy.as_ref()))
			.expect("infinite length input; no invalid inputs for type; qed")
}

#[benchmarks]
mod benchmarks {
	use super::*;

    /// Benchmark: register_entity
    /// It always adds just one entity.
    /// O(1)
    #[benchmark]
    fn register_entity() {
        let issuer = create_user_account::<T>(0);

        // ensure funds in issuer
        let _ = T::Balance::mint_into(&issuer, (u32::MAX << 10).into());

        #[extrinsic_call]
        register_entity(RawOrigin::Signed(issuer));
    }

    /// Benchmark: store_proof
    /// It always adds just one proof.
    /// Worst case: Longest proof, 512 byte
    /// O(1)
    #[benchmark]
    fn store_proof() {
        // setup issuer
        let issuer = create_user_account::<T>(0);

        // ensure funds in issuer
        let _ = T::Balance::mint_into(&issuer, (u32::MAX << 10).into());
        let _ = Proof::<T>::register_entity(RawOrigin::Signed(issuer.clone()).into());

        // generate longest proof
        let mut proof = Vec::new();
        for _i in 0..512 {
            proof.push(1u8)
        }

        let proof = proof.try_into().expect("proof too long");

        #[extrinsic_call]
        store_proof(RawOrigin::Signed(issuer), proof, 10_000.into());

    }
}