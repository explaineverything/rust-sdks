// https://doc.rust-lang.org/rust-by-example/generics/new_types.html

use std::fmt::Display;

const ROOM_PREFIX: &str = "RM_";
const PARTICIPANT_PREFIX: &str = "PA_";
const TRACK_PREFIX: &str = "TR_";

#[derive(Clone, Debug, Eq, Hash, PartialEq, PartialOrd, Ord)]
pub struct ParticipantSid(pub String);

#[derive(Clone, Default, Debug, Eq, Hash, PartialEq, PartialOrd, Ord)]
pub struct ParticipantIdentity(pub String);

#[derive(Clone, Debug, Eq, Hash, PartialEq, PartialOrd, Ord)]
pub struct TrackSid(String);

#[derive(Clone, Debug, Eq, Hash, PartialEq, PartialOrd, Ord)]
pub struct RoomSid(String);

impl From<String> for ParticipantIdentity {
    fn from(value: String) -> Self {
        Self(value)
    }
}

macro_rules! impl_string_into {
    ($from:ty) => {
        impl From<$from> for String {
            fn from(value: $from) -> Self {
                value.0
            }
        }

        impl Display for $from {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}", self.0)
            }
        }

        impl $from {
            pub fn as_str(&self) -> &str {
                &self.0
            }
        }
    };
}

impl_string_into!(ParticipantSid);
impl_string_into!(ParticipantIdentity);
impl_string_into!(TrackSid);
impl_string_into!(RoomSid);

macro_rules! impl_from_prefix {
    ($to:ty, $prefix:ident) => {
        impl TryFrom<String> for $to {
            type Error = String;

            fn try_from(value: String) -> Result<Self, Self::Error> {
                if value.starts_with($prefix) {
                    Ok(Self(value))
                } else {
                    Err(value)
                }
            }
        }
    };
}

impl_from_prefix!(RoomSid, ROOM_PREFIX);
impl_from_prefix!(ParticipantSid, PARTICIPANT_PREFIX);
impl_from_prefix!(TrackSid, TRACK_PREFIX);
