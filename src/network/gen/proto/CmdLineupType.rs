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

//! Generated file from `CmdLineupType.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

#[derive(Clone,Copy,PartialEq,Eq,Debug,Hash)]
// @@protoc_insertion_point(enum:CmdLineupType)
pub enum CmdLineupType {
    // @@protoc_insertion_point(enum_value:CmdLineupType.CmdLineupTypeNone)
    CmdLineupTypeNone = 0,
    // @@protoc_insertion_point(enum_value:CmdLineupType.CmdQuitLineupCsReq)
    CmdQuitLineupCsReq = 752,
    // @@protoc_insertion_point(enum_value:CmdLineupType.CmdGetLineupAvatarDataScRsp)
    CmdGetLineupAvatarDataScRsp = 746,
    // @@protoc_insertion_point(enum_value:CmdLineupType.CmdSwapLineupScRsp)
    CmdSwapLineupScRsp = 793,
    // @@protoc_insertion_point(enum_value:CmdLineupType.CmdSwitchLineupIndexScRsp)
    CmdSwitchLineupIndexScRsp = 775,
    // @@protoc_insertion_point(enum_value:CmdLineupType.CmdGetLineupAvatarDataCsReq)
    CmdGetLineupAvatarDataCsReq = 743,
    // @@protoc_insertion_point(enum_value:CmdLineupType.CmdJoinLineupScRsp)
    CmdJoinLineupScRsp = 728,
    // @@protoc_insertion_point(enum_value:CmdLineupType.CmdSetLineupNameScRsp)
    CmdSetLineupNameScRsp = 726,
    // @@protoc_insertion_point(enum_value:CmdLineupType.CmdGetCurLineupDataCsReq)
    CmdGetCurLineupDataCsReq = 784,
    // @@protoc_insertion_point(enum_value:CmdLineupType.CmdChangeLineupLeaderCsReq)
    CmdChangeLineupLeaderCsReq = 725,
    // @@protoc_insertion_point(enum_value:CmdLineupType.CmdChangeLineupLeaderScRsp)
    CmdChangeLineupLeaderScRsp = 796,
    // @@protoc_insertion_point(enum_value:CmdLineupType.CmdSwitchLineupIndexCsReq)
    CmdSwitchLineupIndexCsReq = 705,
    // @@protoc_insertion_point(enum_value:CmdLineupType.CmdQuitLineupScRsp)
    CmdQuitLineupScRsp = 774,
    // @@protoc_insertion_point(enum_value:CmdLineupType.CmdGetStageLineupCsReq)
    CmdGetStageLineupCsReq = 736,
    // @@protoc_insertion_point(enum_value:CmdLineupType.CmdReplaceLineupScRsp)
    CmdReplaceLineupScRsp = 712,
    // @@protoc_insertion_point(enum_value:CmdLineupType.CmdGetStageLineupScRsp)
    CmdGetStageLineupScRsp = 795,
    // @@protoc_insertion_point(enum_value:CmdLineupType.CmdSetLineupNameCsReq)
    CmdSetLineupNameCsReq = 719,
    // @@protoc_insertion_point(enum_value:CmdLineupType.CmdJoinLineupCsReq)
    CmdJoinLineupCsReq = 767,
    // @@protoc_insertion_point(enum_value:CmdLineupType.CmdReplaceLineupCsReq)
    CmdReplaceLineupCsReq = 759,
    // @@protoc_insertion_point(enum_value:CmdLineupType.CmdSwapLineupCsReq)
    CmdSwapLineupCsReq = 724,
    // @@protoc_insertion_point(enum_value:CmdLineupType.CmdExtraLineupDestroyNotify)
    CmdExtraLineupDestroyNotify = 778,
    // @@protoc_insertion_point(enum_value:CmdLineupType.CmdVirtualLineupDestroyNotify)
    CmdVirtualLineupDestroyNotify = 790,
    // @@protoc_insertion_point(enum_value:CmdLineupType.CmdGetAllLineupDataCsReq)
    CmdGetAllLineupDataCsReq = 707,
    // @@protoc_insertion_point(enum_value:CmdLineupType.CmdSyncLineupNotify)
    CmdSyncLineupNotify = 734,
    // @@protoc_insertion_point(enum_value:CmdLineupType.CmdGetAllLineupDataScRsp)
    CmdGetAllLineupDataScRsp = 753,
    // @@protoc_insertion_point(enum_value:CmdLineupType.CmdGetCurLineupDataScRsp)
    CmdGetCurLineupDataScRsp = 727,
    // @@protoc_insertion_point(enum_value:CmdLineupType.CmdVirtualLineupTrialAvatarChangeScNotify)
    CmdVirtualLineupTrialAvatarChangeScNotify = 729,
}

impl ::protobuf::Enum for CmdLineupType {
    const NAME: &'static str = "CmdLineupType";

    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<CmdLineupType> {
        match value {
            0 => ::std::option::Option::Some(CmdLineupType::CmdLineupTypeNone),
            752 => ::std::option::Option::Some(CmdLineupType::CmdQuitLineupCsReq),
            746 => ::std::option::Option::Some(CmdLineupType::CmdGetLineupAvatarDataScRsp),
            793 => ::std::option::Option::Some(CmdLineupType::CmdSwapLineupScRsp),
            775 => ::std::option::Option::Some(CmdLineupType::CmdSwitchLineupIndexScRsp),
            743 => ::std::option::Option::Some(CmdLineupType::CmdGetLineupAvatarDataCsReq),
            728 => ::std::option::Option::Some(CmdLineupType::CmdJoinLineupScRsp),
            726 => ::std::option::Option::Some(CmdLineupType::CmdSetLineupNameScRsp),
            784 => ::std::option::Option::Some(CmdLineupType::CmdGetCurLineupDataCsReq),
            725 => ::std::option::Option::Some(CmdLineupType::CmdChangeLineupLeaderCsReq),
            796 => ::std::option::Option::Some(CmdLineupType::CmdChangeLineupLeaderScRsp),
            705 => ::std::option::Option::Some(CmdLineupType::CmdSwitchLineupIndexCsReq),
            774 => ::std::option::Option::Some(CmdLineupType::CmdQuitLineupScRsp),
            736 => ::std::option::Option::Some(CmdLineupType::CmdGetStageLineupCsReq),
            712 => ::std::option::Option::Some(CmdLineupType::CmdReplaceLineupScRsp),
            795 => ::std::option::Option::Some(CmdLineupType::CmdGetStageLineupScRsp),
            719 => ::std::option::Option::Some(CmdLineupType::CmdSetLineupNameCsReq),
            767 => ::std::option::Option::Some(CmdLineupType::CmdJoinLineupCsReq),
            759 => ::std::option::Option::Some(CmdLineupType::CmdReplaceLineupCsReq),
            724 => ::std::option::Option::Some(CmdLineupType::CmdSwapLineupCsReq),
            778 => ::std::option::Option::Some(CmdLineupType::CmdExtraLineupDestroyNotify),
            790 => ::std::option::Option::Some(CmdLineupType::CmdVirtualLineupDestroyNotify),
            707 => ::std::option::Option::Some(CmdLineupType::CmdGetAllLineupDataCsReq),
            734 => ::std::option::Option::Some(CmdLineupType::CmdSyncLineupNotify),
            753 => ::std::option::Option::Some(CmdLineupType::CmdGetAllLineupDataScRsp),
            727 => ::std::option::Option::Some(CmdLineupType::CmdGetCurLineupDataScRsp),
            729 => ::std::option::Option::Some(CmdLineupType::CmdVirtualLineupTrialAvatarChangeScNotify),
            _ => ::std::option::Option::None
        }
    }

    fn from_str(str: &str) -> ::std::option::Option<CmdLineupType> {
        match str {
            "CmdLineupTypeNone" => ::std::option::Option::Some(CmdLineupType::CmdLineupTypeNone),
            "CmdQuitLineupCsReq" => ::std::option::Option::Some(CmdLineupType::CmdQuitLineupCsReq),
            "CmdGetLineupAvatarDataScRsp" => ::std::option::Option::Some(CmdLineupType::CmdGetLineupAvatarDataScRsp),
            "CmdSwapLineupScRsp" => ::std::option::Option::Some(CmdLineupType::CmdSwapLineupScRsp),
            "CmdSwitchLineupIndexScRsp" => ::std::option::Option::Some(CmdLineupType::CmdSwitchLineupIndexScRsp),
            "CmdGetLineupAvatarDataCsReq" => ::std::option::Option::Some(CmdLineupType::CmdGetLineupAvatarDataCsReq),
            "CmdJoinLineupScRsp" => ::std::option::Option::Some(CmdLineupType::CmdJoinLineupScRsp),
            "CmdSetLineupNameScRsp" => ::std::option::Option::Some(CmdLineupType::CmdSetLineupNameScRsp),
            "CmdGetCurLineupDataCsReq" => ::std::option::Option::Some(CmdLineupType::CmdGetCurLineupDataCsReq),
            "CmdChangeLineupLeaderCsReq" => ::std::option::Option::Some(CmdLineupType::CmdChangeLineupLeaderCsReq),
            "CmdChangeLineupLeaderScRsp" => ::std::option::Option::Some(CmdLineupType::CmdChangeLineupLeaderScRsp),
            "CmdSwitchLineupIndexCsReq" => ::std::option::Option::Some(CmdLineupType::CmdSwitchLineupIndexCsReq),
            "CmdQuitLineupScRsp" => ::std::option::Option::Some(CmdLineupType::CmdQuitLineupScRsp),
            "CmdGetStageLineupCsReq" => ::std::option::Option::Some(CmdLineupType::CmdGetStageLineupCsReq),
            "CmdReplaceLineupScRsp" => ::std::option::Option::Some(CmdLineupType::CmdReplaceLineupScRsp),
            "CmdGetStageLineupScRsp" => ::std::option::Option::Some(CmdLineupType::CmdGetStageLineupScRsp),
            "CmdSetLineupNameCsReq" => ::std::option::Option::Some(CmdLineupType::CmdSetLineupNameCsReq),
            "CmdJoinLineupCsReq" => ::std::option::Option::Some(CmdLineupType::CmdJoinLineupCsReq),
            "CmdReplaceLineupCsReq" => ::std::option::Option::Some(CmdLineupType::CmdReplaceLineupCsReq),
            "CmdSwapLineupCsReq" => ::std::option::Option::Some(CmdLineupType::CmdSwapLineupCsReq),
            "CmdExtraLineupDestroyNotify" => ::std::option::Option::Some(CmdLineupType::CmdExtraLineupDestroyNotify),
            "CmdVirtualLineupDestroyNotify" => ::std::option::Option::Some(CmdLineupType::CmdVirtualLineupDestroyNotify),
            "CmdGetAllLineupDataCsReq" => ::std::option::Option::Some(CmdLineupType::CmdGetAllLineupDataCsReq),
            "CmdSyncLineupNotify" => ::std::option::Option::Some(CmdLineupType::CmdSyncLineupNotify),
            "CmdGetAllLineupDataScRsp" => ::std::option::Option::Some(CmdLineupType::CmdGetAllLineupDataScRsp),
            "CmdGetCurLineupDataScRsp" => ::std::option::Option::Some(CmdLineupType::CmdGetCurLineupDataScRsp),
            "CmdVirtualLineupTrialAvatarChangeScNotify" => ::std::option::Option::Some(CmdLineupType::CmdVirtualLineupTrialAvatarChangeScNotify),
            _ => ::std::option::Option::None
        }
    }

    const VALUES: &'static [CmdLineupType] = &[
        CmdLineupType::CmdLineupTypeNone,
        CmdLineupType::CmdQuitLineupCsReq,
        CmdLineupType::CmdGetLineupAvatarDataScRsp,
        CmdLineupType::CmdSwapLineupScRsp,
        CmdLineupType::CmdSwitchLineupIndexScRsp,
        CmdLineupType::CmdGetLineupAvatarDataCsReq,
        CmdLineupType::CmdJoinLineupScRsp,
        CmdLineupType::CmdSetLineupNameScRsp,
        CmdLineupType::CmdGetCurLineupDataCsReq,
        CmdLineupType::CmdChangeLineupLeaderCsReq,
        CmdLineupType::CmdChangeLineupLeaderScRsp,
        CmdLineupType::CmdSwitchLineupIndexCsReq,
        CmdLineupType::CmdQuitLineupScRsp,
        CmdLineupType::CmdGetStageLineupCsReq,
        CmdLineupType::CmdReplaceLineupScRsp,
        CmdLineupType::CmdGetStageLineupScRsp,
        CmdLineupType::CmdSetLineupNameCsReq,
        CmdLineupType::CmdJoinLineupCsReq,
        CmdLineupType::CmdReplaceLineupCsReq,
        CmdLineupType::CmdSwapLineupCsReq,
        CmdLineupType::CmdExtraLineupDestroyNotify,
        CmdLineupType::CmdVirtualLineupDestroyNotify,
        CmdLineupType::CmdGetAllLineupDataCsReq,
        CmdLineupType::CmdSyncLineupNotify,
        CmdLineupType::CmdGetAllLineupDataScRsp,
        CmdLineupType::CmdGetCurLineupDataScRsp,
        CmdLineupType::CmdVirtualLineupTrialAvatarChangeScNotify,
    ];
}

impl ::protobuf::EnumFull for CmdLineupType {
    fn enum_descriptor() -> ::protobuf::reflect::EnumDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().enum_by_package_relative_name("CmdLineupType").unwrap()).clone()
    }

    fn descriptor(&self) -> ::protobuf::reflect::EnumValueDescriptor {
        let index = match self {
            CmdLineupType::CmdLineupTypeNone => 0,
            CmdLineupType::CmdQuitLineupCsReq => 1,
            CmdLineupType::CmdGetLineupAvatarDataScRsp => 2,
            CmdLineupType::CmdSwapLineupScRsp => 3,
            CmdLineupType::CmdSwitchLineupIndexScRsp => 4,
            CmdLineupType::CmdGetLineupAvatarDataCsReq => 5,
            CmdLineupType::CmdJoinLineupScRsp => 6,
            CmdLineupType::CmdSetLineupNameScRsp => 7,
            CmdLineupType::CmdGetCurLineupDataCsReq => 8,
            CmdLineupType::CmdChangeLineupLeaderCsReq => 9,
            CmdLineupType::CmdChangeLineupLeaderScRsp => 10,
            CmdLineupType::CmdSwitchLineupIndexCsReq => 11,
            CmdLineupType::CmdQuitLineupScRsp => 12,
            CmdLineupType::CmdGetStageLineupCsReq => 13,
            CmdLineupType::CmdReplaceLineupScRsp => 14,
            CmdLineupType::CmdGetStageLineupScRsp => 15,
            CmdLineupType::CmdSetLineupNameCsReq => 16,
            CmdLineupType::CmdJoinLineupCsReq => 17,
            CmdLineupType::CmdReplaceLineupCsReq => 18,
            CmdLineupType::CmdSwapLineupCsReq => 19,
            CmdLineupType::CmdExtraLineupDestroyNotify => 20,
            CmdLineupType::CmdVirtualLineupDestroyNotify => 21,
            CmdLineupType::CmdGetAllLineupDataCsReq => 22,
            CmdLineupType::CmdSyncLineupNotify => 23,
            CmdLineupType::CmdGetAllLineupDataScRsp => 24,
            CmdLineupType::CmdGetCurLineupDataScRsp => 25,
            CmdLineupType::CmdVirtualLineupTrialAvatarChangeScNotify => 26,
        };
        Self::enum_descriptor().value_by_index(index)
    }
}

impl ::std::default::Default for CmdLineupType {
    fn default() -> Self {
        CmdLineupType::CmdLineupTypeNone
    }
}

impl CmdLineupType {
    fn generated_enum_descriptor_data() -> ::protobuf::reflect::GeneratedEnumDescriptorData {
        ::protobuf::reflect::GeneratedEnumDescriptorData::new::<CmdLineupType>("CmdLineupType")
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x13CmdLineupType.proto*\xb8\x06\n\rCmdLineupType\x12\x15\n\x11CmdLine\
    upTypeNone\x10\0\x12\x17\n\x12CmdQuitLineupCsReq\x10\xf0\x05\x12\x20\n\
    \x1bCmdGetLineupAvatarDataScRsp\x10\xea\x05\x12\x17\n\x12CmdSwapLineupSc\
    Rsp\x10\x99\x06\x12\x1e\n\x19CmdSwitchLineupIndexScRsp\x10\x87\x06\x12\
    \x20\n\x1bCmdGetLineupAvatarDataCsReq\x10\xe7\x05\x12\x17\n\x12CmdJoinLi\
    neupScRsp\x10\xd8\x05\x12\x1a\n\x15CmdSetLineupNameScRsp\x10\xd6\x05\x12\
    \x1d\n\x18CmdGetCurLineupDataCsReq\x10\x90\x06\x12\x1f\n\x1aCmdChangeLin\
    eupLeaderCsReq\x10\xd5\x05\x12\x1f\n\x1aCmdChangeLineupLeaderScRsp\x10\
    \x9c\x06\x12\x1e\n\x19CmdSwitchLineupIndexCsReq\x10\xc1\x05\x12\x17\n\
    \x12CmdQuitLineupScRsp\x10\x86\x06\x12\x1b\n\x16CmdGetStageLineupCsReq\
    \x10\xe0\x05\x12\x1a\n\x15CmdReplaceLineupScRsp\x10\xc8\x05\x12\x1b\n\
    \x16CmdGetStageLineupScRsp\x10\x9b\x06\x12\x1a\n\x15CmdSetLineupNameCsRe\
    q\x10\xcf\x05\x12\x17\n\x12CmdJoinLineupCsReq\x10\xff\x05\x12\x1a\n\x15C\
    mdReplaceLineupCsReq\x10\xf7\x05\x12\x17\n\x12CmdSwapLineupCsReq\x10\xd4\
    \x05\x12\x20\n\x1bCmdExtraLineupDestroyNotify\x10\x8a\x06\x12\"\n\x1dCmd\
    VirtualLineupDestroyNotify\x10\x96\x06\x12\x1d\n\x18CmdGetAllLineupDataC\
    sReq\x10\xc3\x05\x12\x18\n\x13CmdSyncLineupNotify\x10\xde\x05\x12\x1d\n\
    \x18CmdGetAllLineupDataScRsp\x10\xf1\x05\x12\x1d\n\x18CmdGetCurLineupDat\
    aScRsp\x10\xd7\x05\x12.\n)CmdVirtualLineupTrialAvatarChangeScNotify\x10\
    \xd9\x05b\x06proto3\
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
            enums.push(CmdLineupType::generated_enum_descriptor_data());
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
