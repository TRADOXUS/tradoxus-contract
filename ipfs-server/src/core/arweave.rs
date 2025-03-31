use std::path::PathBuf;

use arweave_rs::{crypto::base64::Base64, Arweave};
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use url::Url;

use crate::{config::CFG, error::Error};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ArweaveCertificateRecord {
    pub id: i32,
    pub issuer_pub_key: String,
    pub recipient_pub_key: String,
    pub course_name: String,
    pub course_description: Option<String>,
    pub skills: Vec<String>,
    pub grade: Option<String>,
    pub issuance_date: NaiveDateTime,
    pub completion_date: NaiveDateTime,
    pub proof_type: String,
    pub signature: String,
    pub previous_id: Option<i32>,
    pub previous_arweave_id: Option<String>,
}

impl ArweaveCertificateRecord {
    pub async fn upload_to_arweave(self) -> Result<String, Error> {
        // Arweave configuration is not set. Return empty string
        if CFG.arweave.is_none() {
            return Ok("".into());
        }
        let arweave_config = CFG.arweave.clone().unwrap();

        // create the signer
        let arweave_url = Url::parse(&arweave_config.url)?;
        let client =
            Arweave::from_keypair_path(PathBuf::from(&arweave_config.jwt), arweave_url.clone())?;

        let target = Base64(vec![]);
        let data = serde_json::to_vec(&self)?;

        // query the fee of upload and create the transaction
        let fee = client.get_fee(target.clone(), data.clone()).await?;
        let send_transaction = client
            .create_transaction(target, vec![], data, 0, fee, true)
            .await?;

        let signed_transaction = client.sign_transaction(send_transaction)?;
        let result = client.post_transaction(&signed_transaction).await?;

        // return the transcation id to user
        Ok(result.0)
    }
}
