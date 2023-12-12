// Copyright (c) The Ant Group Core Contributors
// Copyright (c) The Smart Intermediate Representation Contributors
// SPDX-License-Identifier: Apache-2.0

use smart_ir::abi::params::ABIParam;
use smart_ir::ir::cfg::Contract;
use std::collections::HashMap;
use std::str::FromStr;

pub const CURRENT_IR_ABI_VERSION: u16 = 1;

/// The contract meta information for app, including the meta info of the contract,
/// generated by the ir compiler
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct IRContractABIMeta {
    /// The version information of ContractABIMeta, because the structure is generated by
    /// the compiler, may add fields, so you need to select the corresponding schema
    /// to decode according to the version information.
    pub abi_version: u16,
    /// A list of functions in the contract that can be called by transactions and
    /// triggered by special scenarios.
    pub methods: Vec<IRContractMethodMeta>,
}

impl Default for IRContractABIMeta {
    fn default() -> IRContractABIMeta {
        IRContractABIMeta {
            abi_version: 0,
            methods: Vec::new(),
        }
    }
}

impl IRContractABIMeta {
    pub fn to_json(&self) -> Vec<u8> {
        serde_json::to_string_pretty(self)
            .map_err(|e| anyhow::anyhow!("could not serialize to json: {}", e))
            .unwrap()
            .into_bytes()
    }

    pub fn from_json(json_bytes: &[u8]) -> IRContractABIMeta {
        serde_json::from_slice(json_bytes).unwrap()
    }

    pub fn get_method(&self, abi_method_name: &str) -> Option<&IRContractMethodMeta> {
        self.methods.iter().find(|&m| m.name == abi_method_name)
    }

    pub fn from_contract(contract: &Contract) -> IRContractABIMeta {
        let mut methods: Vec<IRContractMethodMeta> = vec![];
        // get methods
        for (func_name, func_def) in contract.functions.iter() {
            let mut inputs: Vec<IRContractMethodInputMeta> = vec![];
            for p in &func_def.params {
                inputs.push(IRContractMethodInputMeta {
                    name: "".to_string(),
                    r#type: p.to_string(),
                });
            }
            let mut outputs: Vec<IRContractMethodOutputMeta> = vec![];
            if !func_def.ret.is_void() {
                outputs.push(IRContractMethodOutputMeta {
                    r#type: func_def.ret.to_string(),
                });
            }
            let abi_name = if let Some(last_dot_pos) = func_name.rfind('.') {
                func_name[(last_dot_pos + 1)..func_name.len()].to_string()
            } else {
                func_name.clone()
            };
            methods.push(IRContractMethodMeta {
                name: abi_name.to_string(),
                r#type: if abi_name == "init" {
                    "constructor".to_string()
                } else {
                    "function".to_string()
                },
                inputs,
                outputs,
            });
        }
        IRContractABIMeta {
            abi_version: CURRENT_IR_ABI_VERSION,
            methods,
        }
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub struct IRConstantMeta {
    pub r#type: String,
    // ParamType
    pub data: String,
    // hex
    pub readable: String, // formatted constant value
}

/// meta info of contract method input/param
#[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
pub struct IRContractMethodInputMeta {
    pub name: String,
    pub r#type: String,
}

/// A method meta info in the corresponding contract for the transaction to call.
#[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
pub struct IRContractMethodMeta {
    pub name: String,
    /// contract abi type
    /// 'constructor' or 'function'
    pub r#type: String,
    /// The schema of method parameters, each uint8 corresponds to a parameter,
    /// and a specific value corresponds to a specific type of parameter, so
    /// that the actual value can be decoded according to the parameter encoding
    /// in the input and transaction body, s
    /// uch as 0: bool; 1: u8; 2: i8 ;3: u16 and so on.
    pub inputs: Vec<IRContractMethodInputMeta>,
    // vector of method param types names
    /// output: The schema of the return value of the method, used to decipher the
    /// return value, similar to input.
    pub outputs: Vec<IRContractMethodOutputMeta>, // // vector of method return types names
}

fn input_type_to_abi_param(input_type_name: &str, param_str: &str) -> Result<ABIParam, String> {
    match input_type_name {
        "bool" => Ok(ABIParam::Bool(param_str == "true")),
        "str" | "string" => Ok(ABIParam::Str(param_str.to_string())),
        "parampack" => {
            let bs = hex::decode(param_str);
            if bs.is_err() {
                return Err(bs.err().unwrap().to_string());
            }
            let bs = bs.unwrap();
            Ok(ABIParam::Parampack(bs))
        }
        "u8" => {
            let int_value = u8::from_str(param_str);
            if int_value.is_err() {
                return Err(int_value.err().unwrap().to_string());
            }
            let int_value = int_value.unwrap();
            Ok(ABIParam::U8(int_value))
        }
        "i8" => {
            let int_value = i8::from_str(param_str);
            if int_value.is_err() {
                return Err(int_value.err().unwrap().to_string());
            }
            let int_value = int_value.unwrap();
            Ok(ABIParam::I8(int_value))
        }
        "u16" => {
            let int_value = u16::from_str(param_str);
            if int_value.is_err() {
                return Err(int_value.err().unwrap().to_string());
            }
            let int_value = int_value.unwrap();
            Ok(ABIParam::U16(int_value))
        }
        "i16" => {
            let int_value = i16::from_str(param_str);
            if int_value.is_err() {
                return Err(int_value.err().unwrap().to_string());
            }
            let int_value = int_value.unwrap();
            Ok(ABIParam::I16(int_value))
        }
        "u32" => {
            let int_value = u32::from_str(param_str);
            if int_value.is_err() {
                return Err(int_value.err().unwrap().to_string());
            }
            let int_value = int_value.unwrap();
            Ok(ABIParam::U32(int_value))
        }
        "i32" => {
            let int_value = i32::from_str(param_str);
            if int_value.is_err() {
                return Err(int_value.err().unwrap().to_string());
            }
            let int_value = int_value.unwrap();
            Ok(ABIParam::I32(int_value))
        }
        "u64" => {
            let int_value = u64::from_str(param_str);
            if int_value.is_err() {
                return Err(int_value.err().unwrap().to_string());
            }
            let int_value = int_value.unwrap();
            Ok(ABIParam::U64(int_value))
        }
        "i64" => {
            let int_value = i64::from_str(param_str);
            if int_value.is_err() {
                return Err(int_value.err().unwrap().to_string());
            }
            let int_value = int_value.unwrap();
            Ok(ABIParam::I64(int_value))
        }
        "u128" => {
            let int_value = u128::from_str(param_str);
            if int_value.is_err() {
                return Err(int_value.err().unwrap().to_string());
            }
            let int_value = int_value.unwrap();
            Ok(ABIParam::U128(int_value))
        }
        "i128" => {
            let int_value = i128::from_str(param_str);
            if int_value.is_err() {
                return Err(int_value.err().unwrap().to_string());
            }
            let int_value = int_value.unwrap();
            Ok(ABIParam::I128(int_value))
        }
        _ => {
            if input_type_name.starts_with('[') {
                let inner_type_name = input_type_name[1..(input_type_name.len() - 1)].to_string();
                let inner_type_name = inner_type_name.as_str();
                let array_params: Vec<&str> = param_str.split(',').collect();

                match inner_type_name {
                    "bool" => {
                        let mut values: Vec<bool> = vec![];
                        for item in array_params {
                            values.push(bool::from_str(item).unwrap());
                        }
                        return Ok(ABIParam::BoolArray(values));
                    }
                    "str" | "string" => {
                        let mut values: Vec<String> = vec![];
                        for item in array_params {
                            values.push(item.to_string());
                        }
                        return Ok(ABIParam::StrArray(values));
                    }
                    "i8" => {
                        let mut values: Vec<i8> = vec![];
                        for item in array_params {
                            values.push(i8::from_str(item).unwrap());
                        }
                        return Ok(ABIParam::I8Array(values));
                    }
                    "u8" => {
                        let mut values: Vec<u8> = vec![];
                        for item in array_params {
                            values.push(u8::from_str(item).unwrap());
                        }
                        return Ok(ABIParam::U8Array(values));
                    }
                    "i16" => {
                        let mut values: Vec<i16> = vec![];
                        for item in array_params {
                            values.push(i16::from_str(item).unwrap());
                        }
                        return Ok(ABIParam::I16Array(values));
                    }
                    "u16" => {
                        let mut values: Vec<u16> = vec![];
                        for item in array_params {
                            values.push(u16::from_str(item).unwrap());
                        }
                        return Ok(ABIParam::U16Array(values));
                    }
                    "i32" => {
                        let mut values: Vec<i32> = vec![];
                        for item in array_params {
                            values.push(i32::from_str(item).unwrap());
                        }
                        return Ok(ABIParam::I32Array(values));
                    }
                    "u32" => {
                        let mut values: Vec<u32> = vec![];
                        for item in array_params {
                            values.push(u32::from_str(item).unwrap());
                        }
                        return Ok(ABIParam::U32Array(values));
                    }
                    "i64" => {
                        let mut values: Vec<i64> = vec![];
                        for item in array_params {
                            values.push(i64::from_str(item).unwrap());
                        }
                        return Ok(ABIParam::I64Array(values));
                    }
                    "u64" => {
                        let mut values: Vec<u64> = vec![];
                        for item in array_params {
                            values.push(u64::from_str(item).unwrap());
                        }
                        return Ok(ABIParam::U64Array(values));
                    }
                    "i128" => {
                        let mut values: Vec<i128> = vec![];
                        for item in array_params {
                            values.push(i128::from_str(item).unwrap());
                        }
                        return Ok(ABIParam::I128Array(values));
                    }
                    "u128" => {
                        let mut values: Vec<u128> = vec![];
                        for item in array_params {
                            values.push(u128::from_str(item).unwrap());
                        }
                        return Ok(ABIParam::U128Array(values));
                    }
                    _ => {
                        return Err("not supported input param type".to_string());
                    }
                }
            } else if input_type_name.starts_with('{') {
                if let Some(sep_pos) = input_type_name.find(':') {
                    let inner_type_name =
                        input_type_name[(sep_pos + 1)..(input_type_name.len() - 1)].to_string();
                    let inner_type_name = inner_type_name.as_str();
                    // k1:v1,k2:v2...
                    let params_pairs: Vec<&str> = param_str.split(',').collect();
                    let mut params_keys: Vec<&str> = vec![];
                    let mut params_values: Vec<&str> = vec![];
                    for p in params_pairs {
                        let splited: Vec<&str> = p.split(':').collect();
                        if splited.len() < 2 {
                            return Err("invalid map entry, expected k1:v1,k2:v2,...".to_string());
                        }
                        params_keys.push(splited[0]);
                        params_values.push(splited[1]);
                    }

                    match inner_type_name {
                        "bool" => {
                            let mut values: HashMap<String, bool> = HashMap::new();
                            for i in 0..params_keys.len() {
                                let item = params_values[i];
                                values.insert(
                                    params_keys[i].to_string(),
                                    bool::from_str(item).unwrap(),
                                );
                            }
                            return Ok(ABIParam::StrBoolMap(values));
                        }
                        "str" | "string" => {
                            let mut values: HashMap<String, String> = HashMap::new();
                            for i in 0..params_keys.len() {
                                let item = params_values[i];
                                values.insert(params_keys[i].to_string(), item.to_string());
                            }
                            return Ok(ABIParam::StrStrMap(values));
                        }
                        "i8" => {
                            let mut values: HashMap<String, i8> = HashMap::new();
                            for i in 0..params_keys.len() {
                                let item = params_values[i];
                                values.insert(
                                    params_keys[i].to_string(),
                                    i8::from_str(item).unwrap(),
                                );
                            }
                            return Ok(ABIParam::StrI8Map(values));
                        }
                        "u8" => {
                            let mut values: HashMap<String, u8> = HashMap::new();
                            for i in 0..params_keys.len() {
                                let item = params_values[i];
                                values.insert(
                                    params_keys[i].to_string(),
                                    u8::from_str(item).unwrap(),
                                );
                            }
                            return Ok(ABIParam::StrU8Map(values));
                        }
                        "i16" => {
                            let mut values: HashMap<String, i16> = HashMap::new();
                            for i in 0..params_keys.len() {
                                let item = params_values[i];
                                values.insert(
                                    params_keys[i].to_string(),
                                    i16::from_str(item).unwrap(),
                                );
                            }
                            return Ok(ABIParam::StrI16Map(values));
                        }
                        "u16" => {
                            let mut values: HashMap<String, u16> = HashMap::new();
                            for i in 0..params_keys.len() {
                                let item = params_values[i];
                                values.insert(
                                    params_keys[i].to_string(),
                                    u16::from_str(item).unwrap(),
                                );
                            }
                            return Ok(ABIParam::StrU16Map(values));
                        }
                        "i32" => {
                            let mut values: HashMap<String, i32> = HashMap::new();
                            for i in 0..params_keys.len() {
                                let item = params_values[i];
                                values.insert(
                                    params_keys[i].to_string(),
                                    i32::from_str(item).unwrap(),
                                );
                            }
                            return Ok(ABIParam::StrI32Map(values));
                        }
                        "u32" => {
                            let mut values: HashMap<String, u32> = HashMap::new();
                            for i in 0..params_keys.len() {
                                let item = params_values[i];
                                values.insert(
                                    params_keys[i].to_string(),
                                    u32::from_str(item).unwrap(),
                                );
                            }
                            return Ok(ABIParam::StrU32Map(values));
                        }
                        "i64" => {
                            let mut values: HashMap<String, i64> = HashMap::new();
                            for i in 0..params_keys.len() {
                                let item = params_values[i];
                                values.insert(
                                    params_keys[i].to_string(),
                                    i64::from_str(item).unwrap(),
                                );
                            }
                            return Ok(ABIParam::StrI64Map(values));
                        }
                        "u64" => {
                            let mut values: HashMap<String, u64> = HashMap::new();
                            for i in 0..params_keys.len() {
                                let item = params_values[i];
                                values.insert(
                                    params_keys[i].to_string(),
                                    u64::from_str(item).unwrap(),
                                );
                            }
                            return Ok(ABIParam::StrU64Map(values));
                        }
                        "i128" => {
                            let mut values: HashMap<String, i128> = HashMap::new();
                            for i in 0..params_keys.len() {
                                let item = params_values[i];
                                values.insert(
                                    params_keys[i].to_string(),
                                    i128::from_str(item).unwrap(),
                                );
                            }
                            return Ok(ABIParam::StrI128Map(values));
                        }
                        "u128" => {
                            let mut values: HashMap<String, u128> = HashMap::new();
                            for i in 0..params_keys.len() {
                                let item = params_values[i];
                                values.insert(
                                    params_keys[i].to_string(),
                                    u128::from_str(item).unwrap(),
                                );
                            }
                            return Ok(ABIParam::StrU128Map(values));
                        }
                        _ => {
                            return Err(format!(
                                "not supported input param type {inner_type_name}"
                            ));
                        }
                    }
                } else {
                    return Err("not supported map param type".to_string());
                }
            }
            Err(format!("not supported abi param type {input_type_name}"))
        }
    }
}

impl IRContractMethodMeta {
    pub fn encode_params(&self, params_strings: &[&str]) -> Result<Vec<u8>, String> {
        if self.inputs.len() != params_strings.len() {
            return Err("params count not match".to_string());
        }
        let mut result: Vec<u8> = vec![0x00]; // first byte is abi version
        for (i, param_str) in params_strings.iter().enumerate() {
            let input_meta = &self.inputs[i];
            let abi_input_param = input_type_to_abi_param(&input_meta.r#type, param_str);
            if abi_input_param.is_err() {
                return Err(abi_input_param.err().unwrap());
            }
            let mut abi_input_param_bytes = abi_input_param.unwrap().as_bytes();
            result.append(&mut abi_input_param_bytes);
        }
        Ok(result)
    }
}

#[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
pub struct IRContractMethodOutputMeta {
    pub r#type: String,
}
