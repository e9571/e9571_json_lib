use serde_json::{Value, json};
use std::collections::HashMap;
use e9571_lib1::e9571_lib1::{word_split, get_data_preg_list};

pub mod e9571_json_lib {
    use super::*;

    /// JSON 快速打包 (Json_Package)
    /// Serializes a JSON value to a string, returns empty string on error
    pub fn json_package(value: &Value) -> String {
        serde_json::to_string(value).unwrap_or_default()
    }

    /// 高级 JSON 数据兼容转换包 (Str_To_Json)
    /// Converts a JSON string to a map, with fallback to non-standard parsing
    pub fn str_to_json(str: &str) -> HashMap<String, String> {
        let mut json_value = match json_to_map(str) {
            Ok(map) => map,
            Err(_) => str_interface_to_json(str),
        };
        if json_value.is_empty() {
            json_value = str_interface_to_json(str);
        }
        json_value
    }

    /// 将 Map 转换成 JSON (Map_to_json)
    /// Converts a map to a JSON string, returns error message on failure
    pub fn map_to_json(m: &HashMap<String, Value>) -> String {
        serde_json::to_string(m).unwrap_or_else(|e| e.to_string())
    }

    /// 将 JSON 转换成 Map 只转换一级结构 (Json_to_map)
    /// Converts a JSON string to a map of strings, handling string, int, and float64 values
    pub fn json_to_map(json_str: &str) -> Result<HashMap<String, String>, Box<dyn std::error::Error>> {
        let result: Value = serde_json::from_str(json_str)?;
        let mut value_str = HashMap::new();

        if let Value::Object(map) = result {
            for (key, value) in map {
                match typeof_json(&value) {
                    "string" => {
                        if let Value::String(s) = value {
                            value_str.insert(key, s);
                        }
                    }
                    "int" => {
                        if let Value::Number(n) = value {
                            if let Some(i) = n.as_i64() {
                                value_str.insert(key, i.to_string());
                            }
                        }
                    }
                    "float64" => {
                        if let Value::Number(n) = value {
                            if let Some(f) = n.as_f64() {
                                value_str.insert(key, format!("{:.}", f));
                            }
                        }
                    }
                    _ => {
                        value_str.insert(key, "0".to_string());
                    }
                }
            }
        }
        Ok(value_str)
    }

    /// 数据反射专用函数 精确解析数据类型 (Typeof_Json)
    /// Returns the type of a JSON value as a static string
    pub fn typeof_json(v: &Value) -> &str {
        match v {
            Value::String(_) => "string",
            Value::Number(n) => {
                if n.is_i64() || n.is_u64() {
                    "int"
                } else {
                    "float64"
                }
            }
            _ => "unknown",
        }
    }

    /// 非标准 JSON 直接转换 (Str_interface_to_json)
    /// Parses non-standard JSON (e.g., "key:value") into a map
    pub fn str_interface_to_json(source: &str) -> HashMap<String, String> {
        let source = source.replace("{", "").replace("}", "");
        let list_str = word_split(&source, ",");
        let mut result = HashMap::new();

        for item in list_str {
            let list_str_tmp = word_split(&item, ":");
            if list_str_tmp.len() == 2 {
                result.insert(list_str_tmp[0].clone(), list_str_tmp[1].clone());
            }
        }
        result
    }

    /// Node.js Map 类操作参数 (Str_To_Json_node)
    /// Parses Node.js-style JSON array of key-value pairs into a map
    pub fn str_to_json_node(str: &str) -> HashMap<String, String> {
        let mut result = HashMap::new();
        let regexp_str = r#""[.\s\S]*?""#;
        let list = get_data_preg_list(regexp_str, str);

        for item in list {
            let list_tmp = get_data_preg_list(regexp_str, &item);
            if list_tmp.len() != 2 {
                println!("Str_To_Json_node length_err 145");
                continue;
            }
            let mut key = list_tmp[0].clone();
            let mut value = list_tmp[1].clone();
            key = key.replace("\"", "");
            value = value.replace("\"", "");
            result.insert(key, value);
        }
        result
    }

    /// 手动 JSON 数据压缩 高精度 (List_to_Json)
    /// Converts a list of strings into a JSON array string
    pub fn list_to_json(list: &[String]) -> String {
        if list.is_empty() {
            return String::new();
        }
        let mut str = String::from("[");
        for (i, item) in list.iter().enumerate() {
            str.push_str(item);
            if i < list.len() - 1 {
                str.push(',');
            }
        }
        str.push(']');
        str
    }
}