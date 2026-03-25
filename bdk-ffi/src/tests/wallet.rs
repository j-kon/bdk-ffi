use crate::bitcoin::Network;
use crate::descriptor::Descriptor;
use crate::store::Persister;
use crate::wallet::Wallet;

use bdk_wallet::KeychainKind;

use std::sync::Arc;

const EXTERNAL_DESCRIPTOR: &str = "wpkh(tprv8ZgxMBicQKsPf2qfrEygW6fdYseJDDrVnDv26PH5BHdvSuG6ecCbHqLVof9yZcMoM31z9ur3tTYbSnr1WBqbGX97CbXcmp5H6qeMpyvx35B/84h/1h/1h/0/*)";
const INTERNAL_DESCRIPTOR: &str = "wpkh(tprv8ZgxMBicQKsPf2qfrEygW6fdYseJDDrVnDv26PH5BHdvSuG6ecCbHqLVof9yZcMoM31z9ur3tTYbSnr1WBqbGX97CbXcmp5H6qeMpyvx35B/84h/1h/1h/1/*)";
const EXPECTED_FIRST_ADDRESS: &str = "tb1qhjys9wxlfykmte7ftryptx975uqgd6kcm6a7z4";

fn external_descriptor() -> Arc<Descriptor> {
    Arc::new(Descriptor::new(EXTERNAL_DESCRIPTOR.to_string(), Network::Signet).unwrap())
}

fn internal_descriptor() -> Arc<Descriptor> {
    Arc::new(Descriptor::new(INTERNAL_DESCRIPTOR.to_string(), Network::Signet).unwrap())
}

fn build_wallet() -> Wallet {
    Wallet::new(
        external_descriptor(),
        internal_descriptor(),
        Network::Signet,
        Arc::new(Persister::new_in_memory().unwrap()),
        25,
    )
    .unwrap()
}

#[test]
fn test_create_wallet() {
    let wallet = build_wallet();

    assert_eq!(wallet.network(), Network::Signet);
    assert_eq!(wallet.balance().total.to_sat(), 0u64);
    assert!(wallet.list_unspent().is_empty());
    assert_eq!(wallet.derivation_index(KeychainKind::External), None);
    assert_eq!(wallet.next_derivation_index(KeychainKind::External), 0);
}

#[test]
fn test_reveal_next_address() {
    let wallet = build_wallet();

    assert_eq!(wallet.derivation_index(KeychainKind::External), None);

    let address_info = wallet.reveal_next_address(KeychainKind::External);

    assert_eq!(address_info.index, 0);
    assert_eq!(address_info.keychain, KeychainKind::External);
    assert_eq!(address_info.address.to_string(), EXPECTED_FIRST_ADDRESS);
    assert_eq!(wallet.derivation_index(KeychainKind::External), Some(0));
    assert_eq!(wallet.next_derivation_index(KeychainKind::External), 1);
}

#[test]
fn test_create_single_wallet() {
    let descriptor = external_descriptor();
    let wallet = Wallet::create_single(
        descriptor.clone(),
        Network::Signet,
        Arc::new(Persister::new_in_memory().unwrap()),
        25,
    )
    .unwrap();

    assert_eq!(wallet.derivation_index(KeychainKind::External), None);

    let address_info = wallet.reveal_next_address(KeychainKind::External);

    assert_eq!(address_info.index, 0);
    assert_eq!(address_info.keychain, KeychainKind::External);
    assert_eq!(address_info.address.to_string(), EXPECTED_FIRST_ADDRESS);
    assert_eq!(wallet.derivation_index(KeychainKind::External), Some(0));
    assert_eq!(wallet.next_derivation_index(KeychainKind::External), 1);
    assert_eq!(
        wallet.public_descriptor(KeychainKind::External),
        descriptor.to_string()
    );
}
