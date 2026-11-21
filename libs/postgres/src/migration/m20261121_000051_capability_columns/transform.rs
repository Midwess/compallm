use serde_json::{Value, json};

pub const COLUMNS: [&str; 11] = [
    "text",
    "vision",
    "pdf",
    "audio",
    "video",
    "image_generation",
    "thinking",
    "tools",
    "structured_output",
    "search",
    "limits",
];

fn support(value: Option<bool>, configuration: Value) -> Value {
    match value {
        Some(true) => json!({"status":"supported","configuration":configuration}),
        Some(false) => json!({"status":"unsupported"}),
        None => json!({"status":"unknown"}),
    }
}
fn with_limit(supported: Option<bool>, key: &str, limit: &Value) -> Value {
    support(
        if limit.is_number() {
            Some(true)
        } else {
            supported
        },
        json!({key:limit}),
    )
}
pub fn convert(row: &Value, observations: &Value) -> Value {
    let list = row["capabilities"].as_array().cloned().unwrap_or_default();
    let flag = |name: &str| {
        if list.iter().any(|v| v == name) {
            Some(true)
        } else if observations[name] == false {
            Some(false)
        } else {
            None
        }
    };
    let limits = &row["limits"];
    let image_service = row["service_kind"] == "image";
    let audio_service = matches!(row["service_kind"].as_str(), Some("audio" | "tts" | "stt"));
    let mut thinking = row["thinking"].clone();
    if let Some(effort) = thinking.get("effort") {
        let mapping: serde_json::Map<String, Value> = effort
            .as_array()
            .cloned()
            .unwrap_or_default()
            .into_iter()
            .filter_map(|v| v.as_str().map(|s| (s.to_owned(), json!(s))))
            .collect();
        thinking = json!({"effortMapping":mapping,"budget":thinking["budget"]});
    }
    if thinking.is_object() && thinking.get("defaultEffort").is_none() {
        thinking["defaultEffort"] = Value::Null;
    }
    let thinking_has_controls = thinking["budget"].is_object()
        || thinking["effortMapping"]
            .as_object()
            .is_some_and(|v| !v.is_empty());
    let thinking = support(
        if thinking_has_controls {
            Some(true)
        } else {
            flag("reasoning")
        },
        if thinking.is_object() {
            thinking
        } else {
            json!({"effortMapping":{},"defaultEffort":null,"budget":null})
        },
    );
    let mut token_limits = limits.as_object().cloned().unwrap_or_default();
    for name in ["maxImagesPerRequest", "maxDurationMs", "maxAudioBytes"] {
        token_limits.remove(name);
    }
    let mut vision = with_limit(
        flag("vision"),
        "maxImages",
        if image_service {
            &Value::Null
        } else {
            &limits["maxImagesPerRequest"]
        },
    );
    let mut image = with_limit(
        flag("image_generation"),
        "maxImages",
        if image_service {
            &limits["maxImagesPerRequest"]
        } else {
            &Value::Null
        },
    );
    for value in [&mut vision, &mut image] {
        if value["status"] == "supported" {
            value["configuration"]["formats"] = Value::Null;
        }
    }
    let audio_config = json!({"formats":null,"maxDurationMs":if audio_service && row["service_kind"]!="tts"{limits["maxDurationMs"].clone()}else{Value::Null},"maxBytes":limits["maxAudioBytes"]});
    let audio_input = support(
        if limits["maxAudioBytes"].is_number()
            || (audio_service
                && row["service_kind"] != "tts"
                && limits["maxDurationMs"].is_number())
        {
            Some(true)
        } else {
            flag("audio")
        },
        audio_config.clone(),
    );
    let audio_output = support(
        if row["service_kind"] == "tts" && limits["maxDurationMs"].is_number() {
            Some(true)
        } else {
            flag("audio_generation")
        },
        json!({"formats":null,"maxDurationMs":if row["service_kind"]=="tts"{limits["maxDurationMs"].clone()}else{Value::Null},"maxBytes":null}),
    );
    json!({
        "text":support(flag("text"),json!({})),
        "vision":vision,"pdf":support(flag("pdf"),json!({"maxFiles":null,"maxPages":null})),
        "audio":{"input":audio_input,"output":audio_output},
        "video":support(if !audio_service && limits["maxDurationMs"].is_number(){Some(true)}else{flag("video")},json!({"formats":null,"maxDurationMs":if audio_service{Value::Null}else{limits["maxDurationMs"].clone()},"maxFrames":null})),
        "image_generation":image,"thinking":thinking,
        "tools":{"calling":support(flag("tool_call"),json!({"maxTools":null})),"forcedChoice":support(flag("forced_tool_choice"),json!({})),"parallelCalls":{"status":"unknown"}},
        "structured_output":support(flag("structured_output"),json!({"jsonMode":{"status":"unknown"},"jsonSchema":{"status":"unknown"}})),
        "search":support(flag("search"),json!({"maxResults":null})),"limits":token_limits
    })
}
