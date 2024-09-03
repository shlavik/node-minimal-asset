#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

use frame::prelude::*;

pub use pallet::*;

pub type Balance = u128;

#[frame::pallet(dev_mode)]
pub mod pallet {
	use super::*;

	#[pallet::config]
	pub trait Config: frame_system::Config {
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
	}

	#[pallet::pallet]
	pub struct Pallet<T>(_);

	#[pallet::storage]
	pub type TotalSupply<T: Config> = StorageValue<_, Balance, ValueQuery>;

	#[pallet::storage]
	pub type Balances<T: Config> = StorageMap<_, Twox64Concat, T::AccountId, Balance, ValueQuery>;

	#[pallet::storage]
	pub type Allowances<T: Config> = StorageDoubleMap<
		_,
		Twox64Concat,
		T::AccountId,
		Twox64Concat,
		T::AccountId,
		Balance,
		ValueQuery,
	>;

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		Mint {
			to: T::AccountId,
			amount: Balance,
		},
		Burn {
			from: T::AccountId,
			amount: Balance,
		},
		TotalSupply {
			total_supply: Balance,
		},
		BalanceOf {
			account: T::AccountId,
			balance: Balance,
		},
		Transfer {
			from: T::AccountId,
			to: T::AccountId,
			amount: Balance,
		},
		Approval {
			owner: T::AccountId,
			spender: T::AccountId,
			amount: Balance,
		},
		Allowance {
			owner: T::AccountId,
			spender: T::AccountId,
			amount: Balance,
		},
	}

	#[pallet::error]
	pub enum Error<T> {
		MaxSupplyExceeded,
		InsufficientBalance,
		InsufficientAllowance,
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::call_index(0)]
		pub fn mint(origin: T::RuntimeOrigin, to: T::AccountId, amount: Balance) -> DispatchResult {
			ensure_root(origin)?;
			TotalSupply::<T>::try_mutate(|supply| -> DispatchResult {
				*supply = supply
					.checked_add(amount)
					.ok_or(Error::<T>::MaxSupplyExceeded)?;
				Ok(())
			})?;
			Balances::<T>::mutate(&to, |balance| *balance = balance.saturating_add(amount));
			Self::deposit_event(Event::Mint { to, amount });
			Ok(())
		}

		#[pallet::call_index(1)]
		pub fn burn(
			origin: T::RuntimeOrigin,
			from: T::AccountId,
			amount: Balance,
		) -> DispatchResult {
			ensure_root(origin)?;
			Balances::<T>::try_mutate(&from, |balance| -> DispatchResult {
				*balance = balance
					.checked_sub(amount)
					.ok_or(Error::<T>::InsufficientBalance)?;
				Ok(())
			})?;
			TotalSupply::<T>::mutate(|supply| *supply = supply.saturating_sub(amount));
			Self::deposit_event(Event::Burn { from, amount });
			Ok(())
		}

		#[pallet::call_index(2)]
		pub fn total_supply(origin: T::RuntimeOrigin) -> DispatchResult {
			ensure_signed(origin)?;
			let total_supply = TotalSupply::<T>::get();
			Self::deposit_event(Event::TotalSupply { total_supply });
			Ok(())
		}

		#[pallet::call_index(3)]
		pub fn balance_of(origin: T::RuntimeOrigin, account: T::AccountId) -> DispatchResult {
			ensure_signed(origin)?;
			let balance = Balances::<T>::get(&account);
			Self::deposit_event(Event::BalanceOf { account, balance });
			Ok(())
		}

		#[pallet::call_index(4)]
		pub fn transfer(
			origin: T::RuntimeOrigin,
			to: T::AccountId,
			amount: Balance,
		) -> DispatchResult {
			let from = ensure_signed(origin)?;
			Self::do_transfer(from, to, amount)
		}

		#[pallet::call_index(5)]
		pub fn approve(
			origin: T::RuntimeOrigin,
			spender: T::AccountId,
			amount: Balance,
		) -> DispatchResult {
			let owner = ensure_signed(origin)?;
			Allowances::<T>::insert(&owner, &spender, amount);
			Self::deposit_event(Event::Approval {
				owner,
				spender,
				amount,
			});
			Ok(())
		}

		#[pallet::call_index(6)]
		pub fn allowance(
			origin: T::RuntimeOrigin,
			owner: T::AccountId,
			spender: T::AccountId,
		) -> DispatchResult {
			ensure_signed(origin)?;
			let amount = Allowances::<T>::get(&owner, &spender);
			Self::deposit_event(Event::Allowance {
				owner,
				spender,
				amount,
			});
			Ok(())
		}

		#[pallet::call_index(7)]
		pub fn transfer_from(
			origin: T::RuntimeOrigin,
			from: T::AccountId,
			to: T::AccountId,
			amount: Balance,
		) -> DispatchResult {
			let spender = ensure_signed(origin)?;
			Allowances::<T>::try_mutate(&from, &spender, |allowance| -> DispatchResult {
				*allowance = allowance
					.checked_sub(amount)
					.ok_or(Error::<T>::InsufficientAllowance)?;
				Self::do_transfer(from.clone(), to, amount)
			})
		}
	}

	impl<T: Config> Pallet<T> {
		fn do_transfer(from: T::AccountId, to: T::AccountId, amount: Balance) -> DispatchResult {
			Balances::<T>::try_mutate(&from, |balance| -> DispatchResult {
				*balance = balance
					.checked_sub(amount)
					.ok_or(Error::<T>::InsufficientBalance)?;
				Ok(())
			})?;
			Balances::<T>::mutate(&to, |to_balance| {
				*to_balance = to_balance.saturating_add(amount)
			});
			Self::deposit_event(Event::Transfer {
				from: from.clone(),
				to,
				amount,
			});
			Ok(())
		}
	}
}
