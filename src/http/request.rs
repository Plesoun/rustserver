use super::method::Method;

pub struct Request {
    path: String,
    query_string: Option<String>, //can be None or some, it is a way to express absence of a value in a type-safe way (no no pointer exceptions)
    // use enums here instead of string
    method: Method,
}
