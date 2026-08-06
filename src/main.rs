#![allow(unused)]

use std::println;



struct Account {
    account_name: String,
    account_number: String,
    phone_number: String,
    balance: f64
}


impl Account {
    fn new(name: &str, phone_no:&str) -> Self {
        let mut account_no = String::new();

        if phone_no.len() < 11 {
            println!("Phone number has be 11 characters");
        }else {
            account_no = generate_account_number(phone_no);
        }

        
        Self {
            account_name: String::from(name), 
            account_number: account_no, 
            phone_number: String::from(phone_no), 
            balance: 0.0
        }
    }
}

fn main() {
    
}

fn generate_account_number(phone_no:&str) -> String {
    let number = (phone_no.len()-10);
    let new_number = &phone_no[number..];

    String::from(new_number)
}