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

//! Generated file from `DOKFNAEEADE.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:DOKFNAEEADE)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct DOKFNAEEADE {
    // message fields
    // @@protoc_insertion_point(field:DOKFNAEEADE.ONECNIKGBGL)
    pub ONECNIKGBGL: ::protobuf::MessageField<super::NADHCNJHLDA::NADHCNJHLDA>,
    // @@protoc_insertion_point(field:DOKFNAEEADE.MCHONACCDME)
    pub MCHONACCDME: ::std::vec::Vec<super::IPBDJPAJODO::IPBDJPAJODO>,
    // @@protoc_insertion_point(field:DOKFNAEEADE.BGOLDBNOPLI)
    pub BGOLDBNOPLI: u32,
    // special fields
    // @@protoc_insertion_point(special_field:DOKFNAEEADE.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a DOKFNAEEADE {
    fn default() -> &'a DOKFNAEEADE {
        <DOKFNAEEADE as ::protobuf::Message>::default_instance()
    }
}

impl DOKFNAEEADE {
    pub fn new() -> DOKFNAEEADE {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(3);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::NADHCNJHLDA::NADHCNJHLDA>(
            "ONECNIKGBGL",
            |m: &DOKFNAEEADE| { &m.ONECNIKGBGL },
            |m: &mut DOKFNAEEADE| { &mut m.ONECNIKGBGL },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "MCHONACCDME",
            |m: &DOKFNAEEADE| { &m.MCHONACCDME },
            |m: &mut DOKFNAEEADE| { &mut m.MCHONACCDME },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "BGOLDBNOPLI",
            |m: &DOKFNAEEADE| { &m.BGOLDBNOPLI },
            |m: &mut DOKFNAEEADE| { &mut m.BGOLDBNOPLI },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<DOKFNAEEADE>(
            "DOKFNAEEADE",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for DOKFNAEEADE {
    const NAME: &'static str = "DOKFNAEEADE";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                66 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.ONECNIKGBGL)?;
                },
                74 => {
                    self.MCHONACCDME.push(is.read_message()?);
                },
                104 => {
                    self.BGOLDBNOPLI = is.read_uint32()?;
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
        if let Some(v) = self.ONECNIKGBGL.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        for value in &self.MCHONACCDME {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        if self.BGOLDBNOPLI != 0 {
            my_size += ::protobuf::rt::uint32_size(13, self.BGOLDBNOPLI);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if let Some(v) = self.ONECNIKGBGL.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(8, v, os)?;
        }
        for v in &self.MCHONACCDME {
            ::protobuf::rt::write_message_field_with_cached_size(9, v, os)?;
        };
        if self.BGOLDBNOPLI != 0 {
            os.write_uint32(13, self.BGOLDBNOPLI)?;
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

    fn new() -> DOKFNAEEADE {
        DOKFNAEEADE::new()
    }

    fn clear(&mut self) {
        self.ONECNIKGBGL.clear();
        self.MCHONACCDME.clear();
        self.BGOLDBNOPLI = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static DOKFNAEEADE {
        static instance: DOKFNAEEADE = DOKFNAEEADE {
            ONECNIKGBGL: ::protobuf::MessageField::none(),
            MCHONACCDME: ::std::vec::Vec::new(),
            BGOLDBNOPLI: 0,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for DOKFNAEEADE {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("DOKFNAEEADE").unwrap()).clone()
    }
}

impl ::std::fmt::Display for DOKFNAEEADE {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for DOKFNAEEADE {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x11DOKFNAEEADE.proto\x1a\x11IPBDJPAJODO.proto\x1a\x11NADHCNJHLDA.prot\
    o\"\x8f\x01\n\x0bDOKFNAEEADE\x12.\n\x0bONECNIKGBGL\x18\x08\x20\x01(\x0b2\
    \x0c.NADHCNJHLDAR\x0bONECNIKGBGL\x12.\n\x0bMCHONACCDME\x18\t\x20\x03(\
    \x0b2\x0c.IPBDJPAJODOR\x0bMCHONACCDME\x12\x20\n\x0bBGOLDBNOPLI\x18\r\x20\
    \x01(\rR\x0bBGOLDBNOPLIb\x06proto3\
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
            deps.push(super::IPBDJPAJODO::file_descriptor().clone());
            deps.push(super::NADHCNJHLDA::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(DOKFNAEEADE::generated_message_descriptor_data());
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
