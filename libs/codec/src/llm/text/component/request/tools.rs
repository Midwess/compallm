#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ToolDefinition<S = String> {
    pub kind: ToolType,
    pub function: FunctionDefinition<S>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FunctionDefinition<S = String> {
    pub name: S,
    pub description: Option<S>,
    pub parameters: Option<S>,
    pub strict: Option<bool>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ToolType {
    Function,
}

impl<S> ToolDefinition<S> {
    pub fn map_text<T>(self, mut convert: impl FnMut(S) -> T) -> ToolDefinition<T> {
        let function = self.function;
        ToolDefinition {
            kind: self.kind,
            function: FunctionDefinition {
                name: convert(function.name),
                description: function.description.map(&mut convert),
                parameters: function.parameters.map(convert),
                strict: function.strict,
            },
        }
    }
}
