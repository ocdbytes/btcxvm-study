use clap::{Arg, Command};
use k256::ecdsa::signature::Signer;
use k256::ecdsa::signature::Verifier;
use k256::ecdsa::Signature;
use k256::ecdsa::SigningKey;
use k256::ecdsa::VerifyingKey;
use k256::elliptic_curve::rand_core::OsRng;
use sha2::{Digest, Sha256};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let matches = Command::new("Bee")
        .version("0.0.1")
        .author("Arun Jangra arunjangra1001@gmail.com")
        .about(
            "==================\n\
BTC helper tools:
==================\n
1. Random Address Generator (generate)
2. Sign message with a private key (sign -p <secret_key> -m <message>)\n

[⚠️ Message will be hashed before signing]",
        )
        .subcommand(Command::new("generate").about("Generates a random BTC Address"))
        .subcommand(
            Command::new("sign")
                .about("Signs a message with a private key")
                .arg(
                    Arg::new("private_key")
                        .short('p')
                        .long("private-key")
                        .help("The private key to sign with")
                        .required(true)
                        .num_args(1),
                )
                .arg(
                    Arg::new("message")
                        .short('m')
                        .long("message")
                        .help("The message to sign")
                        .required(true)
                        .num_args(1),
                ),
        )
        .get_matches();

    match matches.subcommand() {
        Some(("generate", _)) => match generate_address() {
            Ok((private_key, address)) => {
                println!("Generated BTC Address : {}", address);
                println!("Private Key : {}", private_key);
            }
            Err(e) => eprintln!("Error generating address: {}", e),
        },
        Some(("sign", sub_matches)) => {
            let private_key = sub_matches
                .get_one::<String>("private_key")
                .expect("Private key is required");
            let message = sub_matches
                .get_one::<String>("message")
                .expect("Message is required");

            match sign_message(private_key, message) {
                Ok(signature) => println!("Signature: {}", signature),
                Err(e) => eprintln!("Error signing message: {}", e),
            }
        }
        _ => println!("No subcommand was used. Use --help for usage information."),
    }

    Ok(())
}

fn generate_address() -> color_eyre::Result<(String, String)> {
    let signing_key: SigningKey = SigningKey::random(&mut OsRng);
    let verifying_key = signing_key.verifying_key().to_sec1_bytes();

    Ok((
        hex::encode(hex::encode(signing_key.to_bytes())),
        hex::encode(hex::encode(verifying_key)),
    ))
}

#[allow(deprecated)]
fn sign_message(private_key: &str, message: &str) -> color_eyre::Result<String> {
    let private_key_bytes = hex::decode(private_key)?;
    if private_key_bytes.len() != 32 {
        return Err(color_eyre::eyre::eyre!("Private key must be 32 bytes"));
    }

    let secret_key: SigningKey = SigningKey::from_slice(private_key_bytes.as_slice())
        .expect("Unable to derive signing key.");
    let public_key: VerifyingKey = *secret_key.verifying_key();

    // Hash the message
    let mut hasher = Sha256::new();
    hasher.update(message.as_bytes());
    let message_hash = hasher.finalize();
    let message = message_hash.as_slice();

    let signature: Signature = secret_key.sign(&message);

    let result = public_key.verify(message, &signature);

    match result {
        Ok(_) => Ok(signature.to_string()),
        Err(e) => Err(color_eyre::eyre::eyre!(e)),
    }
}
