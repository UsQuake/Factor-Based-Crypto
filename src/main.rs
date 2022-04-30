extern crate factor_based_crypto;
pub use factor_based_crypto::composite_oriented_crypto;
use num_bigint::BigUint;
use num_traits::{One};
use num_bigint::{ToBigUint};

fn main()
{
    let _plain  = 7993;
    println!("plain: {}", _plain);
    let (_crypto,_key) = composite_oriented_crypto::encrypt(_plain, 100);
    println!("key : {}", _key);
    println!("crypto : {}", _crypto);
    let mut f0: BigUint = 1000.to_biguint().unwrap();
    let mut _tmp : BigUint  = One::one();
    for _ in 1000..10000
    {
        f0 = f0.clone() + _tmp.clone();
        //if Attacker knows number of digits it might be vulnerability
        if _crypto.clone() % f0.clone() == 1.to_biguint().unwrap()
        {

            println!("plaintext candidates : {}", f0);
        }
    }
    println!("decrypted plain: {}", composite_oriented_crypto::decrypt(_crypto, _key));
}