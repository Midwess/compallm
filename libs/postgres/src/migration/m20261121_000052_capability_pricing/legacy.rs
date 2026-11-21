use serde_json::{Value, json};
fn rate(value: &Value, currency: &Value, quantity: &Value) -> Value {
    let amount = match value {
        Value::Null => Value::Null,
        Value::String(_) => value.clone(),
        Value::Number(v) => json!(v.to_string()),
        _ => Value::Null,
    };
    json!({"amount":amount,"currency":currency,"quantity":quantity})
}
pub fn configured_pricing(value: &Value) -> Value {
    if let Some(token) = value.get("token") {
        return json!({"usage":{"fixed":{"token":{"input":rate(&token["inputUsdPerMillionTokens"],&json!("USD"),&json!(1000000)),"output":rate(&token["outputUsdPerMillionTokens"],&json!("USD"),&json!(1000000)),"cache":{"type":"separate_rate","configuration":{"rate":rate(&token["cachedUsdPerMillionTokens"],&json!("USD"),&json!(1000000))}}}}}});
    }
    if let Some(request) = value.get("request") {
        return json!({"usage":{"fixed":{"request":{"rate":rate(&request["usdPerRequest"],&json!("USD"),&json!(1))}}}});
    }
    if let Some(audio) = value.get("audio") {
        let audio = if !audio["usdPerSecond"].is_null() {
            json!({"type":"duration","configuration":{"rate":rate(&audio["usdPerSecond"],&json!("USD"),&json!(1))}})
        } else {
            json!({"type":"character","configuration":{"rate":rate(&audio["usdPerMillionCharacters"],&json!("USD"),&json!(1000000))}})
        };
        return json!({"usage":{"fixed":{"audio":audio}}});
    }
    for kind in ["image", "video"] {
        if let Some(media) = value.get(kind) {
            let rates = media["rates"]
                .as_array()
                .cloned()
                .unwrap_or_default()
                .iter()
                .map(|v| {
                    let mut v = v.as_object().cloned().unwrap_or_default();
                    let field = if kind == "image" {
                        "usdPerImage"
                    } else {
                        "usdPerSecond"
                    };
                    let amount = v.remove(field).unwrap_or(Value::Null);
                    v.insert("rate".into(), rate(&amount, &json!("USD"), &json!(1)));
                    if kind == "video" {
                        v.insert("unit".into(), json!("seconds"));
                    }
                    Value::Object(v)
                })
                .collect::<Vec<_>>();
            return json!({"usage":{"fixed":{kind:{"rates":rates}}}});
        }
    }
    Value::Null
}
