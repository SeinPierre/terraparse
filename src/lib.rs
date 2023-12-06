use serde::{Serialize, Deserialize};
use serde_json::from_reader;
use std::{fs::File, path::Path};
use serde_json;
use serde_json::Value;


#[derive(Serialize,Deserialize,Debug)]
pub struct PlannedValues {
    format_version : String,
    terraform_version : String,
    planned_values : String
}
#[derive(Serialize,Deserialize,Debug)]
pub struct Plan {
    format_version : String,
    terraform_version : String,
    planned_values : String
}

pub fn read(file_path: String) -> Plan {
    let json_file_path = Path::new(&file_path);
    let json_file = File::open(json_file_path).expect("file not found");
    let deserialized_file: Plan =
        from_reader(json_file).expect("error while reading json");
    deserialized_file
}

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn ut_read_simple_plan(){
        let result = read("examples/sample01/sample01.json".to_string());
        assert_eq!(result.format_version, "1.1");
    }

    #[test]
    fn ut_read_sample01_no_struct(){
        let json_file_path = Path::new("examples/sample01/sample01a.json");
    let json_file = File::open(json_file_path).expect("file not found");
    let deserialized_file : Value =
        from_reader(json_file).expect("error while reading json");
    // println!("deserialized_file: {:?}", deserialized_file);
    let obj = deserialized_file.as_object().unwrap();
    let resource_changes = obj.get("resource_changes").unwrap();
    // println!("{:?}",resource_changes);
    match resource_changes {
        Value::Array(v) => print!("{:?}",v),
        _ => println!("Toto")
    }
    // for (key,value) in obj.iter() {
    //     println!("{} : {}", key, match value {
    //         Value::Bool(v) => format!("{} (Bool)",v),
    //         Value::String(v) => format!("{} (String)",v),
    //         Value::Number(v) => format!("{} (Number)",v),
    //         Value::Object(v) => format!("{:?} (Object)",v),
    //         _ => format!("Other")
            
    //     })
    // }
    assert_eq!(obj.get("format_version").unwrap(), "1.1");

    }
}
