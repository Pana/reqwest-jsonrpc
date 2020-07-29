use serde::{Serialize, Deserialize};
use jsonrpc_core::{Params, Value, Error, ErrorCode};
use reqwest::{Client};
use crate::util::time_stamp;

#[derive(Serialize, Deserialize, Debug)]
pub struct CallRequest {
    jsonrpc: &'static str,
    id: String,
    method: String,
    params: Params
}

impl CallRequest {
    pub fn new(method: String, params: Params) -> Self {
        CallRequest{
            jsonrpc: "2.0",
            id: format!("{}", time_stamp()),
            method,
            params
        }
    }
}

pub struct HttpProvider {
    url: String,
    client: Client,
}

impl HttpProvider {
    pub fn new(url: String) -> Self {
        let client = Client::new();
        Self{
            url,
            client,
        }
    }

    pub async fn call(&self, req: CallRequest) -> Result<Value, Error> {
        println!("call request {:?}", req.params);
        let resp = self.client.post(self.url.as_str())
            .json(&req)
            .send()
            .await
            .map_err(|_e| {Error::new(ErrorCode::InvalidParams)})?
            .text()
            .await
            .map_err(|_e| {Error::new(ErrorCode::ParseError)})?;
        serde_json::from_str(resp.as_str())
            .map_err(|_e| {Error::new(ErrorCode::ParseError)})
    }
}