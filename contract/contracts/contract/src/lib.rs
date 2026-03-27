#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, Address, Env};

#[contracttype]
#[derive(Clone)]
pub struct Property {
    pub id: u32,
    pub owner: Address,
    pub total_tokens: u32,
}

#[contracttype]
pub enum DataKey {
    Property(u32),
    Balance(Address, u32), // (user, property_id)
}

// ✅ FIX: Add this macro
#[contract]
pub struct RealEstateToken;

#[contractimpl]
impl RealEstateToken {

    // Create a new property and assign tokens to owner
    pub fn create_property(env: Env, owner: Address, property_id: u32, total_tokens: u32) {
        owner.require_auth();

        let property = Property {
            id: property_id,
            owner: owner.clone(),
            total_tokens,
        };

        env.storage().instance().set(&DataKey::Property(property_id), &property);

        // Assign all tokens to owner
        env.storage().instance().set(&DataKey::Balance(owner, property_id), &total_tokens);
    }

    // Transfer tokens between users
    pub fn transfer(
        env: Env,
        from: Address,
        to: Address,
        property_id: u32,
        amount: u32,
    ) {
        from.require_auth();

        let mut from_balance: u32 = env
            .storage()
            .instance()
            .get(&DataKey::Balance(from.clone(), property_id))
            .unwrap_or(0);

        if from_balance < amount {
            panic!("Insufficient balance");
        }

        let mut to_balance: u32 = env
            .storage()
            .instance()
            .get(&DataKey::Balance(to.clone(), property_id))
            .unwrap_or(0);

        from_balance -= amount;
        to_balance += amount;

        env.storage().instance().set(&DataKey::Balance(from, property_id), &from_balance);
        env.storage().instance().set(&DataKey::Balance(to, property_id), &to_balance);
    }

    // Check balance
    pub fn balance(env: Env, user: Address, property_id: u32) -> u32 {
        env.storage()
            .instance()
            .get(&DataKey::Balance(user, property_id))
            .unwrap_or(0)
    }
}