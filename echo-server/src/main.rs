use std::net::SocketAddr;
use rand::Rng;
use structopt::StructOpt;
use warp::Filter;

#[derive(Debug, StructOpt)]
#[structopt(name = "echo-client")]
struct Opt {
    #[structopt(long, env, default_value = "unknown")]
    version: String,
}

#[tokio::main]
async fn main() {
    let opt = Opt::from_args();
    let version = opt.version.as_str().to_string();
    let address = get_addresses(3030);
    // generated random name:
    let server_name = format!("server-{}", rand::thread_rng().gen::<u8>());
    println!("server-name: {}, with version: {}", server_name, version);
    println!("now listing on {}", address);
    // GET /hello/warp => 200 OK with body "Hello, warp!"
    let hello = warp::path!("hello" / String).map(move |name| {
        println!("client called with name: {}", name);
        format!("Hi, {}!, From server-name: {}, Version: {}", name, server_name, version)}
    );

    warp::serve(hello).run(address).await;
}

fn get_addresses(port: u32) -> SocketAddr {
    format!("[::]:{}", port).parse::<SocketAddr>().unwrap()
}
