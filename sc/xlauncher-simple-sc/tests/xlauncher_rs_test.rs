use multiversx_sc_scenario::*;

multiversx_sc::imports!();
multiversx_sc::derive_imports!();

fn world() -> ScenarioWorld {
    let mut blockchain = ScenarioWorld::new();
    blockchain.set_current_dir_from_workspace("");

    blockchain.register_contract(
        "file:output/xlauncher-simple.wasm",
        xlauncher_simple::ContractBuilder,
    );
    blockchain
}

#[test]
fn _01_deploy() {
    multiversx_sc_scenario::run_rs("scenarios/_01_deploy.scen.json", world());
}

#[test]
fn _02_set_contract_settings() {
    multiversx_sc_scenario::run_rs("scenarios/_02_set_contract_settings.scen.json", world());
}
