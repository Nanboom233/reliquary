// This file is generated by rust-protobuf 3.4.0. Do not edit
// .proto file is parsed by pure
// @generated

// https://github.com/rust-lang/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy::all)]

#![allow(unused_attributes)]
#![cfg_attr(rustfmt, rustfmt::skip)]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unused_results)]
#![allow(unused_mut)]

//! Generated file from `EPPGKKCMNEF.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

#[derive(Clone,Copy,PartialEq,Eq,Debug,Hash)]
// @@protoc_insertion_point(enum:EPPGKKCMNEF)
pub enum EPPGKKCMNEF {
    // @@protoc_insertion_point(enum_value:EPPGKKCMNEF.LANGUAGE_NONE)
    LANGUAGE_NONE = 0,
    // @@protoc_insertion_point(enum_value:EPPGKKCMNEF.LANGUAGE_SC)
    LANGUAGE_SC = 1,
    // @@protoc_insertion_point(enum_value:EPPGKKCMNEF.LANGUAGE_TC)
    LANGUAGE_TC = 2,
    // @@protoc_insertion_point(enum_value:EPPGKKCMNEF.LANGUAGE_EN)
    LANGUAGE_EN = 3,
    // @@protoc_insertion_point(enum_value:EPPGKKCMNEF.LANGUAGE_KR)
    LANGUAGE_KR = 4,
    // @@protoc_insertion_point(enum_value:EPPGKKCMNEF.LANGUAGE_JP)
    LANGUAGE_JP = 5,
    // @@protoc_insertion_point(enum_value:EPPGKKCMNEF.LANGUAGE_FR)
    LANGUAGE_FR = 6,
    // @@protoc_insertion_point(enum_value:EPPGKKCMNEF.LANGUAGE_DE)
    LANGUAGE_DE = 7,
    // @@protoc_insertion_point(enum_value:EPPGKKCMNEF.LANGUAGE_ES)
    LANGUAGE_ES = 8,
    // @@protoc_insertion_point(enum_value:EPPGKKCMNEF.LANGUAGE_PT)
    LANGUAGE_PT = 9,
    // @@protoc_insertion_point(enum_value:EPPGKKCMNEF.LANGUAGE_RU)
    LANGUAGE_RU = 10,
    // @@protoc_insertion_point(enum_value:EPPGKKCMNEF.LANGUAGE_TH)
    LANGUAGE_TH = 11,
    // @@protoc_insertion_point(enum_value:EPPGKKCMNEF.LANGUAGE_VI)
    LANGUAGE_VI = 12,
    // @@protoc_insertion_point(enum_value:EPPGKKCMNEF.LANGUAGE_ID)
    LANGUAGE_ID = 13,
}

impl ::protobuf::Enum for EPPGKKCMNEF {
    const NAME: &'static str = "EPPGKKCMNEF";

    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<EPPGKKCMNEF> {
        match value {
            0 => ::std::option::Option::Some(EPPGKKCMNEF::LANGUAGE_NONE),
            1 => ::std::option::Option::Some(EPPGKKCMNEF::LANGUAGE_SC),
            2 => ::std::option::Option::Some(EPPGKKCMNEF::LANGUAGE_TC),
            3 => ::std::option::Option::Some(EPPGKKCMNEF::LANGUAGE_EN),
            4 => ::std::option::Option::Some(EPPGKKCMNEF::LANGUAGE_KR),
            5 => ::std::option::Option::Some(EPPGKKCMNEF::LANGUAGE_JP),
            6 => ::std::option::Option::Some(EPPGKKCMNEF::LANGUAGE_FR),
            7 => ::std::option::Option::Some(EPPGKKCMNEF::LANGUAGE_DE),
            8 => ::std::option::Option::Some(EPPGKKCMNEF::LANGUAGE_ES),
            9 => ::std::option::Option::Some(EPPGKKCMNEF::LANGUAGE_PT),
            10 => ::std::option::Option::Some(EPPGKKCMNEF::LANGUAGE_RU),
            11 => ::std::option::Option::Some(EPPGKKCMNEF::LANGUAGE_TH),
            12 => ::std::option::Option::Some(EPPGKKCMNEF::LANGUAGE_VI),
            13 => ::std::option::Option::Some(EPPGKKCMNEF::LANGUAGE_ID),
            _ => ::std::option::Option::None
        }
    }

    fn from_str(str: &str) -> ::std::option::Option<EPPGKKCMNEF> {
        match str {
            "LANGUAGE_NONE" => ::std::option::Option::Some(EPPGKKCMNEF::LANGUAGE_NONE),
            "LANGUAGE_SC" => ::std::option::Option::Some(EPPGKKCMNEF::LANGUAGE_SC),
            "LANGUAGE_TC" => ::std::option::Option::Some(EPPGKKCMNEF::LANGUAGE_TC),
            "LANGUAGE_EN" => ::std::option::Option::Some(EPPGKKCMNEF::LANGUAGE_EN),
            "LANGUAGE_KR" => ::std::option::Option::Some(EPPGKKCMNEF::LANGUAGE_KR),
            "LANGUAGE_JP" => ::std::option::Option::Some(EPPGKKCMNEF::LANGUAGE_JP),
            "LANGUAGE_FR" => ::std::option::Option::Some(EPPGKKCMNEF::LANGUAGE_FR),
            "LANGUAGE_DE" => ::std::option::Option::Some(EPPGKKCMNEF::LANGUAGE_DE),
            "LANGUAGE_ES" => ::std::option::Option::Some(EPPGKKCMNEF::LANGUAGE_ES),
            "LANGUAGE_PT" => ::std::option::Option::Some(EPPGKKCMNEF::LANGUAGE_PT),
            "LANGUAGE_RU" => ::std::option::Option::Some(EPPGKKCMNEF::LANGUAGE_RU),
            "LANGUAGE_TH" => ::std::option::Option::Some(EPPGKKCMNEF::LANGUAGE_TH),
            "LANGUAGE_VI" => ::std::option::Option::Some(EPPGKKCMNEF::LANGUAGE_VI),
            "LANGUAGE_ID" => ::std::option::Option::Some(EPPGKKCMNEF::LANGUAGE_ID),
            _ => ::std::option::Option::None
        }
    }

    const VALUES: &'static [EPPGKKCMNEF] = &[
        EPPGKKCMNEF::LANGUAGE_NONE,
        EPPGKKCMNEF::LANGUAGE_SC,
        EPPGKKCMNEF::LANGUAGE_TC,
        EPPGKKCMNEF::LANGUAGE_EN,
        EPPGKKCMNEF::LANGUAGE_KR,
        EPPGKKCMNEF::LANGUAGE_JP,
        EPPGKKCMNEF::LANGUAGE_FR,
        EPPGKKCMNEF::LANGUAGE_DE,
        EPPGKKCMNEF::LANGUAGE_ES,
        EPPGKKCMNEF::LANGUAGE_PT,
        EPPGKKCMNEF::LANGUAGE_RU,
        EPPGKKCMNEF::LANGUAGE_TH,
        EPPGKKCMNEF::LANGUAGE_VI,
        EPPGKKCMNEF::LANGUAGE_ID,
    ];
}

impl ::protobuf::EnumFull for EPPGKKCMNEF {
    fn enum_descriptor() -> ::protobuf::reflect::EnumDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().enum_by_package_relative_name("EPPGKKCMNEF").unwrap()).clone()
    }

    fn descriptor(&self) -> ::protobuf::reflect::EnumValueDescriptor {
        let index = *self as usize;
        Self::enum_descriptor().value_by_index(index)
    }
}

impl ::std::default::Default for EPPGKKCMNEF {
    fn default() -> Self {
        EPPGKKCMNEF::LANGUAGE_NONE
    }
}

impl EPPGKKCMNEF {
    fn generated_enum_descriptor_data() -> ::protobuf::reflect::GeneratedEnumDescriptorData {
        ::protobuf::reflect::GeneratedEnumDescriptorData::new::<EPPGKKCMNEF>("EPPGKKCMNEF")
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x11EPPGKKCMNEF.proto*\xfd\x01\n\x0bEPPGKKCMNEF\x12\x11\n\rLANGUAGE_NO\
    NE\x10\0\x12\x0f\n\x0bLANGUAGE_SC\x10\x01\x12\x0f\n\x0bLANGUAGE_TC\x10\
    \x02\x12\x0f\n\x0bLANGUAGE_EN\x10\x03\x12\x0f\n\x0bLANGUAGE_KR\x10\x04\
    \x12\x0f\n\x0bLANGUAGE_JP\x10\x05\x12\x0f\n\x0bLANGUAGE_FR\x10\x06\x12\
    \x0f\n\x0bLANGUAGE_DE\x10\x07\x12\x0f\n\x0bLANGUAGE_ES\x10\x08\x12\x0f\n\
    \x0bLANGUAGE_PT\x10\t\x12\x0f\n\x0bLANGUAGE_RU\x10\n\x12\x0f\n\x0bLANGUA\
    GE_TH\x10\x0b\x12\x0f\n\x0bLANGUAGE_VI\x10\x0c\x12\x0f\n\x0bLANGUAGE_ID\
    \x10\rb\x06proto3\
";

/// `FileDescriptorProto` object which was a source for this generated file
fn file_descriptor_proto() -> &'static ::protobuf::descriptor::FileDescriptorProto {
    static file_descriptor_proto_lazy: ::protobuf::rt::Lazy<::protobuf::descriptor::FileDescriptorProto> = ::protobuf::rt::Lazy::new();
    file_descriptor_proto_lazy.get(|| {
        ::protobuf::Message::parse_from_bytes(file_descriptor_proto_data).unwrap()
    })
}

/// `FileDescriptor` object which allows dynamic access to files
pub fn file_descriptor() -> &'static ::protobuf::reflect::FileDescriptor {
    static generated_file_descriptor_lazy: ::protobuf::rt::Lazy<::protobuf::reflect::GeneratedFileDescriptor> = ::protobuf::rt::Lazy::new();
    static file_descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::FileDescriptor> = ::protobuf::rt::Lazy::new();
    file_descriptor.get(|| {
        let generated_file_descriptor = generated_file_descriptor_lazy.get(|| {
            let mut deps = ::std::vec::Vec::with_capacity(0);
            let mut messages = ::std::vec::Vec::with_capacity(0);
            let mut enums = ::std::vec::Vec::with_capacity(1);
            enums.push(EPPGKKCMNEF::generated_enum_descriptor_data());
            ::protobuf::reflect::GeneratedFileDescriptor::new_generated(
                file_descriptor_proto(),
                deps,
                messages,
                enums,
            )
        });
        ::protobuf::reflect::FileDescriptor::new_generated_2(generated_file_descriptor)
    })
}
