use std::thread;

use rand::Rng;
use reqwest::{Client, RequestBuilder, Url};

use crate::{common::read_lines, TOR_PROCESS};


pub struct RequestManager {
    client: Client,
}

impl RequestManager {

    fn connect_tor(&mut self) -> Result<(), reqwest::Error> {
        let proxy = reqwest::Proxy::all("socks5://127.0.0.1:9050")?;
        self.client = reqwest::ClientBuilder::new()
                .proxy(proxy)
                .gzip(true)
                .build()?;
        Ok(())
    }

    pub fn new() -> Result<Self, reqwest::Error> {
        let proxy = reqwest::Proxy::all("socks5://127.0.0.1:9050")?;
        let client = reqwest::ClientBuilder::new()
                .proxy(proxy)
                .gzip(true)
                .build()?;
        Ok(Self {
            client,
        })
    }

    pub async fn post(&self, url: Url) -> Result<String, reqwest::Error> {
        let req = self.client.post(url);
        self.request(req).await
    }

    pub async fn get(&self, url: &str) -> Result<String, reqwest::Error> {
        let req = self.client.get(url);
        self.request(req).await
    }

    pub async fn request(&self, req_builder: RequestBuilder) -> Result<String, reqwest::Error> {
        // To break the patern
        let mut rng = rand::thread_rng();
        let mut users_agents = read_lines("user-agents.csv").unwrap();
        let n: usize = rng.gen_range(0..1000);
        let users_agents = users_agents.nth(n).unwrap().unwrap();

        // Request
        let res = req_builder
            .header("accept", "text/html,application/xhtml+xml,application/xml;q=0.9,*/*;q=0.8,application/signed-exchange;v=b3;q=0.9")
            .header("user-agent", &users_agents)
            .send().await?;

        if res.status() == 503 {
            println!("Erreur: {}", res.status());
            print!("take delay");
            
            let mut tor_process = TOR_PROCESS.lock().await.take();
        
            if let Some(mut p) = tor_process.take(){
                p.kill().unwrap();
                thread::sleep(std::time::Duration::from_secs(10));
                tor_process = None;
            }
            tor_process.replace(std::process::Command::new("tor").spawn().unwrap());
            thread::sleep(std::time::Duration::from_secs(5));
        } else if res.status() != 200 {
            println!("{}", res.status());
        }
        let body = res.text().await?;

        Ok(body)
    }
}