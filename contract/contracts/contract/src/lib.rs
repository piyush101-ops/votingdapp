#![no_std]

use soroban_sdk::{contract, contractimpl, symbol_short, Symbol, Env, Vec, Map, String};

#[contract]
pub struct VotingContract;

#[contractimpl]
impl VotingContract {
    // Initialize candidates list
    pub fn init(env: Env, candidates: Vec<String>) {
        let mut votes: Map<String, u32> = Map::new(&env);

        for candidate in candidates.iter() {
            votes.set(candidate.clone(), 0);
        }

        env.storage().instance().set(&symbol_short!("votes"), &votes);
    }

    // Vote for a candidate
    pub fn vote(env: Env, candidate: String) {
        let key = symbol_short!("votes");
        let mut votes: Map<String, u32> = env.storage().instance().get(&key).unwrap();

        let count = votes.get(candidate.clone()).unwrap_or(0);
        votes.set(candidate, count + 1);

        env.storage().instance().set(&key, &votes);
    }

    // Get vote count for a candidate
    pub fn get_votes(env: Env, candidate: String) -> u32 {
        let votes: Map<String, u32> = env.storage().instance().get(&symbol_short!("votes")).unwrap();
        votes.get(candidate).unwrap_or(0)
    }

    // Get all results
    pub fn results(env: Env) -> Map<String, u32> {
        env.storage().instance().get(&symbol_short!("votes")).unwrap()
    }
}