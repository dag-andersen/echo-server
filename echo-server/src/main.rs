use rand::Rng;
use std::net::SocketAddr;
use structopt::StructOpt;
use warp::Filter;

#[derive(Debug, StructOpt)]
#[structopt(name = "echo-client")]
struct Opt {
    /// Version name. This will be returned to the client.
    #[structopt(long, env, default_value = "unknown")]
    version: String,

    /// minimum delay between responses. The actual delay will be between (delay_min) and (delay_min + delay_random).
    #[structopt(long, env, default_value = "0")]
    delay_min: u64,

    /// random delay between responses. The actual delay will be between (delay_min) and (delay_min + delay_random).
    #[structopt(long, env, default_value = "0")]
    delay_random: u64,
}

#[tokio::main]
async fn main() {
    let opt = Opt::from_args();
    let delay_min = opt.delay_min;
    let delay_random = opt.delay_random;
    let version = opt.version.as_str().to_string();
    let address = get_addresses(3030);
    // generated random name:
    let server_name = format!("server-{}", rand::thread_rng().gen::<u8>());
    println!("server-name: {}, with version: {}", server_name, version);
    println!("now listing on {}", address);
    // GET /hello/warp => 200 OK with body "Hello, warp!"
    let hello = warp::path!("hello" / String).map(move |name| {
        println!("### I, {}, was called by a client with name: {}", &server_name, name);
        // random interval between 0 and 500 miliseconds
        let interval = if delay_random == 0 {
            delay_min
        } else {
            delay_min + rand::thread_rng().gen_range(0..delay_random)
        };

        if interval > 0 {
            std::thread::sleep(std::time::Duration::from_millis(interval));
        }

        format!(
            "Hi, {}!, from server-name: {}, Version: {}",
            name, server_name, version
        )
    });

    warp::serve(hello).run(address).await;
}

fn get_addresses(port: u32) -> SocketAddr {
    format!("[::]:{}", port).parse::<SocketAddr>().unwrap()
}
