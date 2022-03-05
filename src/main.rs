use serde_json;
use std::fs;
use std::collections::HashMap;
use std::fs::File;
use std::io::{Error, Write};
use serde;

#[tokio::main]
async fn main() {
    let path = "./src/results.json";
    let data = fs::read_to_string(path).expect("Unable to read file");
    let input: HashMap<String,String> = serde_json::from_str(&data).unwrap();

    let client = reqwest::Client::new();

    let mut op_creator = HashMap::<String,String>::new();
    let mut op_pda = HashMap::<String,String>::new();
    let mut count = 0;

    for (k,v) in input.into_iter() {
        let request_data = serde_json::json!({
            "jsonrpc": "2.0",
            "id": 1u64,
            "method": "getTransaction",
            "params": [
              v,
              "json"
            ]
        });
        let fetch = client.post("https://long-broken-breeze.solana-mainnet.quiknode.pro/1044c1203ae3589e8faa89cdd0e24138ebf9e069/")
            .json(&request_data)
            .send()
            .await
            .unwrap()
            .json::<serde_json::Value>()
            .await
            .unwrap();
        let accounts = &fetch["result"]["transaction"]["message"]["accountKeys"];

        let accounts = accounts.as_array().unwrap();
        let token_balance_changes = 
            fetch["result"]["meta"]["postTokenBalances"].as_array().unwrap();
        let account_index = token_balance_changes
            .into_iter()
            .filter(|v|v["mint"].as_str().unwrap()==k.as_str())
            .nth(0)
            .unwrap();
        let account_index = account_index["accountIndex"].as_u64().unwrap();
        let asociated = accounts[account_index as usize].as_str().unwrap();

        let request_data = serde_json::json!({
            "jsonrpc": "2.0",
            "id": 1u64,
            "method": "getAccountInfo",
            "params": [
              asociated,
              {
                "encoding": "jsonParsed"
              }
            ]
        });
        let fetch = client.post("https://long-broken-breeze.solana-mainnet.quiknode.pro/1044c1203ae3589e8faa89cdd0e24138ebf9e069/")
            .json(&request_data)
            .send()
            .await
            .unwrap()
            .json::<serde_json::Value>()
            .await
            .unwrap();
        
        println!("asociated: {}",asociated);
        let owner = fetch["result"]["value"]["data"]["parsed"]["info"]["owner"].as_str().unwrap_or("").to_string();

        count += 1;
        println!("[{}/10000] address {} minter {}",count,k,owner);
        op_creator.insert(k.clone(),owner);
        op_pda.insert(k,asociated.to_string());
    }
    let path = "./src/metadata_to_creators.json";
    let mut output = File::create(path).unwrap();
    let srt = serde_json::Value::from_iter(op_creator.into_iter());
    write!(output,"{}",serde_json::to_string(&srt).unwrap()).unwrap();

    let path = "./src/metadata_to_wallets.json";
    let mut output = File::create(path).unwrap();
    let srt = serde_json::Value::from_iter(op_pda.into_iter());
    write!(output,"{}",serde_json::to_string(&srt).unwrap()).unwrap();

    println!("finished!");
}
