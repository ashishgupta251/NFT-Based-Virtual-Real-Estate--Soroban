#![no_std]
use soroban_sdk::{contract, contracttype, contractimpl, Env, Symbol, String, symbol_short, log};

#[contracttype]
#[derive(Clone)]
pub struct PropertyNFT {
    pub token_id: u64,
    pub owner: String,
    pub location: String,
    pub metadata_uri: String,
    pub is_listed: bool,
}

#[contracttype]
pub enum PropertyKey {
    Token(u64),
}

#[contracttype]
pub struct GlobalStats {
    pub total_properties: u64,
    pub listed_properties: u64,
}

const COUNT_KEY: Symbol = symbol_short!("COUNT");
const STATS_KEY: Symbol = symbol_short!("STATS");

#[contract]
pub struct VirtualRealEstate;

#[contractimpl]
impl VirtualRealEstate {
    pub fn mint_property(
        env: Env,
        owner: String,
        location: String,
        metadata_uri: String,
    ) -> u64 {
        let mut count: u64 = env.storage().instance().get(&COUNT_KEY).unwrap_or(0);
        count += 1;

        let nft = PropertyNFT {
            token_id: count,
            owner: owner.clone(),
            location,
            metadata_uri,
            is_listed: false,
        };

        env.storage().instance().set(&PropertyKey::Token(count), &nft);
        env.storage().instance().set(&COUNT_KEY, &count);

        let mut stats: GlobalStats = env.storage().instance().get(&STATS_KEY).unwrap_or(GlobalStats {
            total_properties: 0,
            listed_properties: 0,
        });

        stats.total_properties += 1;
        env.storage().instance().set(&STATS_KEY, &stats);

        env.storage().instance().extend_ttl(5000, 5000);

        log!(&env, "Minted Property Token-ID: {}", count);
        count
    }

    pub fn list_property(env: Env, token_id: u64) {
        let mut property: PropertyNFT = Self::view_property(env.clone(), token_id);

        if !property.is_listed {
            property.is_listed = true;

            env.storage().instance().set(&PropertyKey::Token(token_id), &property);

            let mut stats: GlobalStats = env.storage().instance().get(&STATS_KEY).unwrap();
            stats.listed_properties += 1;
            env.storage().instance().set(&STATS_KEY, &stats);

            log!(&env, "Property ID {} is now listed", token_id);
        } else {
            panic!("Property already listed!");
        }
    }

    pub fn unlist_property(env: Env, token_id: u64) {
        let mut property: PropertyNFT = Self::view_property(env.clone(), token_id);

        if property.is_listed {
            property.is_listed = false;

            env.storage().instance().set(&PropertyKey::Token(token_id), &property);

            let mut stats: GlobalStats = env.storage().instance().get(&STATS_KEY).unwrap();
            stats.listed_properties -= 1;
            env.storage().instance().set(&STATS_KEY, &stats);

            log!(&env, "Property ID {} is now unlisted", token_id);
        } else {
            panic!("Property already unlisted!");
        }
    }
    pub fn view_property(env: Env, token_id: u64) -> PropertyNFT {
        env.storage().instance().get(&PropertyKey::Token(token_id)).unwrap_or(PropertyNFT {
            token_id: 0,
            owner: String::from_str(&env, "Not Found"),
            location: String::from_str(&env, "Unknown"),
            metadata_uri: String::from_str(&env, "None"),
            is_listed: false,
        })
    }

    pub fn view_global_stats(env: Env) -> GlobalStats {
        env.storage().instance().get(&STATS_KEY).unwrap_or(GlobalStats {
            total_properties: 0,
            listed_properties: 0,
        })
    }
}