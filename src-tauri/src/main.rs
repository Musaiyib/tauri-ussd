// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use autoincrement::{prelude::*, AsyncIncrement};
use openssl::{error::ErrorStack, pkey::PKey, rsa::Padding};
use reqwest::header;
use tokio::time::{sleep, Duration};
use uuid::Uuid;

#[derive(AsyncIncremental, PartialEq, Eq, Debug, Clone)]
struct Int(u64);
fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![send])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

fn rsa_encrypt_with_pem_public_key(
    message: &[u8],
    public_key_pem: &[u8],
) -> Result<String, ErrorStack> {
    // Parse the PEM public key
    let public_key_pkey = PKey::public_key_from_pem(public_key_pem)?;

    // Extract the RSA key from the PKey
    let rsa_public_key = public_key_pkey.rsa()?;

    // Determine the size of the RSA key
    let key_size = rsa_public_key.size() as usize;

    // Ensure the message is not longer than the key size
    if message.len() > key_size - 11 {
        return Err(ErrorStack::get());
    }

    // Allocate memory for the output buffer
    let mut encrypted_data = vec![0; key_size];

    // Perform RSA encryption
    let encrypted_size =
        rsa_public_key.public_encrypt(message, &mut encrypted_data, Padding::PKCS1)?;

    // Resize the output buffer to the actual encrypted size
    encrypted_data.truncate(encrypted_size);

    return Ok(base64::encode_config(encrypted_data, base64::STANDARD));
}
#[derive(Debug, Copy, Clone)]
struct LoginResponse {
    uuid: Uuid,
    // hex: String,
}

async fn login() -> LoginResponse {
    let public_key_pem = b"-----BEGIN PUBLIC KEY-----\n\
                         MFwwDQYJKoZIhvcNAQEBBQADSwAwSAJBALk84jQ1Uqbwo10Ewm6Kkf7KPlAemzO/\n\
                         mE+563HmUk0H6C1ZUb/D4hZq4NNgdFJCpBgZgCIA1v6WsQoJQ6D5VIsCAwEAAQ==\n\
                         -----END PUBLIC KEY-----";
    let client: reqwest::Client = reqwest::Client::new();
    let uuid: Uuid = Uuid::new_v4();

    client
  .post("http://192.168.0.1/action/login")
  .body(format!("password={}",  urlencoding::encode(&rsa_encrypt_with_pem_public_key(b"admin", public_key_pem).unwrap())))
  .header("Connection", "close")
  .header("Content-Type", "application/x-www-form-urlencoded")
  .header("Referer", "http://192.168.0.1/ussd.html")
  .header("X-Requested-With", "XMLHttpRequest")
  .header("Upgrade-Insecure-Requests", 1)
  .header(header::COOKIE, format!("-goahead-session-=::webs.session::11dcf2af9e79b3e8b93e8b210518bdc9; dataUsageSignal=0; token={}; ussd=; url=index.html#ussd", "d632dd48-0923-41aa-b287-5990363ca1c5"))
  .header("User-Agent", "RUST:: Reqwest")
  .send().await.expect("msg");

    return LoginResponse { uuid };
}

#[tauri::command]
async fn send(number: u64, limit: u64) -> Result<(), &'static str> {
    if format!("{}", number).len() > 10 || format!("{}", number).len() < 10 {
        return Err("Invalid number");
    }
    let generator: AsyncIncrement<Int> = Int(number).init_from();

    let var = for _i in 0..limit {
        let number_gen: String = format!("{:?}", generator.pull())
            .replace("Int(", "0")
            .replace(")", "");
        let ussd: String = format!("*671*2*1*4*{}*1*1111#", number_gen);
        // let login_response: LoginResponse = login().await;
        ussd_send(encode_unicode(&ussd)).await.expect("msg");
        ussd_recieved().await.expect("eee");
        // Delay in milliseconds
        let delay_milliseconds = 1000;

        // Convert milliseconds to nanoseconds
        let delay_duration: Duration =
            Duration::from_nanos((delay_milliseconds * 1_000_000) as u64);
        sleep(delay_duration).await;
    };

    return Ok(var);
}

async fn ussd_recieved() -> Result<(), Box<dyn std::error::Error>> {
    let client: reqwest::Client = reqwest::Client::new();
    let response: reqwest::Response= client
  .get(format!("http://192.168.0.1/data.html?method=obj_get&param=%5B%22zteVolteUssdRecv%22%5D&_csrf_token={}", "d632dd48-0923-41aa-b287-5990363ca1c5"))
  .header("Connection", "close")
  .header("Content-Type", "application/json")
  .header("Referer", "http://192.168.0.1/ussd.html")
  .header("X-Requested-With", "XMLHttpRequest")
  .header(header::COOKIE, format!("-goahead-session-=::webs.session::11dcf2af9e79b3e8b93e8b210518bdc9; dataUsageSignal=0; token={}; ussd=; url=index.html#ussd", "d632dd48-0923-41aa-b287-5990363ca1c5"))
  .header("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/117.0.5938.132 Safari/537.36")
  .send().await.expect("msg");
    println!("Response {:?}", response.text().await?);
    Ok(())
}

async fn ussd_send(ussd: String) -> Result<(), Box<dyn std::error::Error>> {
    let client: reqwest::Client = reqwest::Client::new();
    let url = format!("http://192.168.0.1/data.html?method=obj_set&param=%7B%22zteVolteUssdSend%22%3A%22{}%22%7D&_csrf_token={}", ussd, "d632dd48-0923-41aa-b287-5990363ca1c5");
    println!("{}", url);
    let response = client
  .get(url)
  // .header("Connection", "close")
  .header("Content-Type", "application/json")
  .header("Referer", "http://192.168.0.1/ussd.html")
  .header("X-Requested-With", "XMLHttpRequest")
  .header(header::COOKIE, format!("-goahead-session-=::webs.session::11dcf2af9e79b3e8b93e8b210518bdc9; dataUsageSignal=0; token={}; ussd=; url=index.html#ussd", "d632dd48-0923-41aa-b287-5990363ca1c5"))
  .header("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/117.0.5938.132 Safari/537.36")
  .send().await.expect("msg");
    println!("SEND USSD{:?}", response.text().await);

    Ok(())
}

fn encode_unicode(input: &str) -> String {
    let mut result = Vec::with_capacity(input.len() * 4);

    for ch in input.chars() {
        let hex_string = format!("{:04X}", ch as u32);
        result.extend_from_slice(hex_string.as_bytes());
    }

    String::from_utf8(result).unwrap().to_uppercase()
}
