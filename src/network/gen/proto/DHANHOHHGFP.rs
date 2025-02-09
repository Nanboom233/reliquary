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

//! Generated file from `DHANHOHHGFP.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:DHANHOHHGFP)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct DHANHOHHGFP {
    // message fields
    // @@protoc_insertion_point(field:DHANHOHHGFP.EHJDKABPDBC)
    pub EHJDKABPDBC: u32,
    // @@protoc_insertion_point(field:DHANHOHHGFP.AAOHALKFFNM)
    pub AAOHALKFFNM: bool,
    // @@protoc_insertion_point(field:DHANHOHHGFP.BHBHLINIJPD)
    pub BHBHLINIJPD: u32,
    // special fields
    // @@protoc_insertion_point(special_field:DHANHOHHGFP.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a DHANHOHHGFP {
    fn default() -> &'a DHANHOHHGFP {
        <DHANHOHHGFP as ::protobuf::Message>::default_instance()
    }
}

impl DHANHOHHGFP {
    pub fn new() -> DHANHOHHGFP {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(3);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "EHJDKABPDBC",
            |m: &DHANHOHHGFP| { &m.EHJDKABPDBC },
            |m: &mut DHANHOHHGFP| { &mut m.EHJDKABPDBC },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "AAOHALKFFNM",
            |m: &DHANHOHHGFP| { &m.AAOHALKFFNM },
            |m: &mut DHANHOHHGFP| { &mut m.AAOHALKFFNM },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "BHBHLINIJPD",
            |m: &DHANHOHHGFP| { &m.BHBHLINIJPD },
            |m: &mut DHANHOHHGFP| { &mut m.BHBHLINIJPD },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<DHANHOHHGFP>(
            "DHANHOHHGFP",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for DHANHOHHGFP {
    const NAME: &'static str = "DHANHOHHGFP";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                64 => {
                    self.EHJDKABPDBC = is.read_uint32()?;
                },
                120 => {
                    self.AAOHALKFFNM = is.read_bool()?;
                },
                96 => {
                    self.BHBHLINIJPD = is.read_uint32()?;
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
        if self.EHJDKABPDBC != 0 {
            my_size += ::protobuf::rt::uint32_size(8, self.EHJDKABPDBC);
        }
        if self.AAOHALKFFNM != false {
            my_size += 1 + 1;
        }
        if self.BHBHLINIJPD != 0 {
            my_size += ::protobuf::rt::uint32_size(12, self.BHBHLINIJPD);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.EHJDKABPDBC != 0 {
            os.write_uint32(8, self.EHJDKABPDBC)?;
        }
        if self.AAOHALKFFNM != false {
            os.write_bool(15, self.AAOHALKFFNM)?;
        }
        if self.BHBHLINIJPD != 0 {
            os.write_uint32(12, self.BHBHLINIJPD)?;
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

    fn new() -> DHANHOHHGFP {
        DHANHOHHGFP::new()
    }

    fn clear(&mut self) {
        self.EHJDKABPDBC = 0;
        self.AAOHALKFFNM = false;
        self.BHBHLINIJPD = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static DHANHOHHGFP {
        static instance: DHANHOHHGFP = DHANHOHHGFP {
            EHJDKABPDBC: 0,
            AAOHALKFFNM: false,
            BHBHLINIJPD: 0,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for DHANHOHHGFP {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("DHANHOHHGFP").unwrap()).clone()
    }
}

impl ::std::fmt::Display for DHANHOHHGFP {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for DHANHOHHGFP {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x11DHANHOHHGFP.proto\"s\n\x0bDHANHOHHGFP\x12\x20\n\x0bEHJDKABPDBC\x18\
    \x08\x20\x01(\rR\x0bEHJDKABPDBC\x12\x20\n\x0bAAOHALKFFNM\x18\x0f\x20\x01\
    (\x08R\x0bAAOHALKFFNM\x12\x20\n\x0bBHBHLINIJPD\x18\x0c\x20\x01(\rR\x0bBH\
    BHLINIJPDb\x06proto3\
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
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(DHANHOHHGFP::generated_message_descriptor_data());
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
