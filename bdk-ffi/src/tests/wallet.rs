use crate::bitcoin::Network;
use crate::descriptor::Descriptor;
use crate::store::Persister;
use crate::wallet::Wallet;

use bdk_wallet::KeychainKind;

use std::sync::Arc;

const EXTERNAL_DESCRIPTOR: &str = "wpkh(tprv8ZgxMBicQKsPf2qfrEygW6fdYseJDDrVnDv26PH5BHdvSuG6ecCbHqLVof9yZcMoM31z9ur3tTYbSnr1WBqbGX97CbXcmp5H6qeMpyvx35B/84h/1h/1h/0/*)";
const INTERNAL_DESCRIPTOR: &str = "wpkh(tprv8ZgxMBicQKsPf2qfrEygW6fdYseJDDrVnDv26PH5BHdvSuG6ecCbHqLVof9yZcMoM31z9ur3tTYbSnr1WBqbGX97CbXcmp5H6qeMpyvx35B/84h/1h/1h/1/*)";
const EXPECTED_FIRST_ADDRESS: &str = "tb1qhjys9wxlfykmte7ftryptx975uqgd6kcm6a7z4";

fn build_wallet() -> Wallet {
    Wallet::new(
        Arc::new(Descriptor::new(EXTERNAL_DESCRIPTOR.to_string(), Network::Signet).unwrap()),
        Arc::new(Descriptor::new(INTERNAL_DESCRIPTOR.to_string(), Network::Signet).unwrap()),
        Network::Signet,
        Arc::new(Persister::new_in_memory().unwrap()),
        25,
    )
    .unwrap()
}

fn build_single_descriptor_wallet() -> Wallet {
    Wallet::create_single(
        Arc::new(Descriptor::new(EXTERNAL_DESCRIPTOR.to_string(), Network::Signet).unwrap()),
        Network::Signet,
        Arc::new(Persister::new_in_memory().unwrap()),
        25,
    )
    .unwrap()
}

#[test]
fn test_create_wallet_offline() {
    let wallet = build_wallet();

    assert_eq!(wallet.network(), Network::Signet);
    assert!(wallet.list_unspent().is_empty());
}

#[test]
fn test_new_wallet_starts_with_zero_balance() {
    let wallet = build_wallet();

    assert_eq!(wallet.balance().total.to_sat(), 0u64);
}

#[test]
fn test_reveal_next_address_offline() {
    let wallet = build_wallet();
    let address_info = wallet.reveal_next_address(KeychainKind::External);

    assert_eq!(address_info.index, 0);
    assert_eq!(address_info.keychain, KeychainKind::External);
    assert_eq!(address_info.address.to_string(), EXPECTED_FIRST_ADDRESS);
    assert_eq!(wallet.next_derivation_index(KeychainKind::External), 1);
}

#[test]
fn test_single_descriptor_wallet_external_and_internal_peek_match() {
    let wallet = build_single_descriptor_wallet();
    let external = wallet.peek_address(KeychainKind::External, 0);
    let internal = wallet.peek_address(KeychainKind::Internal, 0);

    assert_eq!(external.index, 0);
    assert_eq!(internal.index, 0);
    assert_eq!(external.address.to_string(), EXPECTED_FIRST_ADDRESS);
    assert_eq!(external.address.to_string(), internal.address.to_string());
}
