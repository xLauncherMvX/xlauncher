#![no_std]

multiversx_sc::imports!();
multiversx_sc::derive_imports!();



#[multiversx_sc::contract]
pub trait XlauncherSimple {
    #[init]
    fn init(&self) {
    }

    #[only_owner]
    #[endpoint(setContractSettings)]
    fn set_contract_settings(&self,
                             token_id: TokenIdentifier,
                             initial_price: BigUint){

    }
}
