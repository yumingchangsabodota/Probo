#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

pub mod weights;
pub use weights::*;

#[frame_support::pallet]
pub mod pallet {

	use super::*;
	use codec::MaxEncodedLen;
    use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;
    use sp_core::U256;

	#[pallet::pallet]
	pub struct Pallet<T>(_);

	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// The overarching runtime event type.
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
		// A type representing the weights required by the dispatchables of this pallet.
		type WeightInfo: WeightInfo;
	}


	#[derive(Encode, Decode, TypeInfo, MaxEncodedLen, RuntimeDebug)]
	#[scale_info(skip_type_params(T))]
	pub struct ProofMeta<T: Config> {
		pub issuer: T::AccountId,
        pub expiry_block: U256,
	}

    #[pallet::storage]
    pub type IssuanceProof<T: Config> = StorageMap<_, Blake2_128Concat, BoundedVec<u8, ConstU32<512>>, ProofMeta<T>>;

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// A user has successfully set a new value.
		ProofStored {
			/// The new value set.
			proof: BoundedVec<u8, ConstU32<512>>,
			/// The document issuer's public address
			issuer: T::AccountId,
            /// Expiry Block Number
            expiry_block: U256,
		},
	}

    #[pallet::error]
	pub enum Error<T> {
		/// When user tries to store the same proof.
		ProofAlreadyExist,
	}

    #[pallet::call]
    impl<T: Config> Pallet<T> {
		#[pallet::call_index(0)]
		#[pallet::weight(T::WeightInfo::do_something())]
		pub fn store_proof(origin: OriginFor<T>, proof: BoundedVec<u8, ConstU32<512>>, expiration: U256) -> DispatchResult {
			// Check that the extrinsic was signed and get the signer.
			let who = ensure_signed(origin)?;
			Self::do_store_proof(who, proof, expiration)?;
			// Return a successful `DispatchResult`
			Ok(())
		}
    }

    impl<T: Config> Pallet<T> {

        pub fn do_store_proof(issuer: T::AccountId, proof: BoundedVec<u8, ConstU32<512>>, expiration: U256) -> DispatchResult {
            // check proof already exists
            ensure!(!IssuanceProof::<T>::contains_key(&proof), Error::<T>::ProofAlreadyExist);
            let current_block = frame_system::Pallet::<T>::block_number().into();
            let expiry_block = current_block + expiration;
            let proof_meta = ProofMeta{ issuer: issuer.clone(), expiry_block };
            IssuanceProof::<T>::insert(&proof, proof_meta);
            
			// Emit an event.
			Self::deposit_event(Event::ProofStored { proof, issuer, expiry_block });
            Ok(())
        }
    }

}