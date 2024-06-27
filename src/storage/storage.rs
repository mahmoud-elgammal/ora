use std::{collections::HashMap, vec};

use bson::{bson, document, to_bson, Bson};
use serde::{Deserialize, Serialize};

use crate::{encryptor::encryptor::Encryptor};

use uuid::Uuid;

// document-oriented storage

#[derive(Debug)]
pub enum Error {
    NotFound,
    AlreadyExists,
    // InvalidSecret,
    // InvalidFormat,
    // InvalidTuple,
    // InvalidRecord,
    InvalidTupleSize,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum SecretType {
    Password,
    Token,
    Certificate,
    Key,
    EncryptionKey,
    SignatureKey,
    Email,
    Phone,
    Address,
    Text,
    Number,
    Url,
    Date,
    Time,
    DateTime,
    Binary,
    Domain,
    Hash,
    JSON,
    YAML,
}

#[derive(Clone, Debug)]
pub struct Raw {
    data: HashMap<String, String>
}

impl Raw {
    pub fn new(data: HashMap<String, String>) -> Raw {
        Raw {
            data
        }
    }

    pub fn display(&self) {
        println!("(ora)> here's your secret, don't tell anyone 🤫");
        for (key, value) in &self.data {
            println!("\t{}: {}", key, value);
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Record {
    hash: String,
    value: String,
    version: u32,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Tuple {
    // like columns
    name: String,
    type_: SecretType,
    records: Vec<Record>, // like rows
    hash: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Secret {
    // like tables
    hash: String,
    name: String,
    size: usize,
    representations: Vec<usize>,
    tuples: Vec<Tuple>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Storage {
    // like database
    hash: String,
    secrets: Vec<Secret>,
    encryptor: Encryptor,
    path: String,
    size: usize,
}


impl Record {
    pub fn new(hash: String, value: String) -> Record {
        Record {
            hash,
            value,
            version: 0,
        }
    }
}

impl Tuple {
    pub fn new(name: String, type_: SecretType) -> Tuple {
        Tuple {
            name,
            type_,
            records: Vec::new(),
            hash: Uuid::new_v4().to_string(),
        }
    }
}

impl Storage {
    pub fn new(secret_key: String, path: String) -> Storage {
        Storage {
            secrets: Vec::new(),
            encryptor: Encryptor::new("AES_256_GCM".to_string(), secret_key),
            path,
            hash: Uuid::new_v4().to_string(),
            size: 0,
        }
    }

    pub fn create_document(&mut self, name: String, tuples: Vec<Tuple>, representations: Vec<usize>) -> Result<Secret, Error> {
        // pre-conditions
        if self.secrets.iter().any(|doc| doc.name == name) {
            return Err(Error::AlreadyExists);
        }

        let document = Secret {
            name,
            size: 0,
            tuples,
            representations,
            hash: Uuid::new_v4().to_string(),
        };

        self.secrets.push(document.clone());
        self.size += 1;
        Ok(document)
    }

    pub fn insert(&mut self, document: Secret, raw: Vec<String>) -> Result<(), Error> {
        // pre-conditions
        if document.tuples.len() != raw.len() {
            print!("{} vs {}", document.tuples.len(), raw.len());
            return Err(Error::InvalidTupleSize);
        }

        for _document in &mut self.secrets {
            if _document.name == document.name {
                let hash = Uuid::new_v4().to_string();

                for i in 0.._document.tuples.len() {
                    let value = self.encryptor.clone().encrypt(raw[i].clone());
                    _document.tuples[i].records.push(Record::new(hash.clone(), value));
                }
                return Ok(());
            }
        }

        return Err(Error::NotFound);
    }

    pub fn read(&self, document_name: String, query: HashMap<String, String>) -> Result<Raw, Error> {
        for _document in &self.secrets {
            if _document.name == document_name {
                // let value = self.encryptor.clone().decrypt(_document.tuples[0].records.clone()[0].value.clone());
                // return  Ok(value);

                let mut index: usize = 0;
                let mut found = false;
                let selector: usize = 0;

                for i in 0.._document.tuples[selector].records.len() {
                    let value = self.encryptor.clone().decrypt(_document.tuples[selector].records[i].value.clone());

                    if query.get("email") == Some(&value) {
                        index = i;
                        found = true;
                        break;
                    }
                }

                if !found {
                    return Err(Error::NotFound);
                }

                // make it in hashmap
                let mut result: HashMap<String, String> = HashMap::new();

                for i in 0.._document.tuples.len() {
                    let value = self.encryptor.clone().decrypt(_document.tuples[i].records[index].value.clone());
                    result.insert(_document.tuples[i].name.clone(), value);
                }

                let raw = Raw::new(result);
                return Ok(raw);
            }
        }

        return Err(Error::NotFound);
    }
}
