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

//! Generated file from `SceneBattleInfo.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:SceneBattleInfo)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct SceneBattleInfo {
    // message fields
    // @@protoc_insertion_point(field:SceneBattleInfo.monster_wave_list)
    pub monster_wave_list: ::std::vec::Vec<super::SceneMonsterWave::SceneMonsterWave>,
    // @@protoc_insertion_point(field:SceneBattleInfo.battle_target_info)
    pub battle_target_info: ::std::collections::HashMap<u32, super::BattleTargetList::BattleTargetList>,
    // @@protoc_insertion_point(field:SceneBattleInfo.battle_id)
    pub battle_id: u32,
    // @@protoc_insertion_point(field:SceneBattleInfo.world_level)
    pub world_level: u32,
    // @@protoc_insertion_point(field:SceneBattleInfo.buff_list)
    pub buff_list: ::std::vec::Vec<super::BattleBuff::BattleBuff>,
    // @@protoc_insertion_point(field:SceneBattleInfo.rounds_limit)
    pub rounds_limit: u32,
    // @@protoc_insertion_point(field:SceneBattleInfo.logic_random_seed)
    pub logic_random_seed: u32,
    // @@protoc_insertion_point(field:SceneBattleInfo.battle_avatar_list)
    pub battle_avatar_list: ::std::vec::Vec<super::BattleAvatar::BattleAvatar>,
    // @@protoc_insertion_point(field:SceneBattleInfo.event_battle_info_list)
    pub event_battle_info_list: ::std::vec::Vec<super::BattleEventBattleInfo::BattleEventBattleInfo>,
    // @@protoc_insertion_point(field:SceneBattleInfo.stage_id)
    pub stage_id: u32,
    // special fields
    // @@protoc_insertion_point(special_field:SceneBattleInfo.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a SceneBattleInfo {
    fn default() -> &'a SceneBattleInfo {
        <SceneBattleInfo as ::protobuf::Message>::default_instance()
    }
}

impl SceneBattleInfo {
    pub fn new() -> SceneBattleInfo {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(10);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "monster_wave_list",
            |m: &SceneBattleInfo| { &m.monster_wave_list },
            |m: &mut SceneBattleInfo| { &mut m.monster_wave_list },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_map_simpler_accessor::<_, _, _>(
            "battle_target_info",
            |m: &SceneBattleInfo| { &m.battle_target_info },
            |m: &mut SceneBattleInfo| { &mut m.battle_target_info },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "battle_id",
            |m: &SceneBattleInfo| { &m.battle_id },
            |m: &mut SceneBattleInfo| { &mut m.battle_id },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "world_level",
            |m: &SceneBattleInfo| { &m.world_level },
            |m: &mut SceneBattleInfo| { &mut m.world_level },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "buff_list",
            |m: &SceneBattleInfo| { &m.buff_list },
            |m: &mut SceneBattleInfo| { &mut m.buff_list },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "rounds_limit",
            |m: &SceneBattleInfo| { &m.rounds_limit },
            |m: &mut SceneBattleInfo| { &mut m.rounds_limit },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "logic_random_seed",
            |m: &SceneBattleInfo| { &m.logic_random_seed },
            |m: &mut SceneBattleInfo| { &mut m.logic_random_seed },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "battle_avatar_list",
            |m: &SceneBattleInfo| { &m.battle_avatar_list },
            |m: &mut SceneBattleInfo| { &mut m.battle_avatar_list },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "event_battle_info_list",
            |m: &SceneBattleInfo| { &m.event_battle_info_list },
            |m: &mut SceneBattleInfo| { &mut m.event_battle_info_list },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "stage_id",
            |m: &SceneBattleInfo| { &m.stage_id },
            |m: &mut SceneBattleInfo| { &mut m.stage_id },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<SceneBattleInfo>(
            "SceneBattleInfo",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for SceneBattleInfo {
    const NAME: &'static str = "SceneBattleInfo";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                10 => {
                    self.monster_wave_list.push(is.read_message()?);
                },
                1194 => {
                    let len = is.read_raw_varint32()?;
                    let old_limit = is.push_limit(len as u64)?;
                    let mut key = ::std::default::Default::default();
                    let mut value = ::std::default::Default::default();
                    while let Some(tag) = is.read_raw_tag_or_eof()? {
                        match tag {
                            8 => key = is.read_uint32()?,
                            18 => value = is.read_message()?,
                            _ => ::protobuf::rt::skip_field_for_tag(tag, is)?,
                        };
                    }
                    is.pop_limit(old_limit);
                    self.battle_target_info.insert(key, value);
                },
                112 => {
                    self.battle_id = is.read_uint32()?;
                },
                48 => {
                    self.world_level = is.read_uint32()?;
                },
                26 => {
                    self.buff_list.push(is.read_message()?);
                },
                16 => {
                    self.rounds_limit = is.read_uint32()?;
                },
                96 => {
                    self.logic_random_seed = is.read_uint32()?;
                },
                122 => {
                    self.battle_avatar_list.push(is.read_message()?);
                },
                15722 => {
                    self.event_battle_info_list.push(is.read_message()?);
                },
                32 => {
                    self.stage_id = is.read_uint32()?;
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
        for value in &self.monster_wave_list {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        for (k, v) in &self.battle_target_info {
            let mut entry_size = 0;
            entry_size += ::protobuf::rt::uint32_size(1, *k);
            let len = v.compute_size();
            entry_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
            my_size += 2 + ::protobuf::rt::compute_raw_varint64_size(entry_size) + entry_size
        };
        if self.battle_id != 0 {
            my_size += ::protobuf::rt::uint32_size(14, self.battle_id);
        }
        if self.world_level != 0 {
            my_size += ::protobuf::rt::uint32_size(6, self.world_level);
        }
        for value in &self.buff_list {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        if self.rounds_limit != 0 {
            my_size += ::protobuf::rt::uint32_size(2, self.rounds_limit);
        }
        if self.logic_random_seed != 0 {
            my_size += ::protobuf::rt::uint32_size(12, self.logic_random_seed);
        }
        for value in &self.battle_avatar_list {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        for value in &self.event_battle_info_list {
            let len = value.compute_size();
            my_size += 2 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        if self.stage_id != 0 {
            my_size += ::protobuf::rt::uint32_size(4, self.stage_id);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        for v in &self.monster_wave_list {
            ::protobuf::rt::write_message_field_with_cached_size(1, v, os)?;
        };
        for (k, v) in &self.battle_target_info {
            let mut entry_size = 0;
            entry_size += ::protobuf::rt::uint32_size(1, *k);
            let len = v.cached_size() as u64;
            entry_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
            os.write_raw_varint32(1194)?; // Tag.
            os.write_raw_varint32(entry_size as u32)?;
            os.write_uint32(1, *k)?;
            ::protobuf::rt::write_message_field_with_cached_size(2, v, os)?;
        };
        if self.battle_id != 0 {
            os.write_uint32(14, self.battle_id)?;
        }
        if self.world_level != 0 {
            os.write_uint32(6, self.world_level)?;
        }
        for v in &self.buff_list {
            ::protobuf::rt::write_message_field_with_cached_size(3, v, os)?;
        };
        if self.rounds_limit != 0 {
            os.write_uint32(2, self.rounds_limit)?;
        }
        if self.logic_random_seed != 0 {
            os.write_uint32(12, self.logic_random_seed)?;
        }
        for v in &self.battle_avatar_list {
            ::protobuf::rt::write_message_field_with_cached_size(15, v, os)?;
        };
        for v in &self.event_battle_info_list {
            ::protobuf::rt::write_message_field_with_cached_size(1965, v, os)?;
        };
        if self.stage_id != 0 {
            os.write_uint32(4, self.stage_id)?;
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

    fn new() -> SceneBattleInfo {
        SceneBattleInfo::new()
    }

    fn clear(&mut self) {
        self.monster_wave_list.clear();
        self.battle_target_info.clear();
        self.battle_id = 0;
        self.world_level = 0;
        self.buff_list.clear();
        self.rounds_limit = 0;
        self.logic_random_seed = 0;
        self.battle_avatar_list.clear();
        self.event_battle_info_list.clear();
        self.stage_id = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static SceneBattleInfo {
        static instance: ::protobuf::rt::Lazy<SceneBattleInfo> = ::protobuf::rt::Lazy::new();
        instance.get(SceneBattleInfo::new)
    }
}

impl ::protobuf::MessageFull for SceneBattleInfo {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("SceneBattleInfo").unwrap()).clone()
    }
}

impl ::std::fmt::Display for SceneBattleInfo {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for SceneBattleInfo {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x15SceneBattleInfo.proto\x1a\x16SceneMonsterWave.proto\x1a\x12BattleA\
    vatar.proto\x1a\x10BattleBuff.proto\x1a\x1bBattleEventBattleInfo.proto\
    \x1a\x16BattleTargetList.proto\"\xdc\x04\n\x0fSceneBattleInfo\x12=\n\x11\
    monster_wave_list\x18\x01\x20\x03(\x0b2\x11.SceneMonsterWaveR\x0fmonster\
    WaveList\x12U\n\x12battle_target_info\x18\x95\x01\x20\x03(\x0b2&.SceneBa\
    ttleInfo.BattleTargetInfoEntryR\x10battleTargetInfo\x12\x1b\n\tbattle_id\
    \x18\x0e\x20\x01(\rR\x08battleId\x12\x1f\n\x0bworld_level\x18\x06\x20\
    \x01(\rR\nworldLevel\x12(\n\tbuff_list\x18\x03\x20\x03(\x0b2\x0b.BattleB\
    uffR\x08buffList\x12!\n\x0crounds_limit\x18\x02\x20\x01(\rR\x0broundsLim\
    it\x12*\n\x11logic_random_seed\x18\x0c\x20\x01(\rR\x0flogicRandomSeed\
    \x12;\n\x12battle_avatar_list\x18\x0f\x20\x03(\x0b2\r.BattleAvatarR\x10b\
    attleAvatarList\x12L\n\x16event_battle_info_list\x18\xad\x0f\x20\x03(\
    \x0b2\x16.BattleEventBattleInfoR\x13eventBattleInfoList\x12\x19\n\x08sta\
    ge_id\x18\x04\x20\x01(\rR\x07stageId\x1aV\n\x15BattleTargetInfoEntry\x12\
    \x10\n\x03key\x18\x01\x20\x01(\rR\x03key\x12'\n\x05value\x18\x02\x20\x01\
    (\x0b2\x11.BattleTargetListR\x05value:\x028\x01B\x15\n\x13emu.lunarcore.\
    protob\x06proto3\
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
            let mut deps = ::std::vec::Vec::with_capacity(5);
            deps.push(super::SceneMonsterWave::file_descriptor().clone());
            deps.push(super::BattleAvatar::file_descriptor().clone());
            deps.push(super::BattleBuff::file_descriptor().clone());
            deps.push(super::BattleEventBattleInfo::file_descriptor().clone());
            deps.push(super::BattleTargetList::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(SceneBattleInfo::generated_message_descriptor_data());
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
