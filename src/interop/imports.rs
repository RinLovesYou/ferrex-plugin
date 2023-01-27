use std::error::Error;

use bincode::Encode;

use crate::types::{assembly::UnityAssembly, domain::UnityDomain, class::UnityClass, property::UnityProperty, object::UnityObject, common::ArbitraryData};

#[scotch_guest::host_functions]
extern "C" {
    fn fx_log_str(val: &String);

    fn fx_get_domain() -> UnityDomain;

    fn fx_get_assemblies() -> Vec<UnityAssembly>;
    fn fx_get_assembly(name: &String) -> UnityAssembly;
    fn fx_get_assembly_name(assembly: &UnityAssembly) -> String;

    fn fx_get_class(assembly: &UnityAssembly, namespace: &String, name: &String) -> UnityClass;
    fn fx_get_class_name(class: &UnityClass) -> String;

    fn fx_get_property(class: &UnityClass, name: &String) -> UnityProperty;
    fn fx_get_property_name(prop: &UnityProperty) -> String;
    fn fx_get_property_value(prop: &UnityProperty, object: &UnityObject) -> UnityObject;
    fn fx_set_property_value(prop: &UnityProperty, object: &UnityObject, value: &ArbitraryData);

    fn fx_unbox_i32(obj: &UnityObject) -> i32;
}

pub fn log_str(val: &String) {
    fx_log_str(val);
}

pub fn get_domain() -> Result<UnityDomain, Box<dyn Error>> {
    let domain = fx_get_domain();

    if domain.inner == 0 {
        Err("Failed to get domain".into())
    } else {
        Ok(domain)
    }
}

pub fn get_assemblies() -> Result<Vec<UnityAssembly>, Box<dyn Error>> {
    let assemblies = fx_get_assemblies();

    if assemblies.len() <= 0 {
        Err("Failed to get assemblies".into())
    } else {
        Ok(assemblies)
    }
}

pub fn get_assembly(name: &String) -> Result<UnityAssembly, Box<dyn Error>> {
    let assembly = fx_get_assembly(name);

    if assembly.inner <= 0 {
        Err("Failed to get assembly".into())
    } else {
        Ok(assembly)
    }
}

pub fn get_assembly_name(assembly: &UnityAssembly) -> Result<String, Box<dyn Error>> {
    if assembly.inner <= 0 {
        return Err("assembly is a null pointer!".into());
    }

    let name = fx_get_assembly_name(assembly);

    if name.is_empty() {
        Err("Failed to get Assembly Name".into())
    } else {
        Ok(name)
    }
}

pub fn get_class(assembly: &UnityAssembly, namespace: &str, name: &str) -> Result<UnityClass, Box<dyn Error>> {
    if assembly.inner <= 0 {
        return Err("assembly is a null pointer!".into());
    }

    let class = fx_get_class(assembly, &namespace.to_string(), &name.to_string());

    if class.inner <= 0 {
        Err(format!("Failed to get class {}.{}", namespace, name).into())
    } else {
        Ok(class)
    }
}

pub fn get_class_name(class: &UnityClass) -> Result<String, Box<dyn Error>> {
    if class.inner <= 0 {
        return Err("class is a null pointer!".into());
    }

    let name = fx_get_class_name(class);

    if name.is_empty() {
        Err("Failed to get name".into())
    } else {
        Ok(name)
    }
}

pub fn get_property(class: &UnityClass, name: &str) -> Result<UnityProperty, Box<dyn Error>> {
    if class.inner <= 0 {
        return Err("class is a null pointer!".into());
    }

    let prop = fx_get_property(class, &name.to_string());

    if prop.inner <= 0 {
        Err("Failed to get property".into())
    } else {
        Ok(prop)
    }
}

pub fn get_property_name(property: &UnityProperty) -> Result<String, Box<dyn Error>> {
    if property.inner <= 0 {
        return Err("property is a null pointer!".into());
    }

    let name = fx_get_property_name(property);

    if name.is_empty() {
        Err("Failed to get name".into())
    } else {
        Ok(name)
    }
}

pub fn get_property_value(property: &UnityProperty, object: Option<&UnityObject>) -> Result<UnityObject, Box<dyn Error>> {
    if property.inner <= 0 {
        return Err("property is a null pointer!".into());
    }

    let obj = match object.is_some() {
        true => object.unwrap(),
        false => &UnityObject { inner: 0 }
    };

    let res = fx_get_property_value(property, obj);

    if res.inner <= 0 {
        return Err("property returned null".into())
    } else {
        Ok(res)
    }
}

pub fn set_property_value(property: &UnityProperty, object: Option<&UnityObject>, value: &ArbitraryData) {
    fx_set_property_value(property, &UnityObject { inner: 0 }, value)
}

pub fn unbox_i32(obj: &UnityObject) -> Result<i32, Box<dyn Error>> {
    if obj.inner <= 0 {
        Err("object is null".into())
    } else {
        Ok(fx_unbox_i32(obj))
    }
}