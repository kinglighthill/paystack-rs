//! Transaction
//! ===========
//!
//! Reference: <https://paystack.com/docs/api/transaction/>
//!
//! This example shows how to initiate a transaction
//! for a particular price and a particular customer.
//! The transaction generates a URL that the user can use to pay.
//! This reqires building a transaction body.
//! Please see the type definition to understand how it is constructed

use paystack::{PaystackClient, TransactionBody};

#[tokio::main]
async fn main() {
    let api_key = "API KEY";
    let client = PaystackClient::new(api_key);

    let body = TransactionBody {
        email: "CUSTOMER EMAIL".to_string(),
        amount: "AMOUNT".to_string(),
        currency: Some("CURRENCY CODE".to_string()),
        plan: None,
        transaction_charge: None,
        bearer: None,
    };

    let transaction = client
        .initialize_transaction(body)
        .await
        .expect("Unable to create transaction");

    println!(
        "Created a payment payment URL: {}",
        transaction.data.authorization_url
    );
    println!("Transaction creation status: {}", transaction.status);
    println!("Transaction message: {}", transaction.message);

    // Verify transaction
    // Transaction reference can be a string or pulled out from the transaction response
    let transaction_status = client
        .verify_transaction("TRANSACTION REFERENCE".to_string())
        .await
        .expect("Unable to get transaction status");

    println!("Status: {}", transaction_status.data.status);
    println!(
        "Amount of {} {}",
        transaction_status.data.amount, transaction_status.data.currency
    );
    println!("Channel: {}", transaction_status.data.channel);
}