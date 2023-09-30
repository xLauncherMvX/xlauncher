// Code generated by the multiversx-sc multi-contract system. DO NOT EDIT.

////////////////////////////////////////////////////
////////////////// AUTO-GENERATED //////////////////
////////////////////////////////////////////////////

// Init:                                 1
// Endpoints:                            7
// Async Callback (empty):               1
// Total number of exported functions:   9

#![no_std]

// Configuration that works with rustc < 1.73.0.
// TODO: Recommended rustc version: 1.73.0 or newer.
#![feature(lang_items)]

multiversx_sc_wasm_adapter::allocator!();
multiversx_sc_wasm_adapter::panic_handler!();

multiversx_sc_wasm_adapter::endpoints! {
    xlauncher_simple
    (
        init => init
        setContractSettings => set_contract_settings
        fundContract => fund_contract
        buy => buy
        getTokenBalance => get_token_balance
        getTokenId => token_id
        getPrice => price
        getStartTimestamp => start_timestamp
    )
}

multiversx_sc_wasm_adapter::async_callback_empty! {}
