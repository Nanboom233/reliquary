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

//! Generated file from `FinishTalkMissionCsReq.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:FinishTalkMissionCsReq)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct FinishTalkMissionCsReq {
    // message fields
    // @@protoc_insertion_point(field:FinishTalkMissionCsReq.INDIGLPDHNC)
    pub INDIGLPDHNC: u32,
    // @@protoc_insertion_point(field:FinishTalkMissionCsReq.MAIDCIPBIFL)
    pub MAIDCIPBIFL: ::std::string::String,
    // @@protoc_insertion_point(field:FinishTalkMissionCsReq.EADMKHLGEMI)
    pub EADMKHLGEMI: ::std::vec::Vec<super::JMHLFLNBDEP::JMHLFLNBDEP>,
    // special fields
    // @@protoc_insertion_point(special_field:FinishTalkMissionCsReq.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a FinishTalkMissionCsReq {
    fn default() -> &'a FinishTalkMissionCsReq {
        <FinishTalkMissionCsReq as ::protobuf::Message>::default_instance()
    }
}

impl FinishTalkMissionCsReq {
    pub fn new() -> FinishTalkMissionCsReq {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(3);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "INDIGLPDHNC",
            |m: &FinishTalkMissionCsReq| { &m.INDIGLPDHNC },
            |m: &mut FinishTalkMissionCsReq| { &mut m.INDIGLPDHNC },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "MAIDCIPBIFL",
            |m: &FinishTalkMissionCsReq| { &m.MAIDCIPBIFL },
            |m: &mut FinishTalkMissionCsReq| { &mut m.MAIDCIPBIFL },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "EADMKHLGEMI",
            |m: &FinishTalkMissionCsReq| { &m.EADMKHLGEMI },
            |m: &mut FinishTalkMissionCsReq| { &mut m.EADMKHLGEMI },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<FinishTalkMissionCsReq>(
            "FinishTalkMissionCsReq",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for FinishTalkMissionCsReq {
    const NAME: &'static str = "FinishTalkMissionCsReq";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                48 => {
                    self.INDIGLPDHNC = is.read_uint32()?;
                },
                26 => {
                    self.MAIDCIPBIFL = is.read_string()?;
                },
                66 => {
                    self.EADMKHLGEMI.push(is.read_message()?);
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
        if self.INDIGLPDHNC != 0 {
            my_size += ::protobuf::rt::uint32_size(6, self.INDIGLPDHNC);
        }
        if !self.MAIDCIPBIFL.is_empty() {
            my_size += ::protobuf::rt::string_size(3, &self.MAIDCIPBIFL);
        }
        for value in &self.EADMKHLGEMI {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.INDIGLPDHNC != 0 {
            os.write_uint32(6, self.INDIGLPDHNC)?;
        }
        if !self.MAIDCIPBIFL.is_empty() {
            os.write_string(3, &self.MAIDCIPBIFL)?;
        }
        for v in &self.EADMKHLGEMI {
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

    fn new() -> FinishTalkMissionCsReq {
        FinishTalkMissionCsReq::new()
    }

    fn clear(&mut self) {
        self.INDIGLPDHNC = 0;
        self.MAIDCIPBIFL.clear();
        self.EADMKHLGEMI.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static FinishTalkMissionCsReq {
        static instance: FinishTalkMissionCsReq = FinishTalkMissionCsReq {
            INDIGLPDHNC: 0,
            MAIDCIPBIFL: ::std::string::String::new(),
            EADMKHLGEMI: ::std::vec::Vec::new(),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for FinishTalkMissionCsReq {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("FinishTalkMissionCsReq").unwrap()).clone()
    }
}

impl ::std::fmt::Display for FinishTalkMissionCsReq {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for FinishTalkMissionCsReq {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x1cFinishTalkMissionCsReq.proto\x1a\x11JMHLFLNBDEP.proto\"\x8c\x01\n\
    \x16FinishTalkMissionCsReq\x12\x20\n\x0bINDIGLPDHNC\x18\x06\x20\x01(\rR\
    \x0bINDIGLPDHNC\x12\x20\n\x0bMAIDCIPBIFL\x18\x03\x20\x01(\tR\x0bMAIDCIPB\
    IFL\x12.\n\x0bEADMKHLGEMI\x18\x08\x20\x03(\x0b2\x0c.JMHLFLNBDEPR\x0bEADM\
    KHLGEMIb\x06proto3\
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
            let mut deps = ::std::vec::Vec::with_capacity(1);
            deps.push(super::JMHLFLNBDEP::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(FinishTalkMissionCsReq::generated_message_descriptor_data());
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
