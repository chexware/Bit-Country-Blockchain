# pallet-staking design:
* Calls
  - stake
   - origin: AccountId
   - stake_balance: Balance
  - unstake
    - origin: AccountId
    - unstake_balance: Balance
  - reinvest_rewards - re-stake available rewards
    - origin: AccountId
  - claim_rewards 
    - origin: AccountId
* Storages
  - Payee: map: AccountId => Option(AccountId)
  - Rewards: AccountId => Option(Balance)
  - StakedBalance: map: AccountId => Option(Balance)
  - LockedBalance: map: AccountId => Option(Balance)
  - EraIndex: u32 
  - EraRewardMultiplier: f64 - number between 0 and 1 - could be updated via  governance
  - UnstakeLockPeriod: u32 - could be updated via governance
* Types
  - enum RewardType
    - Inflationary
    - Deflationary
  - enum  RewardDestination
  	- Staked,
  	- Account(AccountId),
  	- None
* Events
  - EraPayout
    - current_era: EraIndex
  - Reward
    - origin: AccountId
    - reward_balance: Balance
  - Stake
    - origin: AccountId
    - staked_balance: Balance
  - Unstake
    - origin: AccountId
    - unstaked_balance: Balance
  - Reinvest
    - origin: AccountId
    - reinvested_balance: Balance

* Other functions
  - unlock_funds 
    - unlocks unstaked funds when  the last block of the lock period is finalised.
  - distribute_rewards
    - triggered via offchain worker
    - updates account rewards based on the account staked balance and the current reward multiplier
  