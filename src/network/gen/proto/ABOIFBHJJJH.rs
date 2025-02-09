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

//! Generated file from `ABOIFBHJJJH.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:ABOIFBHJJJH)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct ABOIFBHJJJH {
    // message fields
    // @@protoc_insertion_point(field:ABOIFBHJJJH.DILIMNCAEKO)
    pub DILIMNCAEKO: ::std::vec::Vec<u32>,
    // @@protoc_insertion_point(field:ABOIFBHJJJH.BNPFGBECLDE)
    pub BNPFGBECLDE: ::protobuf::MessageField<super::COKMEFOLDDH::COKMEFOLDDH>,
    // @@protoc_insertion_point(field:ABOIFBHJJJH.KBMFLPNPKOJ)
    pub KBMFLPNPKOJ: u32,
    // @@protoc_insertion_point(field:ABOIFBHJJJH.BNPDIEGHDMO)
    pub BNPDIEGHDMO: u32,
    // @@protoc_insertion_point(field:ABOIFBHJJJH.OFJLGBACJDG)
    pub OFJLGBACJDG: u32,
    // @@protoc_insertion_point(field:ABOIFBHJJJH.CPEGEHHHIAI)
    pub CPEGEHHHIAI: u32,
    // @@protoc_insertion_point(field:ABOIFBHJJJH.NPOHCFOJAMH)
    pub NPOHCFOJAMH: u32,
    // @@protoc_insertion_point(field:ABOIFBHJJJH.FJGJPJNNEHJ)
    pub FJGJPJNNEHJ: u32,
    // @@protoc_insertion_point(field:ABOIFBHJJJH.HHLBDGMIBNP)
    pub HHLBDGMIBNP: u32,
    // @@protoc_insertion_point(field:ABOIFBHJJJH.NFLLMHOEIIP)
    pub NFLLMHOEIIP: u32,
    // @@protoc_insertion_point(field:ABOIFBHJJJH.BFMIBFPFPMG)
    pub BFMIBFPFPMG: ::protobuf::EnumOrUnknown<super::CKMJGCGAEEI::CKMJGCGAEEI>,
    // special fields
    // @@protoc_insertion_point(special_field:ABOIFBHJJJH.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a ABOIFBHJJJH {
    fn default() -> &'a ABOIFBHJJJH {
        <ABOIFBHJJJH as ::protobuf::Message>::default_instance()
    }
}

impl ABOIFBHJJJH {
    pub fn new() -> ABOIFBHJJJH {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(11);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "DILIMNCAEKO",
            |m: &ABOIFBHJJJH| { &m.DILIMNCAEKO },
            |m: &mut ABOIFBHJJJH| { &mut m.DILIMNCAEKO },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::COKMEFOLDDH::COKMEFOLDDH>(
            "BNPFGBECLDE",
            |m: &ABOIFBHJJJH| { &m.BNPFGBECLDE },
            |m: &mut ABOIFBHJJJH| { &mut m.BNPFGBECLDE },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "KBMFLPNPKOJ",
            |m: &ABOIFBHJJJH| { &m.KBMFLPNPKOJ },
            |m: &mut ABOIFBHJJJH| { &mut m.KBMFLPNPKOJ },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "BNPDIEGHDMO",
            |m: &ABOIFBHJJJH| { &m.BNPDIEGHDMO },
            |m: &mut ABOIFBHJJJH| { &mut m.BNPDIEGHDMO },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "OFJLGBACJDG",
            |m: &ABOIFBHJJJH| { &m.OFJLGBACJDG },
            |m: &mut ABOIFBHJJJH| { &mut m.OFJLGBACJDG },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "CPEGEHHHIAI",
            |m: &ABOIFBHJJJH| { &m.CPEGEHHHIAI },
            |m: &mut ABOIFBHJJJH| { &mut m.CPEGEHHHIAI },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "NPOHCFOJAMH",
            |m: &ABOIFBHJJJH| { &m.NPOHCFOJAMH },
            |m: &mut ABOIFBHJJJH| { &mut m.NPOHCFOJAMH },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "FJGJPJNNEHJ",
            |m: &ABOIFBHJJJH| { &m.FJGJPJNNEHJ },
            |m: &mut ABOIFBHJJJH| { &mut m.FJGJPJNNEHJ },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "HHLBDGMIBNP",
            |m: &ABOIFBHJJJH| { &m.HHLBDGMIBNP },
            |m: &mut ABOIFBHJJJH| { &mut m.HHLBDGMIBNP },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "NFLLMHOEIIP",
            |m: &ABOIFBHJJJH| { &m.NFLLMHOEIIP },
            |m: &mut ABOIFBHJJJH| { &mut m.NFLLMHOEIIP },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "BFMIBFPFPMG",
            |m: &ABOIFBHJJJH| { &m.BFMIBFPFPMG },
            |m: &mut ABOIFBHJJJH| { &mut m.BFMIBFPFPMG },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<ABOIFBHJJJH>(
            "ABOIFBHJJJH",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for ABOIFBHJJJH {
    const NAME: &'static str = "ABOIFBHJJJH";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                34 => {
                    is.read_repeated_packed_uint32_into(&mut self.DILIMNCAEKO)?;
                },
                32 => {
                    self.DILIMNCAEKO.push(is.read_uint32()?);
                },
                122 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.BNPFGBECLDE)?;
                },
                80 => {
                    self.KBMFLPNPKOJ = is.read_uint32()?;
                },
                88 => {
                    self.BNPDIEGHDMO = is.read_uint32()?;
                },
                72 => {
                    self.OFJLGBACJDG = is.read_uint32()?;
                },
                8 => {
                    self.CPEGEHHHIAI = is.read_uint32()?;
                },
                24 => {
                    self.NPOHCFOJAMH = is.read_uint32()?;
                },
                56 => {
                    self.FJGJPJNNEHJ = is.read_uint32()?;
                },
                104 => {
                    self.HHLBDGMIBNP = is.read_uint32()?;
                },
                112 => {
                    self.NFLLMHOEIIP = is.read_uint32()?;
                },
                64 => {
                    self.BFMIBFPFPMG = is.read_enum_or_unknown()?;
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
        for value in &self.DILIMNCAEKO {
            my_size += ::protobuf::rt::uint32_size(4, *value);
        };
        if let Some(v) = self.BNPFGBECLDE.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if self.KBMFLPNPKOJ != 0 {
            my_size += ::protobuf::rt::uint32_size(10, self.KBMFLPNPKOJ);
        }
        if self.BNPDIEGHDMO != 0 {
            my_size += ::protobuf::rt::uint32_size(11, self.BNPDIEGHDMO);
        }
        if self.OFJLGBACJDG != 0 {
            my_size += ::protobuf::rt::uint32_size(9, self.OFJLGBACJDG);
        }
        if self.CPEGEHHHIAI != 0 {
            my_size += ::protobuf::rt::uint32_size(1, self.CPEGEHHHIAI);
        }
        if self.NPOHCFOJAMH != 0 {
            my_size += ::protobuf::rt::uint32_size(3, self.NPOHCFOJAMH);
        }
        if self.FJGJPJNNEHJ != 0 {
            my_size += ::protobuf::rt::uint32_size(7, self.FJGJPJNNEHJ);
        }
        if self.HHLBDGMIBNP != 0 {
            my_size += ::protobuf::rt::uint32_size(13, self.HHLBDGMIBNP);
        }
        if self.NFLLMHOEIIP != 0 {
            my_size += ::protobuf::rt::uint32_size(14, self.NFLLMHOEIIP);
        }
        if self.BFMIBFPFPMG != ::protobuf::EnumOrUnknown::new(super::CKMJGCGAEEI::CKMJGCGAEEI::MATCH3_PLAYER_STATE_ALIVE) {
            my_size += ::protobuf::rt::int32_size(8, self.BFMIBFPFPMG.value());
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        for v in &self.DILIMNCAEKO {
            os.write_uint32(4, *v)?;
        };
        if let Some(v) = self.BNPFGBECLDE.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(15, v, os)?;
        }
        if self.KBMFLPNPKOJ != 0 {
            os.write_uint32(10, self.KBMFLPNPKOJ)?;
        }
        if self.BNPDIEGHDMO != 0 {
            os.write_uint32(11, self.BNPDIEGHDMO)?;
        }
        if self.OFJLGBACJDG != 0 {
            os.write_uint32(9, self.OFJLGBACJDG)?;
        }
        if self.CPEGEHHHIAI != 0 {
            os.write_uint32(1, self.CPEGEHHHIAI)?;
        }
        if self.NPOHCFOJAMH != 0 {
            os.write_uint32(3, self.NPOHCFOJAMH)?;
        }
        if self.FJGJPJNNEHJ != 0 {
            os.write_uint32(7, self.FJGJPJNNEHJ)?;
        }
        if self.HHLBDGMIBNP != 0 {
            os.write_uint32(13, self.HHLBDGMIBNP)?;
        }
        if self.NFLLMHOEIIP != 0 {
            os.write_uint32(14, self.NFLLMHOEIIP)?;
        }
        if self.BFMIBFPFPMG != ::protobuf::EnumOrUnknown::new(super::CKMJGCGAEEI::CKMJGCGAEEI::MATCH3_PLAYER_STATE_ALIVE) {
            os.write_enum(8, ::protobuf::EnumOrUnknown::value(&self.BFMIBFPFPMG))?;
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

    fn new() -> ABOIFBHJJJH {
        ABOIFBHJJJH::new()
    }

    fn clear(&mut self) {
        self.DILIMNCAEKO.clear();
        self.BNPFGBECLDE.clear();
        self.KBMFLPNPKOJ = 0;
        self.BNPDIEGHDMO = 0;
        self.OFJLGBACJDG = 0;
        self.CPEGEHHHIAI = 0;
        self.NPOHCFOJAMH = 0;
        self.FJGJPJNNEHJ = 0;
        self.HHLBDGMIBNP = 0;
        self.NFLLMHOEIIP = 0;
        self.BFMIBFPFPMG = ::protobuf::EnumOrUnknown::new(super::CKMJGCGAEEI::CKMJGCGAEEI::MATCH3_PLAYER_STATE_ALIVE);
        self.special_fields.clear();
    }

    fn default_instance() -> &'static ABOIFBHJJJH {
        static instance: ABOIFBHJJJH = ABOIFBHJJJH {
            DILIMNCAEKO: ::std::vec::Vec::new(),
            BNPFGBECLDE: ::protobuf::MessageField::none(),
            KBMFLPNPKOJ: 0,
            BNPDIEGHDMO: 0,
            OFJLGBACJDG: 0,
            CPEGEHHHIAI: 0,
            NPOHCFOJAMH: 0,
            FJGJPJNNEHJ: 0,
            HHLBDGMIBNP: 0,
            NFLLMHOEIIP: 0,
            BFMIBFPFPMG: ::protobuf::EnumOrUnknown::from_i32(0),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for ABOIFBHJJJH {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("ABOIFBHJJJH").unwrap()).clone()
    }
}

impl ::std::fmt::Display for ABOIFBHJJJH {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ABOIFBHJJJH {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x11ABOIFBHJJJH.proto\x1a\x11CKMJGCGAEEI.proto\x1a\x11COKMEFOLDDH.prot\
    o\"\x9f\x03\n\x0bABOIFBHJJJH\x12\x20\n\x0bDILIMNCAEKO\x18\x04\x20\x03(\r\
    R\x0bDILIMNCAEKO\x12.\n\x0bBNPFGBECLDE\x18\x0f\x20\x01(\x0b2\x0c.COKMEFO\
    LDDHR\x0bBNPFGBECLDE\x12\x20\n\x0bKBMFLPNPKOJ\x18\n\x20\x01(\rR\x0bKBMFL\
    PNPKOJ\x12\x20\n\x0bBNPDIEGHDMO\x18\x0b\x20\x01(\rR\x0bBNPDIEGHDMO\x12\
    \x20\n\x0bOFJLGBACJDG\x18\t\x20\x01(\rR\x0bOFJLGBACJDG\x12\x20\n\x0bCPEG\
    EHHHIAI\x18\x01\x20\x01(\rR\x0bCPEGEHHHIAI\x12\x20\n\x0bNPOHCFOJAMH\x18\
    \x03\x20\x01(\rR\x0bNPOHCFOJAMH\x12\x20\n\x0bFJGJPJNNEHJ\x18\x07\x20\x01\
    (\rR\x0bFJGJPJNNEHJ\x12\x20\n\x0bHHLBDGMIBNP\x18\r\x20\x01(\rR\x0bHHLBDG\
    MIBNP\x12\x20\n\x0bNFLLMHOEIIP\x18\x0e\x20\x01(\rR\x0bNFLLMHOEIIP\x12.\n\
    \x0bBFMIBFPFPMG\x18\x08\x20\x01(\x0e2\x0c.CKMJGCGAEEIR\x0bBFMIBFPFPMGb\
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
            let mut deps = ::std::vec::Vec::with_capacity(2);
            deps.push(super::CKMJGCGAEEI::file_descriptor().clone());
            deps.push(super::COKMEFOLDDH::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(ABOIFBHJJJH::generated_message_descriptor_data());
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
