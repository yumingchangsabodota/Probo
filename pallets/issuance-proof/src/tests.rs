use crate::{mock::*, *};
use frame_support::{assert_noop, assert_ok};
use frame_support::traits::fungible::Mutate;
use frame_support::pallet_prelude::*;



#[test]
fn test_failed_register_entity(){
    new_test_ext().execute_with(|| {
        let issuer = Test::create_user_account(0);
        // test fail registration when fund is not enough
        assert_noop!(Proof::do_register_entity(issuer), Error::<Test>::NotEnoughFund);
    });
}

#[test]
fn test_success_register_entity(){
    new_test_ext().execute_with(|| {
        let issuer = Test::create_user_account(0);

        // mint token before register
        let _ = <mock::Test as pallet::Config>::NativeBalance::mint_into(&issuer, (u32::MAX << 10).into());

        // test registration
        assert_ok!(Proof::do_register_entity(issuer));
    });
}

#[test]
fn test_store_proof_not_whitelist_entity(){
        new_test_ext().execute_with(|| {
        let issuer = Test::create_user_account(0);
        // mint token before register
        let _ = <mock::Test as pallet::Config>::NativeBalance::mint_into(&issuer, (u32::MAX << 10).into());
        
        let proof = vec![1u8; 512];
        let proof: BoundedVec<u8, ConstU32<512>> = proof.try_into().expect("proof too long");
        // store proof when not whitelisted; should fail
        assert_noop!(Proof::do_store_proof(issuer, proof, 1000.into()), Error::<Test>::NotWhitelistEntity);

    });
}


#[test]
fn test_success_store_proof(){
        new_test_ext().execute_with(|| {
        let issuer = Test::create_user_account(0);
        // mint token before register
        let _ = <mock::Test as pallet::Config>::NativeBalance::mint_into(&issuer, (u32::MAX << 10).into());
        // test registration
        assert_ok!(Proof::do_register_entity(issuer));
        let proof = vec![1u8; 512];
        let proof: BoundedVec<u8, ConstU32<512>> = proof.try_into().expect("proof too long");
        // store proof
        assert_ok!(Proof::do_store_proof(issuer, proof, 1000.into()));
    });
}

#[test]
fn test_store_proof_already_exist(){
        new_test_ext().execute_with(|| {
        let issuer = Test::create_user_account(0);
        // mint token before register
        let _ = <mock::Test as pallet::Config>::NativeBalance::mint_into(&issuer, (u32::MAX << 10).into());
        // test registration
        assert_ok!(Proof::do_register_entity(issuer));
        let proof = vec![1u8; 512];
        let proof: BoundedVec<u8, ConstU32<512>> = proof.try_into().expect("proof too long");
        // store proof
        assert_ok!(Proof::do_store_proof(issuer, proof.clone(), 1000.into()));

        // store existing proof; should fail
        assert_noop!(Proof::do_store_proof(issuer, proof.clone(), 1000.into()), Error::<Test>::ProofAlreadyExist);

    });
}