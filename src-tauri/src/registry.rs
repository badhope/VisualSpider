use crate::RegistryValue;
use winreg::RegKey;
use winreg::enums::*;
use encoding_rs::GBK;

fn decode_output(bytes: &[u8]) -> String {
    let (decoded, _, had_errors) = GBK.decode(bytes);
    if had_errors {
        String::from_utf8_lossy(bytes).to_string()
    } else {
        decoded.to_string()
    }
}

pub fn get_registry_tree() -> Result<Vec<serde_json::Value>, String> {
    let roots = vec![
        ("HKEY_CLASSES_ROOT", HKEY_CLASSES_ROOT),
        ("HKEY_CURRENT_USER", HKEY_CURRENT_USER),
        ("HKEY_LOCAL_MACHINE", HKEY_LOCAL_MACHINE),
        ("HKEY_USERS", HKEY_USERS),
        ("HKEY_CURRENT_CONFIG", HKEY_CURRENT_CONFIG),
    ];

    let mut tree = Vec::new();
    
    for (name, hkey) in roots {
        let key = RegKey::predef(hkey);
        let children = get_subkeys(&key, name);
        
        tree.push(serde_json::json!({
            "name": name,
            "path": name,
            "isRoot": true,
            "children": children
        }));
    }

    Ok(tree)
}

fn get_subkeys(key: &RegKey, path: &str) -> Vec<serde_json::Value> {
    let mut children = Vec::new();
    
    for subkey in key.enum_keys().filter_map(|k| k.ok()).take(50) {
        let child_path = format!("{}\\{}", path, subkey);
        if let Ok(child_key) = key.open_subkey(&subkey) {
            let sub_children = get_subkeys(&child_key, &child_path);
            children.push(serde_json::json!({
                "name": subkey,
                "path": child_path,
                "children": sub_children
            }));
        }
    }

    children
}

pub fn get_registry_values(path: &str) -> Result<Vec<RegistryValue>, String> {
    let (hkey, subpath) = parse_registry_path(path)?;
    let key = RegKey::predef(hkey);
    
    let key = key.open_subkey_with_flags(subpath, KEY_READ)
        .map_err(|e| format!("Failed to open registry key: {}", e))?;

    let mut values = Vec::new();

    for (name, value) in key.enum_values().filter_map(|v| v.ok()) {
        let (value_type, value_str) = match value {
            winreg::RegValue { vtype, bytes } => {
                let type_str = match vtype {
                    REG_SZ => "REG_SZ",
                    REG_EXPAND_SZ => "REG_EXPAND_SZ",
                    REG_BINARY => "REG_BINARY",
                    REG_DWORD => "REG_DWORD",
                    REG_DWORD_BIG_ENDIAN => "REG_DWORD_BIG_ENDIAN",
                    REG_LINK => "REG_LINK",
                    REG_MULTI_SZ => "REG_MULTI_SZ",
                    REG_NONE => "REG_NONE",
                    REG_QWORD => "REG_QWORD",
                    _ => "UNKNOWN",
                };

                let val_str = match vtype {
                    REG_SZ | REG_EXPAND_SZ => {
                        String::from_utf16_lossy(
                            &bytes.chunks(2)
                                .map(|chunk| {
                                    if chunk.len() == 2 {
                                        u16::from_le_bytes([chunk[0], chunk[1]])
                                    } else {
                                        0
                                    }
                                })
                                .collect::<Vec<u16>>()
                        ).trim_end_matches('\0').to_string()
                    }
                    REG_DWORD => {
                        if bytes.len() >= 4 {
                            format!("{}", u32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]))
                        } else {
                            "0".to_string()
                        }
                    }
                    REG_QWORD => {
                        if bytes.len() >= 8 {
                            format!("{}", u64::from_le_bytes([
                                bytes[0], bytes[1], bytes[2], bytes[3],
                                bytes[4], bytes[5], bytes[6], bytes[7]
                            ]))
                        } else {
                            "0".to_string()
                        }
                    }
                    _ => format!("{:?}", bytes),
                };

                (type_str.to_string(), val_str)
            }
        };

        values.push(RegistryValue {
            name: if name.is_empty() { "(默认)".to_string() } else { name },
            value_type,
            value: value_str,
        });
    }

    Ok(values)
}

pub fn set_registry_value(path: &str, name: &str, value_type: &str, value: &str) -> Result<(), String> {
    let (hkey, subpath) = parse_registry_path(path)?;
    let key = RegKey::predef(hkey);
    
    let key = key.open_subkey_with_flags(subpath, KEY_SET_VALUE)
        .map_err(|e| format!("Failed to open registry key: {}", e))?;

    match value_type {
        "REG_SZ" => {
            key.set_value(name, &value)
                .map_err(|e| format!("Failed to set value: {}", e))?;
        }
        "REG_DWORD" => {
            let val: u32 = value.parse().unwrap_or(0);
            key.set_value(name, &val)
                .map_err(|e| format!("Failed to set value: {}", e))?;
        }
        "REG_QWORD" => {
            let val: u64 = value.parse().unwrap_or(0);
            key.set_value(name, &val)
                .map_err(|e| format!("Failed to set value: {}", e))?;
        }
        _ => {
            key.set_value(name, &value)
                .map_err(|e| format!("Failed to set value: {}", e))?;
        }
    }

    Ok(())
}

pub fn create_registry_value(path: &str, name: &str, value_type: &str, value: &str) -> Result<(), String> {
    set_registry_value(path, name, value_type, value)
}

pub fn delete_registry_value(path: &str, name: &str) -> Result<(), String> {
    let (hkey, subpath) = parse_registry_path(path)?;
    let key = RegKey::predef(hkey);
    
    let key = key.open_subkey_with_flags(subpath, KEY_SET_VALUE)
        .map_err(|e| format!("Failed to open registry key: {}", e))?;

    key.delete_value(name)
        .map_err(|e| format!("Failed to delete value: {}", e))?;

    Ok(())
}

pub fn export_registry_key(path: &str) -> Result<String, String> {
    let output = std::process::Command::new("reg")
        .args(["export", path])
        .output()
        .map_err(|e| format!("Failed to export registry: {}", e))?;

    Ok(decode_output(&output.stdout))
}

pub fn get_registry_subkeys(path: &str) -> Result<Vec<serde_json::Value>, String> {
    let (hkey, subpath) = parse_registry_path(path)?;
    let key = RegKey::predef(hkey);
    
    let key = key.open_subkey_with_flags(subpath, KEY_READ)
        .map_err(|e| format!("Failed to open registry key: {}", e))?;

    let mut children = Vec::new();
    
    for subkey in key.enum_keys().filter_map(|k| k.ok()).take(100) {
        let child_path = format!("{}\\{}", path, subkey);
        children.push(serde_json::json!({
            "name": subkey,
            "path": child_path
        }));
    }

    Ok(children)
}

fn parse_registry_path(path: &str) -> Result<(isize, &str), String> {
    let (root, subpath) = path.split_once('\\')
        .ok_or_else(|| "Invalid registry path".to_string())?;

    let hkey = match root {
        "HKEY_CLASSES_ROOT" => HKEY_CLASSES_ROOT,
        "HKEY_CURRENT_USER" => HKEY_CURRENT_USER,
        "HKEY_LOCAL_MACHINE" => HKEY_LOCAL_MACHINE,
        "HKEY_USERS" => HKEY_USERS,
        "HKEY_CURRENT_CONFIG" => HKEY_CURRENT_CONFIG,
        _ => return Err(format!("Unknown registry root: {}", root)),
    };

    Ok((hkey, subpath))
}
