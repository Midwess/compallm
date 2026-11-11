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
    let mut fields = 0;
    root_fields(
        &selected.selection_set,
        &document,
        &mut Vec::new(),
        &mut fields,
    )?;
    if fields == 1 { Ok(()) } else { Err(()) }
}

fn root_fields<'a>(
    selections: &[Selection<'a>],
    document: &[Definition<'a, juniper::DefaultScalarValue>],
    visiting: &mut Vec<&'a str>,
    fields: &mut usize,
) -> Result<(), ()> {
    for selection in selections {
        match selection {
            Selection::Field(_) => *fields += 1,
            Selection::InlineFragment(fragment) => {
                root_fields(&fragment.item.selection_set, document, visiting, fields)?
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
                root_fields(&fragment.selection_set, document, visiting, fields)?;
                visiting.pop();
            }
        }
        if *fields > 1 {
            return Err(());
        }
    }
    Ok(())
}
