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

//! Generated file from `KLMFMNOHBLK.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:KLMFMNOHBLK)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct KLMFMNOHBLK {
    // message fields
    // @@protoc_insertion_point(field:KLMFMNOHBLK.FGJFKKFHHKP)
    pub FGJFKKFHHKP: u32,
    // @@protoc_insertion_point(field:KLMFMNOHBLK.NEFMHNPEOAG)
    pub NEFMHNPEOAG: u32,
    // @@protoc_insertion_point(field:KLMFMNOHBLK.LJDBKCBFPPC)
    pub LJDBKCBFPPC: u32,
    // special fields
    // @@protoc_insertion_point(special_field:KLMFMNOHBLK.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a KLMFMNOHBLK {
    fn default() -> &'a KLMFMNOHBLK {
        <KLMFMNOHBLK as ::protobuf::Message>::default_instance()
    }
}

impl KLMFMNOHBLK {
    pub fn new() -> KLMFMNOHBLK {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(3);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "FGJFKKFHHKP",
            |m: &KLMFMNOHBLK| { &m.FGJFKKFHHKP },
            |m: &mut KLMFMNOHBLK| { &mut m.FGJFKKFHHKP },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "NEFMHNPEOAG",
            |m: &KLMFMNOHBLK| { &m.NEFMHNPEOAG },
            |m: &mut KLMFMNOHBLK| { &mut m.NEFMHNPEOAG },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "LJDBKCBFPPC",
            |m: &KLMFMNOHBLK| { &m.LJDBKCBFPPC },
            |m: &mut KLMFMNOHBLK| { &mut m.LJDBKCBFPPC },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<KLMFMNOHBLK>(
            "KLMFMNOHBLK",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for KLMFMNOHBLK {
    const NAME: &'static str = "KLMFMNOHBLK";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                72 => {
                    self.FGJFKKFHHKP = is.read_uint32()?;
                },
                64 => {
                    self.NEFMHNPEOAG = is.read_uint32()?;
                },
                96 => {
                    self.LJDBKCBFPPC = is.read_uint32()?;
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
        if self.FGJFKKFHHKP != 0 {
            my_size += ::protobuf::rt::uint32_size(9, self.FGJFKKFHHKP);
        }
        if self.NEFMHNPEOAG != 0 {
            my_size += ::protobuf::rt::uint32_size(8, self.NEFMHNPEOAG);
        }
        if self.LJDBKCBFPPC != 0 {
            my_size += ::protobuf::rt::uint32_size(12, self.LJDBKCBFPPC);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.FGJFKKFHHKP != 0 {
            os.write_uint32(9, self.FGJFKKFHHKP)?;
        }
        if self.NEFMHNPEOAG != 0 {
            os.write_uint32(8, self.NEFMHNPEOAG)?;
        }
        if self.LJDBKCBFPPC != 0 {
            os.write_uint32(12, self.LJDBKCBFPPC)?;
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

    fn new() -> KLMFMNOHBLK {
        KLMFMNOHBLK::new()
    }

    fn clear(&mut self) {
        self.FGJFKKFHHKP = 0;
        self.NEFMHNPEOAG = 0;
        self.LJDBKCBFPPC = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static KLMFMNOHBLK {
        static instance: KLMFMNOHBLK = KLMFMNOHBLK {
            FGJFKKFHHKP: 0,
            NEFMHNPEOAG: 0,
            LJDBKCBFPPC: 0,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for KLMFMNOHBLK {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("KLMFMNOHBLK").unwrap()).clone()
    }
}

impl ::std::fmt::Display for KLMFMNOHBLK {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for KLMFMNOHBLK {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x11KLMFMNOHBLK.proto\"s\n\x0bKLMFMNOHBLK\x12\x20\n\x0bFGJFKKFHHKP\x18\
    \t\x20\x01(\rR\x0bFGJFKKFHHKP\x12\x20\n\x0bNEFMHNPEOAG\x18\x08\x20\x01(\
    \rR\x0bNEFMHNPEOAG\x12\x20\n\x0bLJDBKCBFPPC\x18\x0c\x20\x01(\rR\x0bLJDBK\
    CBFPPCb\x06proto3\
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
            messages.push(KLMFMNOHBLK::generated_message_descriptor_data());
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
