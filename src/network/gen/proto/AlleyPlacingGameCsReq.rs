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

//! Generated file from `AlleyPlacingGameCsReq.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:AlleyPlacingGameCsReq)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct AlleyPlacingGameCsReq {
    // message fields
    // @@protoc_insertion_point(field:AlleyPlacingGameCsReq.KFNMAPNILOG)
    pub KFNMAPNILOG: u32,
    // @@protoc_insertion_point(field:AlleyPlacingGameCsReq.HBIENMCGONC)
    pub HBIENMCGONC: u32,
    // @@protoc_insertion_point(field:AlleyPlacingGameCsReq.LBCCMFHGMHC)
    pub LBCCMFHGMHC: u32,
    // @@protoc_insertion_point(field:AlleyPlacingGameCsReq.CIFDOMCABDB)
    pub CIFDOMCABDB: u32,
    // @@protoc_insertion_point(field:AlleyPlacingGameCsReq.EFPNPKPKCPO)
    pub EFPNPKPKCPO: u32,
    // @@protoc_insertion_point(field:AlleyPlacingGameCsReq.NKCDCKMIPJM)
    pub NKCDCKMIPJM: u32,
    // @@protoc_insertion_point(field:AlleyPlacingGameCsReq.PIPKAHIFDOD)
    pub PIPKAHIFDOD: u32,
    // @@protoc_insertion_point(field:AlleyPlacingGameCsReq.IJINMKBOPCB)
    pub IJINMKBOPCB: ::protobuf::MessageField<super::AlleyPlacingShip::AlleyPlacingShip>,
    // @@protoc_insertion_point(field:AlleyPlacingGameCsReq.BPGAPMPCEIF)
    pub BPGAPMPCEIF: u32,
    // special fields
    // @@protoc_insertion_point(special_field:AlleyPlacingGameCsReq.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a AlleyPlacingGameCsReq {
    fn default() -> &'a AlleyPlacingGameCsReq {
        <AlleyPlacingGameCsReq as ::protobuf::Message>::default_instance()
    }
}

impl AlleyPlacingGameCsReq {
    pub fn new() -> AlleyPlacingGameCsReq {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(9);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "KFNMAPNILOG",
            |m: &AlleyPlacingGameCsReq| { &m.KFNMAPNILOG },
            |m: &mut AlleyPlacingGameCsReq| { &mut m.KFNMAPNILOG },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "HBIENMCGONC",
            |m: &AlleyPlacingGameCsReq| { &m.HBIENMCGONC },
            |m: &mut AlleyPlacingGameCsReq| { &mut m.HBIENMCGONC },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "LBCCMFHGMHC",
            |m: &AlleyPlacingGameCsReq| { &m.LBCCMFHGMHC },
            |m: &mut AlleyPlacingGameCsReq| { &mut m.LBCCMFHGMHC },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "CIFDOMCABDB",
            |m: &AlleyPlacingGameCsReq| { &m.CIFDOMCABDB },
            |m: &mut AlleyPlacingGameCsReq| { &mut m.CIFDOMCABDB },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "EFPNPKPKCPO",
            |m: &AlleyPlacingGameCsReq| { &m.EFPNPKPKCPO },
            |m: &mut AlleyPlacingGameCsReq| { &mut m.EFPNPKPKCPO },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "NKCDCKMIPJM",
            |m: &AlleyPlacingGameCsReq| { &m.NKCDCKMIPJM },
            |m: &mut AlleyPlacingGameCsReq| { &mut m.NKCDCKMIPJM },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "PIPKAHIFDOD",
            |m: &AlleyPlacingGameCsReq| { &m.PIPKAHIFDOD },
            |m: &mut AlleyPlacingGameCsReq| { &mut m.PIPKAHIFDOD },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::AlleyPlacingShip::AlleyPlacingShip>(
            "IJINMKBOPCB",
            |m: &AlleyPlacingGameCsReq| { &m.IJINMKBOPCB },
            |m: &mut AlleyPlacingGameCsReq| { &mut m.IJINMKBOPCB },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "BPGAPMPCEIF",
            |m: &AlleyPlacingGameCsReq| { &m.BPGAPMPCEIF },
            |m: &mut AlleyPlacingGameCsReq| { &mut m.BPGAPMPCEIF },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<AlleyPlacingGameCsReq>(
            "AlleyPlacingGameCsReq",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for AlleyPlacingGameCsReq {
    const NAME: &'static str = "AlleyPlacingGameCsReq";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                72 => {
                    self.KFNMAPNILOG = is.read_uint32()?;
                },
                88 => {
                    self.HBIENMCGONC = is.read_uint32()?;
                },
                56 => {
                    self.LBCCMFHGMHC = is.read_uint32()?;
                },
                104 => {
                    self.CIFDOMCABDB = is.read_uint32()?;
                },
                8 => {
                    self.EFPNPKPKCPO = is.read_uint32()?;
                },
                80 => {
                    self.NKCDCKMIPJM = is.read_uint32()?;
                },
                64 => {
                    self.PIPKAHIFDOD = is.read_uint32()?;
                },
                114 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.IJINMKBOPCB)?;
                },
                40 => {
                    self.BPGAPMPCEIF = is.read_uint32()?;
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
        if self.KFNMAPNILOG != 0 {
            my_size += ::protobuf::rt::uint32_size(9, self.KFNMAPNILOG);
        }
        if self.HBIENMCGONC != 0 {
            my_size += ::protobuf::rt::uint32_size(11, self.HBIENMCGONC);
        }
        if self.LBCCMFHGMHC != 0 {
            my_size += ::protobuf::rt::uint32_size(7, self.LBCCMFHGMHC);
        }
        if self.CIFDOMCABDB != 0 {
            my_size += ::protobuf::rt::uint32_size(13, self.CIFDOMCABDB);
        }
        if self.EFPNPKPKCPO != 0 {
            my_size += ::protobuf::rt::uint32_size(1, self.EFPNPKPKCPO);
        }
        if self.NKCDCKMIPJM != 0 {
            my_size += ::protobuf::rt::uint32_size(10, self.NKCDCKMIPJM);
        }
        if self.PIPKAHIFDOD != 0 {
            my_size += ::protobuf::rt::uint32_size(8, self.PIPKAHIFDOD);
        }
        if let Some(v) = self.IJINMKBOPCB.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if self.BPGAPMPCEIF != 0 {
            my_size += ::protobuf::rt::uint32_size(5, self.BPGAPMPCEIF);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.KFNMAPNILOG != 0 {
            os.write_uint32(9, self.KFNMAPNILOG)?;
        }
        if self.HBIENMCGONC != 0 {
            os.write_uint32(11, self.HBIENMCGONC)?;
        }
        if self.LBCCMFHGMHC != 0 {
            os.write_uint32(7, self.LBCCMFHGMHC)?;
        }
        if self.CIFDOMCABDB != 0 {
            os.write_uint32(13, self.CIFDOMCABDB)?;
        }
        if self.EFPNPKPKCPO != 0 {
            os.write_uint32(1, self.EFPNPKPKCPO)?;
        }
        if self.NKCDCKMIPJM != 0 {
            os.write_uint32(10, self.NKCDCKMIPJM)?;
        }
        if self.PIPKAHIFDOD != 0 {
            os.write_uint32(8, self.PIPKAHIFDOD)?;
        }
        if let Some(v) = self.IJINMKBOPCB.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(14, v, os)?;
        }
        if self.BPGAPMPCEIF != 0 {
            os.write_uint32(5, self.BPGAPMPCEIF)?;
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

    fn new() -> AlleyPlacingGameCsReq {
        AlleyPlacingGameCsReq::new()
    }

    fn clear(&mut self) {
        self.KFNMAPNILOG = 0;
        self.HBIENMCGONC = 0;
        self.LBCCMFHGMHC = 0;
        self.CIFDOMCABDB = 0;
        self.EFPNPKPKCPO = 0;
        self.NKCDCKMIPJM = 0;
        self.PIPKAHIFDOD = 0;
        self.IJINMKBOPCB.clear();
        self.BPGAPMPCEIF = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static AlleyPlacingGameCsReq {
        static instance: AlleyPlacingGameCsReq = AlleyPlacingGameCsReq {
            KFNMAPNILOG: 0,
            HBIENMCGONC: 0,
            LBCCMFHGMHC: 0,
            CIFDOMCABDB: 0,
            EFPNPKPKCPO: 0,
            NKCDCKMIPJM: 0,
            PIPKAHIFDOD: 0,
            IJINMKBOPCB: ::protobuf::MessageField::none(),
            BPGAPMPCEIF: 0,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for AlleyPlacingGameCsReq {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("AlleyPlacingGameCsReq").unwrap()).clone()
    }
}

impl ::std::fmt::Display for AlleyPlacingGameCsReq {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for AlleyPlacingGameCsReq {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x1bAlleyPlacingGameCsReq.proto\x1a\x16AlleyPlacingShip.proto\"\xdc\
    \x02\n\x15AlleyPlacingGameCsReq\x12\x20\n\x0bKFNMAPNILOG\x18\t\x20\x01(\
    \rR\x0bKFNMAPNILOG\x12\x20\n\x0bHBIENMCGONC\x18\x0b\x20\x01(\rR\x0bHBIEN\
    MCGONC\x12\x20\n\x0bLBCCMFHGMHC\x18\x07\x20\x01(\rR\x0bLBCCMFHGMHC\x12\
    \x20\n\x0bCIFDOMCABDB\x18\r\x20\x01(\rR\x0bCIFDOMCABDB\x12\x20\n\x0bEFPN\
    PKPKCPO\x18\x01\x20\x01(\rR\x0bEFPNPKPKCPO\x12\x20\n\x0bNKCDCKMIPJM\x18\
    \n\x20\x01(\rR\x0bNKCDCKMIPJM\x12\x20\n\x0bPIPKAHIFDOD\x18\x08\x20\x01(\
    \rR\x0bPIPKAHIFDOD\x123\n\x0bIJINMKBOPCB\x18\x0e\x20\x01(\x0b2\x11.Alley\
    PlacingShipR\x0bIJINMKBOPCB\x12\x20\n\x0bBPGAPMPCEIF\x18\x05\x20\x01(\rR\
    \x0bBPGAPMPCEIFb\x06proto3\
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
            let mut deps = ::std::vec::Vec::with_capacity(1);
            deps.push(super::AlleyPlacingShip::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(AlleyPlacingGameCsReq::generated_message_descriptor_data());
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
