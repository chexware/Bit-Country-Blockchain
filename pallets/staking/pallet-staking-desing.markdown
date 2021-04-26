# pallet-staking design:
* Calls
  - stake
   - origin: AccountId
   - country: CountryId
   - rewards_destination: RewardsDestination
   - stake_balance: Balance
  - unstake
    - origin: AccountId
    - country: CountryId
    - unstake_balance: Balance
  - reinvest_rewards - claim re-stake available rewards
    - origin: AccountId
  - claim_rewards 
    - origin: AccountId
* Storages
  - Rewards: AccountId => Balance
  - StakedBalances: double_map: (AccountId,CountryId) => Balance
  - UnstakeRequests: double_map: (BlockNumber,AccountId)  => Balance
  - EraEndTime: map: BlocknNumber, EraId => ()
  - EraIndex: EraId
* Types
* Events
  - EraPayout
    - current_era: EraIndex
  - StakingRewardClaimed
    - origin: AccountId
    - reward_balance: Balance
  - BalanceStaked
    - origin: AccountId
    - country: CountryId
    - staked_balance: Balance
  - BalanceUnstaked
    - origin: AccountId
    - country: CountryId
    - unstaked_balance: Balance
  - UnstakeRequestCompleted
    - origin: AccountId
    - unstaked_balance: Balance
  - BalanceReinvested
    - origin: AccountId
    - reinvested_balance: Balance
* Other functions
  - on_finalize
    - unreserve funds when unstake period finishes.
  - offchain worker
    - updates account rewards based on the account staked balance and the current reward multiplier
  
  