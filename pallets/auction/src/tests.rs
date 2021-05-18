#![cfg(test)]

use super::*;
use frame_support::{assert_noop, assert_ok};
use mock::{Event, *};
use pallet_nft::{TokenType, CollectionType};


fn init_test_nft(owner: Origin) {

    //Create group collection before class
    assert_ok!(NFTModule::<Runtime>::create_group(
        Origin::root(),
        vec![1],
        vec![1]
    ));

    assert_ok!(NFTModule::<Runtime>::create_class(
        owner.clone(),
        vec![1],
        vec![1],
        COLLECTION_ID,
        TokenType::Transferable,
        CollectionType::Collectable,
    ));

    assert_ok!(NFTModule::<Runtime>::mint(
        owner.clone(),
        CLASS_ID,
        vec![1],
        vec![1],
        vec![1],
        1
    ));    
}

#[test]
// Private new_auction should work
fn create_new_auction_work() {
    ExtBuilder::default().build().execute_with(|| {
        let origin = Origin::signed(ALICE);
        init_test_nft(origin.clone());
    });
}

#[test]
// Walk the happy path
fn bid_works() {
    ExtBuilder::default().build().execute_with(|| {
        let owner = Origin::signed(BOB);
        let bidder = Origin::signed(ALICE);
        
        init_test_nft(owner.clone());        
        assert_ok!(NftAuctionModule::create_auction(AuctionType::Auction, ItemId::NFT(0), None, BOB, 100, 0));
        
        assert_ok!(NftAuctionModule::bid(bidder, 0, 200));
        assert_eq!(last_event(), Event::auction(crate::Event::Bid(0, ALICE, 200)));

        assert_eq!(Balances::reserved_balance(ALICE), 200);
    });
}

#[test]
fn cannot_bid_on_non_existent_auction() {
    ExtBuilder::default().build().execute_with(|| {
        assert_noop!(
            NftAuctionModule::bid(Origin::signed(ALICE), 0, 10), 
            Error::<Runtime>::AuctionNotExist
        ); 

        assert_eq!(Balances::free_balance(ALICE), 100000);
    });
}

#[test]
fn cannot_bid_with_insufficient_funds() {
    ExtBuilder::default().build().execute_with(|| {
        let owner = Origin::signed(BOB);
        let bidder = Origin::signed(ALICE);
        
        init_test_nft(owner.clone());        
        assert_ok!(NftAuctionModule::create_auction(AuctionType::Auction, ItemId::NFT(0), None, BOB, 600, 0));
        
        assert_noop!(
            NftAuctionModule::bid(bidder, 0, 100001), 
            "You don\'t have enough free balance for this bid"
        );

        assert_eq!(Balances::free_balance(ALICE), 100000);

    });
}

#[test]
fn cannot_bid_on_own_auction() {
    ExtBuilder::default().build().execute_with(|| {        
        let owner = Origin::signed(ALICE);
        
        init_test_nft(owner.clone());        
        assert_ok!(NftAuctionModule::create_auction(AuctionType::Auction, ItemId::NFT(0), None, ALICE, 100, 0));
        
        assert_noop!(
            NftAuctionModule::bid(owner, 0, 50), 
            Error::<Runtime>::SelfBidNotAccepted
        );
    });
}

#[test]
fn asset_transfers_after_auction() {
    ExtBuilder::default().build().execute_with(|| {
        let owner = Origin::signed(BOB);
        let bidder = Origin::signed(ALICE);
        
        // Make sure balances start off as we expect
        assert_eq!(Balances::free_balance(BOB), 500);
        assert_eq!(Balances::free_balance(ALICE), 100000);

        // Setup NFT and verify that BOB has ownership
        init_test_nft(owner.clone());            
        assert_eq!(NFTModule::<Runtime>::get_assets_by_owner(BOB), [0]);

        assert_ok!(NftAuctionModule::create_auction(AuctionType::Auction, ItemId::NFT(0), None, BOB, 100, 0));
    
        assert_ok!(NftAuctionModule::bid(bidder, 0, 200));
        assert_eq!(last_event(), Event::auction(crate::Event::Bid(0, ALICE, 200)));

        run_to_block(102);
        
        // Verify asset transfers to alice after end of auction
        assert_eq!(
            last_event(), 
            Event::auction(crate::Event::AuctionFinalized(0, 1 ,200))
        );          

        // Verify transfer of funs (minus gas)
        assert_eq!(Balances::free_balance(BOB), 697);
        assert_eq!(Balances::free_balance(ALICE), 99800);

        // Verify Alice has the NFT and Bob doesn't
        assert_eq!(NFTModule::<Runtime>::get_assets_by_owner(ALICE), [0]);                
        assert_eq!(NFTModule::<Runtime>::get_assets_by_owner(BOB), Vec::<u64>::new());
    });
}

#[test]
fn cannot_bid_on_ended_auction() {
    ExtBuilder::default().build().execute_with(|| {
        let owner = Origin::signed(BOB);
        let bidder = Origin::signed(ALICE);
        
        init_test_nft(owner.clone());        
        assert_ok!(NftAuctionModule::create_auction(AuctionType::Auction, ItemId::NFT(0), None, BOB, 150, 0));

        System::set_block_number(101);
                
        assert_noop!(
            NftAuctionModule::bid(bidder, 0, 200), 
            Error::<Runtime>::AuctionIsExpired
        );

        assert_eq!(Balances::free_balance(ALICE), 100000);
    });
}

#[test]
// Private bid_auction should work
fn buy_now_work() {
    ExtBuilder::default().build().execute_with(|| {
        let owner = Origin::signed(BOB);
        let buyer = Origin::signed(ALICE);
        init_test_nft(owner.clone());

        // call create_auction
        assert_ok!(NftAuctionModule::create_auction(AuctionType::BuyNow, ItemId::NFT(0), None, BOB, 150, 0));
        
        //buy now successful
        assert_ok!(NftAuctionModule::buy_now(buyer.clone(), 0, 150));

        assert_eq!(NftAuctionModule::auctions(0), None);
        // check account received asset
        assert_eq!(NFTModule::<Runtime>::get_assets_by_owner(ALICE), [0]);
        // check balances were transferred
        assert_eq!(Balances::free_balance(ALICE), 99850);
        assert_eq!(Balances::free_balance(BOB), 647);

        //event was triggered
        let event = mock::Event::auction(crate::Event::BuyNowFinalised(0, ALICE, 150));
        assert_eq!(last_event(), event);

        //Check that auction is over
        assert_noop!(NftAuctionModule::buy_now(buyer.clone(), 1, 150),Error::<Runtime>::AuctionNotExist);
    });
}

#[test]
// Private bid_auction should work
fn buy_now_should_fail() {
    ExtBuilder::default().build().execute_with(|| {
        let owner = Origin::signed(BOB);
        let buyer = Origin::signed(ALICE);
        // we need this to test auction not started scenario
        System::set_block_number(1);
        init_test_nft(owner.clone());

        // call create_auction
        assert_ok!(NftAuctionModule::create_auction(AuctionType::BuyNow, ItemId::NFT(0), None, BOB, 150, 0));
        
        // no auction id 
        assert_noop!(NftAuctionModule::buy_now(buyer.clone(), 1, 150),Error::<Runtime>::AuctionNotExist);
        // user is seller
        assert_noop!(NftAuctionModule::buy_now(owner.clone(), 0, 150),Error::<Runtime>::CannotBidOnOwnAuction);
        //buy it now value is less than buy_now_amount
        assert_noop!(NftAuctionModule::buy_now(buyer.clone(), 0, 100),Error::<Runtime>::InvalidBuyItNowPrice);
        //buy it now value is more than buy_now_amount
        assert_noop!(NftAuctionModule::buy_now(buyer.clone(), 0, 200),Error::<Runtime>::InvalidBuyItNowPrice);
        // user does not have enough balance in wallet
        assert_ok!(Balances::reserve(&ALICE, 100000));
        assert_noop!(NftAuctionModule::buy_now(buyer.clone(), 0, 150),Error::<Runtime>::InsufficientFunds);
        assert_eq!(Balances::unreserve(&ALICE, 100000), 0);
        //auction has not started or is over
        System::set_block_number(0);
        assert_noop!(NftAuctionModule::buy_now(buyer.clone(), 0, 150),Error::<Runtime>::AuctionNotStarted);
        System::set_block_number(101);
        assert_noop!(NftAuctionModule::buy_now(buyer.clone(), 0, 150),Error::<Runtime>::AuctionIsExpired);
    });
}

#[test]
// Private bid_auction should work
fn invalid_auction_type() {
    ExtBuilder::default().build().execute_with(|| {
        let owner = Origin::signed(BOB);
        init_test_nft(owner.clone());
        let participant = Origin::signed(ALICE);
        assert_ok!(NftAuctionModule::create_auction(AuctionType::BuyNow, ItemId::NFT(0), None, BOB, 150, 0));
        assert_noop!(NftAuctionModule::bid(participant.clone(), 0, 200), Error::<Runtime>::InvalidAuctionType);
        assert_ok!(NftAuctionModule::create_auction(AuctionType::Auction, ItemId::NFT(0), None, BOB, 150, 0));
        assert_noop!(NftAuctionModule::buy_now(participant.clone(), 1, 150),Error::<Runtime>::InvalidAuctionType);
    });
}

