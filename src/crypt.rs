use orion::hazardous::{
    aead::xchacha20poly1305::{seal, open, Nonce, SecretKey as XSecretKey},
    mac::poly1305::POLY1305_OUTSIZE,
    stream::xchacha20::XCHACHA_NONCESIZE,
};
use orion::hazardous::stream::chacha20::CHACHA_KEYSIZE;
use orion::kdf::{derive_key, Password, Salt};
use ring::rand::SecureRandom;

const NONCE_PLUS_AD_SIZE: usize = XCHACHA_NONCESIZE + 32;

/// Split encrypted cipher text into IV, AD and encrypted text
fn split_encrypted( cipher_text: &[u8] ) -> (Vec<u8>, Vec<u8>, Vec<u8>) {
    return (
        cipher_text[..XCHACHA_NONCESIZE].to_vec(),
        cipher_text[XCHACHA_NONCESIZE..NONCE_PLUS_AD_SIZE].to_vec(),
        cipher_text[NONCE_PLUS_AD_SIZE..].to_vec(),
        )
}

/// Fill passed array with cryptographically random data from ring crate
fn get_random( dest: &mut [u8]) {
    let sr = ring::rand::SystemRandom::new();
    sr.fill( dest ).unwrap();
}

fn nonce() -> Vec<u8> {
    let mut randoms: [u8; 24] = [0; 24];
    get_random( &mut randoms );
    return randoms.to_vec();
}

fn auth_tag() -> Vec<u8> {
    let mut randoms: [u8; 32] = [0; 32];
    get_random( &mut randoms );
    return randoms.to_vec();
}

fn create_key( password: String, nonce: Vec<u8> ) -> XSecretKey {
    let password = Password::from_slice(password.as_bytes()).unwrap();
    let salt = Salt::from_slice(nonce.as_slice()).unwrap();
    let kdf_key = derive_key(&password, &salt, 15, 1024, CHACHA_KEYSIZE as u32).unwrap();
    let key = XSecretKey::from_slice( kdf_key.unprotected_as_bytes() ).unwrap();
    return key;
}

pub fn encrypt( password: String, data: String ) -> Vec<u8> {
    let nonce = nonce();
    let key = create_key( password, nonce.clone() );
    let nonce = Nonce::from_slice( nonce.as_slice() ).unwrap();
    let mut ad = auth_tag();

    // Get the output length
    let output_len = match data.len().checked_add( XCHACHA_NONCESIZE + POLY1305_OUTSIZE + ad.len() ) {
        Some( min_output_len ) => min_output_len,
        None => panic!( "Plaintext is too long" ),
    };

    // Allocate a buffer for the output
    let mut output = vec![0u8; output_len];
    output[..XCHACHA_NONCESIZE].copy_from_slice(nonce.as_ref());
    output[XCHACHA_NONCESIZE..NONCE_PLUS_AD_SIZE].copy_from_slice( ad.as_ref() );
    seal(&key, &nonce, data.as_bytes(), Some( ad.clone().as_slice() ), &mut output[NONCE_PLUS_AD_SIZE..]).unwrap();
    return output;
}

pub fn decrypt( password: String, cipher_text: &[u8] ) -> Vec<u8> {
    let key = create_key(password, cipher_text[..XCHACHA_NONCESIZE].to_vec());
    let split = split_encrypted( cipher_text );
    let nonce = Nonce::from_slice( split.0.as_slice() ).unwrap();
    let mut output = vec![0u8; split.2.len()];

    open(&key, &nonce, split.2.as_slice(), Some( split.1.as_slice() ), &mut output ).unwrap();
    // Remove any remaining padding
    output.retain(|&x| x != 0u8);
    return output.to_vec();
}