//! # Factor Based Encryption Algorithm
//!
//! 'factor_based_crypto' crate is encryption library using various factor_based_algorithms
extern crate rand;
extern crate num_bigint;

/// It performs better when plain has a lot of factors.
pub mod composite_oriented_crypto{
    use rand::Rng;
    use num_traits::{Zero, One};
    use num_bigint::BigUint;
    use num_bigint::{ToBigUint};
    ///Encrypt u64-type plain.
    ///
    /// # Examples
    ///
    /// ```
    /// extern crate factor_based_crypto;
    /// pub use factor_based_crypto::composite_oriented_crypto;
    /// 
    /// let _plain  = 24;
    ///
    /// let (_crypto,_key) = composite_oriented_crypto::encrypt(_plain, 24);
    /// ```
    pub fn encrypt(plain : u64, difficulty :u32) -> (BigUint, BigUint){

        let a = 2* rand::thread_rng().gen_range(0..difficulty.clone()) + 1;
        let b = 2* rand::thread_rng().gen_range(0..difficulty.clone()) + 1;
        let x  = a * b + 2;
        let mut _tmp : BigUint  = One::one();
        let mut return_crypto: BigUint = Zero::zero();
        let mut return_key:BigUint = Zero::zero();
        for i in 0..x
        {
            return_crypto = return_crypto.clone() + _tmp.clone();
            if i < a {return_key += _tmp.clone()};
            _tmp *= plain;
        }
            (return_crypto, return_key)
    }
    ///Decrypt u64-type plain with BigUint key,crypto.
    ///
    /// # Examples
    ///
    /// ```
    /// extern crate factor_based_crypto;
    /// pub use factor_based_crypto::composite_oriented_crypto;
    /// let _plain  = 24;
    /// let _difficulty = 24;
    /// 
    /// let (_crypto,_key) = composite_oriented_crypto::encrypt(_plain, _difficulty);
    /// 
    /// let _decrypted_plain  = composite_oriented_crypto::decrypt(_crypto, _key);
    /// 
    /// assert_eq!(_plain, _decrypted_plain);
    /// ```
    pub fn decrypt(crypto : BigUint, key : BigUint) -> u64{ 
        let return_plain: BigUint  = crypto %(key.to_biguint().unwrap()) - 1.to_biguint().unwrap();
        let a = return_plain.to_u64_digits();
        return a[0];
    }


}
