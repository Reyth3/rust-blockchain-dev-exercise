# Rust developer exercise

## Summary

This is my solution for the governance/voting smart contract exercise for the Terra blockchain. I implemented a voting system with an optional whitelist, but not based around cw20 tokens.

### Implementation

The idea behind the implementation of the governance system is as follows:

1. An admin can instantiate a poll with minimum requirements for resolution.
2. Only the admin can whitelist participant addresses.
3. The proposal doesn't have a start or end block - instead, the voting starts at the time of instantiation and can be ended by the admin at any point in time provided the min requirements are met.
4. Every whitelisted address can vote exactly once and can't change their vote in the future.
5. Once the poll is closed, there's 

### Conslusions and considerations

* The way I decided to implement the poll settlement is by making it possible to only close the voting after the required conditions have been met, ie. there's no scenario in which the poll ends on an unresolved note, which looking back on the code might've been a mistake as it can lead to a situation where a poll may never be closed. One solution to that is adding a control parameter `ensure_settlement` to the `ExecuteMsg::Close` call which, when set to `false`, will bypass all the requirement checks which can later be handled in the `GetStatus` query call.
* I realize I have overcomplicated the code a bit by declaring the state in the way I did - particularly with how I am referencing `state.rs` functions from the inner packages, which I feel like should be considered the lowest on the dependency graph. So referencing top-level modules like `state` only complicates the dependencies. I wasn't able to figure out a cleaner way to do it, though.
* In retrospect I feel like I could've thought this out better, but given the fact that before this exercise my Rust knowledge was practically non-existant, I feel like I did well. I learned a ton of things along the way and grew to appreciate some of the quirks of Rust I haven't seen used in other programming languages. 

## Exercise

1. Admin contract creates voting with parameters (minimum votes, percentage number of votes required to settlement).
2. Each address has only 1 vote -> For / Against / Abstain.
3. Summary for e.g rejected, accepted, not resolved with stats how much voted For / Against / Abstain.
4. [optional] Whitelist for addresses which can participate in voting.
5. [optional] Voting based on some cw20 token balance. 
 