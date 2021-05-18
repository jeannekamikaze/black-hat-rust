use chrono::{DateTime, Utc};
use common::api;
use uuid::Uuid;

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct Job {
    pub id: Uuid,
    pub agent_id: Uuid,
    pub encrypted_job: Vec<u8>,
    pub ephemeral_public_key: Vec<u8>,
    pub nonce: Vec<u8>,
    pub signature: Vec<u8>,
    pub encrypted_result: Option<Vec<u8>>,
    pub result_ephemeral_public_key: Option<Vec<u8>>,
    pub result_nonce: Option<Vec<u8>>,
    pub result_signature: Option<Vec<u8>>,
}

impl Into<api::Job> for Job {
    fn into(self) -> api::Job {
        api::Job {
            id: self.id,
            agent_id: self.agent_id,
            encrypted_job: self.encrypted_job,
            ephemeral_public_key: self.ephemeral_public_key,
            nonce: self.nonce,
            signature: self.signature,
            encrypted_result: self.encrypted_result,
            result_ephemeral_public_key: self.result_ephemeral_public_key,
            result_nonce: self.result_nonce,
            result_signature: self.result_signature,
        }
    }
}

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct Agent {
    pub id: Uuid,
    pub created_at: DateTime<Utc>,
    pub last_seen_at: DateTime<Utc>,
    pub identity_public_key: Vec<u8>,
    pub public_prekey: Vec<u8>,
    pub public_prekey_signature: Vec<u8>,
}

impl Into<api::Agent> for Agent {
    fn into(self) -> api::Agent {
        api::Agent {
            id: self.id,
            created_at: self.created_at,
            last_seen_at: self.last_seen_at,
            identity_public_key: self.identity_public_key,
            public_prekey: self.public_prekey,
            public_prekey_signature: self.public_prekey_signature,
        }
    }
}