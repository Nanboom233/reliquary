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

//! Generated file from `LABCKCCJBGA.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

#[derive(Clone,Copy,PartialEq,Eq,Debug,Hash)]
// @@protoc_insertion_point(enum:LABCKCCJBGA)
pub enum LABCKCCJBGA {
    // @@protoc_insertion_point(enum_value:LABCKCCJBGA.WAYPOINT_UNLOCK_NONE)
    WAYPOINT_UNLOCK_NONE = 0,
    // @@protoc_insertion_point(enum_value:LABCKCCJBGA.WAYPOINT_UNLOCK_PRE)
    WAYPOINT_UNLOCK_PRE = 1,
    // @@protoc_insertion_point(enum_value:LABCKCCJBGA.WAYPOINT_UNLOCK_LEVEL)
    WAYPOINT_UNLOCK_LEVEL = 2,
}

impl ::protobuf::Enum for LABCKCCJBGA {
    const NAME: &'static str = "LABCKCCJBGA";

    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<LABCKCCJBGA> {
        match value {
            0 => ::std::option::Option::Some(LABCKCCJBGA::WAYPOINT_UNLOCK_NONE),
            1 => ::std::option::Option::Some(LABCKCCJBGA::WAYPOINT_UNLOCK_PRE),
            2 => ::std::option::Option::Some(LABCKCCJBGA::WAYPOINT_UNLOCK_LEVEL),
            _ => ::std::option::Option::None
        }
    }

    fn from_str(str: &str) -> ::std::option::Option<LABCKCCJBGA> {
        match str {
            "WAYPOINT_UNLOCK_NONE" => ::std::option::Option::Some(LABCKCCJBGA::WAYPOINT_UNLOCK_NONE),
            "WAYPOINT_UNLOCK_PRE" => ::std::option::Option::Some(LABCKCCJBGA::WAYPOINT_UNLOCK_PRE),
            "WAYPOINT_UNLOCK_LEVEL" => ::std::option::Option::Some(LABCKCCJBGA::WAYPOINT_UNLOCK_LEVEL),
            _ => ::std::option::Option::None
        }
    }

    const VALUES: &'static [LABCKCCJBGA] = &[
        LABCKCCJBGA::WAYPOINT_UNLOCK_NONE,
        LABCKCCJBGA::WAYPOINT_UNLOCK_PRE,
        LABCKCCJBGA::WAYPOINT_UNLOCK_LEVEL,
    ];
}

impl ::protobuf::EnumFull for LABCKCCJBGA {
    fn enum_descriptor() -> ::protobuf::reflect::EnumDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().enum_by_package_relative_name("LABCKCCJBGA").unwrap()).clone()
    }

    fn descriptor(&self) -> ::protobuf::reflect::EnumValueDescriptor {
        let index = *self as usize;
        Self::enum_descriptor().value_by_index(index)
    }
}

impl ::std::default::Default for LABCKCCJBGA {
    fn default() -> Self {
        LABCKCCJBGA::WAYPOINT_UNLOCK_NONE
    }
}

impl LABCKCCJBGA {
    fn generated_enum_descriptor_data() -> ::protobuf::reflect::GeneratedEnumDescriptorData {
        ::protobuf::reflect::GeneratedEnumDescriptorData::new::<LABCKCCJBGA>("LABCKCCJBGA")
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x11LABCKCCJBGA.proto*[\n\x0bLABCKCCJBGA\x12\x18\n\x14WAYPOINT_UNLOCK_\
    NONE\x10\0\x12\x17\n\x13WAYPOINT_UNLOCK_PRE\x10\x01\x12\x19\n\x15WAYPOIN\
    T_UNLOCK_LEVEL\x10\x02b\x06proto3\
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
            enums.push(LABCKCCJBGA::generated_enum_descriptor_data());
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
