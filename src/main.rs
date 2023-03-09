// ref credit:
// - https://github.com/hrkrshnn/crunchvanity/blob/develop/src/main.rs
// - Foundry cast `cast create2`

use std::env;

use ethers::{
    prelude::{rand::thread_rng, *},
    utils::hex,
};
use rayon::prelude::*;

fn main() {
    let prefix = env::var("PREFIX").expect("PREFIX not set");
    let prefix = prefix.replace("0x", "").to_lowercase();
    let factory: Address = env::var("FACTORY") // 0x2B89c5c274C484B926D4E0417C45484cC4D634D2
        .expect("FACTORY not set")
        .parse()
        .unwrap();
    let init_code_hash = env::var("INIT_CODE_HASH").expect("INIT_CODE_HASH not set"); // 0x1fc924196cd154e95bffa954e5fd41653ac723075a71aad686f4b7ed1c3d0162
    let init_code_hash = Bytes::from(
        hex::decode(init_code_hash.strip_prefix("0x").unwrap_or(&init_code_hash)).unwrap(),
    );

    let result = std::iter::repeat(())
        .par_bridge()
        .map(|_| {
            let salt = H256::random_using(&mut thread_rng());
            let salt = Bytes::from(salt.to_fixed_bytes());

            let derived_address = ethers::core::utils::get_create2_address_from_hash(
                factory,
                salt.clone(),
                init_code_hash.clone(), // TODO: avoid clone
            );
            (salt, derived_address)
        })
        .find_any(|(_, derived_address)| hex::encode(derived_address).starts_with(&prefix));

    // if find any, print out the result
    if let Some((salt, derived_address)) = result {
        println!("factory: {}", hex::encode(factory));
        println!("salt: {}", salt);
        println!("derived address: {}", hex::encode(derived_address));
    } else {
        println!("Crunching failed. Bigger range?");
    }
}