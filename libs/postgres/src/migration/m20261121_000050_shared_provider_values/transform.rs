use serde_json::{Map, Value, json};

fn amount(value: &Value) -> Value {
    match value {
        Value::Null => Value::Null,
        Value::String(_) => value.clone(),
        Value::Number(n) => Value::String(n.to_string()),
        _ => Value::Null,
    }
}
fn rate(value: &Value, currency: &Value, quantity: &Value) -> Value {
    json!({"amount":amount(value),"currency":currency,"quantity":quantity})
}
fn protocol(value: &str) -> &str {
    match value {
        "openai_chat_completions" => "openai",
        "anthropic_messages" => "anthropic",
        v => v,
    }
}

fn inputs(value: &Value) -> Value {
    let rows = value.as_array().cloned().unwrap_or_default();
    Value::Array(rows.into_iter().map(|v|json!({"name":v["name"],"kind":v["type"],"required":v["required"],"allowedValues":v["allowed_values"]})).collect())
}
fn placement(value: &Value) -> Value {
    match value["location"].as_str() {
        Some("header") => json!({"apiKeyHeader":{"name":value["name"],"prefix":value["prefix"]}}),
        Some("query") => json!({"apiKeyQuery":value["name"]}),
        _ => Value::Null,
    }
}
fn authentication(method: &str, value: &Value, operation: Option<&str>) -> Vec<Value> {
    let placements = value["placements"].as_array().cloned().unwrap_or_default();
    let placements: Vec<_> = placements
        .iter()
        .filter(|p| {
            operation.is_none_or(|op| {
                p["operations"]
                    .as_array()
                    .is_some_and(|values| values.iter().any(|v| v.as_str() == Some(op)))
            })
        })
        .map(placement)
        .collect();
    let required = inputs(&value["required_inputs"]);
    match method {
        "api_key" => {
            if placements.is_empty() {
                vec![
                    json!({"type":"api_key","configuration":{"placement":null,"requiredInputs":required}}),
                ]
            } else {
                placements.into_iter().map(|p|json!({"type":"api_key","configuration":{"placement":p,"requiredInputs":required}})).collect()
            }
        }
        "oauth" => vec![
            json!({"type":"o_auth","configuration":{"flow":value["flow"],"authorizationUrl":value["authorization_url"],"tokenUrl":value["token_url"],"deviceAuthorizationUrl":value["device_authorization_url"],"scopes":value["scopes"].as_array().cloned().unwrap_or_default(),"tokenPlacement":placements.first().cloned().unwrap_or(Value::Null),"requiredInputs":required}}),
        ],
        "none" => vec![json!({"type":"none"})],
        "custom" => vec![
            json!({"type":"custom","configuration":{"adapter":value["adapter"],"parameters":{},"requiredInputs":required}}),
        ],
        _ => vec![],
    }
}
fn connection(old: &Value, auth: Value, path: Value) -> Value {
    json!({"baseUrl":old["base_url"],"path":path,"timeoutMs":null,"query":{},"authentication":auth,"endpoint":{"mode":old["endpoint_mode"],"urlTemplate":null,"requiredInputs":inputs(&old["endpoint"]["required_inputs"])}})
}
pub fn provider(row: &Value) -> Value {
    let mut options = Vec::new();
    if let Some(methods) = row["connection"]["authentication"].as_object() {
        for (method, config) in methods {
            options.extend(authentication(method, config, None));
        }
    }
    json!({"connection":connection(&row["connection"],options.first().cloned().unwrap_or(Value::Null),Value::Null),"authentication_options":options})
}
fn cache(mode: &str, amount: &Value, currency: &Value, quantity: &Value) -> Value {
    match mode {
        "separate_rate" => {
            json!({"type":"separate_rate","configuration":{"rate":rate(amount,currency,quantity)}})
        }
        "standard_input_rate" => json!({"type":"standard_input_rate"}),
        "included" => json!({"type":"included"}),
        "unsupported" => json!({"type":"unsupported"}),
        _ => Value::Null,
    }
}
fn token_rates(rates: &Value, mode: &str, currency: &Value, quantity: &Value) -> Value {
    json!({"input":rate(&rates["input_price"],currency,quantity),"output":rate(&rates["output_price"],currency,quantity),"cache":cache(mode,&rates["cached_price"],currency,quantity)})
}
fn tiers(value: &Value, mode: &str, currency: &Value, quantity: &Value) -> Value {
    json!({"measuredBy":"input_tokens","tiers":value.as_array().cloned().unwrap_or_default().iter().map(|v|json!({"fromTokens":v["from_tokens"],"toTokens":v["to_tokens"],"rates":token_rates(&v["rates"],mode,currency,quantity)})).collect::<Vec<_>>()})
}
fn period(value: &Value) -> Value {
    match value.as_str() {
        Some("monthly" | "month") => json!("month"),
        Some("daily" | "day") => json!("day"),
        Some("weekly" | "week") => json!("week"),
        Some("hourly" | "hour") => json!("hour"),
        Some("yearly" | "year") => json!("year"),
        _ => Value::Null,
    }
}
fn conditions(value: &Value) -> Value {
    let mut extra = Map::new();
    if let Some(fields) = value.as_object() {
        for (key, v) in fields {
            if !["resolution", "quality", "audio", "service_tier"].contains(&key.as_str()) {
                extra.insert(key.clone(), v.clone());
            }
        }
    }
    json!({"resolution":value["resolution"],"quality":value["quality"],"audio":value["audio"],"serviceTier":value["service_tier"],"parameters":extra})
}
fn pricing(value: &Value, service: Option<&str>) -> Value {
    let currency = value["configuration"]["currency"].clone();
    let quantity = value["configuration"]["unit_quantity"].clone();
    let cfg = &value["configuration"];
    let cache_mode = value["cache_billing"].as_str().unwrap_or("unknown");
    match value["billing_model"].as_str() {
        Some("free") => return json!({"free":{"description":cfg["reason"],"allowances":[]}}),
        Some("subscription" | "subscription_with_overage") => {
            let sub = &cfg["subscription"];
            let fee = &sub["fee"];
            let fee = if fee.is_object() {
                json!({"amount":amount(&fee["amount"]),"currency":fee["currency"],"period":period(&fee["period"])})
            } else {
                Value::Null
            };
            return json!({"subscription":{"product":sub["product"].as_str().unwrap_or(""),"plan":sub["plan"],"fee":fee,"entitlements":if sub["entitlement"].is_object(){vec![json!({"unit":sub["entitlement"]["unit"],"quota":sub["entitlement"]["quota"],"resetPeriod":period(&sub["entitlement"]["period"])})]}else{vec![]},"overage":null}});
        }
        _ => {}
    }
    if value["mode"] == "tiered" {
        return json!({"usage":{"tiered_token":tiers(&cfg["tiers"],cache_mode,&currency,&quantity)}});
    }
    if value["mode"] == "scheduled" {
        return json!({"usage":{"scheduled":{"timezone":cfg["schedule"]["timezone"],"slots":cfg["schedule"]["slots"].as_array().cloned().unwrap_or_default().iter().map(|v|json!({"fromHour":v["from_hour"],"toHour":v["to_hour"],"pricing":{"fixed":{"token":token_rates(&v["rates"],cache_mode,&currency,&quantity)}}})).collect::<Vec<_>>()}}});
    }
    if value["billing_unit"] == "tokens"
        || (service == Some("llm")
            && ["input_price", "output_price", "cached_price"]
                .iter()
                .any(|field| !value[*field].is_null()))
    {
        return json!({"usage":{"fixed":{"token":token_rates(value,cache_mode,&currency,&quantity)}}});
    }
    if value["billing_unit"] == "images" {
        return json!({"usage":{"fixed":{"image":{"rates":cfg["rates"].as_array().cloned().unwrap_or_default().iter().map(|v|json!({"rate":rate(&v["amount"],&currency,&v["quantity"]),"conditions":conditions(&v["conditions"])})).collect::<Vec<_>>()}}}});
    }
    Value::Null
}
fn service(value: &Value) -> Value {
    match value.as_str() {
        Some("text") => json!("llm"),
        Some("unknown") | None => Value::Null,
        _ => value.clone(),
    }
}
pub fn model(row: &Value, parent: &Value) -> Value {
    let service = service(&row["service_kind"]);
    let mut connections = Map::new();
    if let Some(operations) = row["connection"]["operations"].as_object() {
        for (operation, path) in operations {
            let mut choices = Vec::new();
            if let Some(methods) = parent["connection"]["authentication"].as_object() {
                for (method, config) in methods {
                    choices.extend(authentication(method, config, Some(operation)));
                }
            }
            connections.insert(
                protocol(operation).into(),
                connection(
                    &parent["connection"],
                    choices.first().cloned().unwrap_or(Value::Null),
                    path.clone(),
                ),
            );
        }
    }
    let mut routes = Vec::new();
    if let Some(options) = row["pricing"]["configuration"]["route_options"].as_array() {
        for option in options {
            let route_pricing = if option["tiers"].as_array().is_some_and(|v| !v.is_empty()) {
                json!({"usage":{"tiered_token":tiers(&option["tiers"],"separate_rate",&json!("USD"),&json!(1000000))}})
            } else {
                json!({"usage":{"fixed":{"token":token_rates(&option["rates"],"separate_rate",&json!("USD"),&json!(1000000))}}})
            };
            routes.push(json!({"id":option["tag"],"provider":option["provider"],"upstreamModelId":option["model_id"],"limits":{"contextTokens":option["context_tokens"],"maxInputTokens":option["max_input_tokens"],"maxOutputTokens":option["max_output_tokens"]},"pricing":route_pricing,"implicitCaching":option["implicit_caching"]}));
        }
    }
    json!({"service_kind":service,"connections":connections,"capabilities":row["capabilities"].as_object().map(|v|v.iter().filter(|(_,supported)|**supported==json!(true)).map(|(k,_)|k.clone()).collect::<Vec<_>>()).unwrap_or_default(),
 "limits":{"contextTokens":row["limits"]["max_context_tokens"],"maxInputTokens":row["limits"]["max_input_tokens"],"maxOutputTokens":row["limits"]["max_output_tokens"]},
 "thinking":if row["thinking"]["effort_mapping"].as_object().is_some_and(|v|!v.is_empty()){json!({"effortMapping":row["thinking"]["effort_mapping"],"budget":null})}else{Value::Null},
 "pricing":pricing(&row["pricing"],service.as_str()),
 "route_selection":if routes.is_empty(){Value::Null}else{json!({"options":routes,"selected":null,"allowFallbacks":row["pricing"]["configuration"]["route_policy"]["allow_fallbacks"],"exactProviderSelection":row["pricing"]["configuration"]["route_policy"]["exact_provider_selection"]})}})
}
pub fn configured_connection(value: &Value) -> Value {
    let mut result = value.as_object().cloned().unwrap_or_default();
    result.insert("path".into(), Value::Null);
    result.insert(
        "endpoint".into(),
        json!({"mode":"fixed","urlTemplate":null,"requiredInputs":[]}),
    );
    result.insert("authentication".into(),json!({"type":"api_key","configuration":{"placement":value["authentication"],"requiredInputs":[]}}));
    Value::Object(result)
}
pub fn configured_thinking(value: &Value) -> Value {
    if value.is_null() {
        return Value::Null;
    }
    let mut map = Map::new();
    if let Some(levels) = value["effort"].as_array() {
        for level in levels {
            if let Some(level) = level.as_str() {
                map.insert(level.into(), json!(level));
            }
        }
    }
    json!({"effortMapping":map,"budget":value["budget"]})
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

pub fn legacy_connection(value: &Value) -> Value {
    let mut result = value.as_object().cloned().unwrap_or_default();
    result.remove("path");
    result.remove("endpoint");
    result.insert(
        "authentication".into(),
        value["authentication"]["configuration"]["placement"].clone(),
    );
    Value::Object(result)
}

pub fn legacy_thinking(value: &Value) -> Value {
    if value.is_null() {
        return Value::Null;
    }
    let effort: Vec<_> = ["low", "medium", "high", "xhigh", "max"]
        .into_iter()
        .filter(|level| value["effortMapping"].get(*level).is_some())
        .collect();
    json!({"effort": effort, "budget": value["budget"]})
}

pub fn legacy_pricing(value: &Value) -> Value {
    let fixed = &value["usage"]["fixed"];
    if let Some(token) = fixed.get("token") {
        return json!({"token": {"inputUsdPerMillionTokens": token["input"]["amount"], "outputUsdPerMillionTokens": token["output"]["amount"], "cachedUsdPerMillionTokens": token["cache"]["configuration"]["rate"]["amount"]}});
    }
    if let Some(request) = fixed.get("request") {
        return json!({"request": {"usdPerRequest": request["rate"]["amount"]}});
    }
    if let Some(audio) = fixed.get("audio") {
        return if audio["type"] == "duration" {
            json!({"audio": {"usdPerSecond": audio["configuration"]["rate"]["amount"]}})
        } else {
            json!({"audio": {"usdPerMillionCharacters": audio["configuration"]["rate"]["amount"]}})
        };
    }
    for kind in ["image", "video"] {
        if let Some(media) = fixed.get(kind) {
            let rates: Vec<_> = media["rates"]
                .as_array()
                .cloned()
                .unwrap_or_default()
                .into_iter()
                .map(|v| {
                    let mut row = v.as_object().cloned().unwrap_or_default();
                    row.remove("rate");
                    row.remove("unit");
                    row.insert(
                        if kind == "image" {
                            "usdPerImage"
                        } else {
                            "usdPerSecond"
                        }
                        .into(),
                        v["rate"]["amount"].clone(),
                    );
                    Value::Object(row)
                })
                .collect();
            return json!({kind: {"rates": rates}});
        }
    }
    Value::Null
}
