# pallet-staking design:
* Calls
  - stake
   - origin: AccountId
   - country: CountryId
   - stake_balance: Balance
  - unstake
    - origin: AccountId
    - country: CountryId
    - unstake_balance: Balance
  - claim_rewards 
    - origin: AccountId
    - country: CountryId
  - reinvest_rewards - claim + re-stake available rewards
    - origin: AccountId
    - country: CountryId
* Storages
  - Rewards: (AccountId, ) => Balance
  - StakedBalances: double_map: (AccountId,CountryId) => Balance
  - TotalStakedBalances: map AccountId => Balance
  - UnstakeRequests: double_map: (BlockNumber,AccountId)  => Balance
  - EraEndTime: double_map: (BlocknNumber, EraId) => RewardMultiplier
  - EraIndex: EraId
* Types
* Events
  - EraPayout
    - current_era: EraIndex
  - StakingRewardClaimed
    - origin: AccountId
    - country: CountryId
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
    - country: CountryId
    - reinvested_balance: Balance
* Other functions
  - on_finalize
    - unreserve funds when unstake period finishes.
  - offchain_worker
    - updates account rewards balances based on the account staked balance and the current era reward multiplier
  
  