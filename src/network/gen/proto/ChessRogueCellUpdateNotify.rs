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

//! Generated file from `ChessRogueCellUpdateNotify.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:ChessRogueCellUpdateNotify)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct ChessRogueCellUpdateNotify {
    // message fields
    // @@protoc_insertion_point(field:ChessRogueCellUpdateNotify.JNPGEBHAOHL)
    pub JNPGEBHAOHL: ::protobuf::EnumOrUnknown<super::RogueModifierSourceType::RogueModifierSourceType>,
    // @@protoc_insertion_point(field:ChessRogueCellUpdateNotify.OLDKAMACFMD)
    pub OLDKAMACFMD: ::protobuf::EnumOrUnknown<super::FAPFDNFCCIH::FAPFDNFCCIH>,
    // @@protoc_insertion_point(field:ChessRogueCellUpdateNotify.HEDDDEKOOLB)
    pub HEDDDEKOOLB: u32,
    // @@protoc_insertion_point(field:ChessRogueCellUpdateNotify.BODGIILMDPN)
    pub BODGIILMDPN: ::std::vec::Vec<super::CJHPIAMGBBI::CJHPIAMGBBI>,
    // special fields
    // @@protoc_insertion_point(special_field:ChessRogueCellUpdateNotify.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a ChessRogueCellUpdateNotify {
    fn default() -> &'a ChessRogueCellUpdateNotify {
        <ChessRogueCellUpdateNotify as ::protobuf::Message>::default_instance()
    }
}

impl ChessRogueCellUpdateNotify {
    pub fn new() -> ChessRogueCellUpdateNotify {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(4);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "JNPGEBHAOHL",
            |m: &ChessRogueCellUpdateNotify| { &m.JNPGEBHAOHL },
            |m: &mut ChessRogueCellUpdateNotify| { &mut m.JNPGEBHAOHL },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "OLDKAMACFMD",
            |m: &ChessRogueCellUpdateNotify| { &m.OLDKAMACFMD },
            |m: &mut ChessRogueCellUpdateNotify| { &mut m.OLDKAMACFMD },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "HEDDDEKOOLB",
            |m: &ChessRogueCellUpdateNotify| { &m.HEDDDEKOOLB },
            |m: &mut ChessRogueCellUpdateNotify| { &mut m.HEDDDEKOOLB },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "BODGIILMDPN",
            |m: &ChessRogueCellUpdateNotify| { &m.BODGIILMDPN },
            |m: &mut ChessRogueCellUpdateNotify| { &mut m.BODGIILMDPN },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<ChessRogueCellUpdateNotify>(
            "ChessRogueCellUpdateNotify",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for ChessRogueCellUpdateNotify {
    const NAME: &'static str = "ChessRogueCellUpdateNotify";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                120 => {
                    self.JNPGEBHAOHL = is.read_enum_or_unknown()?;
                },
                88 => {
                    self.OLDKAMACFMD = is.read_enum_or_unknown()?;
                },
                56 => {
                    self.HEDDDEKOOLB = is.read_uint32()?;
                },
                66 => {
                    self.BODGIILMDPN.push(is.read_message()?);
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
        if self.JNPGEBHAOHL != ::protobuf::EnumOrUnknown::new(super::RogueModifierSourceType::RogueModifierSourceType::ROGUE_MODIFIER_SOURCE_NONE) {
            my_size += ::protobuf::rt::int32_size(15, self.JNPGEBHAOHL.value());
        }
        if self.OLDKAMACFMD != ::protobuf::EnumOrUnknown::new(super::FAPFDNFCCIH::FAPFDNFCCIH::CHESS_ROGUE_CELL_UPDATE_REASON_NONE) {
            my_size += ::protobuf::rt::int32_size(11, self.OLDKAMACFMD.value());
        }
        if self.HEDDDEKOOLB != 0 {
            my_size += ::protobuf::rt::uint32_size(7, self.HEDDDEKOOLB);
        }
        for value in &self.BODGIILMDPN {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.JNPGEBHAOHL != ::protobuf::EnumOrUnknown::new(super::RogueModifierSourceType::RogueModifierSourceType::ROGUE_MODIFIER_SOURCE_NONE) {
            os.write_enum(15, ::protobuf::EnumOrUnknown::value(&self.JNPGEBHAOHL))?;
        }
        if self.OLDKAMACFMD != ::protobuf::EnumOrUnknown::new(super::FAPFDNFCCIH::FAPFDNFCCIH::CHESS_ROGUE_CELL_UPDATE_REASON_NONE) {
            os.write_enum(11, ::protobuf::EnumOrUnknown::value(&self.OLDKAMACFMD))?;
        }
        if self.HEDDDEKOOLB != 0 {
            os.write_uint32(7, self.HEDDDEKOOLB)?;
        }
        for v in &self.BODGIILMDPN {
            ::protobuf::rt::write_message_field_with_cached_size(8, v, os)?;
        };
        os.write_unknown_fields(self.special_fields.unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn special_fields(&self) -> &::protobuf::SpecialFields {
        &self.special_fields
    }

    fn mut_special_fields(&mut self) -> &mut ::protobuf::SpecialFields {
        &mut self.special_fields
    }

    fn new() -> ChessRogueCellUpdateNotify {
        ChessRogueCellUpdateNotify::new()
    }

    fn clear(&mut self) {
        self.JNPGEBHAOHL = ::protobuf::EnumOrUnknown::new(super::RogueModifierSourceType::RogueModifierSourceType::ROGUE_MODIFIER_SOURCE_NONE);
        self.OLDKAMACFMD = ::protobuf::EnumOrUnknown::new(super::FAPFDNFCCIH::FAPFDNFCCIH::CHESS_ROGUE_CELL_UPDATE_REASON_NONE);
        self.HEDDDEKOOLB = 0;
        self.BODGIILMDPN.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static ChessRogueCellUpdateNotify {
        static instance: ChessRogueCellUpdateNotify = ChessRogueCellUpdateNotify {
            JNPGEBHAOHL: ::protobuf::EnumOrUnknown::from_i32(0),
            OLDKAMACFMD: ::protobuf::EnumOrUnknown::from_i32(0),
            HEDDDEKOOLB: 0,
            BODGIILMDPN: ::std::vec::Vec::new(),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for ChessRogueCellUpdateNotify {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("ChessRogueCellUpdateNotify").unwrap()).clone()
    }
}

impl ::std::fmt::Display for ChessRogueCellUpdateNotify {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ChessRogueCellUpdateNotify {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x20ChessRogueCellUpdateNotify.proto\x1a\x11CJHPIAMGBBI.proto\x1a\x11F\
    APFDNFCCIH.proto\x1a\x1dRogueModifierSourceType.proto\"\xda\x01\n\x1aChe\
    ssRogueCellUpdateNotify\x12:\n\x0bJNPGEBHAOHL\x18\x0f\x20\x01(\x0e2\x18.\
    RogueModifierSourceTypeR\x0bJNPGEBHAOHL\x12.\n\x0bOLDKAMACFMD\x18\x0b\
    \x20\x01(\x0e2\x0c.FAPFDNFCCIHR\x0bOLDKAMACFMD\x12\x20\n\x0bHEDDDEKOOLB\
    \x18\x07\x20\x01(\rR\x0bHEDDDEKOOLB\x12.\n\x0bBODGIILMDPN\x18\x08\x20\
    \x03(\x0b2\x0c.CJHPIAMGBBIR\x0bBODGIILMDPNb\x06proto3\
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
            let mut deps = ::std::vec::Vec::with_capacity(3);
            deps.push(super::CJHPIAMGBBI::file_descriptor().clone());
            deps.push(super::FAPFDNFCCIH::file_descriptor().clone());
            deps.push(super::RogueModifierSourceType::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(ChessRogueCellUpdateNotify::generated_message_descriptor_data());
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
