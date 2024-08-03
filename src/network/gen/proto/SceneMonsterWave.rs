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

//! Generated file from `SceneMonsterWave.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:SceneMonsterWave)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct SceneMonsterWave {
    // message fields
    // @@protoc_insertion_point(field:SceneMonsterWave.wave_id)
    pub wave_id: u32,
    // @@protoc_insertion_point(field:SceneMonsterWave.drop_list)
    pub drop_list: ::std::vec::Vec<super::ItemList::ItemList>,
    // @@protoc_insertion_point(field:SceneMonsterWave.stage_id)
    pub stage_id: u32,
    // @@protoc_insertion_point(field:SceneMonsterWave.monster_list)
    pub monster_list: ::std::vec::Vec<super::SceneMonster::SceneMonster>,
    // @@protoc_insertion_point(field:SceneMonsterWave.wave_param)
    pub wave_param: ::protobuf::MessageField<super::SceneMonsterWaveParam::SceneMonsterWaveParam>,
    // special fields
    // @@protoc_insertion_point(special_field:SceneMonsterWave.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a SceneMonsterWave {
    fn default() -> &'a SceneMonsterWave {
        <SceneMonsterWave as ::protobuf::Message>::default_instance()
    }
}

impl SceneMonsterWave {
    pub fn new() -> SceneMonsterWave {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(5);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "wave_id",
            |m: &SceneMonsterWave| { &m.wave_id },
            |m: &mut SceneMonsterWave| { &mut m.wave_id },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "drop_list",
            |m: &SceneMonsterWave| { &m.drop_list },
            |m: &mut SceneMonsterWave| { &mut m.drop_list },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "stage_id",
            |m: &SceneMonsterWave| { &m.stage_id },
            |m: &mut SceneMonsterWave| { &mut m.stage_id },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "monster_list",
            |m: &SceneMonsterWave| { &m.monster_list },
            |m: &mut SceneMonsterWave| { &mut m.monster_list },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::SceneMonsterWaveParam::SceneMonsterWaveParam>(
            "wave_param",
            |m: &SceneMonsterWave| { &m.wave_param },
            |m: &mut SceneMonsterWave| { &mut m.wave_param },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<SceneMonsterWave>(
            "SceneMonsterWave",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for SceneMonsterWave {
    const NAME: &'static str = "SceneMonsterWave";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                72 => {
                    self.wave_id = is.read_uint32()?;
                },
                42 => {
                    self.drop_list.push(is.read_message()?);
                },
                64 => {
                    self.stage_id = is.read_uint32()?;
                },
                98 => {
                    self.monster_list.push(is.read_message()?);
                },
                50 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.wave_param)?;
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
        if self.wave_id != 0 {
            my_size += ::protobuf::rt::uint32_size(9, self.wave_id);
        }
        for value in &self.drop_list {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        if self.stage_id != 0 {
            my_size += ::protobuf::rt::uint32_size(8, self.stage_id);
        }
        for value in &self.monster_list {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        if let Some(v) = self.wave_param.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.wave_id != 0 {
            os.write_uint32(9, self.wave_id)?;
        }
        for v in &self.drop_list {
            ::protobuf::rt::write_message_field_with_cached_size(5, v, os)?;
        };
        if self.stage_id != 0 {
            os.write_uint32(8, self.stage_id)?;
        }
        for v in &self.monster_list {
            ::protobuf::rt::write_message_field_with_cached_size(12, v, os)?;
        };
        if let Some(v) = self.wave_param.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(6, v, os)?;
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

    fn new() -> SceneMonsterWave {
        SceneMonsterWave::new()
    }

    fn clear(&mut self) {
        self.wave_id = 0;
        self.drop_list.clear();
        self.stage_id = 0;
        self.monster_list.clear();
        self.wave_param.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static SceneMonsterWave {
        static instance: SceneMonsterWave = SceneMonsterWave {
            wave_id: 0,
            drop_list: ::std::vec::Vec::new(),
            stage_id: 0,
            monster_list: ::std::vec::Vec::new(),
            wave_param: ::protobuf::MessageField::none(),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for SceneMonsterWave {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("SceneMonsterWave").unwrap()).clone()
    }
}

impl ::std::fmt::Display for SceneMonsterWave {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for SceneMonsterWave {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x16SceneMonsterWave.proto\x1a\x12SceneMonster.proto\x1a\x1bSceneMonst\
    erWaveParam.proto\x1a\x0eItemList.proto\"\xd7\x01\n\x10SceneMonsterWave\
    \x12\x17\n\x07wave_id\x18\t\x20\x01(\rR\x06waveId\x12&\n\tdrop_list\x18\
    \x05\x20\x03(\x0b2\t.ItemListR\x08dropList\x12\x19\n\x08stage_id\x18\x08\
    \x20\x01(\rR\x07stageId\x120\n\x0cmonster_list\x18\x0c\x20\x03(\x0b2\r.S\
    ceneMonsterR\x0bmonsterList\x125\n\nwave_param\x18\x06\x20\x01(\x0b2\x16\
    .SceneMonsterWaveParamR\twaveParamB\x15\n\x13emu.lunarcore.protob\x06pro\
    to3\
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
            deps.push(super::SceneMonster::file_descriptor().clone());
            deps.push(super::SceneMonsterWaveParam::file_descriptor().clone());
            deps.push(super::ItemList::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(SceneMonsterWave::generated_message_descriptor_data());
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
