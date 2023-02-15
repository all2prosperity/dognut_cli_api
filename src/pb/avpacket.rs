// This file is generated by rust-protobuf 3.2.0. Do not edit
// .proto file is parsed by protoc --rust-out=...
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

//! Generated file from `avpacket.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_2_0;

#[derive(PartialEq,Clone,Default,Debug)]
// @@protoc_insertion_point(message:proto.avpacket.VideoHeader)
pub struct VideoHeader {
    // special fields
    // @@protoc_insertion_point(special_field:proto.avpacket.VideoHeader.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a VideoHeader {
    fn default() -> &'a VideoHeader {
        <VideoHeader as ::protobuf::Message>::default_instance()
    }
}

impl VideoHeader {
    pub fn new() -> VideoHeader {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(0);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<VideoHeader>(
            "VideoHeader",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for VideoHeader {
    const NAME: &'static str = "VideoHeader";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
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
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        os.write_unknown_fields(self.special_fields.unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn special_fields(&self) -> &::protobuf::SpecialFields {
        &self.special_fields
    }

    fn mut_special_fields(&mut self) -> &mut ::protobuf::SpecialFields {
        &mut self.special_fields
    }

    fn new() -> VideoHeader {
        VideoHeader::new()
    }

    fn clear(&mut self) {
        self.special_fields.clear();
    }

    fn default_instance() -> &'static VideoHeader {
        static instance: VideoHeader = VideoHeader {
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for VideoHeader {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("VideoHeader").unwrap()).clone()
    }
}

impl ::std::fmt::Display for VideoHeader {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for VideoHeader {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

#[derive(PartialEq,Clone,Default,Debug)]
// @@protoc_insertion_point(message:proto.avpacket.VideoPacket)
pub struct VideoPacket {
    // message fields
    // @@protoc_insertion_point(field:proto.avpacket.VideoPacket.data)
    pub data: ::std::vec::Vec<u8>,
    // @@protoc_insertion_point(field:proto.avpacket.VideoPacket.data_len)
    pub data_len: u32,
    // @@protoc_insertion_point(field:proto.avpacket.VideoPacket.pts)
    pub pts: i64,
    // @@protoc_insertion_point(field:proto.avpacket.VideoPacket.dts)
    pub dts: i64,
    // @@protoc_insertion_point(field:proto.avpacket.VideoPacket.duration)
    pub duration: i64,
    // @@protoc_insertion_point(field:proto.avpacket.VideoPacket.flags)
    pub flags: i32,
    // @@protoc_insertion_point(field:proto.avpacket.VideoPacket.idr_frame)
    pub idr_frame: bool,
    // special fields
    // @@protoc_insertion_point(special_field:proto.avpacket.VideoPacket.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a VideoPacket {
    fn default() -> &'a VideoPacket {
        <VideoPacket as ::protobuf::Message>::default_instance()
    }
}

impl VideoPacket {
    pub fn new() -> VideoPacket {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(7);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "data",
            |m: &VideoPacket| { &m.data },
            |m: &mut VideoPacket| { &mut m.data },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "data_len",
            |m: &VideoPacket| { &m.data_len },
            |m: &mut VideoPacket| { &mut m.data_len },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "pts",
            |m: &VideoPacket| { &m.pts },
            |m: &mut VideoPacket| { &mut m.pts },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "dts",
            |m: &VideoPacket| { &m.dts },
            |m: &mut VideoPacket| { &mut m.dts },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "duration",
            |m: &VideoPacket| { &m.duration },
            |m: &mut VideoPacket| { &mut m.duration },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "flags",
            |m: &VideoPacket| { &m.flags },
            |m: &mut VideoPacket| { &mut m.flags },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "idr_frame",
            |m: &VideoPacket| { &m.idr_frame },
            |m: &mut VideoPacket| { &mut m.idr_frame },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<VideoPacket>(
            "VideoPacket",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for VideoPacket {
    const NAME: &'static str = "VideoPacket";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                10 => {
                    self.data = is.read_bytes()?;
                },
                16 => {
                    self.data_len = is.read_uint32()?;
                },
                24 => {
                    self.pts = is.read_int64()?;
                },
                32 => {
                    self.dts = is.read_int64()?;
                },
                40 => {
                    self.duration = is.read_int64()?;
                },
                48 => {
                    self.flags = is.read_int32()?;
                },
                56 => {
                    self.idr_frame = is.read_bool()?;
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
        if !self.data.is_empty() {
            my_size += ::protobuf::rt::bytes_size(1, &self.data);
        }
        if self.data_len != 0 {
            my_size += ::protobuf::rt::uint32_size(2, self.data_len);
        }
        if self.pts != 0 {
            my_size += ::protobuf::rt::int64_size(3, self.pts);
        }
        if self.dts != 0 {
            my_size += ::protobuf::rt::int64_size(4, self.dts);
        }
        if self.duration != 0 {
            my_size += ::protobuf::rt::int64_size(5, self.duration);
        }
        if self.flags != 0 {
            my_size += ::protobuf::rt::int32_size(6, self.flags);
        }
        if self.idr_frame != false {
            my_size += 1 + 1;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if !self.data.is_empty() {
            os.write_bytes(1, &self.data)?;
        }
        if self.data_len != 0 {
            os.write_uint32(2, self.data_len)?;
        }
        if self.pts != 0 {
            os.write_int64(3, self.pts)?;
        }
        if self.dts != 0 {
            os.write_int64(4, self.dts)?;
        }
        if self.duration != 0 {
            os.write_int64(5, self.duration)?;
        }
        if self.flags != 0 {
            os.write_int32(6, self.flags)?;
        }
        if self.idr_frame != false {
            os.write_bool(7, self.idr_frame)?;
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

    fn new() -> VideoPacket {
        VideoPacket::new()
    }

    fn clear(&mut self) {
        self.data.clear();
        self.data_len = 0;
        self.pts = 0;
        self.dts = 0;
        self.duration = 0;
        self.flags = 0;
        self.idr_frame = false;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static VideoPacket {
        static instance: VideoPacket = VideoPacket {
            data: ::std::vec::Vec::new(),
            data_len: 0,
            pts: 0,
            dts: 0,
            duration: 0,
            flags: 0,
            idr_frame: false,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for VideoPacket {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("VideoPacket").unwrap()).clone()
    }
}

impl ::std::fmt::Display for VideoPacket {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for VideoPacket {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x0eavpacket.proto\x12\x0eproto.avpacket\"\r\n\x0bVideoHeader\"\xaf\
    \x01\n\x0bVideoPacket\x12\x12\n\x04data\x18\x01\x20\x01(\x0cR\x04data\
    \x12\x19\n\x08data_len\x18\x02\x20\x01(\rR\x07dataLen\x12\x10\n\x03pts\
    \x18\x03\x20\x01(\x03R\x03pts\x12\x10\n\x03dts\x18\x04\x20\x01(\x03R\x03\
    dts\x12\x1a\n\x08duration\x18\x05\x20\x01(\x03R\x08duration\x12\x14\n\
    \x05flags\x18\x06\x20\x01(\x05R\x05flags\x12\x1b\n\tidr_frame\x18\x07\
    \x20\x01(\x08R\x08idrFrameJ\xcd\x03\n\x06\x12\x04\0\0\x0f\x01\n\x08\n\
    \x01\x0c\x12\x03\0\0\x12\n\x08\n\x01\x02\x12\x03\x01\0\x17\n\n\n\x02\x04\
    \0\x12\x04\x03\0\x05\x01\n\n\n\x03\x04\0\x01\x12\x03\x03\x08\x13\n\n\n\
    \x02\x04\x01\x12\x04\x07\0\x0f\x01\n\n\n\x03\x04\x01\x01\x12\x03\x07\x08\
    \x13\n\x0b\n\x04\x04\x01\x02\0\x12\x03\x08\x02\x11\n\x0c\n\x05\x04\x01\
    \x02\0\x05\x12\x03\x08\x02\x07\n\x0c\n\x05\x04\x01\x02\0\x01\x12\x03\x08\
    \x08\x0c\n\x0c\n\x05\x04\x01\x02\0\x03\x12\x03\x08\x0f\x10\n\x0b\n\x04\
    \x04\x01\x02\x01\x12\x03\t\x02\x16\n\x0c\n\x05\x04\x01\x02\x01\x05\x12\
    \x03\t\x02\x08\n\x0c\n\x05\x04\x01\x02\x01\x01\x12\x03\t\t\x11\n\x0c\n\
    \x05\x04\x01\x02\x01\x03\x12\x03\t\x14\x15\n\x0b\n\x04\x04\x01\x02\x02\
    \x12\x03\n\x02\x10\n\x0c\n\x05\x04\x01\x02\x02\x05\x12\x03\n\x02\x07\n\
    \x0c\n\x05\x04\x01\x02\x02\x01\x12\x03\n\x08\x0b\n\x0c\n\x05\x04\x01\x02\
    \x02\x03\x12\x03\n\x0e\x0f\n\x0b\n\x04\x04\x01\x02\x03\x12\x03\x0b\x02\
    \x10\n\x0c\n\x05\x04\x01\x02\x03\x05\x12\x03\x0b\x02\x07\n\x0c\n\x05\x04\
    \x01\x02\x03\x01\x12\x03\x0b\x08\x0b\n\x0c\n\x05\x04\x01\x02\x03\x03\x12\
    \x03\x0b\x0e\x0f\n\x0b\n\x04\x04\x01\x02\x04\x12\x03\x0c\x02\x15\n\x0c\n\
    \x05\x04\x01\x02\x04\x05\x12\x03\x0c\x02\x07\n\x0c\n\x05\x04\x01\x02\x04\
    \x01\x12\x03\x0c\x08\x10\n\x0c\n\x05\x04\x01\x02\x04\x03\x12\x03\x0c\x13\
    \x14\n\x0b\n\x04\x04\x01\x02\x05\x12\x03\r\x02\x12\n\x0c\n\x05\x04\x01\
    \x02\x05\x05\x12\x03\r\x02\x07\n\x0c\n\x05\x04\x01\x02\x05\x01\x12\x03\r\
    \x08\r\n\x0c\n\x05\x04\x01\x02\x05\x03\x12\x03\r\x10\x11\n\x0b\n\x04\x04\
    \x01\x02\x06\x12\x03\x0e\x02\x15\n\x0c\n\x05\x04\x01\x02\x06\x05\x12\x03\
    \x0e\x02\x06\n\x0c\n\x05\x04\x01\x02\x06\x01\x12\x03\x0e\x07\x10\n\x0c\n\
    \x05\x04\x01\x02\x06\x03\x12\x03\x0e\x13\x14b\x06proto3\
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
            let mut messages = ::std::vec::Vec::with_capacity(2);
            messages.push(VideoHeader::generated_message_descriptor_data());
            messages.push(VideoPacket::generated_message_descriptor_data());
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
