#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, symbol_short, Address, Env, Vec, Symbol, panic_with_error};

#[contracttype]
#[derive(Clone)]
pub enum DataKey {
    Members,
    PayoutOrder,
    CurrentRound,
    TotalRounds,
}

#[contracttype]
#[derive(Clone)]
pub struct PayoutOrderCompacted {
    pub removed_member: Address,
    pub new_length: u32,
}

#[contract]
pub struct Contract;

#[contractimpl]
impl Contract {
    pub fn approve_exit(env: Env, member: Address) {
        member.require_auth();

        let mut members: Vec<Address> = env.storage().instance()
            .get(&DataKey::Members)
            .unwrap_or_else(|| Vec::new(&env));

        let mut payout_order: Vec<Address> = env.storage().instance()
            .get(&DataKey::PayoutOrder)
            .unwrap_or_else(|| Vec::new(&env));

        // Remove from Members
        if let Some(pos) = members.iter().position(|m| *m == member) {
            members.remove(pos as u32);
        } else {
            panic_with_error!(&env, Error::NoExitRequestFound);
        }

        // Remove from PayoutOrder and compact the gap (fixes #389)
        if let Some(pos) = payout_order.iter().position(|m| *m == member) {
            payout_order.remove(pos as u32);
            env.storage().instance().set(&DataKey::PayoutOrder, &payout_order);
        }

        // Save updated members
        env.storage().instance().set(&DataKey::Members, &members);

        // Emit required event
        env.events().publish(
            (symbol_short!("PayoutOrder"), symbol_short!("Compacted")),
            PayoutOrderCompacted {
                removed_member: member,
                new_length: payout_order.len() as u32,
            },
        );
    }
}

#[derive(Clone)]
#[contracttype]
pub enum Error {
    NoExitRequestFound,
}
