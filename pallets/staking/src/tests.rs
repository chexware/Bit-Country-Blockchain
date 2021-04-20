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
        assert_eq!(last_event(), Event::staking_module(RawEvent::BalanceStaked(ALICE, ALICE, 100)));
    });
}