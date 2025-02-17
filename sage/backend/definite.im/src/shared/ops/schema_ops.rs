use std::{any::type_name, fmt::Debug};

use log::{debug, error};
use schemars::{schema_for, JsonSchema};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

#[derive(Deserialize, Debug, Serialize)]
pub struct OpenAiSchema {
    pub name: String,
    pub schema: Value,
}

/// schemars has issues with Option<> types
pub fn to_openai_schema<T>() -> Option<Value>
where
    T: JsonSchema + Serialize + Debug,
{
    let schema = schema_for!(T);
    match serde_json::to_string_pretty(&schema) {
        Ok(pretty_json) => {
            debug!("Schema: {}", pretty_json);
            let mut schema: Value = serde_json::from_str(&pretty_json).unwrap();
            if let Some(obj) = schema.as_object_mut() {
                obj.remove("$schema");
                obj.remove("title");
            }

            let full_type_name = type_name::<T>();
            let schema_name = full_type_name.split("::").last().unwrap_or(full_type_name);

            Some(json!(OpenAiSchema {
                name: schema_name.to_string(),
                schema,
            }))
        }
        Err(e) => {
            error!("Error converting schema to JSON: {}", e);
            None
        }
    }
}

pub fn to_schema<T>() -> Option<String>
where
    T: JsonSchema + Serialize + Debug,
{
    let schema = schema_for!(T);
    match serde_json::to_string_pretty(&schema) {
        Ok(pretty_json) => {
            debug!("Schema: {}", pretty_json);
            Some(pretty_json)
        }
        Err(e) => {
            error!("Error converting schema to JSON: {}", e);
            None
        }
    }
}
