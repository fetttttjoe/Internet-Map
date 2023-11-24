// src/lib.rs
use axum::Json;
use reqwest;
use scraper::{Html, Selector};
use std::net::TcpStream;
fn is_port_open(ip: &str, port: u16) -> bool {
  match TcpStream::connect((ip, port)) {
    Ok(_) => true,
    Err(_) => false,
  }
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
  let mut acor_tags = Vec::new();
  for element in document.select(&selector) {
    if let Some(href) = element.value().attr("href") {
      acor_tags.push(href.to_string());
    }
  }
  return acor_tags;
}
fn filter_relative_paths(anchor_tags: Vec<String>) -> Vec<String> {
  println!("Anchor tags: {:?}", anchor_tags);
  let filtered_tags: Vec<_> = anchor_tags.clone()
    .into_iter()
    .filter(|href| href.starts_with('/'))
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
    },
    None => return Json(vec!["No website found".to_string()]),
      
  }
}
