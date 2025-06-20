pub use sensor_data_recorder::*;
/// This module was auto-generated with ethers-rs Abigen.
/// More information at: <https://github.com/gakonst/ethers-rs>
#[allow(
    clippy::enum_variant_names,
    clippy::too_many_arguments,
    clippy::upper_case_acronyms,
    clippy::type_complexity,
    dead_code,
    non_camel_case_types,
)]
pub mod sensor_data_recorder {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::None,
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("allSensorReadings"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("allSensorReadings"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("timestampUnixNano"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint64"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("sensorId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("location"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("processStage"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "temperatureCelsiusScaled",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Int(32usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("int32"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "humidityPercentScaled",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint32"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("recorder"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getAllSensorData"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getAllSensorData"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                ::std::vec![
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                                    ::ethers::core::abi::ethabi::ParamType::String,
                                                    ::ethers::core::abi::ethabi::ParamType::String,
                                                    ::ethers::core::abi::ethabi::ParamType::String,
                                                    ::ethers::core::abi::ethabi::ParamType::Int(32usize),
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                                    ::ethers::core::abi::ethabi::ParamType::Address,
                                                ],
                                            ),
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct SensorDataRecorder.SensorReading[]",
                                        ),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getSensorDataCount"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getSensorDataCount"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("recordSensorData"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("recordSensorData"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "_timestampUnixNano",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint64"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_sensorId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_location"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_processStage"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "_temperatureCelsiusScaled",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Int(32usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("int32"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "_humidityPercentScaled",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint32"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
            ]),
            events: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("DataRecorded"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("DataRecorded"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("timestampUnixNano"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("sensorId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "temperatureCelsiusScaled",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Int(32usize),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "humidityPercentScaled",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("recorder"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
            ]),
            errors: ::std::collections::BTreeMap::new(),
            receive: false,
            fallback: false,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static SENSORDATARECORDER_ABI: ::ethers::contract::Lazy<
        ::ethers::core::abi::Abi,
    > = ::ethers::contract::Lazy::new(__abi);
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[Pa\x14\x9B\x80a\0 `\09`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0LW`\x005`\xE0\x1C\x80c\x1E\xB2\x1A\t\x14a\0QW\x80c|]\xDD\x97\x14a\0oW\x80c\x9AK4'\x14a\0\xA5W\x80c\xD4\xFF\x1Cg\x14a\0\xC3W[`\0\x80\xFD[a\0Ya\0\xDFV[`@Qa\0f\x91\x90a\x08\xF0V[`@Q\x80\x91\x03\x90\xF3[a\0\x89`\x04\x806\x03\x81\x01\x90a\0\x84\x91\x90a\tKV[a\0\xEBV[`@Qa\0\x9C\x97\x96\x95\x94\x93\x92\x91\x90a\n\xA7V[`@Q\x80\x91\x03\x90\xF3[a\0\xADa\x03&V[`@Qa\0\xBA\x91\x90a\r\x1DV[`@Q\x80\x91\x03\x90\xF3[a\0\xDD`\x04\x806\x03\x81\x01\x90a\0\xD8\x91\x90a\x0E\xF8V[a\x06\x06V[\0[`\0\x80\x80T\x90P\x90P\x90V[`\0\x81\x81T\x81\x10a\0\xFBW`\0\x80\xFD[\x90`\0R` `\0 \x90`\x05\x02\x01`\0\x91P\x90P\x80`\0\x01`\0\x90T\x90a\x01\0\n\x90\x04g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90\x80`\x01\x01\x80Ta\x018\x90a\x10\x08V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x01d\x90a\x10\x08V[\x80\x15a\x01\xB1W\x80`\x1F\x10a\x01\x86Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x01\xB1V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x01\x94W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x90\x80`\x02\x01\x80Ta\x01\xC6\x90a\x10\x08V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x01\xF2\x90a\x10\x08V[\x80\x15a\x02?W\x80`\x1F\x10a\x02\x14Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x02?V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x02\"W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x90\x80`\x03\x01\x80Ta\x02T\x90a\x10\x08V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x02\x80\x90a\x10\x08V[\x80\x15a\x02\xCDW\x80`\x1F\x10a\x02\xA2Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x02\xCDV[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x02\xB0W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x90\x80`\x04\x01`\0\x90T\x90a\x01\0\n\x90\x04`\x03\x0B\x90\x80`\x04\x01`\x04\x90T\x90a\x01\0\n\x90\x04c\xFF\xFF\xFF\xFF\x16\x90\x80`\x04\x01`\x08\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90P\x87V[```\0\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01`\0\x90[\x82\x82\x10\x15a\x05\xFDW\x83\x82\x90`\0R` `\0 \x90`\x05\x02\x01`@Q\x80`\xE0\x01`@R\x90\x81`\0\x82\x01`\0\x90T\x90a\x01\0\n\x90\x04g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01`\x01\x82\x01\x80Ta\x03\xAF\x90a\x10\x08V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x03\xDB\x90a\x10\x08V[\x80\x15a\x04(W\x80`\x1F\x10a\x03\xFDWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x04(V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x04\x0BW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81R` \x01`\x02\x82\x01\x80Ta\x04A\x90a\x10\x08V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x04m\x90a\x10\x08V[\x80\x15a\x04\xBAW\x80`\x1F\x10a\x04\x8FWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x04\xBAV[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x04\x9DW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81R` \x01`\x03\x82\x01\x80Ta\x04\xD3\x90a\x10\x08V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x04\xFF\x90a\x10\x08V[\x80\x15a\x05LW\x80`\x1F\x10a\x05!Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x05LV[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x05/W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81R` \x01`\x04\x82\x01`\0\x90T\x90a\x01\0\n\x90\x04`\x03\x0B`\x03\x0B`\x03\x0B\x81R` \x01`\x04\x82\x01`\x04\x90T\x90a\x01\0\n\x90\x04c\xFF\xFF\xFF\xFF\x16c\xFF\xFF\xFF\xFF\x16c\xFF\xFF\xFF\xFF\x16\x81R` \x01`\x04\x82\x01`\x08\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81RPP\x81R` \x01\x90`\x01\x01\x90a\x03JV[PPPP\x90P\x90V[\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xF0`\x82`\x03\x0B\x12\x15\x80\x15a\x06>WPa0\xD4\x82`\x03\x0B\x13\x15[a\x06}W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x06t\x90a\x10\xABV[`@Q\x80\x91\x03\x90\xFD[`\0\x81c\xFF\xFF\xFF\xFF\x16\x10\x15\x80\x15a\x06\x9CWPa'\x10\x81c\xFF\xFF\xFF\xFF\x16\x11\x15[a\x06\xDBW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x06\xD2\x90a\x11=V[`@Q\x80\x91\x03\x90\xFD[`\0`@Q\x80`\xE0\x01`@R\x80\x88g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x87\x81R` \x01\x86\x81R` \x01\x85\x81R` \x01\x84`\x03\x0B\x81R` \x01\x83c\xFF\xFF\xFF\xFF\x16\x81R` \x013s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81RP\x90P`\0\x81\x90\x80`\x01\x81T\x01\x80\x82U\x80\x91PP`\x01\x90\x03\x90`\0R` `\0 \x90`\x05\x02\x01`\0\x90\x91\x90\x91\x90\x91P`\0\x82\x01Q\x81`\0\x01`\0a\x01\0\n\x81T\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP` \x82\x01Q\x81`\x01\x01\x90\x81a\x07\xAB\x91\x90a\x13\tV[P`@\x82\x01Q\x81`\x02\x01\x90\x81a\x07\xC1\x91\x90a\x13\tV[P``\x82\x01Q\x81`\x03\x01\x90\x81a\x07\xD7\x91\x90a\x13\tV[P`\x80\x82\x01Q\x81`\x04\x01`\0a\x01\0\n\x81T\x81c\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83`\x03\x0Bc\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP`\xA0\x82\x01Q\x81`\x04\x01`\x04a\x01\0\n\x81T\x81c\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83c\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP`\xC0\x82\x01Q\x81`\x04\x01`\x08a\x01\0\n\x81T\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UPPP\x85`@Qa\x08\x80\x91\x90a\x14\x17V[`@Q\x80\x91\x03\x90 \x87g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7F%B\x8B+\"\xCF\xDA\xB8\x95\xBA\x01\0f\x0F&\xB7\x11'\x0B\xEC\xE9\xA5i0\xA5\x88l \x07\x1F\xBD\x9B\x85\x853`@Qa\x08\xC6\x93\x92\x91\x90a\x14.V[`@Q\x80\x91\x03\x90\xA3PPPPPPPV[`\0\x81\x90P\x91\x90PV[a\x08\xEA\x81a\x08\xD7V[\x82RPPV[`\0` \x82\x01\x90Pa\t\x05`\0\x83\x01\x84a\x08\xE1V[\x92\x91PPV[`\0`@Q\x90P\x90V[`\0\x80\xFD[`\0\x80\xFD[a\t(\x81a\x08\xD7V[\x81\x14a\t3W`\0\x80\xFD[PV[`\0\x815\x90Pa\tE\x81a\t\x1FV[\x92\x91PPV[`\0` \x82\x84\x03\x12\x15a\taWa\t`a\t\x15V[[`\0a\to\x84\x82\x85\x01a\t6V[\x91PP\x92\x91PPV[`\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x90P\x91\x90PV[a\t\x95\x81a\txV[\x82RPPV[`\0\x81Q\x90P\x91\x90PV[`\0\x82\x82R` \x82\x01\x90P\x92\x91PPV[`\0[\x83\x81\x10\x15a\t\xD5W\x80\x82\x01Q\x81\x84\x01R` \x81\x01\x90Pa\t\xBAV[`\0\x84\x84\x01RPPPPV[`\0`\x1F\x19`\x1F\x83\x01\x16\x90P\x91\x90PV[`\0a\t\xFD\x82a\t\x9BV[a\n\x07\x81\x85a\t\xA6V[\x93Pa\n\x17\x81\x85` \x86\x01a\t\xB7V[a\n \x81a\t\xE1V[\x84\x01\x91PP\x92\x91PPV[`\0\x81`\x03\x0B\x90P\x91\x90PV[a\nA\x81a\n+V[\x82RPPV[`\0c\xFF\xFF\xFF\xFF\x82\x16\x90P\x91\x90PV[a\n`\x81a\nGV[\x82RPPV[`\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x90P\x91\x90PV[`\0a\n\x91\x82a\nfV[\x90P\x91\x90PV[a\n\xA1\x81a\n\x86V[\x82RPPV[`\0`\xE0\x82\x01\x90Pa\n\xBC`\0\x83\x01\x8Aa\t\x8CV[\x81\x81\x03` \x83\x01Ra\n\xCE\x81\x89a\t\xF2V[\x90P\x81\x81\x03`@\x83\x01Ra\n\xE2\x81\x88a\t\xF2V[\x90P\x81\x81\x03``\x83\x01Ra\n\xF6\x81\x87a\t\xF2V[\x90Pa\x0B\x05`\x80\x83\x01\x86a\n8V[a\x0B\x12`\xA0\x83\x01\x85a\nWV[a\x0B\x1F`\xC0\x83\x01\x84a\n\x98V[\x98\x97PPPPPPPPV[`\0\x81Q\x90P\x91\x90PV[`\0\x82\x82R` \x82\x01\x90P\x92\x91PPV[`\0\x81\x90P` \x82\x01\x90P\x91\x90PV[a\x0B`\x81a\txV[\x82RPPV[`\0\x82\x82R` \x82\x01\x90P\x92\x91PPV[`\0a\x0B\x82\x82a\t\x9BV[a\x0B\x8C\x81\x85a\x0BfV[\x93Pa\x0B\x9C\x81\x85` \x86\x01a\t\xB7V[a\x0B\xA5\x81a\t\xE1V[\x84\x01\x91PP\x92\x91PPV[a\x0B\xB9\x81a\n+V[\x82RPPV[a\x0B\xC8\x81a\nGV[\x82RPPV[a\x0B\xD7\x81a\n\x86V[\x82RPPV[`\0`\xE0\x83\x01`\0\x83\x01Qa\x0B\xF5`\0\x86\x01\x82a\x0BWV[P` \x83\x01Q\x84\x82\x03` \x86\x01Ra\x0C\r\x82\x82a\x0BwV[\x91PP`@\x83\x01Q\x84\x82\x03`@\x86\x01Ra\x0C'\x82\x82a\x0BwV[\x91PP``\x83\x01Q\x84\x82\x03``\x86\x01Ra\x0CA\x82\x82a\x0BwV[\x91PP`\x80\x83\x01Qa\x0CV`\x80\x86\x01\x82a\x0B\xB0V[P`\xA0\x83\x01Qa\x0Ci`\xA0\x86\x01\x82a\x0B\xBFV[P`\xC0\x83\x01Qa\x0C|`\xC0\x86\x01\x82a\x0B\xCEV[P\x80\x91PP\x92\x91PPV[`\0a\x0C\x93\x83\x83a\x0B\xDDV[\x90P\x92\x91PPV[`\0` \x82\x01\x90P\x91\x90PV[`\0a\x0C\xB3\x82a\x0B+V[a\x0C\xBD\x81\x85a\x0B6V[\x93P\x83` \x82\x02\x85\x01a\x0C\xCF\x85a\x0BGV[\x80`\0[\x85\x81\x10\x15a\r\x0BW\x84\x84\x03\x89R\x81Qa\x0C\xEC\x85\x82a\x0C\x87V[\x94Pa\x0C\xF7\x83a\x0C\x9BV[\x92P` \x8A\x01\x99PP`\x01\x81\x01\x90Pa\x0C\xD3V[P\x82\x97P\x87\x95PPPPPP\x92\x91PPV[`\0` \x82\x01\x90P\x81\x81\x03`\0\x83\x01Ra\r7\x81\x84a\x0C\xA8V[\x90P\x92\x91PPV[a\rH\x81a\txV[\x81\x14a\rSW`\0\x80\xFD[PV[`\0\x815\x90Pa\re\x81a\r?V[\x92\x91PPV[`\0\x80\xFD[`\0\x80\xFD[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`A`\x04R`$`\0\xFD[a\r\xAD\x82a\t\xE1V[\x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17\x15a\r\xCCWa\r\xCBa\ruV[[\x80`@RPPPV[`\0a\r\xDFa\t\x0BV[\x90Pa\r\xEB\x82\x82a\r\xA4V[\x91\x90PV[`\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a\x0E\x0BWa\x0E\na\ruV[[a\x0E\x14\x82a\t\xE1V[\x90P` \x81\x01\x90P\x91\x90PV[\x82\x81\x837`\0\x83\x83\x01RPPPV[`\0a\x0ECa\x0E>\x84a\r\xF0V[a\r\xD5V[\x90P\x82\x81R` \x81\x01\x84\x84\x84\x01\x11\x15a\x0E_Wa\x0E^a\rpV[[a\x0Ej\x84\x82\x85a\x0E!V[P\x93\x92PPPV[`\0\x82`\x1F\x83\x01\x12a\x0E\x87Wa\x0E\x86a\rkV[[\x815a\x0E\x97\x84\x82` \x86\x01a\x0E0V[\x91PP\x92\x91PPV[a\x0E\xA9\x81a\n+V[\x81\x14a\x0E\xB4W`\0\x80\xFD[PV[`\0\x815\x90Pa\x0E\xC6\x81a\x0E\xA0V[\x92\x91PPV[a\x0E\xD5\x81a\nGV[\x81\x14a\x0E\xE0W`\0\x80\xFD[PV[`\0\x815\x90Pa\x0E\xF2\x81a\x0E\xCCV[\x92\x91PPV[`\0\x80`\0\x80`\0\x80`\xC0\x87\x89\x03\x12\x15a\x0F\x15Wa\x0F\x14a\t\x15V[[`\0a\x0F#\x89\x82\x8A\x01a\rVV[\x96PP` \x87\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x0FDWa\x0FCa\t\x1AV[[a\x0FP\x89\x82\x8A\x01a\x0ErV[\x95PP`@\x87\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x0FqWa\x0Fpa\t\x1AV[[a\x0F}\x89\x82\x8A\x01a\x0ErV[\x94PP``\x87\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x0F\x9EWa\x0F\x9Da\t\x1AV[[a\x0F\xAA\x89\x82\x8A\x01a\x0ErV[\x93PP`\x80a\x0F\xBB\x89\x82\x8A\x01a\x0E\xB7V[\x92PP`\xA0a\x0F\xCC\x89\x82\x8A\x01a\x0E\xE3V[\x91PP\x92\x95P\x92\x95P\x92\x95V[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`\"`\x04R`$`\0\xFD[`\0`\x02\x82\x04\x90P`\x01\x82\x16\x80a\x10 W`\x7F\x82\x16\x91P[` \x82\x10\x81\x03a\x103Wa\x102a\x0F\xD9V[[P\x91\x90PV[\x7FInvalid temperature range (-40 t`\0\x82\x01R\x7Fo 125 C scaled)\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x82\x01RPV[`\0a\x10\x95`/\x83a\t\xA6V[\x91Pa\x10\xA0\x82a\x109V[`@\x82\x01\x90P\x91\x90PV[`\0` \x82\x01\x90P\x81\x81\x03`\0\x83\x01Ra\x10\xC4\x81a\x10\x88V[\x90P\x91\x90PV[\x7FInvalid humidity range (0 to 100`\0\x82\x01R\x7F % scaled)\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x82\x01RPV[`\0a\x11'`*\x83a\t\xA6V[\x91Pa\x112\x82a\x10\xCBV[`@\x82\x01\x90P\x91\x90PV[`\0` \x82\x01\x90P\x81\x81\x03`\0\x83\x01Ra\x11V\x81a\x11\x1AV[\x90P\x91\x90PV[`\0\x81\x90P\x81`\0R` `\0 \x90P\x91\x90PV[`\0` `\x1F\x83\x01\x04\x90P\x91\x90PV[`\0\x82\x82\x1B\x90P\x92\x91PPV[`\0`\x08\x83\x02a\x11\xBF\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82a\x11\x82V[a\x11\xC9\x86\x83a\x11\x82V[\x95P\x80\x19\x84\x16\x93P\x80\x86\x16\x84\x17\x92PPP\x93\x92PPPV[`\0\x81\x90P\x91\x90PV[`\0a\x12\x06a\x12\x01a\x11\xFC\x84a\x08\xD7V[a\x11\xE1V[a\x08\xD7V[\x90P\x91\x90PV[`\0\x81\x90P\x91\x90PV[a\x12 \x83a\x11\xEBV[a\x124a\x12,\x82a\x12\rV[\x84\x84Ta\x11\x8FV[\x82UPPPPV[`\0\x90V[a\x12Ia\x12<V[a\x12T\x81\x84\x84a\x12\x17V[PPPV[[\x81\x81\x10\x15a\x12xWa\x12m`\0\x82a\x12AV[`\x01\x81\x01\x90Pa\x12ZV[PPV[`\x1F\x82\x11\x15a\x12\xBDWa\x12\x8E\x81a\x11]V[a\x12\x97\x84a\x11rV[\x81\x01` \x85\x10\x15a\x12\xA6W\x81\x90P[a\x12\xBAa\x12\xB2\x85a\x11rV[\x83\x01\x82a\x12YV[PP[PPPV[`\0\x82\x82\x1C\x90P\x92\x91PPV[`\0a\x12\xE0`\0\x19\x84`\x08\x02a\x12\xC2V[\x19\x80\x83\x16\x91PP\x92\x91PPV[`\0a\x12\xF9\x83\x83a\x12\xCFV[\x91P\x82`\x02\x02\x82\x17\x90P\x92\x91PPV[a\x13\x12\x82a\t\x9BV[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x13+Wa\x13*a\ruV[[a\x135\x82Ta\x10\x08V[a\x13@\x82\x82\x85a\x12|V[`\0` \x90P`\x1F\x83\x11`\x01\x81\x14a\x13sW`\0\x84\x15a\x13aW\x82\x87\x01Q\x90P[a\x13k\x85\x82a\x12\xEDV[\x86UPa\x13\xD3V[`\x1F\x19\x84\x16a\x13\x81\x86a\x11]V[`\0[\x82\x81\x10\x15a\x13\xA9W\x84\x89\x01Q\x82U`\x01\x82\x01\x91P` \x85\x01\x94P` \x81\x01\x90Pa\x13\x84V[\x86\x83\x10\x15a\x13\xC6W\x84\x89\x01Qa\x13\xC2`\x1F\x89\x16\x82a\x12\xCFV[\x83UP[`\x01`\x02\x88\x02\x01\x88UPPP[PPPPPPV[`\0\x81\x90P\x92\x91PPV[`\0a\x13\xF1\x82a\t\x9BV[a\x13\xFB\x81\x85a\x13\xDBV[\x93Pa\x14\x0B\x81\x85` \x86\x01a\t\xB7V[\x80\x84\x01\x91PP\x92\x91PPV[`\0a\x14#\x82\x84a\x13\xE6V[\x91P\x81\x90P\x92\x91PPV[`\0``\x82\x01\x90Pa\x14C`\0\x83\x01\x86a\n8V[a\x14P` \x83\x01\x85a\nWV[a\x14]`@\x83\x01\x84a\n\x98V[\x94\x93PPPPV\xFE\xA2dipfsX\"\x12 \xC0\x1Ap\xACV\x86`V\xB3\xE2\xABe?\xBB\xDB\xEBg,XP+\xD3\xD5\xCB\x82I\xF8\xAC%\xA6\xE8\x9DdsolcC\0\x08\x14\x003";
    /// The bytecode of the contract.
    pub static SENSORDATARECORDER_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0LW`\x005`\xE0\x1C\x80c\x1E\xB2\x1A\t\x14a\0QW\x80c|]\xDD\x97\x14a\0oW\x80c\x9AK4'\x14a\0\xA5W\x80c\xD4\xFF\x1Cg\x14a\0\xC3W[`\0\x80\xFD[a\0Ya\0\xDFV[`@Qa\0f\x91\x90a\x08\xF0V[`@Q\x80\x91\x03\x90\xF3[a\0\x89`\x04\x806\x03\x81\x01\x90a\0\x84\x91\x90a\tKV[a\0\xEBV[`@Qa\0\x9C\x97\x96\x95\x94\x93\x92\x91\x90a\n\xA7V[`@Q\x80\x91\x03\x90\xF3[a\0\xADa\x03&V[`@Qa\0\xBA\x91\x90a\r\x1DV[`@Q\x80\x91\x03\x90\xF3[a\0\xDD`\x04\x806\x03\x81\x01\x90a\0\xD8\x91\x90a\x0E\xF8V[a\x06\x06V[\0[`\0\x80\x80T\x90P\x90P\x90V[`\0\x81\x81T\x81\x10a\0\xFBW`\0\x80\xFD[\x90`\0R` `\0 \x90`\x05\x02\x01`\0\x91P\x90P\x80`\0\x01`\0\x90T\x90a\x01\0\n\x90\x04g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90\x80`\x01\x01\x80Ta\x018\x90a\x10\x08V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x01d\x90a\x10\x08V[\x80\x15a\x01\xB1W\x80`\x1F\x10a\x01\x86Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x01\xB1V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x01\x94W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x90\x80`\x02\x01\x80Ta\x01\xC6\x90a\x10\x08V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x01\xF2\x90a\x10\x08V[\x80\x15a\x02?W\x80`\x1F\x10a\x02\x14Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x02?V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x02\"W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x90\x80`\x03\x01\x80Ta\x02T\x90a\x10\x08V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x02\x80\x90a\x10\x08V[\x80\x15a\x02\xCDW\x80`\x1F\x10a\x02\xA2Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x02\xCDV[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x02\xB0W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x90\x80`\x04\x01`\0\x90T\x90a\x01\0\n\x90\x04`\x03\x0B\x90\x80`\x04\x01`\x04\x90T\x90a\x01\0\n\x90\x04c\xFF\xFF\xFF\xFF\x16\x90\x80`\x04\x01`\x08\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90P\x87V[```\0\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01`\0\x90[\x82\x82\x10\x15a\x05\xFDW\x83\x82\x90`\0R` `\0 \x90`\x05\x02\x01`@Q\x80`\xE0\x01`@R\x90\x81`\0\x82\x01`\0\x90T\x90a\x01\0\n\x90\x04g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01`\x01\x82\x01\x80Ta\x03\xAF\x90a\x10\x08V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x03\xDB\x90a\x10\x08V[\x80\x15a\x04(W\x80`\x1F\x10a\x03\xFDWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x04(V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x04\x0BW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81R` \x01`\x02\x82\x01\x80Ta\x04A\x90a\x10\x08V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x04m\x90a\x10\x08V[\x80\x15a\x04\xBAW\x80`\x1F\x10a\x04\x8FWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x04\xBAV[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x04\x9DW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81R` \x01`\x03\x82\x01\x80Ta\x04\xD3\x90a\x10\x08V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x04\xFF\x90a\x10\x08V[\x80\x15a\x05LW\x80`\x1F\x10a\x05!Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x05LV[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x05/W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81R` \x01`\x04\x82\x01`\0\x90T\x90a\x01\0\n\x90\x04`\x03\x0B`\x03\x0B`\x03\x0B\x81R` \x01`\x04\x82\x01`\x04\x90T\x90a\x01\0\n\x90\x04c\xFF\xFF\xFF\xFF\x16c\xFF\xFF\xFF\xFF\x16c\xFF\xFF\xFF\xFF\x16\x81R` \x01`\x04\x82\x01`\x08\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81RPP\x81R` \x01\x90`\x01\x01\x90a\x03JV[PPPP\x90P\x90V[\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xF0`\x82`\x03\x0B\x12\x15\x80\x15a\x06>WPa0\xD4\x82`\x03\x0B\x13\x15[a\x06}W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x06t\x90a\x10\xABV[`@Q\x80\x91\x03\x90\xFD[`\0\x81c\xFF\xFF\xFF\xFF\x16\x10\x15\x80\x15a\x06\x9CWPa'\x10\x81c\xFF\xFF\xFF\xFF\x16\x11\x15[a\x06\xDBW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x06\xD2\x90a\x11=V[`@Q\x80\x91\x03\x90\xFD[`\0`@Q\x80`\xE0\x01`@R\x80\x88g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x87\x81R` \x01\x86\x81R` \x01\x85\x81R` \x01\x84`\x03\x0B\x81R` \x01\x83c\xFF\xFF\xFF\xFF\x16\x81R` \x013s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81RP\x90P`\0\x81\x90\x80`\x01\x81T\x01\x80\x82U\x80\x91PP`\x01\x90\x03\x90`\0R` `\0 \x90`\x05\x02\x01`\0\x90\x91\x90\x91\x90\x91P`\0\x82\x01Q\x81`\0\x01`\0a\x01\0\n\x81T\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP` \x82\x01Q\x81`\x01\x01\x90\x81a\x07\xAB\x91\x90a\x13\tV[P`@\x82\x01Q\x81`\x02\x01\x90\x81a\x07\xC1\x91\x90a\x13\tV[P``\x82\x01Q\x81`\x03\x01\x90\x81a\x07\xD7\x91\x90a\x13\tV[P`\x80\x82\x01Q\x81`\x04\x01`\0a\x01\0\n\x81T\x81c\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83`\x03\x0Bc\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP`\xA0\x82\x01Q\x81`\x04\x01`\x04a\x01\0\n\x81T\x81c\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83c\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP`\xC0\x82\x01Q\x81`\x04\x01`\x08a\x01\0\n\x81T\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UPPP\x85`@Qa\x08\x80\x91\x90a\x14\x17V[`@Q\x80\x91\x03\x90 \x87g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7F%B\x8B+\"\xCF\xDA\xB8\x95\xBA\x01\0f\x0F&\xB7\x11'\x0B\xEC\xE9\xA5i0\xA5\x88l \x07\x1F\xBD\x9B\x85\x853`@Qa\x08\xC6\x93\x92\x91\x90a\x14.V[`@Q\x80\x91\x03\x90\xA3PPPPPPPV[`\0\x81\x90P\x91\x90PV[a\x08\xEA\x81a\x08\xD7V[\x82RPPV[`\0` \x82\x01\x90Pa\t\x05`\0\x83\x01\x84a\x08\xE1V[\x92\x91PPV[`\0`@Q\x90P\x90V[`\0\x80\xFD[`\0\x80\xFD[a\t(\x81a\x08\xD7V[\x81\x14a\t3W`\0\x80\xFD[PV[`\0\x815\x90Pa\tE\x81a\t\x1FV[\x92\x91PPV[`\0` \x82\x84\x03\x12\x15a\taWa\t`a\t\x15V[[`\0a\to\x84\x82\x85\x01a\t6V[\x91PP\x92\x91PPV[`\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x90P\x91\x90PV[a\t\x95\x81a\txV[\x82RPPV[`\0\x81Q\x90P\x91\x90PV[`\0\x82\x82R` \x82\x01\x90P\x92\x91PPV[`\0[\x83\x81\x10\x15a\t\xD5W\x80\x82\x01Q\x81\x84\x01R` \x81\x01\x90Pa\t\xBAV[`\0\x84\x84\x01RPPPPV[`\0`\x1F\x19`\x1F\x83\x01\x16\x90P\x91\x90PV[`\0a\t\xFD\x82a\t\x9BV[a\n\x07\x81\x85a\t\xA6V[\x93Pa\n\x17\x81\x85` \x86\x01a\t\xB7V[a\n \x81a\t\xE1V[\x84\x01\x91PP\x92\x91PPV[`\0\x81`\x03\x0B\x90P\x91\x90PV[a\nA\x81a\n+V[\x82RPPV[`\0c\xFF\xFF\xFF\xFF\x82\x16\x90P\x91\x90PV[a\n`\x81a\nGV[\x82RPPV[`\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x90P\x91\x90PV[`\0a\n\x91\x82a\nfV[\x90P\x91\x90PV[a\n\xA1\x81a\n\x86V[\x82RPPV[`\0`\xE0\x82\x01\x90Pa\n\xBC`\0\x83\x01\x8Aa\t\x8CV[\x81\x81\x03` \x83\x01Ra\n\xCE\x81\x89a\t\xF2V[\x90P\x81\x81\x03`@\x83\x01Ra\n\xE2\x81\x88a\t\xF2V[\x90P\x81\x81\x03``\x83\x01Ra\n\xF6\x81\x87a\t\xF2V[\x90Pa\x0B\x05`\x80\x83\x01\x86a\n8V[a\x0B\x12`\xA0\x83\x01\x85a\nWV[a\x0B\x1F`\xC0\x83\x01\x84a\n\x98V[\x98\x97PPPPPPPPV[`\0\x81Q\x90P\x91\x90PV[`\0\x82\x82R` \x82\x01\x90P\x92\x91PPV[`\0\x81\x90P` \x82\x01\x90P\x91\x90PV[a\x0B`\x81a\txV[\x82RPPV[`\0\x82\x82R` \x82\x01\x90P\x92\x91PPV[`\0a\x0B\x82\x82a\t\x9BV[a\x0B\x8C\x81\x85a\x0BfV[\x93Pa\x0B\x9C\x81\x85` \x86\x01a\t\xB7V[a\x0B\xA5\x81a\t\xE1V[\x84\x01\x91PP\x92\x91PPV[a\x0B\xB9\x81a\n+V[\x82RPPV[a\x0B\xC8\x81a\nGV[\x82RPPV[a\x0B\xD7\x81a\n\x86V[\x82RPPV[`\0`\xE0\x83\x01`\0\x83\x01Qa\x0B\xF5`\0\x86\x01\x82a\x0BWV[P` \x83\x01Q\x84\x82\x03` \x86\x01Ra\x0C\r\x82\x82a\x0BwV[\x91PP`@\x83\x01Q\x84\x82\x03`@\x86\x01Ra\x0C'\x82\x82a\x0BwV[\x91PP``\x83\x01Q\x84\x82\x03``\x86\x01Ra\x0CA\x82\x82a\x0BwV[\x91PP`\x80\x83\x01Qa\x0CV`\x80\x86\x01\x82a\x0B\xB0V[P`\xA0\x83\x01Qa\x0Ci`\xA0\x86\x01\x82a\x0B\xBFV[P`\xC0\x83\x01Qa\x0C|`\xC0\x86\x01\x82a\x0B\xCEV[P\x80\x91PP\x92\x91PPV[`\0a\x0C\x93\x83\x83a\x0B\xDDV[\x90P\x92\x91PPV[`\0` \x82\x01\x90P\x91\x90PV[`\0a\x0C\xB3\x82a\x0B+V[a\x0C\xBD\x81\x85a\x0B6V[\x93P\x83` \x82\x02\x85\x01a\x0C\xCF\x85a\x0BGV[\x80`\0[\x85\x81\x10\x15a\r\x0BW\x84\x84\x03\x89R\x81Qa\x0C\xEC\x85\x82a\x0C\x87V[\x94Pa\x0C\xF7\x83a\x0C\x9BV[\x92P` \x8A\x01\x99PP`\x01\x81\x01\x90Pa\x0C\xD3V[P\x82\x97P\x87\x95PPPPPP\x92\x91PPV[`\0` \x82\x01\x90P\x81\x81\x03`\0\x83\x01Ra\r7\x81\x84a\x0C\xA8V[\x90P\x92\x91PPV[a\rH\x81a\txV[\x81\x14a\rSW`\0\x80\xFD[PV[`\0\x815\x90Pa\re\x81a\r?V[\x92\x91PPV[`\0\x80\xFD[`\0\x80\xFD[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`A`\x04R`$`\0\xFD[a\r\xAD\x82a\t\xE1V[\x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17\x15a\r\xCCWa\r\xCBa\ruV[[\x80`@RPPPV[`\0a\r\xDFa\t\x0BV[\x90Pa\r\xEB\x82\x82a\r\xA4V[\x91\x90PV[`\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a\x0E\x0BWa\x0E\na\ruV[[a\x0E\x14\x82a\t\xE1V[\x90P` \x81\x01\x90P\x91\x90PV[\x82\x81\x837`\0\x83\x83\x01RPPPV[`\0a\x0ECa\x0E>\x84a\r\xF0V[a\r\xD5V[\x90P\x82\x81R` \x81\x01\x84\x84\x84\x01\x11\x15a\x0E_Wa\x0E^a\rpV[[a\x0Ej\x84\x82\x85a\x0E!V[P\x93\x92PPPV[`\0\x82`\x1F\x83\x01\x12a\x0E\x87Wa\x0E\x86a\rkV[[\x815a\x0E\x97\x84\x82` \x86\x01a\x0E0V[\x91PP\x92\x91PPV[a\x0E\xA9\x81a\n+V[\x81\x14a\x0E\xB4W`\0\x80\xFD[PV[`\0\x815\x90Pa\x0E\xC6\x81a\x0E\xA0V[\x92\x91PPV[a\x0E\xD5\x81a\nGV[\x81\x14a\x0E\xE0W`\0\x80\xFD[PV[`\0\x815\x90Pa\x0E\xF2\x81a\x0E\xCCV[\x92\x91PPV[`\0\x80`\0\x80`\0\x80`\xC0\x87\x89\x03\x12\x15a\x0F\x15Wa\x0F\x14a\t\x15V[[`\0a\x0F#\x89\x82\x8A\x01a\rVV[\x96PP` \x87\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x0FDWa\x0FCa\t\x1AV[[a\x0FP\x89\x82\x8A\x01a\x0ErV[\x95PP`@\x87\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x0FqWa\x0Fpa\t\x1AV[[a\x0F}\x89\x82\x8A\x01a\x0ErV[\x94PP``\x87\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x0F\x9EWa\x0F\x9Da\t\x1AV[[a\x0F\xAA\x89\x82\x8A\x01a\x0ErV[\x93PP`\x80a\x0F\xBB\x89\x82\x8A\x01a\x0E\xB7V[\x92PP`\xA0a\x0F\xCC\x89\x82\x8A\x01a\x0E\xE3V[\x91PP\x92\x95P\x92\x95P\x92\x95V[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`\"`\x04R`$`\0\xFD[`\0`\x02\x82\x04\x90P`\x01\x82\x16\x80a\x10 W`\x7F\x82\x16\x91P[` \x82\x10\x81\x03a\x103Wa\x102a\x0F\xD9V[[P\x91\x90PV[\x7FInvalid temperature range (-40 t`\0\x82\x01R\x7Fo 125 C scaled)\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x82\x01RPV[`\0a\x10\x95`/\x83a\t\xA6V[\x91Pa\x10\xA0\x82a\x109V[`@\x82\x01\x90P\x91\x90PV[`\0` \x82\x01\x90P\x81\x81\x03`\0\x83\x01Ra\x10\xC4\x81a\x10\x88V[\x90P\x91\x90PV[\x7FInvalid humidity range (0 to 100`\0\x82\x01R\x7F % scaled)\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x82\x01RPV[`\0a\x11'`*\x83a\t\xA6V[\x91Pa\x112\x82a\x10\xCBV[`@\x82\x01\x90P\x91\x90PV[`\0` \x82\x01\x90P\x81\x81\x03`\0\x83\x01Ra\x11V\x81a\x11\x1AV[\x90P\x91\x90PV[`\0\x81\x90P\x81`\0R` `\0 \x90P\x91\x90PV[`\0` `\x1F\x83\x01\x04\x90P\x91\x90PV[`\0\x82\x82\x1B\x90P\x92\x91PPV[`\0`\x08\x83\x02a\x11\xBF\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82a\x11\x82V[a\x11\xC9\x86\x83a\x11\x82V[\x95P\x80\x19\x84\x16\x93P\x80\x86\x16\x84\x17\x92PPP\x93\x92PPPV[`\0\x81\x90P\x91\x90PV[`\0a\x12\x06a\x12\x01a\x11\xFC\x84a\x08\xD7V[a\x11\xE1V[a\x08\xD7V[\x90P\x91\x90PV[`\0\x81\x90P\x91\x90PV[a\x12 \x83a\x11\xEBV[a\x124a\x12,\x82a\x12\rV[\x84\x84Ta\x11\x8FV[\x82UPPPPV[`\0\x90V[a\x12Ia\x12<V[a\x12T\x81\x84\x84a\x12\x17V[PPPV[[\x81\x81\x10\x15a\x12xWa\x12m`\0\x82a\x12AV[`\x01\x81\x01\x90Pa\x12ZV[PPV[`\x1F\x82\x11\x15a\x12\xBDWa\x12\x8E\x81a\x11]V[a\x12\x97\x84a\x11rV[\x81\x01` \x85\x10\x15a\x12\xA6W\x81\x90P[a\x12\xBAa\x12\xB2\x85a\x11rV[\x83\x01\x82a\x12YV[PP[PPPV[`\0\x82\x82\x1C\x90P\x92\x91PPV[`\0a\x12\xE0`\0\x19\x84`\x08\x02a\x12\xC2V[\x19\x80\x83\x16\x91PP\x92\x91PPV[`\0a\x12\xF9\x83\x83a\x12\xCFV[\x91P\x82`\x02\x02\x82\x17\x90P\x92\x91PPV[a\x13\x12\x82a\t\x9BV[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x13+Wa\x13*a\ruV[[a\x135\x82Ta\x10\x08V[a\x13@\x82\x82\x85a\x12|V[`\0` \x90P`\x1F\x83\x11`\x01\x81\x14a\x13sW`\0\x84\x15a\x13aW\x82\x87\x01Q\x90P[a\x13k\x85\x82a\x12\xEDV[\x86UPa\x13\xD3V[`\x1F\x19\x84\x16a\x13\x81\x86a\x11]V[`\0[\x82\x81\x10\x15a\x13\xA9W\x84\x89\x01Q\x82U`\x01\x82\x01\x91P` \x85\x01\x94P` \x81\x01\x90Pa\x13\x84V[\x86\x83\x10\x15a\x13\xC6W\x84\x89\x01Qa\x13\xC2`\x1F\x89\x16\x82a\x12\xCFV[\x83UP[`\x01`\x02\x88\x02\x01\x88UPPP[PPPPPPV[`\0\x81\x90P\x92\x91PPV[`\0a\x13\xF1\x82a\t\x9BV[a\x13\xFB\x81\x85a\x13\xDBV[\x93Pa\x14\x0B\x81\x85` \x86\x01a\t\xB7V[\x80\x84\x01\x91PP\x92\x91PPV[`\0a\x14#\x82\x84a\x13\xE6V[\x91P\x81\x90P\x92\x91PPV[`\0``\x82\x01\x90Pa\x14C`\0\x83\x01\x86a\n8V[a\x14P` \x83\x01\x85a\nWV[a\x14]`@\x83\x01\x84a\n\x98V[\x94\x93PPPPV\xFE\xA2dipfsX\"\x12 \xC0\x1Ap\xACV\x86`V\xB3\xE2\xABe?\xBB\xDB\xEBg,XP+\xD3\xD5\xCB\x82I\xF8\xAC%\xA6\xE8\x9DdsolcC\0\x08\x14\x003";
    /// The deployed bytecode of the contract.
    pub static SENSORDATARECORDER_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct SensorDataRecorder<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for SensorDataRecorder<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for SensorDataRecorder<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for SensorDataRecorder<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for SensorDataRecorder<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(SensorDataRecorder))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> SensorDataRecorder<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    SENSORDATARECORDER_ABI.clone(),
                    client,
                ),
            )
        }
        /// Constructs the general purpose `Deployer` instance based on the provided constructor arguments and sends it.
        /// Returns a new instance of a deployer that returns an instance of this contract after sending the transaction
        ///
        /// Notes:
        /// - If there are no constructor arguments, you should pass `()` as the argument.
        /// - The default poll duration is 7 seconds.
        /// - The default number of confirmations is 1 block.
        ///
        ///
        /// # Example
        ///
        /// Generate contract bindings with `abigen!` and deploy a new contract instance.
        ///
        /// *Note*: this requires a `bytecode` and `abi` object in the `greeter.json` artifact.
        ///
        /// ```ignore
        /// # async fn deploy<M: ethers::providers::Middleware>(client: ::std::sync::Arc<M>) {
        ///     abigen!(Greeter, "../greeter.json");
        ///
        ///    let greeter_contract = Greeter::deploy(client, "Hello world!".to_string()).unwrap().send().await.unwrap();
        ///    let msg = greeter_contract.greet().call().await.unwrap();
        /// # }
        /// ```
        pub fn deploy<T: ::ethers::core::abi::Tokenize>(
            client: ::std::sync::Arc<M>,
            constructor_args: T,
        ) -> ::core::result::Result<
            ::ethers::contract::builders::ContractDeployer<M, Self>,
            ::ethers::contract::ContractError<M>,
        > {
            let factory = ::ethers::contract::ContractFactory::new(
                SENSORDATARECORDER_ABI.clone(),
                SENSORDATARECORDER_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `allSensorReadings` (0x7c5ddd97) function
        pub fn all_sensor_readings(
            &self,
            p0: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (
                u64,
                ::std::string::String,
                ::std::string::String,
                ::std::string::String,
                i32,
                u32,
                ::ethers::core::types::Address,
            ),
        > {
            self.0
                .method_hash([124, 93, 221, 151], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getAllSensorData` (0x9a4b3427) function
        pub fn get_all_sensor_data(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::std::vec::Vec<SensorReading>,
        > {
            self.0
                .method_hash([154, 75, 52, 39], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getSensorDataCount` (0x1eb21a09) function
        pub fn get_sensor_data_count(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([30, 178, 26, 9], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `recordSensorData` (0xd4ff1c67) function
        pub fn record_sensor_data(
            &self,
            timestamp_unix_nano: u64,
            sensor_id: ::std::string::String,
            location: ::std::string::String,
            process_stage: ::std::string::String,
            temperature_celsius_scaled: i32,
            humidity_percent_scaled: u32,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [212, 255, 28, 103],
                    (
                        timestamp_unix_nano,
                        sensor_id,
                        location,
                        process_stage,
                        temperature_celsius_scaled,
                        humidity_percent_scaled,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        ///Gets the contract's `DataRecorded` event
        pub fn data_recorded_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            DataRecordedFilter,
        > {
            self.0.event()
        }
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            DataRecordedFilter,
        > {
            self.0.event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for SensorDataRecorder<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(
        name = "DataRecorded",
        abi = "DataRecorded(uint64,string,int32,uint32,address)"
    )]
    pub struct DataRecordedFilter {
        #[ethevent(indexed)]
        pub timestamp_unix_nano: u64,
        #[ethevent(indexed)]
        pub sensor_id: ::ethers::core::types::H256,
        pub temperature_celsius_scaled: i32,
        pub humidity_percent_scaled: u32,
        pub recorder: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `allSensorReadings` function with signature `allSensorReadings(uint256)` and selector `0x7c5ddd97`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "allSensorReadings", abi = "allSensorReadings(uint256)")]
    pub struct AllSensorReadingsCall(pub ::ethers::core::types::U256);
    ///Container type for all input parameters for the `getAllSensorData` function with signature `getAllSensorData()` and selector `0x9a4b3427`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "getAllSensorData", abi = "getAllSensorData()")]
    pub struct GetAllSensorDataCall;
    ///Container type for all input parameters for the `getSensorDataCount` function with signature `getSensorDataCount()` and selector `0x1eb21a09`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "getSensorDataCount", abi = "getSensorDataCount()")]
    pub struct GetSensorDataCountCall;
    ///Container type for all input parameters for the `recordSensorData` function with signature `recordSensorData(uint64,string,string,string,int32,uint32)` and selector `0xd4ff1c67`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(
        name = "recordSensorData",
        abi = "recordSensorData(uint64,string,string,string,int32,uint32)"
    )]
    pub struct RecordSensorDataCall {
        pub timestamp_unix_nano: u64,
        pub sensor_id: ::std::string::String,
        pub location: ::std::string::String,
        pub process_stage: ::std::string::String,
        pub temperature_celsius_scaled: i32,
        pub humidity_percent_scaled: u32,
    }
    ///Container type for all of the contract's call
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum SensorDataRecorderCalls {
        AllSensorReadings(AllSensorReadingsCall),
        GetAllSensorData(GetAllSensorDataCall),
        GetSensorDataCount(GetSensorDataCountCall),
        RecordSensorData(RecordSensorDataCall),
    }
    impl ::ethers::core::abi::AbiDecode for SensorDataRecorderCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <AllSensorReadingsCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::AllSensorReadings(decoded));
            }
            if let Ok(decoded) = <GetAllSensorDataCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetAllSensorData(decoded));
            }
            if let Ok(decoded) = <GetSensorDataCountCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetSensorDataCount(decoded));
            }
            if let Ok(decoded) = <RecordSensorDataCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RecordSensorData(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for SensorDataRecorderCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::AllSensorReadings(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetAllSensorData(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetSensorDataCount(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RecordSensorData(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for SensorDataRecorderCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::AllSensorReadings(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetAllSensorData(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetSensorDataCount(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::RecordSensorData(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<AllSensorReadingsCall> for SensorDataRecorderCalls {
        fn from(value: AllSensorReadingsCall) -> Self {
            Self::AllSensorReadings(value)
        }
    }
    impl ::core::convert::From<GetAllSensorDataCall> for SensorDataRecorderCalls {
        fn from(value: GetAllSensorDataCall) -> Self {
            Self::GetAllSensorData(value)
        }
    }
    impl ::core::convert::From<GetSensorDataCountCall> for SensorDataRecorderCalls {
        fn from(value: GetSensorDataCountCall) -> Self {
            Self::GetSensorDataCount(value)
        }
    }
    impl ::core::convert::From<RecordSensorDataCall> for SensorDataRecorderCalls {
        fn from(value: RecordSensorDataCall) -> Self {
            Self::RecordSensorData(value)
        }
    }
    ///Container type for all return fields from the `allSensorReadings` function with signature `allSensorReadings(uint256)` and selector `0x7c5ddd97`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct AllSensorReadingsReturn {
        pub timestamp_unix_nano: u64,
        pub sensor_id: ::std::string::String,
        pub location: ::std::string::String,
        pub process_stage: ::std::string::String,
        pub temperature_celsius_scaled: i32,
        pub humidity_percent_scaled: u32,
        pub recorder: ::ethers::core::types::Address,
    }
    ///Container type for all return fields from the `getAllSensorData` function with signature `getAllSensorData()` and selector `0x9a4b3427`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct GetAllSensorDataReturn(pub ::std::vec::Vec<SensorReading>);
    ///Container type for all return fields from the `getSensorDataCount` function with signature `getSensorDataCount()` and selector `0x1eb21a09`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct GetSensorDataCountReturn(pub ::ethers::core::types::U256);
    ///`SensorReading(uint64,string,string,string,int32,uint32,address)`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct SensorReading {
        pub timestamp_unix_nano: u64,
        pub sensor_id: ::std::string::String,
        pub location: ::std::string::String,
        pub process_stage: ::std::string::String,
        pub temperature_celsius_scaled: i32,
        pub humidity_percent_scaled: u32,
        pub recorder: ::ethers::core::types::Address,
    }
}
