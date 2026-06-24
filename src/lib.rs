#![no_std]

use soroban_sdk::{
    contract, contractimpl, contracttype,
    symbol_short, Address, Env,
};

#[derive(Clone)]
#[contracttype]
pub enum DataKey {
    Counter,
    Transfer(u64),
}

#[derive(Clone)]
#[contracttype]
pub enum TransferStatus {
    Pending,
    Claimed,
}

#[derive(Clone)]
#[contracttype]
pub struct Transfer {
    pub id: u64,
    pub sender: Address,
    pub receiver: Address,
    pub amount: i128,
    pub status: TransferStatus,
}

#[contract]
pub struct RemittanceContract;

#[contractimpl]
impl RemittanceContract {

    // Tạo giao dịch chuyển tiền
    pub fn create_transfer(
        env: Env,
        sender: Address,
        receiver: Address,
        amount: i128,
    ) -> u64 {

        sender.require_auth();

        let mut counter: u64 = env
            .storage()
            .persistent()
            .get(&DataKey::Counter)
            .unwrap_or(0);

        counter += 1;

        let transfer = Transfer {
            id: counter,
            sender: sender.clone(),
            receiver: receiver.clone(),
            amount,
            status: TransferStatus::Pending,
        };

        env.storage()
            .persistent()
            .set(&DataKey::Transfer(counter), &transfer);

        env.storage()
            .persistent()
            .set(&DataKey::Counter, &counter);

        env.events().publish(
            (symbol_short!("create"), counter),
            amount,
        );

        counter
    }

    // Người nhận xác nhận đã nhận tiền
    pub fn claim_transfer(
        env: Env,
        transfer_id: u64,
        receiver: Address,
    ) {

        receiver.require_auth();

        let key = DataKey::Transfer(transfer_id);

        let mut transfer: Transfer = env
            .storage()
            .persistent()
            .get(&key)
            .unwrap();

        if transfer.receiver != receiver {
            panic!("Not receiver");
        }

        match transfer.status {
            TransferStatus::Claimed => {
                panic!("Already claimed");
            }
            TransferStatus::Pending => {}
        }

        transfer.status = TransferStatus::Claimed;

        env.storage()
            .persistent()
            .set(&key, &transfer);

        env.events().publish(
            (symbol_short!("claim"), transfer_id),
            transfer.amount,
        );
    }

    // Xem giao dịch
    pub fn get_transfer(
        env: Env,
        transfer_id: u64,
    ) -> Transfer {

        env.storage()
            .persistent()
            .get(&DataKey::Transfer(transfer_id))
            .unwrap()
    }

    // Tổng số giao dịch
    pub fn get_counter(env: Env) -> u64 {

        env.storage()
            .persistent()
            .get(&DataKey::Counter)
            .unwrap_or(0)
    }
}