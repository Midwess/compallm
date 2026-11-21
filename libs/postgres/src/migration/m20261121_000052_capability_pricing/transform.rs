#[path = "legacy.rs"]
mod legacy;
use serde_json::{Value, json};

pub const FIELDS: [&str; 12] = [
    "text",
    "vision",
    "pdf",
    "audio",
    "video",
    "image_generation",
    "tools",
    "structured_output",
    "search",
    "billing",
    "route_selection",
    "limits",
];
fn unknown() -> Value {
    json!({"status":"unknown"})
}
fn supported(configuration: Value, pricing: Value) -> Value {
    json!({"status":"supported","configuration":configuration,"pricing":pricing})
}
fn charge(value: Value) -> Value {
    json!({"type":"usage","configuration":value})
}
fn token(value: &Value, direction: &str) -> Value {
    json!({"rate":value[direction],"cache":if direction=="input"{value["cache"].clone()}else{Value::Null}})
}
fn usage(value: &Value, direction: &str) -> Option<Value> {
    if let Some(fixed) = value.get("fixed") {
        return fixed
            .get("token")
            .map(|v| json!({"fixed":{"token":token(v,direction)}}));
    }
    if let Some(tiers) = value.get("tiered_token") {
        return Some(
            json!({"tiered_token":{"measuredBy":tiers["measuredBy"],"tiers":tiers["tiers"].as_array()?.iter().map(|t|json!({"fromTokens":t["fromTokens"],"toTokens":t["toTokens"],"rate":token(&t["rates"],direction)})).collect::<Vec<_>>()}}),
        );
    }
    if let Some(schedule) = value.get("scheduled") {
        let slots:Option<Vec<_>>=schedule["slots"].as_array()?.iter().map(|s|usage(&s["pricing"],direction).map(|pricing|json!({"fromHour":s["fromHour"],"toHour":s["toHour"],"pricing":pricing}))).collect();
        return Some(json!({"scheduled":{"timezone":schedule["timezone"],"slots":slots?}}));
    }
    None
}
fn attach(feature: &mut Value, pricing: Value) -> bool {
    if feature["status"] == "supported" {
        feature["pricing"] = pricing;
        true
    } else {
        false
    }
}
fn upgrade(value: &Value) -> Value {
    let mut v = value.clone();
    if v["status"] == "supported" {
        v["pricing"] = Value::Null;
    }
    v
}
fn allocate(result: &mut Value, policy: &Value, service: Option<&str>) -> bool {
    if let (Some(input), Some(output)) = (usage(policy, "input"), usage(policy, "output")) {
        let input_ok = attach(&mut result["text"]["input"], charge(input));
        let output_ok = attach(&mut result["text"]["output"], charge(output));
        let has_amount =
            |direction: &str| policy["fixed"]["token"][direction]["amount"].is_string();
        return (input_ok || (!has_amount("input") && policy["fixed"].get("token").is_some()))
            && (output_ok
                || matches!(service, Some("embedding" | "rerank"))
                || (!has_amount("output") && policy["fixed"].get("token").is_some()));
    }
    let fixed = &policy["fixed"];
    if let Some(credits) = fixed.get("credit") {
        let mut groups: std::collections::BTreeMap<(String, String), Vec<Value>> =
            std::collections::BTreeMap::new();
        for rate in credits["rates"].as_array().cloned().unwrap_or_default() {
            let target = match (rate["unit"].as_str(), rate["direction"].as_str(), service) {
                (Some("tokens"), Some("input" | "cache_read"), _) => ("text", "input"),
                (Some("tokens"), Some("output"), _) => ("text", "output"),
                (Some("images"), _, _) => ("image_generation", ""),
                (Some("videos"), _, _) | (Some("seconds"), _, Some("video")) => ("video", ""),
                (Some("characters" | "utf8_bytes" | "seconds"), _, Some("tts")) => {
                    ("audio", "output")
                }
                (Some("seconds"), _, Some("stt")) => ("audio", "input"),
                _ => return false,
            };
            groups
                .entry((target.0.into(), target.1.into()))
                .or_default()
                .push(rate);
        }
        for ((name, direction), rates) in groups {
            let price = charge(json!({"fixed":{"credit":{"rates":rates}}}));
            let feature = if direction.is_empty() {
                &mut result[&name]
            } else {
                &mut result[&name][&direction]
            };
            if !attach(feature, price) {
                return false;
            }
        }
        result["billing"] = json!({"credit":credits.get("conversion").filter(|v|v.is_object()).cloned().unwrap_or(json!({"amountPerCredit":null,"currency":null}))});
        return true;
    }
    if fixed.get("image").is_some() {
        if result["image_generation"]["status"] != "supported" {
            result["image_generation"] =
                supported(json!({"formats":null,"maxImages":null}), Value::Null);
        }
        return attach(&mut result["image_generation"], charge(policy.clone()));
    }
    if fixed.get("video").is_some() {
        if result["video"]["status"] != "supported" {
            result["video"] = supported(
                json!({"formats":null,"maxDurationMs":null,"maxFrames":null}),
                Value::Null,
            );
        }
        return attach(&mut result["video"], charge(policy.clone()));
    }
    if fixed.get("audio").is_some() {
        let direction = match service {
            Some("stt") => "input",
            Some("tts") => "output",
            _ => {
                if result["audio"]["input"]["status"] == "supported"
                    && result["audio"]["output"]["status"] != "supported"
                {
                    "input"
                } else if result["audio"]["output"]["status"] == "supported"
                    && result["audio"]["input"]["status"] != "supported"
                {
                    "output"
                } else {
                    return false;
                }
            }
        };
        if result["audio"][direction]["status"] != "supported" {
            result["audio"][direction] = supported(
                json!({"formats":null,"maxDurationMs":null,"maxBytes":null}),
                Value::Null,
            );
        }
        return attach(&mut result["audio"][direction], charge(policy.clone()));
    }
    false
}
fn each_feature(result: &mut Value, pricing: Value) {
    for name in [
        "vision",
        "pdf",
        "video",
        "image_generation",
        "structured_output",
        "search",
    ] {
        attach(&mut result[name], pricing.clone());
    }
    for name in ["input", "output"] {
        attach(&mut result["text"][name], pricing.clone());
        attach(&mut result["audio"][name], pricing.clone());
    }
    for name in ["calling", "forcedChoice", "parallelCalls"] {
        attach(&mut result["tools"][name], pricing.clone());
    }
    // Thinking uses a separate price only when one was explicitly stored.
}
pub fn convert(row: &Value) -> Value {
    let mut normalized = row.clone();
    if ["token", "audio", "image", "video", "request"]
        .iter()
        .any(|name| row["pricing"].get(name).is_some())
    {
        normalized["pricing"] = legacy::configured_pricing(&row["pricing"]);
    }
    let row = &normalized;
    let service = row["service_kind"].as_str();
    let oldtext = &row["text"];
    let text_supported = oldtext["status"] == "supported";
    let input_support =
        if text_supported || matches!(service, Some("llm" | "embedding" | "rerank" | "tts")) {
            true
        } else {
            row["limits"]["maxInputTokens"].is_number()
        };
    let output_support =
        if text_supported && !matches!(service, Some("embedding" | "rerank" | "tts")) {
            true
        } else {
            matches!(service, Some("llm" | "stt"))
                || row["thinking"]["status"] == "supported"
                || row["limits"]["maxOutputTokens"].is_number()
        };
    let no_text = if oldtext["status"] == "unsupported" {
        json!({"status":"unsupported"})
    } else {
        unknown()
    };
    let mut result = json!({
    "text":{"input":if input_support{supported(json!({"maxTokens":row["limits"]["maxInputTokens"]}),Value::Null)}else{no_text.clone()},"output":if output_support{supported(json!({"maxTokens":row["limits"]["maxOutputTokens"],"thinking":upgrade(&row["thinking"])}),Value::Null)}else{no_text}},
    "vision":upgrade(&row["vision"]),"pdf":upgrade(&row["pdf"]),"audio":{"input":upgrade(&row["audio"]["input"]),"output":upgrade(&row["audio"]["output"])},
    "video":upgrade(&row["video"]),"image_generation":upgrade(&row["image_generation"]),"tools":{"calling":upgrade(&row["tools"]["calling"]),"forcedChoice":upgrade(&row["tools"]["forcedChoice"]),"parallelCalls":upgrade(&row["tools"]["parallelCalls"])},
    "structured_output":upgrade(&row["structured_output"]),"search":upgrade(&row["search"]),"billing":null,
    "limits":{"contextTokens":row["limits"]["contextTokens"]},"route_selection":row["route_selection"]
    });
    if result["structured_output"]["status"] == "supported" {
        for name in ["jsonMode", "jsonSchema"] {
            result["structured_output"]["configuration"][name] =
                upgrade(&row["structured_output"]["configuration"][name]);
        }
    }
    if let Some(policy) = row["pricing"].get("usage") {
        if let Some(request) = policy["fixed"].get("request") {
            result["billing"] = json!({"request":request});
            each_feature(&mut result, json!({"type":"model_billing"}));
        } else if !allocate(&mut result, policy, service) {
            result["allocation_error"] = json!(
                "Usage direction or capability was not established by the stored configuration"
            );
        }
    } else if let Some(free) = row["pricing"].get("free") {
        result["billing"] = json!({"free":free});
        match service {
            Some("llm" | "embedding" | "rerank") => {
                attach(&mut result["text"]["input"], json!({"type":"free"}));
                attach(&mut result["text"]["output"], json!({"type":"free"}));
            }
            Some("image") => {
                attach(&mut result["image_generation"], json!({"type":"free"}));
            }
            Some("video") => {
                attach(&mut result["video"], json!({"type":"free"}));
            }
            Some("tts") => {
                attach(&mut result["audio"]["output"], json!({"type":"free"}));
            }
            Some("stt") => {
                attach(&mut result["audio"]["input"], json!({"type":"free"}));
            }
            _ => {}
        }
    } else if let Some(sub) = row["pricing"].get("subscription") {
        let mut billing = sub.as_object().cloned().unwrap_or_default();
        billing.remove("overage");
        result["billing"] = json!({"subscription":billing});
        each_feature(&mut result, json!({"type":"model_billing"}));
        if sub["overage"].is_object() && !allocate(&mut result, &sub["overage"], service) {
            result["allocation_error"] =
                json!("Subscription overage capability was not established");
        }
    }
    if let Some(routes) = row["route_selection"]["options"].as_array() {
        let options:Vec<_>=routes.iter().map(|route|{
   let mut r=route.as_object().cloned().unwrap();
   let transformed=convert(&json!({"service_kind":"llm","text":{"status":"supported","configuration":{}},"vision":unknown(),"pdf":unknown(),"audio":{"input":unknown(),"output":unknown()},"video":unknown(),"image_generation":unknown(),"thinking":unknown(),"tools":{"calling":unknown(),"forcedChoice":unknown(),"parallelCalls":unknown()},"structured_output":unknown(),"search":unknown(),"pricing":route["pricing"],"limits":route["limits"],"route_selection":null}));
   r.remove("pricing");r.insert("text".into(),transformed["text"].clone());r.insert("billing".into(),transformed["billing"].clone());r.insert("limits".into(),transformed["limits"].clone());Value::Object(r)
  }).collect();
        result["route_selection"]["options"] = json!(options);
    }
    result
}
