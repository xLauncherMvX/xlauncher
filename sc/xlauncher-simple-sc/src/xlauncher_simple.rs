#![no_std]

multiversx_sc::imports!();
multiversx_sc::derive_imports!();



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

    // storage
    #[view(getTokenId)]
    #[storage_mapper("tokenId")]
    fn token_id(&self) -> SingleValueMapper<TokenIdentifier>;

    #[view(getPrice)]
    #[storage_mapper("price")]
    fn price(&self) -> SingleValueMapper<BigUint>;
}
