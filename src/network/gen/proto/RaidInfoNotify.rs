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

//! Generated file from `RaidInfoNotify.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:RaidInfoNotify)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct RaidInfoNotify {
    // message fields
    // @@protoc_insertion_point(field:RaidInfoNotify.NBLJPGFHDFI)
    pub NBLJPGFHDFI: u32,
    // @@protoc_insertion_point(field:RaidInfoNotify.CJJAEDIALNG)
    pub CJJAEDIALNG: ::std::vec::Vec<super::BCPDMEMPLBM::BCPDMEMPLBM>,
    // @@protoc_insertion_point(field:RaidInfoNotify.OCDLCGIILKI)
    pub OCDLCGIILKI: ::protobuf::MessageField<super::ItemList::ItemList>,
    // @@protoc_insertion_point(field:RaidInfoNotify.OELFPEOCOCK)
    pub OELFPEOCOCK: u64,
    // @@protoc_insertion_point(field:RaidInfoNotify.IOPEEMNLIDM)
    pub IOPEEMNLIDM: ::protobuf::EnumOrUnknown<super::MFCKNJHDIGG::MFCKNJHDIGG>,
    // @@protoc_insertion_point(field:RaidInfoNotify.CFNJJEJIGOK)
    pub CFNJJEJIGOK: u32,
    // special fields
    // @@protoc_insertion_point(special_field:RaidInfoNotify.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a RaidInfoNotify {
    fn default() -> &'a RaidInfoNotify {
        <RaidInfoNotify as ::protobuf::Message>::default_instance()
    }
}

impl RaidInfoNotify {
    pub fn new() -> RaidInfoNotify {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(6);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "NBLJPGFHDFI",
            |m: &RaidInfoNotify| { &m.NBLJPGFHDFI },
            |m: &mut RaidInfoNotify| { &mut m.NBLJPGFHDFI },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "CJJAEDIALNG",
            |m: &RaidInfoNotify| { &m.CJJAEDIALNG },
            |m: &mut RaidInfoNotify| { &mut m.CJJAEDIALNG },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::ItemList::ItemList>(
            "OCDLCGIILKI",
            |m: &RaidInfoNotify| { &m.OCDLCGIILKI },
            |m: &mut RaidInfoNotify| { &mut m.OCDLCGIILKI },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "OELFPEOCOCK",
            |m: &RaidInfoNotify| { &m.OELFPEOCOCK },
            |m: &mut RaidInfoNotify| { &mut m.OELFPEOCOCK },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "IOPEEMNLIDM",
            |m: &RaidInfoNotify| { &m.IOPEEMNLIDM },
            |m: &mut RaidInfoNotify| { &mut m.IOPEEMNLIDM },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "CFNJJEJIGOK",
            |m: &RaidInfoNotify| { &m.CFNJJEJIGOK },
            |m: &mut RaidInfoNotify| { &mut m.CFNJJEJIGOK },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<RaidInfoNotify>(
            "RaidInfoNotify",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for RaidInfoNotify {
    const NAME: &'static str = "RaidInfoNotify";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                96 => {
                    self.NBLJPGFHDFI = is.read_uint32()?;
                },
                26 => {
                    self.CJJAEDIALNG.push(is.read_message()?);
                },
                66 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.OCDLCGIILKI)?;
                },
                112 => {
                    self.OELFPEOCOCK = is.read_uint64()?;
                },
                40 => {
                    self.IOPEEMNLIDM = is.read_enum_or_unknown()?;
                },
                104 => {
                    self.CFNJJEJIGOK = is.read_uint32()?;
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
        if self.NBLJPGFHDFI != 0 {
            my_size += ::protobuf::rt::uint32_size(12, self.NBLJPGFHDFI);
        }
        for value in &self.CJJAEDIALNG {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        if let Some(v) = self.OCDLCGIILKI.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if self.OELFPEOCOCK != 0 {
            my_size += ::protobuf::rt::uint64_size(14, self.OELFPEOCOCK);
        }
        if self.IOPEEMNLIDM != ::protobuf::EnumOrUnknown::new(super::MFCKNJHDIGG::MFCKNJHDIGG::RAID_STATUS_NONE) {
            my_size += ::protobuf::rt::int32_size(5, self.IOPEEMNLIDM.value());
        }
        if self.CFNJJEJIGOK != 0 {
            my_size += ::protobuf::rt::uint32_size(13, self.CFNJJEJIGOK);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.NBLJPGFHDFI != 0 {
            os.write_uint32(12, self.NBLJPGFHDFI)?;
        }
        for v in &self.CJJAEDIALNG {
            ::protobuf::rt::write_message_field_with_cached_size(3, v, os)?;
        };
        if let Some(v) = self.OCDLCGIILKI.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(8, v, os)?;
        }
        if self.OELFPEOCOCK != 0 {
            os.write_uint64(14, self.OELFPEOCOCK)?;
        }
        if self.IOPEEMNLIDM != ::protobuf::EnumOrUnknown::new(super::MFCKNJHDIGG::MFCKNJHDIGG::RAID_STATUS_NONE) {
            os.write_enum(5, ::protobuf::EnumOrUnknown::value(&self.IOPEEMNLIDM))?;
        }
        if self.CFNJJEJIGOK != 0 {
            os.write_uint32(13, self.CFNJJEJIGOK)?;
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

    fn new() -> RaidInfoNotify {
        RaidInfoNotify::new()
    }

    fn clear(&mut self) {
        self.NBLJPGFHDFI = 0;
        self.CJJAEDIALNG.clear();
        self.OCDLCGIILKI.clear();
        self.OELFPEOCOCK = 0;
        self.IOPEEMNLIDM = ::protobuf::EnumOrUnknown::new(super::MFCKNJHDIGG::MFCKNJHDIGG::RAID_STATUS_NONE);
        self.CFNJJEJIGOK = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static RaidInfoNotify {
        static instance: RaidInfoNotify = RaidInfoNotify {
            NBLJPGFHDFI: 0,
            CJJAEDIALNG: ::std::vec::Vec::new(),
            OCDLCGIILKI: ::protobuf::MessageField::none(),
            OELFPEOCOCK: 0,
            IOPEEMNLIDM: ::protobuf::EnumOrUnknown::from_i32(0),
            CFNJJEJIGOK: 0,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for RaidInfoNotify {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("RaidInfoNotify").unwrap()).clone()
    }
}

impl ::std::fmt::Display for RaidInfoNotify {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for RaidInfoNotify {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x14RaidInfoNotify.proto\x1a\x11BCPDMEMPLBM.proto\x1a\x0eItemList.prot\
    o\x1a\x11MFCKNJHDIGG.proto\"\x83\x02\n\x0eRaidInfoNotify\x12\x20\n\x0bNB\
    LJPGFHDFI\x18\x0c\x20\x01(\rR\x0bNBLJPGFHDFI\x12.\n\x0bCJJAEDIALNG\x18\
    \x03\x20\x03(\x0b2\x0c.BCPDMEMPLBMR\x0bCJJAEDIALNG\x12+\n\x0bOCDLCGIILKI\
    \x18\x08\x20\x01(\x0b2\t.ItemListR\x0bOCDLCGIILKI\x12\x20\n\x0bOELFPEOCO\
    CK\x18\x0e\x20\x01(\x04R\x0bOELFPEOCOCK\x12.\n\x0bIOPEEMNLIDM\x18\x05\
    \x20\x01(\x0e2\x0c.MFCKNJHDIGGR\x0bIOPEEMNLIDM\x12\x20\n\x0bCFNJJEJIGOK\
    \x18\r\x20\x01(\rR\x0bCFNJJEJIGOKb\x06proto3\
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
            deps.push(super::BCPDMEMPLBM::file_descriptor().clone());
            deps.push(super::ItemList::file_descriptor().clone());
            deps.push(super::MFCKNJHDIGG::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(RaidInfoNotify::generated_message_descriptor_data());
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
