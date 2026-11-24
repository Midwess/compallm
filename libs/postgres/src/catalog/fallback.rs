use super::records;
use sea_orm::{ConnectionTrait, DbErr};
use serde_json::{Value, json};
use std::collections::{BTreeMap, BTreeSet};
struct Rule {
    pattern: fancy_regex::Regex,
    providers: BTreeSet<String>,
    defaults: Vec<(String, Value)>,
}
pub(crate) struct Rules(Vec<Rule>);
impl Rules {
    pub(crate) async fn load<C: ConnectionTrait>(db: &C) -> Result<Self, DbErr> {
        let mut rules = records::rows(db, "catalog_fallback_book", None).await?;
        rules.sort_by_key(|v| v["priority"].as_i64());
        let scopes = records::rows(db, "catalog_fallback_provider", None).await?;
        let facts = records::rows(db, "catalog_fallback_fact", None).await?;
        let mut out = vec![];
        for rule in rules {
            if rule["rule_kind"] != "capability" || rule["fill_missing"] != true {
                continue;
            }
            let providers = scopes
                .iter()
                .filter(|v| v["rule_id"] == rule["id"])
                .map(|v| v["provider_catalog_id"].as_str().unwrap().to_owned())
                .collect();
            let defaults = facts
                .iter()
                .filter(|v| v["rule_id"] == rule["id"])
                .map(|v| {
                    (
                        v["field"].as_str().unwrap().to_owned(),
                        v.get("boolean_value")
                            .filter(|v| !v.is_null())
                            .or_else(|| v.get("integer_value").filter(|v| !v.is_null()))
                            .or_else(|| v.get("text_value"))
                            .cloned()
                            .unwrap_or(Value::Null),
                    )
                })
                .collect();
            let pattern = fancy_regex::RegexBuilder::new(&format!(
                "(?i){}",
                rule["model_pattern"].as_str().unwrap()
            ))
            .backtrack_limit(10000)
            .build()
            .map_err(|e| DbErr::Custom(e.to_string()))?;
            out.push(Rule {
                pattern,
                providers,
                defaults,
            });
        }
        Ok(Self(out))
    }
    pub(crate) fn apply(&self, provider: &str, name: &str, model: &mut Value) -> Result<(), DbErr> {
        let mut defaults = BTreeMap::new();
        for rule in &self.0 {
            if rule.providers.contains(provider)
                && rule
                    .pattern
                    .is_match(name)
                    .map_err(|e| DbErr::Custom(e.to_string()))?
            {
                defaults.extend(rule.defaults.iter().cloned());
            }
        }
        for (key, pointer, configuration) in [
            (
                "supports_function_calling",
                "/tools/calling",
                json!({"maxTools":null}),
            ),
            (
                "supports_parallel_function_calling",
                "/tools/parallelCalls",
                json!({}),
            ),
            ("supports_tool_choice", "/tools/forcedChoice", json!({})),
            (
                "supports_vision",
                "/vision",
                json!({"formats":null,"maxImages":null}),
            ),
            (
                "supports_pdf_input",
                "/pdf",
                json!({"maxFiles":null,"maxPages":null}),
            ),
            (
                "supports_response_schema",
                "/structuredOutput",
                json!({"jsonMode":{"status":"unknown"},"jsonSchema":{"status":"unknown"}}),
            ),
            ("supports_web_search", "/search", json!({"maxResults":null})),
            (
                "supports_reasoning",
                "/text/output/configuration/thinking",
                json!({"effortMapping":{},"defaultEffort":null,"budget":null}),
            ),
        ] {
            if let Some(flag) = defaults.get(key).and_then(Value::as_bool)
                && let Some(target) = model.pointer_mut(pointer)
                && target["status"] == "unknown"
            {
                *target = if flag {
                    json!({"status":"supported","configuration":configuration,"pricing":null})
                } else {
                    json!({"status":"unsupported"})
                };
            }
        }
        let context = model["limits"]["contextTokens"].as_u64();
        for (key, pointer) in [
            ("max_input_tokens", "/text/input/configuration/maxTokens"),
            ("max_output_tokens", "/text/output/configuration/maxTokens"),
        ] {
            if let Some(v) = defaults.get(key) {
                if context
                    .zip(v.as_u64())
                    .is_some_and(|(context, limit)| limit > context)
                {
                    continue;
                }
                if let Some(target) = model.pointer_mut(pointer)
                    && target.is_null()
                {
                    *target = v.clone();
                }
            }
        }
        Ok(())
    }
}
