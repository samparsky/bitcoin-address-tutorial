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

fn p2sh(spending_pub_key: &str) {
    title("Generating P2SH address");
    // 0x21 is the bitcoin push data OPCODE 
    // 0xac is the bitcoin OPCODE for CHECKSIG
    // 
    // This is simple Bitcoin script the enables only the spending pubkey to spend
    // any UTXO sent to the P2SH address
    // generate redeem script
    // 
    let redeem_script = format!("21{}ac", spending_pub_key);
    println!("Redeem Script: \n{}",  redeem_script);
    println!("P2SH Address: \n{}", generate_p2sh_address(&redeem_script));
    end();
}

fn p2sh_multisig(spending_pub_keys: &[&str]) {
    // Redeem Script for 2-3 multisig address
    //
    // 52 \ 21 \ {pub_key} \ 21 \ {pub_key} \ 21 \ {pub_key} \ 53 \ ae
    // 0x52 is Bitcoin opcode for constant 2
    // 0x21 is Bitcoin push data opcode pushes the pub keys on to the stack
    // 0x53 is Bitcoin opcode for constant 3
    // 0xae is the Bitcoin OPCODE for OP_CHECKMULTISIG
    // 
    // This is a redeems 
    let redeem_script = format!("5221{}21{}21{}53ae", spending_pub_keys[0], spending_pub_keys[1], spending_pub_keys[2]);
    println!("Redeem Script: \n{}",  redeem_script);
    println!("P2SH Address: \n{}", generate_p2sh_address(&redeem_script));
    end();

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