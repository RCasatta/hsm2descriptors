use hkdf::Hkdf;
use sha2::Sha256;
use std::io::Read;
use bitcoin::util::bip32::{ExtendedPrivKey, ExtendedPubKey};
use bitcoin::Network;
use std::io;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
pub struct Options {
    #[structopt(long)]
    pub network: Network,

    #[structopt(long)]
    pub private: bool,

    #[structopt(long)]
    pub wrapped_segwit: bool,
}

fn main() -> Result<(), String> {
    let mut ikm = vec![];
    io::stdin().read_to_end(&mut ikm).map_err(|e| e.to_string())?;
    if ikm.len() < 32 {
        return Err(format!("at least 32 bytes required, provided {}", ikm.len()));
    }

    let options = Options::from_args();

    let mut salt = 0u32;
    let info = "bip32 seed".as_bytes();

    let private_key = loop {
        let h = Hkdf::<Sha256>::new(Some(&salt.to_be_bytes()[..]), &ikm);
        let mut okm = [0u8; 32];
        h.expand(&info, &mut okm).map_err(|e| e.to_string())?;
        match ExtendedPrivKey::new_master(options.network, &okm) {
            Ok(key) => break key,
            Err(_) => salt += 1,
        }
    };
    let string_key = if !options.private {
        let secp = bitcoin::secp256k1::Secp256k1::new();
        let public_key = ExtendedPubKey::from_private(&secp, &private_key);
        public_key.to_string()
    } else {
        private_key.to_string()
    };

    if options.wrapped_segwit {
        println!("sh(wpkh(({}/0/0/*))", string_key);
    } else {
        println!("wpkh({}/0/0/*)", string_key);
    }

    Ok(())
}
