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

//! Generated file from `HeliobusStartRaidCsReq.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:HeliobusStartRaidCsReq)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct HeliobusStartRaidCsReq {
    // message fields
    // @@protoc_insertion_point(field:HeliobusStartRaidCsReq.FCBODANKHEE)
    pub FCBODANKHEE: bool,
    // @@protoc_insertion_point(field:HeliobusStartRaidCsReq.CFAPNLFIMLN)
    pub CFAPNLFIMLN: u32,
    // @@protoc_insertion_point(field:HeliobusStartRaidCsReq.NBLJPGFHDFI)
    pub NBLJPGFHDFI: u32,
    // @@protoc_insertion_point(field:HeliobusStartRaidCsReq.avatar_list)
    pub avatar_list: ::std::vec::Vec<u32>,
    // @@protoc_insertion_point(field:HeliobusStartRaidCsReq.LCLCDINHHNP)
    pub LCLCDINHHNP: u32,
    // @@protoc_insertion_point(field:HeliobusStartRaidCsReq.AKJKBEKAKCN)
    pub AKJKBEKAKCN: u32,
    // special fields
    // @@protoc_insertion_point(special_field:HeliobusStartRaidCsReq.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a HeliobusStartRaidCsReq {
    fn default() -> &'a HeliobusStartRaidCsReq {
        <HeliobusStartRaidCsReq as ::protobuf::Message>::default_instance()
    }
}

impl HeliobusStartRaidCsReq {
    pub fn new() -> HeliobusStartRaidCsReq {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(6);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "FCBODANKHEE",
            |m: &HeliobusStartRaidCsReq| { &m.FCBODANKHEE },
            |m: &mut HeliobusStartRaidCsReq| { &mut m.FCBODANKHEE },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "CFAPNLFIMLN",
            |m: &HeliobusStartRaidCsReq| { &m.CFAPNLFIMLN },
            |m: &mut HeliobusStartRaidCsReq| { &mut m.CFAPNLFIMLN },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "NBLJPGFHDFI",
            |m: &HeliobusStartRaidCsReq| { &m.NBLJPGFHDFI },
            |m: &mut HeliobusStartRaidCsReq| { &mut m.NBLJPGFHDFI },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "avatar_list",
            |m: &HeliobusStartRaidCsReq| { &m.avatar_list },
            |m: &mut HeliobusStartRaidCsReq| { &mut m.avatar_list },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "LCLCDINHHNP",
            |m: &HeliobusStartRaidCsReq| { &m.LCLCDINHHNP },
            |m: &mut HeliobusStartRaidCsReq| { &mut m.LCLCDINHHNP },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "AKJKBEKAKCN",
            |m: &HeliobusStartRaidCsReq| { &m.AKJKBEKAKCN },
            |m: &mut HeliobusStartRaidCsReq| { &mut m.AKJKBEKAKCN },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<HeliobusStartRaidCsReq>(
            "HeliobusStartRaidCsReq",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for HeliobusStartRaidCsReq {
    const NAME: &'static str = "HeliobusStartRaidCsReq";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                104 => {
                    self.FCBODANKHEE = is.read_bool()?;
                },
                16 => {
                    self.CFAPNLFIMLN = is.read_uint32()?;
                },
                48 => {
                    self.NBLJPGFHDFI = is.read_uint32()?;
                },
                10 => {
                    is.read_repeated_packed_uint32_into(&mut self.avatar_list)?;
                },
                8 => {
                    self.avatar_list.push(is.read_uint32()?);
                },
                40 => {
                    self.LCLCDINHHNP = is.read_uint32()?;
                },
                56 => {
                    self.AKJKBEKAKCN = is.read_uint32()?;
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
        if self.FCBODANKHEE != false {
            my_size += 1 + 1;
        }
        if self.CFAPNLFIMLN != 0 {
            my_size += ::protobuf::rt::uint32_size(2, self.CFAPNLFIMLN);
        }
        if self.NBLJPGFHDFI != 0 {
            my_size += ::protobuf::rt::uint32_size(6, self.NBLJPGFHDFI);
        }
        for value in &self.avatar_list {
            my_size += ::protobuf::rt::uint32_size(1, *value);
        };
        if self.LCLCDINHHNP != 0 {
            my_size += ::protobuf::rt::uint32_size(5, self.LCLCDINHHNP);
        }
        if self.AKJKBEKAKCN != 0 {
            my_size += ::protobuf::rt::uint32_size(7, self.AKJKBEKAKCN);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.FCBODANKHEE != false {
            os.write_bool(13, self.FCBODANKHEE)?;
        }
        if self.CFAPNLFIMLN != 0 {
            os.write_uint32(2, self.CFAPNLFIMLN)?;
        }
        if self.NBLJPGFHDFI != 0 {
            os.write_uint32(6, self.NBLJPGFHDFI)?;
        }
        for v in &self.avatar_list {
            os.write_uint32(1, *v)?;
        };
        if self.LCLCDINHHNP != 0 {
            os.write_uint32(5, self.LCLCDINHHNP)?;
        }
        if self.AKJKBEKAKCN != 0 {
            os.write_uint32(7, self.AKJKBEKAKCN)?;
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

    fn new() -> HeliobusStartRaidCsReq {
        HeliobusStartRaidCsReq::new()
    }

    fn clear(&mut self) {
        self.FCBODANKHEE = false;
        self.CFAPNLFIMLN = 0;
        self.NBLJPGFHDFI = 0;
        self.avatar_list.clear();
        self.LCLCDINHHNP = 0;
        self.AKJKBEKAKCN = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static HeliobusStartRaidCsReq {
        static instance: HeliobusStartRaidCsReq = HeliobusStartRaidCsReq {
            FCBODANKHEE: false,
            CFAPNLFIMLN: 0,
            NBLJPGFHDFI: 0,
            avatar_list: ::std::vec::Vec::new(),
            LCLCDINHHNP: 0,
            AKJKBEKAKCN: 0,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for HeliobusStartRaidCsReq {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("HeliobusStartRaidCsReq").unwrap()).clone()
    }
}

impl ::std::fmt::Display for HeliobusStartRaidCsReq {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for HeliobusStartRaidCsReq {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x1cHeliobusStartRaidCsReq.proto\"\xe3\x01\n\x16HeliobusStartRaidCsReq\
    \x12\x20\n\x0bFCBODANKHEE\x18\r\x20\x01(\x08R\x0bFCBODANKHEE\x12\x20\n\
    \x0bCFAPNLFIMLN\x18\x02\x20\x01(\rR\x0bCFAPNLFIMLN\x12\x20\n\x0bNBLJPGFH\
    DFI\x18\x06\x20\x01(\rR\x0bNBLJPGFHDFI\x12\x1f\n\x0bavatar_list\x18\x01\
    \x20\x03(\rR\navatarList\x12\x20\n\x0bLCLCDINHHNP\x18\x05\x20\x01(\rR\
    \x0bLCLCDINHHNP\x12\x20\n\x0bAKJKBEKAKCN\x18\x07\x20\x01(\rR\x0bAKJKBEKA\
    KCNb\x06proto3\
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
            messages.push(HeliobusStartRaidCsReq::generated_message_descriptor_data());
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
