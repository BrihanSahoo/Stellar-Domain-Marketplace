#![no_std]

use soroban_sdk::{
    contract, contractimpl, contracttype,
    Env, String, Address
};

#[contract]
pub struct DomainMarketplace;

#[contracttype]
#[derive(Clone)]
pub struct Domain {
    pub owner: Address,
    pub price: i128,
    pub for_sale: bool,
}

#[contracttype]
pub enum DataKey {
    Domain(String),
}

#[contractimpl]
impl DomainMarketplace {

    // 🔹 Register a new domain
    pub fn register_domain(env: Env, domain: String, owner: Address, price: i128) {
        owner.require_auth(); // 🔐 security

        let key = DataKey::Domain(domain.clone());

        if env.storage().persistent().has(&key) {
            panic!("Domain already exists");
        }

        let domain_data = Domain {
            owner,
            price,
            for_sale: true,
        };

        env.storage().persistent().set(&key, &domain_data);
    }

    // 🔹 Buy a domain
    pub fn buy_domain(env: Env, domain: String, buyer: Address) {
        buyer.require_auth(); // 🔐 security

        let key = DataKey::Domain(domain.clone());

        let mut d: Domain = env
            .storage()
            .persistent()
            .get(&key)
            .unwrap_or_else(|| panic!("Domain not found"));

        if !d.for_sale {
            panic!("Domain not for sale");
        }

        // ⚠️ Payment logic should be added here (token transfer)

        d.owner = buyer;
        d.for_sale = false;

        env.storage().persistent().set(&key, &d);
    }

    // 🔹 List domain for sale
    pub fn list_domain(env: Env, domain: String, owner: Address, price: i128) {
        owner.require_auth(); // 🔐 security

        let key = DataKey::Domain(domain.clone());

        let mut d: Domain = env
            .storage()
            .persistent()
            .get(&key)
            .unwrap_or_else(|| panic!("Domain not found"));

        if d.owner != owner {
            panic!("Not the owner");
        }

        d.price = price;
        d.for_sale = true;

        env.storage().persistent().set(&key, &d);
    }

    // 🔹 Get domain details
    pub fn get_domain(env: Env, domain: String) -> Domain {
        let key = DataKey::Domain(domain);

        env.storage()
            .persistent()
            .get(&key)
            .unwrap_or_else(|| panic!("Domain not found"))
    }
}