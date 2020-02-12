use crate::cli::util::{key_validator, parse_privkey};
use crate::crypto::{secp256k1_sign, HASH_BYTES_LEN};
use clap::{App, Arg, ArgMatches};
use tiny_keccak::{Hasher, Keccak};
use types::H256;

/// Generate sign sub command
pub fn sign_command() -> App<'static, 'static> {
    App::new("sign")
        .about("Sign a data.")
        .arg(
            Arg::with_name("data")
                .long("data")
                .takes_value(true)
                .required(true)
                .help("The data to be signed."),
        )
        .arg(
            Arg::with_name("private-key")
                .long("private-key")
                .takes_value(true)
                .required(true)
                .validator(|privkey| key_validator(privkey.as_ref()).map(|_| ())),
        )
}

/// sign processor
pub fn sign_processor(sub_matches: &ArgMatches) -> Result<(), String> {
    let data = sub_matches.value_of("data").expect("Cannot get data");

    let data_bytes = data.to_owned().into_bytes();

    // Get data Hash
    let mut hasher = Keccak::v256();
    hasher.update(&data_bytes);
    let mut output = [0u8; HASH_BYTES_LEN];
    hasher.finalize(&mut output);

    // Sign the fingerprint
    if let Some(private_key) = sub_matches.value_of("private-key") {
        let priv_key = parse_privkey(private_key)?;

        if let Ok(sign) = secp256k1_sign(&priv_key, &H256(output)) {
            println!("Signature is : {}", sign);
        }
    }

    Ok(())
}
