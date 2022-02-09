use jieba_rs::Jieba;
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use std::sync::Mutex;
use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[derive(Serialize, Deserialize)]
pub struct RetToken<'a> {
    /// Word of the token
    pub word: &'a str,
    /// Unicode start position of the token
    pub start: usize,
    /// Unicode end position of the token
    pub end: usize,
}
#[derive(Serialize, Deserialize)]
pub struct RetTag<'a> {
    /// Word of the token
    pub word: &'a str,
    pub tag: &'a str,

}
lazy_static! {
    pub static ref JIEBA: Mutex<Jieba> = Mutex::new(Jieba::new());
}
#[wasm_bindgen]
pub fn tag(sentence:&str,hmm:bool)-> Result<Vec<JsValue>, JsValue>{
    let jieba = JIEBA.lock().unwrap();
    let tags = jieba.tag(sentence,hmm);
    let js_tags = tags
        .into_iter()
        .map(|tag| {
            let t = RetTag {
                word: tag.word,
                tag: tag.tag
            };
            serde_wasm_bindgen::to_value(&t).unwrap()
        })
        .collect();
        Ok(js_tags)
}