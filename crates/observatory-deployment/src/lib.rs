//! Deployed contract inspection.
//!
//! Deployment lookup reads the contract instance ledger entry through
//! `getLedgerEntries` and decodes the returned `LedgerEntryData` XDR to recover
//! the deployed WASM hash. The ledger key and entry are real Stellar XDR types;
//! nothing about the RPC shape is invented.

use std::cell::RefCell;
use std::collections::VecDeque;

use observatory_core::limits::MAX_WASM_BYTES;
use observatory_core::{ObservatoryError, Result};
use observatory_rpc::{LedgerEntryResult, RpcClient, Transport};
use serde::Serialize;
use stellar_xdr::{
    ContractDataDurability, ContractExecutable, ContractId, Hash, LedgerEntryData, LedgerKey,
    LedgerKeyContractCode, LedgerKeyContractData, Limits, ReadXdr, ScAddress, ScVal, WriteXdr,
};

/// Default number of ledger entries retained by a [`LedgerEntryCache`].
pub const DEFAULT_LEDGER_CACHE_CAPACITY: usize = 128;

/// Information recovered about a deployed contract.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct DeploymentInfo {
    /// The contract id (C... strkey) that was queried.
    pub contract_id: String,
    /// The deployed executable WASM hash (hex), if the contract is WASM-backed.
    pub wasm_hash: Option<String>,
    /// Last modified ledger sequence of the instance entry.
    pub last_modified_ledger_seq: Option<u32>,
    /// Live-until ledger sequence of the instance entry.
    pub live_until_ledger_seq: Option<u32>,
    /// Whether a contract instance entry was found.
    pub found: bool,
}

/// A small, bounded, in-memory cache for `getLedgerEntries` lookups.
///
/// The cache is scoped to one process run: it is never persisted and never
/// shared between runs or processes. It is opt-in — pass it to
/// [`inspect_with_cache`] or [`deployed_wasm_with_cache`]. Entries are evicted
/// in insertion order once [`capacity`](LedgerEntryCache::capacity) is reached,
/// and missing entries are cached too so repeated misses do not hit the RPC.
///
/// A capacity of `0` disables caching while keeping the same call shape.
#[derive(Debug)]
pub struct LedgerEntryCache {
    capacity: usize,
    entries: RefCell<VecDeque<(String, Option<LedgerEntryResult>)>>,
}

impl LedgerEntryCache {
    /// Creates a cache with the given capacity. `0` disables caching.
    #[must_use]
    pub fn new(capacity: usize) -> Self {
        LedgerEntryCache {
            capacity,
            entries: RefCell::new(VecDeque::new()),
        }
    }

    /// Creates a cache with [`DEFAULT_LEDGER_CACHE_CAPACITY`].
    #[must_use]
    pub fn with_default_capacity() -> Self {
        LedgerEntryCache::new(DEFAULT_LEDGER_CACHE_CAPACITY)
    }

    /// The maximum number of entries retained.
    #[must_use]
    pub fn capacity(&self) -> usize {
        self.capacity
    }

    /// The number of cached entries, including cached misses.
    #[must_use]
    pub fn len(&self) -> usize {
        self.entries.borrow().len()
    }

    /// Whether the cache is empty.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Removes every cached entry.
    pub fn clear(&self) {
        self.entries.borrow_mut().clear();
    }

    fn get(&self, key: &str) -> Option<Option<LedgerEntryResult>> {
        self.entries
            .borrow()
            .iter()
            .find(|(cached, _)| cached == key)
            .map(|(_, entry)| entry.clone())
    }

    fn insert(&self, key: String, entry: Option<LedgerEntryResult>) {
        if self.capacity == 0 {
            return;
        }
        let mut entries = self.entries.borrow_mut();
        if entries.len() >= self.capacity {
            entries.pop_front();
        }
        entries.push_back((key, entry));
    }

    /// Returns a cached entry, fetching it through `client` on a miss.
    ///
    /// Both hits and misses are cached; a miss is stored as `None`.
    pub fn get_or_fetch<T: Transport>(
        &self,
        client: &RpcClient<T>,
        ledger_key: &str,
    ) -> Result<Option<LedgerEntryResult>> {
        if let Some(cached) = self.get(ledger_key) {
            return Ok(cached);
        }
        let entries = client.get_ledger_entries(&[ledger_key.to_string()])?;
        let entry = entries.into_iter().next();
        self.insert(ledger_key.to_string(), entry.clone());
        Ok(entry)
    }
}

/// Decode a contract id strkey into its 32-byte hash.
pub fn contract_id_hash(input: &str) -> Result<[u8; 32]> {
    let parsed = stellar_strkey::Strkey::from_string(input.trim())
        .map_err(|error| ObservatoryError::invalid(format!("invalid contract id: {error}")))?;
    match parsed {
        stellar_strkey::Strkey::Contract(contract) => Ok(contract.0),
        _ => Err(ObservatoryError::invalid(
            "provided strkey is not a contract id (expected a C... strkey)",
        )),
    }
}

/// Build the base64-XDR ledger key for a contract instance entry.
pub fn contract_instance_key(contract_id: &str) -> Result<String> {
    let hash = contract_id_hash(contract_id)?;
    let key = LedgerKey::ContractData(LedgerKeyContractData {
        contract: ScAddress::Contract(ContractId(Hash(hash))),
        key: ScVal::LedgerKeyContractInstance,
        durability: ContractDataDurability::Persistent,
    });
    key.to_xdr_base64(Limits::none()).map_err(|error| {
        ObservatoryError::internal(format!("failed to encode ledger key: {error}"))
    })
}

/// Inspect a deployed contract, returning its deployed WASM hash when known.
pub fn inspect<T: Transport>(client: &RpcClient<T>, contract_id: &str) -> Result<DeploymentInfo> {
    let key = contract_instance_key(contract_id)?;
    let entries = client.get_ledger_entries(&[key])?;
    deployment_info(contract_id, entries.first())
}

/// Inspect a deployed contract using a [`LedgerEntryCache`].
///
/// Repeating the same lookup with the same cache performs one RPC call.
pub fn inspect_with_cache<T: Transport>(
    client: &RpcClient<T>,
    contract_id: &str,
    cache: &LedgerEntryCache,
) -> Result<DeploymentInfo> {
    let key = contract_instance_key(contract_id)?;
    let entry = cache.get_or_fetch(client, &key)?;
    deployment_info(contract_id, entry.as_ref())
}

fn deployment_info(contract_id: &str, entry: Option<&LedgerEntryResult>) -> Result<DeploymentInfo> {
    let Some(entry) = entry else {
        return Ok(DeploymentInfo {
            contract_id: contract_id.to_string(),
            wasm_hash: None,
            last_modified_ledger_seq: None,
            live_until_ledger_seq: None,
            found: false,
        });
    };
    Ok(DeploymentInfo {
        contract_id: contract_id.to_string(),
        wasm_hash: decode_wasm_hash(entry)?,
        last_modified_ledger_seq: entry.last_modified_ledger_seq,
        live_until_ledger_seq: entry.live_until_ledger_seq,
        found: true,
    })
}

/// Decode the WASM hash from a ledger entry result, if it is a WASM-backed instance.
pub fn decode_wasm_hash(entry: &LedgerEntryResult) -> Result<Option<String>> {
    let data = LedgerEntryData::from_xdr_base64(&entry.xdr, Limits::none())
        .map_err(|error| ObservatoryError::decode(format!("invalid ledger entry XDR: {error}")))?;
    let LedgerEntryData::ContractData(contract) = data else {
        return Ok(None);
    };
    let ScVal::ContractInstance(instance) = contract.val else {
        return Ok(None);
    };
    match instance.executable {
        ContractExecutable::Wasm(hash) => Ok(Some(hex::encode(hash.0))),
        _ => Ok(None),
    }
}

/// Fetch and decode the deployed WASM code for a WASM hash (hex).
///
/// Returns `None` when no code entry exists for the hash.
pub fn deployed_wasm<T: Transport>(
    client: &RpcClient<T>,
    wasm_hash_hex: &str,
) -> Result<Option<Vec<u8>>> {
    let key = contract_code_key(wasm_hash_hex)?;
    let entries = client.get_ledger_entries(&[key])?;
    decode_deployed_wasm(entries.first())
}

/// Fetch deployed WASM using a [`LedgerEntryCache`].
///
/// Repeating the same lookup with the same cache performs one RPC call.
pub fn deployed_wasm_with_cache<T: Transport>(
    client: &RpcClient<T>,
    wasm_hash_hex: &str,
    cache: &LedgerEntryCache,
) -> Result<Option<Vec<u8>>> {
    let key = contract_code_key(wasm_hash_hex)?;
    let entry = cache.get_or_fetch(client, &key)?;
    decode_deployed_wasm(entry.as_ref())
}

fn contract_code_key(wasm_hash_hex: &str) -> Result<String> {
    let bytes = hex::decode(wasm_hash_hex.trim())
        .map_err(|error| ObservatoryError::invalid(format!("invalid wasm hash: {error}")))?;
    let hash: [u8; 32] = bytes
        .try_into()
        .map_err(|_| ObservatoryError::invalid("wasm hash must be 32 bytes"))?;
    let key = LedgerKey::ContractCode(LedgerKeyContractCode { hash: Hash(hash) });
    key.to_xdr_base64(Limits::none()).map_err(|error| {
        ObservatoryError::internal(format!("failed to encode ledger key: {error}"))
    })
}

fn decode_deployed_wasm(entry: Option<&LedgerEntryResult>) -> Result<Option<Vec<u8>>> {
    let Some(entry) = entry else {
        return Ok(None);
    };
    let data = LedgerEntryData::from_xdr_base64(&entry.xdr, Limits::none())
        .map_err(|error| ObservatoryError::decode(format!("invalid ledger entry XDR: {error}")))?;
    let LedgerEntryData::ContractCode(code) = data else {
        return Ok(None);
    };
    let wasm = code.code.to_vec();
    if wasm.len() > MAX_WASM_BYTES {
        return Err(ObservatoryError::invalid(format!(
            "deployed WASM is {} bytes, exceeding the {} byte limit",
            wasm.len(),
            MAX_WASM_BYTES
        )));
    }
    Ok(Some(wasm))
}

#[cfg(test)]
mod tests {
    use super::*;
    use observatory_rpc::{Endpoint, MockTransport};
    use serde_json::json;
    use stellar_xdr::{
        ContractCodeCostInputs, ContractCodeEntry, ContractCodeEntryExt, ContractCodeEntryV1,
        ContractDataEntry, ExtensionPoint, LedgerEntryData, ScContractInstance, WriteXdr,
    };

    fn contract_id() -> String {
        format!("{}", stellar_strkey::Contract([9u8; 32]))
    }

    fn instance_entry(hash_byte: u8) -> LedgerEntryResult {
        let data = LedgerEntryData::ContractData(ContractDataEntry {
            ext: ExtensionPoint::V0,
            contract: ScAddress::Contract(ContractId(Hash([9u8; 32]))),
            key: ScVal::LedgerKeyContractInstance,
            durability: ContractDataDurability::Persistent,
            val: ScVal::ContractInstance(ScContractInstance {
                executable: ContractExecutable::Wasm(Hash([hash_byte; 32])),
                storage: None,
            }),
        });
        LedgerEntryResult {
            key: String::new(),
            xdr: data.to_xdr_base64(Limits::none()).unwrap(),
            last_modified_ledger_seq: Some(123),
            live_until_ledger_seq: None,
        }
    }

    fn code_entry(code: &[u8]) -> LedgerEntryResult {
        let data = LedgerEntryData::ContractCode(ContractCodeEntry {
            ext: ContractCodeEntryExt::V1(ContractCodeEntryV1 {
                ext: ExtensionPoint::V0,
                cost_inputs: ContractCodeCostInputs {
                    ext: ExtensionPoint::V0,
                    n_instructions: 1,
                    n_functions: 1,
                    n_globals: 0,
                    n_table_entries: 0,
                    n_types: 0,
                    n_data_segments: 0,
                    n_elem_segments: 0,
                    n_imports: 0,
                    n_exports: 1,
                    n_data_segment_bytes: 0,
                },
            }),
            hash: Hash([5u8; 32]),
            code: code.to_vec().try_into().unwrap(),
        });
        LedgerEntryResult {
            key: String::new(),
            xdr: data.to_xdr_base64(Limits::none()).unwrap(),
            last_modified_ledger_seq: Some(1),
            live_until_ledger_seq: None,
        }
    }

    #[test]
    fn parses_contract_id_and_builds_key() {
        let id = contract_id();
        assert_eq!(contract_id_hash(&id).unwrap(), [9u8; 32]);
        let key = contract_instance_key(&id).unwrap();
        assert!(!key.is_empty());
    }

    #[test]
    fn rejects_non_contract_strkey() {
        let account = stellar_strkey::ed25519::PublicKey([0u8; 32]).to_string();
        assert!(contract_id_hash(&account).is_err());
        assert!(contract_id_hash("not-a-strkey").is_err());
    }

    #[test]
    fn decodes_wasm_hash() {
        let entry = instance_entry(7);
        assert_eq!(
            decode_wasm_hash(&entry).unwrap(),
            Some(hex::encode([7u8; 32]))
        );
    }

    #[test]
    fn mock_deployment_inspection() {
        let entry = instance_entry(3);
        let transport = MockTransport::new().with_result(
            "getLedgerEntries",
            json!({ "entries": [ {
                "key": entry.key,
                "xdr": entry.xdr,
                "lastModifiedLedgerSeq": 123
            } ] }),
        );
        let client = RpcClient::new(Endpoint::parse("https://example.org").unwrap(), transport);
        let info = inspect(&client, &contract_id()).unwrap();
        assert!(info.found);
        assert_eq!(info.wasm_hash, Some(hex::encode([3u8; 32])));
        assert_eq!(info.last_modified_ledger_seq, Some(123));
    }

    #[test]
    fn missing_deployment_is_reported() {
        let transport =
            MockTransport::new().with_result("getLedgerEntries", json!({ "entries": [] }));
        let client = RpcClient::new(Endpoint::parse("https://example.org").unwrap(), transport);
        let info = inspect(&client, &contract_id()).unwrap();
        assert!(!info.found);
        assert_eq!(info.wasm_hash, None);
    }

    #[test]
    fn fetches_deployed_wasm_bytes() {
        let wasm = [0x00, 0x61, 0x73, 0x6d, 0x01, 0x00, 0x00, 0x00];
        let entry = code_entry(&wasm);
        let transport = MockTransport::new().with_result(
            "getLedgerEntries",
            json!({ "entries": [ { "key": "", "xdr": entry.xdr } ] }),
        );
        let client = RpcClient::new(Endpoint::parse("https://example.org").unwrap(), transport);
        let fetched = deployed_wasm(&client, &hex::encode([5u8; 32])).unwrap();
        assert_eq!(fetched, Some(wasm.to_vec()));
    }

    #[test]
    fn deployed_wasm_missing_entry_is_none() {
        let transport =
            MockTransport::new().with_result("getLedgerEntries", json!({ "entries": [] }));
        let client = RpcClient::new(Endpoint::parse("https://example.org").unwrap(), transport);
        assert_eq!(
            deployed_wasm(&client, &hex::encode([5u8; 32])).unwrap(),
            None
        );
    }

    #[test]
    fn deployed_wasm_rejects_bad_hash() {
        let transport = MockTransport::new();
        let client = RpcClient::new(Endpoint::parse("https://example.org").unwrap(), transport);
        assert!(deployed_wasm(&client, "not-hex").is_err());
        assert!(deployed_wasm(&client, &hex::encode([1u8; 8])).is_err());
    }

    fn instance_transport(entry: &LedgerEntryResult) -> MockTransport {
        MockTransport::new().with_result(
            "getLedgerEntries",
            json!({ "entries": [ {
                "key": entry.key,
                "xdr": entry.xdr,
                "lastModifiedLedgerSeq": 123
            } ] }),
        )
    }

    #[test]
    fn repeated_cached_lookup_calls_the_transport_once() {
        let entry = instance_entry(3);
        let transport = instance_transport(&entry);
        let client = RpcClient::new(Endpoint::parse("https://example.org").unwrap(), &transport);
        let cache = LedgerEntryCache::with_default_capacity();

        let first = inspect_with_cache(&client, &contract_id(), &cache).unwrap();
        let second = inspect_with_cache(&client, &contract_id(), &cache).unwrap();

        assert_eq!(first, second);
        assert_eq!(transport.calls(), vec!["getLedgerEntries".to_string()]);
        assert_eq!(cache.len(), 1);
    }

    #[test]
    fn cached_misses_are_reused_too() {
        let transport =
            MockTransport::new().with_result("getLedgerEntries", json!({ "entries": [] }));
        let client = RpcClient::new(Endpoint::parse("https://example.org").unwrap(), &transport);
        let cache = LedgerEntryCache::new(4);

        assert!(
            !inspect_with_cache(&client, &contract_id(), &cache)
                .unwrap()
                .found
        );
        assert!(
            !inspect_with_cache(&client, &contract_id(), &cache)
                .unwrap()
                .found
        );

        assert_eq!(transport.calls().len(), 1);
        assert_eq!(cache.len(), 1);
    }

    #[test]
    fn cache_capacity_is_bounded_and_zero_disables_it() {
        let entry = instance_entry(3);
        let transport = instance_transport(&entry);
        let client = RpcClient::new(Endpoint::parse("https://example.org").unwrap(), &transport);

        let bounded = LedgerEntryCache::new(1);
        let other = format!("{}", stellar_strkey::Contract([8u8; 32]));
        let _ = inspect_with_cache(&client, &contract_id(), &bounded).unwrap();
        let _ = inspect_with_cache(&client, &other, &bounded).unwrap();
        assert_eq!(bounded.len(), 1);

        let disabled = LedgerEntryCache::new(0);
        let _ = inspect_with_cache(&client, &contract_id(), &disabled).unwrap();
        let _ = inspect_with_cache(&client, &contract_id(), &disabled).unwrap();
        assert!(disabled.is_empty());
        assert_eq!(transport.calls().len(), 4);
    }

    #[test]
    fn deployed_wasm_cache_calls_the_transport_once() {
        let wasm = [0x00, 0x61, 0x73, 0x6d, 0x01, 0x00, 0x00, 0x00];
        let entry = code_entry(&wasm);
        let transport = MockTransport::new().with_result(
            "getLedgerEntries",
            json!({ "entries": [ { "key": "", "xdr": entry.xdr } ] }),
        );
        let client = RpcClient::new(Endpoint::parse("https://example.org").unwrap(), &transport);
        let cache = LedgerEntryCache::with_default_capacity();
        let hash = hex::encode([5u8; 32]);

        assert_eq!(
            deployed_wasm_with_cache(&client, &hash, &cache).unwrap(),
            Some(wasm.to_vec())
        );
        assert_eq!(
            deployed_wasm_with_cache(&client, &hash, &cache).unwrap(),
            Some(wasm.to_vec())
        );

        assert_eq!(transport.calls().len(), 1);
        cache.clear();
        assert!(cache.is_empty());
    }
}
