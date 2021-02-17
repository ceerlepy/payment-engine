mod charge_back;
mod client;
mod deposit;
mod dispute;
mod payment_process;
mod resolve;
mod transaction;
mod withdrawal;

use charge_back::ChargeBack;
use client::ClientAccount;
use deposit::Deposit;
use dispute::Dispute;
use payment_process::PaymentProcess;
use resolve::Resolve;
use transaction::{ Transaction, TransactionType};
use withdrawal::WithDrawal;

use std::error::Error;
use std::collections::HashMap;
use std::process;

use csv::{Reader, Writer};

fn read_transactions(client_hashmap: &mut HashMap<String, ClientAccount>, transaction_hashmap: &mut HashMap<String, (f64, bool)>) -> Result<(), Box<dyn Error>> {
    // Build the CSV reader and iterate over each record.
    let mut rdr = Reader::from_path(std::env::args().nth(1).unwrap())?;
    for result in rdr.records() {
        // The iterator yields Result<StringRecord, Error>, so we check the
        // error here.
        let record = result?;
        let transaction: Transaction = record.deserialize(None)?;
        process_transaction(transaction, client_hashmap, transaction_hashmap);
    }
    Ok(())
}

fn process_transaction(transaction: Transaction, client_hashmap: &mut HashMap<String, ClientAccount>, transaction_hashmap: &mut HashMap<String, (f64, bool)>){

    let client_account = client_hashmap.entry(transaction.client.to_string()).or_insert(ClientAccount::new(transaction.client));
    let (amount, dispute) = *transaction_hashmap.entry(transaction.tx.to_string()).or_insert((transaction.amount, false));
    let is_ready_for_resolve_and_charge_back = transaction_hashmap.contains_key(&transaction.tx.to_string()) && dispute;

    match TransactionType::from_string(transaction.transaction_type){
        Ok(TransactionType::Deposit) => {
            process(&mut Deposit{client_account, amount: transaction.amount});
        }
        Ok(TransactionType::Withdrawal) => {
            process(&mut WithDrawal{client_account, amount: transaction.amount});
        },
        Ok(TransactionType::Dispute) => {
            process(&mut Dispute{client_account, amount});
            transaction_hashmap.insert(transaction.tx.to_string(), (amount, true));
        },
        Ok(TransactionType::Resolve) => {
            if is_ready_for_resolve_and_charge_back {
                process(&mut Resolve{client_account, amount});
                transaction_hashmap.insert(transaction.tx.to_string(), (amount, false));
            }else{
                println!("Resolve operation is failed bacuse dispute operation or existing transaction could not found");
            }    
        },
        Ok(TransactionType::Chargeback)=> {
            if is_ready_for_resolve_and_charge_back {
                process(&mut ChargeBack{client_account, amount});
                transaction_hashmap.insert(transaction.tx.to_string(), (amount, false));
            }else{
                println!("ChargeBack operation is failed bacuse dispute operation or existing transaction could not found");
            }
        },
        Err(err) => println!("Error: {:?}", err)
    }

}

fn process(ps: &mut dyn PaymentProcess){
    ps.process();
}

fn write_client_accounts(client_hashmap: &HashMap<String, ClientAccount>) -> Result<(), Box<dyn Error>> {

    let output_file = match std::env::args().nth(2){
        Some(filename) => filename,
        None => "accounts.csv".to_string()
    };

    let mut wtr = Writer::from_path(output_file)?;

    println!("client,available,held,total,locked");

    for (_, client_account) in client_hashmap{
        print!("{},", client_account.client);

        let available = if client_account.available.fract() != 0.0 {
            print!("{:.4},", client_account.available);
            format!("{:.4}", client_account.available).parse::<f64>().unwrap()
        } else {
            print!("{},", client_account.available);
            format!("{}", client_account.available).parse::<f64>().unwrap()
        };

        let held = if client_account.held.fract() != 0.0 {
            print!("{:.4},", client_account.held);
            format!("{:.4}", client_account.held).parse::<f64>().unwrap()
        } else {
            print!("{},", client_account.held);
            format!("{}", client_account.held).parse::<f64>().unwrap()
        };

        let total = if client_account.total.fract() != 0.0 {
            print!("{:.4},", client_account.total);
            format!("{:.4}", client_account.total).parse::<f64>().unwrap()
        } else {
            print!("{},", client_account.total);
            format!("{}", client_account.total).parse::<f64>().unwrap()
        };

        println!("{},", client_account.locked);

        wtr.serialize(ClientAccount {
            available,
            held,
            total,
            .. *client_account
        })?;
    }

    wtr.flush()?;
    Ok(())
}

fn main() {

    let mut client_hashmap: HashMap<String, ClientAccount> = HashMap::new();
    let mut transaction_hashmap: HashMap<String, (f64, bool)> = HashMap::new();

    if let Err(err) = read_transactions(&mut client_hashmap, &mut transaction_hashmap) {
        println!("error running example: {}", err);
        process::exit(1);
    }

    match write_client_accounts(&client_hashmap) {
        Ok(_) => println!("Completed!"),
        Err(err) => println!("Error : {}", err)
    }

}