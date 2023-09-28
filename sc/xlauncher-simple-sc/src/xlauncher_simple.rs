#![no_std]

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
                             initial_price: BigUint) {
        require!(
            token_id.is_valid_esdt_identifier(),
            "Invalid token identifier"
        );
        require!(initial_price > 0, "Initial price must be positive value");

        self.token_id().set(&token_id);
        self.price().set(initial_price);

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

    #[payable("EGLD")]
    #[endpoint]
    fn buy(&self) {
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
}
