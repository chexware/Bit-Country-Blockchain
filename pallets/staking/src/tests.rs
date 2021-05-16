#![cfg(test)]

use super::*;
use mock::{Event, *};
use frame_support::{assert_ok, assert_noop};

#[test]
// Staking should work
fn staking_should_work() {
    ExtBuilder::default().build().execute_with(|| {
        let origin = Origin::signed(ALICE);
        assert_ok!(CountryModule::create_country(origin.clone(),vec![1]));
        assert_ok!(StakingModule::stake(origin.clone(),COUNTRY, 100));
        assert_eq!(StakingModule::staked_balance(&ALICE,COUNTRY),100);
        assert_eq!(last_event(), Event::staking_module(crate::Event::BalanceStaked(ALICE,COUNTRY, 100)));
        assert_ok!(StakingModule::stake(origin.clone(),COUNTRY, 100));
        assert_eq!(StakingModule::staked_balance(&ALICE,COUNTRY),200);
        assert_eq!(last_event(), Event::staking_module(crate::Event::BalanceStaked(ALICE,COUNTRY, 100)));
    });
}

#[test]
// Staking insufficient balance shouldn't work
fn staking_insufficient_balance_should_not_work() {
    ExtBuilder::default().build().execute_with(|| {
        let origin = Origin::signed(BOB);
        assert_ok!(CountryModule::create_country(origin.clone(),vec![1]));
        assert_noop!(StakingModule::stake(origin.clone(),COUNTRY,2001),Error::<Runtime>::InsufficientFreeBalance);
   
    });
}

#[test]
// Staking on non-existing country should not work
fn staking_on_non_existing_country_should_not_work() {
    ExtBuilder::default().build().execute_with(|| {
        let origin = Origin::signed(BOB);
        assert_noop!(StakingModule::stake(origin.clone(),COUNTRY_ID_NOT_EXIST,100),Error::<Runtime>::CountryDoesNotExist);
    });
}

#[test]
// Unstaking should work
fn unstaking_should_work() {
    ExtBuilder::default().build().execute_with(|| {
        let origin = Origin::signed(ALICE);
        assert_ok!(CountryModule::create_country(origin.clone(),vec![1]));
        assert_ok!(StakingModule::stake(origin.clone(), COUNTRY, 100));
        assert_ok!(StakingModule::unstake(origin.clone(), COUNTRY, 50));
        assert_eq!(StakingModule::staked_balance(&ALICE,COUNTRY),50);
        assert_eq!(Balances::reserved_balance(&ALICE),100);
        assert_eq!(StakingModule::unstake_request(101,&ALICE),50);
        assert_eq!(last_event(), Event::staking_module(crate::Event::BalanceUnstaked(ALICE, COUNTRY, 50)));
        run_to_block(102);
        assert_eq!(Balances::reserved_balance(&ALICE),50);
        assert_eq!(last_event(), Event::staking_module(crate::Event::UnstakeRequestCompleted(ALICE, 50)));
        
    });
}

#[test]
// Unstaking insufficient balance shouldn't work
fn unstaking_insufficient_balance_should_not_work() {
    ExtBuilder::default().build().execute_with(|| {
        let origin = Origin::signed(ALICE);
        assert_ok!(CountryModule::create_country(origin.clone(),vec![1]));
        assert_ok!(StakingModule::stake(origin.clone(),COUNTRY,100));
        assert_noop!(StakingModule::unstake(origin.clone(),COUNTRY, 105), Error::<Runtime>::InsufficientStakedBalance);
    });
}

#[test]
// Reinvest rewards should work
fn reinvest_rewards_should_work() {
    ExtBuilder::default().build().execute_with(|| {
        let origin = Origin::signed(ALICE);
        assert_ok!(CountryModule::create_country(origin.clone(),vec![1]));
        assert_ok!(StakingModule::stake(origin.clone(),COUNTRY, 100));
        assert_eq!(StakingModule::staked_balance(&ALICE,COUNTRY),100);
        assert_eq!(StakingModule::account_rewards(&ALICE,COUNTRY),0);
        Rewards::<Runtime>::insert(&ALICE,COUNTRY,10);
        assert_eq!(StakingModule::account_rewards(&ALICE,COUNTRY),10);
        assert_ok!(StakingModule::reinvest(origin.clone(),COUNTRY));
        assert_eq!(StakingModule::account_rewards(&ALICE,COUNTRY),0);
        assert_eq!(Balances::free_balance(&ALICE), 99900);
        assert_eq!(StakingModule::staked_balance(&ALICE,COUNTRY),110);
        assert_eq!(last_event(), Event::staking_module(crate::Event::BalanceReinvested(ALICE,COUNTRY,10)));
    });
}
#[test]
// Reinvesting no rewards shouldn't work
fn reinvesting_no_rewards_should_not_work() {
    ExtBuilder::default().build().execute_with(|| {
        let origin = Origin::signed(ALICE);
        assert_ok!(CountryModule::create_country(origin.clone(),vec![1]));
        assert_ok!(StakingModule::stake(origin.clone(),COUNTRY, 100));
        assert_eq!(StakingModule::staked_balance(&ALICE,COUNTRY),100);
        run_to_block(121); // less than end of staking era
        assert_noop!(StakingModule::reinvest(origin.clone(),COUNTRY),Error::<Runtime>::NoRewardsAvailable);
    });
}

#[test]
// Claiming rewards should work
fn claim_rewards_should_work() {
    ExtBuilder::default().build().execute_with(|| {
        let origin = Origin::signed(ALICE);
        assert_ok!(CountryModule::create_country(origin.clone(),vec![1]));
        assert_ok!(StakingModule::stake(origin.clone(),COUNTRY,100));
        assert_eq!(StakingModule::staked_balance(&ALICE,COUNTRY),100);
        assert_eq!(StakingModule::account_rewards(&ALICE,COUNTRY),0);
        Rewards::<Runtime>::insert(&ALICE,COUNTRY,20);
        assert_eq!(StakingModule::account_rewards(&ALICE,COUNTRY),20);
        assert_ok!(StakingModule::claim(origin.clone(),COUNTRY));
        assert_eq!(StakingModule::account_rewards(&ALICE,COUNTRY),0);
        assert_eq!(Balances::free_balance(&ALICE), 99920);
        assert_eq!(StakingModule::staked_balance(&ALICE,COUNTRY),100);
        assert_eq!(last_event(), Event::staking_module(crate::Event::StakingRewardClaimed(ALICE,COUNTRY,20)));
    });
}

#[test]
// Claiming no rewards shouldn't work
fn claiming_no_rewards_should_not_work() {
    ExtBuilder::default().build().execute_with(|| {
        let origin = Origin::signed(ALICE);
        assert_ok!(CountryModule::create_country(origin.clone(),vec![1]));
        assert_ok!(StakingModule::stake(origin.clone(),COUNTRY, 100));
        assert_eq!(StakingModule::staked_balance(&ALICE,COUNTRY),100);
        run_to_block(121); // less than end of staking era
        assert_noop!(StakingModule::claim(origin.clone(),COUNTRY),Error::<Runtime>::NoRewardsAvailable);
    });
}

#[test]
fn era_should_update() {
    ExtBuilder::default().build().execute_with(|| {
        run_to_block(105);
        assert_eq!(StakingModule::current_era(),1);
        run_to_block(205); // after end of era 1
        assert_eq!(StakingModule::current_era(),2);
        run_to_block(405); // after end of era 1
        assert_eq!(StakingModule::current_era(),3);
    });
}

#[test]
// Era payout should work
fn era_payout_should_work() {
    ExtBuilder::default().build().execute_with(|| {
        let origin = Origin::signed(ALICE);
        assert_ok!(CountryModule::create_country(origin.clone(),vec![1]));
        assert_ok!(StakingModule::stake(origin.clone(),COUNTRY,100));
        assert_eq!(StakingModule::staked_balance(&ALICE,COUNTRY),100);
        run_to_block(105);
        assert_eq!(StakingModule::account_rewards(&ALICE,COUNTRY),0);
        run_to_block(205); // end of era 1
        assert_eq!(StakingModule::account_rewards(&ALICE,COUNTRY),10);
        assert_eq!(last_event(), Event::staking_module(crate::Event::EraPayout(1)));
        run_to_block(408); // end of era 2
        assert_eq!(StakingModule::account_rewards(&ALICE,COUNTRY),20);
        assert_eq!(last_event(), Event::staking_module(crate::Event::EraPayout(2)));
    });
}
