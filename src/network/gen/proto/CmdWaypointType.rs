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

//! Generated file from `CmdWaypointType.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

#[derive(Clone,Copy,PartialEq,Eq,Debug,Hash)]
// @@protoc_insertion_point(enum:CmdWaypointType)
pub enum CmdWaypointType {
    // @@protoc_insertion_point(enum_value:CmdWaypointType.CmdWaypointTypeNone)
    CmdWaypointTypeNone = 0,
    // @@protoc_insertion_point(enum_value:CmdWaypointType.CmdTakeChapterRewardScRsp)
    CmdTakeChapterRewardScRsp = 424,
    // @@protoc_insertion_point(enum_value:CmdWaypointType.CmdGetChapterScRsp)
    CmdGetChapterScRsp = 428,
    // @@protoc_insertion_point(enum_value:CmdWaypointType.CmdSetCurWaypointCsReq)
    CmdSetCurWaypointCsReq = 484,
    // @@protoc_insertion_point(enum_value:CmdWaypointType.CmdGetWaypointCsReq)
    CmdGetWaypointCsReq = 436,
    // @@protoc_insertion_point(enum_value:CmdWaypointType.CmdGetWaypointScRsp)
    CmdGetWaypointScRsp = 495,
    // @@protoc_insertion_point(enum_value:CmdWaypointType.CmdTakeChapterRewardCsReq)
    CmdTakeChapterRewardCsReq = 474,
    // @@protoc_insertion_point(enum_value:CmdWaypointType.CmdWaypointShowNewCsNotify)
    CmdWaypointShowNewCsNotify = 452,
    // @@protoc_insertion_point(enum_value:CmdWaypointType.CmdSetCurWaypointScRsp)
    CmdSetCurWaypointScRsp = 427,
    // @@protoc_insertion_point(enum_value:CmdWaypointType.CmdGetChapterCsReq)
    CmdGetChapterCsReq = 467,
}

impl ::protobuf::Enum for CmdWaypointType {
    const NAME: &'static str = "CmdWaypointType";

    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<CmdWaypointType> {
        match value {
            0 => ::std::option::Option::Some(CmdWaypointType::CmdWaypointTypeNone),
            424 => ::std::option::Option::Some(CmdWaypointType::CmdTakeChapterRewardScRsp),
            428 => ::std::option::Option::Some(CmdWaypointType::CmdGetChapterScRsp),
            484 => ::std::option::Option::Some(CmdWaypointType::CmdSetCurWaypointCsReq),
            436 => ::std::option::Option::Some(CmdWaypointType::CmdGetWaypointCsReq),
            495 => ::std::option::Option::Some(CmdWaypointType::CmdGetWaypointScRsp),
            474 => ::std::option::Option::Some(CmdWaypointType::CmdTakeChapterRewardCsReq),
            452 => ::std::option::Option::Some(CmdWaypointType::CmdWaypointShowNewCsNotify),
            427 => ::std::option::Option::Some(CmdWaypointType::CmdSetCurWaypointScRsp),
            467 => ::std::option::Option::Some(CmdWaypointType::CmdGetChapterCsReq),
            _ => ::std::option::Option::None
        }
    }

    fn from_str(str: &str) -> ::std::option::Option<CmdWaypointType> {
        match str {
            "CmdWaypointTypeNone" => ::std::option::Option::Some(CmdWaypointType::CmdWaypointTypeNone),
            "CmdTakeChapterRewardScRsp" => ::std::option::Option::Some(CmdWaypointType::CmdTakeChapterRewardScRsp),
            "CmdGetChapterScRsp" => ::std::option::Option::Some(CmdWaypointType::CmdGetChapterScRsp),
            "CmdSetCurWaypointCsReq" => ::std::option::Option::Some(CmdWaypointType::CmdSetCurWaypointCsReq),
            "CmdGetWaypointCsReq" => ::std::option::Option::Some(CmdWaypointType::CmdGetWaypointCsReq),
            "CmdGetWaypointScRsp" => ::std::option::Option::Some(CmdWaypointType::CmdGetWaypointScRsp),
            "CmdTakeChapterRewardCsReq" => ::std::option::Option::Some(CmdWaypointType::CmdTakeChapterRewardCsReq),
            "CmdWaypointShowNewCsNotify" => ::std::option::Option::Some(CmdWaypointType::CmdWaypointShowNewCsNotify),
            "CmdSetCurWaypointScRsp" => ::std::option::Option::Some(CmdWaypointType::CmdSetCurWaypointScRsp),
            "CmdGetChapterCsReq" => ::std::option::Option::Some(CmdWaypointType::CmdGetChapterCsReq),
            _ => ::std::option::Option::None
        }
    }

    const VALUES: &'static [CmdWaypointType] = &[
        CmdWaypointType::CmdWaypointTypeNone,
        CmdWaypointType::CmdTakeChapterRewardScRsp,
        CmdWaypointType::CmdGetChapterScRsp,
        CmdWaypointType::CmdSetCurWaypointCsReq,
        CmdWaypointType::CmdGetWaypointCsReq,
        CmdWaypointType::CmdGetWaypointScRsp,
        CmdWaypointType::CmdTakeChapterRewardCsReq,
        CmdWaypointType::CmdWaypointShowNewCsNotify,
        CmdWaypointType::CmdSetCurWaypointScRsp,
        CmdWaypointType::CmdGetChapterCsReq,
    ];
}

impl ::protobuf::EnumFull for CmdWaypointType {
    fn enum_descriptor() -> ::protobuf::reflect::EnumDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().enum_by_package_relative_name("CmdWaypointType").unwrap()).clone()
    }

    fn descriptor(&self) -> ::protobuf::reflect::EnumValueDescriptor {
        let index = match self {
            CmdWaypointType::CmdWaypointTypeNone => 0,
            CmdWaypointType::CmdTakeChapterRewardScRsp => 1,
            CmdWaypointType::CmdGetChapterScRsp => 2,
            CmdWaypointType::CmdSetCurWaypointCsReq => 3,
            CmdWaypointType::CmdGetWaypointCsReq => 4,
            CmdWaypointType::CmdGetWaypointScRsp => 5,
            CmdWaypointType::CmdTakeChapterRewardCsReq => 6,
            CmdWaypointType::CmdWaypointShowNewCsNotify => 7,
            CmdWaypointType::CmdSetCurWaypointScRsp => 8,
            CmdWaypointType::CmdGetChapterCsReq => 9,
        };
        Self::enum_descriptor().value_by_index(index)
    }
}

impl ::std::default::Default for CmdWaypointType {
    fn default() -> Self {
        CmdWaypointType::CmdWaypointTypeNone
    }
}

impl CmdWaypointType {
    fn generated_enum_descriptor_data() -> ::protobuf::reflect::GeneratedEnumDescriptorData {
        ::protobuf::reflect::GeneratedEnumDescriptorData::new::<CmdWaypointType>("CmdWaypointType")
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x15CmdWaypointType.proto*\xab\x02\n\x0fCmdWaypointType\x12\x17\n\x13C\
    mdWaypointTypeNone\x10\0\x12\x1e\n\x19CmdTakeChapterRewardScRsp\x10\xa8\
    \x03\x12\x17\n\x12CmdGetChapterScRsp\x10\xac\x03\x12\x1b\n\x16CmdSetCurW\
    aypointCsReq\x10\xe4\x03\x12\x18\n\x13CmdGetWaypointCsReq\x10\xb4\x03\
    \x12\x18\n\x13CmdGetWaypointScRsp\x10\xef\x03\x12\x1e\n\x19CmdTakeChapte\
    rRewardCsReq\x10\xda\x03\x12\x1f\n\x1aCmdWaypointShowNewCsNotify\x10\xc4\
    \x03\x12\x1b\n\x16CmdSetCurWaypointScRsp\x10\xab\x03\x12\x17\n\x12CmdGet\
    ChapterCsReq\x10\xd3\x03b\x06proto3\
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
            enums.push(CmdWaypointType::generated_enum_descriptor_data());
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
