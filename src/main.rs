use ed25519_dalek::{Keypair, Signer};
use rand::rngs::OsRng;

fn print_bytes<const N: usize>(message: &str, array: &[u8; N]) {
    let format = format!("{:#04X?}", array).replace(|c: char| c.is_whitespace(), "");
    println!("{message}: {format}");
}

fn main() {
    //generate keypair
    let keypair = Keypair::generate(&mut OsRng);

    //print keypair
    print_bytes("private", keypair.secret.as_bytes());
    print_bytes("public", keypair.public.as_bytes());

    //sign message with private key
    let message = b"test message";
    let signature = keypair.sign(message);

    //print message and signature
    print_bytes("message", message);
    print_bytes("signature", &signature.to_bytes());

    assert!(keypair.verify(message, &signature).is_ok());
}
