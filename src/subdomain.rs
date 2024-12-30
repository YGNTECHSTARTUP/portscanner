use std::collections::HashSet;

use crate::model::Domain;
use crate::model::Scaentry;
use anyhow::Error;
use reqwest::blocking::Client;
pub fn fetch(http_client: &Client, target: &str) -> Result<Vec<Domain>, Error> {
    let entries: Vec<Scaentry> = http_client
        .get(&format!("https://crt.sh/?q=%25.{}&output=json", target))
        .send()?
        .json()?;
    let mut subdomains: HashSet<String> = entries
        .into_iter()
        .flat_map(|entry| {
            entry
                .name_value
                .split('\n')
                .map(|subdomain| subdomain.trim().to_string())
                .collect::<Vec<String>>()
        })
        .filter(|subdomain: &String| subdomain != target)
        .filter(|subdomain: &String| !subdomain.contains('*'))
        .collect();
    subdomains.insert(target.to_string());
    let sub_domain: Vec<Domain> = subdomains
        .into_iter()
        .map(|ele| Domain {
            dom_name: ele,
            ports: Vec::new(),
        })
        .collect();
    Ok(sub_domain)
}
