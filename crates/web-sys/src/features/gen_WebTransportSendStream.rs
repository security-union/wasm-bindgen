#![allow(unused_imports)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = WritableStream , extends = :: js_sys :: Object , js_name = WebTransportSendStream , typescript_type = "WebTransportSendStream")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `WebTransportSendStream` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebTransportSendStream)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebTransportSendStream`*"]
    pub type WebTransportSendStream;
    # [wasm_bindgen (method , structural , js_class = "WebTransportSendStream" , js_name = getStats)]
    #[doc = "The `getStats()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebTransportSendStream/getStats)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebTransportSendStream`*"]
    pub fn get_stats(this: &WebTransportSendStream) -> ::js_sys::Promise;
}
