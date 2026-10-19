#![no_std]

//! Prism USTRY hodl strategy for the DeFindex vault framework.
//!
//! Single-asset and oracle-free: the strategy only holds Etherfuse USTRY.
//! Compounding is the issuer redemption NAV, so `harvest` is a no-op.

use soroban_sdk::{
    contract, contracterror, contractimpl, contracttype, symbol_short, token, Address, Bytes, Env,
    Symbol, Val, Vec,
};

const DAY_LEDGERS: u32 = 17280;
const INSTANCE_BUMP: u32 = DAY_LEDGERS * 30;
const INSTANCE_THRESHOLD: u32 = DAY_LEDGERS * 1;
const BALANCE_BUMP: u32 = DAY_LEDGERS * 30;
const BALANCE_THRESHOLD: u32 = DAY_LEDGERS * 1;

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum StrategyError {
    NotInitialized = 401,
    NegativeNotAllowed = 410,
    InsufficientBalance = 412,
}

#[contracttype]
#[derive(Clone)]
enum DataKey {
    Asset,
    Balance(Address),
}

#[contract]
pub struct UstryHodlStrategy;

fn extend_instance(e: &Env) {
    e.storage()
        .instance()
        .extend_ttl(INSTANCE_THRESHOLD, INSTANCE_BUMP);
}

fn asset(e: &Env) -> Result<Address, StrategyError> {
    e.storage()
        .instance()
        .get(&DataKey::Asset)
        .ok_or(StrategyError::NotInitialized)
}

fn read_balance(e: &Env, owner: &Address) -> i128 {
    let key = DataKey::Balance(owner.clone());
    e.storage().persistent().get(&key).unwrap_or(0)
}

fn write_balance(e: &Env, owner: &Address, amount: i128) {
    let key = DataKey::Balance(owner.clone());
    e.storage().persistent().set(&key, &amount);
    e.storage()
        .persistent()
        .extend_ttl(&key, BALANCE_THRESHOLD, BALANCE_BUMP);
}

fn require_positive(amount: i128) -> Result<(), StrategyError> {
    if amount < 0 {
        return Err(StrategyError::NegativeNotAllowed);
    }
    Ok(())
}

#[contractimpl]
impl UstryHodlStrategy {
    /// DeFindex constructor: `asset` is the USTRY SAC. `init_args` is unused.
    pub fn __constructor(e: Env, asset: Address, _init_args: Vec<Val>) {
        e.storage().instance().set(&DataKey::Asset, &asset);
        extend_instance(&e);
    }

    pub fn asset(e: Env) -> Result<Address, StrategyError> {
        extend_instance(&e);
        asset(&e)
    }

    /// Pull `amount` of USTRY from `from` (the vault) and credit its strategy balance.
    /// Returns the vault's remaining strategy balance.
    pub fn deposit(e: Env, amount: i128, from: Address) -> Result<i128, StrategyError> {
        require_positive(amount)?;
        from.require_auth();
        extend_instance(&e);

        let token = asset(&e)?;
        token::Client::new(&e, &token).transfer(&from, &e.current_contract_address(), &amount);

        let next = read_balance(&e, &from) + amount;
        write_balance(&e, &from, next);

        e.events().publish(
            (symbol_short!("deposit"), from.clone()),
            amount,
        );
        Ok(next)
    }

    /// USTRY yield is issuer NAV. Nothing to claim or swap.
    pub fn harvest(e: Env, from: Address, _data: Option<Bytes>) -> Result<(), StrategyError> {
        extend_instance(&e);
        e.events()
            .publish((Symbol::new(&e, "harvest"), from), 0i128);
        Ok(())
    }

    pub fn balance(e: Env, from: Address) -> Result<i128, StrategyError> {
        extend_instance(&e);
        Ok(read_balance(&e, &from))
    }

    /// Send `amount` of USTRY from the strategy to `to`. Returns remaining `from` balance.
    pub fn withdraw(e: Env, amount: i128, from: Address, to: Address) -> Result<i128, StrategyError> {
        require_positive(amount)?;
        from.require_auth();
        extend_instance(&e);

        let current = read_balance(&e, &from);
        if current < amount {
            return Err(StrategyError::InsufficientBalance);
        }
        let next = current - amount;
        write_balance(&e, &from, next);

        let token = asset(&e)?;
        token::Client::new(&e, &token).transfer(&e.current_contract_address(), &to, &amount);

        e.events()
            .publish((symbol_short!("withdraw"), from.clone()), amount);
        Ok(next)
    }
}

#[cfg(test)]
mod test;
