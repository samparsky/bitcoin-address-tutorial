use clap::{Arg, App};
use secp256k1::{Secp256k1, SecretKey, PublicKey};
use sha2::{Sha256};
use ripemd160::{Ripemd160, Digest};
use base58check::ToBase58Check;

fn main() {
    let app = App::new("Bitcoin Address Generator")
                .version("1.0")
                .about("Simple Rust tutorial to generate different types of Bitcoin Addresses i.e. P2PKH, P2SH")
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
}

fn p2pkh(private_key: &str) {
    
    title("Generating P2PKH Address");

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
    let address = ripemd_result.to_base58check(0);

    println!("P2PKH Address: \n{}", address);

    end();
}


fn title(title: &str) {
    println!("================= {} =================", title);   
}

fn end() {
    println!("======================================");
}