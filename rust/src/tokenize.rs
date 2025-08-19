use std::collections::HashMap;

use anyhow::{Result, anyhow};
use lazy_static::lazy_static;
use serde_json::Value;
use tracing::info;

type MergeMap = HashMap<Vec<u8>, Vec<u8>>;
type TokenMap = HashMap<Vec<u8>, u16>;

#[derive(Clone, Debug)]
pub struct Encoder {
    pub merge_map: MergeMap,
    pub token_map: TokenMap,
}

const ENCODER_STR: &str = include_str!("data/encoder.json");
const VOCAB_STR: &str = include_str!("data/vocab.bpe");

pub fn tiktoken(value: &str) -> Result<()> {
    info!("length = {}", value.len());
    Ok(())
}

fn tiktoken_parse() -> Result<Encoder> {
    let mut merge_map = MergeMap::new();
    for line in VOCAB_STR.split_terminator("\n") {
        let (a, b) = line.split_once(" ").expect("Cannot split line");
        merge_map.insert(a.as_bytes().to_vec(), b.as_bytes().to_vec());
    }

    let encoder_str = serde_json::from_str::<Value>(ENCODER_STR)?;
    let encoder_map = encoder_str.as_object().ok_or(anyhow!("encoder.json is not an json object"))?;
    let mut token_map = TokenMap::new();
    for (k, v) in encoder_map {
        let k = k.as_bytes().to_vec();
        let v = v.as_u64().unwrap() as u16;
        token_map.insert(k, v);
    }

    Ok(Encoder {
        merge_map,
        token_map,
    })
}

lazy_static! {
    pub static ref TIKTOKEN_TABLES: Encoder = tiktoken_parse().unwrap();
}
