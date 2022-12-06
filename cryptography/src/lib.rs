use std::num::NonZeroU32;
use rand::RngCore;
use rand::rngs::OsRng;
use rug::{Complete, Integer};
use rug::rand::RandState;
use ring::{digest, pbkdf2};
use crate::e521::{MultiplePointMontgomery, PointE521};

pub mod e521;

const X: &str = "1571054894184995387535939749894317568645297350402905821437625181152304994381188529632591196067604100772673927915114267193389905003276673749012051148356041324";

pub fn generate_private_key() -> Integer {
    let rng = OsRng.next_u32();
    let seed = Integer::from(rng);

    let mut rand = RandState::new();
    rand.seed(&seed);
    let private_key = Integer::from(Integer::random_bits(512, &mut rand));

    private_key
}

pub fn generate_public_key(private_key: &Integer) -> PointE521 {
    let e521 = PointE521::create_from_x(&Integer::parse(X).unwrap().complete());

    e521.multiple_number_by_montgomery(private_key)
}

pub fn diffie_hellman(private_key: &Integer, public_key: &PointE521) -> PointE521 {
    public_key.multiple_number_by_montgomery(private_key)
}

const CREDENTIAL_LEN: usize = digest::SHA512_OUTPUT_LEN;

pub fn encrypt_data(data: &[u8], salt: &[u8]) -> [u8; 64] {
    let n_iter = NonZeroU32::new(100_000).unwrap();
    let mut pbkdf2_hash = [0u8; CREDENTIAL_LEN];

    pbkdf2::derive(
        pbkdf2::PBKDF2_HMAC_SHA512,
        n_iter,
        salt,
        data,
        &mut pbkdf2_hash,
    );

    pbkdf2_hash
}

pub fn verify(previous_hash: &mut [u8], data: &[u8], salt: &[u8]) -> bool {
    let n_iter = NonZeroU32::new(100_000).unwrap();

    pbkdf2::verify(
        pbkdf2::PBKDF2_HMAC_SHA512,
        n_iter,
        salt,
        data,
        previous_hash,
    ) == Ok(())
}

// fn main() {
//     let private_key_alice = generate_private_key();
//     let public_point_alice = generate_public_key(&private_key_alice);
//
//     let private_key_bob = generate_private_key();
//     let public_point_bob = generate_public_key(&private_key_bob);
//
//     let private_point_bob = diffie_hellman(&private_key_bob, &public_point_alice);
//     let private_point_alice = diffie_hellman(&private_key_alice, &public_point_bob);
//
//     assert_eq!(private_point_alice.x, private_point_bob.x);
//     assert_eq!(private_point_alice.y, private_point_bob.y)
// }