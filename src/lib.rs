/*
RUSTFLAGS='-C link-arg=-s' cargo build --target wasm32-unknown-unknown --release
near login
near deploy --wasmFile ./target/wasm32-unknown-unknown/release/zokshpay-near-admin.wasm --accountId mooadmin.testnet
near call admin.testnet new --accountId admin.testnet --args '{"admin_id": "admin.testnet"}'
*/

use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
//use near_sdk::collections::LookupSet;
use near_sdk::serde::Deserialize;
use near_sdk::{env, log, near_bindgen, AccountId, PanicOnDefault};
use serde::Serialize;

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault, Deserialize, Serialize)]
pub struct MoopayAdmin {
    admin_list: Vec<AccountId>,
    manager_list: Vec<AccountId>,
}

#[near_bindgen]
impl MoopayAdmin {
    /* Initialize the contract */
    #[init]
    pub fn new(admin_id: String, manager_id: String) -> Self {
        let admin_id = AccountId::new_unchecked(admin_id);
        let manager_id = AccountId::new_unchecked(manager_id);
        let mut admin_list = Vec::new();
        let mut manager_list = Vec::new();
        admin_list.push(admin_id);
        manager_list.push(manager_id);

        log!("Admin List initialized");
        Self {
            admin_list: admin_list,
            manager_list: manager_list,
        }
    }

    pub fn is_admin(&self, admin_id: AccountId) -> bool {
        self.admin_list.contains(&admin_id)
    }

    pub fn add_admin(&mut self, admin_id: AccountId) {
        if !self.is_admin(env::predecessor_account_id()) {
            log!("Not Authorized. Please contact Admin !!!");
            return;
        }

        if self.is_admin(admin_id.clone()) {
            log!("{} already an admin.", &admin_id);
        } else {
            self.admin_list.push(admin_id.clone());
            log!("{} added as admin.", &admin_id);
        }
    }

    pub fn remove_admin(&mut self, admin_id: AccountId) {
        if !self.is_admin(env::predecessor_account_id()) {
            log!("Not Authorized. Please contact Admin !!!");
            return;
        }

        if !self.is_admin(admin_id.clone()) {
            log!("{} not an admin.", &admin_id);
        } else {
            let x = self
                .admin_list
                .iter()
                .position(|x| *x == admin_id)
                .unwrap_or(9999);
            if x != 9999 {
                self.admin_list.remove(x);
            }
            log!("{} removed as admin.", &admin_id);
        }
    }

    pub fn is_manager(&self, manager_id: AccountId) -> bool {
        self.manager_list.contains(&manager_id)
    }

    pub fn add_manager(&mut self, manager_id: AccountId) {
        if !self.is_admin(env::predecessor_account_id()) {
            log!("Not Authorized. Please contact Admin !!!");
            return;
        }

        if self.is_manager(manager_id.clone()) {
            log!("{} already a manager.", &manager_id);
        } else {
            self.manager_list.push(manager_id.clone());
            log!("{} added as manager.", &manager_id);
        }
    }

    pub fn remove_manager(&mut self, manager_id: AccountId) {
        if !self.is_admin(env::predecessor_account_id()) {
            log!("Not Authorized. Please contact Admin !!!");
            return;
        }

        if !self.is_manager(manager_id.clone()) {
            log!("{} not a manager.", &manager_id);
        } else {
            let x = self
                .manager_list
                .iter()
                .position(|x| *x == manager_id)
                .unwrap_or(9999);
            if x != 9999 {
                self.manager_list.remove(x);
            }
            log!("{} removed as manager.", &manager_id);
        }
    }
}
/*
#[cfg(test)]
mod tests {
    use super::*;
    use near_sdk::test_utils::VMContextBuilder;
    use near_sdk::testing_env;

    fn get_context(predecessor: AccountId) -> VMContextBuilder {
        let mut builder = VMContextBuilder::new();
        builder.predecessor_account_id(predecessor);
        builder
    }

    #[test]
    fn check_is_admin() {
        // Basic set up for a unit test
        let alice = AccountId::new_unchecked("alice.testnet".to_string());
        // Set up the testing context and unit test environment
        let context = get_context(alice.clone());
        testing_env!(context.build());

        let admin = MoopayAdmin::new(alice.clone());

        assert!(
            admin.is_admin(alice),
            "The admin_list should have contained alice."
        )
    }

    #[test]
    fn check_add_admin() {
        // Basic set up for a unit test
        let alice = AccountId::new_unchecked("alice.testnet".to_string());
        // Set up the testing context and unit test environment
        let context = get_context(alice.clone());
        testing_env!(context.build());

        let mut admin = MoopayAdmin::new(alice);
        let new_admin = AccountId::new_unchecked("new_admin.testnet".to_string());
        admin.add_admin(new_admin.clone());
        assert!(
            admin.admin_list.contains(&new_admin),
            "The admin_list should have contained new_admin."
        );
        assert!(
            admin.is_admin(new_admin),
            "new_admin should have been added as admin."
        );
    }

        #[test]
        fn check_add_manager() {
            // Basic set up for a unit test
            let alice = AccountId::new_unchecked("alice.testnet".to_string());
            // Set up the testing context and unit test environment
            let context = get_context(alice.clone());
            testing_env!(context.build());

            let mut admin = MoopayAdmin::new(alice);
            let new_admin = AccountId::new_unchecked("new_admin.testnet".to_string());
            admin.add_manager(new_admin.clone());
            assert!(
                admin.manager_list.contains(&new_admin),
                "The manager_list should have contained new_admin."
            );
            assert!(
                admin.is_manager(new_admin),
                "new_admin should have been added as manager."
            );
        }

    #[test]
    fn check_remove_admin() {
        // Basic set up for a unit test
        let alice = AccountId::new_unchecked("alice.testnet".to_string());
        // Set up the testing context and unit test environment
        let context = get_context(alice.clone());
        testing_env!(context.build());

        let mut admin = MoopayAdmin::new(alice);
        let new_admin = AccountId::new_unchecked("new_admin.testnet".to_string());
        admin.add_admin(new_admin.clone());
        assert!(
            admin.admin_list.contains(&new_admin),
            "The admin_list should have contained new_admin."
        );
        admin.remove_admin(new_admin.clone());
        assert!(
            !admin.admin_list.contains(&new_admin),
            "The admin_list should not have contained new_admin."
        );
        assert!(
            !admin.is_admin(new_admin),
            "new_admin should have been removed as admin."
        );
    }
}*/
