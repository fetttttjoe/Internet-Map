// src/lib.rs
use axum::Json;
use rand::seq::SliceRandom;
use reqwest;
use scraper::{Html, Selector};
use std::net::IpAddr;
use std::net::Ipv4Addr;
use std::net::TcpStream;
use std::str::FromStr;
use std::time::Duration;
use std::vec;
use trust_dns_resolver::config::ResolverConfig;
use trust_dns_resolver::config::ResolverOpts;
use trust_dns_resolver::error::ResolveError;
use trust_dns_resolver::Resolver;

fn is_port_open(ip: &str, port: u16) -> bool {
    match TcpStream::connect((ip, port)) {
        Ok(_) => true,
        Err(_) => false,
    }
}

pub async fn scan_ips(start_ip: &str, end_ip: &str, port: u16) -> Vec<Json<Vec<String>>> {
    let start = Ipv4Addr::from_str(start_ip).expect(&format!("Invalid IP address {}", start_ip));
    let end = Ipv4Addr::from_str(end_ip).expect(&format!("Invalid IP address {}", end_ip));
    let mut ips: Vec<Ipv4Addr> = (u32::from(start)..=u32::from(end))
        .map(|ip| Ipv4Addr::from(ip))
        .collect();
    // Randomize the order of the IPs
    ips.shuffle(&mut rand::thread_rng());
    let mut result = vec![];
    for ip in ips {
        let ip_str = Ipv4Addr::from(ip).to_string();
        if is_port_open(&ip_str, port) {
            let name = ip_lookup(ip.into()).await;
            match name {
                Ok(names) => {
                    // Process the names here
                    println!("Port {} is open on IP: {}", port, ip_str);
                    println!("Names: {:?}", names);
                }
                Err(err) => {
                    eprintln!("Error: {}", err);
                }
            }
            tokio::time::sleep(Duration::from_secs(2)).await;
            result.push(handle_request(&ip_str).await);
        }
    }
    return result;
}
async fn download_index_html(ip: &str) -> Result<String, reqwest::Error> {
    let url = format!("http://{}/index.html", ip);
    reqwest::get(url).await?.text().await
}

async fn fetch_website(url: &str) -> Result<String, reqwest::Error> {
    let body = reqwest::get(url).await?.text().await?;
    Ok(body)
}

fn find_anchor_tags(html: &str) -> Vec<String> {
    let document = Html::parse_document(html);
    let selector = Selector::parse("a").expect("Failed to parse anchor tag selector");
    println!("Selector: {:#?}", document.select(&selector));
    let mut acor_tags = Vec::new();
    for element in document.select(&selector) {
        if let Some(href) = element.value().attr("href") {
            acor_tags.push(href.to_string());
        }
    }
    return acor_tags;
}
async fn ip_lookup(ip: IpAddr) -> Result<Vec<String>, ResolveError> {
    let resolver = Resolver::new(ResolverConfig::default(), ResolverOpts::default())?;
    let response = tokio::task::spawn_blocking(move || resolver.reverse_lookup(ip)).await;
    let mut result = vec![];
    println!("resonse  {:#?}",response);
    match response {
        Ok(names) => {
            for name in names.iter() {
              let dns_name = format!("{:?}", name);
              result.push(dns_name.clone());
              println!("Domain Name: {:?}", name);
            }
        }
        Err(_) => println!("Error"),
    }

    Ok(result)
}

fn filter_relative_paths(anchor_tags: Vec<String>) -> Vec<String> {
    // println!("Anchor tags: {:?}", anchor_tags);
    let filtered_tags: Vec<_> = anchor_tags
        .clone()
        .into_iter()
        .filter(|href| href.starts_with('/') || href.starts_with('.'))
        .collect();
    if filtered_tags.len() > 0 {
        return filtered_tags;
    } else {
        let mut error_message: Vec<String> = Vec::new();
        let message = format!("No relative paths found in {:?}", anchor_tags);
        error_message.push(message);
        error_message.extend(anchor_tags.clone());
        return error_message;
    }
}
async fn handle_ip(ip: &str) -> Option<String> {
    if is_port_open(ip, 80) {
        match download_index_html(ip).await {
            Ok(html) => return Some(html),
            Err(e) => return None,
        }
    } else {
        return None;
    }
}
pub async fn handle_request(url: &str) -> Json<Vec<String>> {
    let html = handle_ip(url).await;
    match html {
        Some(html) => {
            let anchor_tags = find_anchor_tags(&html);
            let relative_paths = filter_relative_paths(anchor_tags);
            return Json(relative_paths);
        }
        None => return Json(vec!["No website found".to_string()]),
    }
}
