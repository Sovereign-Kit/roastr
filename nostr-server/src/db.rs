use std::collections::BTreeMap;

use fedimint_core::encoding::{Decodable, Encodable};
use fedimint_core::{impl_db_lookup, impl_db_record, PeerId};
use nostr_common::{NonceKeyPair, NostrEventId, SignatureShare, SigningSession, UnsignedEvent};
use serde::{Deserialize, Serialize};
use strum_macros::EnumIter;

#[repr(u8)]
#[derive(Clone, EnumIter, Debug)]
pub enum DbKeyPrefix {
    Nonce = 0x01,
    SessionNonces = 0x02,
    SignatureShare = 0x03,
}

impl std::fmt::Display for DbKeyPrefix {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

#[derive(Debug, Clone, Encodable, Decodable, Eq, PartialEq, Hash, Serialize)]
pub struct NonceKey {
    pub peer_id: PeerId,
    pub nonce: NonceKeyPair,
}

#[derive(Debug, Clone, Encodable, Decodable, Eq, PartialEq, Hash, Serialize)]
pub struct NonceKeyPrefix {
    pub peer_id: PeerId,
}

impl_db_record!(key = NonceKey, value = (), db_prefix = DbKeyPrefix::Nonce);

impl_db_lookup!(key = NonceKey, query_prefix = NonceKeyPrefix);

#[derive(Debug, Clone, Encodable, Decodable, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct SessionNonceKey {
    pub peers: SigningSession,
    pub event_id: NostrEventId,
}

#[derive(Debug, Clone, Encodable, Decodable, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct SessionNonces {
    pub nonces: BTreeMap<PeerId, NonceKeyPair>,
    pub unsigned_event: UnsignedEvent,
}

impl SessionNonces {
    pub fn new(unsigned_event: UnsignedEvent) -> SessionNonces {
        SessionNonces {
            nonces: BTreeMap::new(),
            unsigned_event,
        }
    }
}

#[derive(Debug, Clone, Encodable, Decodable, Eq, PartialEq, Hash, Serialize)]
pub struct SigningSessionKeyPrefix;

impl_db_record!(
    key = SessionNonceKey,
    value = SessionNonces,
    db_prefix = DbKeyPrefix::SessionNonces
);

impl_db_lookup!(
    key = SessionNonceKey,
    query_prefix = SigningSessionKeyPrefix
);

#[derive(Debug, Clone, Encodable, Decodable, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct SignatureShareKey {
    pub event_id: NostrEventId,
    pub peers: SigningSession,
}

#[derive(Debug, Clone, Encodable, Decodable, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct SignatureShareKeyPrefix {
    pub event_id: NostrEventId,
}

impl_db_record!(
    key = SignatureShareKey,
    value = SignatureShare,
    db_prefix = DbKeyPrefix::SignatureShare
);

impl_db_lookup!(
    key = SignatureShareKey,
    query_prefix = SignatureShareKeyPrefix
);
