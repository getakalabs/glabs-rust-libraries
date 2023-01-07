#[macro_export]
macro_rules! set_error_messages {
    ($obj:expr, $( $key:ident = $val:expr ),*) => {
        {
            use serde_json;

            let mut _obj = $obj;
            let mut map = serde_json::Map::new();
            $(
                map.insert(stringify!($key).to_owned(), serde_json::to_value($val.to_string()).unwrap_or(serde_json::Value::Null));
            )*
            let json = serde_json::to_string(&map).unwrap_or(String::default());
            let value = serde_json::from_str(&json).unwrap_or(serde_json::Value::Null);
            _obj = serde_json::from_value(value).unwrap_or(_obj);
            _obj
        }
    }
}