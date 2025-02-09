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

//! Generated file from `GNKEPMFPBAJ.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:GNKEPMFPBAJ)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct GNKEPMFPBAJ {
    // message fields
    // @@protoc_insertion_point(field:GNKEPMFPBAJ.JGLKGMMIEEE)
    pub JGLKGMMIEEE: ::std::vec::Vec<super::IJHJAFMGEMI::IJHJAFMGEMI>,
    // @@protoc_insertion_point(field:GNKEPMFPBAJ.CHEHLJEKONK)
    pub CHEHLJEKONK: ::std::string::String,
    // @@protoc_insertion_point(field:GNKEPMFPBAJ.IOOJBIKELEK)
    pub IOOJBIKELEK: ::std::string::String,
    // @@protoc_insertion_point(field:GNKEPMFPBAJ.FPBLKIEMANP)
    pub FPBLKIEMANP: ::std::vec::Vec<super::AIHFEEECBCI::AIHFEEECBCI>,
    // @@protoc_insertion_point(field:GNKEPMFPBAJ.BHONGJMFKIM)
    pub BHONGJMFKIM: u32,
    // @@protoc_insertion_point(field:GNKEPMFPBAJ.BJNKEEFKAIK)
    pub BJNKEEFKAIK: u32,
    // @@protoc_insertion_point(field:GNKEPMFPBAJ.FBKLOLCJIPK)
    pub FBKLOLCJIPK: u32,
    // @@protoc_insertion_point(field:GNKEPMFPBAJ.LEEGNIFHGDH)
    pub LEEGNIFHGDH: u32,
    // @@protoc_insertion_point(field:GNKEPMFPBAJ.IBOJJFGBMAK)
    pub IBOJJFGBMAK: ::protobuf::MessageField<super::HAPEOBJJPAF::HAPEOBJJPAF>,
    // special fields
    // @@protoc_insertion_point(special_field:GNKEPMFPBAJ.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a GNKEPMFPBAJ {
    fn default() -> &'a GNKEPMFPBAJ {
        <GNKEPMFPBAJ as ::protobuf::Message>::default_instance()
    }
}

impl GNKEPMFPBAJ {
    pub fn new() -> GNKEPMFPBAJ {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(9);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "JGLKGMMIEEE",
            |m: &GNKEPMFPBAJ| { &m.JGLKGMMIEEE },
            |m: &mut GNKEPMFPBAJ| { &mut m.JGLKGMMIEEE },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "CHEHLJEKONK",
            |m: &GNKEPMFPBAJ| { &m.CHEHLJEKONK },
            |m: &mut GNKEPMFPBAJ| { &mut m.CHEHLJEKONK },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "IOOJBIKELEK",
            |m: &GNKEPMFPBAJ| { &m.IOOJBIKELEK },
            |m: &mut GNKEPMFPBAJ| { &mut m.IOOJBIKELEK },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "FPBLKIEMANP",
            |m: &GNKEPMFPBAJ| { &m.FPBLKIEMANP },
            |m: &mut GNKEPMFPBAJ| { &mut m.FPBLKIEMANP },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "BHONGJMFKIM",
            |m: &GNKEPMFPBAJ| { &m.BHONGJMFKIM },
            |m: &mut GNKEPMFPBAJ| { &mut m.BHONGJMFKIM },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "BJNKEEFKAIK",
            |m: &GNKEPMFPBAJ| { &m.BJNKEEFKAIK },
            |m: &mut GNKEPMFPBAJ| { &mut m.BJNKEEFKAIK },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "FBKLOLCJIPK",
            |m: &GNKEPMFPBAJ| { &m.FBKLOLCJIPK },
            |m: &mut GNKEPMFPBAJ| { &mut m.FBKLOLCJIPK },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "LEEGNIFHGDH",
            |m: &GNKEPMFPBAJ| { &m.LEEGNIFHGDH },
            |m: &mut GNKEPMFPBAJ| { &mut m.LEEGNIFHGDH },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::HAPEOBJJPAF::HAPEOBJJPAF>(
            "IBOJJFGBMAK",
            |m: &GNKEPMFPBAJ| { &m.IBOJJFGBMAK },
            |m: &mut GNKEPMFPBAJ| { &mut m.IBOJJFGBMAK },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<GNKEPMFPBAJ>(
            "GNKEPMFPBAJ",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for GNKEPMFPBAJ {
    const NAME: &'static str = "GNKEPMFPBAJ";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                10 => {
                    self.JGLKGMMIEEE.push(is.read_message()?);
                },
                18 => {
                    self.CHEHLJEKONK = is.read_string()?;
                },
                26 => {
                    self.IOOJBIKELEK = is.read_string()?;
                },
                34 => {
                    self.FPBLKIEMANP.push(is.read_message()?);
                },
                40 => {
                    self.BHONGJMFKIM = is.read_uint32()?;
                },
                48 => {
                    self.BJNKEEFKAIK = is.read_uint32()?;
                },
                56 => {
                    self.FBKLOLCJIPK = is.read_uint32()?;
                },
                64 => {
                    self.LEEGNIFHGDH = is.read_uint32()?;
                },
                74 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.IBOJJFGBMAK)?;
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
        for value in &self.JGLKGMMIEEE {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        if !self.CHEHLJEKONK.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.CHEHLJEKONK);
        }
        if !self.IOOJBIKELEK.is_empty() {
            my_size += ::protobuf::rt::string_size(3, &self.IOOJBIKELEK);
        }
        for value in &self.FPBLKIEMANP {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        if self.BHONGJMFKIM != 0 {
            my_size += ::protobuf::rt::uint32_size(5, self.BHONGJMFKIM);
        }
        if self.BJNKEEFKAIK != 0 {
            my_size += ::protobuf::rt::uint32_size(6, self.BJNKEEFKAIK);
        }
        if self.FBKLOLCJIPK != 0 {
            my_size += ::protobuf::rt::uint32_size(7, self.FBKLOLCJIPK);
        }
        if self.LEEGNIFHGDH != 0 {
            my_size += ::protobuf::rt::uint32_size(8, self.LEEGNIFHGDH);
        }
        if let Some(v) = self.IBOJJFGBMAK.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        for v in &self.JGLKGMMIEEE {
            ::protobuf::rt::write_message_field_with_cached_size(1, v, os)?;
        };
        if !self.CHEHLJEKONK.is_empty() {
            os.write_string(2, &self.CHEHLJEKONK)?;
        }
        if !self.IOOJBIKELEK.is_empty() {
            os.write_string(3, &self.IOOJBIKELEK)?;
        }
        for v in &self.FPBLKIEMANP {
            ::protobuf::rt::write_message_field_with_cached_size(4, v, os)?;
        };
        if self.BHONGJMFKIM != 0 {
            os.write_uint32(5, self.BHONGJMFKIM)?;
        }
        if self.BJNKEEFKAIK != 0 {
            os.write_uint32(6, self.BJNKEEFKAIK)?;
        }
        if self.FBKLOLCJIPK != 0 {
            os.write_uint32(7, self.FBKLOLCJIPK)?;
        }
        if self.LEEGNIFHGDH != 0 {
            os.write_uint32(8, self.LEEGNIFHGDH)?;
        }
        if let Some(v) = self.IBOJJFGBMAK.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(9, v, os)?;
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

    fn new() -> GNKEPMFPBAJ {
        GNKEPMFPBAJ::new()
    }

    fn clear(&mut self) {
        self.JGLKGMMIEEE.clear();
        self.CHEHLJEKONK.clear();
        self.IOOJBIKELEK.clear();
        self.FPBLKIEMANP.clear();
        self.BHONGJMFKIM = 0;
        self.BJNKEEFKAIK = 0;
        self.FBKLOLCJIPK = 0;
        self.LEEGNIFHGDH = 0;
        self.IBOJJFGBMAK.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static GNKEPMFPBAJ {
        static instance: GNKEPMFPBAJ = GNKEPMFPBAJ {
            JGLKGMMIEEE: ::std::vec::Vec::new(),
            CHEHLJEKONK: ::std::string::String::new(),
            IOOJBIKELEK: ::std::string::String::new(),
            FPBLKIEMANP: ::std::vec::Vec::new(),
            BHONGJMFKIM: 0,
            BJNKEEFKAIK: 0,
            FBKLOLCJIPK: 0,
            LEEGNIFHGDH: 0,
            IBOJJFGBMAK: ::protobuf::MessageField::none(),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for GNKEPMFPBAJ {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("GNKEPMFPBAJ").unwrap()).clone()
    }
}

impl ::std::fmt::Display for GNKEPMFPBAJ {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for GNKEPMFPBAJ {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x11GNKEPMFPBAJ.proto\x1a\x11AIHFEEECBCI.proto\x1a\x11HAPEOBJJPAF.prot\
    o\x1a\x11IJHJAFMGEMI.proto\"\xe9\x02\n\x0bGNKEPMFPBAJ\x12.\n\x0bJGLKGMMI\
    EEE\x18\x01\x20\x03(\x0b2\x0c.IJHJAFMGEMIR\x0bJGLKGMMIEEE\x12\x20\n\x0bC\
    HEHLJEKONK\x18\x02\x20\x01(\tR\x0bCHEHLJEKONK\x12\x20\n\x0bIOOJBIKELEK\
    \x18\x03\x20\x01(\tR\x0bIOOJBIKELEK\x12.\n\x0bFPBLKIEMANP\x18\x04\x20\
    \x03(\x0b2\x0c.AIHFEEECBCIR\x0bFPBLKIEMANP\x12\x20\n\x0bBHONGJMFKIM\x18\
    \x05\x20\x01(\rR\x0bBHONGJMFKIM\x12\x20\n\x0bBJNKEEFKAIK\x18\x06\x20\x01\
    (\rR\x0bBJNKEEFKAIK\x12\x20\n\x0bFBKLOLCJIPK\x18\x07\x20\x01(\rR\x0bFBKL\
    OLCJIPK\x12\x20\n\x0bLEEGNIFHGDH\x18\x08\x20\x01(\rR\x0bLEEGNIFHGDH\x12.\
    \n\x0bIBOJJFGBMAK\x18\t\x20\x01(\x0b2\x0c.HAPEOBJJPAFR\x0bIBOJJFGBMAKb\
    \x06proto3\
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
            deps.push(super::AIHFEEECBCI::file_descriptor().clone());
            deps.push(super::HAPEOBJJPAF::file_descriptor().clone());
            deps.push(super::IJHJAFMGEMI::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(GNKEPMFPBAJ::generated_message_descriptor_data());
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
