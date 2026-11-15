#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Stop {
    Null,
    Text(String),
    Array(Vec<String>),
}
