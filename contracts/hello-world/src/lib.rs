#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, token, Address, Env};

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Stream {
    pub employer: Address,
    pub employee: Address,
    pub token: Address,
    pub total_amount: i128,
    pub rate_per_second: i128,
    pub start_time: u64,
    pub stop_time: u64,
    pub withdrawn_amount: i128,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum DataKey {
    StreamInfo,
    AccumulatedYield,
}

#[contract]
pub struct StreamPayContract;

#[contractimpl]
impl StreamPayContract {

    pub fn create_stream(
        env: Env,
        employer: Address,
        employee: Address,
        token: Address,
        amount: i128,
        duration: u64,
    ) {
        employer.require_auth();

        let start_time = env.ledger().timestamp();
        let stop_time = start_time + duration;
        let rate_per_second = amount / (duration as i128);

        let client = token::Client::new(&env, &token);
        client.transfer(&employer, &env.current_contract_address(), &amount);

        let stream = Stream {
            employer,
            employee,
            token,
            total_amount: amount,
            rate_per_second,
            start_time,
            stop_time,
            withdrawn_amount: 0i128, // Đã sửa thành 0i128
        };

        env.storage().instance().set(&DataKey::StreamInfo, &stream);
        env.storage().instance().set(&DataKey::AccumulatedYield, &0i128); // Đã sửa thành 0i128
    }

    pub fn balance_of(env: Env) -> i128 {
        let stream: Stream = env.storage().instance().get(&DataKey::StreamInfo).unwrap();
        let current_time = env.ledger().timestamp();

        if current_time <= stream.start_time {
            return 0i128; // Đã sửa thành 0i128
        }

        let elapsed = if current_time >= stream.stop_time {
            stream.stop_time - stream.start_time
        } else {
            current_time - stream.start_time
        };

        let total_earned = (elapsed as i128) * stream.rate_per_second;
        
        total_earned - stream.withdrawn_amount
    }

    pub fn withdraw(env: Env) {
        let mut stream: Stream = env.storage().instance().get(&DataKey::StreamInfo).unwrap();
        stream.employee.require_auth();

        let claimable = Self::balance_of(env.clone());
        if claimable > 0 {
            stream.withdrawn_amount += claimable;
            env.storage().instance().set(&DataKey::StreamInfo, &stream);

            let client = token::Client::new(&env, &stream.token);
            client.transfer(&env.current_contract_address(), &stream.employee, &claimable);

            let idle_balance = stream.total_amount - stream.withdrawn_amount;
            
            let mock_yield = (idle_balance * 5i128) / 100i128 / 365i128 / 24i128 / 3600i128;
            
            let mut current_yield: i128 = env.storage().instance().get(&DataKey::AccumulatedYield).unwrap_or(0i128);
            current_yield += mock_yield;
            env.storage().instance().set(&DataKey::AccumulatedYield, &current_yield);
        }
    }

    pub fn claim_yield(env: Env) -> i128 {
        let stream: Stream = env.storage().instance().get(&DataKey::StreamInfo).unwrap();
        stream.employer.require_auth();

        let yield_amount: i128 = env.storage().instance().get(&DataKey::AccumulatedYield).unwrap_or(0i128);
        
        if yield_amount > 0 {
            env.storage().instance().set(&DataKey::AccumulatedYield, &0i128);
        }
        yield_amount
    }
}