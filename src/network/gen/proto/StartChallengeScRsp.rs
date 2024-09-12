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

//! Generated file from `StartChallengeScRsp.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:StartChallengeScRsp)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct StartChallengeScRsp {
    // message fields
    // @@protoc_insertion_point(field:StartChallengeScRsp.lineup_list)
    pub lineup_list: ::std::vec::Vec<super::LineupInfo::LineupInfo>,
    // @@protoc_insertion_point(field:StartChallengeScRsp.scene)
    pub scene: ::protobuf::MessageField<super::SceneInfo::SceneInfo>,
    // @@protoc_insertion_point(field:StartChallengeScRsp.challenge_info)
    pub challenge_info: ::protobuf::MessageField<super::ChallengeInfo::ChallengeInfo>,
    // @@protoc_insertion_point(field:StartChallengeScRsp.retcode)
    pub retcode: u32,
    // @@protoc_insertion_point(field:StartChallengeScRsp.ext_info)
    pub ext_info: ::protobuf::MessageField<super::ChallengeExtInfo::ChallengeExtInfo>,
    // special fields
    // @@protoc_insertion_point(special_field:StartChallengeScRsp.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a StartChallengeScRsp {
    fn default() -> &'a StartChallengeScRsp {
        <StartChallengeScRsp as ::protobuf::Message>::default_instance()
    }
}

impl StartChallengeScRsp {
    pub fn new() -> StartChallengeScRsp {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(5);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "lineup_list",
            |m: &StartChallengeScRsp| { &m.lineup_list },
            |m: &mut StartChallengeScRsp| { &mut m.lineup_list },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::SceneInfo::SceneInfo>(
            "scene",
            |m: &StartChallengeScRsp| { &m.scene },
            |m: &mut StartChallengeScRsp| { &mut m.scene },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::ChallengeInfo::ChallengeInfo>(
            "challenge_info",
            |m: &StartChallengeScRsp| { &m.challenge_info },
            |m: &mut StartChallengeScRsp| { &mut m.challenge_info },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "retcode",
            |m: &StartChallengeScRsp| { &m.retcode },
            |m: &mut StartChallengeScRsp| { &mut m.retcode },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::ChallengeExtInfo::ChallengeExtInfo>(
            "ext_info",
            |m: &StartChallengeScRsp| { &m.ext_info },
            |m: &mut StartChallengeScRsp| { &mut m.ext_info },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<StartChallengeScRsp>(
            "StartChallengeScRsp",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for StartChallengeScRsp {
    const NAME: &'static str = "StartChallengeScRsp";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                82 => {
                    self.lineup_list.push(is.read_message()?);
                },
                122 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.scene)?;
                },
                66 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.challenge_info)?;
                },
                48 => {
                    self.retcode = is.read_uint32()?;
                },
                34 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.ext_info)?;
                },
                tag => {
                    ::protobuf::rt::read_unknown_or_skip_group(tag, is, self.special_fields.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u64 {
        let mut my_size = 0;
        for value in &self.lineup_list {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        if let Some(v) = self.scene.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if let Some(v) = self.challenge_info.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if self.retcode != 0 {
            my_size += ::protobuf::rt::uint32_size(6, self.retcode);
        }
        if let Some(v) = self.ext_info.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        for v in &self.lineup_list {
            ::protobuf::rt::write_message_field_with_cached_size(10, v, os)?;
        };
        if let Some(v) = self.scene.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(15, v, os)?;
        }
        if let Some(v) = self.challenge_info.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(8, v, os)?;
        }
        if self.retcode != 0 {
            os.write_uint32(6, self.retcode)?;
        }
        if let Some(v) = self.ext_info.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(4, v, os)?;
        }
        os.write_unknown_fields(self.special_fields.unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn special_fields(&self) -> &::protobuf::SpecialFields {
        &self.special_fields
    }

    fn mut_special_fields(&mut self) -> &mut ::protobuf::SpecialFields {
        &mut self.special_fields
    }

    fn new() -> StartChallengeScRsp {
        StartChallengeScRsp::new()
    }

    fn clear(&mut self) {
        self.lineup_list.clear();
        self.scene.clear();
        self.challenge_info.clear();
        self.retcode = 0;
        self.ext_info.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static StartChallengeScRsp {
        static instance: StartChallengeScRsp = StartChallengeScRsp {
            lineup_list: ::std::vec::Vec::new(),
            scene: ::protobuf::MessageField::none(),
            challenge_info: ::protobuf::MessageField::none(),
            retcode: 0,
            ext_info: ::protobuf::MessageField::none(),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for StartChallengeScRsp {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("StartChallengeScRsp").unwrap()).clone()
    }
}

impl ::std::fmt::Display for StartChallengeScRsp {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for StartChallengeScRsp {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x19StartChallengeScRsp.proto\x1a\x13ChallengeInfo.proto\x1a\x16Challe\
    ngeExtInfo.proto\x1a\x10LineupInfo.proto\x1a\x0fSceneInfo.proto\"\xe4\
    \x01\n\x13StartChallengeScRsp\x12,\n\x0blineup_list\x18\n\x20\x03(\x0b2\
    \x0b.LineupInfoR\nlineupList\x12\x20\n\x05scene\x18\x0f\x20\x01(\x0b2\n.\
    SceneInfoR\x05scene\x125\n\x0echallenge_info\x18\x08\x20\x01(\x0b2\x0e.C\
    hallengeInfoR\rchallengeInfo\x12\x18\n\x07retcode\x18\x06\x20\x01(\rR\
    \x07retcode\x12,\n\x08ext_info\x18\x04\x20\x01(\x0b2\x11.ChallengeExtInf\
    oR\x07extInfoB\x15\n\x13emu.lunarcore.protob\x06proto3\
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
            let mut deps = ::std::vec::Vec::with_capacity(4);
            deps.push(super::ChallengeInfo::file_descriptor().clone());
            deps.push(super::ChallengeExtInfo::file_descriptor().clone());
            deps.push(super::LineupInfo::file_descriptor().clone());
            deps.push(super::SceneInfo::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(StartChallengeScRsp::generated_message_descriptor_data());
            let mut enums = ::std::vec::Vec::with_capacity(0);
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
