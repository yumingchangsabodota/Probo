
#![cfg(feature = "runtime-benchmarks")]
use super::*;

#[allow(unused)]
use crate::{Config, Pallet as Proof}; 
use frame_benchmarking::v2::*;
use frame_system::RawOrigin;
use frame_support::traits::fungible::Mutate;
use frame::runtime::{prelude::*};

fn create_user_account<T: Config>(seed: u32) -> T::AccountId {
	let entropy = (b"probo", seed).using_encoded(blake2_256);
	Decode::decode(&mut TrailingZeroInput::new(entropy.as_ref()))
			.expect("infinite length input; no invalid inputs for type; qed")
}

#[benchmarks]
mod benchmarks {
	use super::*;

    #[benchmark]
    fn register_entity() {
        let issuer = create_user_account::<T>(0);

        // ensure funds in issuer
        let _ = T::Balance::mint_into(&issuer, (u32::MAX << 10).into());

        #[extrinsic_call]
        register_entity(RawOrigin::Signed(issuer));
    }
}