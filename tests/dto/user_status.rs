#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[prost_enum::enhance]
#[repr(i32)]
pub enum UserStatus {
    Unknown = 0,
    Normal = 1,
    Inactivate = 2,
    Disabled = 3,
    Unregistered = 4,
}
impl UserStatus {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            UserStatus::Unknown => "USER_STATUS_UNKNOWN",
            UserStatus::Normal => "USER_STATUS_NORMAL",
            UserStatus::Inactivate => "USER_STATUS_INACTIVATE",
            UserStatus::Disabled => "USER_STATUS_DISABLED",
            UserStatus::Unregistered => "USER_STATUS_UNREGISTERED",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "USER_STATUS_UNKNOWN" => Some(Self::Unknown),
            "USER_STATUS_NORMAL" => Some(Self::Normal),
            "USER_STATUS_INACTIVATE" => Some(Self::Inactivate),
            "USER_STATUS_DISABLED" => Some(Self::Disabled),
            "USER_STATUS_UNREGISTERED" => Some(Self::Unregistered),
            _ => None,
        }
    }
}
