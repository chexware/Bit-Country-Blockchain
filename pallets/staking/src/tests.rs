#![cfg(test)]

use super::*;
use mock::{Event, *};
use frame_support::{assert_ok, assert_noop};

#[test]
// Staking should work
fn staking_should_work() {
    ExtBuilder::default().build().execute_with(|| {
        let origin = Origin::signed(ALICE);
        assert_ok!(StakingModule::stake(origin,ALICE, 100));
        assert_eq!(StakingModule::get_staked_balance(&ALICE),Some(100));
        assert_eq!(last_event(), Event::staking_module(RawEvent::BalanceStaked(ALICE, ALICE, 100)));
    });
}

#[test]
// Staking insuficient balance shouldn't work
fn staking_insuficient_balance_should_not_work() {
    ExtBuilder::default().build().execute_with(|| {
        let origin = Origin::signed(BOB);
        assert_noop!(StakingModule::stake(origin,BOB, 2001),Error::<Runtime>::InsufficientFreeBalance);
   
    });
}
#[test]
// Unstaking should work
fn unstaking_should_work() {
    ExtBuilder::default().build().execute_with(|| {
        let origin = Origin::signed(ALICE);
        assert_ok!(StakingModule::stake(origin,ALICE, 100));
        assert_eq!(StakingModule::get_staked_balance(&ALICE),Some(100));
        assert_ok!(StakingModule::unstake(Origin::signed(ALICE), 100));
        assert_eq!(StakingModule::get_staked_balance(&ALICE),Some(0));
        assert_eq!(StakingModule::get_locked_balance(&ALICE),Some(100));
        run_to_block(101);
        assert_eq!(StakingModule::get_staked_balance(&ALICE),Some(0));
        assert_eq!(StakingModule::get_locked_balance(&ALICE),Some(0));
        assert_eq!(last_event(), Event::staking_module(RawEvent::BalanceUnstaked(ALICE, 100)));
    });
}

#[test]
// Unstaking insuficient balance shouldn't work
fn unstaking_insuficient_balance_should_not_work() {
    ExtBuilder::default().build().execute_with(|| {
        let origin = Origin::signed(ALICE);
        assert_ok!(StakingModule::stake(origin,ALICE, 100));
        assert_noop!(StakingModule::unstake(Origin::signed(ALICE), 105), Error::<Runtime>::InsufficientStakedBalance);
    });
}
#[test]
// Reinvest rewards should work
fn reinvest_rewards_should_work() {
    ExtBuilder::default().build().execute_with(|| {
        let origin = Origin::signed(ALICE);
        assert_ok!(StakingModule::stake(origin,ALICE, 100));
        assert_eq!(StakingModule::get_staked_balance(&ALICE),Some(100));
        run_to_block(201); //end of staking era
        assert_ok!(StakingModule::reinvest_rewards(Origin::signed(ALICE)));
        assert_eq!(StakingModule::get_staked_balance(&ALICE),Some(110));
        assert_eq!(last_event(), Event::staking_module(RawEvent::BalanceReinvested(ALICE,10)));
    });
}
#[test]
// Reinvesting no rewards shouldn't work
fn reinvesting_no_rewards_should_not_work() {
    ExtBuilder::default().build().execute_with(|| {
        let origin = Origin::signed(ALICE);
        assert_ok!(StakingModule::stake(origin,ALICE, 100));
        assert_eq!(StakingModule::get_staked_balance(&ALICE),Some(100));
        run_to_block(121); // less than end of staking era
        assert_noop!(StakingModule::reinvest_rewards(Origin::signed(ALICE)),Error::<Runtime>::NoRewardsAvailable);
    });
}

#[test]
// Claiming rewards should work
fn claim_rewards_should_work() {
    ExtBuilder::default().build().execute_with(|| {
        let origin = Origin::signed(ALICE);
        assert_ok!(StakingModule::stake(origin,ALICE, 100));
        assert_eq!(StakingModule::get_staked_balance(&ALICE),Some(100));
        run_to_block(201); // end of staking era
        assert_ok!(StakingModule::claim_rewards(Origin::signed(ALICE)));
        assert_eq!(Balances::free_balance(&ALICE), 99910);
        assert_eq!(last_event(), Event::staking_module(RawEvent::StakingRewardClaimed(ALICE,10)));
    });
}

#[test]
// Claiming no rewards shouldn't work
fn claiming_no_rewards_should_not_work() {
    ExtBuilder::default().build().execute_with(|| {
        let origin = Origin::signed(ALICE);
        assert_ok!(StakingModule::stake(origin,ALICE, 100));
        assert_eq!(StakingModule::get_staked_balance(&ALICE),Some(100));
        run_to_block(121); // less than end of staking era
        assert_noop!(StakingModule::claim_rewards(Origin::signed(ALICE)),Error::<Runtime>::NoRewardsAvailable);
    });
}