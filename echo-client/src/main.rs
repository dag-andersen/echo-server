use rand::Rng;
use std::time::Duration;
use structopt::StructOpt;
use tokio::time::sleep;

#[derive(Debug, StructOpt)]
#[structopt(name = "echo-client")]
struct Opt {
    /// server address
    #[structopt(long, env, default_value = "http://localhost:3030/hello")]
    address: String,

    #[structopt(long, env, default_value = "5")]
    call_frequency: u64,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let opt = Opt::from_args();
    let client_name = format!("client-{}", rand::thread_rng().gen::<u8>());
    let address = format!("{}/{}", opt.address, client_name);
    println!("Connecting to {}.", address);
    loop {
        match call_address(&address).await {
            Err(x) => println!("Error: {}", x),
            Ok(s) => println!("### I, {}, called '{}' and received: {:?}", &client_name, &address, s),
        }
        sleep(Duration::from_secs(opt.call_frequency)).await;
    }
}

async fn call_address(add: &str) -> Result<String, Box<dyn std::error::Error>> {
    let body = reqwest::get(add).await?.text().await?;
    Ok(body)
}
