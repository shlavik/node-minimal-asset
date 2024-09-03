use crate::{mock::*, Allowances, Balances, Error, Event, Pallet, TotalSupply};

const ALICE: u64 = 1;
const BOB: u64 = 2;
const CHARLIE: u64 = 3;

#[test]
fn mint_works() {
	TestState::new_empty().execute_with(|| {
		System::set_block_number(1);
		assert_ok!(Pallet::<Mock>::mint(RuntimeOrigin::root(), ALICE, 100));
		assert_eq!(Balances::<Mock>::get(ALICE), 100);
		assert_eq!(TotalSupply::<Mock>::get(), 100);
		assert!(System::events().iter().any(|record| matches!(
			record.event,
			RuntimeEvent::Asset(Event::Mint {
				to: ALICE,
				amount: 100
			})
		)));
	});
}

#[test]
fn burn_works() {
	TestState::new_empty().execute_with(|| {
		System::set_block_number(1);
		assert_ok!(Pallet::<Mock>::mint(RuntimeOrigin::root(), ALICE, 100));
		assert_ok!(Pallet::<Mock>::burn(RuntimeOrigin::root(), ALICE, 50));
		assert_eq!(Balances::<Mock>::get(ALICE), 50);
		assert_eq!(TotalSupply::<Mock>::get(), 50);
		assert!(System::events().iter().any(|record| matches!(
			record.event,
			RuntimeEvent::Asset(Event::Burn {
				from: ALICE,
				amount: 50
			})
		)));
	});
}

#[test]
fn transfer_works() {
	TestState::new_empty().execute_with(|| {
		System::set_block_number(1);
		assert_ok!(Pallet::<Mock>::mint(RuntimeOrigin::root(), ALICE, 100));
		assert_ok!(Pallet::<Mock>::transfer(
			RuntimeOrigin::signed(ALICE),
			BOB,
			50
		));
		assert_eq!(Balances::<Mock>::get(ALICE), 50);
		assert_eq!(Balances::<Mock>::get(BOB), 50);
		assert_eq!(TotalSupply::<Mock>::get(), 100);
		assert!(System::events().iter().any(|record| matches!(
			record.event,
			RuntimeEvent::Asset(Event::Transfer {
				from: ALICE,
				to: BOB,
				amount: 50
			})
		)));
	});
}

#[test]
fn transfer_from_non_existent_fails() {
	TestState::new_empty().execute_with(|| {
		assert_err!(
			Pallet::<Mock>::transfer(RuntimeOrigin::signed(CHARLIE), ALICE, 10),
			Error::<Mock>::InsufficientBalance
		);
		assert_eq!(Balances::<Mock>::get(ALICE), 0);
		assert_eq!(Balances::<Mock>::get(BOB), 0);
		assert_eq!(Balances::<Mock>::get(CHARLIE), 0);
		assert_eq!(TotalSupply::<Mock>::get(), 0);
	});
}

#[test]
fn balance_of_and_total_supply_works() {
	TestState::new_empty().execute_with(|| {
		System::set_block_number(1);
		assert_ok!(Pallet::<Mock>::mint(RuntimeOrigin::root(), ALICE, 100));
		assert_ok!(Pallet::<Mock>::balance_of(
			RuntimeOrigin::signed(ALICE),
			ALICE
		));
		assert!(System::events().iter().any(|record| matches!(
			record.event,
			RuntimeEvent::Asset(Event::BalanceOf {
				account: ALICE,
				balance: 100
			})
		)));
		assert_ok!(Pallet::<Mock>::total_supply(RuntimeOrigin::signed(ALICE)));
		assert!(System::events().iter().any(|record| matches!(
			record.event,
			RuntimeEvent::Asset(Event::TotalSupply { total_supply: 100 })
		)));
	});
}

#[test]
fn approve_and_allowance_works() {
	TestState::new_empty().execute_with(|| {
		System::set_block_number(1);
		assert_ok!(Pallet::<Mock>::approve(
			RuntimeOrigin::signed(ALICE),
			BOB,
			50
		));
		assert!(System::events().iter().any(|record| matches!(
			record.event,
			RuntimeEvent::Asset(Event::Approval {
				owner: ALICE,
				spender: BOB,
				amount: 50
			})
		)));
		assert_ok!(Pallet::<Mock>::allowance(
			RuntimeOrigin::signed(ALICE),
			ALICE,
			BOB
		));
		assert!(System::events().iter().any(|record| matches!(
			record.event,
			RuntimeEvent::Asset(Event::Allowance {
				owner: ALICE,
				spender: BOB,
				amount: 50
			})
		)));
	});
}

#[test]
fn transfer_from_works() {
	TestState::new_empty().execute_with(|| {
		System::set_block_number(1);
		assert_ok!(Pallet::<Mock>::mint(RuntimeOrigin::root(), ALICE, 100));
		assert_ok!(Pallet::<Mock>::approve(
			RuntimeOrigin::signed(ALICE),
			BOB,
			50
		));
		assert_ok!(Pallet::<Mock>::transfer_from(
			RuntimeOrigin::signed(BOB),
			ALICE,
			CHARLIE,
			50
		));
		assert_eq!(Balances::<Mock>::get(ALICE), 50);
		assert_eq!(Balances::<Mock>::get(CHARLIE), 50);
		assert_eq!(Allowances::<Mock>::get(ALICE, BOB), 0);
		assert!(System::events().iter().any(|record| matches!(
			record.event,
			RuntimeEvent::Asset(Event::Transfer {
				from: ALICE,
				to: CHARLIE,
				amount: 50
			})
		)));
	});
}
