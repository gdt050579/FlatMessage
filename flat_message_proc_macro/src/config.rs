use crate::{
    attribute_parser, utils,
    validate_checksum::{self, ValidateChecksum},
    version_validator_parser::VersionValidatorParser,
};
use proc_macro::*;

pub(crate) struct Config {
    pub(crate) namehash: bool,
    pub(crate) metadata: bool,
    pub(crate) checksum: bool,
    pub(crate) version: u8,
    pub(crate) validate_name: bool,
    pub(crate) compatible_versions: Option<VersionValidatorParser>,
    pub(crate) validate_checksum: ValidateChecksum,
}

impl Config {
    pub(crate) fn new(args: TokenStream) -> Self {
        let mut store_name = true;
        let mut add_metadata = true;
        let mut add_checksum = false;
        let mut validate_name = false;
        let mut version = 0u8;
        let mut compatible_versions = None;
        let mut validate_checksum = ValidateChecksum::Auto;

        let attrs = attribute_parser::parse(args);
        for (attr_name, attr_value) in attrs.iter() {
            match attr_name.as_str() {
                "store_name" => store_name = utils::to_bool(attr_value).unwrap_or_else(|| panic!("Invalid boolean value ('{}') for attribute '{}'. Allowed values are 'true' or 'false' !",attr_value, attr_name)),
                "metadata" => add_metadata = utils::to_bool(attr_value).unwrap_or_else(|| panic!("Invalid boolean value ('{}') for attribute '{}'. Allowed values are 'true' or 'false' !",attr_value, attr_name)),
                "checksum" => add_checksum = utils::to_bool(attr_value).unwrap_or_else(|| panic!("Invalid boolean value ('{}') for attribute '{}'. Allowed values are 'true' or 'false' !",attr_value, attr_name)),
                "version" => version = utils::to_version(attr_value).unwrap_or_else(|| panic!("Invalid version value ('{}') for attribute '{}'. Allowed values are between 1 and 255 !",attr_value, attr_name)),
                "validate_name" => validate_name = utils::to_bool(attr_value).unwrap_or_else(|| panic!("Invalid boolean value ('{}') for attribute '{}'. Allowed values are 'true' or 'false' !",attr_value, attr_name)),
                "validate_checksum" => validate_checksum = validate_checksum::ValidateChecksum::from_str(attr_value.as_str()),
                "compatible_versions" => {
                    match VersionValidatorParser::try_from(attr_value.replace("\"", "").as_str()) {
                        Ok(cv) => compatible_versions = Some(cv),
                        Err(def) => panic!("Fail to parse compatible_versions: {}", def),
                    }
                }
                _ => {
                    panic!("Unknown attribute: {}. Supported attributes are: 'store_name', 'metadata', 'checksum', validate_name', 'compatible_versions' and 'version' !", attr_name);
                }
            }
        }

        if !store_name && validate_name {
            panic!("You can not use the attribute 'validate_name' with value 'true' unless the attribute 'store_name' is also set to 'true'.  If this was allowed, you will not be able to deserialize a structure of this type !");
        }

        Self {
            namehash: store_name,
            metadata: add_metadata,
            checksum: add_checksum,
            version,
            validate_name,
            validate_checksum,
            compatible_versions,
        }
    }
}
