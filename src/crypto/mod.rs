pub mod hashing;
pub mod signature;

use rand_core::OsRng;
use ed25519_dalek::SigningKey;

pub fn generate_keys() {
    let mut csprng = OsRng;

    let hmac_key = SigningKey::generate(&mut csprng);
    println!("HMAC_KEY={}", 
        hex::encode(hmac_key.to_bytes())
    );

    let signing_key = SigningKey::generate(&mut csprng);
    println!("SIG_KEY={}", 
        hex::encode(signing_key.to_bytes())
    );
}
