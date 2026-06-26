#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SqlExecutionKind {
    ReadRows,
    Execute,
    InsertReturningId,
}
