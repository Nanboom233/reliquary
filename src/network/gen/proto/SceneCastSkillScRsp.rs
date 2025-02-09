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

//! Generated file from `SceneCastSkillScRsp.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:SceneCastSkillScRsp)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct SceneCastSkillScRsp {
    // message fields
    // @@protoc_insertion_point(field:SceneCastSkillScRsp.LPPMFJOCKNO)
    pub LPPMFJOCKNO: ::protobuf::MessageField<super::DDCPCKIHMEF::DDCPCKIHMEF>,
    // @@protoc_insertion_point(field:SceneCastSkillScRsp.OPGGEDKDIEI)
    pub OPGGEDKDIEI: u32,
    // @@protoc_insertion_point(field:SceneCastSkillScRsp.retcode)
    pub retcode: u32,
    // @@protoc_insertion_point(field:SceneCastSkillScRsp.EBKAEGBGIJI)
    pub EBKAEGBGIJI: ::std::vec::Vec<super::BONJKDEAHDP::BONJKDEAHDP>,
    // special fields
    // @@protoc_insertion_point(special_field:SceneCastSkillScRsp.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a SceneCastSkillScRsp {
    fn default() -> &'a SceneCastSkillScRsp {
        <SceneCastSkillScRsp as ::protobuf::Message>::default_instance()
    }
}

impl SceneCastSkillScRsp {
    pub fn new() -> SceneCastSkillScRsp {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(4);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::DDCPCKIHMEF::DDCPCKIHMEF>(
            "LPPMFJOCKNO",
            |m: &SceneCastSkillScRsp| { &m.LPPMFJOCKNO },
            |m: &mut SceneCastSkillScRsp| { &mut m.LPPMFJOCKNO },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "OPGGEDKDIEI",
            |m: &SceneCastSkillScRsp| { &m.OPGGEDKDIEI },
            |m: &mut SceneCastSkillScRsp| { &mut m.OPGGEDKDIEI },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "retcode",
            |m: &SceneCastSkillScRsp| { &m.retcode },
            |m: &mut SceneCastSkillScRsp| { &mut m.retcode },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "EBKAEGBGIJI",
            |m: &SceneCastSkillScRsp| { &m.EBKAEGBGIJI },
            |m: &mut SceneCastSkillScRsp| { &mut m.EBKAEGBGIJI },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<SceneCastSkillScRsp>(
            "SceneCastSkillScRsp",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for SceneCastSkillScRsp {
    const NAME: &'static str = "SceneCastSkillScRsp";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                42 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.LPPMFJOCKNO)?;
                },
                16 => {
                    self.OPGGEDKDIEI = is.read_uint32()?;
                },
                96 => {
                    self.retcode = is.read_uint32()?;
                },
                122 => {
                    self.EBKAEGBGIJI.push(is.read_message()?);
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
        if let Some(v) = self.LPPMFJOCKNO.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if self.OPGGEDKDIEI != 0 {
            my_size += ::protobuf::rt::uint32_size(2, self.OPGGEDKDIEI);
        }
        if self.retcode != 0 {
            my_size += ::protobuf::rt::uint32_size(12, self.retcode);
        }
        for value in &self.EBKAEGBGIJI {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if let Some(v) = self.LPPMFJOCKNO.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(5, v, os)?;
        }
        if self.OPGGEDKDIEI != 0 {
            os.write_uint32(2, self.OPGGEDKDIEI)?;
        }
        if self.retcode != 0 {
            os.write_uint32(12, self.retcode)?;
        }
        for v in &self.EBKAEGBGIJI {
            ::protobuf::rt::write_message_field_with_cached_size(15, v, os)?;
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

    fn new() -> SceneCastSkillScRsp {
        SceneCastSkillScRsp::new()
    }

    fn clear(&mut self) {
        self.LPPMFJOCKNO.clear();
        self.OPGGEDKDIEI = 0;
        self.retcode = 0;
        self.EBKAEGBGIJI.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static SceneCastSkillScRsp {
        static instance: SceneCastSkillScRsp = SceneCastSkillScRsp {
            LPPMFJOCKNO: ::protobuf::MessageField::none(),
            OPGGEDKDIEI: 0,
            retcode: 0,
            EBKAEGBGIJI: ::std::vec::Vec::new(),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for SceneCastSkillScRsp {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("SceneCastSkillScRsp").unwrap()).clone()
    }
}

impl ::std::fmt::Display for SceneCastSkillScRsp {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for SceneCastSkillScRsp {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x19SceneCastSkillScRsp.proto\x1a\x11BONJKDEAHDP.proto\x1a\x11DDCPCKIH\
    MEF.proto\"\xb1\x01\n\x13SceneCastSkillScRsp\x12.\n\x0bLPPMFJOCKNO\x18\
    \x05\x20\x01(\x0b2\x0c.DDCPCKIHMEFR\x0bLPPMFJOCKNO\x12\x20\n\x0bOPGGEDKD\
    IEI\x18\x02\x20\x01(\rR\x0bOPGGEDKDIEI\x12\x18\n\x07retcode\x18\x0c\x20\
    \x01(\rR\x07retcode\x12.\n\x0bEBKAEGBGIJI\x18\x0f\x20\x03(\x0b2\x0c.BONJ\
    KDEAHDPR\x0bEBKAEGBGIJIb\x06proto3\
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
            let mut deps = ::std::vec::Vec::with_capacity(2);
            deps.push(super::BONJKDEAHDP::file_descriptor().clone());
            deps.push(super::DDCPCKIHMEF::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(SceneCastSkillScRsp::generated_message_descriptor_data());
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
