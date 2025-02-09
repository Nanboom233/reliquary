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

//! Generated file from `GetSaveRaidScRsp.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:GetSaveRaidScRsp)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct GetSaveRaidScRsp {
    // message fields
    // @@protoc_insertion_point(field:GetSaveRaidScRsp.CJJAEDIALNG)
    pub CJJAEDIALNG: ::std::vec::Vec<super::BCPDMEMPLBM::BCPDMEMPLBM>,
    // @@protoc_insertion_point(field:GetSaveRaidScRsp.retcode)
    pub retcode: u32,
    // @@protoc_insertion_point(field:GetSaveRaidScRsp.FCBODANKHEE)
    pub FCBODANKHEE: bool,
    // @@protoc_insertion_point(field:GetSaveRaidScRsp.NBLJPGFHDFI)
    pub NBLJPGFHDFI: u32,
    // @@protoc_insertion_point(field:GetSaveRaidScRsp.CFNJJEJIGOK)
    pub CFNJJEJIGOK: u32,
    // special fields
    // @@protoc_insertion_point(special_field:GetSaveRaidScRsp.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a GetSaveRaidScRsp {
    fn default() -> &'a GetSaveRaidScRsp {
        <GetSaveRaidScRsp as ::protobuf::Message>::default_instance()
    }
}

impl GetSaveRaidScRsp {
    pub fn new() -> GetSaveRaidScRsp {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(5);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "CJJAEDIALNG",
            |m: &GetSaveRaidScRsp| { &m.CJJAEDIALNG },
            |m: &mut GetSaveRaidScRsp| { &mut m.CJJAEDIALNG },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "retcode",
            |m: &GetSaveRaidScRsp| { &m.retcode },
            |m: &mut GetSaveRaidScRsp| { &mut m.retcode },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "FCBODANKHEE",
            |m: &GetSaveRaidScRsp| { &m.FCBODANKHEE },
            |m: &mut GetSaveRaidScRsp| { &mut m.FCBODANKHEE },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "NBLJPGFHDFI",
            |m: &GetSaveRaidScRsp| { &m.NBLJPGFHDFI },
            |m: &mut GetSaveRaidScRsp| { &mut m.NBLJPGFHDFI },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "CFNJJEJIGOK",
            |m: &GetSaveRaidScRsp| { &m.CFNJJEJIGOK },
            |m: &mut GetSaveRaidScRsp| { &mut m.CFNJJEJIGOK },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<GetSaveRaidScRsp>(
            "GetSaveRaidScRsp",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for GetSaveRaidScRsp {
    const NAME: &'static str = "GetSaveRaidScRsp";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                58 => {
                    self.CJJAEDIALNG.push(is.read_message()?);
                },
                88 => {
                    self.retcode = is.read_uint32()?;
                },
                24 => {
                    self.FCBODANKHEE = is.read_bool()?;
                },
                80 => {
                    self.NBLJPGFHDFI = is.read_uint32()?;
                },
                48 => {
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
        for value in &self.CJJAEDIALNG {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        if self.retcode != 0 {
            my_size += ::protobuf::rt::uint32_size(11, self.retcode);
        }
        if self.FCBODANKHEE != false {
            my_size += 1 + 1;
        }
        if self.NBLJPGFHDFI != 0 {
            my_size += ::protobuf::rt::uint32_size(10, self.NBLJPGFHDFI);
        }
        if self.CFNJJEJIGOK != 0 {
            my_size += ::protobuf::rt::uint32_size(6, self.CFNJJEJIGOK);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        for v in &self.CJJAEDIALNG {
            ::protobuf::rt::write_message_field_with_cached_size(7, v, os)?;
        };
        if self.retcode != 0 {
            os.write_uint32(11, self.retcode)?;
        }
        if self.FCBODANKHEE != false {
            os.write_bool(3, self.FCBODANKHEE)?;
        }
        if self.NBLJPGFHDFI != 0 {
            os.write_uint32(10, self.NBLJPGFHDFI)?;
        }
        if self.CFNJJEJIGOK != 0 {
            os.write_uint32(6, self.CFNJJEJIGOK)?;
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

    fn new() -> GetSaveRaidScRsp {
        GetSaveRaidScRsp::new()
    }

    fn clear(&mut self) {
        self.CJJAEDIALNG.clear();
        self.retcode = 0;
        self.FCBODANKHEE = false;
        self.NBLJPGFHDFI = 0;
        self.CFNJJEJIGOK = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static GetSaveRaidScRsp {
        static instance: GetSaveRaidScRsp = GetSaveRaidScRsp {
            CJJAEDIALNG: ::std::vec::Vec::new(),
            retcode: 0,
            FCBODANKHEE: false,
            NBLJPGFHDFI: 0,
            CFNJJEJIGOK: 0,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for GetSaveRaidScRsp {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("GetSaveRaidScRsp").unwrap()).clone()
    }
}

impl ::std::fmt::Display for GetSaveRaidScRsp {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for GetSaveRaidScRsp {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x16GetSaveRaidScRsp.proto\x1a\x11BCPDMEMPLBM.proto\"\xc2\x01\n\x10Get\
    SaveRaidScRsp\x12.\n\x0bCJJAEDIALNG\x18\x07\x20\x03(\x0b2\x0c.BCPDMEMPLB\
    MR\x0bCJJAEDIALNG\x12\x18\n\x07retcode\x18\x0b\x20\x01(\rR\x07retcode\
    \x12\x20\n\x0bFCBODANKHEE\x18\x03\x20\x01(\x08R\x0bFCBODANKHEE\x12\x20\n\
    \x0bNBLJPGFHDFI\x18\n\x20\x01(\rR\x0bNBLJPGFHDFI\x12\x20\n\x0bCFNJJEJIGO\
    K\x18\x06\x20\x01(\rR\x0bCFNJJEJIGOKb\x06proto3\
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
            deps.push(super::BCPDMEMPLBM::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(GetSaveRaidScRsp::generated_message_descriptor_data());
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
