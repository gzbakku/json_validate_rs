
use std::convert::From;
use json::JsonValue;
use std::collections::HashSet;
use regex::Regex;

#[derive(Debug)]
pub enum FormatError{
    InvalidDataType,InvalidFormat,
    InvalidMin,InvalidMax,
    InvalidSchemaOnData,InvalidSchema,
    InvalidOptions,OptionsNotAllowed,
    IsNotObject,IsNotStringArray,
    ChildrenTypeMissing,InvalidChildrenType,
    InvalidArrayUniqueKeyType
}

#[derive(Debug)]
pub enum DataError{
    Min,Max,NotFound(String),InvalidDataType,IsNotObject,
    ExternalDataNotAllowed,InvalidNum,
    InvalidMax,InvalidMaxNum,InvalidMaxDataType,
    InvalidMin,InvalidMinNum,InvalidMinDataType,
    OutOfOptions,DataMaxReached,UnmatchedKey(String),
    UnMatchedSize,UnSupportedData,InvalidEmail,
    MissingRequiredOptionField(String),
    MissingRequiredField(String),
    ArrayUniqueKeyNotFound(String),ArrayUniqueKeyNotStringType(String),
    ArrayUniqueKeyDuplicate(String,String),DuplicateArrayString(String),
    ArrayUniqueValueNotString,InvalidChildDataType(String,String)
}

pub fn validate_email(email:&str)->Result<(),RuleError>{

    let re = Regex::new(
        r"([\w\d_\-+#$%&*\/^]+)@([\w\d_-]+)[\.]{1,1}([\w\d]+)"
    ).unwrap();

    if !re.is_match(email){
        return Err(RuleError::Data(DataError::InvalidEmail));
    }

    Ok(())

}

#[derive(Debug)]
pub enum Error{
    Key(String,RuleError),Format(FormatError),Data(DataError),RuleError(RuleError)
}

#[derive(Debug)]
pub enum RuleError{
    Format(FormatError),Data(DataError),None,Sub(Box<Error>)
}

impl From<RuleError> for Error {
    fn from(v: RuleError) -> Error{
        Error::RuleError(v)
    }
}

#[doc = include_str!("../example.md")]
///schema type can be
///dynamic validation allows undefined fields
///static validation only allows defined fields
pub fn run(
    format:&JsonValue,
    data:&JsonValue,
    schema_type:&str,
    max_size:u32
)->Result<(),Error>{

    if !format.is_object(){
        return Err(RuleError::Format(FormatError::IsNotObject).into());
    }
    if !data.is_object(){
        return Err(RuleError::Data(DataError::IsNotObject).into());
    }
    let is_dynamic;
    if schema_type == "dynamic"{
        if data.len() > max_size as usize{
            return Err(RuleError::Data(DataError::DataMaxReached).into());
        }
        is_dynamic = true;
    } else {
        is_dynamic = false;
        // if data.len() != format.len(){
        //     return Err(RuleError::data(DataError::UnMatchedSize).into());
        // }
    }

    for (key,rules) in format.entries(){

        if !data.has_key(key){
            if rules.has_key("elective"){
                if rules["elective"].is_boolean(){
                    let elective = rules["elective"].as_bool().unwrap();
                    if !elective{
                        return Err(RuleError::Data(DataError::NotFound(key.to_string())).into());
                    }
                } else {
                    return Err(RuleError::Data(DataError::NotFound(key.to_string())).into());
                }
            } else {
                return Err(RuleError::Data(DataError::NotFound(key.to_string())).into());
            }
        }

        if data.has_key(key){
            let value = &data[key];
            match check_field(
                key, value, rules, &is_dynamic, &data
            ){
                Ok(_)=>{},
                Err(e)=>{
                    return Err(Error::Key(key.to_string(), e));
                }
            }
        }

    }

    if !is_dynamic{
        for (key,_) in data.entries(){
            if !format.has_key(key){
                return Err(RuleError::Data(DataError::UnmatchedKey(key.to_string())).into());
            }
        }
    }

    return Ok(());

}

fn check_field(
    _key:&str,value:&JsonValue,
    rules:&JsonValue,_is_dynamic:&bool,
    all_values:&JsonValue
)->Result<(),RuleError>{

    if !rules.is_object(){
        return Err(RuleError::Format(FormatError::InvalidFormat).into());
    }

    match check_rule_data_type(&rules["type"],&value,rules){
        Ok(_)=>{},
        Err(e)=>{
            return Err(e);
        }
    }

    let email_type = "string";
    let mut data_type = rules["type"].as_str().unwrap();

    if data_type == "email"{
        let email = value.as_str().unwrap();
        match validate_email(email){
            Ok(_)=>{},
            Err(e)=>{
                return Err(e);
            }
        }
        data_type = &email_type;
    }

    if data_type == "any"{
        if value.is_null(){
            return Err(RuleError::Data(DataError::NotFound(_key.to_string())));
        }
    }

    if 
        data_type == "string" && 
        rules.has_key("options") &&
        rules.has_key("option_required_fields")
    {
        match check_option_required_fields(&value,&rules["option_required_fields"],&all_values){
            Ok(_)=>{},
            Err(e)=>{
                return Err(e);
            }
        }
    }

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

    if rules.has_key("required_fields"){
        match check_required_fields(&rules["required_fields"],&all_values){
            Ok(_)=>{},
            Err(e)=>{
                return Err(e);
            }
        }
    }

    return Ok(());

}

fn check_required_fields(rule:&JsonValue,all_values:&JsonValue)->Result<(),RuleError>{

    if !rule.is_array(){
        return Err(RuleError::Format(FormatError::IsNotObject));
    }
    if !all_values.is_object(){
        return Err(RuleError::Format(FormatError::IsNotObject));
    }

    for item in rule.members(){
        if !item.is_string(){
            return Err(RuleError::Format(FormatError::IsNotStringArray));
        }
        let value = item.as_str().unwrap();
        if !all_values.has_key(value){
            return Err(RuleError::Data(DataError::MissingRequiredField(value.to_string()))); 
        }
    }

    return Ok(());

}

fn check_option_required_fields(value:&JsonValue,rule:&JsonValue,all_values:&JsonValue)->Result<(),RuleError>{

    if !rule.is_object(){
        return Err(RuleError::Format(FormatError::IsNotObject));
    }
    if !all_values.is_object(){
        return Err(RuleError::Format(FormatError::IsNotObject));
    }
    if !value.is_string(){
        return Err(RuleError::Data(DataError::InvalidDataType));
    }
    let value = value.as_str().unwrap();
    if !rule.has_key(value){
        return Ok(());
    }

    if rule[value].is_array(){
        for item in rule[value].members(){
            if !item.is_string(){
                return Err(RuleError::Format(FormatError::IsNotStringArray));
            }
            let value = item.as_str().unwrap();
            if !all_values.has_key(value){
                return Err(RuleError::Data(DataError::MissingRequiredField(value.to_string()))); 
            }
        }
    } else if rule[value].is_string(){
        let field_name = rule[value].as_str().unwrap();
        if !all_values.has_key(field_name){
            return Err(RuleError::Data(DataError::MissingRequiredField(field_name.to_string()))); 
        }
    } else {
        return Err(RuleError::Data(DataError::InvalidDataType));
    }

    return Ok(());

}

fn check_options(_data_type:&str,value:&JsonValue,rule:&JsonValue)->Result<(),RuleError>{

    if
        !value.is_string() && 
        !value.is_number() &&
        !value.is_array()
    {
        return Err(RuleError::Format(FormatError::OptionsNotAllowed));
    }

    if !rule.is_array(){
        return Err(RuleError::Format(FormatError::InvalidOptions));
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
                        return Err(RuleError::Data(DataError::OutOfOptions));
                    }
                },
                None=>{}
            }
        }
    }

    if value.is_string(){
        let v = value.as_str().unwrap();
        if !set.contains(v){
            return Err(RuleError::Data(DataError::OutOfOptions));
        }
    }

    if value.is_number(){
        let v = value.as_i64().unwrap();
        let n = &v.to_string();
        let p = n.as_str();
        if !set.contains(p){
            return Err(RuleError::Data(DataError::OutOfOptions));
        }
    }

    return Ok(());

    // return Err(RuleError::None);

}

fn check_validate(data_type:&str,value:&JsonValue,rule:&JsonValue)->Result<(),RuleError>{

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

    let valid_child_type = ["bool","string","number","array","object"];

    if data_type == "array"{

        if !value.is_array(){
            return Err(RuleError::Data(DataError::UnSupportedData));
        }

        if !rule["children_type"].is_string(){
            return Err(RuleError::Format(FormatError::ChildrenTypeMissing));
        }

        let children_type = rule["children_type"].as_str().unwrap();
        let child_data_type = children_type;
        if !valid_child_type.contains(&child_data_type){
            return Err(RuleError::Format(FormatError::InvalidChildrenType));
        }

        for item in value.members(){
            let item_data_type = get_json_value_data_type(item)?;
            if item_data_type != child_data_type{
                return Err(RuleError::Data(DataError::InvalidDataType));
            }
            if child_data_type == "object" && rule["children_schema"].is_object(){
                let schema = &rule["children_schema"];
                match run(schema,item,schema_type,max_size){
                    Ok(_)=>{},
                    Err(e)=>{
                        return Err(RuleError::Sub(Box::new(e)));
                    }
                }
            }
        }

        if children_type == "object" && rule["unique_keys"].is_array(){
            for item in rule["unique_keys"].members(){
                if !item.is_string(){
                    return Err(RuleError::Format(
                        FormatError::InvalidArrayUniqueKeyType
                    ));
                }
                let u_key = item.as_str().unwrap().to_string();
                let mut map = HashSet::new();
                for item in value.members(){
                    if !item.has_key(&u_key){
                        return Err(RuleError::Data(
                            DataError::ArrayUniqueKeyNotFound(u_key)
                        ));
                    }
                    if !item[&u_key].is_string(){
                        return Err(RuleError::Data(
                            DataError::ArrayUniqueKeyNotStringType(u_key)
                        ));
                    }
                    let u_key_val = item[&u_key].as_str().unwrap();
                    // println!("u_key_val : {u_key_val}");
                    if map.contains(u_key_val){
                        return Err(RuleError::Data(
                            DataError::ArrayUniqueKeyDuplicate(
                                u_key,
                                u_key_val.to_string()
                            )
                        ));
                    } else {
                        map.insert(u_key_val);
                    }
                } 
            }
        }

        if 
            children_type == "string" && 
            rule["unique"].is_boolean() &&
            rule["unique"].as_bool().unwrap()
        {
            let mut map = HashSet::new();
            for item in value.members(){
                if !item.is_string(){
                    return Err(RuleError::Data(
                        DataError::ArrayUniqueValueNotString
                    ));
                }
                let s = item.as_str().unwrap().to_string();
                if map.contains(&s){
                    return Err(RuleError::Data(
                        DataError::DuplicateArrayString(s)
                    ));
                } else {
                    map.insert(s);
                }
            }
        }

        return Ok(());

    }

    if data_type == "object"{

        if !value.is_object(){
            return Err(RuleError::Format(FormatError::InvalidSchemaOnData));
        }
    
        if rule["schema"].is_object(){
            let schema = &rule["schema"];
            match run(schema,value,schema_type,max_size){
                Ok(_)=>{
                    return Ok(());
                },
                Err(e)=>{
                    return Err(RuleError::Sub(Box::new(e)));
                }
            }
        }
    
        if 
            rule["children_type"].is_string()
        {
            let children_type = rule["children_type"].as_str().unwrap();
            // let child_data_type = children_type;
            if !valid_child_type.contains(&children_type){
                return Err(RuleError::Format(FormatError::InvalidChildrenType));
            }
            for (_,value) in value.entries(){
                let item_data_type = get_json_value_data_type(value)?;
                if item_data_type != children_type{
                    println!("children_type : {children_type}");
                    println!("item_data_type : {item_data_type}");
                    return Err(RuleError::Data(DataError::InvalidChildDataType(
                        children_type.to_string(),
                        item_data_type.to_string(),
                    )));
                }
            }
        }

        if 
            rule["children_schema"].is_object()
        {
            let schema = &rule["children_schema"];
            for (_,t_val) in value.entries(){
                match run(schema,t_val,schema_type,max_size){
                    Ok(_)=>{
                        return Ok(());
                    },
                    Err(e)=>{
                        return Err(RuleError::Sub(Box::new(e)));
                    }
                }
            }
        }

        if 
            rule["children_type"] == "array" && 
            rule["array_validate"].is_object()
        {
            let array_validate = &rule["array_validate"];
            if array_validate["array_child_type"].is_string(){
                let array_child_type = array_validate["array_child_type"].as_str().unwrap();
                if !valid_child_type.contains(&array_child_type){
                    return Err(RuleError::Format(FormatError::InvalidChildrenType));
                }
                for (_,t_val) in value.entries(){
                    for array_value in t_val.members(){
                        let array_value_type = get_json_value_data_type(array_value)?;
                        if array_value_type != array_child_type{
                            return Err(RuleError::Data(DataError::InvalidDataType));
                        }
                        if 
                            array_child_type == "object" &&
                            array_validate["validate"].is_object()
                        {
                            validate_array_children_schema(
                                &array_validate["validate"],
                                array_value
                            )?;
                        }
                    }
                }
            }
        }
        
        return Ok(());

    }

    // Err()

    Ok(())

}

fn validate_array_children_schema(
    rule:&JsonValue,
    value:&JsonValue
)->Result<(),RuleError>{

    if !rule["schema"].is_object(){
        return Err(RuleError::Format(FormatError::InvalidSchema)); 
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
        Ok(_)=>{},
        Err(e)=>{
            return Err(RuleError::Sub(Box::new(e)));
        }
    }

    Ok(())

}

fn check_max(data_type:&str,value:&JsonValue,rule:&JsonValue)->Result<(),RuleError>{

    if data_type == "number"{
        let r;
        if value.as_u64().is_some() && rule.as_u64().is_some(){
            r = value.as_u64().unwrap() > rule.as_u64().unwrap();
        } else
        if value.as_i64().is_some() && rule.as_i64().is_some(){
            r = value.as_i64().unwrap() > rule.as_i64().unwrap();
        } else
        if value.as_f64().is_some() && rule.as_f64().is_some(){
            r = value.as_f64().unwrap() > rule.as_f64().unwrap();
        } else {
            return Err(RuleError::Data(DataError::InvalidMaxNum));
        }
        if r{return Err(RuleError::Data(DataError::Max));}
        return Ok(());
    }

    if !rule.as_u64().is_some(){
        return Err(RuleError::Data(DataError::InvalidMax));
    }

    let max = rule.as_u64().unwrap();
    if data_type == "string"{
        let v = value.as_str().unwrap();
        if (v.len() as u64) > max{return Err(RuleError::Data(DataError::Max));}
    } else if data_type == "array"{
        if (value.len() as u64) > max{return Err(RuleError::Data(DataError::Max));}
    } else if data_type == "object"{
        if (value.len() as u64) > max{return Err(RuleError::Data(DataError::Max));}
    } else {
        return Err(RuleError::Data(DataError::InvalidMaxDataType));
    }

    return Ok(());

}

fn check_min(data_type:&str,value:&JsonValue,rule:&JsonValue)->Result<(),RuleError>{

    if data_type == "number"{
        let r;
        if value.as_u64().is_some() && rule.as_u64().is_some(){
            r = value.as_u64().unwrap() < rule.as_u64().unwrap();
        } else
        if value.as_i64().is_some() && rule.as_i64().is_some(){
            r = value.as_i64().unwrap() < rule.as_i64().unwrap();
        } else
        if value.as_f64().is_some() && rule.as_f64().is_some(){
            r = value.as_f64().unwrap() < rule.as_f64().unwrap();
        } else {
            return Err(RuleError::Data(DataError::InvalidMinNum));
        }
        if r{return Err(RuleError::Data(DataError::Max));}
        return Ok(());
    }

    if !rule.as_u64().is_some(){
        return Err(RuleError::Data(DataError::InvalidMin));
    }

    let min = rule.as_u64().unwrap();
    if data_type == "string"{
        let v = value.as_str().unwrap();
        if (v.len() as u64) < min{return Err(RuleError::Data(DataError::Min));}
    } else if data_type == "array"{
        if (value.len() as u64) < min{return Err(RuleError::Data(DataError::Min));}
    } else if data_type == "object"{
        if (value.len() as u64) < min{return Err(RuleError::Data(DataError::Min));}
    } else {
        return Err(RuleError::Data(DataError::InvalidMinDataType));
    }

    return Ok(());

}

fn check_rule_data_type(rule:&JsonValue,value:&JsonValue,field:&JsonValue)->Result<(),RuleError>{

    if !rule.is_string(){
        return Err(RuleError::Format(FormatError::InvalidDataType));
    }

    let rule_data_type = rule.as_str().unwrap();

    if
        rule_data_type != "any" &&
        rule_data_type != "bool" &&
        rule_data_type != "string" &&
        rule_data_type != "number" &&
        rule_data_type != "object" &&
        rule_data_type != "array" &&
        rule_data_type != "email"
    {
        return Err(RuleError::Format(FormatError::InvalidDataType));
    }

    if rule_data_type == "any"{
        return Ok(());
    }

    let value_data_type;
    if value.is_string(){value_data_type = "string";} else
    if value.is_number(){value_data_type = "number";} else 
    if value.is_object(){value_data_type = "object";} else 
    if value.is_array(){value_data_type = "array";} else
    if value.is_boolean() {value_data_type = "bool";} else {
        return Err(RuleError::Data(DataError::UnSupportedData));
    }

    if rule_data_type == "email" && value_data_type == "string"{
        return Ok(());
    }

    if rule_data_type != value_data_type{
        return Err(RuleError::Data(DataError::InvalidDataType));
    }

    if rule_data_type == "number" && field["int"].is_string(){
        let int_t = field["int"].as_str().unwrap();
        if 
            int_t == "u8" && value.as_u8().is_none() ||
            int_t == "u16" && value.as_u16().is_none() ||
            int_t == "u32" && value.as_u32().is_none() ||
            int_t == "u64" && value.as_u64().is_none() ||

            int_t == "i8" && value.as_i8().is_none() ||
            int_t == "i16" && value.as_i16().is_none() ||
            int_t == "i32" && value.as_i32().is_none() ||
            int_t == "i64" && value.as_i64().is_none() ||

            int_t == "f64" && value.as_f64().is_none()
        {
            return Err(RuleError::Data(DataError::InvalidNum));
        }
    }

    return Ok(());

}

fn get_json_value_data_type(value:&JsonValue)->Result<&'static str,RuleError>{
    let value_data_type;
    if value.is_string(){value_data_type = "string";} else
    if value.is_number(){value_data_type = "number";} else 
    if value.is_object(){value_data_type = "object";} else 
    if value.is_array(){value_data_type = "array";} else
    if value.is_boolean() {value_data_type = "bool";} else {
        return Err(RuleError::Data(DataError::UnSupportedData));
    }
    Ok(value_data_type)
}