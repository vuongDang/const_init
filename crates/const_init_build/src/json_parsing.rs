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
        JsonValue::Array(array) => {
            let mut res = String::new();
            for (i, value) in array.iter().enumerate() {
                let var_name = format!("{}_{}", field_name.as_ref().unwrap(), i);
                json_to_constants(&mut res, value, recursion_depth, Some(var_name));
            }
            res
        }
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
        _ => {
            let mut res = String::new();
            json_leaf_to_constants(&mut res, json, field_name.unwrap(), recursion_depth);
            res
        }
    };
    content.push_str(&generated);
}

fn json_leaf_to_constants(
    content: &mut String,
    json: &JsonValue,
    name: String,
    recursion_depth: usize,
) {
    let spacing = INDENT.repeat(recursion_depth);
    let name = name.to_uppercase();
    let line = match json {
        JsonValue::Boolean(v) => {
            format!("{}pub const {}: bool = {};\n", spacing, name, v)
        }
        JsonValue::Number(v) => {
            if v.is_nan() {
                panic!("Nan value in input json file");
            }
            let var_type = match v.as_parts() {
                (_, _, exponent) if exponent < 0 => "f64",
                _ => "isize",
            };
            format!("{spacing}pub const {name}: {var_type} = {v};\n")
        }
        JsonValue::Short(v) => format!(r#"{}pub const {}: &str = "{}";{}"#, spacing, name, v, "\n"),
        JsonValue::String(v) => {
            format!(r#"{}pub const {}: &str = "{}";{}"#, spacing, name, v, "\n")
        }

        JsonValue::Null => {
            // We don't have enough information to transform it into a Rust type
            unimplemented!()
        }
        JsonValue::Array(_) => unreachable!(),
        JsonValue::Object(_) => unreachable!(),
    };
    content.push_str(&line);
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
    "f": -35
}
"#,
        )
        .unwrap();

        let mut generated = String::new();
        json_to_constants(&mut generated, &parsed, 0, None);
        let generated = generated.trim();

        let expected = r#"
pub const A: bool = true;
pub const B: isize = 234;
pub const C: &str = "azer";
pub const D: &str = "aaaaa";
pub const E: f64 = 3.14;
pub const F: isize = -35;
            "#
        .trim();
        assert_eq!(generated, expected)
    }

    #[test]
    fn test_json_complex_types() {
        let parsed = json::parse(
            r#"
{
    "a": [
        true,
        234,
        "azer",
        [
            false,
            "toto",
            [
                "foo",
                true,
                "bar"
            ]
        ],
        {
            "a": true,
            "b": 45,
            "c": [
                1,
                2,
                3
            ]
        }
    ],
    "b": {
        "d": false,
        "e": 65,
        "f": "a string",
        "g": [
            "abc",
            false,
            456
        ],
        "e": {
            "m": true,
            "n": {
                "o": [
                1, 2.15, -3
                ]
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
pub const A_0: bool = true;
pub const A_1: isize = 234;
pub const A_2: &str = "azer";
pub const A_3_0: bool = false;
pub const A_3_1: &str = "toto";
pub const A_3_2_0: &str = "foo";
pub const A_3_2_1: bool = true;
pub const A_3_2_2: &str = "bar";
pub mod a_4 {
        pub const A: bool = true;
        pub const B: isize = 45;
        pub const C_0: isize = 1;
        pub const C_1: isize = 2;
        pub const C_2: isize = 3;
}
pub mod b {
        pub const D: bool = false;
        pub mod e {
                pub const M: bool = true;
                pub mod n {
                        pub const O_0: isize = 1;
                        pub const O_1: f64 = 2.15;
                        pub const O_2: isize = -3;
                }
        }
        pub const F: &str = "a string";
        pub const G_0: &str = "abc";
        pub const G_1: bool = false;
        pub const G_2: isize = 456;
}
"#
        .split_whitespace()
        .collect();
        assert_eq!(generated, expected)
    }
}
