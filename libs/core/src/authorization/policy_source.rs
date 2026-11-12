use cedar_policy::{EntityUid, PolicySet};
use cedar_policy_formatter::lexer::get_token_stream;
use std::collections::BTreeMap;

/// Remove comments and unnecessary whitespace without altering Cedar tokens.
pub fn compact_policy_source(source: &str) -> String {
    let Some((tokens, _)) = get_token_stream(source) else {
        return source.to_owned();
    };
    if tokens.is_empty() {
        return String::new();
    }
    let mut result = String::with_capacity(source.len());
    for (index, token) in tokens.iter().enumerate() {
        if index > 0 {
            let previous = &tokens[index - 1];
            let left = &source[previous.span.clone()];
            let right = &source[token.span.clone()];
            let pair = format!("{left}{right}");
            let unchanged = get_token_stream(&pair).is_some_and(|(pair_tokens, _)| {
                pair_tokens.len() == 2
                    && pair_tokens[0].token == previous.token
                    && pair_tokens[1].token == token.token
                    && pair_tokens[0].span.end == left.len()
            });
            if !unchanged {
                result.push(' ');
            }
        }
        result.push_str(&source[token.span.clone()]);
    }
    result
}

pub fn compact_policy_input(source: &mut String) {
    *source = compact_policy_source(source);
}

fn current_entities(
    organization_id: u64,
    role_id: u64,
) -> anyhow::Result<BTreeMap<EntityUid, EntityUid>> {
    Ok(BTreeMap::from([
        (
            "Compallm::Organization::\"current\"".parse()?,
            format!("Compallm::Organization::\"{organization_id}\"").parse()?,
        ),
        (
            "Compallm::Role::\"current\"".parse()?,
            format!("Compallm::Role::\"{organization_id}:{role_id}\"").parse()?,
        ),
    ]))
}

fn substitute_policy_entities(
    source: &str,
    replacements: BTreeMap<EntityUid, EntityUid>,
) -> anyhow::Result<String> {
    let source = compact_policy_source(source);
    let policies: PolicySet = source.parse()?;
    anyhow::ensure!(
        policies.templates().next().is_none(),
        "unlinked role templates are unsupported"
    );
    let (tokens, _) =
        get_token_stream(&source).ok_or_else(|| anyhow::anyhow!("invalid Cedar tokens"))?;
    let mut edits = Vec::new();
    for (index, window) in tokens.windows(5).enumerate() {
        if &source[window[0].span.clone()] != "Compallm"
            || &source[window[1].span.clone()] != "::"
            || &source[window[3].span.clone()] != "::"
            || (index > 0 && &source[tokens[index - 1].span.clone()] == "::")
        {
            continue;
        }
        let range = window[0].span.start..window[4].span.end;
        if let Ok(uid) = source[range.clone()].parse::<EntityUid>()
            && let Some(replacement) = replacements.get(&uid)
        {
            edits.push((range, replacement.to_string()));
        }
    }
    let mut source = source.to_owned();
    for (range, replacement) in edits.into_iter().rev() {
        source.replace_range(range, &replacement);
    }
    Ok(compact_policy_source(&source))
}

/// Resolve authoring aliases from the verified target role and organization.
pub fn bind_current_policy_source(
    source: &str,
    organization_id: u64,
    role_id: u64,
) -> anyhow::Result<String> {
    substitute_policy_entities(source, current_entities(organization_id, role_id)?)
}

/// Present this role's own entity references as aliases, retaining other entities.
pub fn authoring_policy_source(
    source: &str,
    organization_id: u64,
    role_id: u64,
) -> anyhow::Result<String> {
    let replacements = current_entities(organization_id, role_id)?
        .into_iter()
        .map(|(alias, concrete)| (concrete, alias))
        .collect();
    substitute_policy_entities(source, replacements)
}
