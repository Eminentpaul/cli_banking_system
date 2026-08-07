#![allow(unused)]

use std::{format, io::{self, BufReader, BufWriter}, println};
use serde::{Serialize, Deserialize};
use std::fs::File;


#[derive(Serialize, Deserialize, Debug, Clone)]
struct Account {
    account_name: String,
    account_number: String,
    phone_number: String,
    balance: f64
}


impl Account {
    fn new(name: &str, phone_no:&str) -> Self {
        let acct_no = generate_account_number(phone_no);

        Self {
            account_name: String::from(name), 
            account_number: acct_no, 
            phone_number: String::from(phone_no), 
            balance: 0.0
        }
    }
}

fn main() {
    println!("CLI BANKING SYSTEM");
    
    loop {
        println!("List of Operations:\n1. Create Account\n2. View Account\n3. Deposit Money\n4. Withdraw Money\n5. Transfer Money\n6. List Accounts\n7. Exit");

        let input = user_input("Select an Option:");

        match input.trim() {
            "1" => {
                let full_name = user_input("Enter your Full Name:");
                let phone_number = user_input("Enter your Phone Number");

                let phone_number = match phone_number.trim().parse::<i64>() {
                    Ok(num) => num,
                    Err(err) => {
                        println!("Phone number should be only number digits");
                        return;
                    }
                };

                // let phone_number = phone_number.to_string();

                let phone_number = format!("0{}", phone_number.to_string());
                if phone_number.len() < 11 {
                    println!("Phone number should be up to 11 digits")
                }else {
                    // println!("Number: {}", phone_number);
                    create_account(&full_name, &phone_number);
                }
            },


            _ => {
                println!("Invalid Input")
            }
        }
    }
}

fn generate_account_number(phone_no:&str) -> String {
    let number = (phone_no.len()-10);
    let new_number = &phone_no[number..];

    String::from(new_number)
}

fn user_input(option:&str) -> String {

    println!("{}", option);
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("No input found!");


    input
}

fn load_data(path:&str) -> Vec<Account>{

    let file = match File::open(path) {
        Ok(file ) => file,
        Err(err) => {
            // println!("File failed to open with the following error: {}", err);
            return Vec::new();
        }
    };

    let _reader = BufReader::new(file);

    let db: Vec<Account> = match serde_json::from_reader(_reader) {
        Ok(tasks) => tasks,
        Err(err) => {
            println!("Cant Deserialize the file with following err: {}", err);
            return Vec::new();
        }
    };

    db
}


fn save_to_db(path:&str, database: Vec<Account>) -> bool {
    let file = match File::create(path) {
        Ok(file) => file,
        Err(err) => {
            println!("File not Created with the error: {}", err);
            return false;
        }
    };

    let _writer = BufWriter::new(file);

    serde_json::to_writer_pretty(_writer, &database);

    true
}



fn create_account(full_name:&str, phone_no:&str) {
    let path = "database.json";
    let mut db = load_data(path);


    for account in db.iter(){
        if account.account_number == generate_account_number(phone_no){
            println!("The Phone number has been used!");
            return;
        }
    }

    let new_account = Account::new(full_name, phone_no);

    db.push(new_account);

    let saved = save_to_db(path, db);

    if saved{
        println!("Account Created Successfully!");
    }else {
        println!("Account Not Creaeted")
    }

}