use json::JsonValue;
use std::collections::HashSet;
use md5::compute as md5Hasher;

pub fn hash_md5(v:&Vec<u8>)->String{
    format!("{:?}",md5Hasher(v))
}

pub fn init(v1:&JsonValue,v2:&JsonValue)->bool{

    let c_type = get_json_type(v1);
    let n_type = get_json_type(v2);
    if c_type != n_type{return false;}
    let data_type = c_type;

    if data_type == "object"{
        if v1.len() != v2.len(){
            return false;
        }
        for (key,value) in v1.entries(){
            if !v2.has_key(key){return false;}
            if !init(value, &v2[key]){return false;}
        }
    } else if data_type == "array"{
        if v1.len() != v2.len(){
            return false;
        }
        let mut v1_hashes = HashSet::new();
        for item in v1.members(){
            v1_hashes.insert(get_safe_hash(item));
        }
        for item in v2.members(){
            let v2_hash = get_safe_hash(item);
            if !v1_hashes.contains(&v2_hash){
                return false;
            }
        }
    } else if data_type == "string"{
        if v1 != v2{return false;}
    } else if data_type == "number"{
        if v1 != v2{return false;}
    } else if data_type == "bool"{
        if v1 == true && v2 == true{
            return true;
        } else if v1 == false && v2 == false{
            return true;
        } else {
            return false;
        }
    } else if data_type == "null"{
        if v1 != v2{return false;}
    } else {
        if v1 != v2{return false;}
    }

    return true;

}

pub fn get_safe_hash(v:&JsonValue)->String{

    let data_type = get_json_type(v);

    if data_type == "object"{
        let mut keys = vec![];
        for (key,_) in v.entries(){
            keys.push(key);
        }
        keys.sort();
        let mut build = String::new();
        for key in keys{
            let value_hash = get_safe_hash(&v[key]);
            build.push_str(&format!("{}:{}",key,value_hash));
        }
        return hash_md5(&build.as_bytes().to_vec());
    }

    if data_type == "string"{
        let val = v.as_str().unwrap();
        return hash_md5(&val.as_bytes().to_vec());
    }

    if data_type == "number"{
        let val = v.as_f64().unwrap();
        return hash_md5(&val.to_be_bytes().to_vec());
    }

    if data_type == "bool"{
        let val = v.as_bool().unwrap();
        if val == true{
            return hash_md5(&"true".as_bytes().to_vec());
        } else {
            return hash_md5(&"false".as_bytes().to_vec());
        }
    }

    if data_type == "array"{
        let mut hashes = vec![];
        for item in v.members(){
            hashes.push(get_safe_hash(item));
        }
        hashes.sort();
        let mut build = String::new();
        for item in hashes{
            build.push_str(&format!("_{}",item));
        }
        return hash_md5(&build.as_bytes().to_vec());
    }

    if data_type == "null"{
        return hash_md5(&"null".as_bytes().to_vec());
    }

    return hash_md5(&"none".as_bytes().to_vec());

}

fn get_json_type(data:&JsonValue)->&'static str{
    let hold;
    if data.is_object(){hold = "object";} else
    if data.is_array(){hold = "array";} else
    if data.is_string(){hold = "string";} else
    if data.is_number(){hold = "number";} else
    if data.is_boolean(){hold = "bool";} else
    if data.is_null(){hold = "null";} else
    {hold = "none";}
    return hold;
}