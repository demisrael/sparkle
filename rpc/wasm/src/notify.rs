use crate::imports::*;
use kaspa_rpc_macros::declare_typescript_wasm_interface as declare;

#[wasm_bindgen(typescript_custom_section)]
const TS_HEADER: &'static str = r#"

/**
 * RPC notification events.
 * 
 * @category Sparkle RPC
 * 
 * @see {RpcClient.addEventListener}, {RpcClient.removeEventListener}
 */
export enum SparkleRpcEventType {
    Connect = "connect",
    Disconnect = "disconnect",
    TestNotification = "test-notification",
}

/**
 * RPC notification data payload.
 * 
 * @category Sparkle RPC
 */
export type SparkleRpcEventData = ITestNotification;

/**
 * RPC notification event data map.
 * 
 * @category Sparkle RPC
 */
export type SparkleRpcEventMap = {
    "connect" : undefined,
    "disconnect" : undefined,
    "test-notification" : ITestNotification,
}

/**
 * RPC notification event.
 * 
 * @category Sparkle RPC
 */
export type SparkleRpcEvent = {
    [K in keyof SparkleRpcEventMap]: { event: K, data: SparkleRpcEventMap[K] }
}[keyof SparkleRpcEventMap];

/**
 * RPC notification callback type.
 * 
 * This type is used to define the callback function that is called when an RPC notification is received.
 * 
 * @see {@link RpcClient.subscribeVirtualDaaScoreChanged},
 * {@link RpcClient.subscribeTestNotification}, 
 * 
 * @category Sparkle RPC
 */
export type SparkleRpcEventCallback = (event: SparkleRpcEvent) => void;

"#;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = js_sys::Function, typescript_type = "SparkleRpcEventCallback")]
    pub type SparkleRpcEventCallback;

    #[wasm_bindgen(extends = js_sys::Function, typescript_type = "SparkleRpcEventType | string")]
    #[derive(Debug)]
    pub type SparkleRpcEventType;

    #[wasm_bindgen(typescript_type = "SparkleRpcEventType | string | SparkleRpcEventCallback")]
    #[derive(Debug)]
    pub type SparkleRpcEventTypeOrCallback;
}

declare! {
    ITestNotification,
    r#"
    /**
     * Test Notification.
     * 
     * @category Sparkle RPC
     */
    export interface ITestNotification {
        [key: string]: any;
    }
    "#,
}

