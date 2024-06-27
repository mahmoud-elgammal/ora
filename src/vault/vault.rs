use std::collections::HashMap;
use std::fs::File;
use std::io::{Write, BufReader, BufRead, Error};

use bson::{to_bson};
use serde::{Deserialize, Serialize};

// mod encryptor;
use crate::storage::storage::{Raw, Storage};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Vault {
    name: String,
    path: String,
    version: String,
    pub storage: Storage,
}

impl Vault {
    pub fn new(name: String, path: String, secret_key: String) -> Vault {
        Vault {
            name,
            path: path.clone(),
            storage: Storage::new(secret_key, path),
            version: "1.0".to_string(),
        }
    }

    pub fn save(&self) -> Result<(), ()>{
            // Create a file in write-only mode
            let data = to_bson(&self).unwrap();

            // print!("{:?}", file);

            let mut file = File::create(self.path.clone()).unwrap();

            // let mut output = File::create(path)?;
            file.write(data.to_string().as_bytes()).unwrap();

            // let input = File::open(path);
            // let buffered = BufReader::new(input);

            // for line in buffered.lines() {
            //     println!("{}", line?);
            // }
            Ok(())
    }

    pub fn insert(&self, data: &str) {
        // println!("{}", data)
    }

    pub fn select(&self, document_name: String, query: HashMap<String, String>) -> Raw {
       self.storage.read(document_name, query).unwrap()
    }

}