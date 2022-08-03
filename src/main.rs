use rayon::prelude::*;
use std::io::{self, BufRead};
use std::{thread, time};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = io::stdin();
    let mut input = io::BufReader::new(input.lock());
    let mut url_list: Vec<String> = Vec::new();
    loop {
        let mut line = String::new();
        let bytes = input.read_line(&mut line)?;
        if bytes == 0 {
            break;
        }
        url_list.push(line.trim().to_string());
    }
    println!("url,status,content-type,content-length,last-modified"); // CSV形式の出力のヘッダー
    let duration_millis = time::Duration::from_millis(100);
    url_list.par_iter()
        .for_each(|url| {
            if let Ok(resp) = reqwest::blocking::get(url) {
                let status = resp.status().to_string();
                let content_type = if let Some(content_type) = resp.headers().get("content-type") {
                    content_type.to_str().unwrap()
                } else {
                    ""
                };
                let content_length = if let Some(content_length) = resp.content_length() {
                    content_length
                } else {
                    0
                };
                let last_modified = if let Some(last_modified) = resp.headers().get("last-modified") {
                    last_modified.to_str().unwrap()
                } else {
                    ""
                };
                println!("{},{},{},{},{}", url, status, content_type, content_length, last_modified);
            }
            thread::sleep(duration_millis); // sleepを挟んでリクエストを緩くする
        });
    Ok(())
}
