use crate::crypto::{Address, Secp256k1Signature, HASH_BYTES_LEN, SIGNATURE_BYTES_LEN};
use clap::{App, Arg, ArgMatches};
use hex::decode_to_slice;
use tiny_keccak::{Hasher, Keccak};
use types::H256;

/// Generate recover sub command
pub fn recover_command() -> App<'static, 'static> {
    App::new("recover")
        .about("Recover a signature")
        .arg(
            Arg::with_name("data")
                .long("data")
                .takes_value(true)
                .required(true)
                .help("The data have been signed."),
        )
        .arg(
            Arg::with_name("sign")
                .long("sign")
                .required(true)
                .takes_value(true)
                .help("Signature for the data."),
        )
}

/// recover processor
pub fn recover_processor(sub_matches: &ArgMatches) -> Result<(), String> {
    let data = sub_matches.value_of("data").expect("Cannot get data");
    let sign_str = sub_matches.value_of("sign").expect("Cannot get sign");

    let data_bytes = data.to_owned().into_bytes();

    // Get data Hash
    let mut hasher = Keccak::v256();
    hasher.update(&data_bytes);
    let mut output = [0u8; HASH_BYTES_LEN];
    hasher.finalize(&mut output);

    let mut sign = [4u8; SIGNATURE_BYTES_LEN];

    decode_to_slice(sign_str, &mut sign as &mut [u8])
        .map_err(|e| format!("Can not parse sign: {}", e))?;

    let sign = Secp256k1Signature(sign);

    let pub_key = sign
        .recover(&H256(output))
        .map_err(|e| format!("Recover sign error: {:?}", e))?;

    if !sign
        .verify_public(&pub_key, &H256(output))
        .map_err(|e| format!("{:?}", e))?
    {
        return Err("Incorrect sign".to_owned());
    }

    // Get license bytes Hash
    let mut hasher = Keccak::v256();
    hasher.update(&pub_key);
    let mut output = [0u8; HASH_BYTES_LEN];
    hasher.finalize(&mut output);
    let addr = Address::from(H256(output));

    println!("Address: {:?}", addr);
    println!("Public key: {:?}", pub_key);
    Ok(())
}
