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

//! Generated file from `SelectRogueCommonDialogueOptionScRsp.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:SelectRogueCommonDialogueOptionScRsp)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct SelectRogueCommonDialogueOptionScRsp {
    // message fields
    // @@protoc_insertion_point(field:SelectRogueCommonDialogueOptionScRsp.event_has_effect)
    pub event_has_effect: bool,
    // @@protoc_insertion_point(field:SelectRogueCommonDialogueOptionScRsp.event_unique_id)
    pub event_unique_id: u32,
    // @@protoc_insertion_point(field:SelectRogueCommonDialogueOptionScRsp.option_id)
    pub option_id: u32,
    // @@protoc_insertion_point(field:SelectRogueCommonDialogueOptionScRsp.retcode)
    pub retcode: u32,
    // @@protoc_insertion_point(field:SelectRogueCommonDialogueOptionScRsp.effect_event_id_list)
    pub effect_event_id_list: ::std::vec::Vec<u32>,
    // @@protoc_insertion_point(field:SelectRogueCommonDialogueOptionScRsp.dialogue_data)
    pub dialogue_data: ::protobuf::MessageField<super::RogueCommonDialogueDataInfo::RogueCommonDialogueDataInfo>,
    // special fields
    // @@protoc_insertion_point(special_field:SelectRogueCommonDialogueOptionScRsp.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a SelectRogueCommonDialogueOptionScRsp {
    fn default() -> &'a SelectRogueCommonDialogueOptionScRsp {
        <SelectRogueCommonDialogueOptionScRsp as ::protobuf::Message>::default_instance()
    }
}

impl SelectRogueCommonDialogueOptionScRsp {
    pub fn new() -> SelectRogueCommonDialogueOptionScRsp {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(6);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "event_has_effect",
            |m: &SelectRogueCommonDialogueOptionScRsp| { &m.event_has_effect },
            |m: &mut SelectRogueCommonDialogueOptionScRsp| { &mut m.event_has_effect },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "event_unique_id",
            |m: &SelectRogueCommonDialogueOptionScRsp| { &m.event_unique_id },
            |m: &mut SelectRogueCommonDialogueOptionScRsp| { &mut m.event_unique_id },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "option_id",
            |m: &SelectRogueCommonDialogueOptionScRsp| { &m.option_id },
            |m: &mut SelectRogueCommonDialogueOptionScRsp| { &mut m.option_id },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "retcode",
            |m: &SelectRogueCommonDialogueOptionScRsp| { &m.retcode },
            |m: &mut SelectRogueCommonDialogueOptionScRsp| { &mut m.retcode },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "effect_event_id_list",
            |m: &SelectRogueCommonDialogueOptionScRsp| { &m.effect_event_id_list },
            |m: &mut SelectRogueCommonDialogueOptionScRsp| { &mut m.effect_event_id_list },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::RogueCommonDialogueDataInfo::RogueCommonDialogueDataInfo>(
            "dialogue_data",
            |m: &SelectRogueCommonDialogueOptionScRsp| { &m.dialogue_data },
            |m: &mut SelectRogueCommonDialogueOptionScRsp| { &mut m.dialogue_data },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<SelectRogueCommonDialogueOptionScRsp>(
            "SelectRogueCommonDialogueOptionScRsp",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for SelectRogueCommonDialogueOptionScRsp {
    const NAME: &'static str = "SelectRogueCommonDialogueOptionScRsp";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                48 => {
                    self.event_has_effect = is.read_bool()?;
                },
                72 => {
                    self.event_unique_id = is.read_uint32()?;
                },
                16 => {
                    self.option_id = is.read_uint32()?;
                },
                64 => {
                    self.retcode = is.read_uint32()?;
                },
                26 => {
                    is.read_repeated_packed_uint32_into(&mut self.effect_event_id_list)?;
                },
                24 => {
                    self.effect_event_id_list.push(is.read_uint32()?);
                },
                82 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.dialogue_data)?;
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
        if self.event_has_effect != false {
            my_size += 1 + 1;
        }
        if self.event_unique_id != 0 {
            my_size += ::protobuf::rt::uint32_size(9, self.event_unique_id);
        }
        if self.option_id != 0 {
            my_size += ::protobuf::rt::uint32_size(2, self.option_id);
        }
        if self.retcode != 0 {
            my_size += ::protobuf::rt::uint32_size(8, self.retcode);
        }
        for value in &self.effect_event_id_list {
            my_size += ::protobuf::rt::uint32_size(3, *value);
        };
        if let Some(v) = self.dialogue_data.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.event_has_effect != false {
            os.write_bool(6, self.event_has_effect)?;
        }
        if self.event_unique_id != 0 {
            os.write_uint32(9, self.event_unique_id)?;
        }
        if self.option_id != 0 {
            os.write_uint32(2, self.option_id)?;
        }
        if self.retcode != 0 {
            os.write_uint32(8, self.retcode)?;
        }
        for v in &self.effect_event_id_list {
            os.write_uint32(3, *v)?;
        };
        if let Some(v) = self.dialogue_data.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(10, v, os)?;
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

    fn new() -> SelectRogueCommonDialogueOptionScRsp {
        SelectRogueCommonDialogueOptionScRsp::new()
    }

    fn clear(&mut self) {
        self.event_has_effect = false;
        self.event_unique_id = 0;
        self.option_id = 0;
        self.retcode = 0;
        self.effect_event_id_list.clear();
        self.dialogue_data.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static SelectRogueCommonDialogueOptionScRsp {
        static instance: SelectRogueCommonDialogueOptionScRsp = SelectRogueCommonDialogueOptionScRsp {
            event_has_effect: false,
            event_unique_id: 0,
            option_id: 0,
            retcode: 0,
            effect_event_id_list: ::std::vec::Vec::new(),
            dialogue_data: ::protobuf::MessageField::none(),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for SelectRogueCommonDialogueOptionScRsp {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("SelectRogueCommonDialogueOptionScRsp").unwrap()).clone()
    }
}

impl ::std::fmt::Display for SelectRogueCommonDialogueOptionScRsp {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for SelectRogueCommonDialogueOptionScRsp {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n*SelectRogueCommonDialogueOptionScRsp.proto\x1a!RogueCommonDialogueDat\
    aInfo.proto\"\xa3\x02\n$SelectRogueCommonDialogueOptionScRsp\x12(\n\x10e\
    vent_has_effect\x18\x06\x20\x01(\x08R\x0eeventHasEffect\x12&\n\x0fevent_\
    unique_id\x18\t\x20\x01(\rR\reventUniqueId\x12\x1b\n\toption_id\x18\x02\
    \x20\x01(\rR\x08optionId\x12\x18\n\x07retcode\x18\x08\x20\x01(\rR\x07ret\
    code\x12/\n\x14effect_event_id_list\x18\x03\x20\x03(\rR\x11effectEventId\
    List\x12A\n\rdialogue_data\x18\n\x20\x01(\x0b2\x1c.RogueCommonDialogueDa\
    taInfoR\x0cdialogueDataB\x15\n\x13emu.lunarcore.protob\x06proto3\
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
            deps.push(super::RogueCommonDialogueDataInfo::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(SelectRogueCommonDialogueOptionScRsp::generated_message_descriptor_data());
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
