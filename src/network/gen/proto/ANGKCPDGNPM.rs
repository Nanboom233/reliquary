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

//! Generated file from `ANGKCPDGNPM.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:ANGKCPDGNPM)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct ANGKCPDGNPM {
    // message fields
    // @@protoc_insertion_point(field:ANGKCPDGNPM.NLAHMOBFPIB)
    pub NLAHMOBFPIB: ::protobuf::MessageField<super::APOIBBGLIPM::APOIBBGLIPM>,
    // @@protoc_insertion_point(field:ANGKCPDGNPM.ECOKOGMFGAF)
    pub ECOKOGMFGAF: ::protobuf::EnumOrUnknown<super::PlayingState::PlayingState>,
    // @@protoc_insertion_point(field:ANGKCPDGNPM.is_marked)
    pub is_marked: bool,
    // @@protoc_insertion_point(field:ANGKCPDGNPM.NHFPIKLJBCI)
    pub NHFPIKLJBCI: ::protobuf::MessageField<super::DGCOHFIBHPK::DGCOHFIBHPK>,
    // @@protoc_insertion_point(field:ANGKCPDGNPM.MFJAGFLPKFO)
    pub MFJAGFLPKFO: i64,
    // @@protoc_insertion_point(field:ANGKCPDGNPM.JNJJBIGBGPA)
    pub JNJJBIGBGPA: ::std::string::String,
    // special fields
    // @@protoc_insertion_point(special_field:ANGKCPDGNPM.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a ANGKCPDGNPM {
    fn default() -> &'a ANGKCPDGNPM {
        <ANGKCPDGNPM as ::protobuf::Message>::default_instance()
    }
}

impl ANGKCPDGNPM {
    pub fn new() -> ANGKCPDGNPM {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(6);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::APOIBBGLIPM::APOIBBGLIPM>(
            "NLAHMOBFPIB",
            |m: &ANGKCPDGNPM| { &m.NLAHMOBFPIB },
            |m: &mut ANGKCPDGNPM| { &mut m.NLAHMOBFPIB },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "ECOKOGMFGAF",
            |m: &ANGKCPDGNPM| { &m.ECOKOGMFGAF },
            |m: &mut ANGKCPDGNPM| { &mut m.ECOKOGMFGAF },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "is_marked",
            |m: &ANGKCPDGNPM| { &m.is_marked },
            |m: &mut ANGKCPDGNPM| { &mut m.is_marked },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::DGCOHFIBHPK::DGCOHFIBHPK>(
            "NHFPIKLJBCI",
            |m: &ANGKCPDGNPM| { &m.NHFPIKLJBCI },
            |m: &mut ANGKCPDGNPM| { &mut m.NHFPIKLJBCI },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "MFJAGFLPKFO",
            |m: &ANGKCPDGNPM| { &m.MFJAGFLPKFO },
            |m: &mut ANGKCPDGNPM| { &mut m.MFJAGFLPKFO },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "JNJJBIGBGPA",
            |m: &ANGKCPDGNPM| { &m.JNJJBIGBGPA },
            |m: &mut ANGKCPDGNPM| { &mut m.JNJJBIGBGPA },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<ANGKCPDGNPM>(
            "ANGKCPDGNPM",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for ANGKCPDGNPM {
    const NAME: &'static str = "ANGKCPDGNPM";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                42 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.NLAHMOBFPIB)?;
                },
                120 => {
                    self.ECOKOGMFGAF = is.read_enum_or_unknown()?;
                },
                16 => {
                    self.is_marked = is.read_bool()?;
                },
                82 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.NHFPIKLJBCI)?;
                },
                88 => {
                    self.MFJAGFLPKFO = is.read_int64()?;
                },
                98 => {
                    self.JNJJBIGBGPA = is.read_string()?;
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
        if let Some(v) = self.NLAHMOBFPIB.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if self.ECOKOGMFGAF != ::protobuf::EnumOrUnknown::new(super::PlayingState::PlayingState::PLAYING_STATE_NONE) {
            my_size += ::protobuf::rt::int32_size(15, self.ECOKOGMFGAF.value());
        }
        if self.is_marked != false {
            my_size += 1 + 1;
        }
        if let Some(v) = self.NHFPIKLJBCI.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if self.MFJAGFLPKFO != 0 {
            my_size += ::protobuf::rt::int64_size(11, self.MFJAGFLPKFO);
        }
        if !self.JNJJBIGBGPA.is_empty() {
            my_size += ::protobuf::rt::string_size(12, &self.JNJJBIGBGPA);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if let Some(v) = self.NLAHMOBFPIB.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(5, v, os)?;
        }
        if self.ECOKOGMFGAF != ::protobuf::EnumOrUnknown::new(super::PlayingState::PlayingState::PLAYING_STATE_NONE) {
            os.write_enum(15, ::protobuf::EnumOrUnknown::value(&self.ECOKOGMFGAF))?;
        }
        if self.is_marked != false {
            os.write_bool(2, self.is_marked)?;
        }
        if let Some(v) = self.NHFPIKLJBCI.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(10, v, os)?;
        }
        if self.MFJAGFLPKFO != 0 {
            os.write_int64(11, self.MFJAGFLPKFO)?;
        }
        if !self.JNJJBIGBGPA.is_empty() {
            os.write_string(12, &self.JNJJBIGBGPA)?;
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

    fn new() -> ANGKCPDGNPM {
        ANGKCPDGNPM::new()
    }

    fn clear(&mut self) {
        self.NLAHMOBFPIB.clear();
        self.ECOKOGMFGAF = ::protobuf::EnumOrUnknown::new(super::PlayingState::PlayingState::PLAYING_STATE_NONE);
        self.is_marked = false;
        self.NHFPIKLJBCI.clear();
        self.MFJAGFLPKFO = 0;
        self.JNJJBIGBGPA.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static ANGKCPDGNPM {
        static instance: ANGKCPDGNPM = ANGKCPDGNPM {
            NLAHMOBFPIB: ::protobuf::MessageField::none(),
            ECOKOGMFGAF: ::protobuf::EnumOrUnknown::from_i32(0),
            is_marked: false,
            NHFPIKLJBCI: ::protobuf::MessageField::none(),
            MFJAGFLPKFO: 0,
            JNJJBIGBGPA: ::std::string::String::new(),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for ANGKCPDGNPM {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("ANGKCPDGNPM").unwrap()).clone()
    }
}

impl ::std::fmt::Display for ANGKCPDGNPM {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ANGKCPDGNPM {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x11ANGKCPDGNPM.proto\x1a\x11APOIBBGLIPM.proto\x1a\x11DGCOHFIBHPK.prot\
    o\x1a\x12PlayingState.proto\"\xff\x01\n\x0bANGKCPDGNPM\x12.\n\x0bNLAHMOB\
    FPIB\x18\x05\x20\x01(\x0b2\x0c.APOIBBGLIPMR\x0bNLAHMOBFPIB\x12/\n\x0bECO\
    KOGMFGAF\x18\x0f\x20\x01(\x0e2\r.PlayingStateR\x0bECOKOGMFGAF\x12\x1b\n\
    \tis_marked\x18\x02\x20\x01(\x08R\x08isMarked\x12.\n\x0bNHFPIKLJBCI\x18\
    \n\x20\x01(\x0b2\x0c.DGCOHFIBHPKR\x0bNHFPIKLJBCI\x12\x20\n\x0bMFJAGFLPKF\
    O\x18\x0b\x20\x01(\x03R\x0bMFJAGFLPKFO\x12\x20\n\x0bJNJJBIGBGPA\x18\x0c\
    \x20\x01(\tR\x0bJNJJBIGBGPAb\x06proto3\
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
            deps.push(super::APOIBBGLIPM::file_descriptor().clone());
            deps.push(super::DGCOHFIBHPK::file_descriptor().clone());
            deps.push(super::PlayingState::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(ANGKCPDGNPM::generated_message_descriptor_data());
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
