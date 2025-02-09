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

//! Generated file from `JMGFGEMNIOC.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

#[derive(Clone,Copy,PartialEq,Eq,Debug,Hash)]
// @@protoc_insertion_point(enum:JMGFGEMNIOC)
pub enum JMGFGEMNIOC {
    // @@protoc_insertion_point(enum_value:JMGFGEMNIOC.GAMEPLAY_COUNTER_NONE)
    GAMEPLAY_COUNTER_NONE = 0,
    // @@protoc_insertion_point(enum_value:JMGFGEMNIOC.GAMEPLAY_COUNTER_MONSTER_SNEAK_VISION)
    GAMEPLAY_COUNTER_MONSTER_SNEAK_VISION = 280001,
}

impl ::protobuf::Enum for JMGFGEMNIOC {
    const NAME: &'static str = "JMGFGEMNIOC";

    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<JMGFGEMNIOC> {
        match value {
            0 => ::std::option::Option::Some(JMGFGEMNIOC::GAMEPLAY_COUNTER_NONE),
            280001 => ::std::option::Option::Some(JMGFGEMNIOC::GAMEPLAY_COUNTER_MONSTER_SNEAK_VISION),
            _ => ::std::option::Option::None
        }
    }

    fn from_str(str: &str) -> ::std::option::Option<JMGFGEMNIOC> {
        match str {
            "GAMEPLAY_COUNTER_NONE" => ::std::option::Option::Some(JMGFGEMNIOC::GAMEPLAY_COUNTER_NONE),
            "GAMEPLAY_COUNTER_MONSTER_SNEAK_VISION" => ::std::option::Option::Some(JMGFGEMNIOC::GAMEPLAY_COUNTER_MONSTER_SNEAK_VISION),
            _ => ::std::option::Option::None
        }
    }

    const VALUES: &'static [JMGFGEMNIOC] = &[
        JMGFGEMNIOC::GAMEPLAY_COUNTER_NONE,
        JMGFGEMNIOC::GAMEPLAY_COUNTER_MONSTER_SNEAK_VISION,
    ];
}

impl ::protobuf::EnumFull for JMGFGEMNIOC {
    fn enum_descriptor() -> ::protobuf::reflect::EnumDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().enum_by_package_relative_name("JMGFGEMNIOC").unwrap()).clone()
    }

    fn descriptor(&self) -> ::protobuf::reflect::EnumValueDescriptor {
        let index = match self {
            JMGFGEMNIOC::GAMEPLAY_COUNTER_NONE => 0,
            JMGFGEMNIOC::GAMEPLAY_COUNTER_MONSTER_SNEAK_VISION => 1,
        };
        Self::enum_descriptor().value_by_index(index)
    }
}

impl ::std::default::Default for JMGFGEMNIOC {
    fn default() -> Self {
        JMGFGEMNIOC::GAMEPLAY_COUNTER_NONE
    }
}

impl JMGFGEMNIOC {
    fn generated_enum_descriptor_data() -> ::protobuf::reflect::GeneratedEnumDescriptorData {
        ::protobuf::reflect::GeneratedEnumDescriptorData::new::<JMGFGEMNIOC>("JMGFGEMNIOC")
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x11JMGFGEMNIOC.proto*U\n\x0bJMGFGEMNIOC\x12\x19\n\x15GAMEPLAY_COUNTER\
    _NONE\x10\0\x12+\n%GAMEPLAY_COUNTER_MONSTER_SNEAK_VISION\x10\xc1\x8b\x11\
    b\x06proto3\
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
            enums.push(JMGFGEMNIOC::generated_enum_descriptor_data());
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
