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

//! Generated file from `DBOKGOCMGGP.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:DBOKGOCMGGP)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct DBOKGOCMGGP {
    // message fields
    // @@protoc_insertion_point(field:DBOKGOCMGGP.DNNHNHABFIO)
    pub DNNHNHABFIO: u32,
    // @@protoc_insertion_point(field:DBOKGOCMGGP.EDFAHEKIFMM)
    pub EDFAHEKIFMM: bool,
    // special fields
    // @@protoc_insertion_point(special_field:DBOKGOCMGGP.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a DBOKGOCMGGP {
    fn default() -> &'a DBOKGOCMGGP {
        <DBOKGOCMGGP as ::protobuf::Message>::default_instance()
    }
}

impl DBOKGOCMGGP {
    pub fn new() -> DBOKGOCMGGP {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(2);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "DNNHNHABFIO",
            |m: &DBOKGOCMGGP| { &m.DNNHNHABFIO },
            |m: &mut DBOKGOCMGGP| { &mut m.DNNHNHABFIO },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "EDFAHEKIFMM",
            |m: &DBOKGOCMGGP| { &m.EDFAHEKIFMM },
            |m: &mut DBOKGOCMGGP| { &mut m.EDFAHEKIFMM },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<DBOKGOCMGGP>(
            "DBOKGOCMGGP",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for DBOKGOCMGGP {
    const NAME: &'static str = "DBOKGOCMGGP";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                96 => {
                    self.DNNHNHABFIO = is.read_uint32()?;
                },
                88 => {
                    self.EDFAHEKIFMM = is.read_bool()?;
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
        if self.DNNHNHABFIO != 0 {
            my_size += ::protobuf::rt::uint32_size(12, self.DNNHNHABFIO);
        }
        if self.EDFAHEKIFMM != false {
            my_size += 1 + 1;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.DNNHNHABFIO != 0 {
            os.write_uint32(12, self.DNNHNHABFIO)?;
        }
        if self.EDFAHEKIFMM != false {
            os.write_bool(11, self.EDFAHEKIFMM)?;
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

    fn new() -> DBOKGOCMGGP {
        DBOKGOCMGGP::new()
    }

    fn clear(&mut self) {
        self.DNNHNHABFIO = 0;
        self.EDFAHEKIFMM = false;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static DBOKGOCMGGP {
        static instance: DBOKGOCMGGP = DBOKGOCMGGP {
            DNNHNHABFIO: 0,
            EDFAHEKIFMM: false,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for DBOKGOCMGGP {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("DBOKGOCMGGP").unwrap()).clone()
    }
}

impl ::std::fmt::Display for DBOKGOCMGGP {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for DBOKGOCMGGP {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x11DBOKGOCMGGP.proto\"Q\n\x0bDBOKGOCMGGP\x12\x20\n\x0bDNNHNHABFIO\x18\
    \x0c\x20\x01(\rR\x0bDNNHNHABFIO\x12\x20\n\x0bEDFAHEKIFMM\x18\x0b\x20\x01\
    (\x08R\x0bEDFAHEKIFMMb\x06proto3\
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
            messages.push(DBOKGOCMGGP::generated_message_descriptor_data());
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
