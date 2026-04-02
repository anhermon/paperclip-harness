/// Built-in tools registered by default.
///
/// Currently a stub — real implementations added as needed.
use crate::registry::{ToolHandler, ToolOutput};
use crate::schema::ToolSchema;
use async_trait::async_trait;
use serde_json::Value;

/// Echo tool — useful for testing the tool pipeline.
pub struct EchoTool;

#[async_trait]
impl ToolHandler for EchoTool {
    fn schema(&self) -> ToolSchema {
        ToolSchema::simple("echo", "Echo the input message back", &["message"])
    }

    async fn call(&self, input: Value) -> ToolOutput {
        let msg = input["message"].as_str().unwrap_or("(empty)");
        ToolOutput::ok(msg)
    }
}
