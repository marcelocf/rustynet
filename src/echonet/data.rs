//! Contains data types that extends he echonet-lite-rs types.
//! Special attention on handling [`Prorperty`]

/// Properties common to wide variety of devices.
#[repr(u8)]
pub(crate) enum GeneralPropertyCode {
    /// Indicates the node operating status
    /// accessRule:
    ///    get: required
    ///    set: notApplicable
    ///    inf: required
    /// states:
    ///    0x30u8: Booting
    ///    0x31u8: Not booting
    OperatingStatus = 0x80u8,

    /// Indicates ECHONET Lite versionused by communication middleware and message types supported by communication middleware
    /// 1st byte: major version, 2nd byte: minor version, 3rd data: bitmap or data format, 4th byte: 0x00.
    /// accessRule:
    ///    get: required
    ///    set: notApplicable
    ///    inf: optional
    VersionInformation = 0x81u8,

    /// Number to identify the node implementing the device object in the domain
    /// 1st data is 0xFE, 2nd to 4th data is manufacture code. The rest should be unique to each device.
    /// accesRule:
    ///    get: required
    ///    set: notApplicable
    ///    inf: optional
    IndentificationNumber = 0x83u8,

    /// indicates whether a fault has occurred or not
    /// accessRule:
    ///    get: optional
    ///    set: notApplicable
    ///    inf: optional
    /// states: YES/NO
    FaultStatus = 0x88u8,

    /// Fault description
    /// accessRule:
    ///    get: optional
    ///    set: notApplicable
    ///    inf:optional
    /// data: number between 0 and 1004.
    FaultDescription = 0x89u8,

    /// 3-byte Manufacturer code
    /// accesRule:
    ///    get: required
    ///    set: notApplicable
    ///    inf: optional
    ManufacturerCode = 0x8Au8,

    /// 3-byte business facility code
    /// accessRule:
    ///    get: optional
    ///    set: notApplicable
    ///    inf: optional
    BusinessFacilityCode = 0x8Bu8,

    /// Identifies the product using ASCII code
    /// accessRule:
    ///    get: optional
    ///    set: notApplicable
    ///    inf: optional
    /// raw_12
    ProductCode = 0x8Cu8,

    /// Indicates the product number using ASCII code
    /// accesRule:
    ///    get: optional
    ///    set: notApplicable
    ///    inf: optional
    /// raw_12
    SerialNumber = 0x8Du8,

    /// 4-byte production date code
    /// accessRule:
    ///    get: optional
    ///    set: notApplicable
    ///    inf: optional
    ProductionDate = 0x8Eu8,

    /// Enumeration of EPC in case of the count is than 16, or bitmap in case of the count is more than 15.
    /// 1st byte is the count of property.
    /// accesRule:
    ///    get: required
    ///    set: notApplicable
    ///    inf: optional
    StatusChangeAnnouncementPropertyMap = 0x9Du8,

    /// Enumeration of EPC in case of the count is less than 16, or bitmap in case of the count is more than 15.
    /// 1st byte is count of property.
    /// accessRule:
    ///    get: required
    ///    set: notApplicable
    ///    inf: optional
    SetPropertyMap = 0x9Eu8,

    /// Enumeration of EPC in case of the count is less than 16, or bitmap in case of the count is more than 15.
    /// 1st byte is count of property.
    /// accessRule:
    ///    get: required
    ///    set: notApplicable
    ///    inf: optional
    GetPropertyMap = 0x9Fu8,
}

/// Property code for profile node 0x0EF0
#[repr(u8)]
pub(crate) enum ProfilePropertyCode {
    /// 2 byte data to identify each node in a domain
    /// accessRule:
    ///    get: optional
    ///    set: optional
    ///    inf: optional
    UniqueIdentifierData = 0xBFu8,

    /// Total number of instances held by self-node
    /// accessRule:
    ///    get: required
    ///    set: notApplicable
    ///    inf: optional
    /// 3-byte data for integer. excluding node profile object instance.
    SelfNodeInstances = 0xD3u8,

    /// Total number of classes held by self-node
    /// accessRule:
    ///    get: required
    ///    set: notApplicable
    ///    inf: optional
    /// Including node profile class
    SelfNodeClasses = 0xD4u8,

    /// Instance list when self-node instance configuration is changed.
    /// Number of instances + Instance list
    /// accessRule:
    ///    get: Optional
    ///    set: notApplicable
    ///    inf: required
    InstanceListNotification = 0xD5u8,

    /// Number of Instances + Instance List
    /// Instance list is an array of EOJ. (3 bytes)
    /// accessRule:
    ///    get: required
    ///    set: notApplicable
    ///    inf: optional
    SelfNodeInstanceListS = 0xd6u8,

    /// Number of classess + class list, excluding node profile class
    /// accessRule:
    ///    get: required
    ///    set: notApplicable
    ///    inf: optional
    SelfNodeClassListS = 0xD7u8,
}
