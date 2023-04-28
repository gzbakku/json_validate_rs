
use std::convert::From;
use json::JsonValue;
use std::collections::HashSet;

#[derive(Debug)]
pub enum FormatError{
    InvalidDataType,InvalidFormat,
    InvalidMin,InvalidMax,
    InvalidSchemaOnData,InvalidSchema,
    InvalidOptions,OptionsNotAllowed,
    isNotObject
}

#[derive(Debug)]
pub enum DataError{
    Min,Max,NotFound,InvalidDataType,isNotObject,
    ExternalDataNotAllowed,
    InvalidMax,InvalidMaxNum,InvalidMaxDataType,
    InvalidMin,InvalidMinNum,InvalidMinDataType,
    OutOfOptions,DataMaxReached,UnmatchedKey,
    UnMatchedSize,UnSupportedData
}

#[derive(Debug)]
pub enum Errors{
    format
}

#[derive(Debug)]
pub enum Error{
    Key(String,RuleError),Format(FormatError),Data(DataError),RuleError(RuleError)
}

#[derive(Debug)]
pub enum RuleError{
    format(FormatError),data(DataError),None,sub(Box<Error>)
}

impl From<RuleError> for Error {
    fn from(v: RuleError) -> Error{
        Error::RuleError(v)
    }
}

pub fn run(
    format:&JsonValue,
    data:&JsonValue,
    schema_type:&str,
    max_size:u32
)->Result<(),Error>{

    if !format.is_object(){
        return Err(RuleError::format(FormatError::isNotObject).into());
    }
    if !data.is_object(){
        return Err(RuleError::data(DataError::isNotObject).into());
    }
    let is_dynamic;
    if schema_type == "dynamic"{
        if data.len() > max_size as usize{
            return Err(RuleError::data(DataError::DataMaxReached).into());
        }
        is_dynamic = true;
    } else {
        is_dynamic = false;
        if data.len() != format.len(){
            return Err(RuleError::data(DataError::UnMatchedSize).into());
        }
    }

    for (key,rules) in format.entries(){

        if !data.has_key(key){
            if rules.has_key("elective"){
                if rules["elective"].is_boolean(){
                    let elective = rules["elective"].as_bool().unwrap();
                    if !elective{
                        return Err(RuleError::data(DataError::NotFound).into());
                    }
                } else {
                    return Err(RuleError::data(DataError::NotFound).into());
                }
            } else {
                return Err(RuleError::data(DataError::NotFound).into());
            }
        }

        if data.has_key(key){
            let value = &data[key];
            match check_field(key, value, rules, &is_dynamic){
                Ok(_)=>{},
                Err(e)=>{
                    return Err(Error::Key(key.to_string(), e));
                }
            }
        }

    }

    return Ok(());

}

fn check_field(key:&str,value:&JsonValue,rules:&JsonValue,is_dynamic:&bool)->Result<(),RuleError>{

    if !rules.is_object(){
        return Err(RuleError::format(FormatError::InvalidFormat).into());
    }

    match check_rule_data_type(&rules["type"],&value){
        Ok(_)=>{},
        Err(e)=>{
            return Err(e);
        }
    }

    let data_type = rules["type"].as_str().unwrap();

    if rules.has_key("min"){
        match check_min(&data_type,&value,&rules["min"]){
            Ok(_)=>{},
            Err(e)=>{
                return Err(e);
            }
        }
    }
    
    if rules.has_key("max"){
        match check_max(&data_type,&value,&rules["max"]){
            Ok(_)=>{},
            Err(e)=>{
                return Err(e);
            }
        }
    }

    if rules.has_key("options"){
        match check_options(&data_type,&value,&rules["options"]){
            Ok(_)=>{},
            Err(e)=>{
                return Err(e);
            }
        }
    }

    if rules.has_key("validate"){
        match check_validate(&data_type,&value,&rules["validate"]){
            Ok(_)=>{},
            Err(e)=>{
                return Err(e);
            }
        }
    }

    return Ok(());

}

fn check_options(data_type:&str,value:&JsonValue,rule:&JsonValue)->Result<(),RuleError>{

    if
        !value.is_string() && 
        !value.is_number() &&
        !value.is_array()
    {
        return Err(RuleError::format(FormatError::OptionsNotAllowed));
    }

    if !rule.is_array(){
        return Err(RuleError::format(FormatError::InvalidOptions));
    }

    let mut set = HashSet::new();
    for item in rule.members(){
        if item.is_string(){
            set.insert(item.as_str().unwrap());
        }
    }

    if value.is_array(){
        for item in value.members(){
            let v:Option<String>;
            if item.is_string(){
                let s = item.as_str().unwrap();
                v = Some(s.to_string());
            } else if item.is_number(){
                let i = item.as_i64().unwrap();
                let j = i.clone().to_string();
                v = Some(j.to_string());
            } else {
                v = None; 
            }
            match v{
                Some(c)=>{
                    if !set.contains(c.as_str()){
                        return Err(RuleError::data(DataError::OutOfOptions));
                    }
                },
                None=>{}
            }
        }
    }

    if value.is_string(){
        let v = value.as_str().unwrap();
        if !set.contains(v){
            return Err(RuleError::data(DataError::OutOfOptions));
        }
    }

    if value.is_number(){
        let v = value.as_i64().unwrap();
        let n = &v.to_string();
        let p = n.as_str();
        if !set.contains(p){
            return Err(RuleError::data(DataError::OutOfOptions));
        }
    }

    return Ok(());

    // return Err(RuleError::None);

}

fn check_validate(data_type:&str,value:&JsonValue,rule:&JsonValue)->Result<(),RuleError>{

    if !value.is_object(){
        return Err(RuleError::format(FormatError::InvalidSchemaOnData));
    }

    if !rule["schema"].is_object(){
        return Ok(());
    }

    let schema_type;
    if rule["dynamic"].is_boolean(){
        let v = rule["dynamic"].as_bool().unwrap();
        if v{
            schema_type = "dynamic";
        } else {
            schema_type = "static";
        }
    } else {
        schema_type = "static";
    }

    let max_size;
    if rule["maxSize"].is_string(){
        max_size = rule["maxSize"].as_u32().unwrap();
    } else {
        max_size = 0;
    }

    let schema = &rule["schema"];

    match run(schema,value,schema_type,max_size){
        Ok(_)=>{
            return Ok(());
        },
        Err(e)=>{
            return Err(RuleError::sub(Box::new(e)));
        }
    }

}

fn check_max(data_type:&str,value:&JsonValue,rule:&JsonValue)->Result<(),RuleError>{

    let max;
    match rule.as_u64(){
        Some(v)=>{max = v;},
        None=>{return Err(RuleError::data(DataError::InvalidMax));}
    }

    if data_type == "string"{
        let v = value.as_str().unwrap();
        if (v.len() as u64) > max{return Err(RuleError::data(DataError::Max));}
    } else if data_type == "number"{
        match value.as_u64(){
            Some(v)=>{if v > max{return Err(RuleError::data(DataError::Max));}},
            None=>{return Err(RuleError::data(DataError::InvalidMaxNum));}
        }
    } else if data_type == "array"{
        if (value.len() as u64) > max{return Err(RuleError::data(DataError::Max));}
    } else if data_type == "object"{
        if (value.len() as u64) > max{return Err(RuleError::data(DataError::Max));}
    } else {
        return Err(RuleError::data(DataError::InvalidMaxDataType));
    }

    return Ok(());

}

fn check_min(data_type:&str,value:&JsonValue,rule:&JsonValue)->Result<(),RuleError>{

    let min;
    match rule.as_u64(){
        Some(v)=>{min = v;},
        None=>{return Err(RuleError::data(DataError::InvalidMin));}
    }

    if data_type == "string"{
        let v = value.as_str().unwrap();
        if (v.len() as u64) < min{return Err(RuleError::data(DataError::Min));}
    } else if data_type == "number"{
        match value.as_u64(){
            Some(v)=>{if v < min{return Err(RuleError::data(DataError::Min));}},
            None=>{return Err(RuleError::data(DataError::InvalidMinNum));}
        }
    } else if data_type == "array"{
        if (value.len() as u64) < min{return Err(RuleError::data(DataError::Min));}
    } else if data_type == "object"{
        if (value.len() as u64) < min{return Err(RuleError::data(DataError::Min));}
    } else {
        return Err(RuleError::data(DataError::InvalidMinDataType));
    }

    return Ok(());

}

fn check_rule_data_type(rule:&JsonValue,value:&JsonValue)->Result<(),RuleError>{

    if !rule.is_string(){
        return Err(RuleError::format(FormatError::InvalidDataType));
    }

    let rule_data_type = rule.as_str().unwrap();

    if
        rule_data_type != "bool" &&
        rule_data_type != "string" &&
        rule_data_type != "number" &&
        rule_data_type != "object" &&
        rule_data_type != "array"
    {
        return Err(RuleError::format(FormatError::InvalidDataType));
    }

    let value_data_type;
    if value.is_string(){value_data_type = "string";} else
    if value.is_number(){value_data_type = "number";} else 
    if value.is_object(){value_data_type = "object";} else 
    if value.is_array(){value_data_type = "array";} else 
    if value.is_boolean() {value_data_type = "bool";} else {
        return Err(RuleError::data(DataError::UnSupportedData));
    }

    if rule_data_type != value_data_type{
        return Err(RuleError::data(DataError::InvalidDataType));
    }

    return Ok(());

}