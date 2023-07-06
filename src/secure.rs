use jsonwebtoken::{decode, encode, Algorithm, Header, Validation, EncodingKey, DecodingKey, errors::ErrorKind};
use serde::{Serialize, de::DeserializeOwned};
use std::{path::Path, fs::{File, read_to_string}, io::prelude::*};
use openssl::{pkey::Private, rsa::Rsa, hash::{Hasher, MessageDigest}};
use lazy_static::lazy_static;

use crate::{types::CacheKeyData, utils::DEFAULT_FILE_PATH};

pub const PRIVATE_KEY_FILE: &str = "private.pem";
pub const PUBLIC_KEY_FILE: &str = "public.pem";

lazy_static! {
    static ref KEY: KeyPair = {
        KeyPair::new(Algorithm::RS256).unwrap_or_else(|err| {
            panic!("Error loading encryption key: {}", err);
        })
    };
}

pub struct KeyPair {
    private: EncodingKey,
    public: DecodingKey,
    algorithm: Algorithm
}

pub enum JWTError {
    Expired,
    Invalid
}

impl KeyPair {
    #[allow(unused_assignments)]
    pub fn new(algorithm: Algorithm) -> Result<Self, Box<dyn std::error::Error>> {
        let mut private_key: Option<EncodingKey> = None;
        let mut public_key: Option<DecodingKey> = None;

        let private_key_path = format!("{}/{}", *DEFAULT_FILE_PATH, PRIVATE_KEY_FILE);
        let public_key_path = format!("{}/{}", *DEFAULT_FILE_PATH, PUBLIC_KEY_FILE);
        
        if !Path::new(&private_key_path).exists() || !Path::new(&public_key_path).exists() {
            let rsa_key: Rsa<Private> = Rsa::generate(2048)?;

            let private_pem = rsa_key.private_key_to_pem()?;
            let public_pem = rsa_key.public_key_to_pem()?;

            File::create(&private_key_path)?.write_all(&private_pem)?;
            File::create(&public_key_path)?.write_all(&public_pem)?;

            private_key = Some(EncodingKey::from_rsa_pem(&private_pem)?);
            public_key = Some(DecodingKey::from_rsa_pem(&public_pem)?);
        } else {
            private_key = Some(EncodingKey::from_rsa_pem(&read_to_string(&private_key_path)?.as_bytes())?);
            public_key = Some(DecodingKey::from_rsa_pem(&read_to_string(&public_key_path)?.as_bytes())?);
        }

        Ok(Self {
            private: private_key.unwrap(),
            public: public_key.unwrap(),
            algorithm
        })
    }

    pub fn sign_jwt<T>(&self, claims: &T) -> Result<String, Box<dyn std::error::Error>>
    where
        T: Serialize
    {
        Ok(encode(&Header::new(self.algorithm), claims, &self.private)?)
    }

    pub fn decode_jwt<T>(&self, token: &str) -> Result<jsonwebtoken::TokenData<T>, JWTError>
    where
        T: DeserializeOwned
    {
        let validation = Validation::new(self.algorithm);
        match decode::<T>(token, &self.public, &validation) {
            Ok(jwt) => Ok(jwt),
            Err(err) => match err.kind() {
                ErrorKind::ExpiredSignature => Err(JWTError::Expired),
                _ => Err(JWTError::Invalid)
            }
        }
    }

    // TODO: create a function which is able to encrypt data or verify api server
    // pub fn encryption<T>(&self, content: T) -> [u8]
    // where
    //     T: Serialize
    // {

    // }
}

pub fn sign_jwt<T>(claims: &T) -> Result<std::string::String, Box<dyn std::error::Error>>
where
    T: Serialize
{
    KEY.sign_jwt::<T>(claims)
}

pub fn decode_jwt<T>(token: &str) -> Result<jsonwebtoken::TokenData<T>, JWTError>
where
    T: DeserializeOwned
{
    KEY.decode_jwt::<T>(token)
}

pub fn create_hash(algorithm: MessageDigest, data: &[u8]) -> Vec<u8> {
    let mut hasher = Hasher::new(algorithm).expect("Cannot create hasher");
    hasher.update(data).expect("Cannot update hash");
    hasher.finish().expect("Cannot complete hash").to_vec()
}

pub fn create_cache_key(school_number: &str, username: &str, class_name: &str) -> CacheKeyData {
    let data = &[school_number.as_bytes(), username.as_bytes(), class_name.as_bytes()].concat();
    let hash = create_hash(MessageDigest::sha512(), &data).to_ascii_lowercase();

    CacheKeyData {
        id: hash[0..32].to_owned(),
        key: hash[34..98].to_owned(),
        iv: hash[81..113].to_owned(),
    }
}