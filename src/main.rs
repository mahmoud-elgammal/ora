#[allow(unused_imports)]
use error::TableErrors;
use storage::storage::{Storage, Tuple};
use vault::vault::Vault;
use std::collections::HashMap;
use std::env;
use std::string::String;

mod encryptor;
mod error;
mod table;
mod vault;
mod storage;
mod register;
mod cli;


fn main() -> Result<(), ()> {
    // let config 
    let vault = Vault::new("personal-vault".to_string(), "".to_string(), "password".to_string());

    let args: Vec<String> = env::args().collect();

    let command = &args[1];
    let args = &args[2..];


    if command == "secrets" || command == "s" {
        secrets(args, &vault);
    }

    Ok(())
}


fn secrets (args: &[String], vault: &Vault) {
        let command = &args[0];
        let args = &args[1..];

        if command == "filter" || command == "f" {
            return filter(args, vault);
        }

        println!("usage: secrets <command> <args>");
}

fn filter(args: &[String], vault: &Vault) {
    let index: usize = 0;
    let mut query: HashMap<String, String> = HashMap::new();

        let command = &args[0];
        let re = regex::Regex::new(r"(\w+)=([^\s]+)").unwrap();
        let captures = re.captures(command).expect("usage: secrets <command> <key>=<value>");

        let key = captures.get(1).unwrap().as_str();
        let value = captures.get(2).unwrap().as_str();

        query.insert(key.to_string(), value.to_string());


        let raws = vault.select("Basic Credentials".to_string(), query);

        raws.display()
}

/*
we must reach 4x
Storage (x)
Encryptor (x)
interpreter (?)

commands:
whoami
where

        secrets insert <document name | document id | document short name> <secret> (if secret is in-complete turn into interactive mode)
        select <document name | document id | document short name> -filter <key> <value>
        delete <document name | document id | document short name>
        
        commit
        
        use <path>
        
        ora create <document name> -t <tuple name>:<type>
        
        ***********************
        *   File Header       * (magic bytes, version, hashed password, encryption algorithm, salt, iv, nonce, tag, document count)
        ***********************
        *   Document Header   *
        ***********************
        *   Tuple Header      *
        ***********************
        *   Tuple            *
        ***********************
        * Environments        *
        **********************
        
    Vault {
        magic_bytes: Vec<u8>,
        name: String,
        version: String,
        hashed_password: Vec<u8>,
        algorithm: String,
        salt: Vec<u8>,
        iv: Vec<u8>,
        nonce: Vec<u8>,
        tag: Vec<u8>,
        document_count: u32,
        documents: HashMap<String, Document>,
        }
        
        Document {
            name: String,
            size: u32,
            tuples: Vec<Tuple>,
            representations: Vec<usize>,
            }
            
            Vault -> Secrets -> Tuples

            ora create secret <secret name> -t <tuple name>:<type>
            ora insert secret <secret name>
            
            ora g password
            -> iokjws9999
            
            ora reg $0 ($ event on $1)
            -> iokjws9999
            
            */
            
            /*
            
            Today Tasks
            - Save the vault
            - Load the vault
            - register
            
            Commands that need to be implemented
            - Insert <secret name> -<key> <value> -<key> <value>
            - Select <secret name> -filter <key> <value>
            - Delete <secret name>
            
            */

            // vault.insert("insert my facebook password");