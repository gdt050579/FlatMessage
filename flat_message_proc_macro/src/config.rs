use crate::{
    attribute_parser, utils,
    validate_checksum::{self, ValidateChecksum},
    version_validator_parser::VersionValidatorParser,
};
use proc_macro::*;

pub(crate) struct Config {
    pub(crate) namehash: bool,
    pub(crate) checksum: bool,
    pub(crate) version: u8,
    pub(crate) validate_name: bool,
    pub(crate) compatible_versions: Option<VersionValidatorParser>,
    pub(crate) validate_checksum: ValidateChecksum,
    pub(crate) optimized_unchecked_code: bool,
    pub(crate) use_default_if_deserialize_fails: Option<bool>,
}

impl Config {
    pub(crate) fn new(args: TokenStream) -> Self {
        let mut store_name = true;
        let mut add_checksum = false;
        let mut validate_name = false;
        let mut version = 0u8;
        let mut compatible_versions = None;
        let mut validate_checksum = ValidateChecksum::Auto;
        let mut optimized_unchecked_code = true;
        let mut use_default_if_deserialize_fails = None;
        //println!("--Parsing attributes: '{}'", args.to_string());
        let attrs = attribute_parser::parse(args);
        for (attr_name, attr_value) in attrs.iter() {
            //println!("--Evaluete: '{}' => '{}'",attr_name,attr_value);
            match attr_name.as_str() {
                "store_name" => store_name = utils::to_bool(attr_value.as_str()).unwrap_or_else(|| panic!("Invalid boolean value ('{}') for attribute '{}'. Allowed values are 'true' or 'false' !",attr_value.as_str(), attr_name)),
                "checksum" => add_checksum = utils::to_bool(attr_value.as_str()).unwrap_or_else(|| panic!("Invalid boolean value ('{}') for attribute '{}'. Allowed values are 'true' or 'false' !",attr_value.as_str(), attr_name)),
                "version" => version = utils::to_version(attr_value.as_str()).unwrap_or_else(|| panic!("Invalid version value ('{}') for attribute '{}'. Allowed values are between 1 and 255 !",attr_value.as_str(), attr_name)),
                "validate_name" => validate_name = utils::to_bool(attr_value.as_str()).unwrap_or_else(|| panic!("Invalid boolean value ('{}') for attribute '{}'. Allowed values are 'true' or 'false' !",attr_value.as_str(), attr_name)),
                "validate_checksum" => validate_checksum = validate_checksum::ValidateChecksum::from_str(attr_value.as_str()),
                "compatible_versions" => {
                    match VersionValidatorParser::try_from(attr_value.as_str()) {
                        Ok(cv) => compatible_versions = Some(cv),
                        Err(def) => panic!("Fail to parse compatible_versions: {}", def),
                    }
                }
                "optimized_unchecked_code" => optimized_unchecked_code = utils::to_bool(attr_value.as_str()).unwrap_or_else(|| panic!("Invalid boolean value ('{}') for attribute '{}'. Allowed values are 'true' or 'false' !",attr_value.as_str(), attr_name)),
                "validate" => {
                    match attr_value.as_str() {
                        "strict" => use_default_if_deserialize_fails = Some(false),
                        "fallback" => use_default_if_deserialize_fails = Some(true),
                        _ => panic!("Invalid value for attribute 'validate': {}. Allowed values are 'strict' or 'fallback' !", attr_value.as_str()),
                    }
                }
                _ => {
                    panic!("Unknown attribute: {}. Supported attributes are: 'store_name', 'metadata', 'checksum', validate_name', 'optimized_unchecked_code', 'validate', 'compatible_versions' and 'version' !", attr_name);
                }
            }
        }

        if !store_name && validate_name {
            panic!("You can not use the attribute 'validate_name' with value 'true' unless the attribute 'store_name' is also set to 'true'.  If this was allowed, you will not be able to deserialize a structure of this type !");
        }

        Self {
            namehash: store_name,
            checksum: add_checksum,
            version,
            validate_name,
            validate_checksum,
            compatible_versions,
            optimized_unchecked_code,
            use_default_if_deserialize_fails,
        }
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            namehash: true,
            checksum: false,
            version: 0,
            validate_name: false,
            compatible_versions: None,
            validate_checksum: ValidateChecksum::Auto,
            optimized_unchecked_code: true,
            use_default_if_deserialize_fails: None,
        }
    }
}
