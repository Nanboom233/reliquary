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

//! Generated file from `UpdateGroupPropertyScRsp.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:UpdateGroupPropertyScRsp)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct UpdateGroupPropertyScRsp {
    // message fields
    // @@protoc_insertion_point(field:UpdateGroupPropertyScRsp.BJNKEEFKAIK)
    pub BJNKEEFKAIK: u32,
    // @@protoc_insertion_point(field:UpdateGroupPropertyScRsp.retcode)
    pub retcode: u32,
    // @@protoc_insertion_point(field:UpdateGroupPropertyScRsp.HLOMDJJPMEP)
    pub HLOMDJJPMEP: ::std::string::String,
    // @@protoc_insertion_point(field:UpdateGroupPropertyScRsp.IOPPGEGDHGL)
    pub IOPPGEGDHGL: u32,
    // @@protoc_insertion_point(field:UpdateGroupPropertyScRsp.FCDAEPHKFOK)
    pub FCDAEPHKFOK: i32,
    // @@protoc_insertion_point(field:UpdateGroupPropertyScRsp.OLGCJIKGFFC)
    pub OLGCJIKGFFC: u32,
    // @@protoc_insertion_point(field:UpdateGroupPropertyScRsp.NNBJMFPPEAI)
    pub NNBJMFPPEAI: i32,
    // special fields
    // @@protoc_insertion_point(special_field:UpdateGroupPropertyScRsp.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a UpdateGroupPropertyScRsp {
    fn default() -> &'a UpdateGroupPropertyScRsp {
        <UpdateGroupPropertyScRsp as ::protobuf::Message>::default_instance()
    }
}

impl UpdateGroupPropertyScRsp {
    pub fn new() -> UpdateGroupPropertyScRsp {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(7);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "BJNKEEFKAIK",
            |m: &UpdateGroupPropertyScRsp| { &m.BJNKEEFKAIK },
            |m: &mut UpdateGroupPropertyScRsp| { &mut m.BJNKEEFKAIK },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "retcode",
            |m: &UpdateGroupPropertyScRsp| { &m.retcode },
            |m: &mut UpdateGroupPropertyScRsp| { &mut m.retcode },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "HLOMDJJPMEP",
            |m: &UpdateGroupPropertyScRsp| { &m.HLOMDJJPMEP },
            |m: &mut UpdateGroupPropertyScRsp| { &mut m.HLOMDJJPMEP },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "IOPPGEGDHGL",
            |m: &UpdateGroupPropertyScRsp| { &m.IOPPGEGDHGL },
            |m: &mut UpdateGroupPropertyScRsp| { &mut m.IOPPGEGDHGL },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "FCDAEPHKFOK",
            |m: &UpdateGroupPropertyScRsp| { &m.FCDAEPHKFOK },
            |m: &mut UpdateGroupPropertyScRsp| { &mut m.FCDAEPHKFOK },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "OLGCJIKGFFC",
            |m: &UpdateGroupPropertyScRsp| { &m.OLGCJIKGFFC },
            |m: &mut UpdateGroupPropertyScRsp| { &mut m.OLGCJIKGFFC },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "NNBJMFPPEAI",
            |m: &UpdateGroupPropertyScRsp| { &m.NNBJMFPPEAI },
            |m: &mut UpdateGroupPropertyScRsp| { &mut m.NNBJMFPPEAI },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<UpdateGroupPropertyScRsp>(
            "UpdateGroupPropertyScRsp",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for UpdateGroupPropertyScRsp {
    const NAME: &'static str = "UpdateGroupPropertyScRsp";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                40 => {
                    self.BJNKEEFKAIK = is.read_uint32()?;
                },
                112 => {
                    self.retcode = is.read_uint32()?;
                },
                82 => {
                    self.HLOMDJJPMEP = is.read_string()?;
                },
                104 => {
                    self.IOPPGEGDHGL = is.read_uint32()?;
                },
                88 => {
                    self.FCDAEPHKFOK = is.read_int32()?;
                },
                24 => {
                    self.OLGCJIKGFFC = is.read_uint32()?;
                },
                16 => {
                    self.NNBJMFPPEAI = is.read_int32()?;
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
        if self.BJNKEEFKAIK != 0 {
            my_size += ::protobuf::rt::uint32_size(5, self.BJNKEEFKAIK);
        }
        if self.retcode != 0 {
            my_size += ::protobuf::rt::uint32_size(14, self.retcode);
        }
        if !self.HLOMDJJPMEP.is_empty() {
            my_size += ::protobuf::rt::string_size(10, &self.HLOMDJJPMEP);
        }
        if self.IOPPGEGDHGL != 0 {
            my_size += ::protobuf::rt::uint32_size(13, self.IOPPGEGDHGL);
        }
        if self.FCDAEPHKFOK != 0 {
            my_size += ::protobuf::rt::int32_size(11, self.FCDAEPHKFOK);
        }
        if self.OLGCJIKGFFC != 0 {
            my_size += ::protobuf::rt::uint32_size(3, self.OLGCJIKGFFC);
        }
        if self.NNBJMFPPEAI != 0 {
            my_size += ::protobuf::rt::int32_size(2, self.NNBJMFPPEAI);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.BJNKEEFKAIK != 0 {
            os.write_uint32(5, self.BJNKEEFKAIK)?;
        }
        if self.retcode != 0 {
            os.write_uint32(14, self.retcode)?;
        }
        if !self.HLOMDJJPMEP.is_empty() {
            os.write_string(10, &self.HLOMDJJPMEP)?;
        }
        if self.IOPPGEGDHGL != 0 {
            os.write_uint32(13, self.IOPPGEGDHGL)?;
        }
        if self.FCDAEPHKFOK != 0 {
            os.write_int32(11, self.FCDAEPHKFOK)?;
        }
        if self.OLGCJIKGFFC != 0 {
            os.write_uint32(3, self.OLGCJIKGFFC)?;
        }
        if self.NNBJMFPPEAI != 0 {
            os.write_int32(2, self.NNBJMFPPEAI)?;
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

    fn new() -> UpdateGroupPropertyScRsp {
        UpdateGroupPropertyScRsp::new()
    }

    fn clear(&mut self) {
        self.BJNKEEFKAIK = 0;
        self.retcode = 0;
        self.HLOMDJJPMEP.clear();
        self.IOPPGEGDHGL = 0;
        self.FCDAEPHKFOK = 0;
        self.OLGCJIKGFFC = 0;
        self.NNBJMFPPEAI = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static UpdateGroupPropertyScRsp {
        static instance: UpdateGroupPropertyScRsp = UpdateGroupPropertyScRsp {
            BJNKEEFKAIK: 0,
            retcode: 0,
            HLOMDJJPMEP: ::std::string::String::new(),
            IOPPGEGDHGL: 0,
            FCDAEPHKFOK: 0,
            OLGCJIKGFFC: 0,
            NNBJMFPPEAI: 0,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for UpdateGroupPropertyScRsp {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("UpdateGroupPropertyScRsp").unwrap()).clone()
    }
}

impl ::std::fmt::Display for UpdateGroupPropertyScRsp {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for UpdateGroupPropertyScRsp {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x1eUpdateGroupPropertyScRsp.proto\"\x80\x02\n\x18UpdateGroupPropertyS\
    cRsp\x12\x20\n\x0bBJNKEEFKAIK\x18\x05\x20\x01(\rR\x0bBJNKEEFKAIK\x12\x18\
    \n\x07retcode\x18\x0e\x20\x01(\rR\x07retcode\x12\x20\n\x0bHLOMDJJPMEP\
    \x18\n\x20\x01(\tR\x0bHLOMDJJPMEP\x12\x20\n\x0bIOPPGEGDHGL\x18\r\x20\x01\
    (\rR\x0bIOPPGEGDHGL\x12\x20\n\x0bFCDAEPHKFOK\x18\x0b\x20\x01(\x05R\x0bFC\
    DAEPHKFOK\x12\x20\n\x0bOLGCJIKGFFC\x18\x03\x20\x01(\rR\x0bOLGCJIKGFFC\
    \x12\x20\n\x0bNNBJMFPPEAI\x18\x02\x20\x01(\x05R\x0bNNBJMFPPEAIb\x06proto\
    3\
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
            messages.push(UpdateGroupPropertyScRsp::generated_message_descriptor_data());
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
