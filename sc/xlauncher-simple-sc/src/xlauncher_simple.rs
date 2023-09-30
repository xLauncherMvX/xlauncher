#![no_std]

use crate::endpoints::start_timestamp;
multiversx_sc::imports!();
multiversx_sc::derive_imports!();

const EGLD_DECIMALS_VALUE: u64 = 1_000_000_000_000_000_000;

#[multiversx_sc::contract]
pub trait XlauncherSimple {
    #[init]
    fn init(&self) {}

    #[only_owner]
    #[endpoint(setContractSettings)]
    fn set_contract_settings(&self,
                             token_id: TokenIdentifier,
                             initial_price: BigUint,
                             start_stamp_val: u64,
    ) {
        require!(
            token_id.is_valid_esdt_identifier(),
            "Invalid token identifier"
        );
        require!(initial_price > 0, "Initial price must be positive value");
        require!(start_stamp_val >= 0_u64, "Start time stamp must be grater then zero");

        self.token_id().set(&token_id);
        self.price().set(initial_price);
        self.start_timestamp().set(start_stamp_val);
    }

    #[payable("*")]
    #[endpoint(fundContract)]
    fn fund_contract(
        &self,
        #[payment_token] token_identifier: EgldOrEsdtTokenIdentifier,
        #[payment] _payment: BigUint,
    ) {
        require!(!self.token_id().is_empty(), "Token id not set");
        let my_token_id = self.token_id().get();
        require!(my_token_id == token_identifier, "Invalid fund token")
    }

    #[only_owner]
    #[endpoint]
    fn collect(&self) {
        let owner = self.blockchain().get_owner_address();

        let sc_address: ManagedAddress = self.blockchain().get_sc_address();
        let egld_balance = self.blockchain().get_balance(&sc_address);

        let my_token_id = self.token_id().get();
        let egld_or_esdt_token_identifier = EgldOrEsdtTokenIdentifier::esdt(my_token_id.clone());
        let token_balance = self.blockchain().get_sc_balance(&egld_or_esdt_token_identifier, 0);

        let big_zero: BigUint = BigUint::zero();
        if big_zero < token_balance {
            self.send()
                .direct_esdt(&owner, &my_token_id, 0, &token_balance);
        }

        if big_zero < egld_balance {
            self.send().direct_egld(&owner, &egld_balance)
        }
    }

    #[payable("EGLD")]
    #[endpoint]
    fn buy(&self) {
        let current_time_stamp: u64 = self.blockchain().get_block_timestamp();
        let start_time_stamp = self.start_timestamp().get();
        require!(start_time_stamp <= current_time_stamp, "Start time bigger then current time");

        let egld_or_esdt_token_identifier = self.call_value().egld_or_single_esdt();
        let payment_token = egld_or_esdt_token_identifier.token_identifier;
        let payment_amount = egld_or_esdt_token_identifier.amount;

        require!(payment_token.is_egld(), "Only EGLD");

        let current_price = self.price().get();
        let one_egld = BigUint::from(EGLD_DECIMALS_VALUE);
        let result_esdt_token_amount = (&current_price * &payment_amount) / &one_egld;

        let balance = self.get_token_balance();
        require!(
            balance >= result_esdt_token_amount,
            "Not enough tokens for sale."
        );

        let token_id_val = self.token_id().get();
        let caller = self.blockchain().get_caller();
        self.send()
            .direct_esdt(&caller, &token_id_val, 0, &result_esdt_token_amount);
    }

    #[view(getTokenBalance)]
    fn get_token_balance(&self) -> BigUint {
        let my_token_id = self.token_id().get();

        let egld_or_esdt_token_identifier = EgldOrEsdtTokenIdentifier::esdt(my_token_id);
        let balance: BigUint = self.blockchain().get_sc_balance(&egld_or_esdt_token_identifier, 0);
        return balance;
    }

    // storage
    #[view(getTokenId)]
    #[storage_mapper("tokenId")]
    fn token_id(&self) -> SingleValueMapper<TokenIdentifier>;

    #[view(getPrice)]
    #[storage_mapper("price")]
    fn price(&self) -> SingleValueMapper<BigUint>;

    #[view(getStartTimestamp)]
    #[storage_mapper("startTimestamp")]
    fn start_timestamp(&self) -> SingleValueMapper<u64>;
}
