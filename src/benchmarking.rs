#![cfg(feature = "runtime-benchmarks")]
#![allow(unused_imports)]

use crate::*;
use frame_benchmarking::{account, benchmarks, impl_benchmark_test_suite, whitelisted_caller};
use frame_support::{
    pallet_prelude::*,
    sp_runtime::traits::{Saturating, StaticLookup},
    traits::{Currency, ExistenceRequirement},
};
use frame_system::RawOrigin;
use pallet_balances as pb;
use pallet_balances::Pallet as Balances;
use sp_std::prelude::*;

const SEED: u32 = 0;

benchmarks! {
    where_clause { where T: pb::Config }

    approve {
        let balance: BalanceOf<T> = 10000u32.into();
        let caller: T::AccountId = whitelisted_caller();
        let recipient: T::AccountId = account("recipient", 0, SEED);
        let recipient_lookup: LookupAddress<T> = T::Lookup::unlookup(recipient.clone());
    }: _(RawOrigin::Signed(caller.clone()), recipient_lookup, balance)
    verify {
        assert_eq!(Pallet::<T>::get_allowance(&caller, &recipient), balance);
    }

    transfer_from {
        let balance: BalanceOf<T> = 10000u32.into();
        let count: BalanceOf<T> = 3000u32.into();
        let allowance: BalanceOf<T> = 5000u32.into();

        let caller: T::AccountId = whitelisted_caller();
        let sender: T::AccountId = account("sender", 0, SEED);
        let sender_lookup: LookupAddress<T> = T::Lookup::unlookup(sender.clone());
        let recipient: T::AccountId = account("recipient", 0, SEED);
        let recipient_lookup: LookupAddress<T> = T::Lookup::unlookup(recipient.clone());

        let _ = T::Currency::make_free_balance_be(&sender, balance);
        Pallet::<T>::set_allowance(sender.clone(), caller.clone(), allowance);
    }: _(RawOrigin::Signed(caller.clone()), sender_lookup, recipient_lookup, count)
    verify {
        assert_eq!(Pallet::<T>::get_allowance(&sender, &caller), allowance - count);
        assert_eq!(T::Currency::free_balance(&sender), balance - count);
        assert_eq!(T::Currency::free_balance(&recipient), count);
    }

    increase_allowance {
        let count: BalanceOf<T> = 3000u32.into();
        let allowance: BalanceOf<T> = 5000u32.into();

        let caller: T::AccountId = whitelisted_caller();
        let sender: T::AccountId = account("sender", 0, SEED);
        let sender_lookup: LookupAddress<T> = T::Lookup::unlookup(sender.clone());

        Pallet::<T>::set_allowance(caller.clone(), sender.clone(), allowance);
    }: _(RawOrigin::Signed(caller.clone()), sender_lookup, count)
    verify {
        assert_eq!(Pallet::<T>::get_allowance(&caller, &sender), allowance + count);
    }

    decrease_allowance {
        let count: BalanceOf<T> = 3000u32.into();
        let allowance: BalanceOf<T> = 5000u32.into();

        let caller: T::AccountId = whitelisted_caller();
        let sender: T::AccountId = account("sender", 0, SEED);
        let sender_lookup: LookupAddress<T> = T::Lookup::unlookup(sender.clone());

        Pallet::<T>::set_allowance(caller.clone(), sender.clone(), allowance);
    }: _(RawOrigin::Signed(caller.clone()), sender_lookup, count)
    verify {
        assert_eq!(Pallet::<T>::get_allowance(&caller, &sender), allowance - count);
    }
}

impl_benchmark_test_suite!(Pallet, crate::tests::new_test_ext(), crate::tests::Test);
