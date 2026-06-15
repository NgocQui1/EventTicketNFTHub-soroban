#![no_std]

use soroban_sdk::{
    contract, contractimpl, contracttype,
    Address, Env, Symbol, Vec,
};

#[derive(Clone)]
#[contracttype]
pub struct Ticket {
    pub owner: Address,
    pub event_id: u32,
    pub used: bool,
    pub revoked: bool,
}

#[derive(Clone)]
#[contracttype]
pub enum DataKey {
    Admin,
    Ticket(u32),
    NextTicketId,
}

#[contract]
pub struct EventTicketContract;

#[contractimpl]
impl EventTicketContract {

    // Initialize contract
    pub fn initialize(env: Env, admin: Address) {
        admin.require_auth();

        env.storage()
            .instance()
            .set(&DataKey::Admin, &admin);

        env.storage()
            .instance()
            .set(&DataKey::NextTicketId, &1u32);
    }

    // Mint NFT Ticket
    pub fn mint_ticket(
        env: Env,
        to: Address,
        event_id: u32,
    ) -> u32 {

        let admin: Address = env
            .storage()
            .instance()
            .get(&DataKey::Admin)
            .unwrap();

        admin.require_auth();

        let mut next_id: u32 = env
            .storage()
            .instance()
            .get(&DataKey::NextTicketId)
            .unwrap();

        let ticket = Ticket {
            owner: to.clone(),
            event_id,
            used: false,
            revoked: false,
        };

        env.storage()
            .persistent()
            .set(&DataKey::Ticket(next_id), &ticket);

        env.storage()
            .instance()
            .set(&DataKey::NextTicketId, &(next_id + 1));

        next_id
    }

    // Transfer ticket
    pub fn transfer_ticket(
        env: Env,
        from: Address,
        to: Address,
        ticket_id: u32,
    ) {

        from.require_auth();

        let key = DataKey::Ticket(ticket_id);

        let mut ticket: Ticket = env
            .storage()
            .persistent()
            .get(&key)
            .unwrap();

        if ticket.revoked {
            panic!("Ticket revoked");
        }

        if ticket.used {
            panic!("Ticket already used");
        }

        if ticket.owner != from {
            panic!("Not owner");
        }

        ticket.owner = to;

        env.storage()
            .persistent()
            .set(&key, &ticket);
    }

    // Check in event
    pub fn check_in(
        env: Env,
        ticket_id: u32,
    ) {

        let key = DataKey::Ticket(ticket_id);

        let mut ticket: Ticket = env
            .storage()
            .persistent()
            .get(&key)
            .unwrap();

        if ticket.revoked {
            panic!("Ticket revoked");
        }

        if ticket.used {
            panic!("Already checked in");
        }

        ticket.used = true;

        env.storage()
            .persistent()
            .set(&key, &ticket);
    }

    // Clawback ticket
    pub fn clawback_ticket(
        env: Env,
        ticket_id: u32,
    ) {

        let admin: Address = env
            .storage()
            .instance()
            .get(&DataKey::Admin)
            .unwrap();

        admin.require_auth();

        let key = DataKey::Ticket(ticket_id);

        let mut ticket: Ticket = env
            .storage()
            .persistent()
            .get(&key)
            .unwrap();

        ticket.revoked = true;

        env.storage()
            .persistent()
            .set(&key, &ticket);
    }

    // Query ticket
    pub fn get_ticket(
        env: Env,
        ticket_id: u32,
    ) -> Ticket {

        env.storage()
            .persistent()
            .get(&DataKey::Ticket(ticket_id))
            .unwrap()
    }
}