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

//! Generated file from `GCNAPNPOPEK.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:GCNAPNPOPEK)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct GCNAPNPOPEK {
    // message fields
    // @@protoc_insertion_point(field:GCNAPNPOPEK.level)
    pub level: u32,
    // @@protoc_insertion_point(field:GCNAPNPOPEK.JCCFFLHDAFM)
    pub JCCFFLHDAFM: u32,
    // @@protoc_insertion_point(field:GCNAPNPOPEK.OKMFNBOHJNL)
    pub OKMFNBOHJNL: u32,
    // @@protoc_insertion_point(field:GCNAPNPOPEK.NFIDKGHKPLC)
    pub NFIDKGHKPLC: bool,
    // special fields
    // @@protoc_insertion_point(special_field:GCNAPNPOPEK.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a GCNAPNPOPEK {
    fn default() -> &'a GCNAPNPOPEK {
        <GCNAPNPOPEK as ::protobuf::Message>::default_instance()
    }
}

impl GCNAPNPOPEK {
    pub fn new() -> GCNAPNPOPEK {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(4);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "level",
            |m: &GCNAPNPOPEK| { &m.level },
            |m: &mut GCNAPNPOPEK| { &mut m.level },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "JCCFFLHDAFM",
            |m: &GCNAPNPOPEK| { &m.JCCFFLHDAFM },
            |m: &mut GCNAPNPOPEK| { &mut m.JCCFFLHDAFM },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "OKMFNBOHJNL",
            |m: &GCNAPNPOPEK| { &m.OKMFNBOHJNL },
            |m: &mut GCNAPNPOPEK| { &mut m.OKMFNBOHJNL },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "NFIDKGHKPLC",
            |m: &GCNAPNPOPEK| { &m.NFIDKGHKPLC },
            |m: &mut GCNAPNPOPEK| { &mut m.NFIDKGHKPLC },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<GCNAPNPOPEK>(
            "GCNAPNPOPEK",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for GCNAPNPOPEK {
    const NAME: &'static str = "GCNAPNPOPEK";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                80 => {
                    self.level = is.read_uint32()?;
                },
                8 => {
                    self.JCCFFLHDAFM = is.read_uint32()?;
                },
                64 => {
                    self.OKMFNBOHJNL = is.read_uint32()?;
                },
                112 => {
                    self.NFIDKGHKPLC = is.read_bool()?;
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
        if self.level != 0 {
            my_size += ::protobuf::rt::uint32_size(10, self.level);
        }
        if self.JCCFFLHDAFM != 0 {
            my_size += ::protobuf::rt::uint32_size(1, self.JCCFFLHDAFM);
        }
        if self.OKMFNBOHJNL != 0 {
            my_size += ::protobuf::rt::uint32_size(8, self.OKMFNBOHJNL);
        }
        if self.NFIDKGHKPLC != false {
            my_size += 1 + 1;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.level != 0 {
            os.write_uint32(10, self.level)?;
        }
        if self.JCCFFLHDAFM != 0 {
            os.write_uint32(1, self.JCCFFLHDAFM)?;
        }
        if self.OKMFNBOHJNL != 0 {
            os.write_uint32(8, self.OKMFNBOHJNL)?;
        }
        if self.NFIDKGHKPLC != false {
            os.write_bool(14, self.NFIDKGHKPLC)?;
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

    fn new() -> GCNAPNPOPEK {
        GCNAPNPOPEK::new()
    }

    fn clear(&mut self) {
        self.level = 0;
        self.JCCFFLHDAFM = 0;
        self.OKMFNBOHJNL = 0;
        self.NFIDKGHKPLC = false;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static GCNAPNPOPEK {
        static instance: GCNAPNPOPEK = GCNAPNPOPEK {
            level: 0,
            JCCFFLHDAFM: 0,
            OKMFNBOHJNL: 0,
            NFIDKGHKPLC: false,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for GCNAPNPOPEK {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("GCNAPNPOPEK").unwrap()).clone()
    }
}

impl ::std::fmt::Display for GCNAPNPOPEK {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for GCNAPNPOPEK {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x11GCNAPNPOPEK.proto\"\x89\x01\n\x0bGCNAPNPOPEK\x12\x14\n\x05level\
    \x18\n\x20\x01(\rR\x05level\x12\x20\n\x0bJCCFFLHDAFM\x18\x01\x20\x01(\rR\
    \x0bJCCFFLHDAFM\x12\x20\n\x0bOKMFNBOHJNL\x18\x08\x20\x01(\rR\x0bOKMFNBOH\
    JNL\x12\x20\n\x0bNFIDKGHKPLC\x18\x0e\x20\x01(\x08R\x0bNFIDKGHKPLCb\x06pr\
    oto3\
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
            messages.push(GCNAPNPOPEK::generated_message_descriptor_data());
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
