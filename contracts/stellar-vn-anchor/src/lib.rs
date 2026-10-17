#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, Address, BytesN, Env};

#[contracttype]
#[derive(Clone)]
pub struct Anchor {
    pub owner: Address,
    pub digest: BytesN<32>,
    pub timestamp: u64,
}

#[contract]
pub struct StellarVnAnchor;

#[contractimpl]
impl StellarVnAnchor {
    /// Record a SHA-256 digest without revealing the original document.
    /// The owner's wallet MUST authorize this invocation.
    /// Duplicate digests are rejected to preserve the first timestamp.
    pub fn anchor(env: Env, owner: Address, digest: BytesN<32>) -> bool {
        owner.require_auth();
        let store = env.storage().persistent();
        if store.has(&digest) {
            return false;
        }
        let item = Anchor {
            owner,
            digest: digest.clone(),
            timestamp: env.ledger().timestamp(),
        };
        store.set(&digest, &item);
        store.extend_ttl(&digest, 518400, 1036800);
        true
    }

    /// Returns the original on-chain record, if any.
    pub fn get(env: Env, digest: BytesN<32>) -> Option<Anchor> {
        env.storage().persistent().get(&digest)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use soroban_sdk::testutils::Address as _;
    #[test]
    fn records_hash_and_preserves_first_owner() {
        let env = Env::default();
        env.mock_all_auths();
        let contract_id = env.register(StellarVnAnchor, ());
        let client = StellarVnAnchorClient::new(&env, &contract_id);
        let owner = Address::generate(&env);
        let digest = BytesN::from_array(&env, &[7u8; 32]);
        assert!(client.anchor(&owner, &digest));
        assert!(!client.anchor(&owner, &digest));
        assert_eq!(client.get(&digest).unwrap().owner, owner);
    }
}
