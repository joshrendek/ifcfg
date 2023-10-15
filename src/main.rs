mod bgp;

#[macro_use]
extern crate rocket;

use crate::bgp::{ASLookup, Data};
use dns_lookup::lookup_host;
use futures;
use reqwest::Error;
use rocket::serde::json::Json;
use rocket::tokio;
use serde::Deserialize;
use serde::Serialize;
use std::net::Ipv4Addr;
use std::net::SocketAddr;
use std::process::Command;
use std::time;
use std::time::Duration;

#[derive(Serialize)]
struct RBL {
    list: String,
    hostname: String,
    status: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct IPLookup {
    data: IpData,
}

#[derive(Serialize, Deserialize, Debug)]
struct IpData {
    prefixes: Vec<Prefix>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Prefix {
    asn: ASN,
}

#[derive(Serialize, Deserialize, Debug)]
struct ASN {
    asn: u32,
}

#[get("/")]
fn index(remote_addr: SocketAddr) -> String {
    format!("Remote Address: {:?}", remote_addr.ip())
}

async fn lookup_ip(ip: String) -> Result<(IPLookup), Error> {
    let request_url = format!("https://api.bgpview.io/ip/{ip}", ip = ip);
    println!("{}", request_url);
    let response = reqwest::get(&request_url).await?;

    let ip_data: IPLookup = response.json().await?;
    println!("{:?}", ip_data);
    Ok(ip_data)
}

async fn lookup_asn(asn: u32) -> Result<(ASLookup), Error> {
    let request_url = format!("https://api.bgpview.io/asn/{as}",
                            as = asn);
    println!("{}", request_url);
    let response = reqwest::get(&request_url).await?;

    let res: ASLookup = response.json().await?;
    println!("{:?}", res);
    Ok(res)
}

#[get("/as/<num>")]
async fn lookup_as(num: u32) -> Json<Data> {
    let res = lookup_asn(num).await;
    Json(res.unwrap().data)
}

#[get("/ping/<ip>/<count>")]
fn ping(ip: Ipv4Addr, count: u8) -> String {
    // execute the ping command command line
    let mut echo_hello = Command::new("ping");
    if count > 20 {
        return "Count must be less than 20".to_string();
    }
    echo_hello.arg("-c").arg(count.to_string());

    let mut echo_hello = echo_hello
        .arg(ip.to_string())
        .output()
        .expect("failed to execute process");
    // convert the output to a string
    let output = String::from_utf8_lossy(&echo_hello.stdout);
    output.to_string()
}

#[get("/rbl/<ip>/<list>")]
fn rbl(ip: Ipv4Addr, list: String) -> Json<Vec<RBL>> {
    let mut rbls: Vec<RBL> = Vec::new();
    // split list into a vector of strings on the + delimeter
    let list: Vec<&str> = list.split("+").collect();
    // reverse an ip address
    let octets = ip.octets();
    // reverse the octets
    let reversed_octets = [
        octets[3].clone(),
        octets[2].clone(),
        octets[1].clone(),
        octets[0].clone(),
    ];
    // convert reversed octets to string
    let rev_ip = reversed_octets
        .iter()
        .map(|o| o.to_string())
        .collect::<Vec<String>>()
        .join(".");

    // do a dns lookup on revIP
    for list in &list {
        // concat revIP and list
        if !is_valid_blocklist(list.to_string()) {
            println!("{} is not a valid blocklist", list);
            continue;
        }
        let reversed_host = rev_ip.clone() + "." + &list;
        let ips = lookup_host(&reversed_host);
        // check if ips is an error
        let mut is_rbl = false;
        if !ips.is_err() {
            is_rbl = true;
        }
        rbls.append(&mut vec![RBL {
            list: list.to_string(),
            hostname: reversed_host,
            status: is_rbl.to_string(),
        }]);
    }

    Json(rbls)
}

fn is_valid_blocklist(list: String) -> bool {
    // read in blacklist.txt
    let mut blocklists: Vec<String> = Vec::new();
    let contents = std::fs::read_to_string("src/blacklist.txt")
        .expect("Something went wrong reading the file");
    for line in contents.lines() {
        blocklists.push(line.to_string());
    }
    // check if list is in blocklists
    if blocklists.contains(&list) {
        return true;
    }

    false
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, rbl, ping, lookup_as])
}
