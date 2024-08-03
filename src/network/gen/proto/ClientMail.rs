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

//! Generated file from `ClientMail.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:ClientMail)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct ClientMail {
    // message fields
    // @@protoc_insertion_point(field:ClientMail.is_read)
    pub is_read: bool,
    // @@protoc_insertion_point(field:ClientMail.expire_time)
    pub expire_time: i64,
    // @@protoc_insertion_point(field:ClientMail.template_id)
    pub template_id: u32,
    // @@protoc_insertion_point(field:ClientMail.attachment)
    pub attachment: ::protobuf::MessageField<super::ItemList::ItemList>,
    // @@protoc_insertion_point(field:ClientMail.title)
    pub title: ::std::string::String,
    // @@protoc_insertion_point(field:ClientMail.sender)
    pub sender: ::std::string::String,
    // @@protoc_insertion_point(field:ClientMail.para_list)
    pub para_list: ::std::vec::Vec<::std::string::String>,
    // @@protoc_insertion_point(field:ClientMail.id)
    pub id: u32,
    // @@protoc_insertion_point(field:ClientMail.content)
    pub content: ::std::string::String,
    // @@protoc_insertion_point(field:ClientMail.time)
    pub time: i64,
    // special fields
    // @@protoc_insertion_point(special_field:ClientMail.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a ClientMail {
    fn default() -> &'a ClientMail {
        <ClientMail as ::protobuf::Message>::default_instance()
    }
}

impl ClientMail {
    pub fn new() -> ClientMail {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(10);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "is_read",
            |m: &ClientMail| { &m.is_read },
            |m: &mut ClientMail| { &mut m.is_read },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "expire_time",
            |m: &ClientMail| { &m.expire_time },
            |m: &mut ClientMail| { &mut m.expire_time },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "template_id",
            |m: &ClientMail| { &m.template_id },
            |m: &mut ClientMail| { &mut m.template_id },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::ItemList::ItemList>(
            "attachment",
            |m: &ClientMail| { &m.attachment },
            |m: &mut ClientMail| { &mut m.attachment },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "title",
            |m: &ClientMail| { &m.title },
            |m: &mut ClientMail| { &mut m.title },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "sender",
            |m: &ClientMail| { &m.sender },
            |m: &mut ClientMail| { &mut m.sender },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "para_list",
            |m: &ClientMail| { &m.para_list },
            |m: &mut ClientMail| { &mut m.para_list },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "id",
            |m: &ClientMail| { &m.id },
            |m: &mut ClientMail| { &mut m.id },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "content",
            |m: &ClientMail| { &m.content },
            |m: &mut ClientMail| { &mut m.content },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "time",
            |m: &ClientMail| { &m.time },
            |m: &mut ClientMail| { &mut m.time },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<ClientMail>(
            "ClientMail",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for ClientMail {
    const NAME: &'static str = "ClientMail";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                32 => {
                    self.is_read = is.read_bool()?;
                },
                8 => {
                    self.expire_time = is.read_int64()?;
                },
                40 => {
                    self.template_id = is.read_uint32()?;
                },
                122 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.attachment)?;
                },
                66 => {
                    self.title = is.read_string()?;
                },
                74 => {
                    self.sender = is.read_string()?;
                },
                26 => {
                    self.para_list.push(is.read_string()?);
                },
                56 => {
                    self.id = is.read_uint32()?;
                },
                82 => {
                    self.content = is.read_string()?;
                },
                48 => {
                    self.time = is.read_int64()?;
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
        if self.is_read != false {
            my_size += 1 + 1;
        }
        if self.expire_time != 0 {
            my_size += ::protobuf::rt::int64_size(1, self.expire_time);
        }
        if self.template_id != 0 {
            my_size += ::protobuf::rt::uint32_size(5, self.template_id);
        }
        if let Some(v) = self.attachment.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if !self.title.is_empty() {
            my_size += ::protobuf::rt::string_size(8, &self.title);
        }
        if !self.sender.is_empty() {
            my_size += ::protobuf::rt::string_size(9, &self.sender);
        }
        for value in &self.para_list {
            my_size += ::protobuf::rt::string_size(3, &value);
        };
        if self.id != 0 {
            my_size += ::protobuf::rt::uint32_size(7, self.id);
        }
        if !self.content.is_empty() {
            my_size += ::protobuf::rt::string_size(10, &self.content);
        }
        if self.time != 0 {
            my_size += ::protobuf::rt::int64_size(6, self.time);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.is_read != false {
            os.write_bool(4, self.is_read)?;
        }
        if self.expire_time != 0 {
            os.write_int64(1, self.expire_time)?;
        }
        if self.template_id != 0 {
            os.write_uint32(5, self.template_id)?;
        }
        if let Some(v) = self.attachment.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(15, v, os)?;
        }
        if !self.title.is_empty() {
            os.write_string(8, &self.title)?;
        }
        if !self.sender.is_empty() {
            os.write_string(9, &self.sender)?;
        }
        for v in &self.para_list {
            os.write_string(3, &v)?;
        };
        if self.id != 0 {
            os.write_uint32(7, self.id)?;
        }
        if !self.content.is_empty() {
            os.write_string(10, &self.content)?;
        }
        if self.time != 0 {
            os.write_int64(6, self.time)?;
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

    fn new() -> ClientMail {
        ClientMail::new()
    }

    fn clear(&mut self) {
        self.is_read = false;
        self.expire_time = 0;
        self.template_id = 0;
        self.attachment.clear();
        self.title.clear();
        self.sender.clear();
        self.para_list.clear();
        self.id = 0;
        self.content.clear();
        self.time = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static ClientMail {
        static instance: ClientMail = ClientMail {
            is_read: false,
            expire_time: 0,
            template_id: 0,
            attachment: ::protobuf::MessageField::none(),
            title: ::std::string::String::new(),
            sender: ::std::string::String::new(),
            para_list: ::std::vec::Vec::new(),
            id: 0,
            content: ::std::string::String::new(),
            time: 0,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for ClientMail {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("ClientMail").unwrap()).clone()
    }
}

impl ::std::fmt::Display for ClientMail {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ClientMail {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x10ClientMail.proto\x1a\x0eItemList.proto\"\x9b\x02\n\nClientMail\x12\
    \x17\n\x07is_read\x18\x04\x20\x01(\x08R\x06isRead\x12\x1f\n\x0bexpire_ti\
    me\x18\x01\x20\x01(\x03R\nexpireTime\x12\x1f\n\x0btemplate_id\x18\x05\
    \x20\x01(\rR\ntemplateId\x12)\n\nattachment\x18\x0f\x20\x01(\x0b2\t.Item\
    ListR\nattachment\x12\x14\n\x05title\x18\x08\x20\x01(\tR\x05title\x12\
    \x16\n\x06sender\x18\t\x20\x01(\tR\x06sender\x12\x1b\n\tpara_list\x18\
    \x03\x20\x03(\tR\x08paraList\x12\x0e\n\x02id\x18\x07\x20\x01(\rR\x02id\
    \x12\x18\n\x07content\x18\n\x20\x01(\tR\x07content\x12\x12\n\x04time\x18\
    \x06\x20\x01(\x03R\x04timeB\x15\n\x13emu.lunarcore.protob\x06proto3\
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
            deps.push(super::ItemList::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(ClientMail::generated_message_descriptor_data());
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
