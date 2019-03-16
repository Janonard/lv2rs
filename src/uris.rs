use std::ffi::CStr;
use urid::URID;

pub const URI: &[u8] = b"http://lv2plug.in/ns/ext/atom\0";

pub const ATOM_TYPE_URI: &[u8] = b"http://lv2plug.in/ns/ext/atom#Atom\0";
pub const ATOM_PORT_TYPE_URI: &[u8] = b"http://lv2plug.in/ns/ext/atom#AtomPort\0";
pub const BLANK_TYPE_URI: &[u8] = b"http://lv2plug.in/ns/ext/atom#Blank\0";
pub const BOOL_TYPE_URI: &[u8] = b"http://lv2plug.in/ns/ext/atom#Bool\0";
pub const CHUNK_TYPE_URI: &[u8] = b"http://lv2plug.in/ns/ext/atom#Chunk\0";
pub const DOUBLE_TYPE_URI: &[u8] = b"http://lv2plug.in/ns/ext/atom#Double\0";
pub const EVENT_TYPE_URI: &[u8] = b"http://lv2plug.in/ns/ext/atom#Event\0";
pub const FLOAT_TYPE_URI: &[u8] = b"http://lv2plug.in/ns/ext/atom#Float\0";
pub const INT_TYPE_URI: &[u8] = b"http://lv2plug.in/ns/ext/atom#Int\0";
pub const LITERAL_TYPE_URI: &[u8] = b"http://lv2plug.in/ns/ext/atom#Literal\0";
pub const LONG_TYPE_URI: &[u8] = b"http://lv2plug.in/ns/ext/atom#Long\0";
pub const NUMBER_TYPE_URI: &[u8] = b"http://lv2plug.in/ns/ext/atom#Number\0";
pub const OBJECT_TYPE_URI: &[u8] = b"http://lv2plug.in/ns/ext/atom#Object\0";
pub const PATH_TYPE_URI: &[u8] = b"http://lv2plug.in/ns/ext/atom#Path\0";
pub const PROPERTY_TYPE_URI: &[u8] = b"http://lv2plug.in/ns/ext/atom#Property\0";
pub const RESOURCE_TYPE_URI: &[u8] = b"http://lv2plug.in/ns/ext/atom#Resource\0";
pub const SEQUENCE_TYPE_URI: &[u8] = b"http://lv2plug.in/ns/ext/atom#Sequence\0";
pub const SOUND_TYPE_URI: &[u8] = b"http://lv2plug.in/ns/ext/atom#Sound\0";
pub const STRING_TYPE_URI: &[u8] = b"http://lv2plug.in/ns/ext/atom#String\0";
pub const TUPLE_TYPE_URI: &[u8] = b"http://lv2plug.in/ns/ext/atom#Tuple\0";
pub const URI_TYPE_URI: &[u8] = b"http://lv2plug.in/ns/ext/atom#URI\0";
pub const URID_TYPE_URI: &[u8] = b"http://lv2plug.in/ns/ext/atom#URID\0";
pub const VECTOR_TYPE_URI: &[u8] = b"http://lv2plug.in/ns/ext/atom#Vector\0";
pub const ATOM_TRANSFER_URI: &[u8] = b"http://lv2plug.in/ns/ext/atom#atomTransfer\0";
pub const BEAT_TIME_URI: &[u8] = b"http://lv2plug.in/ns/ext/atom#beatTime\0";
pub const BUFFER_TYPE_URI: &[u8] = b"http://lv2plug.in/ns/ext/atom#bufferType\0";
pub const CHILD_TYPE_URI: &[u8] = b"http://lv2plug.in/ns/ext/atom#childType\0";
pub const EVENT_TRANSFER_URI: &[u8] = b"http://lv2plug.in/ns/ext/atom#eventTransfer\0";
pub const FRAME_TIME_URI: &[u8] = b"http://lv2plug.in/ns/ext/atom#frameTime\0";
pub const SUPPORTS_URI: &[u8] = b"http://lv2plug.in/ns/ext/atom#supports\0";
pub const TIME_UNIT_URI: &[u8] = b"http://lv2plug.in/ns/ext/atom#timeUnit\0";

#[derive(Clone)]
pub struct MappedURIDs {
    pub root: URID,
    pub atom: URID,
    pub atom_port: URID,
    pub blank: URID,
    pub bool: URID,
    pub chunk: URID,
    pub double: URID,
    pub event: URID,
    pub float: URID,
    pub int: URID,
    pub literal: URID,
    pub long: URID,
    pub number: URID,
    pub object: URID,
    pub path: URID,
    pub property: URID,
    pub resource: URID,
    pub sequence: URID,
    pub sound: URID,
    pub string: URID,
    pub tuple: URID,
    pub uri: URID,
    pub urid: URID,
    pub vector: URID,
    pub atom_transfer: URID,
    pub beat_time: URID,
    pub buffer: URID,
    pub child: URID,
    pub event_transfer: URID,
    pub frame_time: URID,
    pub supports: URID,
    pub time_unit: URID,
}

impl Default for MappedURIDs {
    fn default() -> Self {
        Self {
            root: 0,
            atom: 1,
            atom_port: 2,
            blank: 3,
            bool: 4,
            chunk: 5,
            double: 6,
            event: 7,
            float: 8,
            int: 9,
            literal: 10,
            long: 11,
            number: 12,
            object: 13,
            path: 14,
            property: 15,
            resource: 16,
            sequence: 17,
            sound: 18,
            string: 19,
            tuple: 20,
            uri: 21,
            urid: 22,
            vector: 23,
            atom_transfer: 24,
            beat_time: 25,
            buffer: 26,
            child: 27,
            event_transfer: 28,
            frame_time: 29,
            supports: 30,
            time_unit: 31,
        }
    }
}

impl From<&mut urid::Map> for MappedURIDs {
    fn from(map: &mut urid::Map) -> Self {
        Self {
            root: map.map(CStr::from_bytes_with_nul(URI).unwrap()),
            atom: map.map(CStr::from_bytes_with_nul(ATOM_TYPE_URI).unwrap()),
            atom_port: map.map(CStr::from_bytes_with_nul(ATOM_PORT_TYPE_URI).unwrap()),
            blank: map.map(CStr::from_bytes_with_nul(BLANK_TYPE_URI).unwrap()),
            bool: map.map(CStr::from_bytes_with_nul(BOOL_TYPE_URI).unwrap()),
            chunk: map.map(CStr::from_bytes_with_nul(CHUNK_TYPE_URI).unwrap()),
            double: map.map(CStr::from_bytes_with_nul(DOUBLE_TYPE_URI).unwrap()),
            event: map.map(CStr::from_bytes_with_nul(EVENT_TYPE_URI).unwrap()),
            float: map.map(CStr::from_bytes_with_nul(FLOAT_TYPE_URI).unwrap()),
            int: map.map(CStr::from_bytes_with_nul(INT_TYPE_URI).unwrap()),
            literal: map.map(CStr::from_bytes_with_nul(LITERAL_TYPE_URI).unwrap()),
            long: map.map(CStr::from_bytes_with_nul(LONG_TYPE_URI).unwrap()),
            number: map.map(CStr::from_bytes_with_nul(NUMBER_TYPE_URI).unwrap()),
            object: map.map(CStr::from_bytes_with_nul(OBJECT_TYPE_URI).unwrap()),
            path: map.map(CStr::from_bytes_with_nul(PATH_TYPE_URI).unwrap()),
            property: map.map(CStr::from_bytes_with_nul(PROPERTY_TYPE_URI).unwrap()),
            resource: map.map(CStr::from_bytes_with_nul(RESOURCE_TYPE_URI).unwrap()),
            sequence: map.map(CStr::from_bytes_with_nul(SEQUENCE_TYPE_URI).unwrap()),
            sound: map.map(CStr::from_bytes_with_nul(SOUND_TYPE_URI).unwrap()),
            string: map.map(CStr::from_bytes_with_nul(STRING_TYPE_URI).unwrap()),
            tuple: map.map(CStr::from_bytes_with_nul(TUPLE_TYPE_URI).unwrap()),
            uri: map.map(CStr::from_bytes_with_nul(URI_TYPE_URI).unwrap()),
            urid: map.map(CStr::from_bytes_with_nul(URID_TYPE_URI).unwrap()),
            vector: map.map(CStr::from_bytes_with_nul(VECTOR_TYPE_URI).unwrap()),
            atom_transfer: map.map(CStr::from_bytes_with_nul(ATOM_TRANSFER_URI).unwrap()),
            beat_time: map.map(CStr::from_bytes_with_nul(BEAT_TIME_URI).unwrap()),
            buffer: map.map(CStr::from_bytes_with_nul(BUFFER_TYPE_URI).unwrap()),
            child: map.map(CStr::from_bytes_with_nul(CHILD_TYPE_URI).unwrap()),
            event_transfer: map.map(CStr::from_bytes_with_nul(EVENT_TRANSFER_URI).unwrap()),
            frame_time: map.map(CStr::from_bytes_with_nul(FRAME_TIME_URI).unwrap()),
            supports: map.map(CStr::from_bytes_with_nul(SUPPORTS_URI).unwrap()),
            time_unit: map.map(CStr::from_bytes_with_nul(TIME_UNIT_URI).unwrap()),
        }
    }
}
