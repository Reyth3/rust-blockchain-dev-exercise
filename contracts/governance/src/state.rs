use cosmwasm_std::{StdResult, Storage, Addr, Order};
use cw_storage_plus::{Item, Map};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};


// cs-storage-plus docs: https://crates.io/crates/cw-storage-plus

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Config {
    //TODO contract admin, voting settings
    //pub name : String, // Could possibly be useful for identifying multiple issues to vote on in frontend? dunno
    pub admin : Addr,
    pub ongoing : bool, // this flag will determine whether the result can be shown conclusively
    pub min_votes : u32,
    pub percentage : u8,
    pub cur_votes : u32,
}

pub struct VoteSummary {
    pub for_count : u32,
    pub against_count : u32,
    pub abstain_count : u32,
    pub for_percentage : u8,
    pub against_percentage : u8,
}

const CONFIG: Item<Config> = Item::new("\u{0}\u{6}config"); 
const VOTES: Map<&[u8], i8> = Map::new("votes");
const WHITELIST: Map<&[u8], bool> = Map::new("whitelist");

// ===============
// Config Helper Functions
// ===============
pub fn store_config(storage: &mut dyn Storage, config: &Config) -> StdResult<()> {
    CONFIG.save(storage, config)
}

pub fn read_config(storage: &dyn Storage) -> StdResult<Config> {
    CONFIG.load(storage);
}

// ===============
// Votes Helper Functions
// ===============
pub fn already_voted(storage: &dyn Storage, addr: &[u8]) -> bool {
    return VOTES.has(storage, addr);
}

pub fn cast_vote(storage: &mut dyn Storage, addr: &[u8], vote: i8) -> StdResult<()> {
    return VOTES.save(storage, addr, &vote);
}

pub fn count_votes(storage : &dyn Storage) -> StdResult<VoteSummary>
{
    let fc : u32 = 0;
    let ac : u32 = 0;
    let abc : u32 = 0;


    /* Okay let's scrap that
    // loosely inspired by this production ready cw20 allowances enum impl: https://github.com/Papi94/cosmwasm-plus/blob/main/contracts/cw20-base/src/enumerable.rs
    VOTES.range(storage, None, None, Order::Ascending).enumerate().for_each(|(i, x)| {
        //let r : StdResult<(Vec<u8>, i8)> = x;
        // let (k,v) = r?; <- errors out so let's try another thing
        match x {
            Ok(e) => {
                let (k,v) = e; // Somehow this seems to have worked, while 'r?' straight up to get the tuple didn't. Is it because the for_each() is considered a scope hee, not the function, and foreach doesn't return a Result<T>?
                // I'd rather have this look a bit more ugly than try shortcuts and mess something up in the process tbh
                if v == -1 { // Against
                    ac += 1;
                }
                else if v == 0 { // Abstain
                    abc += 1;
                }
                else if v == 1 {
                    fc += 1;
                }
            },
            Err(e) => (),
        }

        
    }); */

    // This is the new way.
    let enumerate = VOTES.range(storage, None, None, Order::Ascending).enumerate();
    for vote in enumerate {
        let (i, item) = vote;
        let (k,v) = item?;
        // I'd rather have this look a bit more ugly than try shortcuts and mess something up in the process tbh
        if v == -1 { // Against
            ac += 1;
        }
        else if v == 0 { // Abstain
            abc += 1;
        }
        else if v == 1 {
            fc += 1;
        }
    }

    let cfg = read_config(storage)?;
    let fp = fc * 100 / cfg.cur_votes;
    let ap = ac * 100 / cfg.cur_votes;

    return Ok(VoteSummary {
        for_count : fc,
        abstain_count : abc,
        against_count : ac,
        for_percentage : fp as u8,
        against_percentage : ap as u8,
    });
}

// ===============
// Whitelist Helper Functions
// ===============
pub fn set_whitelist_status(storage: &mut dyn Storage, addr: &[u8], status: bool) -> StdResult<()> {
    return WHITELIST.save(storage, addr, &status);
}

pub fn get_whitelist_status(storage: &dyn Storage, addr: &[u8]) -> bool {
    let result = WHITELIST.may_load(storage, addr);
    match result {
        Ok(x) => {
            match x {
                Some(x) => return x,
                None => return false
            }
        },
        Err(x) => return false
    }
}