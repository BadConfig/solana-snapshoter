use serde_json;
use std::fs;
use std::collections::HashMap;
use std::fs::File;
use std::io::{Error, Write};


#[tokio::main]
async fn main() {
    let path = "./src/results.json";
    let data = fs::read_to_string(path).expect("Unable to read file");
    let input: HashMap<String,String> = serde_json::from_str(&data).unwrap();

    let client = reqwest::Client::new();

    let mut op = HashMap::<String,String>::new();
    let mut count = 0;

    for (k,v) in input.into_iter() {
        let request_data = serde_json::json!({
        "method":"getConfirmedSignaturesForAddress2","jsonrpc":"2.0",
        "params":[k,{"limit":1000u64}],
        "id":"f0e99b40-f9d3-4814-808f-aba664c62063"});
        let fetch = client.post("https://long-broken-breeze.solana-mainnet.quiknode.pro/1044c1203ae3589e8faa89cdd0e24138ebf9e069/")
            .json(&request_data)
            .send()
            .await
            .unwrap()
            .json::<serde_json::Value>()
            .await
            .unwrap();
        let ins = fetch["result"]
            .as_array()
            .unwrap()
            .last()
            .unwrap()["signature"]
            .as_str()
            .unwrap()
            .to_string();
        count += 1;
        println!("[{}/10000] address {} txn {}",count,k,ins);
        op.insert(k,ins);
    }
    let path = "./src/results.json";
    let mut output = File::create(path).unwrap();
    let srt = serde_json::Value::from_iter(op.into_iter());
    write!(output,"{}",serde_json::to_string(&srt).unwrap()).unwrap();
    println!("finished!");
}
