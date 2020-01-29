use base58check::ToBase58Check;
use clap::{App, Arg};
use ripemd160::{Digest, Ripemd160};
use secp256k1::{PublicKey, Secp256k1, SecretKey};
use sha2::Sha256;

fn main() {
    let app = App::new("Bitcoin Address Generator")
                .version("1.0")
                .about("Simple Rust CLI to generate different types of Bitcoin Addresses i.e. P2PKH, P2SH")
                .arg(Arg::with_name("private_key")
                    .short("p")
                    .long("private_key")
                    .help("Sets the private key string")
                    .required_if("type", "p2pkh")
                    .takes_value(true))
                .arg(Arg::with_name("type")
                    .short("t")
                    .long("type")
                    .help("Address type to generate")
                    .possible_values(&["p2pkh", "p2sh"])
                    .takes_value(true))
                .arg(Arg::with_name("spending_pub_key")
                    .short("s")
                    .long("spending_pub_key")
                    .required_if("type", "p2sh")
                    .help("Public key that redeems P2SH transaction")
                    .takes_value(true))
                .arg(Arg::with_name("key_pair")
                    .short("k")
                    .long("key_pair")
                    .help("Outputs Random Secp256k1 public and compressed private key"))
                .get_matches();
    if app.is_present("key_pair") {
        generate_key_pair();
        return;
    }

    match app.value_of("type").unwrap_or("p2pkh") {
        "p2pkh" => {
            let private_key = app.value_of("private_key").expect("invalid private key");

            title("Generating P2PKH Address");
            let address = p2pkh(&private_key);
            println!("P2PKH Address: \n{}", address);
            end();
        }
        "p2sh" => {
            let spending_pub_key = app
                .value_of("spending_pub_key")
                .expect("failed to get pubkey");
            let spending_pub_key = spending_pub_key.split(",").collect::<Vec<&str>>();

            title("Generating P2SH address");
            if spending_pub_key.len() == 1 {
                let address = p2sh(spending_pub_key[0]);
                println!("P2SH Address: \n{}", address);
            } else if spending_pub_key.len() == 3 {
                let address = p2sh_multisig(&spending_pub_key);
                println!("P2SH Address: \n{}", address);
            } else {
                println!("invalid spending pub keys");
            }
            end();
        }
        _ => panic!("address type not implemented"),
    }
}

fn p2pkh(private_key: &str) -> String {
    let private_key = hex::decode(private_key).expect("Invalid private key supplied");
    let secp = Secp256k1::new();
    let secret_key = SecretKey::from_slice(&private_key).expect("32 bytes, within curve order");
    let public_key = PublicKey::from_secret_key(&secp, &secret_key).to_string();

    // Sha256
    let mut hasher = Sha256::new();
    hasher.input(&hex::decode(&public_key).expect("should decode"));
    let result = hasher.result();

    // Ripemd160
    let mut ripemd_hash = Ripemd160::new();
    ripemd_hash.input(&result);
    let ripemd_result = ripemd_hash.result().as_slice().to_owned();

    // Base58Check encoding
    ripemd_result.to_base58check(0)
}

fn title(title: &str) {
    println!("================= {} =================", title);
}

fn end() {
    println!("======================================");
}

fn p2sh(spending_pub_key: &str) -> String {
    // 0x21 is the bitcoin push data OPCODE
    // 0xac is the bitcoin OPCODE for CHECKSIG
    //
    // This is simple Bitcoin script the enables only the spending pubkey to spend
    // any UTXO sent to the P2SH address
    // generate redeem script
    //
    let redeem_script = format!("21{}ac", spending_pub_key);
    println!("Redeem Script: \n{}", redeem_script);
    generate_p2sh_address(&redeem_script)
}

fn p2sh_multisig(spending_pub_keys: &[&str]) -> String {
    // Redeem Script for 2-3 multisig address
    //
    // 52 \ 21 \ {pub_key} \ 21 \ {pub_key} \ 21 \ {pub_key} \ 53 \ ae
    // 0x52 is Bitcoin opcode for constant 2
    // 0x21 is Bitcoin push data opcode pushes the pub keys on to the stack
    // 0x53 is Bitcoin opcode for constant 3
    // 0xae is the Bitcoin OPCODE for OP_CHECKMULTISIG
    //
    // This is a redeems
    let redeem_script = format!(
        "5221{}21{}21{}53ae",
        spending_pub_keys[0], spending_pub_keys[1], spending_pub_keys[2]
    );
    println!("Redeem Script: \n{}", redeem_script);
    generate_p2sh_address(&redeem_script)
}

fn generate_p2sh_address(redeem_script: &str) -> String {
    let mut sha2 = Sha256::new();
    sha2.input(&hex::decode(&redeem_script).expect("fialed to deserialize redeem script"));
    let result = sha2.result();

    let mut ripemd_hash = Ripemd160::new();
    ripemd_hash.input(&result);
    let ripemd_result = ripemd_hash.result().as_slice().to_owned();

    ripemd_result.to_base58check(5)
}

fn generate_key_pair() {
    title("key pair");
    use secp256k1::rand::rngs::OsRng;
    let secp = Secp256k1::new();
    let mut rng = OsRng::new().unwrap();
    let (secret_key, public_key) = secp.generate_keypair(&mut rng);
    println!("Private Key");
    println!("{}", secret_key.to_string());
    println!("Public Key");
    println!("{}", public_key.to_string());
    end();
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn should_generate_correct_p2pkh_address() {
        assert_eq!(
            "16JrGhLx5bcBSA34kew9V6Mufa4aXhFe9X",
            p2pkh("a966eb6058f8ec9f47074a2faadd3dab42e2c60ed05bc34d39d6c0e1d32b8bdf")
        );
    }
    #[test]
    fn should_generate_correct_p2sh_address() {
        assert_eq!(
            "38RgUAR367PFbFFgS57BYcERHkpqHEMBvA",
            p2sh("020ae29f86f404e4b302cfa17ff15d93149af6a54c80a4172d47e41f55f6a78d73")
        );
    }
    #[test]
    fn should_generate_correct_multisig_p2sh_address() {
        assert_eq!(
            "3DD4YP2T75TQtf84KrHzYVLYgNAeaHWqxq",
            p2sh_multisig(&[
                "020ae29f86f404e4b302cfa17ff15d93149af6a54c80a4172d47e41f55f6a78d73",
                "03664d528eb80096671ef9011c533ceb5df133238e3690d88f2960c786398b86b1",
                "029a449ea4a2155ea10002d704604bb3e8606631d35af20889a74b82b2dab572f6"
            ])
        );
    }
}
