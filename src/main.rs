mod conflux;
mod util;
mod http_provider;
mod transaction;
use client::accounts::{account_provider};
use std::error::Error;
use std::fmt;
use primitives::{Transaction, Action};
use cfx_types::{U256};
use cfxkey::password::Password;
use rlp::encode;
use hex::encode as hex_encode;

#[derive(Debug)]
struct CfxError{
    message: String
}

impl fmt::Display for CfxError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "CfxError: {}", self.message)
    }
}

impl Error for CfxError {
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // init the account_provider
    let acc_provider = account_provider(None, None, None)?;
    let accounts = acc_provider.accounts()
        .map_err(|_e| {CfxError{message: "error".to_string()}})?;
    println!("local the acounts {:?}", accounts);
    println!("first account {:?}", accounts[0].to_string());

    let cfx = conflux::Conflux::new("http://localhost:12537".to_string());

    let status = cfx.get_status().await?;
    println!("status {:?}", status);
    let epoch = cfx.get_epoch_number(None).await?;
    let nonce = cfx.get_next_nonce(accounts[0].to_string(), None).await?;

    let tx = Transaction{
        nonce: U256::from(nonce),
        gas_price: U256::from(100),
        gas: U256::from(0),
        action: Action::default(),
        value: U256::from(100),
        storage_limit: 1000,
        epoch_height: epoch,
        chain_id: 3436,
        data: vec![],
    };

    // let pwd = Password
    // let address = Address::from_slice("0x114673d23b3f2667ea45498bb55eeb620372d25c".as_bytes());
    // println!("hello {}", address);
    let sig = acc_provider.sign(accounts[0], Some(Password::from("123456".to_string())), tx.hash())
        .map_err(|_e| {CfxError{message: "sign error".to_string()}})?;
    let txwithsig = tx.with_signature(sig);
    let rlpencode = encode(&txwithsig);
    // println!("the encode result {:?}", rlpencode);
    // println!("hex encode {}", hex_encode(rlpencode));

    Ok(())
}

