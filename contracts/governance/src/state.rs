use cosmwasm_std::{StdResult, Storage, Addr};
use cw_storage_plus::{Item, Map};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};


// cs-storage-plus docs: https://crates.io/crates/cw-storage-plus

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Config {
    //TODO contract admin, voting settings
    //pub name : String, // Could possibly be useful for identifying multiple issues to vote on in frontend? dunno
    pub admin : Addr,
    pub min_votes : u32,
    pub percentage : u8,
    pub cur_votes : u32,
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
    CONFIG.load(storage)
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

// ===============
// Whitelist Helper Functions
// ===============
pub fn set_whitelist_status(storage: &mut dyn Storage, addr: &[u8], status: bool`) -> StdResult<()> {
    return WHITELIST.save(storage, addr, &status);
}

pub fn get_whitelist_status(storage: &dyn Storage, addr: &[u8], status: bool`) -> bool {
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