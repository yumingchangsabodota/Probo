#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

pub mod weights;
pub use weights::*;

#[frame_support::pallet]
pub mod pallet {

    use super::*;
    use codec::MaxEncodedLen;
    use frame_support::traits::fungible;
    use frame_support::traits::fungible::{Inspect, MutateHold};
    use frame_support::traits::tokens::{Fortitude, Preservation};
    use frame_support::{pallet_prelude::*};
    use frame_system::pallet_prelude::*;
    use sp_core::U256;

    pub type BalanceOf<T> = <<T as Config>::Balance as fungible::Inspect<
        <T as frame_system::Config>::AccountId,
    >>::Balance;

    #[pallet::pallet]
    pub struct Pallet<T>(_);

    #[pallet::config]
    pub trait Config: frame_system::Config {
        /// The overarching runtime event type.
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
        // A type representing the weights required by the dispatchables of this pallet.
        type WeightInfo: WeightInfo;

        type Balance: fungible::Inspect<Self::AccountId>
            + fungible::Mutate<Self::AccountId>
            + fungible::hold::Inspect<Self::AccountId>
            + fungible::hold::Mutate<Self::AccountId, Reason = Self::RuntimeHoldReason>
            + fungible::freeze::Inspect<Self::AccountId>
            + fungible::freeze::Mutate<Self::AccountId>;

        /// Overarching hold reason.
        type RuntimeHoldReason: From<HoldReason>;
    }

    #[pallet::composite_enum]
    pub enum HoldReason {
        /// Funds are held for registration as whitelisted entity that can store proof.
        #[codec(index = 0)]
        WhitelistEntity,
    }

    #[derive(Encode, Decode, TypeInfo, MaxEncodedLen, RuntimeDebug)]
    #[scale_info(skip_type_params(T))]
    pub struct ProofMeta<T: Config> {
        pub issuer: T::AccountId,
        pub expiry_block: U256,
    }

    #[pallet::storage]
    pub type IssuanceProof<T: Config> =
        StorageMap<_, Blake2_128Concat, BoundedVec<u8, ConstU32<512>>, ProofMeta<T>>;

    #[pallet::storage]
    pub type WhitelistEntity<T: Config> = StorageMap<_, Blake2_128Concat, T::AccountId, bool>;

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
        /// When register as whitelist entity is successful
        WhitelistEntityRegistered { entity: T::AccountId },
    }

    #[pallet::error]
    pub enum Error<T> {
        /// When user tries to store the same proof.
        ProofAlreadyExist,
        /// Emitted when user does not have enough fund to hold for becoming a legitimate entity.
        NotEnoughFund,
        /// Not whitelist entity
        NotWhitelistEntity,
    }

    #[pallet::call]
    impl<T: Config> Pallet<T> {

        /// Store proof on chain
        #[pallet::call_index(0)]
        #[pallet::weight(T::WeightInfo::do_something())]
        pub fn store_proof(
            origin: OriginFor<T>,
            proof: BoundedVec<u8, ConstU32<512>>,
            expiration: U256,
        ) -> DispatchResult {
            // Check that the extrinsic was signed and get the signer.
            let who = ensure_signed(origin)?;
            Self::do_store_proof(who, proof, expiration)?;
            // Return a successful `DispatchResult`
            Ok(())
        }

        /// Register to become legitimate entity to store proof on chain
        #[pallet::call_index(1)]
        #[pallet::weight(T::WeightInfo::do_something())]
        pub fn register_entity(origin: OriginFor<T>) -> DispatchResult {
            let who = ensure_signed(origin)?;
            Self::do_register_entity(who)?;

            Ok(())
        }
    }

    impl<T: Config> Pallet<T> {
        pub fn do_store_proof(
            issuer: T::AccountId,
            proof: BoundedVec<u8, ConstU32<512>>,
            expiration: U256,
        ) -> DispatchResult {
            // check proof already exists
            ensure!(
                !IssuanceProof::<T>::contains_key(&proof),
                Error::<T>::ProofAlreadyExist
            );
            // check is whitelisted entity
            Self::is_whitelist_entity(&issuer)?;

            let current_block = frame_system::Pallet::<T>::block_number().into();
            let expiry_block = current_block + expiration;
            let proof_meta = ProofMeta {
                issuer: issuer.clone(),
                expiry_block,
            };
            IssuanceProof::<T>::insert(&proof, proof_meta);

            // Emit an event.
            Self::deposit_event(Event::ProofStored {
                proof,
                issuer,
                expiry_block,
            });
            Ok(())
        }

        pub fn do_register_entity(issuer: T::AccountId) -> DispatchResult {
            // ensure enough balance to hold
            Self::has_enough_balance(&issuer, 1_000_000_000u32.into())?;
            // hold funds
            T::Balance::hold(
                &HoldReason::WhitelistEntity.into(),
                &issuer,
                1_000_000_000u32.into(),
            )?;

            // register whitelist entity
            WhitelistEntity::<T>::insert(&issuer, true);

            // Emit an event.
            Self::deposit_event(Event::WhitelistEntityRegistered { entity: issuer });
            Ok(())
        }

        fn has_enough_balance(issuer: &T::AccountId, amount: BalanceOf<T>) -> DispatchResult {
            let reducible =
                T::Balance::reducible_balance(issuer, Preservation::Preserve, Fortitude::Polite);
            if reducible < amount {
                return Err(Error::<T>::NotEnoughFund.into());
            }
            Ok(())
        }

        fn is_whitelist_entity(issuer: &T::AccountId) -> DispatchResult {
            ensure!(
                WhitelistEntity::<T>::contains_key(issuer),
                Error::<T>::NotWhitelistEntity
            );
            if let Some(still_whitelisted) = WhitelistEntity::<T>::get(issuer) {
                if !still_whitelisted {
                    return Err(Error::<T>::NotWhitelistEntity.into());
                }
            } else {
                return Err(Error::<T>::NotWhitelistEntity.into());
            }

            Ok(())
        }
    }
}
