#![allow(unused)]

use std::{io::{self, BufReader, BufWriter}};
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
    let path = "database.json";
    
    loop {
        println!("List of Operations:\n1. Create Account\n2. View Account\n3. Deposit Money\n4. Withdraw Money\n5. Transfer Money\n6. List Accounts\n7. Exit");

        let input = user_input("Select an Option:");

        match input.trim() {
            "1" => {
                let full_name = user_input("Enter your Full Name:");
                let phone_number = user_input("Enter your Phone Number");

                match check_phone_no(&phone_number) {
                    Ok(phone) => {

                        create_account(&full_name, &phone);
                    },
                    Err(err) => {
                        println!("{}", err);
                    }
                }
                
                
            },

            "2" => {
                let phone_no = &user_input("Enter your Phone number: ");
                
                let phone_no = match check_phone_no(phone_no) {
                    Ok(phone) => phone,
                    Err(err) => {
                        println!("{}", err);
                        return;
                    }
                };

                view_account(&phone_no, path);
                
                
                
            },

            "3" => {
                let phone_no = &user_input("Enter your Phone number: ");
                
                let amount = &user_input("Enter Amount: ");
                let amount = match check_amount(amount) {
                    Ok(amount) => amount,
                    Err(err) => {
                        println!("{}", err);
                        return; 
                    }
                };

                let phone_number = match check_phone_no(phone_no) {
                    Ok(phone) => phone,
                    Err(err) => {
                        println!("{}", err);
                        return;
                    }
                };

                if deposit_money(&phone_number, amount, path){
                        println!("N{:.2} deposited Successfully into the account", amount)
                    }else {
                        println!("Deposit not Successful!")
                    }
                
            },

            "4" => {
                let phone_no = &user_input("Enter your Phone number: ");
                
                let amount = &user_input("Enter Amount: ");
                let amount = match check_amount(amount) {
                    Ok(amount) => amount,
                    Err(err) => {
                        println!("{}", err);
                        return; 
                    }
                };

                let phone_number = match check_phone_no(phone_no) {
                    Ok(phone) => phone,
                    Err(err) => {
                        println!("{}", err);
                        return;
                    }
                };

                if withdraw_money(&phone_number, amount, path){
                        println!("N{:.2} withdrawed Successfully from the account", amount)
                    }else {
                        println!("Withdrawal not Successful!")
                    }
                
            },

            "5" => {
                let sender_phone_number = user_input("Enter Sender's Phone Number:");
                let sender_phone_number = match check_phone_no(&sender_phone_number) {
                    Ok(phone) => phone,
                    Err(err) => {
                        println!("Sender: {}", err);
                        return;
                    }
                };

                let mut sender_account = match check_account_availability(&sender_phone_number, path, "Sender") {
                    Ok(acc) => acc,
                    Err(err) => {
                        println!("{}", err);
                        return;
                    }
                };

                let receiver_phone_number = user_input("Enter Receiver's Phone Number:");
                let receiver_phone_number = match check_phone_no(&receiver_phone_number) {
                    Ok(phone) => phone,
                    Err(err) => {
                        println!("Sender: {}", err);
                        return;
                    }
                };

                let mut receiver_account = match check_account_availability(&receiver_phone_number, path, "Receiver") {
                    Ok(acc) => acc,
                    Err(err) => {
                        println!("{}", err);
                        return;
                    }
                };

                let amount = user_input("Enter Amount:");
                let amount = match check_amount(&amount) {
                    Ok(amount) => amount,
                    Err(err) => {
                        println!("{}", err);
                        return;
                    }
                };

                if transfer(&mut sender_account, &mut receiver_account, amount, path){
                    println!("Transaction Successful")
                }else {
                    println!("Transaction not Successful")
                }
            },

            "6" => {
                println!("List of All the Accounts");
                list_accounts(path);
            }

            "7" => {
                println!("Exiting.....");
                break;
            },


            _ => {
                println!("Invalid Input")
            }
        }
    }
}

fn generate_account_number(phone_no:&str) -> String {
    let number = phone_no.len()-10;
    let new_number = &phone_no[number..];
    String::from(new_number)
}

fn user_input(option:&str) -> String {

    println!("{}", option);
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("No input found!");


    input
}


fn check_phone_no(phone_no: &str) -> Result<String, String> {
    // let phone_number = match phone_no.trim().parse::<i64>() {
    //     Ok(num) => num,
    //     Err(err) => {
    //         println!("Phone number should be only number digits");
    //         return "".to_string();
    //     }
    // };

    // // let phone_number = phone_number.to_string();

    // let phone_number = format!("0{}", phone_number.to_string());
    // if phone_number.len() != 11 {
    //     println!("Phone number should be up to 11 digits");
    //     "".to_string()
    // }else {
    //     // println!("Number: {}", phone_number);
    //     phone_number
    // }

    let phone_no = phone_no.trim();

    if !phone_no.chars().all(|c| c.is_ascii_digit()) {
        return Err("Phone number should contain only digits".to_string());
    }

    if !phone_no.starts_with("0") {
        return Err("Phone Number must start with zeror".to_string());
    }

    if phone_no.len() != 11 {
        return Err("Phone number must be 11 digits".to_string());
    }



    Ok(phone_no.to_string())
}



fn check_amount(amount: &str) -> Result<f64, String> {
    let amount = match amount.trim().parse::<f64>() {
        Ok(num) => num,
        Err(err) => {
            return Err("Invalid Input for the Amount".to_string());
            
        }
    };

    Ok(amount)
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

fn user_data_output(account:&Account){
    println!(
        "\nFull Name: {}Account Number: {}\nPhone Number: {}\nAccount Balance: {:.2}\n==============================\n",
        account.account_name,
        account.account_number,
        account.phone_number,
        account.balance
    )
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


fn check_account_availability(phone_no: &str, path:&str, option: &str) -> Result<Account, String> {
    let database = load_data(path);

    for account in database.iter(){
        if account.account_number == generate_account_number(phone_no) {
            return Ok(account.clone());
        }
    }
    let error = format!("{} Account Not available", option);
    return Err(error.to_string());
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


fn view_account(phone_no:&str, path:&str) {
    let acct_number = generate_account_number(phone_no);
    let db = load_data(path);

    for account in db.iter(){
        if account.account_number == acct_number {
            println!("User Account Information:");
            user_data_output(account);
            // return;
        }else {
            println!("User Account not Found!")
        }
    }
}



fn deposit_money(phone_no: &str, amount: f64, path: &str) -> bool {
    let acct_number = generate_account_number(phone_no);

    let mut db = load_data(path);

    for account in db.iter_mut() {
        if account.account_number == acct_number {
            account.balance += amount;

            save_to_db(path, db);

            return true;
        }
    }

    println!("Account not Found!");
    false
}


fn withdraw_money(phone_no: &str, amount: f64, path: &str) -> bool {
    let acct_number = generate_account_number(phone_no);

    let mut db = load_data(path);

    for account in db.iter_mut() {
        if account.account_number == acct_number {
            if account.balance < amount {
                println!("Insufficient Balance for withdrawal");
                return false;
            }else {
                account.balance -= amount;

                save_to_db(path, db);

                return true;
            }
        }
    }

    println!("Account not Found!");
    false
}



fn transfer(sender: &mut Account, receiver: &mut Account, amount: f64, path:&str) -> bool {
    let mut db = load_data(path);

    // for account in db.iter_mut(){
    //     if account.account_number == sender.account_number {
    //         if sender.balance < amount {
    //             println!("Insufficient Fund to complete the transsaction");
    //             return false;
    //         }else {
    //             if account.account_number == receiver.account_number {
    //                 sender.balance -= amount;
    //                 receiver.balance += amount;

    //                 save_to_db(path, db);
    //                 return true
    //             }
    //         }
    //     }
    // }

    // false

    
    let sender_index = match db.iter().position(|c| c.account_number == sender.account_number) {
        Some(index ) => index,
        None => {
            println!("Sender Account not found!");
            return  false;
        }
    };

    let receiver_index = match db.iter().position(|c| c.account_number == receiver.account_number) {
        Some(index) => index,
        None => {
            println!("Receiver Account not found!");
            return false;
        }
    };

    if sender_index == receiver_index {
        println!("Cannot Transfer fund between the same account");
        return false;
    }

    if db[sender_index].balance < amount {
        println!("Insufficient Fund to complete the Transaction");
        return false;
    }

    db[sender_index].balance -= amount;
    db[receiver_index].balance += amount;
    save_to_db(path, db);

    true
}


fn list_accounts(path:&str) {
    let db = load_data(path);

    for account in db.iter() {
        user_data_output(account);
    }
}