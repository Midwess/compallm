//! Generic GraphQL execution constraints; independent of credential requirements.
use super::ApplicationSchema;
use juniper::{Definition, OperationType, Selection, http::GraphQLRequest};

pub(super) fn validate(request: &GraphQLRequest, schema: &ApplicationSchema) -> Result<(), ()> {
    let document =
        juniper::parser::parse_document_source(&request.query, &schema.schema).map_err(|_| ())?;
    let operations = document
        .iter()
        .filter_map(|definition| match definition {
            Definition::Operation(operation) => Some(&operation.item),
            _ => None,
        })
        .collect::<Vec<_>>();
    let selected = if let Some(name) = &request.operation_name {
        let matches = operations
            .iter()
            .filter(|op| op.name.as_ref().is_some_and(|value| value.item == name))
            .collect::<Vec<_>>();
        if matches.len() != 1 {
            return Err(());
        }
        *matches[0]
    } else {
        if operations.len() != 1 {
            return Err(());
        }
        operations[0]
    };
    if selected.operation_type == OperationType::Query {
        return Ok(());
    }
    if selected.operation_type != OperationType::Mutation {
        return Err(());
    }
    let root = single_field(&selected.selection_set, &document, &mut Vec::new())?;
    let Selection::Field(root) = root else {
        return Err(());
    };
    if root.item.name.item == "authentication" {
        single_field(
            root.item.selection_set.as_deref().ok_or(())?,
            &document,
            &mut Vec::new(),
        )?;
    }
    if root.item.name.item == "organization" {
        let domain = single_field(
            root.item.selection_set.as_deref().ok_or(())?,
            &document,
            &mut Vec::new(),
        )?;
        let Selection::Field(domain) = domain else {
            return Err(());
        };
        single_field(
            domain.item.selection_set.as_deref().ok_or(())?,
            &document,
            &mut Vec::new(),
        )?;
    }
    Ok(())
}

fn single_field<'a, 'b>(
    selections: &'b [Selection<'a>],
    document: &'b [Definition<'a, juniper::DefaultScalarValue>],
    visiting: &mut Vec<&'a str>,
) -> Result<&'b Selection<'a>, ()> {
    let mut found = None;
    collect_field(selections, document, visiting, &mut found)?;
    found.ok_or(())
}

fn collect_field<'a, 'b>(
    selections: &'b [Selection<'a>],
    document: &'b [Definition<'a, juniper::DefaultScalarValue>],
    visiting: &mut Vec<&'a str>,
    found: &mut Option<&'b Selection<'a>>,
) -> Result<(), ()> {
    for selection in selections {
        match selection {
            Selection::Field(_) => {
                if found.replace(selection).is_some() {
                    return Err(());
                }
            }
            Selection::InlineFragment(fragment) => {
                collect_field(&fragment.item.selection_set, document, visiting, found)?
            }
            Selection::FragmentSpread(spread) => {
                let name = spread.item.name.item;
                if visiting.contains(&name) {
                    return Err(());
                }
                let fragment = document
                    .iter()
                    .find_map(|definition| match definition {
                        Definition::Fragment(fragment) if fragment.item.name.item == name => {
                            Some(&fragment.item)
                        }
                        _ => None,
                    })
                    .ok_or(())?;
                visiting.push(name);
                collect_field(&fragment.selection_set, document, visiting, found)?;
                visiting.pop();
            }
        }
    }
    Ok(())
}
