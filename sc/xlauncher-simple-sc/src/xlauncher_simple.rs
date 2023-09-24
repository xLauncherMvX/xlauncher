#![no_std]

multiversx_sc::imports!();
multiversx_sc::derive_imports!();



#[multiversx_sc::contract]
pub trait XlauncherSimple {
    #[init]
    fn init(&self) {
    }
}
