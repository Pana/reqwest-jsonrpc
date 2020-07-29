use jsonrpc_core::{Params, Value, Error, ErrorCode};
use crate::http_provider::{HttpProvider, CallRequest};
use client::rpc::types::*;
use serde_json::value::from_value;

pub struct Conflux {
    provider: HttpProvider,
}

impl Conflux {
    pub fn new(url: String) -> Self {
        let provider = HttpProvider::new(url);
        Conflux{
            provider
        }
    }

    #[allow(dead_code)]
    pub async fn get_status(&self) -> Result<Status, Error> {
        let req = CallRequest::new("cfx_getStatus".to_string(), Params::None);
        let value = self.provider.call(req).await?;
        // TODO 下面的判断，应该有更好的实现方法
        if value["result"] != Value::Null {
            let status = from_value(value["result"].clone())
                .map_err(map_parse_error)?;
            Ok(status)
        } else {
            Err(internal_err())
        }
    }

    #[allow(dead_code)]
    pub async fn get_epoch_number(&self, tag: Option<String>) -> Result<u64, Error> {
        let mut vv = vec![];
        if let Some(t) = tag {
            vv.push(Value::String(t));
        }
        let params = if vv.len() == 0 {
            Params::None
        } else {
            Params::Array(vv)
        };
        let req = CallRequest::new("cfx_epochNumber".to_string(), params);
        let value = self.provider.call(req).await?;
        println!("return value {}", value);
        if value["result"] != Value::Null {
            let number = from_value(value["result"].clone())
                .map_err(map_parse_error)?;
            Ok(number)
        } else {
            Err(internal_err())
        }
    }

    #[allow(dead_code)]
    pub async fn get_next_nonce(&self, address: String, epoch: Option<String>) -> Result<u64, Error> {
        let mut vv = vec![Value::String(address)];
        if let Some(num) = epoch {
            vv.push(Value::String(num));
        }
        let params = Params::Array(vv);
        let req = CallRequest::new("cfx_getNextNonce".to_string(), params);
        let value = self.provider.call(req).await?;
        if value["result"] != Value::Null {
            let nonce = from_value(value["result"].clone())
                .map_err(map_parse_error)?;
            Ok(nonce)
        } else {
            Err(internal_err())
        }
    }

    #[allow(dead_code)]
    pub async fn estimate_gas_and_collateral(&self, tx: Value, tag: Option<String>) -> Result<EstimateGasAndCollateralResponse, Error> {
        let mut vv = vec![tx];
        if let Some(t) = tag {
            vv.push(Value::String(t));
        }
        let req = CallRequest::new("cfx_estimateGasAndCollateral".to_string(), Params::Array(vv));
        let value = self.provider.call(req).await?;
        if value["result"] != Value::Null {
            let result = from_value(value["result"].clone())
                .map_err(map_parse_error)?;
            Ok(result)
        } else {
            Err(internal_err())
        }
    }

    #[allow(dead_code)]
    pub async fn send_raw_transaction(&self, raw: String) -> Result<String, Error> {
        let params = Params::Array(vec![Value::String(raw)]);
        let req = CallRequest::new("cfx_sendRawTransaction".to_string(), params);
        let value = self.provider.call(req).await?;
        if value["result"] != Value::Null {
            let txhash = from_value(value["result"].clone())
                .map_err(map_parse_error)?;
            Ok(txhash)
        } else {
            Err(internal_err())
        }
    }

    // TODO add more methods
}

fn map_parse_error(_err: serde_json::Error) -> Error {
    Error::new(ErrorCode::ParseError)
}

fn internal_err() -> Error {
    Error::new(ErrorCode::InternalError)
}