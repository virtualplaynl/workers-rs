use wasm_bindgen::prelude::*;

use crate::types::{
    Container, DurableObjectId, DurableObjectStorage, WebSocketRequestResponsePair,
};

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends=js_sys::Object)]
    pub type DurableObjectState;

    #[wasm_bindgen(method, catch, getter)]
    pub fn id(this: &DurableObjectState) -> Result<DurableObjectId, JsValue>;

    #[wasm_bindgen(method, catch, getter)]
    pub fn storage(this: &DurableObjectState) -> Result<DurableObjectStorage, JsValue>;

    #[wasm_bindgen(method, getter)]
    pub fn container(this: &DurableObjectState) -> Option<Container>;

    #[wasm_bindgen(method, catch, js_name=waitUntil)]
    pub fn wait_until(this: &DurableObjectState, promise: &js_sys::Promise) -> Result<(), JsValue>;

    #[wasm_bindgen(method, catch, js_name=blockConcurrencyWhile)]
    pub fn block_concurrency_while(
        this: &DurableObjectState,
        callback: &js_sys::Function,
    ) -> Result<js_sys::Promise, JsValue>;

    #[wasm_bindgen(method, catch, js_name=acceptWebSocket)]
    pub fn accept_websocket(
        this: &DurableObjectState,
        ws: &web_sys::WebSocket,
    ) -> Result<(), JsValue>;

    #[wasm_bindgen(method, catch, js_name=acceptWebSocket)]
    pub fn accept_websocket_with_tags(
        this: &DurableObjectState,
        ws: &web_sys::WebSocket,
        tags: Vec<JsValue>,
    ) -> Result<(), JsValue>;

    #[wasm_bindgen(method, catch, js_name=getWebSockets)]
    pub fn get_websockets(this: &DurableObjectState) -> Result<Vec<web_sys::WebSocket>, JsValue>;

    #[wasm_bindgen(method, catch, js_name=getWebSockets)]
    pub fn get_websockets_with_tag(
        this: &DurableObjectState,
        tag: &str,
    ) -> Result<Vec<web_sys::WebSocket>, JsValue>;

    #[wasm_bindgen(method, catch, js_name=getTags)]
    pub fn get_tags(
        this: &DurableObjectState,
        ws: &web_sys::WebSocket,
    ) -> Result<Vec<String>, JsValue>;

    #[wasm_bindgen(method, catch, js_name=setWebSocketAutoResponse)]
    pub fn set_websocket_auto_response(
        this: &DurableObjectState,
        pair: &WebSocketRequestResponsePair,
    ) -> Result<(), JsValue>;

    #[wasm_bindgen(method, catch, js_name=getWebSocketAutoResponse)]
    pub fn get_websocket_auto_response(
        this: &DurableObjectState,
    ) -> Result<Option<WebSocketRequestResponsePair>, JsValue>;

    #[wasm_bindgen(method, js_name=getWebSocketAutoResponseTimestamp)]
    pub fn get_web_socket_auto_response_timestamp(
        this: &DurableObjectState,
        ws: &web_sys::WebSocket,
    ) -> Option<js_sys::Date>;

    #[wasm_bindgen(method, catch, js_name=getWebSocketAutoResponseTimestamp)]
    pub fn try_get_web_socket_auto_response_timestamp(
        this: &DurableObjectState,
        ws: &web_sys::WebSocket,
    ) -> Result<Option<js_sys::Date>, JsValue>;

    #[wasm_bindgen(method, js_name=setHibernatableWebSocketEventTimeout)]
    pub fn set_hibernatable_web_socket_event_timeout(this: &DurableObjectState);

    #[wasm_bindgen(method, catch, js_name=setHibernatableWebSocketEventTimeout)]
    pub fn try_set_hibernatable_web_socket_event_timeout(
        this: &DurableObjectState,
    ) -> Result<(), JsValue>;

    #[wasm_bindgen(method, js_name=setHibernatableWebSocketEventTimeout)]
    pub fn set_hibernatable_web_socket_event_timeout_with_timeout_ms(
        this: &DurableObjectState,
        timeout_ms: f64,
    );

    #[wasm_bindgen(method, catch, js_name=setHibernatableWebSocketEventTimeout)]
    pub fn try_set_hibernatable_web_socket_event_timeout_with_timeout_ms(
        this: &DurableObjectState,
        timeout_ms: f64,
    ) -> Result<(), JsValue>;

    #[wasm_bindgen(method, js_name=getHibernatableWebSocketEventTimeout)]
    pub fn get_hibernatable_web_socket_event_timeout(this: &DurableObjectState) -> Option<f64>;

    #[wasm_bindgen(method, catch, js_name=getHibernatableWebSocketEventTimeout)]
    pub fn try_get_hibernatable_web_socket_event_timeout(
        this: &DurableObjectState,
    ) -> Result<Option<f64>, JsValue>;
}

impl core::fmt::Debug for DurableObjectState {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("DurableObjectState")
            .field("id", &self.id())
            .finish()
    }
}
