pub mod schemas {
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum AccountStatusReason {
        UnknownReason,
        UnsupportedGoogleAppsUser,
    }
    impl AccountStatusReason {
        pub fn as_str(self) -> &'static str {
            match self {
                AccountStatusReason::UnknownReason => "unknownReason",
                AccountStatusReason::UnsupportedGoogleAppsUser => "unsupportedGoogleAppsUser",
            }
        }
    }
    impl ::std::fmt::Display for AccountStatusReason {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for AccountStatusReason {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for AccountStatusReason {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "unknownReason" => AccountStatusReason::UnknownReason,
                "unsupportedGoogleAppsUser" => AccountStatusReason::UnsupportedGoogleAppsUser,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::field_selector::FieldSelector for AccountStatusReason {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct AccountStatus {
        #[doc = "If supported is false, indicates the reason why the account is not supported."]
        #[serde(rename = "reason", default)]
        pub reason: ::std::option::Option<crate::schemas::AccountStatusReason>,
        #[doc = "True if the account is supported."]
        #[serde(rename = "supported", default)]
        pub supported: ::std::option::Option<bool>,
    }
    impl ::field_selector::FieldSelector for AccountStatus {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct Application {
        #[doc = "An optional URI that can be used to link back to the application."]
        #[serde(rename = "detailsUrl", default)]
        pub details_url: ::std::option::Option<String>,
        #[doc = "The name of this application. This is required for REST clients, but we do not enforce uniqueness of this name. It is provided as a matter of convenience for other developers who would like to identify which REST created an Application or Data Source."]
        #[serde(rename = "name", default)]
        pub name: ::std::option::Option<String>,
        #[doc = "Package name for this application. This is used as a unique identifier when created by Android applications, but cannot be specified by REST clients. REST clients will have their developer project number reflected into the Data Source data stream IDs, instead of the packageName."]
        #[serde(rename = "packageName", default)]
        pub package_name: ::std::option::Option<String>,
        #[doc = "Version of the application. You should update this field whenever the application changes in a way that affects the computation of the data."]
        #[serde(rename = "version", default)]
        pub version: ::std::option::Option<String>,
    }
    impl ::field_selector::FieldSelector for Application {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct ApplyDataPointChangesResponse {
        #[doc = "list of changes that failed to apply"]
        #[serde(rename = "fail", default)]
        pub fail: ::std::option::Option<Vec<crate::schemas::DataPointChange>>,
    }
    impl ::field_selector::FieldSelector for ApplyDataPointChangesResponse {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct DataPoint {
        #[doc = "DO NOT USE THIS FIELD. It is ignored, and not stored."]
        #[serde(rename = "computationTimeMillis", default)]
        #[serde(with = "crate::parsed_string")]
        pub computation_time_millis: ::std::option::Option<i64>,
        #[doc = "The data type defining the format of the values in this data point."]
        #[serde(rename = "dataTypeName", default)]
        pub data_type_name: ::std::option::Option<String>,
        #[doc = "The end time of the interval represented by this data point, in nanoseconds since epoch."]
        #[serde(rename = "endTimeNanos", default)]
        #[serde(with = "crate::parsed_string")]
        pub end_time_nanos: ::std::option::Option<i64>,
        #[doc = "Internal use only. If present, should be a serialized FitnessCommon.HiddenMetadata proto."]
        #[serde(rename = "hiddenMetadata", default)]
        pub hidden_metadata: ::std::option::Option<crate::bytes::Bytes>,
        #[doc = "Indicates the last time this data point was modified. Useful only in contexts where we are listing the data changes, rather than representing the current state of the data."]
        #[serde(rename = "modifiedTimeMillis", default)]
        #[serde(with = "crate::parsed_string")]
        pub modified_time_millis: ::std::option::Option<i64>,
        #[doc = "If the data point is contained in a dataset for a derived data source, this field will be populated with the data source stream ID that created the data point originally.\n\nWARNING: do not rely on this field for anything other than debugging. The value of this field, if it is set at all, is an implementation detail and is not guaranteed to remain consistent."]
        #[serde(rename = "originDataSourceId", default)]
        pub origin_data_source_id: ::std::option::Option<String>,
        #[doc = "The raw timestamp from the original SensorEvent."]
        #[serde(rename = "rawTimestampNanos", default)]
        #[serde(with = "crate::parsed_string")]
        pub raw_timestamp_nanos: ::std::option::Option<i64>,
        #[doc = "The start time of the interval represented by this data point, in nanoseconds since epoch."]
        #[serde(rename = "startTimeNanos", default)]
        #[serde(with = "crate::parsed_string")]
        pub start_time_nanos: ::std::option::Option<i64>,
        #[doc = "Values of each data type field for the data point. It is expected that each value corresponding to a data type field will occur in the same order that the field is listed with in the data type specified in a data source.\n\nOnly one of integer and floating point fields will be populated, depending on the format enum value within data source's type field."]
        #[serde(rename = "value", default)]
        pub value: ::std::option::Option<Vec<crate::schemas::Value>>,
    }
    impl ::field_selector::FieldSelector for DataPoint {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum DataPointChangeChangeType {
        Deletion,
        DeletionByTime,
        Insertion,
        Unknown,
    }
    impl DataPointChangeChangeType {
        pub fn as_str(self) -> &'static str {
            match self {
                DataPointChangeChangeType::Deletion => "deletion",
                DataPointChangeChangeType::DeletionByTime => "deletionByTime",
                DataPointChangeChangeType::Insertion => "insertion",
                DataPointChangeChangeType::Unknown => "unknown",
            }
        }
    }
    impl ::std::fmt::Display for DataPointChangeChangeType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for DataPointChangeChangeType {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for DataPointChangeChangeType {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "deletion" => DataPointChangeChangeType::Deletion,
                "deletionByTime" => DataPointChangeChangeType::DeletionByTime,
                "insertion" => DataPointChangeChangeType::Insertion,
                "unknown" => DataPointChangeChangeType::Unknown,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::field_selector::FieldSelector for DataPointChangeChangeType {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct DataPointChange {
        #[doc = "ID of the data point change in the local store, for simple deletion."]
        #[serde(rename = "changeLogId", default)]
        #[serde(with = "crate::parsed_string")]
        pub change_log_id: ::std::option::Option<i64>,
        #[doc = "Timestamp of when this change occurred."]
        #[serde(rename = "changeTimestampMillis", default)]
        #[serde(with = "crate::parsed_string")]
        pub change_timestamp_millis: ::std::option::Option<i64>,
        #[serde(rename = "changeType", default)]
        pub change_type: ::std::option::Option<crate::schemas::DataPointChangeChangeType>,
        #[doc = "Not used if change_type = DELETION_BY_TIME (but must remain required for backward-compatibility)."]
        #[serde(rename = "dataPoint", default)]
        pub data_point: ::std::option::Option<crate::schemas::DataPoint>,
        #[serde(rename = "dataSourceId", default)]
        pub data_source_id: ::std::option::Option<String>,
        #[doc = "Deprecated, prefer setting change_type. This must still be read as it is set by GMSCore v20 and earlier."]
        #[serde(rename = "delete", default)]
        pub delete: ::std::option::Option<bool>,
        #[doc = "Used when change_type = DELETION_BY_TIME."]
        #[serde(rename = "deleteRange", default)]
        pub delete_range: ::std::option::Option<crate::schemas::DeletionClosedTimeRange>,
        #[serde(rename = "retryCount", default)]
        pub retry_count: ::std::option::Option<i32>,
    }
    impl ::field_selector::FieldSelector for DataPointChange {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct DataPointChangeList {
        #[serde(rename = "change", default)]
        pub change: ::std::option::Option<Vec<crate::schemas::DataPointChange>>,
    }
    impl ::field_selector::FieldSelector for DataPointChangeList {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum DataSourceDataQualityStandardItems {
        DataQualityBloodGlucoseIso151972003,
        DataQualityBloodGlucoseIso151972013,
        DataQualityBloodPressureAami,
        DataQualityBloodPressureBhsAA,
        DataQualityBloodPressureBhsAB,
        DataQualityBloodPressureBhsBA,
        DataQualityBloodPressureBhsBB,
        DataQualityBloodPressureEsh2002,
        DataQualityBloodPressureEsh2010,
        DataQualityUnknown,
    }
    impl DataSourceDataQualityStandardItems {
        pub fn as_str(self) -> &'static str {
            match self {
                DataSourceDataQualityStandardItems::DataQualityBloodGlucoseIso151972003 => {
                    "dataQualityBloodGlucoseIso151972003"
                }
                DataSourceDataQualityStandardItems::DataQualityBloodGlucoseIso151972013 => {
                    "dataQualityBloodGlucoseIso151972013"
                }
                DataSourceDataQualityStandardItems::DataQualityBloodPressureAami => {
                    "dataQualityBloodPressureAami"
                }
                DataSourceDataQualityStandardItems::DataQualityBloodPressureBhsAA => {
                    "dataQualityBloodPressureBhsAA"
                }
                DataSourceDataQualityStandardItems::DataQualityBloodPressureBhsAB => {
                    "dataQualityBloodPressureBhsAB"
                }
                DataSourceDataQualityStandardItems::DataQualityBloodPressureBhsBA => {
                    "dataQualityBloodPressureBhsBA"
                }
                DataSourceDataQualityStandardItems::DataQualityBloodPressureBhsBB => {
                    "dataQualityBloodPressureBhsBB"
                }
                DataSourceDataQualityStandardItems::DataQualityBloodPressureEsh2002 => {
                    "dataQualityBloodPressureEsh2002"
                }
                DataSourceDataQualityStandardItems::DataQualityBloodPressureEsh2010 => {
                    "dataQualityBloodPressureEsh2010"
                }
                DataSourceDataQualityStandardItems::DataQualityUnknown => "dataQualityUnknown",
            }
        }
    }
    impl ::std::fmt::Display for DataSourceDataQualityStandardItems {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for DataSourceDataQualityStandardItems {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for DataSourceDataQualityStandardItems {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "dataQualityBloodGlucoseIso151972003" => {
                    DataSourceDataQualityStandardItems::DataQualityBloodGlucoseIso151972003
                }
                "dataQualityBloodGlucoseIso151972013" => {
                    DataSourceDataQualityStandardItems::DataQualityBloodGlucoseIso151972013
                }
                "dataQualityBloodPressureAami" => {
                    DataSourceDataQualityStandardItems::DataQualityBloodPressureAami
                }
                "dataQualityBloodPressureBhsAA" => {
                    DataSourceDataQualityStandardItems::DataQualityBloodPressureBhsAA
                }
                "dataQualityBloodPressureBhsAB" => {
                    DataSourceDataQualityStandardItems::DataQualityBloodPressureBhsAB
                }
                "dataQualityBloodPressureBhsBA" => {
                    DataSourceDataQualityStandardItems::DataQualityBloodPressureBhsBA
                }
                "dataQualityBloodPressureBhsBB" => {
                    DataSourceDataQualityStandardItems::DataQualityBloodPressureBhsBB
                }
                "dataQualityBloodPressureEsh2002" => {
                    DataSourceDataQualityStandardItems::DataQualityBloodPressureEsh2002
                }
                "dataQualityBloodPressureEsh2010" => {
                    DataSourceDataQualityStandardItems::DataQualityBloodPressureEsh2010
                }
                "dataQualityUnknown" => DataSourceDataQualityStandardItems::DataQualityUnknown,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::field_selector::FieldSelector for DataSourceDataQualityStandardItems {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum DataSourceType {
        Cleaned,
        Converted,
        Derived,
        Raw,
    }
    impl DataSourceType {
        pub fn as_str(self) -> &'static str {
            match self {
                DataSourceType::Cleaned => "cleaned",
                DataSourceType::Converted => "converted",
                DataSourceType::Derived => "derived",
                DataSourceType::Raw => "raw",
            }
        }
    }
    impl ::std::fmt::Display for DataSourceType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for DataSourceType {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for DataSourceType {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "cleaned" => DataSourceType::Cleaned,
                "converted" => DataSourceType::Converted,
                "derived" => DataSourceType::Derived,
                "raw" => DataSourceType::Raw,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::field_selector::FieldSelector for DataSourceType {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct DataSource {
        #[doc = "Information about an application which feeds sensor data into the platform."]
        #[serde(rename = "application", default)]
        pub application: ::std::option::Option<crate::schemas::Application>,
        #[doc = "DO NOT POPULATE THIS FIELD. It is never populated in responses from the platform, and is ignored in queries. It will be removed in a future version entirely."]
        #[serde(rename = "dataQualityStandard", default)]
        pub data_quality_standard:
            ::std::option::Option<Vec<crate::schemas::DataSourceDataQualityStandardItems>>,
        #[doc = "A unique identifier for the data stream produced by this data source. The identifier includes:\n\n* The physical device's manufacturer, model, and serial number (UID). \n* The application's package name or name. Package name is used when the data source was created by an Android application. The developer project number is used when the data source was created by a REST client. \n* The data source's type. \n* The data source's stream name.  Note that not all attributes of the data source are used as part of the stream identifier. In particular, the version of the hardware/the application isn't used. This allows us to preserve the same stream through version updates. This also means that two DataSource objects may represent the same data stream even if they're not equal.\n\nThe exact format of the data stream ID created by an Android application is: type:dataType.name:application.packageName:device.manufacturer:device.model:device.uid:dataStreamName \n\nThe exact format of the data stream ID created by a REST client is: type:dataType.name:developer project number:device.manufacturer:device.model:device.uid:dataStreamName \n\nWhen any of the optional fields that make up the data stream ID are absent, they will be omitted from the data stream ID. The minimum viable data stream ID would be: type:dataType.name:developer project number\n\nFinally, the developer project number is obfuscated when read by any REST or Android client that did not create the data source. Only the data source creator will see the developer project number in clear and normal form."]
        #[serde(rename = "dataStreamId", default)]
        pub data_stream_id: ::std::option::Option<String>,
        #[doc = "The stream name uniquely identifies this particular data source among other data sources of the same type from the same underlying producer. Setting the stream name is optional, but should be done whenever an application exposes two streams for the same data type, or when a device has two equivalent sensors."]
        #[serde(rename = "dataStreamName", default)]
        pub data_stream_name: ::std::option::Option<String>,
        #[doc = "The data type defines the schema for a stream of data being collected by, inserted into, or queried from the Fitness API."]
        #[serde(rename = "dataType", default)]
        pub data_type: ::std::option::Option<crate::schemas::DataType>,
        #[doc = "Representation of an integrated device (such as a phone or a wearable) that can hold sensors."]
        #[serde(rename = "device", default)]
        pub device: ::std::option::Option<crate::schemas::Device>,
        #[doc = "An end-user visible name for this data source."]
        #[serde(rename = "name", default)]
        pub name: ::std::option::Option<String>,
        #[doc = "A constant describing the type of this data source. Indicates whether this data source produces raw or derived data."]
        #[serde(rename = "type", default)]
        pub r#type: ::std::option::Option<crate::schemas::DataSourceType>,
    }
    impl ::field_selector::FieldSelector for DataSource {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct DataSourceChange {
        #[doc = "Timestamp of when this change occurred."]
        #[serde(rename = "changeTimestampMillis", default)]
        #[serde(with = "crate::parsed_string")]
        pub change_timestamp_millis: ::std::option::Option<i64>,
        #[doc = "Delete a data source. The data source must contain no data points."]
        #[serde(rename = "deleteDataSourceId", default)]
        pub delete_data_source_id: ::std::option::Option<String>,
        #[doc = "Delete a data source and all data points it contains."]
        #[serde(rename = "purgeDataSourceId", default)]
        pub purge_data_source_id: ::std::option::Option<String>,
        #[doc = "Create a new data source if it does not already exist."]
        #[serde(rename = "upsertDataSource", default)]
        pub upsert_data_source: ::std::option::Option<crate::schemas::DataSource>,
    }
    impl ::field_selector::FieldSelector for DataSourceChange {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct DataSourceChangeList {
        #[serde(rename = "change", default)]
        pub change: ::std::option::Option<Vec<crate::schemas::DataSourceChange>>,
    }
    impl ::field_selector::FieldSelector for DataSourceChangeList {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct DataType {
        #[doc = "A field represents one dimension of a data type."]
        #[serde(rename = "field", default)]
        pub field: ::std::option::Option<Vec<crate::schemas::DataTypeField>>,
        #[doc = "Each data type has a unique, namespaced, name. All data types in the com.google namespace are shared as part of the platform."]
        #[serde(rename = "name", default)]
        pub name: ::std::option::Option<String>,
    }
    impl ::field_selector::FieldSelector for DataType {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum DataTypeFieldFormat {
        Blob,
        FloatList,
        FloatPoint,
        Integer,
        IntegerList,
        Map,
        String,
    }
    impl DataTypeFieldFormat {
        pub fn as_str(self) -> &'static str {
            match self {
                DataTypeFieldFormat::Blob => "blob",
                DataTypeFieldFormat::FloatList => "floatList",
                DataTypeFieldFormat::FloatPoint => "floatPoint",
                DataTypeFieldFormat::Integer => "integer",
                DataTypeFieldFormat::IntegerList => "integerList",
                DataTypeFieldFormat::Map => "map",
                DataTypeFieldFormat::String => "string",
            }
        }
    }
    impl ::std::fmt::Display for DataTypeFieldFormat {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for DataTypeFieldFormat {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for DataTypeFieldFormat {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "blob" => DataTypeFieldFormat::Blob,
                "floatList" => DataTypeFieldFormat::FloatList,
                "floatPoint" => DataTypeFieldFormat::FloatPoint,
                "integer" => DataTypeFieldFormat::Integer,
                "integerList" => DataTypeFieldFormat::IntegerList,
                "map" => DataTypeFieldFormat::Map,
                "string" => DataTypeFieldFormat::String,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::field_selector::FieldSelector for DataTypeFieldFormat {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct DataTypeField {
        #[doc = "The different supported formats for each field in a data type."]
        #[serde(rename = "format", default)]
        pub format: ::std::option::Option<crate::schemas::DataTypeFieldFormat>,
        #[doc = "Defines the name and format of data. Unlike data type names, field names are not namespaced, and only need to be unique within the data type."]
        #[serde(rename = "name", default)]
        pub name: ::std::option::Option<String>,
        #[serde(rename = "optional", default)]
        pub optional: ::std::option::Option<bool>,
    }
    impl ::field_selector::FieldSelector for DataTypeField {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct DeleteHistory {
        #[serde(rename = "endMillis", default)]
        #[serde(with = "crate::parsed_string")]
        pub end_millis: ::std::option::Option<i64>,
    }
    impl ::field_selector::FieldSelector for DeleteHistory {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct DeleteHistoryRequest {
        #[doc = "Never set. Do not use."]
        #[serde(rename = "alsoDeleteAppData", default)]
        pub also_delete_app_data: ::std::option::Option<bool>,
    }
    impl ::field_selector::FieldSelector for DeleteHistoryRequest {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct DeletionClosedTimeRange {
        #[doc = "Inclusive."]
        #[serde(rename = "endTimeNanos", default)]
        #[serde(with = "crate::parsed_string")]
        pub end_time_nanos: ::std::option::Option<i64>,
        #[doc = "Inclusive."]
        #[serde(rename = "startTimeNanos", default)]
        #[serde(with = "crate::parsed_string")]
        pub start_time_nanos: ::std::option::Option<i64>,
    }
    impl ::field_selector::FieldSelector for DeletionClosedTimeRange {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum DevicePlatformType {
        PlatformTypeAndroid,
        PlatformTypeBle,
        PlatformTypeUnknown,
    }
    impl DevicePlatformType {
        pub fn as_str(self) -> &'static str {
            match self {
                DevicePlatformType::PlatformTypeAndroid => "platformTypeAndroid",
                DevicePlatformType::PlatformTypeBle => "platformTypeBle",
                DevicePlatformType::PlatformTypeUnknown => "platformTypeUnknown",
            }
        }
    }
    impl ::std::fmt::Display for DevicePlatformType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for DevicePlatformType {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for DevicePlatformType {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "platformTypeAndroid" => DevicePlatformType::PlatformTypeAndroid,
                "platformTypeBle" => DevicePlatformType::PlatformTypeBle,
                "platformTypeUnknown" => DevicePlatformType::PlatformTypeUnknown,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::field_selector::FieldSelector for DevicePlatformType {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum DeviceType {
        ChestStrap,
        HeadMounted,
        Phone,
        Scale,
        Tablet,
        Unknown,
        Watch,
    }
    impl DeviceType {
        pub fn as_str(self) -> &'static str {
            match self {
                DeviceType::ChestStrap => "chestStrap",
                DeviceType::HeadMounted => "headMounted",
                DeviceType::Phone => "phone",
                DeviceType::Scale => "scale",
                DeviceType::Tablet => "tablet",
                DeviceType::Unknown => "unknown",
                DeviceType::Watch => "watch",
            }
        }
    }
    impl ::std::fmt::Display for DeviceType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for DeviceType {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for DeviceType {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "chestStrap" => DeviceType::ChestStrap,
                "headMounted" => DeviceType::HeadMounted,
                "phone" => DeviceType::Phone,
                "scale" => DeviceType::Scale,
                "tablet" => DeviceType::Tablet,
                "unknown" => DeviceType::Unknown,
                "watch" => DeviceType::Watch,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::field_selector::FieldSelector for DeviceType {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct Device {
        #[doc = "Manufacturer of the product/hardware."]
        #[serde(rename = "manufacturer", default)]
        pub manufacturer: ::std::option::Option<String>,
        #[doc = "End-user visible model name for the device."]
        #[serde(rename = "model", default)]
        pub model: ::std::option::Option<String>,
        #[doc = ""]
        #[serde(rename = "platformType", default)]
        pub platform_type: ::std::option::Option<crate::schemas::DevicePlatformType>,
        #[doc = "A constant representing the type of the device."]
        #[serde(rename = "type", default)]
        pub r#type: ::std::option::Option<crate::schemas::DeviceType>,
        #[doc = "The serial number or other unique ID for the hardware. This field is obfuscated when read by any REST or Android client that did not create the data source. Only the data source creator will see the uid field in clear and normal form."]
        #[serde(rename = "uid", default)]
        pub uid: ::std::option::Option<String>,
        #[doc = "Version string for the device hardware/software."]
        #[serde(rename = "version", default)]
        pub version: ::std::option::Option<String>,
    }
    impl ::field_selector::FieldSelector for Device {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct ListDataPointChangesResponse {
        #[doc = "The data source for the data protos. Used when sending responses back to Wear."]
        #[serde(rename = "dataSource", default)]
        pub data_source: ::std::option::Option<crate::schemas::DataSource>,
        #[doc = "Delete these time ranges on the client"]
        #[serde(rename = "deleteRange", default)]
        pub delete_range: ::std::option::Option<Vec<crate::schemas::TimeRange>>,
        #[doc = "Data points that have been removed and will not be included in any other request for dataset contents."]
        #[serde(rename = "deletedDataPoint", default)]
        pub deleted_data_point: ::std::option::Option<Vec<crate::schemas::DataPoint>>,
        #[doc = "Data points listed."]
        #[serde(rename = "insertedDataPoint", default)]
        pub inserted_data_point: ::std::option::Option<Vec<crate::schemas::DataPoint>>,
        #[doc = "The continuation token, which is used to page through large result sets. Provide this value in a subsequent request to return the next page of results. Note that this is also used as a \"sync token\" to retrieve changes after the last sync occurred. Syncing terminates when an empty response (no insertions/deletions) is returned."]
        #[serde(rename = "nextPageToken", default)]
        pub next_page_token: ::std::option::Option<String>,
        #[doc = "Whether the sync is done, i.e. there is no more data to request."]
        #[serde(rename = "noMoreChanges", default)]
        pub no_more_changes: ::std::option::Option<bool>,
    }
    impl ::field_selector::FieldSelector for ListDataPointChangesResponse {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct ListDataPointsResponse {
        #[doc = "Data points listed in reverse end_time order."]
        #[serde(rename = "dataPoints", default)]
        pub data_points: ::std::option::Option<Vec<crate::schemas::DataPoint>>,
        #[doc = "Data points that have been removed and will not be included in any other request for dataset contents. This field is deprecated and never populated in outgoing responses."]
        #[serde(rename = "deletedDataPoints", default)]
        pub deleted_data_points: ::std::option::Option<Vec<crate::schemas::DataPoint>>,
        #[doc = "The continuation token, which is used to page through large result sets. Provide this value in a subsequent request to return the next page of results."]
        #[serde(rename = "nextPageToken", default)]
        pub next_page_token: ::std::option::Option<String>,
    }
    impl ::field_selector::FieldSelector for ListDataPointsResponse {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct MapValue {
        #[doc = "Floating point value."]
        #[serde(rename = "fpVal", default)]
        pub fp_val: ::std::option::Option<f64>,
    }
    impl ::field_selector::FieldSelector for MapValue {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct PurgeDataSourcesRequest {
        #[doc = "List of data source IDs which are to be deleted. For GMS < v24, the dataSourceId will be interpreted as a purge request for the contained *type*; that is, a purge for `raw:com.google.height:com.google.android.apps.fitness:user_input` will purge all sources of type `com.google.height`."]
        #[serde(rename = "dataSourceId", default)]
        pub data_source_id: ::std::option::Option<Vec<String>>,
        #[doc = "List of data type names for which we should purge all of the sources."]
        #[serde(rename = "dataTypeName", default)]
        pub data_type_name: ::std::option::Option<Vec<String>>,
    }
    impl ::field_selector::FieldSelector for PurgeDataSourcesRequest {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct Session {
        #[doc = "The type of activity this session represents."]
        #[serde(rename = "activityType", default)]
        pub activity_type: ::std::option::Option<i32>,
        #[doc = "The application that created the session."]
        #[serde(rename = "application", default)]
        pub application: ::std::option::Option<crate::schemas::Application>,
        #[doc = "A description for this session."]
        #[serde(rename = "description", default)]
        pub description: ::std::option::Option<String>,
        #[doc = "An end time, in milliseconds since epoch, inclusive."]
        #[serde(rename = "endTimeMillis", default)]
        #[serde(with = "crate::parsed_string")]
        pub end_time_millis: ::std::option::Option<i64>,
        #[doc = "A client-generated identifier that is unique across all sessions owned by this particular user."]
        #[serde(rename = "id", default)]
        pub id: ::std::option::Option<String>,
        #[doc = "A timestamp that indicates when the session was last modified."]
        #[serde(rename = "modifiedTimeMillis", default)]
        #[serde(with = "crate::parsed_string")]
        pub modified_time_millis: ::std::option::Option<i64>,
        #[doc = "A human readable name of the session."]
        #[serde(rename = "name", default)]
        pub name: ::std::option::Option<String>,
        #[doc = "A start time, in milliseconds since epoch, inclusive."]
        #[serde(rename = "startTimeMillis", default)]
        #[serde(with = "crate::parsed_string")]
        pub start_time_millis: ::std::option::Option<i64>,
    }
    impl ::field_selector::FieldSelector for Session {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct SessionChange {
        #[doc = "ID of the session change log in the local store, for simple deletion."]
        #[serde(rename = "changeLogId", default)]
        #[serde(with = "crate::parsed_string")]
        pub change_log_id: ::std::option::Option<i64>,
        #[doc = "Timestamp of when this change occured."]
        #[serde(rename = "changeTimestampMillis", default)]
        #[serde(with = "crate::parsed_string")]
        pub change_timestamp_millis: ::std::option::Option<i64>,
        #[doc = "The full session we're deleting."]
        #[serde(rename = "deleteSession", default)]
        pub delete_session: ::std::option::Option<crate::schemas::Session>,
        #[serde(rename = "deleteSessionId", default)]
        pub delete_session_id: ::std::option::Option<String>,
        #[doc = "Package of the session we're deleting."]
        #[serde(rename = "packageName", default)]
        pub package_name: ::std::option::Option<String>,
        #[serde(rename = "upsertSession", default)]
        pub upsert_session: ::std::option::Option<crate::schemas::Session>,
    }
    impl ::field_selector::FieldSelector for SessionChange {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct SessionChangeList {
        #[serde(rename = "change", default)]
        pub change: ::std::option::Option<Vec<crate::schemas::SessionChange>>,
    }
    impl ::field_selector::FieldSelector for SessionChangeList {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct SyncDataSourcesResponse {
        #[doc = "TODO(ricebin): move this to BulkSync rpc"]
        #[serde(rename = "deleteHistory", default)]
        pub delete_history: ::std::option::Option<Vec<crate::schemas::DeleteHistory>>,
        #[doc = "list of DataSource that failed to upsert"]
        #[serde(rename = "fail", default)]
        pub fail: ::std::option::Option<Vec<crate::schemas::DataSourceChange>>,
        #[doc = "list of new DataSource that the client should insert TODO(ricebin): rename to change"]
        #[serde(rename = "insertion", default)]
        pub insertion: ::std::option::Option<Vec<crate::schemas::DataSourceChange>>,
        #[doc = "The continuation token which should be sent with the next SyncDataSourcesRequest. Historic detail: integers were concatenated to the end of this token (e.g. \"1234567890,1,2\" to denote resyncs which should occur once. If this is reimplemented, the next one should be 6."]
        #[serde(rename = "nextPageToken", default)]
        pub next_page_token: ::std::option::Option<String>,
    }
    impl ::field_selector::FieldSelector for SyncDataSourcesResponse {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct SyncSessionsResponse {
        #[doc = "list of Session that failed to upsert"]
        #[serde(rename = "fail", default)]
        pub fail: ::std::option::Option<Vec<crate::schemas::SessionChange>>,
    }
    impl ::field_selector::FieldSelector for SyncSessionsResponse {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct TimeCut {
        #[serde(rename = "inclusive", default)]
        pub inclusive: ::std::option::Option<bool>,
        #[serde(rename = "millis", default)]
        #[serde(with = "crate::parsed_string")]
        pub millis: ::std::option::Option<i64>,
    }
    impl ::field_selector::FieldSelector for TimeCut {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct TimeRange {
        #[serde(rename = "endTime", default)]
        pub end_time: ::std::option::Option<crate::schemas::TimeCut>,
        #[serde(rename = "startTime", default)]
        pub start_time: ::std::option::Option<crate::schemas::TimeCut>,
    }
    impl ::field_selector::FieldSelector for TimeRange {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct Value {
        #[serde(rename = "blob", default)]
        pub blob: ::std::option::Option<crate::bytes::Bytes>,
        #[doc = "Float list value. When this is set, other values must not be set."]
        #[serde(rename = "floatListVal", default)]
        pub float_list_val: ::std::option::Option<Vec<f32>>,
        #[doc = "Floating point value. When this is set, other values must not be set."]
        #[serde(rename = "fpVal", default)]
        pub fp_val: ::std::option::Option<f64>,
        #[doc = "Integer list value. When this is set, other values must not be set."]
        #[serde(rename = "intListVal", default)]
        pub int_list_val: ::std::option::Option<Vec<i32>>,
        #[doc = "Integer value. When this is set, other values must not be set."]
        #[serde(rename = "intVal", default)]
        pub int_val: ::std::option::Option<i32>,
        #[doc = "Set to true when the value is unset. Needed since, without reftypes, zero and unset would look the same."]
        #[serde(rename = "isUnset", default)]
        pub is_unset: ::std::option::Option<bool>,
        #[doc = "Map value. The valid key space and units for the corresponding value of each entry should be documented as part of the data type definition. Keys should be kept small whenever possible. Data streams with large keys and high data frequency may be down sampled."]
        #[serde(rename = "mapVal", default)]
        pub map_val: ::std::option::Option<Vec<crate::schemas::ValueMapValEntry>>,
        #[doc = "String value. When this is set, other values must not be set. Strings should be kept small whenever possible. Data streams with large string values and high data frequency may be down sampled."]
        #[serde(rename = "stringVal", default)]
        pub string_val: ::std::option::Option<String>,
    }
    impl ::field_selector::FieldSelector for Value {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct ValueMapValEntry {
        #[serde(rename = "key", default)]
        pub key: ::std::option::Option<String>,
        #[serde(rename = "value", default)]
        pub value: ::std::option::Option<crate::schemas::MapValue>,
    }
    impl ::field_selector::FieldSelector for ValueMapValEntry {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
}
pub mod params {
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum Alt {
        #[doc = "Responses with Content-Type of application/json"]
        Json,
    }
    impl Alt {
        pub fn as_str(self) -> &'static str {
            match self {
                Alt::Json => "json",
            }
        }
    }
    impl ::std::fmt::Display for Alt {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for Alt {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for Alt {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "json" => Alt::Json,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::field_selector::FieldSelector for Alt {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
}
pub struct Client<A> {
    reqwest: ::reqwest::Client,
    auth: ::std::sync::Mutex<A>,
}
impl<A: yup_oauth2::GetToken> Client<A> {
    pub fn new(auth: A) -> Self {
        Client {
            reqwest: ::reqwest::Client::builder().timeout(None).build().unwrap(),
            auth: ::std::sync::Mutex::new(auth),
        }
    }
    #[doc = "Actions that can be performed on the users resource"]
    pub fn users(&self) -> crate::resources::users::UsersActions<A> {
        crate::resources::users::UsersActions {
            reqwest: &self.reqwest,
            auth: &self.auth,
        }
    }
}
pub mod resources {
    pub mod users {
        pub mod params {}
        pub struct UsersActions<'a, A> {
            pub(crate) reqwest: &'a reqwest::Client,
            pub(crate) auth: &'a std::sync::Mutex<A>,
        }
        impl<'a, A: yup_oauth2::GetToken> UsersActions<'a, A> {
            #[doc = ""]
            pub fn apply_data_point_changes(
                &self,
                request: crate::schemas::DataPointChangeList,
                user_id: impl Into<String>,
            ) -> ApplyDataPointChangesRequestBuilder<A> {
                ApplyDataPointChangesRequestBuilder {
                    reqwest: &self.reqwest,
                    auth: &self.auth,
                    request,
                    alt: None,
                    fields: None,
                    key: None,
                    oauth_token: None,
                    pretty_print: None,
                    quota_user: None,
                    user_ip: None,
                    user_id: user_id.into(),
                    current_time_millis: None,
                    header_server_token: None,
                    header_user_agent: None,
                }
            }
            #[doc = ""]
            pub fn delete_history(
                &self,
                request: crate::schemas::DeleteHistoryRequest,
                user_id: impl Into<String>,
            ) -> DeleteHistoryRequestBuilder<A> {
                DeleteHistoryRequestBuilder {
                    reqwest: &self.reqwest,
                    auth: &self.auth,
                    request,
                    alt: None,
                    fields: None,
                    key: None,
                    oauth_token: None,
                    pretty_print: None,
                    quota_user: None,
                    user_ip: None,
                    user_id: user_id.into(),
                }
            }
            #[doc = "Get the status of an account, can be used to determine whether the account is supported."]
            pub fn get_account_status(
                &self,
                user_id: impl Into<String>,
            ) -> GetAccountStatusRequestBuilder<A> {
                GetAccountStatusRequestBuilder {
                    reqwest: &self.reqwest,
                    auth: &self.auth,
                    alt: None,
                    fields: None,
                    key: None,
                    oauth_token: None,
                    pretty_print: None,
                    quota_user: None,
                    user_ip: None,
                    user_id: user_id.into(),
                    header_server_token: None,
                    header_user_agent: None,
                }
            }
            #[doc = "Delete all data associated with specified data sources."]
            pub fn purge_data_sources(
                &self,
                request: crate::schemas::PurgeDataSourcesRequest,
                user_id: impl Into<String>,
            ) -> PurgeDataSourcesRequestBuilder<A> {
                PurgeDataSourcesRequestBuilder {
                    reqwest: &self.reqwest,
                    auth: &self.auth,
                    request,
                    alt: None,
                    fields: None,
                    key: None,
                    oauth_token: None,
                    pretty_print: None,
                    quota_user: None,
                    user_ip: None,
                    user_id: user_id.into(),
                }
            }
            #[doc = ""]
            pub fn sync_data_sources(
                &self,
                request: crate::schemas::DataSourceChangeList,
                user_id: impl Into<String>,
            ) -> SyncDataSourcesRequestBuilder<A> {
                SyncDataSourcesRequestBuilder {
                    reqwest: &self.reqwest,
                    auth: &self.auth,
                    request,
                    alt: None,
                    fields: None,
                    key: None,
                    oauth_token: None,
                    pretty_print: None,
                    quota_user: None,
                    user_ip: None,
                    user_id: user_id.into(),
                    current_time_millis: None,
                    header_server_token: None,
                    header_user_agent: None,
                    last_sync_time_millis: None,
                    page_token: None,
                }
            }
            #[doc = ""]
            pub fn sync_sessions(
                &self,
                request: crate::schemas::SessionChangeList,
                user_id: impl Into<String>,
            ) -> SyncSessionsRequestBuilder<A> {
                SyncSessionsRequestBuilder {
                    reqwest: &self.reqwest,
                    auth: &self.auth,
                    request,
                    alt: None,
                    fields: None,
                    key: None,
                    oauth_token: None,
                    pretty_print: None,
                    quota_user: None,
                    user_ip: None,
                    user_id: user_id.into(),
                    current_time_millis: None,
                    header_server_token: None,
                    header_user_agent: None,
                }
            }
            #[doc = "Actions that can be performed on the data_sources resource"]
            pub fn data_sources(
                &self,
            ) -> crate::resources::users::data_sources::DataSourcesActions<A> {
                crate::resources::users::data_sources::DataSourcesActions {
                    reqwest: &self.reqwest,
                    auth: &self.auth,
                }
            }
        }
        #[derive(Debug, Clone)]
        pub struct ApplyDataPointChangesRequestBuilder<'a, A> {
            pub(crate) reqwest: &'a ::reqwest::Client,
            pub(crate) auth: &'a ::std::sync::Mutex<A>,
            request: crate::schemas::DataPointChangeList,
            user_id: String,
            current_time_millis: Option<i64>,
            header_server_token: Option<String>,
            header_user_agent: Option<String>,
            alt: Option<crate::params::Alt>,
            fields: Option<String>,
            key: Option<String>,
            oauth_token: Option<String>,
            pretty_print: Option<bool>,
            quota_user: Option<String>,
            user_ip: Option<String>,
        }
        impl<'a, A: yup_oauth2::GetToken> ApplyDataPointChangesRequestBuilder<'a, A> {
            #[doc = ""]
            pub fn current_time_millis(mut self, value: i64) -> Self {
                self.current_time_millis = Some(value);
                self
            }
            #[doc = ""]
            pub fn header_server_token(mut self, value: impl Into<String>) -> Self {
                self.header_server_token = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn header_user_agent(mut self, value: impl Into<String>) -> Self {
                self.header_user_agent = Some(value.into());
                self
            }
            #[doc = "API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token."]
            pub fn key(mut self, value: impl Into<String>) -> Self {
                self.key = Some(value.into());
                self
            }
            #[doc = "OAuth 2.0 token for the current user."]
            pub fn oauth_token(mut self, value: impl Into<String>) -> Self {
                self.oauth_token = Some(value.into());
                self
            }
            #[doc = "Returns response with indentations and line breaks."]
            pub fn pretty_print(mut self, value: bool) -> Self {
                self.pretty_print = Some(value);
                self
            }
            #[doc = "An opaque string that represents a user for quota purposes. Must not exceed 40 characters."]
            pub fn quota_user(mut self, value: impl Into<String>) -> Self {
                self.quota_user = Some(value.into());
                self
            }
            #[doc = "Deprecated. Please use quotaUser instead."]
            pub fn user_ip(mut self, value: impl Into<String>) -> Self {
                self.user_ip = Some(value.into());
                self
            }
            #[doc = r" Execute the given operation. The fields requested are"]
            #[doc = r" determined by the FieldSelector attribute of the return type."]
            #[doc = r" This allows for flexible and ergonomic partial responses. See"]
            #[doc = r" `execute_standard` and `execute_debug` for interfaces that"]
            #[doc = r" are not generic over the return type and deserialize the"]
            #[doc = r" response into an auto-generated struct will all possible"]
            #[doc = r" fields."]
            pub fn execute<T>(self) -> Result<T, Box<dyn ::std::error::Error>>
            where
                T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
            {
                let fields = T::field_selector();
                let fields: Option<String> = if fields.is_empty() {
                    None
                } else {
                    Some(fields)
                };
                self.execute_fields(fields)
            }
            #[doc = r" Execute the given operation. This will not provide any"]
            #[doc = r" `fields` selector indicating that the server will determine"]
            #[doc = r" the fields returned. This typically includes the most common"]
            #[doc = r" fields, but it will not include every possible attribute of"]
            #[doc = r" the response resource."]
            pub fn execute_standard(
                self,
            ) -> Result<crate::schemas::ApplyDataPointChangesResponse, Box<dyn ::std::error::Error>>
            {
                self.execute_fields::<_, &str>(None)
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub fn execute_debug(
                self,
            ) -> Result<crate::schemas::ApplyDataPointChangesResponse, Box<dyn ::std::error::Error>>
            {
                self.execute_fields(Some("*"))
            }
            #[doc = r" Execute the given operation. This will use the `fields`"]
            #[doc = r" selector provided and will deserialize the response into"]
            #[doc = r" whatever return value is provided."]
            pub fn execute_fields<T, F>(
                mut self,
                fields: Option<F>,
            ) -> Result<T, Box<dyn ::std::error::Error>>
            where
                T: ::serde::de::DeserializeOwned,
                F: Into<String>,
            {
                self.fields = fields.map(Into::into);
                self._execute()
            }
            fn _execute<T>(&mut self) -> Result<T, Box<dyn ::std::error::Error>>
            where
                T: ::serde::de::DeserializeOwned,
            {
                let req = self._request(&self._path());
                let req = req.json(&self.request);
                Ok(req.send()?.error_for_status()?.json()?)
            }
            fn _path(&self) -> String {
                let mut output =
                    "https://www.googleapis.com/fitness/v0dogfoodfirstparty/users/".to_owned();
                {
                    let var_as_str = &self.user_id;
                    output.extend(::percent_encoding::utf8_percent_encode(
                        &var_as_str,
                        crate::SIMPLE,
                    ));
                }
                output.push_str("/applyDataPointChanges");
                output
            }
            fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
                let req = self.reqwest.request(::reqwest::Method::POST, path);
                let req = req.query(&[("currentTimeMillis", &self.current_time_millis)]);
                let req = req.query(&[("header.serverToken", &self.header_server_token)]);
                let req = req.query(&[("header.userAgent", &self.header_user_agent)]);
                let req = req.query(&[("alt", &self.alt)]);
                let req = req.query(&[("fields", &self.fields)]);
                let req = req.query(&[("key", &self.key)]);
                let req = req.query(&[("oauth_token", &self.oauth_token)]);
                let req = req.query(&[("prettyPrint", &self.pretty_print)]);
                let req = req.query(&[("quotaUser", &self.quota_user)]);
                let req = req.query(&[("userIp", &self.user_ip)]);
                req
            }
        }
        #[derive(Debug, Clone)]
        pub struct DeleteHistoryRequestBuilder<'a, A> {
            pub(crate) reqwest: &'a ::reqwest::Client,
            pub(crate) auth: &'a ::std::sync::Mutex<A>,
            request: crate::schemas::DeleteHistoryRequest,
            user_id: String,
            alt: Option<crate::params::Alt>,
            fields: Option<String>,
            key: Option<String>,
            oauth_token: Option<String>,
            pretty_print: Option<bool>,
            quota_user: Option<String>,
            user_ip: Option<String>,
        }
        impl<'a, A: yup_oauth2::GetToken> DeleteHistoryRequestBuilder<'a, A> {
            #[doc = "API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token."]
            pub fn key(mut self, value: impl Into<String>) -> Self {
                self.key = Some(value.into());
                self
            }
            #[doc = "OAuth 2.0 token for the current user."]
            pub fn oauth_token(mut self, value: impl Into<String>) -> Self {
                self.oauth_token = Some(value.into());
                self
            }
            #[doc = "Returns response with indentations and line breaks."]
            pub fn pretty_print(mut self, value: bool) -> Self {
                self.pretty_print = Some(value);
                self
            }
            #[doc = "An opaque string that represents a user for quota purposes. Must not exceed 40 characters."]
            pub fn quota_user(mut self, value: impl Into<String>) -> Self {
                self.quota_user = Some(value.into());
                self
            }
            #[doc = "Deprecated. Please use quotaUser instead."]
            pub fn user_ip(mut self, value: impl Into<String>) -> Self {
                self.user_ip = Some(value.into());
                self
            }
            pub fn execute(self) -> Result<(), Box<dyn ::std::error::Error>> {
                let req = self._request(&self._path());
                let req = req.json(&self.request);
                req.send()?.error_for_status()?;
                Ok(())
            }
            fn _path(&self) -> String {
                let mut output =
                    "https://www.googleapis.com/fitness/v0dogfoodfirstparty/users/".to_owned();
                {
                    let var_as_str = &self.user_id;
                    output.extend(::percent_encoding::utf8_percent_encode(
                        &var_as_str,
                        crate::SIMPLE,
                    ));
                }
                output.push_str("/deleteHistory");
                output
            }
            fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
                let req = self.reqwest.request(::reqwest::Method::POST, path);
                let req = req.query(&[("alt", &self.alt)]);
                let req = req.query(&[("fields", &self.fields)]);
                let req = req.query(&[("key", &self.key)]);
                let req = req.query(&[("oauth_token", &self.oauth_token)]);
                let req = req.query(&[("prettyPrint", &self.pretty_print)]);
                let req = req.query(&[("quotaUser", &self.quota_user)]);
                let req = req.query(&[("userIp", &self.user_ip)]);
                req
            }
        }
        #[derive(Debug, Clone)]
        pub struct GetAccountStatusRequestBuilder<'a, A> {
            pub(crate) reqwest: &'a ::reqwest::Client,
            pub(crate) auth: &'a ::std::sync::Mutex<A>,
            user_id: String,
            header_server_token: Option<String>,
            header_user_agent: Option<String>,
            alt: Option<crate::params::Alt>,
            fields: Option<String>,
            key: Option<String>,
            oauth_token: Option<String>,
            pretty_print: Option<bool>,
            quota_user: Option<String>,
            user_ip: Option<String>,
        }
        impl<'a, A: yup_oauth2::GetToken> GetAccountStatusRequestBuilder<'a, A> {
            #[doc = ""]
            pub fn header_server_token(mut self, value: impl Into<String>) -> Self {
                self.header_server_token = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn header_user_agent(mut self, value: impl Into<String>) -> Self {
                self.header_user_agent = Some(value.into());
                self
            }
            #[doc = "API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token."]
            pub fn key(mut self, value: impl Into<String>) -> Self {
                self.key = Some(value.into());
                self
            }
            #[doc = "OAuth 2.0 token for the current user."]
            pub fn oauth_token(mut self, value: impl Into<String>) -> Self {
                self.oauth_token = Some(value.into());
                self
            }
            #[doc = "Returns response with indentations and line breaks."]
            pub fn pretty_print(mut self, value: bool) -> Self {
                self.pretty_print = Some(value);
                self
            }
            #[doc = "An opaque string that represents a user for quota purposes. Must not exceed 40 characters."]
            pub fn quota_user(mut self, value: impl Into<String>) -> Self {
                self.quota_user = Some(value.into());
                self
            }
            #[doc = "Deprecated. Please use quotaUser instead."]
            pub fn user_ip(mut self, value: impl Into<String>) -> Self {
                self.user_ip = Some(value.into());
                self
            }
            #[doc = r" Execute the given operation. The fields requested are"]
            #[doc = r" determined by the FieldSelector attribute of the return type."]
            #[doc = r" This allows for flexible and ergonomic partial responses. See"]
            #[doc = r" `execute_standard` and `execute_debug` for interfaces that"]
            #[doc = r" are not generic over the return type and deserialize the"]
            #[doc = r" response into an auto-generated struct will all possible"]
            #[doc = r" fields."]
            pub fn execute<T>(self) -> Result<T, Box<dyn ::std::error::Error>>
            where
                T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
            {
                let fields = T::field_selector();
                let fields: Option<String> = if fields.is_empty() {
                    None
                } else {
                    Some(fields)
                };
                self.execute_fields(fields)
            }
            #[doc = r" Execute the given operation. This will not provide any"]
            #[doc = r" `fields` selector indicating that the server will determine"]
            #[doc = r" the fields returned. This typically includes the most common"]
            #[doc = r" fields, but it will not include every possible attribute of"]
            #[doc = r" the response resource."]
            pub fn execute_standard(
                self,
            ) -> Result<crate::schemas::AccountStatus, Box<dyn ::std::error::Error>> {
                self.execute_fields::<_, &str>(None)
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub fn execute_debug(
                self,
            ) -> Result<crate::schemas::AccountStatus, Box<dyn ::std::error::Error>> {
                self.execute_fields(Some("*"))
            }
            #[doc = r" Execute the given operation. This will use the `fields`"]
            #[doc = r" selector provided and will deserialize the response into"]
            #[doc = r" whatever return value is provided."]
            pub fn execute_fields<T, F>(
                mut self,
                fields: Option<F>,
            ) -> Result<T, Box<dyn ::std::error::Error>>
            where
                T: ::serde::de::DeserializeOwned,
                F: Into<String>,
            {
                self.fields = fields.map(Into::into);
                self._execute()
            }
            fn _execute<T>(&mut self) -> Result<T, Box<dyn ::std::error::Error>>
            where
                T: ::serde::de::DeserializeOwned,
            {
                let req = self._request(&self._path());
                Ok(req.send()?.error_for_status()?.json()?)
            }
            fn _path(&self) -> String {
                let mut output =
                    "https://www.googleapis.com/fitness/v0dogfoodfirstparty/users/".to_owned();
                {
                    let var_as_str = &self.user_id;
                    output.extend(::percent_encoding::utf8_percent_encode(
                        &var_as_str,
                        crate::SIMPLE,
                    ));
                }
                output.push_str("/accountStatus");
                output
            }
            fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
                let req = self.reqwest.request(::reqwest::Method::GET, path);
                let req = req.query(&[("header.serverToken", &self.header_server_token)]);
                let req = req.query(&[("header.userAgent", &self.header_user_agent)]);
                let req = req.query(&[("alt", &self.alt)]);
                let req = req.query(&[("fields", &self.fields)]);
                let req = req.query(&[("key", &self.key)]);
                let req = req.query(&[("oauth_token", &self.oauth_token)]);
                let req = req.query(&[("prettyPrint", &self.pretty_print)]);
                let req = req.query(&[("quotaUser", &self.quota_user)]);
                let req = req.query(&[("userIp", &self.user_ip)]);
                req
            }
        }
        #[derive(Debug, Clone)]
        pub struct PurgeDataSourcesRequestBuilder<'a, A> {
            pub(crate) reqwest: &'a ::reqwest::Client,
            pub(crate) auth: &'a ::std::sync::Mutex<A>,
            request: crate::schemas::PurgeDataSourcesRequest,
            user_id: String,
            alt: Option<crate::params::Alt>,
            fields: Option<String>,
            key: Option<String>,
            oauth_token: Option<String>,
            pretty_print: Option<bool>,
            quota_user: Option<String>,
            user_ip: Option<String>,
        }
        impl<'a, A: yup_oauth2::GetToken> PurgeDataSourcesRequestBuilder<'a, A> {
            #[doc = "API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token."]
            pub fn key(mut self, value: impl Into<String>) -> Self {
                self.key = Some(value.into());
                self
            }
            #[doc = "OAuth 2.0 token for the current user."]
            pub fn oauth_token(mut self, value: impl Into<String>) -> Self {
                self.oauth_token = Some(value.into());
                self
            }
            #[doc = "Returns response with indentations and line breaks."]
            pub fn pretty_print(mut self, value: bool) -> Self {
                self.pretty_print = Some(value);
                self
            }
            #[doc = "An opaque string that represents a user for quota purposes. Must not exceed 40 characters."]
            pub fn quota_user(mut self, value: impl Into<String>) -> Self {
                self.quota_user = Some(value.into());
                self
            }
            #[doc = "Deprecated. Please use quotaUser instead."]
            pub fn user_ip(mut self, value: impl Into<String>) -> Self {
                self.user_ip = Some(value.into());
                self
            }
            pub fn execute(self) -> Result<(), Box<dyn ::std::error::Error>> {
                let req = self._request(&self._path());
                let req = req.json(&self.request);
                req.send()?.error_for_status()?;
                Ok(())
            }
            fn _path(&self) -> String {
                let mut output =
                    "https://www.googleapis.com/fitness/v0dogfoodfirstparty/users/".to_owned();
                {
                    let var_as_str = &self.user_id;
                    output.extend(::percent_encoding::utf8_percent_encode(
                        &var_as_str,
                        crate::SIMPLE,
                    ));
                }
                output.push_str("/purgeDataSources");
                output
            }
            fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
                let req = self.reqwest.request(::reqwest::Method::POST, path);
                let req = req.query(&[("alt", &self.alt)]);
                let req = req.query(&[("fields", &self.fields)]);
                let req = req.query(&[("key", &self.key)]);
                let req = req.query(&[("oauth_token", &self.oauth_token)]);
                let req = req.query(&[("prettyPrint", &self.pretty_print)]);
                let req = req.query(&[("quotaUser", &self.quota_user)]);
                let req = req.query(&[("userIp", &self.user_ip)]);
                req
            }
        }
        #[derive(Debug, Clone)]
        pub struct SyncDataSourcesRequestBuilder<'a, A> {
            pub(crate) reqwest: &'a ::reqwest::Client,
            pub(crate) auth: &'a ::std::sync::Mutex<A>,
            request: crate::schemas::DataSourceChangeList,
            user_id: String,
            current_time_millis: Option<i64>,
            header_server_token: Option<String>,
            header_user_agent: Option<String>,
            last_sync_time_millis: Option<i64>,
            page_token: Option<String>,
            alt: Option<crate::params::Alt>,
            fields: Option<String>,
            key: Option<String>,
            oauth_token: Option<String>,
            pretty_print: Option<bool>,
            quota_user: Option<String>,
            user_ip: Option<String>,
        }
        impl<'a, A: yup_oauth2::GetToken> SyncDataSourcesRequestBuilder<'a, A> {
            #[doc = ""]
            pub fn current_time_millis(mut self, value: i64) -> Self {
                self.current_time_millis = Some(value);
                self
            }
            #[doc = ""]
            pub fn header_server_token(mut self, value: impl Into<String>) -> Self {
                self.header_server_token = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn header_user_agent(mut self, value: impl Into<String>) -> Self {
                self.header_user_agent = Some(value.into());
                self
            }
            #[doc = "TODO(ricebin): use page_token instead of last_sync_time_millis"]
            pub fn last_sync_time_millis(mut self, value: i64) -> Self {
                self.last_sync_time_millis = Some(value);
                self
            }
            #[doc = ""]
            pub fn page_token(mut self, value: impl Into<String>) -> Self {
                self.page_token = Some(value.into());
                self
            }
            #[doc = "API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token."]
            pub fn key(mut self, value: impl Into<String>) -> Self {
                self.key = Some(value.into());
                self
            }
            #[doc = "OAuth 2.0 token for the current user."]
            pub fn oauth_token(mut self, value: impl Into<String>) -> Self {
                self.oauth_token = Some(value.into());
                self
            }
            #[doc = "Returns response with indentations and line breaks."]
            pub fn pretty_print(mut self, value: bool) -> Self {
                self.pretty_print = Some(value);
                self
            }
            #[doc = "An opaque string that represents a user for quota purposes. Must not exceed 40 characters."]
            pub fn quota_user(mut self, value: impl Into<String>) -> Self {
                self.quota_user = Some(value.into());
                self
            }
            #[doc = "Deprecated. Please use quotaUser instead."]
            pub fn user_ip(mut self, value: impl Into<String>) -> Self {
                self.user_ip = Some(value.into());
                self
            }
            #[doc = r" Return an iterator that iterates over all `#prop_ident`. The"]
            #[doc = r" items yielded by the iterator are chosen by the caller of this"]
            #[doc = r" method and must implement `Deserialize` and `FieldSelector`. The"]
            #[doc = r" populated fields in the yielded items will be determined by the"]
            #[doc = r" `FieldSelector` implementation."]
            pub fn iter_delete_history<T>(mut self) -> crate::iter::PageItemIter<Self, T>
            where
                T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
            {
                let mut fields = concat!("nextPageToken,", "deleteHistory").to_owned();
                let items_fields = T::field_selector();
                if !items_fields.is_empty() {
                    fields.push_str("(");
                    fields.push_str(&items_fields);
                    fields.push_str(")");
                }
                self.fields = Some(fields);
                crate::iter::PageItemIter::new(self, "deleteHistory")
            }
            #[doc = r" Return an iterator that iterates over all `#prop_ident`. The"]
            #[doc = r" items yielded by the iterator are `#items_type`. The populated"]
            #[doc = r" fields in `#items_type` will be the default fields populated by"]
            #[doc = r" the server."]
            pub fn iter_delete_history_standard(
                mut self,
            ) -> crate::iter::PageItemIter<Self, crate::schemas::DeleteHistory> {
                self.fields = Some(concat!("nextPageToken,", "deleteHistory").to_owned());
                crate::iter::PageItemIter::new(self, "deleteHistory")
            }
            #[doc = r" Return an iterator that iterates over all `#prop_ident`. The"]
            #[doc = r" items yielded by the iterator are `#items_type`. The populated"]
            #[doc = r" fields in `#items_type` will be all fields available. This should"]
            #[doc = r" primarily be used during developement and debugging as fetching"]
            #[doc = r" all fields can be expensive both in bandwidth and server"]
            #[doc = r" resources."]
            pub fn iter_delete_history_debug(
                mut self,
            ) -> crate::iter::PageItemIter<Self, crate::schemas::DeleteHistory> {
                self.fields = Some(concat!("nextPageToken,", "deleteHistory", "(*)").to_owned());
                crate::iter::PageItemIter::new(self, "deleteHistory")
            }
            #[doc = r" Return an iterator that iterates over all `#prop_ident`. The"]
            #[doc = r" items yielded by the iterator are chosen by the caller of this"]
            #[doc = r" method and must implement `Deserialize` and `FieldSelector`. The"]
            #[doc = r" populated fields in the yielded items will be determined by the"]
            #[doc = r" `FieldSelector` implementation."]
            pub fn iter_fail<T>(mut self) -> crate::iter::PageItemIter<Self, T>
            where
                T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
            {
                let mut fields = concat!("nextPageToken,", "fail").to_owned();
                let items_fields = T::field_selector();
                if !items_fields.is_empty() {
                    fields.push_str("(");
                    fields.push_str(&items_fields);
                    fields.push_str(")");
                }
                self.fields = Some(fields);
                crate::iter::PageItemIter::new(self, "fail")
            }
            #[doc = r" Return an iterator that iterates over all `#prop_ident`. The"]
            #[doc = r" items yielded by the iterator are `#items_type`. The populated"]
            #[doc = r" fields in `#items_type` will be the default fields populated by"]
            #[doc = r" the server."]
            pub fn iter_fail_standard(
                mut self,
            ) -> crate::iter::PageItemIter<Self, crate::schemas::DataSourceChange> {
                self.fields = Some(concat!("nextPageToken,", "fail").to_owned());
                crate::iter::PageItemIter::new(self, "fail")
            }
            #[doc = r" Return an iterator that iterates over all `#prop_ident`. The"]
            #[doc = r" items yielded by the iterator are `#items_type`. The populated"]
            #[doc = r" fields in `#items_type` will be all fields available. This should"]
            #[doc = r" primarily be used during developement and debugging as fetching"]
            #[doc = r" all fields can be expensive both in bandwidth and server"]
            #[doc = r" resources."]
            pub fn iter_fail_debug(
                mut self,
            ) -> crate::iter::PageItemIter<Self, crate::schemas::DataSourceChange> {
                self.fields = Some(concat!("nextPageToken,", "fail", "(*)").to_owned());
                crate::iter::PageItemIter::new(self, "fail")
            }
            #[doc = r" Return an iterator that iterates over all `#prop_ident`. The"]
            #[doc = r" items yielded by the iterator are chosen by the caller of this"]
            #[doc = r" method and must implement `Deserialize` and `FieldSelector`. The"]
            #[doc = r" populated fields in the yielded items will be determined by the"]
            #[doc = r" `FieldSelector` implementation."]
            pub fn iter_insertion<T>(mut self) -> crate::iter::PageItemIter<Self, T>
            where
                T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
            {
                let mut fields = concat!("nextPageToken,", "insertion").to_owned();
                let items_fields = T::field_selector();
                if !items_fields.is_empty() {
                    fields.push_str("(");
                    fields.push_str(&items_fields);
                    fields.push_str(")");
                }
                self.fields = Some(fields);
                crate::iter::PageItemIter::new(self, "insertion")
            }
            #[doc = r" Return an iterator that iterates over all `#prop_ident`. The"]
            #[doc = r" items yielded by the iterator are `#items_type`. The populated"]
            #[doc = r" fields in `#items_type` will be the default fields populated by"]
            #[doc = r" the server."]
            pub fn iter_insertion_standard(
                mut self,
            ) -> crate::iter::PageItemIter<Self, crate::schemas::DataSourceChange> {
                self.fields = Some(concat!("nextPageToken,", "insertion").to_owned());
                crate::iter::PageItemIter::new(self, "insertion")
            }
            #[doc = r" Return an iterator that iterates over all `#prop_ident`. The"]
            #[doc = r" items yielded by the iterator are `#items_type`. The populated"]
            #[doc = r" fields in `#items_type` will be all fields available. This should"]
            #[doc = r" primarily be used during developement and debugging as fetching"]
            #[doc = r" all fields can be expensive both in bandwidth and server"]
            #[doc = r" resources."]
            pub fn iter_insertion_debug(
                mut self,
            ) -> crate::iter::PageItemIter<Self, crate::schemas::DataSourceChange> {
                self.fields = Some(concat!("nextPageToken,", "insertion", "(*)").to_owned());
                crate::iter::PageItemIter::new(self, "insertion")
            }
            pub fn iter<T>(mut self) -> crate::iter::PageIter<Self, T>
            where
                T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
            {
                let mut fields = T::field_selector();
                if !fields.is_empty() {
                    match fields.chars().rev().nth(0) {
                        Some(',') | None => {}
                        _ => fields.push_str(","),
                    }
                    fields.push_str("nextPageToken");
                    self.fields = Some(fields);
                }
                crate::iter::PageIter::new(self)
            }
            pub fn iter_standard(
                self,
            ) -> crate::iter::PageIter<Self, crate::schemas::SyncDataSourcesResponse> {
                crate::iter::PageIter::new(self)
            }
            pub fn iter_debug(
                mut self,
            ) -> crate::iter::PageIter<Self, crate::schemas::SyncDataSourcesResponse> {
                self.fields = Some("*".to_owned());
                crate::iter::PageIter::new(self)
            }
            #[doc = r" Execute the given operation. The fields requested are"]
            #[doc = r" determined by the FieldSelector attribute of the return type."]
            #[doc = r" This allows for flexible and ergonomic partial responses. See"]
            #[doc = r" `execute_standard` and `execute_debug` for interfaces that"]
            #[doc = r" are not generic over the return type and deserialize the"]
            #[doc = r" response into an auto-generated struct will all possible"]
            #[doc = r" fields."]
            pub fn execute<T>(self) -> Result<T, Box<dyn ::std::error::Error>>
            where
                T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
            {
                let fields = T::field_selector();
                let fields: Option<String> = if fields.is_empty() {
                    None
                } else {
                    Some(fields)
                };
                self.execute_fields(fields)
            }
            #[doc = r" Execute the given operation. This will not provide any"]
            #[doc = r" `fields` selector indicating that the server will determine"]
            #[doc = r" the fields returned. This typically includes the most common"]
            #[doc = r" fields, but it will not include every possible attribute of"]
            #[doc = r" the response resource."]
            pub fn execute_standard(
                self,
            ) -> Result<crate::schemas::SyncDataSourcesResponse, Box<dyn ::std::error::Error>>
            {
                self.execute_fields::<_, &str>(None)
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub fn execute_debug(
                self,
            ) -> Result<crate::schemas::SyncDataSourcesResponse, Box<dyn ::std::error::Error>>
            {
                self.execute_fields(Some("*"))
            }
            #[doc = r" Execute the given operation. This will use the `fields`"]
            #[doc = r" selector provided and will deserialize the response into"]
            #[doc = r" whatever return value is provided."]
            pub fn execute_fields<T, F>(
                mut self,
                fields: Option<F>,
            ) -> Result<T, Box<dyn ::std::error::Error>>
            where
                T: ::serde::de::DeserializeOwned,
                F: Into<String>,
            {
                self.fields = fields.map(Into::into);
                self._execute()
            }
            fn _execute<T>(&mut self) -> Result<T, Box<dyn ::std::error::Error>>
            where
                T: ::serde::de::DeserializeOwned,
            {
                let req = self._request(&self._path());
                let req = req.json(&self.request);
                Ok(req.send()?.error_for_status()?.json()?)
            }
            fn _path(&self) -> String {
                let mut output =
                    "https://www.googleapis.com/fitness/v0dogfoodfirstparty/users/".to_owned();
                {
                    let var_as_str = &self.user_id;
                    output.extend(::percent_encoding::utf8_percent_encode(
                        &var_as_str,
                        crate::SIMPLE,
                    ));
                }
                output.push_str("/syncDataSources");
                output
            }
            fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
                let req = self.reqwest.request(::reqwest::Method::POST, path);
                let req = req.query(&[("currentTimeMillis", &self.current_time_millis)]);
                let req = req.query(&[("header.serverToken", &self.header_server_token)]);
                let req = req.query(&[("header.userAgent", &self.header_user_agent)]);
                let req = req.query(&[("lastSyncTimeMillis", &self.last_sync_time_millis)]);
                let req = req.query(&[("pageToken", &self.page_token)]);
                let req = req.query(&[("alt", &self.alt)]);
                let req = req.query(&[("fields", &self.fields)]);
                let req = req.query(&[("key", &self.key)]);
                let req = req.query(&[("oauth_token", &self.oauth_token)]);
                let req = req.query(&[("prettyPrint", &self.pretty_print)]);
                let req = req.query(&[("quotaUser", &self.quota_user)]);
                let req = req.query(&[("userIp", &self.user_ip)]);
                req
            }
        }
        impl<'a, A: yup_oauth2::GetToken> crate::iter::IterableMethod
            for SyncDataSourcesRequestBuilder<'a, A>
        {
            fn set_page_token(&mut self, value: String) {
                self.page_token = value.into();
            }
            fn execute<T>(&mut self) -> Result<T, Box<dyn ::std::error::Error>>
            where
                T: ::serde::de::DeserializeOwned,
            {
                self._execute()
            }
        }
        #[derive(Debug, Clone)]
        pub struct SyncSessionsRequestBuilder<'a, A> {
            pub(crate) reqwest: &'a ::reqwest::Client,
            pub(crate) auth: &'a ::std::sync::Mutex<A>,
            request: crate::schemas::SessionChangeList,
            user_id: String,
            current_time_millis: Option<i64>,
            header_server_token: Option<String>,
            header_user_agent: Option<String>,
            alt: Option<crate::params::Alt>,
            fields: Option<String>,
            key: Option<String>,
            oauth_token: Option<String>,
            pretty_print: Option<bool>,
            quota_user: Option<String>,
            user_ip: Option<String>,
        }
        impl<'a, A: yup_oauth2::GetToken> SyncSessionsRequestBuilder<'a, A> {
            #[doc = ""]
            pub fn current_time_millis(mut self, value: i64) -> Self {
                self.current_time_millis = Some(value);
                self
            }
            #[doc = ""]
            pub fn header_server_token(mut self, value: impl Into<String>) -> Self {
                self.header_server_token = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn header_user_agent(mut self, value: impl Into<String>) -> Self {
                self.header_user_agent = Some(value.into());
                self
            }
            #[doc = "API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token."]
            pub fn key(mut self, value: impl Into<String>) -> Self {
                self.key = Some(value.into());
                self
            }
            #[doc = "OAuth 2.0 token for the current user."]
            pub fn oauth_token(mut self, value: impl Into<String>) -> Self {
                self.oauth_token = Some(value.into());
                self
            }
            #[doc = "Returns response with indentations and line breaks."]
            pub fn pretty_print(mut self, value: bool) -> Self {
                self.pretty_print = Some(value);
                self
            }
            #[doc = "An opaque string that represents a user for quota purposes. Must not exceed 40 characters."]
            pub fn quota_user(mut self, value: impl Into<String>) -> Self {
                self.quota_user = Some(value.into());
                self
            }
            #[doc = "Deprecated. Please use quotaUser instead."]
            pub fn user_ip(mut self, value: impl Into<String>) -> Self {
                self.user_ip = Some(value.into());
                self
            }
            #[doc = r" Execute the given operation. The fields requested are"]
            #[doc = r" determined by the FieldSelector attribute of the return type."]
            #[doc = r" This allows for flexible and ergonomic partial responses. See"]
            #[doc = r" `execute_standard` and `execute_debug` for interfaces that"]
            #[doc = r" are not generic over the return type and deserialize the"]
            #[doc = r" response into an auto-generated struct will all possible"]
            #[doc = r" fields."]
            pub fn execute<T>(self) -> Result<T, Box<dyn ::std::error::Error>>
            where
                T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
            {
                let fields = T::field_selector();
                let fields: Option<String> = if fields.is_empty() {
                    None
                } else {
                    Some(fields)
                };
                self.execute_fields(fields)
            }
            #[doc = r" Execute the given operation. This will not provide any"]
            #[doc = r" `fields` selector indicating that the server will determine"]
            #[doc = r" the fields returned. This typically includes the most common"]
            #[doc = r" fields, but it will not include every possible attribute of"]
            #[doc = r" the response resource."]
            pub fn execute_standard(
                self,
            ) -> Result<crate::schemas::SyncSessionsResponse, Box<dyn ::std::error::Error>>
            {
                self.execute_fields::<_, &str>(None)
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub fn execute_debug(
                self,
            ) -> Result<crate::schemas::SyncSessionsResponse, Box<dyn ::std::error::Error>>
            {
                self.execute_fields(Some("*"))
            }
            #[doc = r" Execute the given operation. This will use the `fields`"]
            #[doc = r" selector provided and will deserialize the response into"]
            #[doc = r" whatever return value is provided."]
            pub fn execute_fields<T, F>(
                mut self,
                fields: Option<F>,
            ) -> Result<T, Box<dyn ::std::error::Error>>
            where
                T: ::serde::de::DeserializeOwned,
                F: Into<String>,
            {
                self.fields = fields.map(Into::into);
                self._execute()
            }
            fn _execute<T>(&mut self) -> Result<T, Box<dyn ::std::error::Error>>
            where
                T: ::serde::de::DeserializeOwned,
            {
                let req = self._request(&self._path());
                let req = req.json(&self.request);
                Ok(req.send()?.error_for_status()?.json()?)
            }
            fn _path(&self) -> String {
                let mut output =
                    "https://www.googleapis.com/fitness/v0dogfoodfirstparty/users/".to_owned();
                {
                    let var_as_str = &self.user_id;
                    output.extend(::percent_encoding::utf8_percent_encode(
                        &var_as_str,
                        crate::SIMPLE,
                    ));
                }
                output.push_str("/syncSessions");
                output
            }
            fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
                let req = self.reqwest.request(::reqwest::Method::POST, path);
                let req = req.query(&[("currentTimeMillis", &self.current_time_millis)]);
                let req = req.query(&[("header.serverToken", &self.header_server_token)]);
                let req = req.query(&[("header.userAgent", &self.header_user_agent)]);
                let req = req.query(&[("alt", &self.alt)]);
                let req = req.query(&[("fields", &self.fields)]);
                let req = req.query(&[("key", &self.key)]);
                let req = req.query(&[("oauth_token", &self.oauth_token)]);
                let req = req.query(&[("prettyPrint", &self.pretty_print)]);
                let req = req.query(&[("quotaUser", &self.quota_user)]);
                let req = req.query(&[("userIp", &self.user_ip)]);
                req
            }
        }
        pub mod data_sources {
            pub mod params {}
            pub struct DataSourcesActions<'a, A> {
                pub(crate) reqwest: &'a reqwest::Client,
                pub(crate) auth: &'a std::sync::Mutex<A>,
            }
            impl<'a, A: yup_oauth2::GetToken> DataSourcesActions<'a, A> {
                #[doc = "Actions that can be performed on the data_point_changes resource"]pub fn data_point_changes ( & self ) -> crate :: resources :: users :: data_sources :: data_point_changes :: DataPointChangesActions < A >{
                    crate :: resources :: users :: data_sources :: data_point_changes :: DataPointChangesActions { reqwest : & self . reqwest , auth : & self . auth }
                }
                #[doc = "Actions that can be performed on the datasets resource"]
                pub fn datasets(
                    &self,
                ) -> crate::resources::users::data_sources::datasets::DatasetsActions<A>
                {
                    crate::resources::users::data_sources::datasets::DatasetsActions {
                        reqwest: &self.reqwest,
                        auth: &self.auth,
                    }
                }
            }
            pub mod data_point_changes {
                pub mod params {}
                pub struct DataPointChangesActions<'a, A> {
                    pub(crate) reqwest: &'a reqwest::Client,
                    pub(crate) auth: &'a std::sync::Mutex<A>,
                }
                impl<'a, A: yup_oauth2::GetToken> DataPointChangesActions<'a, A> {
                    #[doc = "TODO(jdwan): Remove this rpc after releasing the same rpc (with reduced fields) to 3p and after migrating 1p callers to use the 3p proto. results ordered by descending end_time"]
                    pub fn list(
                        &self,
                        user_id: impl Into<String>,
                        data_source_id: impl Into<String>,
                    ) -> ListRequestBuilder<A> {
                        ListRequestBuilder {
                            reqwest: &self.reqwest,
                            auth: &self.auth,
                            alt: None,
                            fields: None,
                            key: None,
                            oauth_token: None,
                            pretty_print: None,
                            quota_user: None,
                            user_ip: None,
                            user_id: user_id.into(),
                            data_source_id: data_source_id.into(),
                            current_time_millis: None,
                            deduplicate_changes: None,
                            header_server_token: None,
                            header_user_agent: None,
                            min_datapoints: None,
                            page_token: None,
                        }
                    }
                }
                #[derive(Debug, Clone)]
                pub struct ListRequestBuilder<'a, A> {
                    pub(crate) reqwest: &'a ::reqwest::Client,
                    pub(crate) auth: &'a ::std::sync::Mutex<A>,
                    user_id: String,
                    data_source_id: String,
                    current_time_millis: Option<i64>,
                    deduplicate_changes: Option<bool>,
                    header_server_token: Option<String>,
                    header_user_agent: Option<String>,
                    min_datapoints: Option<i32>,
                    page_token: Option<String>,
                    alt: Option<crate::params::Alt>,
                    fields: Option<String>,
                    key: Option<String>,
                    oauth_token: Option<String>,
                    pretty_print: Option<bool>,
                    quota_user: Option<String>,
                    user_ip: Option<String>,
                }
                impl<'a, A: yup_oauth2::GetToken> ListRequestBuilder<'a, A> {
                    #[doc = "The client's current time"]
                    pub fn current_time_millis(mut self, value: i64) -> Self {
                        self.current_time_millis = Some(value);
                        self
                    }
                    #[doc = "An indication of whether or not to de-duplicate the changes. If true, only one change per data point will be returned. If false, the full history of changes will be made available."]
                    pub fn deduplicate_changes(mut self, value: bool) -> Self {
                        self.deduplicate_changes = Some(value);
                        self
                    }
                    #[doc = ""]
                    pub fn header_server_token(mut self, value: impl Into<String>) -> Self {
                        self.header_server_token = Some(value.into());
                        self
                    }
                    #[doc = ""]
                    pub fn header_user_agent(mut self, value: impl Into<String>) -> Self {
                        self.header_user_agent = Some(value.into());
                        self
                    }
                    #[doc = "If recent changes are empty, include at least x datapoints"]
                    pub fn min_datapoints(mut self, value: i32) -> Self {
                        self.min_datapoints = Some(value);
                        self
                    }
                    #[doc = "The continuation token, which is used to page through large result sets. To get the next page of results, set this parameter to the value of nextPageToken from the previous response."]
                    pub fn page_token(mut self, value: impl Into<String>) -> Self {
                        self.page_token = Some(value.into());
                        self
                    }
                    #[doc = "API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token."]
                    pub fn key(mut self, value: impl Into<String>) -> Self {
                        self.key = Some(value.into());
                        self
                    }
                    #[doc = "OAuth 2.0 token for the current user."]
                    pub fn oauth_token(mut self, value: impl Into<String>) -> Self {
                        self.oauth_token = Some(value.into());
                        self
                    }
                    #[doc = "Returns response with indentations and line breaks."]
                    pub fn pretty_print(mut self, value: bool) -> Self {
                        self.pretty_print = Some(value);
                        self
                    }
                    #[doc = "An opaque string that represents a user for quota purposes. Must not exceed 40 characters."]
                    pub fn quota_user(mut self, value: impl Into<String>) -> Self {
                        self.quota_user = Some(value.into());
                        self
                    }
                    #[doc = "Deprecated. Please use quotaUser instead."]
                    pub fn user_ip(mut self, value: impl Into<String>) -> Self {
                        self.user_ip = Some(value.into());
                        self
                    }
                    #[doc = r" Return an iterator that iterates over all `#prop_ident`. The"]
                    #[doc = r" items yielded by the iterator are chosen by the caller of this"]
                    #[doc = r" method and must implement `Deserialize` and `FieldSelector`. The"]
                    #[doc = r" populated fields in the yielded items will be determined by the"]
                    #[doc = r" `FieldSelector` implementation."]
                    pub fn iter_delete_range<T>(mut self) -> crate::iter::PageItemIter<Self, T>
                    where
                        T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
                    {
                        let mut fields = concat!("nextPageToken,", "deleteRange").to_owned();
                        let items_fields = T::field_selector();
                        if !items_fields.is_empty() {
                            fields.push_str("(");
                            fields.push_str(&items_fields);
                            fields.push_str(")");
                        }
                        self.fields = Some(fields);
                        crate::iter::PageItemIter::new(self, "deleteRange")
                    }
                    #[doc = r" Return an iterator that iterates over all `#prop_ident`. The"]
                    #[doc = r" items yielded by the iterator are `#items_type`. The populated"]
                    #[doc = r" fields in `#items_type` will be the default fields populated by"]
                    #[doc = r" the server."]
                    pub fn iter_delete_range_standard(
                        mut self,
                    ) -> crate::iter::PageItemIter<Self, crate::schemas::TimeRange>
                    {
                        self.fields = Some(concat!("nextPageToken,", "deleteRange").to_owned());
                        crate::iter::PageItemIter::new(self, "deleteRange")
                    }
                    #[doc = r" Return an iterator that iterates over all `#prop_ident`. The"]
                    #[doc = r" items yielded by the iterator are `#items_type`. The populated"]
                    #[doc = r" fields in `#items_type` will be all fields available. This should"]
                    #[doc = r" primarily be used during developement and debugging as fetching"]
                    #[doc = r" all fields can be expensive both in bandwidth and server"]
                    #[doc = r" resources."]
                    pub fn iter_delete_range_debug(
                        mut self,
                    ) -> crate::iter::PageItemIter<Self, crate::schemas::TimeRange>
                    {
                        self.fields =
                            Some(concat!("nextPageToken,", "deleteRange", "(*)").to_owned());
                        crate::iter::PageItemIter::new(self, "deleteRange")
                    }
                    #[doc = r" Return an iterator that iterates over all `#prop_ident`. The"]
                    #[doc = r" items yielded by the iterator are chosen by the caller of this"]
                    #[doc = r" method and must implement `Deserialize` and `FieldSelector`. The"]
                    #[doc = r" populated fields in the yielded items will be determined by the"]
                    #[doc = r" `FieldSelector` implementation."]
                    pub fn iter_deleted_data_point<T>(
                        mut self,
                    ) -> crate::iter::PageItemIter<Self, T>
                    where
                        T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
                    {
                        let mut fields = concat!("nextPageToken,", "deletedDataPoint").to_owned();
                        let items_fields = T::field_selector();
                        if !items_fields.is_empty() {
                            fields.push_str("(");
                            fields.push_str(&items_fields);
                            fields.push_str(")");
                        }
                        self.fields = Some(fields);
                        crate::iter::PageItemIter::new(self, "deletedDataPoint")
                    }
                    #[doc = r" Return an iterator that iterates over all `#prop_ident`. The"]
                    #[doc = r" items yielded by the iterator are `#items_type`. The populated"]
                    #[doc = r" fields in `#items_type` will be the default fields populated by"]
                    #[doc = r" the server."]
                    pub fn iter_deleted_data_point_standard(
                        mut self,
                    ) -> crate::iter::PageItemIter<Self, crate::schemas::DataPoint>
                    {
                        self.fields =
                            Some(concat!("nextPageToken,", "deletedDataPoint").to_owned());
                        crate::iter::PageItemIter::new(self, "deletedDataPoint")
                    }
                    #[doc = r" Return an iterator that iterates over all `#prop_ident`. The"]
                    #[doc = r" items yielded by the iterator are `#items_type`. The populated"]
                    #[doc = r" fields in `#items_type` will be all fields available. This should"]
                    #[doc = r" primarily be used during developement and debugging as fetching"]
                    #[doc = r" all fields can be expensive both in bandwidth and server"]
                    #[doc = r" resources."]
                    pub fn iter_deleted_data_point_debug(
                        mut self,
                    ) -> crate::iter::PageItemIter<Self, crate::schemas::DataPoint>
                    {
                        self.fields =
                            Some(concat!("nextPageToken,", "deletedDataPoint", "(*)").to_owned());
                        crate::iter::PageItemIter::new(self, "deletedDataPoint")
                    }
                    #[doc = r" Return an iterator that iterates over all `#prop_ident`. The"]
                    #[doc = r" items yielded by the iterator are chosen by the caller of this"]
                    #[doc = r" method and must implement `Deserialize` and `FieldSelector`. The"]
                    #[doc = r" populated fields in the yielded items will be determined by the"]
                    #[doc = r" `FieldSelector` implementation."]
                    pub fn iter_inserted_data_point<T>(
                        mut self,
                    ) -> crate::iter::PageItemIter<Self, T>
                    where
                        T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
                    {
                        let mut fields = concat!("nextPageToken,", "insertedDataPoint").to_owned();
                        let items_fields = T::field_selector();
                        if !items_fields.is_empty() {
                            fields.push_str("(");
                            fields.push_str(&items_fields);
                            fields.push_str(")");
                        }
                        self.fields = Some(fields);
                        crate::iter::PageItemIter::new(self, "insertedDataPoint")
                    }
                    #[doc = r" Return an iterator that iterates over all `#prop_ident`. The"]
                    #[doc = r" items yielded by the iterator are `#items_type`. The populated"]
                    #[doc = r" fields in `#items_type` will be the default fields populated by"]
                    #[doc = r" the server."]
                    pub fn iter_inserted_data_point_standard(
                        mut self,
                    ) -> crate::iter::PageItemIter<Self, crate::schemas::DataPoint>
                    {
                        self.fields =
                            Some(concat!("nextPageToken,", "insertedDataPoint").to_owned());
                        crate::iter::PageItemIter::new(self, "insertedDataPoint")
                    }
                    #[doc = r" Return an iterator that iterates over all `#prop_ident`. The"]
                    #[doc = r" items yielded by the iterator are `#items_type`. The populated"]
                    #[doc = r" fields in `#items_type` will be all fields available. This should"]
                    #[doc = r" primarily be used during developement and debugging as fetching"]
                    #[doc = r" all fields can be expensive both in bandwidth and server"]
                    #[doc = r" resources."]
                    pub fn iter_inserted_data_point_debug(
                        mut self,
                    ) -> crate::iter::PageItemIter<Self, crate::schemas::DataPoint>
                    {
                        self.fields =
                            Some(concat!("nextPageToken,", "insertedDataPoint", "(*)").to_owned());
                        crate::iter::PageItemIter::new(self, "insertedDataPoint")
                    }
                    pub fn iter<T>(mut self) -> crate::iter::PageIter<Self, T>
                    where
                        T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
                    {
                        let mut fields = T::field_selector();
                        if !fields.is_empty() {
                            match fields.chars().rev().nth(0) {
                                Some(',') | None => {}
                                _ => fields.push_str(","),
                            }
                            fields.push_str("nextPageToken");
                            self.fields = Some(fields);
                        }
                        crate::iter::PageIter::new(self)
                    }
                    pub fn iter_standard(
                        self,
                    ) -> crate::iter::PageIter<Self, crate::schemas::ListDataPointChangesResponse>
                    {
                        crate::iter::PageIter::new(self)
                    }
                    pub fn iter_debug(
                        mut self,
                    ) -> crate::iter::PageIter<Self, crate::schemas::ListDataPointChangesResponse>
                    {
                        self.fields = Some("*".to_owned());
                        crate::iter::PageIter::new(self)
                    }
                    #[doc = r" Execute the given operation. The fields requested are"]
                    #[doc = r" determined by the FieldSelector attribute of the return type."]
                    #[doc = r" This allows for flexible and ergonomic partial responses. See"]
                    #[doc = r" `execute_standard` and `execute_debug` for interfaces that"]
                    #[doc = r" are not generic over the return type and deserialize the"]
                    #[doc = r" response into an auto-generated struct will all possible"]
                    #[doc = r" fields."]
                    pub fn execute<T>(self) -> Result<T, Box<dyn ::std::error::Error>>
                    where
                        T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
                    {
                        let fields = T::field_selector();
                        let fields: Option<String> = if fields.is_empty() {
                            None
                        } else {
                            Some(fields)
                        };
                        self.execute_fields(fields)
                    }
                    #[doc = r" Execute the given operation. This will not provide any"]
                    #[doc = r" `fields` selector indicating that the server will determine"]
                    #[doc = r" the fields returned. This typically includes the most common"]
                    #[doc = r" fields, but it will not include every possible attribute of"]
                    #[doc = r" the response resource."]
                    pub fn execute_standard(
                        self,
                    ) -> Result<
                        crate::schemas::ListDataPointChangesResponse,
                        Box<dyn ::std::error::Error>,
                    > {
                        self.execute_fields::<_, &str>(None)
                    }
                    #[doc = r" Execute the given operation. This will provide a `fields`"]
                    #[doc = r" selector of `*`. This will include every attribute of the"]
                    #[doc = r" response resource and should be limited to use during"]
                    #[doc = r" development or debugging."]
                    pub fn execute_debug(
                        self,
                    ) -> Result<
                        crate::schemas::ListDataPointChangesResponse,
                        Box<dyn ::std::error::Error>,
                    > {
                        self.execute_fields(Some("*"))
                    }
                    #[doc = r" Execute the given operation. This will use the `fields`"]
                    #[doc = r" selector provided and will deserialize the response into"]
                    #[doc = r" whatever return value is provided."]
                    pub fn execute_fields<T, F>(
                        mut self,
                        fields: Option<F>,
                    ) -> Result<T, Box<dyn ::std::error::Error>>
                    where
                        T: ::serde::de::DeserializeOwned,
                        F: Into<String>,
                    {
                        self.fields = fields.map(Into::into);
                        self._execute()
                    }
                    fn _execute<T>(&mut self) -> Result<T, Box<dyn ::std::error::Error>>
                    where
                        T: ::serde::de::DeserializeOwned,
                    {
                        let req = self._request(&self._path());
                        Ok(req.send()?.error_for_status()?.json()?)
                    }
                    fn _path(&self) -> String {
                        let mut output =
                            "https://www.googleapis.com/fitness/v0dogfoodfirstparty/users/"
                                .to_owned();
                        {
                            let var_as_str = &self.user_id;
                            output.extend(::percent_encoding::utf8_percent_encode(
                                &var_as_str,
                                crate::SIMPLE,
                            ));
                        }
                        output.push_str("/dataSources/");
                        {
                            let var_as_str = &self.data_source_id;
                            output.extend(::percent_encoding::utf8_percent_encode(
                                &var_as_str,
                                crate::SIMPLE,
                            ));
                        }
                        output.push_str("/dataPointChanges");
                        output
                    }
                    fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
                        let req = self.reqwest.request(::reqwest::Method::GET, path);
                        let req = req.query(&[("currentTimeMillis", &self.current_time_millis)]);
                        let req = req.query(&[("deduplicateChanges", &self.deduplicate_changes)]);
                        let req = req.query(&[("header.serverToken", &self.header_server_token)]);
                        let req = req.query(&[("header.userAgent", &self.header_user_agent)]);
                        let req = req.query(&[("minDatapoints", &self.min_datapoints)]);
                        let req = req.query(&[("pageToken", &self.page_token)]);
                        let req = req.query(&[("alt", &self.alt)]);
                        let req = req.query(&[("fields", &self.fields)]);
                        let req = req.query(&[("key", &self.key)]);
                        let req = req.query(&[("oauth_token", &self.oauth_token)]);
                        let req = req.query(&[("prettyPrint", &self.pretty_print)]);
                        let req = req.query(&[("quotaUser", &self.quota_user)]);
                        let req = req.query(&[("userIp", &self.user_ip)]);
                        req
                    }
                }
                impl<'a, A: yup_oauth2::GetToken> crate::iter::IterableMethod for ListRequestBuilder<'a, A> {
                    fn set_page_token(&mut self, value: String) {
                        self.page_token = value.into();
                    }
                    fn execute<T>(&mut self) -> Result<T, Box<dyn ::std::error::Error>>
                    where
                        T: ::serde::de::DeserializeOwned,
                    {
                        self._execute()
                    }
                }
            }
            pub mod datasets {
                pub mod params {}
                pub struct DatasetsActions<'a, A> {
                    pub(crate) reqwest: &'a reqwest::Client,
                    pub(crate) auth: &'a std::sync::Mutex<A>,
                }
                impl<'a, A: yup_oauth2::GetToken> DatasetsActions<'a, A> {
                    #[doc = "Actions that can be performed on the data_points resource"]pub fn data_points ( & self ) -> crate :: resources :: users :: data_sources :: datasets :: data_points :: DataPointsActions < A >{
                        crate :: resources :: users :: data_sources :: datasets :: data_points :: DataPointsActions { reqwest : & self . reqwest , auth : & self . auth }
                    }
                }
                pub mod data_points {
                    pub mod params {}
                    pub struct DataPointsActions<'a, A> {
                        pub(crate) reqwest: &'a reqwest::Client,
                        pub(crate) auth: &'a std::sync::Mutex<A>,
                    }
                    impl<'a, A: yup_oauth2::GetToken> DataPointsActions<'a, A> {
                        #[doc = "Data points within a dataset can be listed, but this method is primarily useful for listing data points that can be contained within any dataset, modified after a given time."]
                        pub fn list(
                            &self,
                            user_id: impl Into<String>,
                            data_source_id: impl Into<String>,
                            dataset_id: impl Into<String>,
                        ) -> ListRequestBuilder<A> {
                            ListRequestBuilder {
                                reqwest: &self.reqwest,
                                auth: &self.auth,
                                alt: None,
                                fields: None,
                                key: None,
                                oauth_token: None,
                                pretty_print: None,
                                quota_user: None,
                                user_ip: None,
                                user_id: user_id.into(),
                                data_source_id: data_source_id.into(),
                                dataset_id: dataset_id.into(),
                                current_time_millis: None,
                                header_server_token: None,
                                header_user_agent: None,
                                include_latest_n: None,
                                max_end_time_millis: None,
                            }
                        }
                    }
                    #[derive(Debug, Clone)]
                    pub struct ListRequestBuilder<'a, A> {
                        pub(crate) reqwest: &'a ::reqwest::Client,
                        pub(crate) auth: &'a ::std::sync::Mutex<A>,
                        user_id: String,
                        data_source_id: String,
                        dataset_id: String,
                        current_time_millis: Option<i64>,
                        header_server_token: Option<String>,
                        header_user_agent: Option<String>,
                        include_latest_n: Option<i64>,
                        max_end_time_millis: Option<i64>,
                        alt: Option<crate::params::Alt>,
                        fields: Option<String>,
                        key: Option<String>,
                        oauth_token: Option<String>,
                        pretty_print: Option<bool>,
                        quota_user: Option<String>,
                        user_ip: Option<String>,
                    }
                    impl<'a, A: yup_oauth2::GetToken> ListRequestBuilder<'a, A> {
                        #[doc = "The client's current time"]
                        pub fn current_time_millis(mut self, value: i64) -> Self {
                            self.current_time_millis = Some(value);
                            self
                        }
                        #[doc = ""]
                        pub fn header_server_token(mut self, value: impl Into<String>) -> Self {
                            self.header_server_token = Some(value.into());
                            self
                        }
                        #[doc = ""]
                        pub fn header_user_agent(mut self, value: impl Into<String>) -> Self {
                            self.header_user_agent = Some(value.into());
                            self
                        }
                        #[doc = "If specified, only the most recent N points will be included in the response."]
                        pub fn include_latest_n(mut self, value: i64) -> Self {
                            self.include_latest_n = Some(value);
                            self
                        }
                        #[doc = "If specified, only data points with end time equal to or before this value value will be included."]
                        pub fn max_end_time_millis(mut self, value: i64) -> Self {
                            self.max_end_time_millis = Some(value);
                            self
                        }
                        #[doc = "API key. Your API key identifies your project and provides you with API access, quota, and reports. Required unless you provide an OAuth 2.0 token."]
                        pub fn key(mut self, value: impl Into<String>) -> Self {
                            self.key = Some(value.into());
                            self
                        }
                        #[doc = "OAuth 2.0 token for the current user."]
                        pub fn oauth_token(mut self, value: impl Into<String>) -> Self {
                            self.oauth_token = Some(value.into());
                            self
                        }
                        #[doc = "Returns response with indentations and line breaks."]
                        pub fn pretty_print(mut self, value: bool) -> Self {
                            self.pretty_print = Some(value);
                            self
                        }
                        #[doc = "An opaque string that represents a user for quota purposes. Must not exceed 40 characters."]
                        pub fn quota_user(mut self, value: impl Into<String>) -> Self {
                            self.quota_user = Some(value.into());
                            self
                        }
                        #[doc = "Deprecated. Please use quotaUser instead."]
                        pub fn user_ip(mut self, value: impl Into<String>) -> Self {
                            self.user_ip = Some(value.into());
                            self
                        }
                        #[doc = r" Execute the given operation. The fields requested are"]
                        #[doc = r" determined by the FieldSelector attribute of the return type."]
                        #[doc = r" This allows for flexible and ergonomic partial responses. See"]
                        #[doc = r" `execute_standard` and `execute_debug` for interfaces that"]
                        #[doc = r" are not generic over the return type and deserialize the"]
                        #[doc = r" response into an auto-generated struct will all possible"]
                        #[doc = r" fields."]
                        pub fn execute<T>(self) -> Result<T, Box<dyn ::std::error::Error>>
                        where
                            T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
                        {
                            let fields = T::field_selector();
                            let fields: Option<String> = if fields.is_empty() {
                                None
                            } else {
                                Some(fields)
                            };
                            self.execute_fields(fields)
                        }
                        #[doc = r" Execute the given operation. This will not provide any"]
                        #[doc = r" `fields` selector indicating that the server will determine"]
                        #[doc = r" the fields returned. This typically includes the most common"]
                        #[doc = r" fields, but it will not include every possible attribute of"]
                        #[doc = r" the response resource."]
                        pub fn execute_standard(
                            self,
                        ) -> Result<
                            crate::schemas::ListDataPointsResponse,
                            Box<dyn ::std::error::Error>,
                        > {
                            self.execute_fields::<_, &str>(None)
                        }
                        #[doc = r" Execute the given operation. This will provide a `fields`"]
                        #[doc = r" selector of `*`. This will include every attribute of the"]
                        #[doc = r" response resource and should be limited to use during"]
                        #[doc = r" development or debugging."]
                        pub fn execute_debug(
                            self,
                        ) -> Result<
                            crate::schemas::ListDataPointsResponse,
                            Box<dyn ::std::error::Error>,
                        > {
                            self.execute_fields(Some("*"))
                        }
                        #[doc = r" Execute the given operation. This will use the `fields`"]
                        #[doc = r" selector provided and will deserialize the response into"]
                        #[doc = r" whatever return value is provided."]
                        pub fn execute_fields<T, F>(
                            mut self,
                            fields: Option<F>,
                        ) -> Result<T, Box<dyn ::std::error::Error>>
                        where
                            T: ::serde::de::DeserializeOwned,
                            F: Into<String>,
                        {
                            self.fields = fields.map(Into::into);
                            self._execute()
                        }
                        fn _execute<T>(&mut self) -> Result<T, Box<dyn ::std::error::Error>>
                        where
                            T: ::serde::de::DeserializeOwned,
                        {
                            let req = self._request(&self._path());
                            Ok(req.send()?.error_for_status()?.json()?)
                        }
                        fn _path(&self) -> String {
                            let mut output =
                                "https://www.googleapis.com/fitness/v0dogfoodfirstparty/users/"
                                    .to_owned();
                            {
                                let var_as_str = &self.user_id;
                                output.extend(::percent_encoding::utf8_percent_encode(
                                    &var_as_str,
                                    crate::SIMPLE,
                                ));
                            }
                            output.push_str("/dataSources/");
                            {
                                let var_as_str = &self.data_source_id;
                                output.extend(::percent_encoding::utf8_percent_encode(
                                    &var_as_str,
                                    crate::SIMPLE,
                                ));
                            }
                            output.push_str("/datasets/");
                            {
                                let var_as_str = &self.dataset_id;
                                output.extend(::percent_encoding::utf8_percent_encode(
                                    &var_as_str,
                                    crate::SIMPLE,
                                ));
                            }
                            output.push_str("/dataPoints");
                            output
                        }
                        fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
                            let req = self.reqwest.request(::reqwest::Method::GET, path);
                            let req =
                                req.query(&[("currentTimeMillis", &self.current_time_millis)]);
                            let req =
                                req.query(&[("header.serverToken", &self.header_server_token)]);
                            let req = req.query(&[("header.userAgent", &self.header_user_agent)]);
                            let req = req.query(&[("includeLatestN", &self.include_latest_n)]);
                            let req = req.query(&[("maxEndTimeMillis", &self.max_end_time_millis)]);
                            let req = req.query(&[("alt", &self.alt)]);
                            let req = req.query(&[("fields", &self.fields)]);
                            let req = req.query(&[("key", &self.key)]);
                            let req = req.query(&[("oauth_token", &self.oauth_token)]);
                            let req = req.query(&[("prettyPrint", &self.pretty_print)]);
                            let req = req.query(&[("quotaUser", &self.quota_user)]);
                            let req = req.query(&[("userIp", &self.user_ip)]);
                            req
                        }
                    }
                }
            }
        }
    }
}
#[allow(dead_code)]
const SIMPLE: &::percent_encoding::AsciiSet = &::percent_encoding::NON_ALPHANUMERIC
    .remove(b'-')
    .remove(b'.')
    .remove(b'_')
    .remove(b'~');

#[allow(dead_code)]
const RESERVED: &::percent_encoding::AsciiSet = &SIMPLE
    .remove(b'%')
    .remove(b':')
    .remove(b'/')
    .remove(b'?')
    .remove(b'#')
    .remove(b'[')
    .remove(b']')
    .remove(b'@')
    .remove(b'!')
    .remove(b'$')
    .remove(b'&')
    .remove(b'\'')
    .remove(b'(')
    .remove(b')')
    .remove(b'*')
    .remove(b'+')
    .remove(b',')
    .remove(b';')
    .remove(b'=');
#[allow(dead_code)]
mod multipart {
    pub(crate) struct RelatedMultiPart {
        parts: Vec<Part>,
        boundary: String,
    }

    impl RelatedMultiPart {
        pub(crate) fn new() -> Self {
            RelatedMultiPart {
                parts: Vec::new(),
                boundary: ::textnonce::TextNonce::sized(68).unwrap().0,
            }
        }

        pub(crate) fn new_part(&mut self, part: Part) {
            self.parts.push(part);
        }

        pub(crate) fn boundary(&self) -> &str {
            &self.boundary
        }

        pub(crate) fn into_reader(self) -> RelatedMultiPartReader {
            let boundary_marker = boundary_marker(&self.boundary);
            RelatedMultiPartReader {
                state: RelatedMultiPartReaderState::WriteBoundary {
                    start: 0,
                    boundary: format!("{}\r\n", &boundary_marker),
                },
                boundary: boundary_marker,
                next_body: None,
                parts: self.parts.into_iter(),
            }
        }
    }

    pub(crate) struct Part {
        content_type: ::mime::Mime,
        body: Box<dyn ::std::io::Read + Send>,
    }

    impl Part {
        pub(crate) fn new(
            content_type: ::mime::Mime,
            body: Box<dyn ::std::io::Read + Send>,
        ) -> Part {
            Part { content_type, body }
        }
    }

    pub(crate) struct RelatedMultiPartReader {
        state: RelatedMultiPartReaderState,
        boundary: String,
        next_body: Option<Box<dyn ::std::io::Read + Send>>,
        parts: std::vec::IntoIter<Part>,
    }

    enum RelatedMultiPartReaderState {
        WriteBoundary {
            start: usize,
            boundary: String,
        },
        WriteContentType {
            start: usize,
            content_type: Vec<u8>,
        },
        WriteBody {
            body: Box<dyn ::std::io::Read + Send>,
        },
    }

    impl ::std::io::Read for RelatedMultiPartReader {
        fn read(&mut self, buf: &mut [u8]) -> ::std::io::Result<usize> {
            use RelatedMultiPartReaderState::*;
            let mut bytes_written: usize = 0;
            loop {
                let rem_buf = &mut buf[bytes_written..];
                match &mut self.state {
                    WriteBoundary { start, boundary } => {
                        let bytes_to_copy = std::cmp::min(boundary.len() - *start, rem_buf.len());
                        rem_buf[..bytes_to_copy]
                            .copy_from_slice(&boundary.as_bytes()[*start..*start + bytes_to_copy]);
                        *start += bytes_to_copy;
                        bytes_written += bytes_to_copy;
                        if *start == boundary.len() {
                            let next_part = match self.parts.next() {
                                None => break,
                                Some(part) => part,
                            };
                            self.next_body = Some(next_part.body);
                            self.state = WriteContentType {
                                start: 0,
                                content_type: format!(
                                    "Content-Type: {}\r\n\r\n",
                                    next_part.content_type
                                )
                                .into_bytes(),
                            };
                        } else {
                            break;
                        }
                    }
                    WriteContentType {
                        start,
                        content_type,
                    } => {
                        let bytes_to_copy =
                            std::cmp::min(content_type.len() - *start, rem_buf.len());
                        rem_buf[..bytes_to_copy]
                            .copy_from_slice(&content_type[*start..*start + bytes_to_copy]);
                        *start += bytes_to_copy;
                        bytes_written += bytes_to_copy;
                        if *start == content_type.len() {
                            self.state = WriteBody {
                                body: self.next_body.take().unwrap(),
                            };
                        } else {
                            break;
                        }
                    }
                    WriteBody { body } => {
                        let written = body.read(rem_buf)?;
                        bytes_written += written;
                        if written == 0 {
                            self.state = WriteBoundary {
                                start: 0,
                                boundary: format!("\r\n{}\r\n", &self.boundary),
                            };
                        } else {
                            break;
                        }
                    }
                }
            }
            Ok(bytes_written)
        }
    }

    fn boundary_marker(boundary: &str) -> String {
        let mut marker = String::with_capacity(boundary.len() + 2);
        marker.push_str("--");
        marker.push_str(boundary);
        marker
    }
}
pub struct ResumableUpload {
    reqwest: ::reqwest::Client,
    url: String,
    progress: Option<i64>,
}

impl ResumableUpload {
    pub fn new(reqwest: ::reqwest::Client, url: String) -> Self {
        ResumableUpload {
            reqwest,
            url,
            progress: None,
        }
    }

    pub fn url(&self) -> &str {
        &self.url
    }

    pub fn upload<R>(&mut self, mut reader: R) -> Result<(), Box<dyn ::std::error::Error>>
    where
        R: ::std::io::Read + ::std::io::Seek + Send + 'static,
    {
        let reader_len = {
            let start = reader.seek(::std::io::SeekFrom::Current(0))?;
            let end = reader.seek(::std::io::SeekFrom::End(0))?;
            reader.seek(::std::io::SeekFrom::Start(start))?;
            end
        };
        let progress = match self.progress {
            Some(progress) => progress,
            None => {
                let req = self.reqwest.request(::reqwest::Method::PUT, &self.url);
                let req = req.header(::reqwest::header::CONTENT_LENGTH, 0);
                let req = req.header(
                    ::reqwest::header::CONTENT_RANGE,
                    format!("bytes */{}", reader_len),
                );
                let resp = req.send()?.error_for_status()?;
                match resp.headers().get(::reqwest::header::RANGE) {
                    Some(range_header) => {
                        let (_, progress) = parse_range_header(range_header)
                            .map_err(|e| format!("invalid RANGE header: {}", e))?;
                        progress + 1
                    }
                    None => 0,
                }
            }
        };

        reader.seek(::std::io::SeekFrom::Start(progress as u64))?;
        let content_length = reader_len - progress as u64;
        let content_range = format!("bytes {}-{}/{}", progress, reader_len - 1, reader_len);
        let req = self.reqwest.request(::reqwest::Method::PUT, &self.url);
        let req = req.header(::reqwest::header::CONTENT_RANGE, content_range);
        let req = req.body(::reqwest::Body::sized(reader, content_length));
        req.send()?.error_for_status()?;
        Ok(())
    }
}

fn parse_range_header(
    range: &::reqwest::header::HeaderValue,
) -> Result<(i64, i64), Box<dyn ::std::error::Error>> {
    let range = range.to_str()?;
    if !range.starts_with("bytes ") {
        return Err(r#"does not begin with "bytes""#.to_owned().into());
    }
    let range = &range[6..];
    let slash_idx = range
        .find('/')
        .ok_or_else(|| r#"does not contain"#.to_owned())?;
    let (begin, end) = range.split_at(slash_idx);
    let end = &end[1..]; // remove '/'
    let begin: i64 = begin.parse()?;
    let end: i64 = end.parse()?;
    Ok((begin, end))
}
// A serde helper module that can be used with the `with` attribute
// to deserialize any string to a FromStr type and serialize any
// Display type to a String. Google API's encode i64, u64 values as
// strings.
#[allow(dead_code)]
mod parsed_string {
    pub fn serialize<T, S>(
        value: &Option<T>,
        serializer: S,
    ) -> ::std::result::Result<S::Ok, S::Error>
    where
        T: ::std::fmt::Display,
        S: ::serde::Serializer,
    {
        use ::serde::Serialize;
        value.as_ref().map(|x| x.to_string()).serialize(serializer)
    }

    pub fn deserialize<'de, T, D>(deserializer: D) -> ::std::result::Result<Option<T>, D::Error>
    where
        T: ::std::str::FromStr,
        T::Err: ::std::fmt::Display,
        D: ::serde::de::Deserializer<'de>,
    {
        use ::serde::Deserialize;
        match Option::<String>::deserialize(deserializer)? {
            Some(x) => Ok(Some(x.parse().map_err(::serde::de::Error::custom)?)),
            None => Ok(None),
        }
    }
}
#[allow(dead_code)]
pub mod iter {
    pub trait IterableMethod {
        fn set_page_token(&mut self, value: String);
        fn execute<T>(&mut self) -> Result<T, Box<dyn ::std::error::Error>>
        where
            T: ::serde::de::DeserializeOwned;
    }

    pub struct PageIter<M, T> {
        pub method: M,
        pub finished: bool,
        pub _phantom: ::std::marker::PhantomData<T>,
    }

    impl<M, T> PageIter<M, T>
    where
        M: IterableMethod,
        T: ::serde::de::DeserializeOwned,
    {
        pub(crate) fn new(method: M) -> Self {
            PageIter {
                method,
                finished: false,
                _phantom: ::std::marker::PhantomData,
            }
        }
    }

    impl<M, T> Iterator for PageIter<M, T>
    where
        M: IterableMethod,
        T: ::serde::de::DeserializeOwned,
    {
        type Item = Result<T, Box<dyn ::std::error::Error>>;

        fn next(&mut self) -> Option<Result<T, Box<dyn ::std::error::Error>>> {
            if self.finished {
                return None;
            }
            let paginated_result: ::serde_json::Map<String, ::serde_json::Value> =
                match self.method.execute() {
                    Ok(r) => r,
                    Err(err) => return Some(Err(err)),
                };
            if let Some(next_page_token) = paginated_result
                .get("nextPageToken")
                .and_then(|t| t.as_str())
            {
                self.method.set_page_token(next_page_token.to_owned());
            } else {
                self.finished = true;
            }

            Some(
                match ::serde_json::from_value(::serde_json::Value::Object(paginated_result)) {
                    Ok(resp) => Ok(resp),
                    Err(err) => Err(err.into()),
                },
            )
        }
    }

    pub struct PageItemIter<M, T> {
        items_field: &'static str,
        page_iter: PageIter<M, ::serde_json::Map<String, ::serde_json::Value>>,
        items: ::std::vec::IntoIter<T>,
    }

    impl<M, T> PageItemIter<M, T>
    where
        M: IterableMethod,
        T: ::serde::de::DeserializeOwned,
    {
        pub(crate) fn new(method: M, items_field: &'static str) -> Self {
            PageItemIter {
                items_field,
                page_iter: PageIter::new(method),
                items: Vec::new().into_iter(),
            }
        }
    }

    impl<M, T> Iterator for PageItemIter<M, T>
    where
        M: IterableMethod,
        T: ::serde::de::DeserializeOwned,
    {
        type Item = Result<T, Box<dyn ::std::error::Error>>;

        fn next(&mut self) -> Option<Result<T, Box<dyn ::std::error::Error>>> {
            loop {
                if let Some(v) = self.items.next() {
                    return Some(Ok(v));
                }

                let next_page = self.page_iter.next();
                match next_page {
                    None => return None,
                    Some(Err(err)) => return Some(Err(err)),
                    Some(Ok(next_page)) => {
                        let mut next_page: ::serde_json::Map<String, ::serde_json::Value> =
                            next_page;
                        let items_array = match next_page.remove(self.items_field) {
                            Some(items) => items,
                            None => {
                                return Some(Err(format!(
                                    "no {} field found in iter response",
                                    self.items_field
                                )
                                .into()))
                            }
                        };
                        let items_vec: Result<Vec<T>, _> = ::serde_json::from_value(items_array);
                        match items_vec {
                            Ok(items) => self.items = items.into_iter(),
                            Err(err) => return Some(Err(err.into())),
                        }
                    }
                }
            }
        }
    }
} // Bytes in google apis are represented as urlsafe base64 encoded strings.
  // This defines a Bytes type that is a simple wrapper around a Vec<u8> used
  // internally to handle byte fields in google apis.
#[allow(dead_code)]
mod bytes {
    use radix64::URL_SAFE as BASE64_CFG;

    #[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
    pub struct Bytes(Vec<u8>);

    impl ::std::convert::From<Vec<u8>> for Bytes {
        fn from(x: Vec<u8>) -> Bytes {
            Bytes(x)
        }
    }

    impl ::std::fmt::Display for Bytes {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> ::std::fmt::Result {
            ::radix64::Display::new(BASE64_CFG, &self.0).fmt(f)
        }
    }

    impl ::serde::Serialize for Bytes {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::Serializer,
        {
            let encoded = BASE64_CFG.encode(&self.0);
            encoded.serialize(serializer)
        }
    }

    impl<'de> ::serde::Deserialize<'de> for Bytes {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Bytes, D::Error>
        where
            D: ::serde::Deserializer<'de>,
        {
            let encoded = String::deserialize(deserializer)?;
            let decoded = BASE64_CFG
                .decode(&encoded)
                .map_err(|_| ::serde::de::Error::custom("invalid base64 input"))?;
            Ok(Bytes(decoded))
        }
    }
}
