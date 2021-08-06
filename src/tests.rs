use crate::{mock::*, Error};
use frame_support::{assert_noop, assert_ok, error::BadOrigin, traits::Currency};
use pallet_balances as pb;

#[test]
fn zero_allowance_error() {
    new_test_ext().execute_with(|| {
        assert_eq!(ERC20::get_allowance(1, 0), 0);
        assert_noop!(
            ERC20::transfer_from(Origin::signed(0), 1, 2, 2000000),
            Error::<Test>::NotEnoughAllowance,
        );
        assert_eq!(pallet_balances::Pallet::<Test>::total_balance(&0), 1000000);
        assert_eq!(pallet_balances::Pallet::<Test>::total_balance(&1), 1000000);
        assert_eq!(pallet_balances::Pallet::<Test>::total_balance(&2), 1000000);
    });
}

#[test]
fn unsigned_transfer() {
    new_test_ext().execute_with(|| {
        assert_noop!(
            ERC20::transfer_from(Origin::none(), 1, 2, 200000),
            BadOrigin
        );
        assert_eq!(pb::Pallet::<Test>::total_balance(&1), 1000000);
        assert_eq!(pb::Pallet::<Test>::total_balance(&2), 1000000);
    });
}

#[test]
fn signed_transfer() {
    new_test_ext().execute_with(|| {
        assert_eq!(ERC20::get_allowance(3, 4), 500000);
        assert_ok!(ERC20::transfer_from(Origin::signed(4), 3, 1, 200000));
        assert_eq!(pb::Pallet::<Test>::total_balance(&1), 1200000);
        assert_eq!(pb::Pallet::<Test>::total_balance(&3), 800000);
        assert_eq!(pb::Pallet::<Test>::total_balance(&4), 1000000);
        assert_eq!(ERC20::get_allowance(3, 4), 300000);
    });
}

#[test]
fn increase_allowance() {
    new_test_ext().execute_with(|| {
        assert_eq!(ERC20::get_allowance(3, 4), 500000);
        assert_ok!(ERC20::increase_allowance(Origin::signed(3), 4, 200000));
        assert_eq!(ERC20::get_allowance(3, 4), 700000);
    });
}

#[test]
fn decrease_allowance() {
    new_test_ext().execute_with(|| {
        assert_eq!(ERC20::get_allowance(3, 4), 500000);
        assert_ok!(ERC20::decrease_allowance(Origin::signed(3), 4, 200000));
        assert_eq!(ERC20::get_allowance(3, 4), 300000);
    });
}

#[test]
fn decrease_allowance_error() {
    new_test_ext().execute_with(|| {
        assert_eq!(ERC20::get_allowance(3, 4), 500000);
        assert_noop!(
            ERC20::decrease_allowance(Origin::signed(3), 4, 800000),
            Error::<Test>::DecreasedAllowanceBelowZero
        );
        assert_eq!(ERC20::get_allowance(3, 4), 500000);
    });
}
