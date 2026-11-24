use crate::llm::text::component::*;
use serde::Deserialize;

#[derive(Deserialize)]
#[serde(tag = "type", rename_all = "snake_case", deny_unknown_fields)]
pub(super) enum InputItem {
    Message {
        role: InputRole,
        content: String,
    },
    FunctionCall {
        call_id: String,
        name: String,
        arguments: String,
    },
    FunctionCallOutput {
        call_id: String,
        output: String,
    },
}

#[derive(Deserialize)]
#[serde(rename_all = "snake_case")]
pub(super) enum InputRole {
    System,
    Developer,
    User,
    Assistant,
}

impl From<InputRole> for Role {
    fn from(value: InputRole) -> Self {
        match value {
            InputRole::System => Self::System,
            InputRole::Developer => Self::Developer,
            InputRole::User => Self::User,
            InputRole::Assistant => Self::Assistant,
        }
    }
}

impl From<InputItem> for Message {
    fn from(value: InputItem) -> Self {
        let mut message = Self {
            role: Role::User,
            content: None,
            tool_calls: None,
            tool_call_id: None,
            reasoning: None,
            reasoning_content: None,
            reasoning_details: None,
        };
        match value {
            InputItem::Message { role, content } => {
                message.role = role.into();
                message.content = Some(Content::Text(content));
            }
            InputItem::FunctionCall {
                call_id,
                name,
                arguments,
            } => {
                message.role = Role::Assistant;
                message.tool_calls = Some(vec![ToolCall {
                    id: call_id,
                    kind: ToolType::Function,
                    function: FunctionCall { name, arguments },
                }]);
            }
            InputItem::FunctionCallOutput { call_id, output } => {
                message.role = Role::Tool;
                message.tool_call_id = Some(call_id);
                message.content = Some(Content::Text(output));
            }
        }
        message
    }
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub(super) struct Tool {
    #[serde(rename = "type")]
    kind: FunctionType,
    #[serde(deserialize_with = "compallm_serde_rules::deserialize::nonempty")]
    name: String,
    description: Option<String>,
    #[serde(
        default,
        deserialize_with = "compallm_serde_rules::deserialize::optional_schema"
    )]
    parameters: Option<String>,
    strict: Option<bool>,
}

#[derive(Deserialize)]
#[serde(rename_all = "snake_case")]
enum FunctionType {
    Function,
}

impl From<Tool> for ToolDefinition {
    fn from(value: Tool) -> Self {
        let FunctionType::Function = value.kind;
        Self {
            kind: ToolType::Function,
            function: FunctionDefinition {
                name: value.name,
                description: value.description,
                parameters: value.parameters,
                strict: value.strict,
            },
        }
    }
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub(super) struct Text {
    pub format: Format,
}

#[derive(Deserialize)]
#[serde(tag = "type", rename_all = "snake_case", deny_unknown_fields)]
pub(super) enum Format {
    Text,
    JsonObject,
    JsonSchema {
        #[serde(deserialize_with = "compallm_serde_rules::deserialize::nonempty")]
        name: String,
        description: Option<String>,
        strict: Option<bool>,
        #[serde(deserialize_with = "compallm_serde_rules::deserialize::schema")]
        schema: String,
    },
}

impl From<Format> for ResponseFormat {
    fn from(value: Format) -> Self {
        match value {
            Format::Text => Self::Text,
            Format::JsonObject => Self::JsonObject,
            Format::JsonSchema {
                name,
                description,
                strict,
                schema,
            } => Self::JsonSchema(JsonSchema {
                name,
                description,
                strict,
                schema,
            }),
        }
    }
}
