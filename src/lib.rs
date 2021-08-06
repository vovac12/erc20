#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

pub mod weights;

pub(crate) mod prelude {}

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
    pub use super::weights::WeightInfo;
    use frame_support::{
        pallet_prelude::*,
        sp_runtime::traits::StaticLookup,
        traits::{Currency, ExistenceRequirement},
    };
    use frame_system::pallet_prelude::*;
    use sp_std::prelude::Vec;

    pub(crate) type BalanceOf<T> =
        <<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;
    pub(crate) type LookupAddress<T> =
        <<T as frame_system::Config>::Lookup as StaticLookup>::Source;

    #[pallet::config]
    pub trait Config: frame_system::Config {
        type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
        type Currency: Currency<Self::AccountId>;
        type WeightInfo: WeightInfo;
    }

    #[pallet::event]
    #[pallet::metadata(T::AccountId = "AccountId")]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        AllowanceChanged(T::AccountId, T::AccountId, BalanceOf<T>),
    }

    #[pallet::error]
    pub enum Error<T> {
        NotEnoughAllowance,
        DecreasedAllowanceBelowZero,
    }

    #[pallet::pallet]
    #[pallet::generate_store(trait Store)]
    pub struct Pallet<T>(PhantomData<T>);

    #[pallet::storage]
    #[pallet::getter(fn get_allowance)]
    type Allowances<T: Config> = StorageDoubleMap<
        _,
        Blake2_128Concat,
        T::AccountId,
        Blake2_128Concat,
        T::AccountId,
        BalanceOf<T>,
        ValueQuery,
    >;

    #[pallet::hooks]
    impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}

    impl<T: Config> Pallet<T> {
        pub(crate) fn set_allowance(
            owner: T::AccountId,
            sender: T::AccountId,
            count: BalanceOf<T>,
        ) {
            <Allowances<T>>::insert(owner.clone(), sender.clone(), count);
            Self::deposit_event(Event::AllowanceChanged(owner, sender, count));
        }
    }

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        #[pallet::weight(T::WeightInfo::approve())]
        pub fn approve(
            owner: OriginFor<T>,
            sender: LookupAddress<T>,
            count: BalanceOf<T>,
        ) -> DispatchResultWithPostInfo {
            let owner = ensure_signed(owner)?;
            let sender = T::Lookup::lookup(sender)?;
            Self::set_allowance(owner.clone(), sender.clone(), count);
            Ok(().into())
        }

        #[pallet::weight(T::WeightInfo::transfer_from())]
        pub fn transfer_from(
            origin: OriginFor<T>,
            src: LookupAddress<T>,
            dst: LookupAddress<T>,
            #[pallet::compact] count: BalanceOf<T>,
        ) -> DispatchResultWithPostInfo {
            let origin = ensure_signed(origin)?;
            let src = T::Lookup::lookup(src)?;
            let dst = T::Lookup::lookup(dst)?;
            let allowance = Self::get_allowance(&src, &origin);
            ensure!(allowance >= count, Error::<T>::NotEnoughAllowance);

            T::Currency::transfer(&src, &dst, count, ExistenceRequirement::KeepAlive)?;

            let result_allowance = allowance - count;
            Self::set_allowance(src.clone(), origin.clone(), result_allowance);

            Ok(().into())
        }

        #[pallet::weight(T::WeightInfo::increase_allowance())]
        pub fn increase_allowance(
            origin: OriginFor<T>,
            sender: LookupAddress<T>,
            count: BalanceOf<T>,
        ) -> DispatchResultWithPostInfo {
            let owner = ensure_signed(origin)?;
            let sender = T::Lookup::lookup(sender)?;
            let new_allowance = Self::get_allowance(&owner, &sender) + count;
            Self::set_allowance(owner, sender, new_allowance);
            Ok(().into())
        }

        #[pallet::weight(T::WeightInfo::decrease_allowance())]
        pub fn decrease_allowance(
            owner: OriginFor<T>,
            sender: LookupAddress<T>,
            count: BalanceOf<T>,
        ) -> DispatchResultWithPostInfo {
            let owner = ensure_signed(owner)?;
            let sender = T::Lookup::lookup(sender)?;
            let allowance = Self::get_allowance(&owner, &sender);
            ensure!(allowance >= count, Error::<T>::DecreasedAllowanceBelowZero);
            let new_allowance = allowance - count;
            Self::set_allowance(owner, sender, new_allowance);
            Ok(().into())
        }
    }

    #[pallet::storage]
    #[pallet::getter(fn get_name)]
    pub(super) type Name<T: Config> = StorageValue<_, Vec<u8>, ValueQuery>;

    #[pallet::storage]
    #[pallet::getter(fn get_symbol)]
    pub(super) type Symbol<T: Config> = StorageValue<_, Vec<u8>, ValueQuery>;

    #[pallet::genesis_config]
    pub struct GenesisConfig<T: Config> {
        pub allowances: Vec<(T::AccountId, T::AccountId, BalanceOf<T>)>,
    }

    #[cfg(feature = "std")]
    impl<T: Config> Default for GenesisConfig<T> {
        fn default() -> Self {
            Self { allowances: vec![] }
        }
    }

    #[pallet::genesis_build]
    impl<T: Config> GenesisBuild<T> for GenesisConfig<T> {
        fn build(&self) {
            for (owner, sender, count) in &self.allowances {
                <Allowances<T>>::insert(owner.clone(), sender.clone(), count);
            }
        }
    }
}
