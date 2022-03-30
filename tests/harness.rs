#![allow(unused)]
fn main() {
    use fuel_tx::Salt;
    use fuels_abigen_macro::abigen;
    use fuels_rs::contract::Contract;
    use rand::rngs::StdRng;
    use rand::{Rng, SeedableRng};

    // Generate Rust bindings from our contract JSON ABI
    // This will look in the root directory, not inside `tests/`
    abigen!(TestContract, "./test_contract.json");
    
    #[tokio::test]
    async fn harness() {
        let rng = &mut StdRng::seed_from_u64(2322u64);

        // Build the contract
        let salt: [u8; 32] = rng.gen();
        let salt = Salt::from(salt);
        let compiled = Contract::compile_sway_contract("../src/test_contract.sw", salt).unwrap();

        // Launch a local network and deploy the contract
        let (client, contract_id) = Contract::launch_and_deploy(&compiled).await.unwrap();

        let contract_instance = TestContract::new(compiled, client);

        // Call `initialize_counter()` method in our deployed contract
        // Note that you get type-safety for free
        let result = contract_instance
            .initialize_counter(42)
            .call()
            .await
            .unwrap();

        assert_eq!(42, result);

        // Call `increment_counter()` method in our deployed contract
        let result = contract_instance
            .increment_counter(10)
            .call()
            .await
            .unwrap();
        
        assert_eq!(52, result);
    }
}
