use hmac::{Hmac, Mac};
use sha2::Sha256;
use tracing::info;

use crate::{DBG_MODE, HMAC_KEY};

type HmacSha256 = Hmac<Sha256>;

pub fn verify_signature(body: &[u8], signature_hex: &str) -> bool {
    let mut mac = HmacSha256::new_from_slice(HMAC_KEY.as_bytes())
        .expect("HMAC can take key of any size");
    mac.update(body);

    let result = mac.finalize();
    let code_bytes = result.into_bytes();

    // deprecated feature
    if DBG_MODE {
        println!("  real_sig = {}", hex::encode(code_bytes));
    }

    info!("real request signature = {}", hex::encode(code_bytes));
    
    if let Ok(expected_bytes) = hex::decode(signature_hex) {
        return code_bytes.as_slice() == expected_bytes.as_slice();
    }
    false
}
