use ed25519_dalek::{Signature, Signer, SigningKey};

use crate::SIG_KEY;

pub fn sign_artwork(data: &[u8]) -> String {
    let priv_key_hex = SIG_KEY.as_str(); 
    let priv_key_bytes = hex::decode(priv_key_hex).unwrap();
    
    let signing_key = SigningKey::from_bytes(
        priv_key_bytes.as_slice().try_into().unwrap()
    );
    
    // Sign the raw bytes of the image
    let signature: Signature = signing_key.sign(data);
    
    hex::encode(signature.to_bytes())
}
