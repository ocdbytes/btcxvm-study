use clap::{Arg, Command};
use secp256k1::rand::rngs::OsRng;
use secp256k1::{Message, Secp256k1, SecretKey};
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
    let secp = Secp256k1::new();

    let (secret_key, public_key) = secp.generate_keypair(&mut OsRng);

    Ok((
        hex::encode(secret_key.secret_bytes()),
        public_key.to_string(),
    ))
}

#[allow(deprecated)]
fn sign_message(private_key: &str, message: &str) -> color_eyre::Result<String> {
    let private_key_bytes = hex::decode(private_key)?;
    if private_key_bytes.len() != 32 {
        return Err(color_eyre::eyre::eyre!("Private key must be 32 bytes"));
    }

    let secp = Secp256k1::new();
    let secret_key = SecretKey::from_slice(&private_key_bytes)?;
    let public_key = secret_key.public_key(&secp);

    // Hash the message
    let mut hasher = Sha256::new();
    hasher.update(message.as_bytes());
    let message_hash = hasher.finalize();
    let message = Message::from_slice(&message_hash)?;

    let signature = secp.sign_ecdsa(&message, &secret_key);

    let result = secp.verify_ecdsa(&message, &signature, &public_key);

    match result {
        Ok(_) => Ok(signature.to_string()),
        Err(e) => Err(color_eyre::eyre::eyre!(e)),
    }
}
