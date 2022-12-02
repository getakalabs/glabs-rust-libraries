/// Should implement `to_json()` function
pub trait ToJson {
    fn to_json(&self) -> serde_json::Value;
}

/// Should implement various codes such as new, clear, reconfigure and none
pub trait Configurable<T> {
    fn new() -> Self;

    fn clear(&mut self) -> Self;

    fn reconfigure(&mut self, item: &T);

    fn is_none(&self) -> bool;
}