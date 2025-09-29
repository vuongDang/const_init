use json::JsonValue;
use std::path::Path;

/// This function is used in build scripts to generate a rust file from a json file.
/// The output file contains all the variables from input file as rust constants.
///
/// # Panics
///
/// Panics if does not find the input json file, can't parse it, or can't write the output rust file.
pub fn generate_constants_from_json<P: AsRef<Path>>(input_json_file: P, output_rust_file: P) {
    let contents = std::fs::read_to_string(&input_json_file).expect("Failed to read input file");
    let json = json::parse(&contents).expect("Failed to deserialize input json file");

    // Produce the content of the output rust file containing constants
    let mut generated_content = String::new();
    generated_content.push_str("#![allow(dead_code)]\n");
    generated_content.push_str("// Generated file, don't modify it\n");
    generated_content.push_str(&format!(
        r#"// This file is built at compile-time and contains the variable from "{}""#,
        input_json_file.as_ref().to_string_lossy()
    ));
    generated_content.push_str("\n\n");
    json_to_constants(&mut generated_content, &json, 0, None);

    // Generate the output file
    std::fs::write(output_rust_file, generated_content)
        .expect("Failed to generated Rust file for settings");
}

const INDENT: &str = "\t";
// Turn a json object to rust constants
fn json_to_constants(
    content: &mut String,
    json: &JsonValue,
    recursion_depth: usize,
    field_name: Option<String>,
) {
    let spacing = INDENT.repeat(recursion_depth);
    let generated = match json {
        JsonValue::Object(object) => {
            let mut res = String::new();
            if let Some(ref name) = field_name {
                // If this is not the initial object of the json file
                res.push_str(&format!("{}pub mod {} {{\n", spacing, name));
            }
            for (name, value) in object.iter() {
                let mut depth = recursion_depth;
                if field_name.is_some() {
                    // If this is not the initial object of the json file
                    depth += 1
                }
                json_to_constants(&mut res, value, depth, Some(name.to_owned()));
            }

            if field_name.is_some() {
                // If this is not the initial object of the json file
                res.push_str(&format!("{}}}\n", spacing));
            }
            res
        }
        JsonValue::Short(_) | JsonValue::String(_) => {
            let name = field_name.expect("JSON file ill formatted").to_uppercase();
            let var_type = json_to_rust_type(json);
            format!(
                r#"{spacing}pub const {name}: {var_type} = "{json}";{}"#,
                "\n"
            )
        }
        _ => {
            let name = field_name.expect("JSON file ill formatted").to_uppercase();
            let var_type = json_to_rust_type(json);
            format!("{spacing}pub const {name}: {var_type} = {json};\n")
        }
    };
    content.push_str(&generated);
}

fn json_to_rust_type(json: &JsonValue) -> String {
    match json {
        JsonValue::Null => unimplemented!("null values are not handled"),
        JsonValue::Short(_) | JsonValue::String(_) => "&str".to_string(),
        JsonValue::Number(v) => {
            if v.is_nan() {
                panic!("Nan value in input json file");
            }
            match v.as_parts() {
                (_, _, exponent) if exponent < 0 => "f64".to_string(),
                _ => "isize".to_string(),
            }
        }
        JsonValue::Boolean(_) => "bool".to_string(),
        JsonValue::Array(json_values) => {
            let len = json_values.len();
            if len == 0 {
                return "[isize; 0]".to_string();
            }
            let mut types = json_values.iter().map(|v| json_to_rust_type(v));
            let first_type = types.next().unwrap();
            let all_types_equal = types.all(|json_type| json_type == first_type);
            if !all_types_equal {
                panic!("JSON arrays with different types are not supported")
            }
            format!("[{first_type}; {len}]")
        }
        JsonValue::Object(_) => unreachable!("Type conversion of json object should not be called"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_json_basic_types() {
        let parsed = json::parse(
            r#"
{
    "a": true,
    "b": 234,
    "c": "azer",
    "d": "aaaaa",
    "e": 3.14,
    "f": -35,
    "g": [1, 2, 3, 4],
    "h": ["abc", "def", "hij"]
}
"#,
        )
        .unwrap();

        let mut generated = String::new();
        json_to_constants(&mut generated, &parsed, 0, None);
        // println!("{generated}");
        let generated = generated.trim();

        let expected = r#"
pub const A: bool = true;
pub const B: isize = 234;
pub const C: &str = "azer";
pub const D: &str = "aaaaa";
pub const E: f64 = 3.14;
pub const F: isize = -35;
pub const G: [isize; 4] = [1,2,3,4];
pub const H: [&str; 3] = ["abc","def","hij"];
            "#
        .trim();
        assert_eq!(generated, expected)
    }

    #[test]
    fn test_json_complex_types() {
        let parsed = json::parse(
            r#"
{
    "a": {
        "d": false,
        "e": 65,
        "f": "a string",
        "g": [
            "abc",
            "foo",
            "bar"
        ],
        "h": {
            "i": true,
            "j": {
                "k": [1.5,2.15,-3.45]
            }
        }
    }
}
"#,
        )
        .unwrap();

        let mut generated = String::new();
        json_to_constants(&mut generated, &parsed, 0, None);
        println!("{}", generated);
        let generated: String = generated.split_whitespace().collect();

        let expected: String = r#"
pub mod a {
        pub const D: bool = false;
        pub const E: isize = 65;
        pub const F: &str = "a string";
        pub const G: [&str; 3] = ["abc","foo","bar"];
        pub mod h {
                pub const I: bool = true;
                pub mod j {
                        pub const K: [f64; 3] = [1.5,2.15,-3.45];
                }
        }
}
"#
        .split_whitespace()
        .collect();
        assert_eq!(generated, expected)
    }

    #[test]
    #[should_panic]
    fn json_null_should_panic() {
        let parsed = json::parse(
            r#"
            {
                "a": 1,
                "b": null
            }
            "#,
        )
        .unwrap();

        let mut generated = String::new();
        json_to_constants(&mut generated, &parsed, 0, None);
    }

    #[test]
    #[should_panic]
    fn json_multi_types_array_should_panic() {
        let parsed = json::parse(
            r#"
                {
                    "a": 1,
                    "b": [1, "abc", true]
                }
                "#,
        )
        .unwrap();

        let mut generated = String::new();
        json_to_constants(&mut generated, &parsed, 0, None);
    }

    #[test]
    #[should_panic]
    #[allow(non_snake_case)]
    fn json_Nan_should_panic() {
        let parsed = json::parse(
            r#"
                {
                    "a": 1,
                    "b": NaN
                    }
                "#,
        )
        .unwrap();

        let mut generated = String::new();
        json_to_constants(&mut generated, &parsed, 0, None);
    }
}
