use anyhow;
use error::Error;
mod common_ports;
mod error;
mod model;
mod ports;
use crate::model::Domain;
use rayon::{prelude::*, ThreadPoolBuilder};
use reqwest::blocking::ClientBuilder;
use reqwest::redirect::Policy;
mod subdomain;
use std::{env::args, time::Duration};
fn main() -> Result<(), anyhow::Error> {
    let args: Vec<String> = args().collect();
    if args.len() != 2 {
        return Err(Error::CliError.into());
    }

    let target = args[1].as_str().trim();
    let http_timeout = Duration::from_secs(10);
    let http_client = ClientBuilder::new()
        .redirect(Policy::limited(4))
        .timeout(http_timeout)
        .build()?;
    let pool = ThreadPoolBuilder::new().num_threads(256).build().unwrap();
    pool.install(|| {
        let res: Vec<Domain> = subdomain::fetch(&http_client, target)
            .expect("Failed to Fetch the Subdomains")
            .into_par_iter()
            .map(ports::scanports)
            .collect();
        for subdomain in res {
            println!("\n🌐 **Domain Name:** {:?}", &subdomain.dom_name);

            println!("🚪 **Ports:**");
            println!("-----------------------------------");
            println!("| 🆔 Port Name       | 🔒 Is Open   |");
            println!("-----------------------------------");

            for port in subdomain.ports {
                println!(
                    "| {:<16} | {:<12} |",
                    port.port_name,
                    if port.is_open { "✅ Yes" } else { "❌ No" }
                );
            }

            println!("-----------------------------------");
        }
        println!("")
    });
    Ok(())
}
