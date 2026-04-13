#![no_std]
use soroban_sdk::{contract, contractimpl, Env, Symbol, symbol_short, Address, Map};

#[contract]
pub struct RoyaltyEngine;

#[contractimpl]
impl RoyaltyEngine {

    // Store royalty info: asset_id -> (creator, percentage)
    pub fn register_asset(env: Env, asset_id: Symbol, creator: Address, percentage: u32) {
        let mut royalties: Map<Symbol, (Address, u32)> =
            env.storage().instance().get(&symbol_short!("royalties")).unwrap_or(Map::new(&env));

        royalties.set(asset_id.clone(), (creator, percentage));

        env.storage().instance().set(&symbol_short!("royalties"), &royalties);
    }

    // Get royalty info
    pub fn get_royalty(env: Env, asset_id: Symbol) -> (Address, u32) {
        let royalties: Map<Symbol, (Address, u32)> =
            env.storage().instance().get(&symbol_short!("royalties")).unwrap();

        royalties.get(asset_id).unwrap()
    }

    // Calculate royalty amount from sale price
    pub fn calculate_royalty(env: Env, asset_id: Symbol, sale_price: i128) -> i128 {
        let (_, percentage) = Self::get_royalty(env, asset_id);
        sale_price * percentage as i128 / 100
    }
}