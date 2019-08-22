pub mod schemas {
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum CapsuleCapsuleType {
        CityRank,
        Goal,
        GoalV2,
        Graph,
        Invite,
        Notification,
        Nutrition,
        PersonalRecord,
        Session,
        SingleMetric,
        Today,
        UnknownCapsuleType,
        Weekly,
    }
    impl CapsuleCapsuleType {
        pub fn as_str(self) -> &'static str {
            match self {
                CapsuleCapsuleType::CityRank => "cityRank",
                CapsuleCapsuleType::Goal => "goal",
                CapsuleCapsuleType::GoalV2 => "goalV2",
                CapsuleCapsuleType::Graph => "graph",
                CapsuleCapsuleType::Invite => "invite",
                CapsuleCapsuleType::Notification => "notification",
                CapsuleCapsuleType::Nutrition => "nutrition",
                CapsuleCapsuleType::PersonalRecord => "personalRecord",
                CapsuleCapsuleType::Session => "session",
                CapsuleCapsuleType::SingleMetric => "singleMetric",
                CapsuleCapsuleType::Today => "today",
                CapsuleCapsuleType::UnknownCapsuleType => "unknownCapsuleType",
                CapsuleCapsuleType::Weekly => "weekly",
            }
        }
    }
    impl ::std::fmt::Display for CapsuleCapsuleType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for CapsuleCapsuleType {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for CapsuleCapsuleType {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "cityRank" => CapsuleCapsuleType::CityRank,
                "goal" => CapsuleCapsuleType::Goal,
                "goalV2" => CapsuleCapsuleType::GoalV2,
                "graph" => CapsuleCapsuleType::Graph,
                "invite" => CapsuleCapsuleType::Invite,
                "notification" => CapsuleCapsuleType::Notification,
                "nutrition" => CapsuleCapsuleType::Nutrition,
                "personalRecord" => CapsuleCapsuleType::PersonalRecord,
                "session" => CapsuleCapsuleType::Session,
                "singleMetric" => CapsuleCapsuleType::SingleMetric,
                "today" => CapsuleCapsuleType::Today,
                "unknownCapsuleType" => CapsuleCapsuleType::UnknownCapsuleType,
                "weekly" => CapsuleCapsuleType::Weekly,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::field_selector::FieldSelector for CapsuleCapsuleType {
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
    pub struct Capsule {
        #[serde(rename = "capsuleType", default)]
        pub capsule_type: ::std::option::Option<crate::schemas::CapsuleCapsuleType>,
        #[serde(rename = "cityRank", default)]
        pub city_rank: ::std::option::Option<crate::schemas::NotificationCityRankData>,
        #[serde(rename = "creationTimeMillis", default)]
        #[serde(with = "crate::parsed_string")]
        pub creation_time_millis: ::std::option::Option<i64>,
        #[serde(rename = "id", default)]
        pub id: ::std::option::Option<String>,
        #[serde(rename = "notification", default)]
        pub notification: ::std::option::Option<crate::schemas::NotificationCapsuleConfig>,
        #[serde(rename = "personalRecord", default)]
        pub personal_record: ::std::option::Option<crate::schemas::PersonalRecordCapsuleConfig>,
        #[serde(rename = "singleMetric", default)]
        pub single_metric: ::std::option::Option<crate::schemas::SingleMetricCapsuleConfig>,
    }
    impl ::field_selector::FieldSelector for Capsule {
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
    pub struct ChecksumChunk {
        #[serde(rename = "endMillis", default)]
        #[serde(with = "crate::parsed_string")]
        pub end_millis: ::std::option::Option<i64>,
        #[serde(rename = "startMillis", default)]
        #[serde(with = "crate::parsed_string")]
        pub start_millis: ::std::option::Option<i64>,
        #[serde(rename = "value", default)]
        #[serde(with = "crate::parsed_string")]
        pub value: ::std::option::Option<i64>,
    }
    impl ::field_selector::FieldSelector for ChecksumChunk {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum ChecksumChunkContainerChecksumType {
        AggregateStepCountDelta,
        DatapointCount,
    }
    impl ChecksumChunkContainerChecksumType {
        pub fn as_str(self) -> &'static str {
            match self {
                ChecksumChunkContainerChecksumType::AggregateStepCountDelta => {
                    "aggregateStepCountDelta"
                }
                ChecksumChunkContainerChecksumType::DatapointCount => "datapointCount",
            }
        }
    }
    impl ::std::fmt::Display for ChecksumChunkContainerChecksumType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for ChecksumChunkContainerChecksumType {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for ChecksumChunkContainerChecksumType {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "aggregateStepCountDelta" => {
                    ChecksumChunkContainerChecksumType::AggregateStepCountDelta
                }
                "datapointCount" => ChecksumChunkContainerChecksumType::DatapointCount,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::field_selector::FieldSelector for ChecksumChunkContainerChecksumType {
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
    pub struct ChecksumChunkContainer {
        #[serde(rename = "checksumChunks", default)]
        pub checksum_chunks: ::std::option::Option<Vec<crate::schemas::ChecksumChunk>>,
        #[serde(rename = "checksumType", default)]
        pub checksum_type:
            ::std::option::Option<crate::schemas::ChecksumChunkContainerChecksumType>,
        #[serde(rename = "dataStreamId", default)]
        pub data_stream_id: ::std::option::Option<String>,
        #[serde(rename = "isSubscribed", default)]
        pub is_subscribed: ::std::option::Option<bool>,
    }
    impl ::field_selector::FieldSelector for ChecksumChunkContainer {
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
    pub struct FavoriteActivities {
        #[serde(rename = "favoriteActivity", default)]
        pub favorite_activity: ::std::option::Option<Vec<crate::schemas::FavoriteActivity>>,
        #[doc = "Identifies the source of the favorites data, e.g. \"web\", \"mobile\", \"system_default_v1\", etc."]
        #[serde(rename = "sourceName", default)]
        pub source_name: ::std::option::Option<String>,
        #[serde(rename = "timestampMillis", default)]
        #[serde(with = "crate::parsed_string")]
        pub timestamp_millis: ::std::option::Option<i64>,
        #[serde(rename = "version", default)]
        #[serde(with = "crate::parsed_string")]
        pub version: ::std::option::Option<i64>,
    }
    impl ::field_selector::FieldSelector for FavoriteActivities {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum FavoriteActivityColor {
        First,
        Fourth,
        Second,
        Third,
        UnknownColor,
    }
    impl FavoriteActivityColor {
        pub fn as_str(self) -> &'static str {
            match self {
                FavoriteActivityColor::First => "first",
                FavoriteActivityColor::Fourth => "fourth",
                FavoriteActivityColor::Second => "second",
                FavoriteActivityColor::Third => "third",
                FavoriteActivityColor::UnknownColor => "unknownColor",
            }
        }
    }
    impl ::std::fmt::Display for FavoriteActivityColor {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for FavoriteActivityColor {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for FavoriteActivityColor {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "first" => FavoriteActivityColor::First,
                "fourth" => FavoriteActivityColor::Fourth,
                "second" => FavoriteActivityColor::Second,
                "third" => FavoriteActivityColor::Third,
                "unknownColor" => FavoriteActivityColor::UnknownColor,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::field_selector::FieldSelector for FavoriteActivityColor {
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
    pub struct FavoriteActivity {
        #[doc = "The user selected color associated with the activity."]
        #[serde(rename = "color", default)]
        pub color: ::std::option::Option<crate::schemas::FavoriteActivityColor>,
        #[doc = "The string form of the fitness platform?s activity, e.g. \"walking\", \"badminton\", etc. The list of activities can be found in j/c/g/wireless/android/fitness/constants/Activities.java."]
        #[serde(rename = "fitnessActivity", default)]
        pub fitness_activity: ::std::option::Option<String>,
    }
    impl ::field_selector::FieldSelector for FavoriteActivity {
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
    pub struct GetTimelineDataRequest {
        #[serde(rename = "timeRange", default)]
        pub time_range:
            ::std::option::Option<crate::schemas::GetTimelineDataRequestGetTimeRangeRequest>,
        #[serde(rename = "timeZoneId", default)]
        pub time_zone_id: ::std::option::Option<String>,
        #[serde(rename = "userId", default)]
        pub user_id: ::std::option::Option<String>,
    }
    impl ::field_selector::FieldSelector for GetTimelineDataRequest {
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
    pub struct GetTimelineDataRequestGetTimeRangeRequest {
        #[serde(rename = "endTime", default)]
        #[serde(with = "crate::parsed_string")]
        pub end_time: ::std::option::Option<i64>,
        #[serde(rename = "startTime", default)]
        #[serde(with = "crate::parsed_string")]
        pub start_time: ::std::option::Option<i64>,
    }
    impl ::field_selector::FieldSelector for GetTimelineDataRequestGetTimeRangeRequest {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoalState {
        Cancelled,
        Completed,
        InProgress,
        Planned,
        Unknown,
    }
    impl GoalState {
        pub fn as_str(self) -> &'static str {
            match self {
                GoalState::Cancelled => "cancelled",
                GoalState::Completed => "completed",
                GoalState::InProgress => "inProgress",
                GoalState::Planned => "planned",
                GoalState::Unknown => "unknown",
            }
        }
    }
    impl ::std::fmt::Display for GoalState {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GoalState {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for GoalState {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "cancelled" => GoalState::Cancelled,
                "completed" => GoalState::Completed,
                "inProgress" => GoalState::InProgress,
                "planned" => GoalState::Planned,
                "unknown" => GoalState::Unknown,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::field_selector::FieldSelector for GoalState {
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
    pub struct Goal {
        #[doc = "The criteria by which progress an achievement of this goal will be tracked. If unset, achievement must be marked manually."]
        #[serde(rename = "criteria", default)]
        pub criteria: ::std::option::Option<crate::schemas::GoalAchievementCriteria>,
        #[doc = "Default goal created for the user during onboarding."]
        #[serde(rename = "defaultGoal", default)]
        pub default_goal: ::std::option::Option<bool>,
        #[doc = "Identifies the logic or notification source used to generate the goal. e.g. \"Custom goal...\", \"recommendation:goal:baseline_activity_duration\""]
        #[serde(rename = "description", default)]
        pub description: ::std::option::Option<String>,
        #[doc = "Deprecated. This description should not be surfaced in the UI. Reference to the TransConsole message id for the description."]
        #[serde(rename = "descriptionMsg", default)]
        pub description_msg: ::std::option::Option<crate::schemas::LocalizableMessage>,
        #[doc = "The end or due date of the goal. This may be omitted for open- ended or cyclical goals."]
        #[serde(rename = "endTimeMillis", default)]
        #[serde(with = "crate::parsed_string")]
        pub end_time_millis: ::std::option::Option<i64>,
        #[doc = "A unique identifier within the user?s account. Simply a hash of currentTime, gaiaID, id generation attempt, and toString() of the Goal object."]
        #[serde(rename = "id", default)]
        pub id: ::std::option::Option<String>,
        #[doc = "If this goal was created as a result of a longer term goal, we reference the parent goal here."]
        #[serde(rename = "parentGoalId", default)]
        #[serde(with = "crate::parsed_string")]
        pub parent_goal_id: ::std::option::Option<i64>,
        #[doc = "Most goals are effective immediately so start time is effectively the creation time but this can be used to schedule a more discrete goal like a spinning workout for some time in the future."]
        #[serde(rename = "startTimeMillis", default)]
        #[serde(with = "crate::parsed_string")]
        pub start_time_millis: ::std::option::Option<i64>,
        #[serde(rename = "state", default)]
        pub state: ::std::option::Option<crate::schemas::GoalState>,
        #[doc = "Reference to a global template for this goal type (e.g. ?take X steps per day?) which has the localizable message format as well as any preset field value (e.g. DAILY for achievement period in the example above)."]
        #[serde(rename = "templateId", default)]
        pub template_id: ::std::option::Option<String>,
        #[doc = "The period of time over which the goal?s achievement criteria is measured and after which is the goal?s progress is reset."]
        #[serde(rename = "timePeriod", default)]
        pub time_period: ::std::option::Option<crate::schemas::GoalTimePeriod>,
    }
    impl ::field_selector::FieldSelector for Goal {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoalAchievementCriteriaType {
        Cumulative,
        Target,
        Trend,
        Unknown,
    }
    impl GoalAchievementCriteriaType {
        pub fn as_str(self) -> &'static str {
            match self {
                GoalAchievementCriteriaType::Cumulative => "cumulative",
                GoalAchievementCriteriaType::Target => "target",
                GoalAchievementCriteriaType::Trend => "trend",
                GoalAchievementCriteriaType::Unknown => "unknown",
            }
        }
    }
    impl ::std::fmt::Display for GoalAchievementCriteriaType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GoalAchievementCriteriaType {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for GoalAchievementCriteriaType {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "cumulative" => GoalAchievementCriteriaType::Cumulative,
                "target" => GoalAchievementCriteriaType::Target,
                "trend" => GoalAchievementCriteriaType::Trend,
                "unknown" => GoalAchievementCriteriaType::Unknown,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::field_selector::FieldSelector for GoalAchievementCriteriaType {
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
    pub struct GoalAchievementCriteria {
        #[doc = "The xSensor name of the property to be tracked (e.g. ?com.google.step_count?)"]
        #[serde(rename = "dataType", default)]
        pub data_type: ::std::option::Option<crate::schemas::DataType>,
        #[doc = "Upper and lower bounds for the property values. Either may be unset depending on the criteria but at least one must be set."]
        #[serde(rename = "maxValue", default)]
        pub max_value: ::std::option::Option<f64>,
        #[serde(rename = "minValue", default)]
        pub min_value: ::std::option::Option<f64>,
        #[serde(rename = "type", default)]
        pub r#type: ::std::option::Option<crate::schemas::GoalAchievementCriteriaType>,
        #[serde(rename = "trendPeriod", default)]
        pub trend_period: ::std::option::Option<crate::schemas::GoalTimePeriod>,
    }
    impl ::field_selector::FieldSelector for GoalAchievementCriteria {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoalTimePeriodUnit {
        Day,
        Hour,
        Minute,
        Month,
        Second,
        Unknown,
        Week,
        Year,
    }
    impl GoalTimePeriodUnit {
        pub fn as_str(self) -> &'static str {
            match self {
                GoalTimePeriodUnit::Day => "day",
                GoalTimePeriodUnit::Hour => "hour",
                GoalTimePeriodUnit::Minute => "minute",
                GoalTimePeriodUnit::Month => "month",
                GoalTimePeriodUnit::Second => "second",
                GoalTimePeriodUnit::Unknown => "unknown",
                GoalTimePeriodUnit::Week => "week",
                GoalTimePeriodUnit::Year => "year",
            }
        }
    }
    impl ::std::fmt::Display for GoalTimePeriodUnit {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GoalTimePeriodUnit {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for GoalTimePeriodUnit {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "day" => GoalTimePeriodUnit::Day,
                "hour" => GoalTimePeriodUnit::Hour,
                "minute" => GoalTimePeriodUnit::Minute,
                "month" => GoalTimePeriodUnit::Month,
                "second" => GoalTimePeriodUnit::Second,
                "unknown" => GoalTimePeriodUnit::Unknown,
                "week" => GoalTimePeriodUnit::Week,
                "year" => GoalTimePeriodUnit::Year,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::field_selector::FieldSelector for GoalTimePeriodUnit {
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
    pub struct GoalTimePeriod {
        #[serde(rename = "count", default)]
        pub count: ::std::option::Option<i32>,
        #[serde(rename = "unit", default)]
        pub unit: ::std::option::Option<crate::schemas::GoalTimePeriodUnit>,
    }
    impl ::field_selector::FieldSelector for GoalTimePeriod {
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
    pub struct GoalV2 {
        #[doc = "The goal objective, depending on which objective type is present, dictates how progress is calculated and which fields are applicable. Exactly one of these are required."]
        #[serde(rename = "cumulativeObjective", default)]
        pub cumulative_objective: ::std::option::Option<crate::schemas::GoalV2CumulativeObjective>,
        #[doc = "The metric that the goal tracks."]
        #[serde(rename = "dataTypeName", default)]
        pub data_type_name: ::std::option::Option<String>,
        #[serde(rename = "endTimeNanos", default)]
        #[serde(with = "crate::parsed_string")]
        pub end_time_nanos: ::std::option::Option<i64>,
        #[doc = "List of criteria that the data must meet in order to qualify for counting towards the goal."]
        #[serde(rename = "filter", default)]
        pub filter: ::std::option::Option<Vec<crate::schemas::GoalV2Criteria>>,
        #[doc = "If this is a recurring goal, amount of time user has to achieve the goal before goal is reset and repeated. Unset if not a recurring goal. Optional."]
        #[serde(rename = "recurInterval", default)]
        pub recur_interval: ::std::option::Option<crate::schemas::GoalV2TimePeriod>,
        #[serde(rename = "segmentObjective", default)]
        pub segment_objective: ::std::option::Option<crate::schemas::GoalV2SegmentObjective>,
        #[doc = "The start and end time represent the time at which the goal started to be active or inactive, but this goal may still be edited later on. The timestamps in the data stream represent goal edit timestamps, which will NOT be equal to the start and end time if the goal has been edited. For this reason, the key may not be unique within the data stream."]
        #[serde(rename = "startTimeNanos", default)]
        #[serde(with = "crate::parsed_string")]
        pub start_time_nanos: ::std::option::Option<i64>,
        #[doc = "The stream name under which this datapoint is stored."]
        #[serde(rename = "streamId", default)]
        pub stream_id: ::std::option::Option<String>,
        #[serde(rename = "targetObjective", default)]
        pub target_objective: ::std::option::Option<crate::schemas::GoalV2TargetObjective>,
    }
    impl ::field_selector::FieldSelector for GoalV2 {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoalV2CriteriaOperator {
        Eq,
        Gte,
        Lte,
        UnknownOperator,
    }
    impl GoalV2CriteriaOperator {
        pub fn as_str(self) -> &'static str {
            match self {
                GoalV2CriteriaOperator::Eq => "eq",
                GoalV2CriteriaOperator::Gte => "gte",
                GoalV2CriteriaOperator::Lte => "lte",
                GoalV2CriteriaOperator::UnknownOperator => "unknownOperator",
            }
        }
    }
    impl ::std::fmt::Display for GoalV2CriteriaOperator {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GoalV2CriteriaOperator {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for GoalV2CriteriaOperator {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "eq" => GoalV2CriteriaOperator::Eq,
                "gte" => GoalV2CriteriaOperator::Gte,
                "lte" => GoalV2CriteriaOperator::Lte,
                "unknownOperator" => GoalV2CriteriaOperator::UnknownOperator,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::field_selector::FieldSelector for GoalV2CriteriaOperator {
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
    pub struct GoalV2Criteria {
        #[doc = "A field name in the given DataType, OR special values ?duration? and ?activity?, which will be extracted from segment. Required."]
        #[serde(rename = "fieldName", default)]
        pub field_name: ::std::option::Option<String>,
        #[doc = "The comparison operator to use on the value, when determining whether the criteria is met. Required."]
        #[serde(rename = "operator", default)]
        pub operator: ::std::option::Option<crate::schemas::GoalV2CriteriaOperator>,
        #[doc = "The value of the field, to use when determining whether the criteria is met. Required."]
        #[serde(rename = "value", default)]
        pub value: ::std::option::Option<crate::schemas::Value>,
    }
    impl ::field_selector::FieldSelector for GoalV2Criteria {
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
    pub struct GoalV2CumulativeObjective {
        #[doc = "Required."]
        #[serde(rename = "objective", default)]
        pub objective: ::std::option::Option<crate::schemas::GoalV2Criteria>,
    }
    impl ::field_selector::FieldSelector for GoalV2CumulativeObjective {
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
    pub struct GoalV2SegmentObjective {
        #[doc = "Number of segments that pass the filter criteria list."]
        #[serde(rename = "frequency", default)]
        pub frequency: ::std::option::Option<i32>,
    }
    impl ::field_selector::FieldSelector for GoalV2SegmentObjective {
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
    pub struct GoalV2TargetObjective {
        #[doc = "Value at the start time of the goal, which we evaluate towards the objective value."]
        #[serde(rename = "initialvalue", default)]
        pub initialvalue: ::std::option::Option<crate::schemas::Value>,
        #[doc = "Required."]
        #[serde(rename = "objective", default)]
        pub objective: ::std::option::Option<crate::schemas::GoalV2Criteria>,
    }
    impl ::field_selector::FieldSelector for GoalV2TargetObjective {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoalV2TimePeriodUnit {
        Day,
        Month,
        UnknownUnit,
        Week,
        Year,
    }
    impl GoalV2TimePeriodUnit {
        pub fn as_str(self) -> &'static str {
            match self {
                GoalV2TimePeriodUnit::Day => "day",
                GoalV2TimePeriodUnit::Month => "month",
                GoalV2TimePeriodUnit::UnknownUnit => "unknownUnit",
                GoalV2TimePeriodUnit::Week => "week",
                GoalV2TimePeriodUnit::Year => "year",
            }
        }
    }
    impl ::std::fmt::Display for GoalV2TimePeriodUnit {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GoalV2TimePeriodUnit {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for GoalV2TimePeriodUnit {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "day" => GoalV2TimePeriodUnit::Day,
                "month" => GoalV2TimePeriodUnit::Month,
                "unknownUnit" => GoalV2TimePeriodUnit::UnknownUnit,
                "week" => GoalV2TimePeriodUnit::Week,
                "year" => GoalV2TimePeriodUnit::Year,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::field_selector::FieldSelector for GoalV2TimePeriodUnit {
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
    pub struct GoalV2TimePeriod {
        #[serde(rename = "count", default)]
        pub count: ::std::option::Option<f64>,
        #[doc = "Required."]
        #[serde(rename = "unit", default)]
        pub unit: ::std::option::Option<crate::schemas::GoalV2TimePeriodUnit>,
    }
    impl ::field_selector::FieldSelector for GoalV2TimePeriod {
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
    pub struct ListCapsulesResponse {
        #[serde(rename = "capsule", default)]
        pub capsule: ::std::option::Option<Vec<crate::schemas::Capsule>>,
    }
    impl ::field_selector::FieldSelector for ListCapsulesResponse {
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
    pub struct ListGoalsResponse {
        #[serde(rename = "goals", default)]
        pub goals: ::std::option::Option<Vec<crate::schemas::Goal>>,
    }
    impl ::field_selector::FieldSelector for ListGoalsResponse {
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
    pub struct ListGoalsV2Response {
        #[serde(rename = "goals", default)]
        pub goals: ::std::option::Option<Vec<crate::schemas::GoalV2>>,
    }
    impl ::field_selector::FieldSelector for ListGoalsV2Response {
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
    pub struct ListNotificationsResponse {
        #[serde(rename = "notifications", default)]
        pub notifications: ::std::option::Option<Vec<crate::schemas::Notification>>,
    }
    impl ::field_selector::FieldSelector for ListNotificationsResponse {
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
    pub struct ListSessionsResponse {
        #[serde(rename = "session", default)]
        pub session: ::std::option::Option<Vec<crate::schemas::TimelineSession>>,
    }
    impl ::field_selector::FieldSelector for ListSessionsResponse {
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
    pub struct LocalizableMessage {
        #[serde(rename = "id", default)]
        pub id: ::std::option::Option<String>,
        #[serde(rename = "placeholder", default)]
        pub placeholder: ::std::option::Option<Vec<crate::schemas::LocalizableMessagePlaceholder>>,
    }
    impl ::field_selector::FieldSelector for LocalizableMessage {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum LocalizableMessagePlaceholderLengthUnit {
        Imperial,
        Metric,
        UnknownLengthUnit,
    }
    impl LocalizableMessagePlaceholderLengthUnit {
        pub fn as_str(self) -> &'static str {
            match self {
                LocalizableMessagePlaceholderLengthUnit::Imperial => "imperial",
                LocalizableMessagePlaceholderLengthUnit::Metric => "metric",
                LocalizableMessagePlaceholderLengthUnit::UnknownLengthUnit => "unknownLengthUnit",
            }
        }
    }
    impl ::std::fmt::Display for LocalizableMessagePlaceholderLengthUnit {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for LocalizableMessagePlaceholderLengthUnit {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for LocalizableMessagePlaceholderLengthUnit {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "imperial" => LocalizableMessagePlaceholderLengthUnit::Imperial,
                "metric" => LocalizableMessagePlaceholderLengthUnit::Metric,
                "unknownLengthUnit" => LocalizableMessagePlaceholderLengthUnit::UnknownLengthUnit,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::field_selector::FieldSelector for LocalizableMessagePlaceholderLengthUnit {
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
    pub struct LocalizableMessagePlaceholder {
        #[doc = "Distances formatted as \"4.23 km\" or \"4.23 miles\". Note to clients: Always set the distance in meters and specify the length unit. If there is no length unit pref, the default for the user's locale will be chosen."]
        #[serde(rename = "distanceMeters", default)]
        pub distance_meters: ::std::option::Option<f32>,
        #[serde(rename = "distanceMiles", default)]
        pub distance_miles: ::std::option::Option<f32>,
        #[doc = "This will be formatted as a short duration like \"1 hr 13 min\""]
        #[serde(rename = "durationMillis", default)]
        #[serde(with = "crate::parsed_string")]
        pub duration_millis: ::std::option::Option<i64>,
        #[serde(rename = "fpVal", default)]
        pub fp_val: ::std::option::Option<f32>,
        #[doc = "Value of placeholder."]
        #[serde(rename = "intVal", default)]
        #[serde(with = "crate::parsed_string")]
        pub int_val: ::std::option::Option<i64>,
        #[serde(rename = "lengthUnit", default)]
        pub length_unit:
            ::std::option::Option<crate::schemas::LocalizableMessagePlaceholderLengthUnit>,
        #[doc = "Name of placeholder for messages like \"Take {STEPCOUNT} steps\" where \"STEPCOUNT\" is the name."]
        #[serde(rename = "name", default)]
        pub name: ::std::option::Option<String>,
        #[serde(rename = "strVal", default)]
        pub str_val: ::std::option::Option<String>,
        #[doc = "This will be formatted as a short relative date eg. \"today\", \"Wed Aug 26\""]
        #[serde(rename = "timeMillis", default)]
        #[serde(with = "crate::parsed_string")]
        pub time_millis: ::std::option::Option<i64>,
    }
    impl ::field_selector::FieldSelector for LocalizableMessagePlaceholder {
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
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum NotificationViewEnum {
        AlertCards,
        ChallengeCompletedCard,
        ChallengePendingCard,
        CityCard,
        CreateGoalCard,
        DialogAccountSwitch,
        DialogTurningArOn,
        EmptyStateCard,
        EmptyStateGoalsNoActivity,
        EmptyStateNoGoals,
        FeatureHighlightActiveMode,
        FeatureHighlightTimeline,
        GotoTimelineCard,
        GroupInviteCard,
        GroupsCard,
        LoadingCard,
        LocalNotificationActivityDetection,
        LocalNotificationDataCollectionOff,
        LocalNotificationDisengaged,
        LocalNotificationEngagement,
        LocalNotificationHighAccuracy,
        LocalNotificationLocationHistory,
        LocalNotificationLocationOff,
        LocalNotificationLocationPermissionOff,
        LocalNotificationMultipleIssues,
        LocalNotificationSyncOff,
        MultiGoalCard,
        NmoreSessionsCard,
        NotificationAchievementBike,
        NotificationAchievementRun,
        NotificationCard,
        NotificationDefaultActivityGoal,
        NotificationInsightRankingCity,
        NotificationInsightUserDailyNutrition,
        NotificationInsightUserTrendsMonthly,
        NotificationInsightUserTrendsUpDaily,
        NotificationPersonalRecordIntroduction,
        NotificationSuggestionActivityMeetGoal,
        NotificationSuggestionGoalBaseline,
        NotificationSuggestionGoalIncrease,
        NotificationSystemEstimatedDistance,
        NotificationSystemM8Incompatible,
        NotificationSystemSurvey,
        NutritionCard,
        OnboardingBikeCarryPositionsCard,
        OnboardingConnectedAppsCard,
        OnboardingWalkCarryPositionsCard,
        PersonalRecordCard,
        PersonalRecordIntroCard,
        ProfileCompletionCard,
        SectionCard,
        SessionCard,
        SessionFromLocationCard,
        SettingsApiCard,
        SharingEducation,
        SingleGoalCard,
        SleepTrackingCard,
        SummaryBarCard,
        SyncProgressCard,
        SysNotificationEngagement,
        UnknownView,
        WeightTrackingCard,
        WorkoutSummaryMapView,
    }
    impl NotificationViewEnum {
        pub fn as_str(self) -> &'static str {
            match self {
                NotificationViewEnum::AlertCards => "alertCards",
                NotificationViewEnum::ChallengeCompletedCard => "challengeCompletedCard",
                NotificationViewEnum::ChallengePendingCard => "challengePendingCard",
                NotificationViewEnum::CityCard => "cityCard",
                NotificationViewEnum::CreateGoalCard => "createGoalCard",
                NotificationViewEnum::DialogAccountSwitch => "dialogAccountSwitch",
                NotificationViewEnum::DialogTurningArOn => "dialogTurningArOn",
                NotificationViewEnum::EmptyStateCard => "emptyStateCard",
                NotificationViewEnum::EmptyStateGoalsNoActivity => "emptyStateGoalsNoActivity",
                NotificationViewEnum::EmptyStateNoGoals => "emptyStateNoGoals",
                NotificationViewEnum::FeatureHighlightActiveMode => "featureHighlightActiveMode",
                NotificationViewEnum::FeatureHighlightTimeline => "featureHighlightTimeline",
                NotificationViewEnum::GotoTimelineCard => "gotoTimelineCard",
                NotificationViewEnum::GroupInviteCard => "groupInviteCard",
                NotificationViewEnum::GroupsCard => "groupsCard",
                NotificationViewEnum::LoadingCard => "loadingCard",
                NotificationViewEnum::LocalNotificationActivityDetection => {
                    "localNotificationActivityDetection"
                }
                NotificationViewEnum::LocalNotificationDataCollectionOff => {
                    "localNotificationDataCollectionOff"
                }
                NotificationViewEnum::LocalNotificationDisengaged => "localNotificationDisengaged",
                NotificationViewEnum::LocalNotificationEngagement => "localNotificationEngagement",
                NotificationViewEnum::LocalNotificationHighAccuracy => {
                    "localNotificationHighAccuracy"
                }
                NotificationViewEnum::LocalNotificationLocationHistory => {
                    "localNotificationLocationHistory"
                }
                NotificationViewEnum::LocalNotificationLocationOff => {
                    "localNotificationLocationOff"
                }
                NotificationViewEnum::LocalNotificationLocationPermissionOff => {
                    "localNotificationLocationPermissionOff"
                }
                NotificationViewEnum::LocalNotificationMultipleIssues => {
                    "localNotificationMultipleIssues"
                }
                NotificationViewEnum::LocalNotificationSyncOff => "localNotificationSyncOff",
                NotificationViewEnum::MultiGoalCard => "multiGoalCard",
                NotificationViewEnum::NmoreSessionsCard => "nmoreSessionsCard",
                NotificationViewEnum::NotificationAchievementBike => "notificationAchievementBike",
                NotificationViewEnum::NotificationAchievementRun => "notificationAchievementRun",
                NotificationViewEnum::NotificationCard => "notificationCard",
                NotificationViewEnum::NotificationDefaultActivityGoal => {
                    "notificationDefaultActivityGoal"
                }
                NotificationViewEnum::NotificationInsightRankingCity => {
                    "notificationInsightRankingCity"
                }
                NotificationViewEnum::NotificationInsightUserDailyNutrition => {
                    "notificationInsightUserDailyNutrition"
                }
                NotificationViewEnum::NotificationInsightUserTrendsMonthly => {
                    "notificationInsightUserTrendsMonthly"
                }
                NotificationViewEnum::NotificationInsightUserTrendsUpDaily => {
                    "notificationInsightUserTrendsUpDaily"
                }
                NotificationViewEnum::NotificationPersonalRecordIntroduction => {
                    "notificationPersonalRecordIntroduction"
                }
                NotificationViewEnum::NotificationSuggestionActivityMeetGoal => {
                    "notificationSuggestionActivityMeetGoal"
                }
                NotificationViewEnum::NotificationSuggestionGoalBaseline => {
                    "notificationSuggestionGoalBaseline"
                }
                NotificationViewEnum::NotificationSuggestionGoalIncrease => {
                    "notificationSuggestionGoalIncrease"
                }
                NotificationViewEnum::NotificationSystemEstimatedDistance => {
                    "notificationSystemEstimatedDistance"
                }
                NotificationViewEnum::NotificationSystemM8Incompatible => {
                    "notificationSystemM8Incompatible"
                }
                NotificationViewEnum::NotificationSystemSurvey => "notificationSystemSurvey",
                NotificationViewEnum::NutritionCard => "nutritionCard",
                NotificationViewEnum::OnboardingBikeCarryPositionsCard => {
                    "onboardingBikeCarryPositionsCard"
                }
                NotificationViewEnum::OnboardingConnectedAppsCard => "onboardingConnectedAppsCard",
                NotificationViewEnum::OnboardingWalkCarryPositionsCard => {
                    "onboardingWalkCarryPositionsCard"
                }
                NotificationViewEnum::PersonalRecordCard => "personalRecordCard",
                NotificationViewEnum::PersonalRecordIntroCard => "personalRecordIntroCard",
                NotificationViewEnum::ProfileCompletionCard => "profileCompletionCard",
                NotificationViewEnum::SectionCard => "sectionCard",
                NotificationViewEnum::SessionCard => "sessionCard",
                NotificationViewEnum::SessionFromLocationCard => "sessionFromLocationCard",
                NotificationViewEnum::SettingsApiCard => "settingsApiCard",
                NotificationViewEnum::SharingEducation => "sharingEducation",
                NotificationViewEnum::SingleGoalCard => "singleGoalCard",
                NotificationViewEnum::SleepTrackingCard => "sleepTrackingCard",
                NotificationViewEnum::SummaryBarCard => "summaryBarCard",
                NotificationViewEnum::SyncProgressCard => "syncProgressCard",
                NotificationViewEnum::SysNotificationEngagement => "sysNotificationEngagement",
                NotificationViewEnum::UnknownView => "unknownView",
                NotificationViewEnum::WeightTrackingCard => "weightTrackingCard",
                NotificationViewEnum::WorkoutSummaryMapView => "workoutSummaryMapView",
            }
        }
    }
    impl ::std::fmt::Display for NotificationViewEnum {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for NotificationViewEnum {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for NotificationViewEnum {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "alertCards" => NotificationViewEnum::AlertCards,
                "challengeCompletedCard" => NotificationViewEnum::ChallengeCompletedCard,
                "challengePendingCard" => NotificationViewEnum::ChallengePendingCard,
                "cityCard" => NotificationViewEnum::CityCard,
                "createGoalCard" => NotificationViewEnum::CreateGoalCard,
                "dialogAccountSwitch" => NotificationViewEnum::DialogAccountSwitch,
                "dialogTurningArOn" => NotificationViewEnum::DialogTurningArOn,
                "emptyStateCard" => NotificationViewEnum::EmptyStateCard,
                "emptyStateGoalsNoActivity" => NotificationViewEnum::EmptyStateGoalsNoActivity,
                "emptyStateNoGoals" => NotificationViewEnum::EmptyStateNoGoals,
                "featureHighlightActiveMode" => NotificationViewEnum::FeatureHighlightActiveMode,
                "featureHighlightTimeline" => NotificationViewEnum::FeatureHighlightTimeline,
                "gotoTimelineCard" => NotificationViewEnum::GotoTimelineCard,
                "groupInviteCard" => NotificationViewEnum::GroupInviteCard,
                "groupsCard" => NotificationViewEnum::GroupsCard,
                "loadingCard" => NotificationViewEnum::LoadingCard,
                "localNotificationActivityDetection" => {
                    NotificationViewEnum::LocalNotificationActivityDetection
                }
                "localNotificationDataCollectionOff" => {
                    NotificationViewEnum::LocalNotificationDataCollectionOff
                }
                "localNotificationDisengaged" => NotificationViewEnum::LocalNotificationDisengaged,
                "localNotificationEngagement" => NotificationViewEnum::LocalNotificationEngagement,
                "localNotificationHighAccuracy" => {
                    NotificationViewEnum::LocalNotificationHighAccuracy
                }
                "localNotificationLocationHistory" => {
                    NotificationViewEnum::LocalNotificationLocationHistory
                }
                "localNotificationLocationOff" => {
                    NotificationViewEnum::LocalNotificationLocationOff
                }
                "localNotificationLocationPermissionOff" => {
                    NotificationViewEnum::LocalNotificationLocationPermissionOff
                }
                "localNotificationMultipleIssues" => {
                    NotificationViewEnum::LocalNotificationMultipleIssues
                }
                "localNotificationSyncOff" => NotificationViewEnum::LocalNotificationSyncOff,
                "multiGoalCard" => NotificationViewEnum::MultiGoalCard,
                "nmoreSessionsCard" => NotificationViewEnum::NmoreSessionsCard,
                "notificationAchievementBike" => NotificationViewEnum::NotificationAchievementBike,
                "notificationAchievementRun" => NotificationViewEnum::NotificationAchievementRun,
                "notificationCard" => NotificationViewEnum::NotificationCard,
                "notificationDefaultActivityGoal" => {
                    NotificationViewEnum::NotificationDefaultActivityGoal
                }
                "notificationInsightRankingCity" => {
                    NotificationViewEnum::NotificationInsightRankingCity
                }
                "notificationInsightUserDailyNutrition" => {
                    NotificationViewEnum::NotificationInsightUserDailyNutrition
                }
                "notificationInsightUserTrendsMonthly" => {
                    NotificationViewEnum::NotificationInsightUserTrendsMonthly
                }
                "notificationInsightUserTrendsUpDaily" => {
                    NotificationViewEnum::NotificationInsightUserTrendsUpDaily
                }
                "notificationPersonalRecordIntroduction" => {
                    NotificationViewEnum::NotificationPersonalRecordIntroduction
                }
                "notificationSuggestionActivityMeetGoal" => {
                    NotificationViewEnum::NotificationSuggestionActivityMeetGoal
                }
                "notificationSuggestionGoalBaseline" => {
                    NotificationViewEnum::NotificationSuggestionGoalBaseline
                }
                "notificationSuggestionGoalIncrease" => {
                    NotificationViewEnum::NotificationSuggestionGoalIncrease
                }
                "notificationSystemEstimatedDistance" => {
                    NotificationViewEnum::NotificationSystemEstimatedDistance
                }
                "notificationSystemM8Incompatible" => {
                    NotificationViewEnum::NotificationSystemM8Incompatible
                }
                "notificationSystemSurvey" => NotificationViewEnum::NotificationSystemSurvey,
                "nutritionCard" => NotificationViewEnum::NutritionCard,
                "onboardingBikeCarryPositionsCard" => {
                    NotificationViewEnum::OnboardingBikeCarryPositionsCard
                }
                "onboardingConnectedAppsCard" => NotificationViewEnum::OnboardingConnectedAppsCard,
                "onboardingWalkCarryPositionsCard" => {
                    NotificationViewEnum::OnboardingWalkCarryPositionsCard
                }
                "personalRecordCard" => NotificationViewEnum::PersonalRecordCard,
                "personalRecordIntroCard" => NotificationViewEnum::PersonalRecordIntroCard,
                "profileCompletionCard" => NotificationViewEnum::ProfileCompletionCard,
                "sectionCard" => NotificationViewEnum::SectionCard,
                "sessionCard" => NotificationViewEnum::SessionCard,
                "sessionFromLocationCard" => NotificationViewEnum::SessionFromLocationCard,
                "settingsApiCard" => NotificationViewEnum::SettingsApiCard,
                "sharingEducation" => NotificationViewEnum::SharingEducation,
                "singleGoalCard" => NotificationViewEnum::SingleGoalCard,
                "sleepTrackingCard" => NotificationViewEnum::SleepTrackingCard,
                "summaryBarCard" => NotificationViewEnum::SummaryBarCard,
                "syncProgressCard" => NotificationViewEnum::SyncProgressCard,
                "sysNotificationEngagement" => NotificationViewEnum::SysNotificationEngagement,
                "unknownView" => NotificationViewEnum::UnknownView,
                "weightTrackingCard" => NotificationViewEnum::WeightTrackingCard,
                "workoutSummaryMapView" => NotificationViewEnum::WorkoutSummaryMapView,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::field_selector::FieldSelector for NotificationViewEnum {
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
    pub struct Notification {
        #[serde(rename = "action", default)]
        pub action: ::std::option::Option<Vec<crate::schemas::NotificationNotificationAction>>,
        #[serde(rename = "blockable", default)]
        pub blockable: ::std::option::Option<bool>,
        #[serde(rename = "cityRankData", default)]
        pub city_rank_data: ::std::option::Option<crate::schemas::NotificationCityRankData>,
        #[serde(rename = "condition", default)]
        pub condition: ::std::option::Option<crate::schemas::NotificationTriggerCondition>,
        #[doc = "Timestamp when this notification was generated."]
        #[serde(rename = "creationTimeMillis", default)]
        #[serde(with = "crate::parsed_string")]
        pub creation_time_millis: ::std::option::Option<i64>,
        #[doc = "Date/time after which the clients should no longer display this notification."]
        #[serde(rename = "expirationTimeMillis", default)]
        #[serde(with = "crate::parsed_string")]
        pub expiration_time_millis: ::std::option::Option<i64>,
        #[doc = "A unique identifier for capturing impression and response for this notification. Simply a hash of currentTime, gaia id, id generation attempt, and toString() of the Notification object."]
        #[serde(rename = "id", default)]
        pub id: ::std::option::Option<String>,
        #[serde(rename = "increaseGoalV2Data", default)]
        pub increase_goal_v2_data:
            ::std::option::Option<crate::schemas::NotificationIncreaseGoalV2Data>,
        #[doc = "If this notification was generated to support an existing goal we?ll capture that goal?s ID here."]
        #[serde(rename = "parentGoalId", default)]
        #[serde(with = "crate::parsed_string")]
        pub parent_goal_id: ::std::option::Option<i64>,
        #[serde(rename = "personalRecordData", default)]
        pub personal_record_data:
            ::std::option::Option<crate::schemas::NotificationPersonalRecordData>,
        #[serde(rename = "personalRecordIntroData", default)]
        pub personal_record_intro_data:
            ::std::option::Option<crate::schemas::NotificationPersonalRecordIntroData>,
        #[serde(rename = "position", default)]
        pub position: ::std::option::Option<crate::schemas::NotificationTimelinePosition>,
        #[serde(rename = "sourceDescription", default)]
        pub source_description: ::std::option::Option<String>,
        #[doc = "Identifies the logic used to generate the notification -- possibly with version."]
        #[serde(rename = "sourceName", default)]
        pub source_name: ::std::option::Option<String>,
        #[doc = "The source title/description are used in the blocked notifications UI."]
        #[serde(rename = "sourceTitle", default)]
        pub source_title: ::std::option::Option<String>,
        #[serde(rename = "uiModel", default)]
        pub ui_model: ::std::option::Option<Vec<crate::schemas::NotificationUiModel>>,
        #[serde(rename = "viewEnum", default)]
        pub view_enum: ::std::option::Option<crate::schemas::NotificationViewEnum>,
    }
    impl ::field_selector::FieldSelector for Notification {
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
    pub struct NotificationCapsuleConfig {
        #[serde(rename = "notification", default)]
        pub notification: ::std::option::Option<crate::schemas::Notification>,
    }
    impl ::field_selector::FieldSelector for NotificationCapsuleConfig {
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
    pub struct NotificationCityRankData {
        #[serde(rename = "activeTimeMillis", default)]
        #[serde(with = "crate::parsed_string")]
        pub active_time_millis: ::std::option::Option<i64>,
        #[serde(rename = "city", default)]
        pub city: ::std::option::Option<String>,
        #[doc = "Percentile expressed as a number between 0 and 100."]
        #[serde(rename = "percentile", default)]
        pub percentile: ::std::option::Option<f64>,
        #[serde(rename = "population", default)]
        #[serde(with = "crate::parsed_string")]
        pub population: ::std::option::Option<i64>,
        #[serde(rename = "steps", default)]
        pub steps: ::std::option::Option<f64>,
    }
    impl ::field_selector::FieldSelector for NotificationCityRankData {
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
    pub struct NotificationIncreaseGoalV2Data {
        #[serde(rename = "currentGoal", default)]
        pub current_goal: ::std::option::Option<crate::schemas::GoalV2>,
        #[serde(rename = "recommendedGoal", default)]
        pub recommended_goal: ::std::option::Option<Vec<crate::schemas::GoalV2>>,
        #[serde(rename = "recommendedValue", default)]
        pub recommended_value: ::std::option::Option<Vec<i64>>,
    }
    impl ::field_selector::FieldSelector for NotificationIncreaseGoalV2Data {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum NotificationNotificationActionAction {
        Accepted,
        Blocked,
        Dismissed,
        Displayed,
    }
    impl NotificationNotificationActionAction {
        pub fn as_str(self) -> &'static str {
            match self {
                NotificationNotificationActionAction::Accepted => "accepted",
                NotificationNotificationActionAction::Blocked => "blocked",
                NotificationNotificationActionAction::Dismissed => "dismissed",
                NotificationNotificationActionAction::Displayed => "displayed",
            }
        }
    }
    impl ::std::fmt::Display for NotificationNotificationActionAction {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for NotificationNotificationActionAction {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for NotificationNotificationActionAction {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "accepted" => NotificationNotificationActionAction::Accepted,
                "blocked" => NotificationNotificationActionAction::Blocked,
                "dismissed" => NotificationNotificationActionAction::Dismissed,
                "displayed" => NotificationNotificationActionAction::Displayed,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::field_selector::FieldSelector for NotificationNotificationActionAction {
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
    pub struct NotificationNotificationAction {
        #[serde(rename = "action", default)]
        pub action: ::std::option::Option<crate::schemas::NotificationNotificationActionAction>,
        #[serde(rename = "actionTimeMillis", default)]
        #[serde(with = "crate::parsed_string")]
        pub action_time_millis: ::std::option::Option<i64>,
    }
    impl ::field_selector::FieldSelector for NotificationNotificationAction {
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
    pub struct NotificationPersonalRecordData {
        #[serde(rename = "previousValue", default)]
        pub previous_value: ::std::option::Option<f64>,
        #[serde(rename = "sessionStartMillis", default)]
        #[serde(with = "crate::parsed_string")]
        pub session_start_millis: ::std::option::Option<i64>,
        #[doc = "In millis for time and meters for distance."]
        #[serde(rename = "value", default)]
        pub value: ::std::option::Option<f64>,
    }
    impl ::field_selector::FieldSelector for NotificationPersonalRecordData {
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
    pub struct NotificationPersonalRecordIntroData {
        #[serde(rename = "bikeDistance", default)]
        pub bike_distance: ::std::option::Option<crate::schemas::NotificationPersonalRecordData>,
        #[serde(rename = "bikeDuration", default)]
        pub bike_duration: ::std::option::Option<crate::schemas::NotificationPersonalRecordData>,
        #[serde(rename = "runDistance", default)]
        pub run_distance: ::std::option::Option<crate::schemas::NotificationPersonalRecordData>,
        #[serde(rename = "runDuration", default)]
        pub run_duration: ::std::option::Option<crate::schemas::NotificationPersonalRecordData>,
    }
    impl ::field_selector::FieldSelector for NotificationPersonalRecordIntroData {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum NotificationStatusStatus {
        Accepted,
        Blocked,
        DeprecatedComplete,
        Dismissed,
        LocalOnly,
        NoStatus,
        SystemCancel,
        Visible,
    }
    impl NotificationStatusStatus {
        pub fn as_str(self) -> &'static str {
            match self {
                NotificationStatusStatus::Accepted => "accepted",
                NotificationStatusStatus::Blocked => "blocked",
                NotificationStatusStatus::DeprecatedComplete => "deprecatedComplete",
                NotificationStatusStatus::Dismissed => "dismissed",
                NotificationStatusStatus::LocalOnly => "localOnly",
                NotificationStatusStatus::NoStatus => "noStatus",
                NotificationStatusStatus::SystemCancel => "systemCancel",
                NotificationStatusStatus::Visible => "visible",
            }
        }
    }
    impl ::std::fmt::Display for NotificationStatusStatus {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for NotificationStatusStatus {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for NotificationStatusStatus {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "accepted" => NotificationStatusStatus::Accepted,
                "blocked" => NotificationStatusStatus::Blocked,
                "deprecatedComplete" => NotificationStatusStatus::DeprecatedComplete,
                "dismissed" => NotificationStatusStatus::Dismissed,
                "localOnly" => NotificationStatusStatus::LocalOnly,
                "noStatus" => NotificationStatusStatus::NoStatus,
                "systemCancel" => NotificationStatusStatus::SystemCancel,
                "visible" => NotificationStatusStatus::Visible,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::field_selector::FieldSelector for NotificationStatusStatus {
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
    pub struct NotificationStatus {
        #[serde(rename = "actionTimeMillis", default)]
        #[serde(with = "crate::parsed_string")]
        pub action_time_millis: ::std::option::Option<i64>,
        #[serde(rename = "notificationId", default)]
        pub notification_id: ::std::option::Option<String>,
        #[serde(rename = "rating", default)]
        pub rating: ::std::option::Option<f64>,
        #[doc = "Identifies the notification module; we may want to allow some wild carding so the user can squelch entire categories of notification."]
        #[serde(rename = "sourceName", default)]
        pub source_name: ::std::option::Option<String>,
        #[serde(rename = "status", default)]
        pub status: ::std::option::Option<crate::schemas::NotificationStatusStatus>,
    }
    impl ::field_selector::FieldSelector for NotificationStatus {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum NotificationTimelinePositionSpecialPosition {
        Top,
    }
    impl NotificationTimelinePositionSpecialPosition {
        pub fn as_str(self) -> &'static str {
            match self {
                NotificationTimelinePositionSpecialPosition::Top => "top",
            }
        }
    }
    impl ::std::fmt::Display for NotificationTimelinePositionSpecialPosition {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for NotificationTimelinePositionSpecialPosition {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for NotificationTimelinePositionSpecialPosition {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "top" => NotificationTimelinePositionSpecialPosition::Top,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::field_selector::FieldSelector for NotificationTimelinePositionSpecialPosition {
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
    pub struct NotificationTimelinePosition {
        #[doc = "Show in a fixed position in the timeline."]
        #[serde(rename = "specialPosition", default)]
        pub special_position:
            ::std::option::Option<crate::schemas::NotificationTimelinePositionSpecialPosition>,
        #[doc = "Show in a specific spot in the timeline denoted by a timestamp."]
        #[serde(rename = "timestampMillis", default)]
        #[serde(with = "crate::parsed_string")]
        pub timestamp_millis: ::std::option::Option<i64>,
    }
    impl ::field_selector::FieldSelector for NotificationTimelinePosition {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum NotificationTriggerConditionCaloriesExpendedLockedState {
        Locked,
        Unlocked,
        Unspecified,
    }
    impl NotificationTriggerConditionCaloriesExpendedLockedState {
        pub fn as_str(self) -> &'static str {
            match self {
                NotificationTriggerConditionCaloriesExpendedLockedState::Locked => "locked",
                NotificationTriggerConditionCaloriesExpendedLockedState::Unlocked => "unlocked",
                NotificationTriggerConditionCaloriesExpendedLockedState::Unspecified => {
                    "unspecified"
                }
            }
        }
    }
    impl ::std::fmt::Display for NotificationTriggerConditionCaloriesExpendedLockedState {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for NotificationTriggerConditionCaloriesExpendedLockedState {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for NotificationTriggerConditionCaloriesExpendedLockedState {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "locked" => NotificationTriggerConditionCaloriesExpendedLockedState::Locked,
                "unlocked" => NotificationTriggerConditionCaloriesExpendedLockedState::Unlocked,
                "unspecified" => {
                    NotificationTriggerConditionCaloriesExpendedLockedState::Unspecified
                }
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::field_selector::FieldSelector for NotificationTriggerConditionCaloriesExpendedLockedState {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum NotificationTriggerConditionClient {
        MobileAndTablet,
        Web,
    }
    impl NotificationTriggerConditionClient {
        pub fn as_str(self) -> &'static str {
            match self {
                NotificationTriggerConditionClient::MobileAndTablet => "mobileAndTablet",
                NotificationTriggerConditionClient::Web => "web",
            }
        }
    }
    impl ::std::fmt::Display for NotificationTriggerConditionClient {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for NotificationTriggerConditionClient {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for NotificationTriggerConditionClient {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "mobileAndTablet" => NotificationTriggerConditionClient::MobileAndTablet,
                "web" => NotificationTriggerConditionClient::Web,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::field_selector::FieldSelector for NotificationTriggerConditionClient {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum NotificationTriggerConditionDistanceLockedState {
        Locked,
        Unlocked,
        Unspecified,
    }
    impl NotificationTriggerConditionDistanceLockedState {
        pub fn as_str(self) -> &'static str {
            match self {
                NotificationTriggerConditionDistanceLockedState::Locked => "locked",
                NotificationTriggerConditionDistanceLockedState::Unlocked => "unlocked",
                NotificationTriggerConditionDistanceLockedState::Unspecified => "unspecified",
            }
        }
    }
    impl ::std::fmt::Display for NotificationTriggerConditionDistanceLockedState {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for NotificationTriggerConditionDistanceLockedState {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for NotificationTriggerConditionDistanceLockedState {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "locked" => NotificationTriggerConditionDistanceLockedState::Locked,
                "unlocked" => NotificationTriggerConditionDistanceLockedState::Unlocked,
                "unspecified" => NotificationTriggerConditionDistanceLockedState::Unspecified,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::field_selector::FieldSelector for NotificationTriggerConditionDistanceLockedState {
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
    pub struct NotificationTriggerCondition {
        #[doc = "NOTE: the following trigger conditions were added for Bolt (2015104000+). The app will use the key specified to decide whether to trigger. It will trigger if the value is true, suppress if the value is false."]
        #[serde(rename = "booleanGservicesKey", default)]
        pub boolean_gservices_key: ::std::option::Option<String>,
        #[serde(rename = "caloriesExpendedLockedState", default)]
        pub calories_expended_locked_state: ::std::option::Option<
            crate::schemas::NotificationTriggerConditionCaloriesExpendedLockedState,
        >,
        #[serde(rename = "client", default)]
        pub client: ::std::option::Option<crate::schemas::NotificationTriggerConditionClient>,
        #[serde(rename = "distanceLockedState", default)]
        pub distance_locked_state:
            ::std::option::Option<crate::schemas::NotificationTriggerConditionDistanceLockedState>,
        #[serde(rename = "endTimeMillis", default)]
        #[serde(with = "crate::parsed_string")]
        pub end_time_millis: ::std::option::Option<i64>,
        #[doc = "A time time interval within which this notification is relevant."]
        #[serde(rename = "startTimeMillis", default)]
        #[serde(with = "crate::parsed_string")]
        pub start_time_millis: ::std::option::Option<i64>,
        #[doc = "Ordinal of FitnessMode.java enum. What modes should this notification be displayed in? If empty, we assume all modes."]
        #[serde(rename = "supportedMode", default)]
        pub supported_mode: ::std::option::Option<Vec<i32>>,
    }
    impl ::field_selector::FieldSelector for NotificationTriggerCondition {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum NotificationUiModelIcon {
        Assignment,
        Biking,
        Goal,
        HighAccuracy,
        Nutrition,
        OtherActivity,
        PlayStore,
        Profile,
        Running,
        Settings,
        Trend,
        Trophy,
        Walking,
    }
    impl NotificationUiModelIcon {
        pub fn as_str(self) -> &'static str {
            match self {
                NotificationUiModelIcon::Assignment => "assignment",
                NotificationUiModelIcon::Biking => "biking",
                NotificationUiModelIcon::Goal => "goal",
                NotificationUiModelIcon::HighAccuracy => "highAccuracy",
                NotificationUiModelIcon::Nutrition => "nutrition",
                NotificationUiModelIcon::OtherActivity => "otherActivity",
                NotificationUiModelIcon::PlayStore => "playStore",
                NotificationUiModelIcon::Profile => "profile",
                NotificationUiModelIcon::Running => "running",
                NotificationUiModelIcon::Settings => "settings",
                NotificationUiModelIcon::Trend => "trend",
                NotificationUiModelIcon::Trophy => "trophy",
                NotificationUiModelIcon::Walking => "walking",
            }
        }
    }
    impl ::std::fmt::Display for NotificationUiModelIcon {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for NotificationUiModelIcon {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for NotificationUiModelIcon {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "assignment" => NotificationUiModelIcon::Assignment,
                "biking" => NotificationUiModelIcon::Biking,
                "goal" => NotificationUiModelIcon::Goal,
                "highAccuracy" => NotificationUiModelIcon::HighAccuracy,
                "nutrition" => NotificationUiModelIcon::Nutrition,
                "otherActivity" => NotificationUiModelIcon::OtherActivity,
                "playStore" => NotificationUiModelIcon::PlayStore,
                "profile" => NotificationUiModelIcon::Profile,
                "running" => NotificationUiModelIcon::Running,
                "settings" => NotificationUiModelIcon::Settings,
                "trend" => NotificationUiModelIcon::Trend,
                "trophy" => NotificationUiModelIcon::Trophy,
                "walking" => NotificationUiModelIcon::Walking,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::field_selector::FieldSelector for NotificationUiModelIcon {
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
    pub struct NotificationUiModel {
        #[doc = "Each of these additional messages will appear on separate lines in the body of a notification"]
        #[serde(rename = "additionalMessage", default)]
        pub additional_message: ::std::option::Option<Vec<String>>,
        #[doc = "Order of these message ids should correspond to those in additional_message. They are used only on the server side for localization."]
        #[serde(rename = "additionalMessageMsg", default)]
        pub additional_message_msg: ::std::option::Option<Vec<crate::schemas::LocalizableMessage>>,
        #[serde(rename = "button", default)]
        pub button: ::std::option::Option<Vec<crate::schemas::NotificationUiModelButton>>,
        #[doc = "Whether or not the user can dismiss this notification (via a swipe or other UI action)."]
        #[serde(rename = "dismissible", default)]
        pub dismissible: ::std::option::Option<bool>,
        #[serde(rename = "footnote", default)]
        pub footnote: ::std::option::Option<String>,
        #[serde(rename = "footnoteMsg", default)]
        pub footnote_msg: ::std::option::Option<crate::schemas::LocalizableMessage>,
        #[serde(rename = "graph", default)]
        pub graph: ::std::option::Option<Vec<crate::schemas::NotificationUiModelGraph>>,
        #[serde(rename = "icon", default)]
        pub icon: ::std::option::Option<crate::schemas::NotificationUiModelIcon>,
        #[serde(rename = "intent", default)]
        pub intent: ::std::option::Option<crate::schemas::NotificationUiModelIntent>,
        #[serde(rename = "message", default)]
        pub message: ::std::option::Option<String>,
        #[serde(rename = "messageMsg", default)]
        pub message_msg: ::std::option::Option<crate::schemas::LocalizableMessage>,
        #[serde(rename = "options", default)]
        pub options: ::std::option::Option<crate::schemas::NotificationUiModelOptions>,
        #[doc = "If provided, sets the title and button colors. If not provided, we get the color from the icon or activity type."]
        #[serde(rename = "primaryColor", default)]
        pub primary_color: ::std::option::Option<i32>,
        #[serde(rename = "recommendation", default)]
        pub recommendation:
            ::std::option::Option<crate::schemas::NotificationUiModelRecommendation>,
        #[doc = "Identifies this UiModel to be shown when an option is selected. Corresponds to option.value.target_state (if specified)."]
        #[serde(rename = "stateName", default)]
        pub state_name: ::std::option::Option<String>,
        #[serde(rename = "subtitle", default)]
        pub subtitle: ::std::option::Option<String>,
        #[serde(rename = "subtitleMsg", default)]
        pub subtitle_msg: ::std::option::Option<crate::schemas::LocalizableMessage>,
        #[doc = "Localized messages that're shown to the user"]
        #[serde(rename = "title", default)]
        pub title: ::std::option::Option<String>,
        #[serde(rename = "titleMsg", default)]
        pub title_msg: ::std::option::Option<crate::schemas::LocalizableMessage>,
    }
    impl ::field_selector::FieldSelector for NotificationUiModel {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum NotificationUiModelButtonAction {
        DeprecatedDismissAsComplete,
        Dismiss,
        DismissAsAccepted,
        DismissAsBlocked,
        EnableHighAccuracy,
        Intent,
        ReengageUser,
        SaveGoal,
        SaveOptionGoal,
        ShowNextState,
        ShowTargetState,
    }
    impl NotificationUiModelButtonAction {
        pub fn as_str(self) -> &'static str {
            match self {
                NotificationUiModelButtonAction::DeprecatedDismissAsComplete => {
                    "deprecatedDismissAsComplete"
                }
                NotificationUiModelButtonAction::Dismiss => "dismiss",
                NotificationUiModelButtonAction::DismissAsAccepted => "dismissAsAccepted",
                NotificationUiModelButtonAction::DismissAsBlocked => "dismissAsBlocked",
                NotificationUiModelButtonAction::EnableHighAccuracy => "enableHighAccuracy",
                NotificationUiModelButtonAction::Intent => "intent",
                NotificationUiModelButtonAction::ReengageUser => "reengageUser",
                NotificationUiModelButtonAction::SaveGoal => "saveGoal",
                NotificationUiModelButtonAction::SaveOptionGoal => "saveOptionGoal",
                NotificationUiModelButtonAction::ShowNextState => "showNextState",
                NotificationUiModelButtonAction::ShowTargetState => "showTargetState",
            }
        }
    }
    impl ::std::fmt::Display for NotificationUiModelButtonAction {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for NotificationUiModelButtonAction {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for NotificationUiModelButtonAction {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "deprecatedDismissAsComplete" => {
                    NotificationUiModelButtonAction::DeprecatedDismissAsComplete
                }
                "dismiss" => NotificationUiModelButtonAction::Dismiss,
                "dismissAsAccepted" => NotificationUiModelButtonAction::DismissAsAccepted,
                "dismissAsBlocked" => NotificationUiModelButtonAction::DismissAsBlocked,
                "enableHighAccuracy" => NotificationUiModelButtonAction::EnableHighAccuracy,
                "intent" => NotificationUiModelButtonAction::Intent,
                "reengageUser" => NotificationUiModelButtonAction::ReengageUser,
                "saveGoal" => NotificationUiModelButtonAction::SaveGoal,
                "saveOptionGoal" => NotificationUiModelButtonAction::SaveOptionGoal,
                "showNextState" => NotificationUiModelButtonAction::ShowNextState,
                "showTargetState" => NotificationUiModelButtonAction::ShowTargetState,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::field_selector::FieldSelector for NotificationUiModelButtonAction {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum NotificationUiModelButtonType {
        Button,
        Link,
    }
    impl NotificationUiModelButtonType {
        pub fn as_str(self) -> &'static str {
            match self {
                NotificationUiModelButtonType::Button => "button",
                NotificationUiModelButtonType::Link => "link",
            }
        }
    }
    impl ::std::fmt::Display for NotificationUiModelButtonType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for NotificationUiModelButtonType {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for NotificationUiModelButtonType {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "button" => NotificationUiModelButtonType::Button,
                "link" => NotificationUiModelButtonType::Link,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::field_selector::FieldSelector for NotificationUiModelButtonType {
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
    pub struct NotificationUiModelButton {
        #[serde(rename = "action", default)]
        pub action: ::std::option::Option<crate::schemas::NotificationUiModelButtonAction>,
        #[serde(rename = "label", default)]
        pub label: ::std::option::Option<String>,
        #[serde(rename = "labelMsg", default)]
        pub label_msg: ::std::option::Option<crate::schemas::LocalizableMessage>,
        #[serde(rename = "type", default)]
        pub r#type: ::std::option::Option<crate::schemas::NotificationUiModelButtonType>,
        #[serde(rename = "targetState", default)]
        pub target_state: ::std::option::Option<String>,
    }
    impl ::field_selector::FieldSelector for NotificationUiModelButton {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum NotificationUiModelGraphStyle {
        HorizontalBar,
        Line,
        Pie,
        StackedHorizontalBar,
    }
    impl NotificationUiModelGraphStyle {
        pub fn as_str(self) -> &'static str {
            match self {
                NotificationUiModelGraphStyle::HorizontalBar => "horizontalBar",
                NotificationUiModelGraphStyle::Line => "line",
                NotificationUiModelGraphStyle::Pie => "pie",
                NotificationUiModelGraphStyle::StackedHorizontalBar => "stackedHorizontalBar",
            }
        }
    }
    impl ::std::fmt::Display for NotificationUiModelGraphStyle {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for NotificationUiModelGraphStyle {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for NotificationUiModelGraphStyle {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "horizontalBar" => NotificationUiModelGraphStyle::HorizontalBar,
                "line" => NotificationUiModelGraphStyle::Line,
                "pie" => NotificationUiModelGraphStyle::Pie,
                "stackedHorizontalBar" => NotificationUiModelGraphStyle::StackedHorizontalBar,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::field_selector::FieldSelector for NotificationUiModelGraphStyle {
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
    pub struct NotificationUiModelGraph {
        #[doc = "Whether or not the data values will be displayed on the graph."]
        #[serde(rename = "includeLabel", default)]
        pub include_label: ::std::option::Option<bool>,
        #[doc = "Whether or not the domain will be displayed to the side of the graph, aka legend."]
        #[serde(rename = "includeLegend", default)]
        pub include_legend: ::std::option::Option<bool>,
        #[doc = "Specify the range's max extent to be applied to nicing functions."]
        #[serde(rename = "rangeMaxExtent", default)]
        pub range_max_extent: ::std::option::Option<f64>,
        #[serde(rename = "series", default)]
        pub series: ::std::option::Option<Vec<crate::schemas::NotificationUiModelGraphSeries>>,
        #[doc = "Type of graph/chart to display."]
        #[serde(rename = "style", default)]
        pub style: ::std::option::Option<crate::schemas::NotificationUiModelGraphStyle>,
        #[serde(rename = "title", default)]
        pub title: ::std::option::Option<String>,
        #[doc = "The text to be displayed above this graph."]
        #[serde(rename = "titleMsg", default)]
        pub title_msg: ::std::option::Option<crate::schemas::LocalizableMessage>,
    }
    impl ::field_selector::FieldSelector for NotificationUiModelGraph {
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
    pub struct NotificationUiModelGraphSeries {
        #[serde(rename = "name", default)]
        pub name: ::std::option::Option<String>,
        #[serde(rename = "nameMsg", default)]
        pub name_msg: ::std::option::Option<crate::schemas::LocalizableMessage>,
        #[serde(rename = "point", default)]
        pub point:
            ::std::option::Option<Vec<crate::schemas::NotificationUiModelGraphSeriesPlotPoint>>,
    }
    impl ::field_selector::FieldSelector for NotificationUiModelGraphSeries {
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
    pub struct NotificationUiModelGraphSeriesPlotPoint {
        #[serde(rename = "color", default)]
        pub color: ::std::option::Option<i32>,
        #[serde(rename = "domain", default)]
        pub domain: ::std::option::Option<String>,
        #[doc = "Domains contribute to labels and legends."]
        #[serde(rename = "domainMsg", default)]
        pub domain_msg: ::std::option::Option<crate::schemas::LocalizableMessage>,
        #[serde(rename = "range", default)]
        pub range: ::std::option::Option<f64>,
    }
    impl ::field_selector::FieldSelector for NotificationUiModelGraphSeriesPlotPoint {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum NotificationUiModelIntentLocation {
        GoogleFitData,
        MyProfile,
        PlayStore,
        Settings,
    }
    impl NotificationUiModelIntentLocation {
        pub fn as_str(self) -> &'static str {
            match self {
                NotificationUiModelIntentLocation::GoogleFitData => "googleFitData",
                NotificationUiModelIntentLocation::MyProfile => "myProfile",
                NotificationUiModelIntentLocation::PlayStore => "playStore",
                NotificationUiModelIntentLocation::Settings => "settings",
            }
        }
    }
    impl ::std::fmt::Display for NotificationUiModelIntentLocation {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for NotificationUiModelIntentLocation {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for NotificationUiModelIntentLocation {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "googleFitData" => NotificationUiModelIntentLocation::GoogleFitData,
                "myProfile" => NotificationUiModelIntentLocation::MyProfile,
                "playStore" => NotificationUiModelIntentLocation::PlayStore,
                "settings" => NotificationUiModelIntentLocation::Settings,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::field_selector::FieldSelector for NotificationUiModelIntentLocation {
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
    pub struct NotificationUiModelIntent {
        #[doc = "A feature screen in the client."]
        #[serde(rename = "location", default)]
        pub location: ::std::option::Option<crate::schemas::NotificationUiModelIntentLocation>,
        #[doc = "A web url."]
        #[serde(rename = "url", default)]
        pub url: ::std::option::Option<String>,
    }
    impl ::field_selector::FieldSelector for NotificationUiModelIntent {
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
    pub struct NotificationUiModelOptions {
        #[doc = "The default option that should be selected by the UI"]
        #[serde(rename = "defaultValueIndex", default)]
        pub default_value_index: ::std::option::Option<i32>,
        #[serde(rename = "value", default)]
        pub value: ::std::option::Option<Vec<crate::schemas::NotificationUiModelOptionsValue>>,
    }
    impl ::field_selector::FieldSelector for NotificationUiModelOptions {
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
    pub struct NotificationUiModelOptionsValue {
        #[serde(rename = "displayString", default)]
        pub display_string: ::std::option::Option<String>,
        #[serde(rename = "displayStringMsg", default)]
        pub display_string_msg: ::std::option::Option<crate::schemas::LocalizableMessage>,
        #[serde(rename = "recommendation", default)]
        pub recommendation:
            ::std::option::Option<crate::schemas::NotificationUiModelRecommendation>,
    }
    impl ::field_selector::FieldSelector for NotificationUiModelOptionsValue {
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
    pub struct NotificationUiModelRecommendation {
        #[doc = "The details for a new goal if we?re recommending a sub goal or replacement goal."]
        #[serde(rename = "goalTemplate", default)]
        pub goal_template: ::std::option::Option<crate::schemas::Goal>,
        #[doc = "A quantified benefit expected to be gained by performing this recommendation (e.g. number of steps toward goal)"]
        #[serde(rename = "projectedBenefit", default)]
        pub projected_benefit:
            ::std::option::Option<crate::schemas::NotificationUiModelRecommendationDataValue>,
        #[doc = "If the goal described in goal_details replaces that identified by parent_goal_id, this should be set to true. If accepted, the parent goal should be cancelled and the contained goal be set in its place."]
        #[serde(rename = "replacesParentGoal", default)]
        pub replaces_parent_goal: ::std::option::Option<bool>,
    }
    impl ::field_selector::FieldSelector for NotificationUiModelRecommendation {
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
    pub struct NotificationUiModelRecommendationDataValue {
        #[serde(rename = "dataType", default)]
        pub data_type: ::std::option::Option<crate::schemas::DataType>,
        #[serde(rename = "value", default)]
        pub value: ::std::option::Option<f64>,
    }
    impl ::field_selector::FieldSelector for NotificationUiModelRecommendationDataValue {
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
    pub struct PersonalRecordCapsuleConfig {
        #[serde(rename = "fitnessActivity", default)]
        pub fitness_activity: ::std::option::Option<String>,
    }
    impl ::field_selector::FieldSelector for PersonalRecordCapsuleConfig {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum ProfileDistanceUnitPref {
        Imperial,
        Metric,
        UnknownLengthUnit,
    }
    impl ProfileDistanceUnitPref {
        pub fn as_str(self) -> &'static str {
            match self {
                ProfileDistanceUnitPref::Imperial => "imperial",
                ProfileDistanceUnitPref::Metric => "metric",
                ProfileDistanceUnitPref::UnknownLengthUnit => "unknownLengthUnit",
            }
        }
    }
    impl ::std::fmt::Display for ProfileDistanceUnitPref {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for ProfileDistanceUnitPref {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for ProfileDistanceUnitPref {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "imperial" => ProfileDistanceUnitPref::Imperial,
                "metric" => ProfileDistanceUnitPref::Metric,
                "unknownLengthUnit" => ProfileDistanceUnitPref::UnknownLengthUnit,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::field_selector::FieldSelector for ProfileDistanceUnitPref {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum ProfileEnergyUnitPref {
        Calorie,
        Kilojoule,
        UnknownEnergyUnit,
    }
    impl ProfileEnergyUnitPref {
        pub fn as_str(self) -> &'static str {
            match self {
                ProfileEnergyUnitPref::Calorie => "calorie",
                ProfileEnergyUnitPref::Kilojoule => "kilojoule",
                ProfileEnergyUnitPref::UnknownEnergyUnit => "unknownEnergyUnit",
            }
        }
    }
    impl ::std::fmt::Display for ProfileEnergyUnitPref {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for ProfileEnergyUnitPref {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for ProfileEnergyUnitPref {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "calorie" => ProfileEnergyUnitPref::Calorie,
                "kilojoule" => ProfileEnergyUnitPref::Kilojoule,
                "unknownEnergyUnit" => ProfileEnergyUnitPref::UnknownEnergyUnit,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::field_selector::FieldSelector for ProfileEnergyUnitPref {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum ProfileGender {
        Female,
        Male,
        Other,
        UnknownGender,
    }
    impl ProfileGender {
        pub fn as_str(self) -> &'static str {
            match self {
                ProfileGender::Female => "female",
                ProfileGender::Male => "male",
                ProfileGender::Other => "other",
                ProfileGender::UnknownGender => "unknownGender",
            }
        }
    }
    impl ::std::fmt::Display for ProfileGender {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for ProfileGender {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for ProfileGender {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "female" => ProfileGender::Female,
                "male" => ProfileGender::Male,
                "other" => ProfileGender::Other,
                "unknownGender" => ProfileGender::UnknownGender,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::field_selector::FieldSelector for ProfileGender {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum ProfileHeightUnitPref {
        Imperial,
        Metric,
        UnknownLengthUnit,
    }
    impl ProfileHeightUnitPref {
        pub fn as_str(self) -> &'static str {
            match self {
                ProfileHeightUnitPref::Imperial => "imperial",
                ProfileHeightUnitPref::Metric => "metric",
                ProfileHeightUnitPref::UnknownLengthUnit => "unknownLengthUnit",
            }
        }
    }
    impl ::std::fmt::Display for ProfileHeightUnitPref {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for ProfileHeightUnitPref {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for ProfileHeightUnitPref {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "imperial" => ProfileHeightUnitPref::Imperial,
                "metric" => ProfileHeightUnitPref::Metric,
                "unknownLengthUnit" => ProfileHeightUnitPref::UnknownLengthUnit,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::field_selector::FieldSelector for ProfileHeightUnitPref {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum ProfileStartOfWeekPref {
        DayOfWeekUnspecified,
        Friday,
        Monday,
        Saturday,
        Sunday,
        Thursday,
        Tuesday,
        Wednesday,
    }
    impl ProfileStartOfWeekPref {
        pub fn as_str(self) -> &'static str {
            match self {
                ProfileStartOfWeekPref::DayOfWeekUnspecified => "dayOfWeekUnspecified",
                ProfileStartOfWeekPref::Friday => "friday",
                ProfileStartOfWeekPref::Monday => "monday",
                ProfileStartOfWeekPref::Saturday => "saturday",
                ProfileStartOfWeekPref::Sunday => "sunday",
                ProfileStartOfWeekPref::Thursday => "thursday",
                ProfileStartOfWeekPref::Tuesday => "tuesday",
                ProfileStartOfWeekPref::Wednesday => "wednesday",
            }
        }
    }
    impl ::std::fmt::Display for ProfileStartOfWeekPref {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for ProfileStartOfWeekPref {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for ProfileStartOfWeekPref {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "dayOfWeekUnspecified" => ProfileStartOfWeekPref::DayOfWeekUnspecified,
                "friday" => ProfileStartOfWeekPref::Friday,
                "monday" => ProfileStartOfWeekPref::Monday,
                "saturday" => ProfileStartOfWeekPref::Saturday,
                "sunday" => ProfileStartOfWeekPref::Sunday,
                "thursday" => ProfileStartOfWeekPref::Thursday,
                "tuesday" => ProfileStartOfWeekPref::Tuesday,
                "wednesday" => ProfileStartOfWeekPref::Wednesday,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::field_selector::FieldSelector for ProfileStartOfWeekPref {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum ProfileWeightUnitPref {
        Kilogram,
        Pound,
        Stone,
        UnknownWeightUnit,
    }
    impl ProfileWeightUnitPref {
        pub fn as_str(self) -> &'static str {
            match self {
                ProfileWeightUnitPref::Kilogram => "kilogram",
                ProfileWeightUnitPref::Pound => "pound",
                ProfileWeightUnitPref::Stone => "stone",
                ProfileWeightUnitPref::UnknownWeightUnit => "unknownWeightUnit",
            }
        }
    }
    impl ::std::fmt::Display for ProfileWeightUnitPref {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for ProfileWeightUnitPref {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for ProfileWeightUnitPref {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "kilogram" => ProfileWeightUnitPref::Kilogram,
                "pound" => ProfileWeightUnitPref::Pound,
                "stone" => ProfileWeightUnitPref::Stone,
                "unknownWeightUnit" => ProfileWeightUnitPref::UnknownWeightUnit,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::field_selector::FieldSelector for ProfileWeightUnitPref {
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
    pub struct Profile {
        #[doc = "Timestamp corresponding to when the user volunteered height, weight and gender info that unlocks calorie data"]
        #[serde(rename = "calorieUnlockTimeMillis", default)]
        #[serde(with = "crate::parsed_string")]
        pub calorie_unlock_time_millis: ::std::option::Option<i64>,
        #[serde(rename = "creationTimeMillis", default)]
        #[serde(with = "crate::parsed_string")]
        pub creation_time_millis: ::std::option::Option<i64>,
        #[serde(rename = "distanceUnitPref", default)]
        pub distance_unit_pref: ::std::option::Option<crate::schemas::ProfileDistanceUnitPref>,
        #[doc = "Timestamp corresponding to when the user volunteered height info that unlocks distance data"]
        #[serde(rename = "distanceUnlockTimeMillis", default)]
        #[serde(with = "crate::parsed_string")]
        pub distance_unlock_time_millis: ::std::option::Option<i64>,
        #[serde(rename = "energyUnitPref", default)]
        pub energy_unit_pref: ::std::option::Option<crate::schemas::ProfileEnergyUnitPref>,
        #[doc = "User specified favorite activities"]
        #[serde(rename = "favoriteActivities", default)]
        pub favorite_activities: ::std::option::Option<crate::schemas::FavoriteActivities>,
        #[doc = "List of up to seven activities to appear at the top of add/edit activity dialogs. Ordered from most recent use to least recent use. Does not include walking, running, and biking which are always at the top."]
        #[serde(rename = "favoriteOtherActivities", default)]
        pub favorite_other_activities: ::std::option::Option<Vec<String>>,
        #[serde(rename = "gender", default)]
        pub gender: ::std::option::Option<crate::schemas::ProfileGender>,
        #[serde(rename = "heightMeters", default)]
        pub height_meters: ::std::option::Option<f64>,
        #[serde(rename = "heightUnitPref", default)]
        pub height_unit_pref: ::std::option::Option<crate::schemas::ProfileHeightUnitPref>,
        #[serde(rename = "isInternalUser", default)]
        pub is_internal_user: ::std::option::Option<bool>,
        #[serde(rename = "notificationSetting", default)]
        pub notification_setting:
            ::std::option::Option<Vec<crate::schemas::ProfileNotificationSetting>>,
        #[serde(rename = "onboardComplete", default)]
        pub onboard_complete: ::std::option::Option<bool>,
        #[doc = "Indicates the day of the week on which to start a week."]
        #[serde(rename = "startOfWeekPref", default)]
        pub start_of_week_pref: ::std::option::Option<crate::schemas::ProfileStartOfWeekPref>,
        #[serde(rename = "weightKg", default)]
        pub weight_kg: ::std::option::Option<f64>,
        #[serde(rename = "weightUnitPref", default)]
        pub weight_unit_pref: ::std::option::Option<crate::schemas::ProfileWeightUnitPref>,
    }
    impl ::field_selector::FieldSelector for Profile {
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
    pub struct ProfileNotificationSetting {
        #[serde(rename = "blocked", default)]
        pub blocked: ::std::option::Option<bool>,
        #[serde(rename = "sourceDescription", default)]
        pub source_description: ::std::option::Option<String>,
        #[serde(rename = "sourceName", default)]
        pub source_name: ::std::option::Option<String>,
        #[doc = "Title/description are used in the blocked notifications UI."]
        #[serde(rename = "sourceTitle", default)]
        pub source_title: ::std::option::Option<String>,
    }
    impl ::field_selector::FieldSelector for ProfileNotificationSetting {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum SingleMetricCapsuleConfigMetricView {
        Distance,
        Duration,
        Energy,
        Step,
        Unknown,
    }
    impl SingleMetricCapsuleConfigMetricView {
        pub fn as_str(self) -> &'static str {
            match self {
                SingleMetricCapsuleConfigMetricView::Distance => "distance",
                SingleMetricCapsuleConfigMetricView::Duration => "duration",
                SingleMetricCapsuleConfigMetricView::Energy => "energy",
                SingleMetricCapsuleConfigMetricView::Step => "step",
                SingleMetricCapsuleConfigMetricView::Unknown => "unknown",
            }
        }
    }
    impl ::std::fmt::Display for SingleMetricCapsuleConfigMetricView {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for SingleMetricCapsuleConfigMetricView {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for SingleMetricCapsuleConfigMetricView {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "distance" => SingleMetricCapsuleConfigMetricView::Distance,
                "duration" => SingleMetricCapsuleConfigMetricView::Duration,
                "energy" => SingleMetricCapsuleConfigMetricView::Energy,
                "step" => SingleMetricCapsuleConfigMetricView::Step,
                "unknown" => SingleMetricCapsuleConfigMetricView::Unknown,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::field_selector::FieldSelector for SingleMetricCapsuleConfigMetricView {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum SingleMetricCapsuleConfigTimeInterval {
        Daily,
        Monthly,
        UnknownTimeInterval,
        Weekly,
        Yearly,
    }
    impl SingleMetricCapsuleConfigTimeInterval {
        pub fn as_str(self) -> &'static str {
            match self {
                SingleMetricCapsuleConfigTimeInterval::Daily => "daily",
                SingleMetricCapsuleConfigTimeInterval::Monthly => "monthly",
                SingleMetricCapsuleConfigTimeInterval::UnknownTimeInterval => "unknownTimeInterval",
                SingleMetricCapsuleConfigTimeInterval::Weekly => "weekly",
                SingleMetricCapsuleConfigTimeInterval::Yearly => "yearly",
            }
        }
    }
    impl ::std::fmt::Display for SingleMetricCapsuleConfigTimeInterval {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for SingleMetricCapsuleConfigTimeInterval {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for SingleMetricCapsuleConfigTimeInterval {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "daily" => SingleMetricCapsuleConfigTimeInterval::Daily,
                "monthly" => SingleMetricCapsuleConfigTimeInterval::Monthly,
                "unknownTimeInterval" => SingleMetricCapsuleConfigTimeInterval::UnknownTimeInterval,
                "weekly" => SingleMetricCapsuleConfigTimeInterval::Weekly,
                "yearly" => SingleMetricCapsuleConfigTimeInterval::Yearly,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::field_selector::FieldSelector for SingleMetricCapsuleConfigTimeInterval {
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
    pub struct SingleMetricCapsuleConfig {
        #[serde(rename = "fitnessActivity", default)]
        pub fitness_activity: ::std::option::Option<String>,
        #[serde(rename = "metricView", default)]
        pub metric_view: ::std::option::Option<crate::schemas::SingleMetricCapsuleConfigMetricView>,
        #[serde(rename = "timeInterval", default)]
        pub time_interval:
            ::std::option::Option<crate::schemas::SingleMetricCapsuleConfigTimeInterval>,
    }
    impl ::field_selector::FieldSelector for SingleMetricCapsuleConfig {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum TimelineSessionPropertyItems {
        DetectedSportsSession,
        ExceedsBaseline,
        External,
        Filler,
        Insignificant,
        StronglyExceedsBaseline,
    }
    impl TimelineSessionPropertyItems {
        pub fn as_str(self) -> &'static str {
            match self {
                TimelineSessionPropertyItems::DetectedSportsSession => "detectedSportsSession",
                TimelineSessionPropertyItems::ExceedsBaseline => "exceedsBaseline",
                TimelineSessionPropertyItems::External => "external",
                TimelineSessionPropertyItems::Filler => "filler",
                TimelineSessionPropertyItems::Insignificant => "insignificant",
                TimelineSessionPropertyItems::StronglyExceedsBaseline => "stronglyExceedsBaseline",
            }
        }
    }
    impl ::std::fmt::Display for TimelineSessionPropertyItems {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for TimelineSessionPropertyItems {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for TimelineSessionPropertyItems {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "detectedSportsSession" => TimelineSessionPropertyItems::DetectedSportsSession,
                "exceedsBaseline" => TimelineSessionPropertyItems::ExceedsBaseline,
                "external" => TimelineSessionPropertyItems::External,
                "filler" => TimelineSessionPropertyItems::Filler,
                "insignificant" => TimelineSessionPropertyItems::Insignificant,
                "stronglyExceedsBaseline" => TimelineSessionPropertyItems::StronglyExceedsBaseline,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::field_selector::FieldSelector for TimelineSessionPropertyItems {
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
    pub struct TimelineSession {
        #[serde(rename = "activityInfo", default)]
        pub activity_info: ::std::option::Option<Vec<crate::schemas::TimelineSessionActivityInfo>>,
        #[serde(rename = "calories", default)]
        pub calories: ::std::option::Option<f32>,
        #[doc = "A description for this session."]
        #[serde(rename = "description", default)]
        pub description: ::std::option::Option<String>,
        #[doc = "The summation of the represented activity segments."]
        #[serde(rename = "durationMillis", default)]
        #[serde(with = "crate::parsed_string")]
        pub duration_millis: ::std::option::Option<i64>,
        #[doc = "The time the last activity segment ended."]
        #[serde(rename = "endTimeMillis", default)]
        #[serde(with = "crate::parsed_string")]
        pub end_time_millis: ::std::option::Option<i64>,
        #[doc = "A unique identifier for a session within the user?s account. Simply a concatenation of the start and end time millis, separated by a '-'."]
        #[serde(rename = "id", default)]
        pub id: ::std::option::Option<String>,
        #[serde(rename = "location", default)]
        pub location: ::std::option::Option<crate::schemas::TimelineSessionLocation>,
        #[doc = "A human readable name of the session."]
        #[serde(rename = "name", default)]
        pub name: ::std::option::Option<String>,
        #[serde(rename = "property", default)]
        pub property: ::std::option::Option<Vec<crate::schemas::TimelineSessionPropertyItems>>,
        #[doc = "The segments that make up the activity."]
        #[serde(rename = "segment", default)]
        pub segment: ::std::option::Option<Vec<crate::schemas::TimelineSessionSegment>>,
        #[doc = "The time the first activity segment started."]
        #[serde(rename = "startTimeMillis", default)]
        #[serde(with = "crate::parsed_string")]
        pub start_time_millis: ::std::option::Option<i64>,
    }
    impl ::field_selector::FieldSelector for TimelineSession {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum TimelineSessionActivityInfoPropertyItems {
        DetectedSportsSession,
        ExceedsBaseline,
        External,
        Filler,
        Insignificant,
        StronglyExceedsBaseline,
    }
    impl TimelineSessionActivityInfoPropertyItems {
        pub fn as_str(self) -> &'static str {
            match self {
                TimelineSessionActivityInfoPropertyItems::DetectedSportsSession => {
                    "detectedSportsSession"
                }
                TimelineSessionActivityInfoPropertyItems::ExceedsBaseline => "exceedsBaseline",
                TimelineSessionActivityInfoPropertyItems::External => "external",
                TimelineSessionActivityInfoPropertyItems::Filler => "filler",
                TimelineSessionActivityInfoPropertyItems::Insignificant => "insignificant",
                TimelineSessionActivityInfoPropertyItems::StronglyExceedsBaseline => {
                    "stronglyExceedsBaseline"
                }
            }
        }
    }
    impl ::std::fmt::Display for TimelineSessionActivityInfoPropertyItems {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for TimelineSessionActivityInfoPropertyItems {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for TimelineSessionActivityInfoPropertyItems {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "detectedSportsSession" => {
                    TimelineSessionActivityInfoPropertyItems::DetectedSportsSession
                }
                "exceedsBaseline" => TimelineSessionActivityInfoPropertyItems::ExceedsBaseline,
                "external" => TimelineSessionActivityInfoPropertyItems::External,
                "filler" => TimelineSessionActivityInfoPropertyItems::Filler,
                "insignificant" => TimelineSessionActivityInfoPropertyItems::Insignificant,
                "stronglyExceedsBaseline" => {
                    TimelineSessionActivityInfoPropertyItems::StronglyExceedsBaseline
                }
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::field_selector::FieldSelector for TimelineSessionActivityInfoPropertyItems {
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
    pub struct TimelineSessionActivityInfo {
        #[doc = "Calories associated with the activity."]
        #[serde(rename = "calories", default)]
        pub calories: ::std::option::Option<f32>,
        #[doc = "The duration of this activity type within the session."]
        #[serde(rename = "durationMillis", default)]
        #[serde(with = "crate::parsed_string")]
        pub duration_millis: ::std::option::Option<i64>,
        #[doc = "The ActivityType and raw detected activities that grouped into this Info."]
        #[serde(rename = "fitnessActivity", default)]
        pub fitness_activity: ::std::option::Option<String>,
        #[doc = "Properties about this activity type that the UI can use to make decisions about hiding or showing this session."]
        #[serde(rename = "property", default)]
        pub property:
            ::std::option::Option<Vec<crate::schemas::TimelineSessionActivityInfoPropertyItems>>,
    }
    impl ::field_selector::FieldSelector for TimelineSessionActivityInfo {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum TimelineSessionLocationPathErrorItems {
        AreaTooDense,
        DistanceNotSupported,
        HighTeleportRate,
        SamplingRateTooLow,
        TooManyGaps,
    }
    impl TimelineSessionLocationPathErrorItems {
        pub fn as_str(self) -> &'static str {
            match self {
                TimelineSessionLocationPathErrorItems::AreaTooDense => "areaTooDense",
                TimelineSessionLocationPathErrorItems::DistanceNotSupported => {
                    "distanceNotSupported"
                }
                TimelineSessionLocationPathErrorItems::HighTeleportRate => "highTeleportRate",
                TimelineSessionLocationPathErrorItems::SamplingRateTooLow => "samplingRateTooLow",
                TimelineSessionLocationPathErrorItems::TooManyGaps => "tooManyGaps",
            }
        }
    }
    impl ::std::fmt::Display for TimelineSessionLocationPathErrorItems {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for TimelineSessionLocationPathErrorItems {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for TimelineSessionLocationPathErrorItems {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "areaTooDense" => TimelineSessionLocationPathErrorItems::AreaTooDense,
                "distanceNotSupported" => {
                    TimelineSessionLocationPathErrorItems::DistanceNotSupported
                }
                "highTeleportRate" => TimelineSessionLocationPathErrorItems::HighTeleportRate,
                "samplingRateTooLow" => TimelineSessionLocationPathErrorItems::SamplingRateTooLow,
                "tooManyGaps" => TimelineSessionLocationPathErrorItems::TooManyGaps,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::field_selector::FieldSelector for TimelineSessionLocationPathErrorItems {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum TimelineSessionLocationSamplingRate {
        High,
        Low,
        Medium,
    }
    impl TimelineSessionLocationSamplingRate {
        pub fn as_str(self) -> &'static str {
            match self {
                TimelineSessionLocationSamplingRate::High => "high",
                TimelineSessionLocationSamplingRate::Low => "low",
                TimelineSessionLocationSamplingRate::Medium => "medium",
            }
        }
    }
    impl ::std::fmt::Display for TimelineSessionLocationSamplingRate {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for TimelineSessionLocationSamplingRate {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for TimelineSessionLocationSamplingRate {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "high" => TimelineSessionLocationSamplingRate::High,
                "low" => TimelineSessionLocationSamplingRate::Low,
                "medium" => TimelineSessionLocationSamplingRate::Medium,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::field_selector::FieldSelector for TimelineSessionLocationSamplingRate {
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
    pub struct TimelineSessionLocation {
        #[serde(rename = "deprecatedField", default)]
        pub deprecated_field: ::std::option::Option<String>,
        #[serde(rename = "latMax", default)]
        pub lat_max: ::std::option::Option<f32>,
        #[doc = "The bounding box."]
        #[serde(rename = "latMin", default)]
        pub lat_min: ::std::option::Option<f32>,
        #[serde(rename = "longMax", default)]
        pub long_max: ::std::option::Option<f32>,
        #[serde(rename = "longMin", default)]
        pub long_min: ::std::option::Option<f32>,
        #[doc = "Issues with the path. You should not show the path in the UX if it has any errors."]
        #[serde(rename = "pathError", default)]
        pub path_error:
            ::std::option::Option<Vec<crate::schemas::TimelineSessionLocationPathErrorItems>>,
        #[doc = "This is the GMS PlaceId that can be used to look up details of this location."]
        #[serde(rename = "placeId", default)]
        pub place_id: ::std::option::Option<String>,
        #[serde(rename = "samplingRate", default)]
        pub sampling_rate:
            ::std::option::Option<crate::schemas::TimelineSessionLocationSamplingRate>,
    }
    impl ::field_selector::FieldSelector for TimelineSessionLocation {
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
    pub struct TimelineSessionSegment {
        #[doc = "The time the last activity segment ended."]
        #[serde(rename = "endTimeMillis", default)]
        #[serde(with = "crate::parsed_string")]
        pub end_time_millis: ::std::option::Option<i64>,
        #[serde(rename = "fitnessActivity", default)]
        pub fitness_activity: ::std::option::Option<String>,
        #[doc = "The time the first activity segment started."]
        #[serde(rename = "startTimeMillis", default)]
        #[serde(with = "crate::parsed_string")]
        pub start_time_millis: ::std::option::Option<i64>,
    }
    impl ::field_selector::FieldSelector for TimelineSessionSegment {
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
    pub struct UpdateGoalV2Request {
        #[serde(rename = "newGoal", default)]
        pub new_goal: ::std::option::Option<crate::schemas::GoalV2>,
        #[serde(rename = "oldGoal", default)]
        pub old_goal: ::std::option::Option<crate::schemas::GoalV2>,
    }
    impl ::field_selector::FieldSelector for UpdateGoalV2Request {
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
    pub struct ValidateChecksumRequest {
        #[serde(rename = "container", default)]
        pub container: ::std::option::Option<Vec<crate::schemas::ChecksumChunkContainer>>,
        #[serde(rename = "elapsedTimeMillis", default)]
        #[serde(with = "crate::parsed_string")]
        pub elapsed_time_millis: ::std::option::Option<i64>,
    }
    impl ::field_selector::FieldSelector for ValidateChecksumRequest {
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
        #[doc = "Floating point value. When this is set, other values must not be set."]
        #[serde(rename = "fpVal", default)]
        pub fp_val: ::std::option::Option<f64>,
        #[doc = "Integer value. When this is set, other values must not be set."]
        #[serde(rename = "intVal", default)]
        pub int_val: ::std::option::Option<i32>,
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
    #[doc = "Actions that can be performed on the fitness_app_private resource"]
    pub fn fitness_app_private(
        &self,
    ) -> crate::resources::fitness_app_private::FitnessAppPrivateActions<A> {
        crate::resources::fitness_app_private::FitnessAppPrivateActions {
            reqwest: &self.reqwest,
            auth: &self.auth,
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
    pub mod fitness_app_private {
        pub mod params {}
        pub struct FitnessAppPrivateActions<'a, A> {
            pub(crate) reqwest: &'a reqwest::Client,
            pub(crate) auth: &'a std::sync::Mutex<A>,
        }
        impl<'a, A: yup_oauth2::GetToken> FitnessAppPrivateActions<'a, A> {
            #[doc = "Get data for display in the web frontend."]
            pub fn gettimelinedata(
                &self,
                request: crate::schemas::GetTimelineDataRequest,
            ) -> GettimelinedataRequestBuilder<A> {
                GettimelinedataRequestBuilder {
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
                }
            }
        }
        #[derive(Debug, Clone)]
        pub struct GettimelinedataRequestBuilder<'a, A> {
            pub(crate) reqwest: &'a ::reqwest::Client,
            pub(crate) auth: &'a ::std::sync::Mutex<A>,
            request: crate::schemas::GetTimelineDataRequest,
            alt: Option<crate::params::Alt>,
            fields: Option<String>,
            key: Option<String>,
            oauth_token: Option<String>,
            pretty_print: Option<bool>,
            quota_user: Option<String>,
            user_ip: Option<String>,
        }
        impl<'a, A: yup_oauth2::GetToken> GettimelinedataRequestBuilder<'a, A> {
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
                    "https://www.googleapis.com/fitness_internal/v0dogfood/".to_owned();
                output.push_str("fitnessAppPrivate/gettimelinedata");
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
    }
    pub mod users {
        pub mod params {}
        pub struct UsersActions<'a, A> {
            pub(crate) reqwest: &'a reqwest::Client,
            pub(crate) auth: &'a std::sync::Mutex<A>,
        }
        impl<'a, A: yup_oauth2::GetToken> UsersActions<'a, A> {
            #[doc = "Validate checksums"]
            pub fn checksum(
                &self,
                request: crate::schemas::ValidateChecksumRequest,
                user_id: impl Into<String>,
            ) -> ChecksumRequestBuilder<A> {
                ChecksumRequestBuilder {
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
            pub fn get_profile(&self, user_id: impl Into<String>) -> GetProfileRequestBuilder<A> {
                GetProfileRequestBuilder {
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
                }
            }
            #[doc = ""]
            pub fn update_delete_history(
                &self,
                user_id: impl Into<String>,
            ) -> UpdateDeleteHistoryRequestBuilder<A> {
                UpdateDeleteHistoryRequestBuilder {
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
                    also_delete_platform_data: None,
                    end_time_millis: None,
                    start_time_millis: None,
                }
            }
            #[doc = ""]
            pub fn update_onboard_new_user(
                &self,
                request: crate::schemas::Profile,
                user_id: impl Into<String>,
            ) -> UpdateOnboardNewUserRequestBuilder<A> {
                UpdateOnboardNewUserRequestBuilder {
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
                    create_default_goal: None,
                }
            }
            #[doc = ""]
            pub fn update_profile(
                &self,
                request: crate::schemas::Profile,
                user_id: impl Into<String>,
            ) -> UpdateProfileRequestBuilder<A> {
                UpdateProfileRequestBuilder {
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
            #[doc = "Actions that can be performed on the capsules resource"]
            pub fn capsules(&self) -> crate::resources::users::capsules::CapsulesActions<A> {
                crate::resources::users::capsules::CapsulesActions {
                    reqwest: &self.reqwest,
                    auth: &self.auth,
                }
            }
            #[doc = "Actions that can be performed on the goals resource"]
            pub fn goals(&self) -> crate::resources::users::goals::GoalsActions<A> {
                crate::resources::users::goals::GoalsActions {
                    reqwest: &self.reqwest,
                    auth: &self.auth,
                }
            }
            #[doc = "Actions that can be performed on the goalsv_2 resource"]
            pub fn goalsv_2(&self) -> crate::resources::users::goalsv_2::Goalsv2Actions<A> {
                crate::resources::users::goalsv_2::Goalsv2Actions {
                    reqwest: &self.reqwest,
                    auth: &self.auth,
                }
            }
            #[doc = "Actions that can be performed on the notifications resource"]
            pub fn notifications(
                &self,
            ) -> crate::resources::users::notifications::NotificationsActions<A> {
                crate::resources::users::notifications::NotificationsActions {
                    reqwest: &self.reqwest,
                    auth: &self.auth,
                }
            }
            #[doc = "Actions that can be performed on the notifications_v_2 resource"]
            pub fn notifications_v_2(
                &self,
            ) -> crate::resources::users::notifications_v_2::NotificationsV2Actions<A> {
                crate::resources::users::notifications_v_2::NotificationsV2Actions {
                    reqwest: &self.reqwest,
                    auth: &self.auth,
                }
            }
            #[doc = "Actions that can be performed on the sessions resource"]
            pub fn sessions(&self) -> crate::resources::users::sessions::SessionsActions<A> {
                crate::resources::users::sessions::SessionsActions {
                    reqwest: &self.reqwest,
                    auth: &self.auth,
                }
            }
        }
        #[derive(Debug, Clone)]
        pub struct ChecksumRequestBuilder<'a, A> {
            pub(crate) reqwest: &'a ::reqwest::Client,
            pub(crate) auth: &'a ::std::sync::Mutex<A>,
            request: crate::schemas::ValidateChecksumRequest,
            user_id: String,
            alt: Option<crate::params::Alt>,
            fields: Option<String>,
            key: Option<String>,
            oauth_token: Option<String>,
            pretty_print: Option<bool>,
            quota_user: Option<String>,
            user_ip: Option<String>,
        }
        impl<'a, A: yup_oauth2::GetToken> ChecksumRequestBuilder<'a, A> {
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
                    "https://www.googleapis.com/fitness_internal/v0dogfood/".to_owned();
                output.push_str("users/");
                {
                    let var_as_str = &self.user_id;
                    output.extend(::percent_encoding::utf8_percent_encode(
                        &var_as_str,
                        crate::SIMPLE,
                    ));
                }
                output.push_str("/checksum");
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
        pub struct GetProfileRequestBuilder<'a, A> {
            pub(crate) reqwest: &'a ::reqwest::Client,
            pub(crate) auth: &'a ::std::sync::Mutex<A>,
            user_id: String,
            alt: Option<crate::params::Alt>,
            fields: Option<String>,
            key: Option<String>,
            oauth_token: Option<String>,
            pretty_print: Option<bool>,
            quota_user: Option<String>,
            user_ip: Option<String>,
        }
        impl<'a, A: yup_oauth2::GetToken> GetProfileRequestBuilder<'a, A> {
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
            ) -> Result<crate::schemas::Profile, Box<dyn ::std::error::Error>> {
                self.execute_fields::<_, &str>(None)
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub fn execute_debug(
                self,
            ) -> Result<crate::schemas::Profile, Box<dyn ::std::error::Error>> {
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
                    "https://www.googleapis.com/fitness_internal/v0dogfood/".to_owned();
                output.push_str("users/");
                {
                    let var_as_str = &self.user_id;
                    output.extend(::percent_encoding::utf8_percent_encode(
                        &var_as_str,
                        crate::SIMPLE,
                    ));
                }
                output.push_str("/profile");
                output
            }
            fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
                let req = self.reqwest.request(::reqwest::Method::GET, path);
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
        pub struct UpdateDeleteHistoryRequestBuilder<'a, A> {
            pub(crate) reqwest: &'a ::reqwest::Client,
            pub(crate) auth: &'a ::std::sync::Mutex<A>,
            user_id: String,
            also_delete_platform_data: Option<bool>,
            end_time_millis: Option<String>,
            start_time_millis: Option<String>,
            alt: Option<crate::params::Alt>,
            fields: Option<String>,
            key: Option<String>,
            oauth_token: Option<String>,
            pretty_print: Option<bool>,
            quota_user: Option<String>,
            user_ip: Option<String>,
        }
        impl<'a, A: yup_oauth2::GetToken> UpdateDeleteHistoryRequestBuilder<'a, A> {
            #[doc = ""]
            pub fn also_delete_platform_data(mut self, value: bool) -> Self {
                self.also_delete_platform_data = Some(value);
                self
            }
            #[doc = "Deprecated. This field doesn't have any effect on history deletion."]
            pub fn end_time_millis(mut self, value: impl Into<String>) -> Self {
                self.end_time_millis = Some(value.into());
                self
            }
            #[doc = "Deprecated. This field doesn't have any effect on history deletion."]
            pub fn start_time_millis(mut self, value: impl Into<String>) -> Self {
                self.start_time_millis = Some(value.into());
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
            pub fn execute(self) -> Result<(), Box<dyn ::std::error::Error>> {
                let req = self._request(&self._path());
                req.send()?.error_for_status()?;
                Ok(())
            }
            fn _path(&self) -> String {
                let mut output =
                    "https://www.googleapis.com/fitness_internal/v0dogfood/".to_owned();
                output.push_str("users/");
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
                let req = self.reqwest.request(::reqwest::Method::PUT, path);
                let req = req.query(&[("alsoDeletePlatformData", &self.also_delete_platform_data)]);
                let req = req.query(&[("endTimeMillis", &self.end_time_millis)]);
                let req = req.query(&[("startTimeMillis", &self.start_time_millis)]);
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
        pub struct UpdateOnboardNewUserRequestBuilder<'a, A> {
            pub(crate) reqwest: &'a ::reqwest::Client,
            pub(crate) auth: &'a ::std::sync::Mutex<A>,
            request: crate::schemas::Profile,
            user_id: String,
            create_default_goal: Option<bool>,
            alt: Option<crate::params::Alt>,
            fields: Option<String>,
            key: Option<String>,
            oauth_token: Option<String>,
            pretty_print: Option<bool>,
            quota_user: Option<String>,
            user_ip: Option<String>,
        }
        impl<'a, A: yup_oauth2::GetToken> UpdateOnboardNewUserRequestBuilder<'a, A> {
            #[doc = ""]
            pub fn create_default_goal(mut self, value: bool) -> Self {
                self.create_default_goal = Some(value);
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
            ) -> Result<crate::schemas::Profile, Box<dyn ::std::error::Error>> {
                self.execute_fields::<_, &str>(None)
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub fn execute_debug(
                self,
            ) -> Result<crate::schemas::Profile, Box<dyn ::std::error::Error>> {
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
                    "https://www.googleapis.com/fitness_internal/v0dogfood/".to_owned();
                output.push_str("users/");
                {
                    let var_as_str = &self.user_id;
                    output.extend(::percent_encoding::utf8_percent_encode(
                        &var_as_str,
                        crate::SIMPLE,
                    ));
                }
                output.push_str("/onboardNewUser");
                output
            }
            fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
                let req = self.reqwest.request(::reqwest::Method::PUT, path);
                let req = req.query(&[("createDefaultGoal", &self.create_default_goal)]);
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
        pub struct UpdateProfileRequestBuilder<'a, A> {
            pub(crate) reqwest: &'a ::reqwest::Client,
            pub(crate) auth: &'a ::std::sync::Mutex<A>,
            request: crate::schemas::Profile,
            user_id: String,
            alt: Option<crate::params::Alt>,
            fields: Option<String>,
            key: Option<String>,
            oauth_token: Option<String>,
            pretty_print: Option<bool>,
            quota_user: Option<String>,
            user_ip: Option<String>,
        }
        impl<'a, A: yup_oauth2::GetToken> UpdateProfileRequestBuilder<'a, A> {
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
            ) -> Result<crate::schemas::Profile, Box<dyn ::std::error::Error>> {
                self.execute_fields::<_, &str>(None)
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub fn execute_debug(
                self,
            ) -> Result<crate::schemas::Profile, Box<dyn ::std::error::Error>> {
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
                    "https://www.googleapis.com/fitness_internal/v0dogfood/".to_owned();
                output.push_str("users/");
                {
                    let var_as_str = &self.user_id;
                    output.extend(::percent_encoding::utf8_percent_encode(
                        &var_as_str,
                        crate::SIMPLE,
                    ));
                }
                output.push_str("/profile");
                output
            }
            fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
                let req = self.reqwest.request(::reqwest::Method::PUT, path);
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
        pub mod capsules {
            pub mod params {}
            pub struct CapsulesActions<'a, A> {
                pub(crate) reqwest: &'a reqwest::Client,
                pub(crate) auth: &'a std::sync::Mutex<A>,
            }
            impl<'a, A: yup_oauth2::GetToken> CapsulesActions<'a, A> {
                #[doc = "Deletes a capsule"]
                pub fn delete(
                    &self,
                    user_id: impl Into<String>,
                    capsule_id: impl Into<String>,
                ) -> DeleteRequestBuilder<A> {
                    DeleteRequestBuilder {
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
                        capsule_id: capsule_id.into(),
                    }
                }
                #[doc = "Gets the list of web dashboard capsules."]
                pub fn list(&self, user_id: impl Into<String>) -> ListRequestBuilder<A> {
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
                        should_initialize: None,
                    }
                }
                #[doc = "Updates a capsule"]
                pub fn update(
                    &self,
                    request: crate::schemas::Capsule,
                    user_id: impl Into<String>,
                    capsule_id: impl Into<String>,
                ) -> UpdateRequestBuilder<A> {
                    UpdateRequestBuilder {
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
                        capsule_id: capsule_id.into(),
                    }
                }
            }
            #[derive(Debug, Clone)]
            pub struct DeleteRequestBuilder<'a, A> {
                pub(crate) reqwest: &'a ::reqwest::Client,
                pub(crate) auth: &'a ::std::sync::Mutex<A>,
                user_id: String,
                capsule_id: String,
                alt: Option<crate::params::Alt>,
                fields: Option<String>,
                key: Option<String>,
                oauth_token: Option<String>,
                pretty_print: Option<bool>,
                quota_user: Option<String>,
                user_ip: Option<String>,
            }
            impl<'a, A: yup_oauth2::GetToken> DeleteRequestBuilder<'a, A> {
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
                    req.send()?.error_for_status()?;
                    Ok(())
                }
                fn _path(&self) -> String {
                    let mut output =
                        "https://www.googleapis.com/fitness_internal/v0dogfood/".to_owned();
                    output.push_str("users/");
                    {
                        let var_as_str = &self.user_id;
                        output.extend(::percent_encoding::utf8_percent_encode(
                            &var_as_str,
                            crate::SIMPLE,
                        ));
                    }
                    output.push_str("/capsules/");
                    {
                        let var_as_str = &self.capsule_id;
                        output.extend(::percent_encoding::utf8_percent_encode(
                            &var_as_str,
                            crate::SIMPLE,
                        ));
                    }
                    output
                }
                fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
                    let req = self.reqwest.request(::reqwest::Method::DELETE, path);
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
            pub struct ListRequestBuilder<'a, A> {
                pub(crate) reqwest: &'a ::reqwest::Client,
                pub(crate) auth: &'a ::std::sync::Mutex<A>,
                user_id: String,
                should_initialize: Option<bool>,
                alt: Option<crate::params::Alt>,
                fields: Option<String>,
                key: Option<String>,
                oauth_token: Option<String>,
                pretty_print: Option<bool>,
                quota_user: Option<String>,
                user_ip: Option<String>,
            }
            impl<'a, A: yup_oauth2::GetToken> ListRequestBuilder<'a, A> {
                #[doc = "If there are no capsules, they will be initialized to all available capsules."]
                pub fn should_initialize(mut self, value: bool) -> Self {
                    self.should_initialize = Some(value);
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
                ) -> Result<crate::schemas::ListCapsulesResponse, Box<dyn ::std::error::Error>>
                {
                    self.execute_fields::<_, &str>(None)
                }
                #[doc = r" Execute the given operation. This will provide a `fields`"]
                #[doc = r" selector of `*`. This will include every attribute of the"]
                #[doc = r" response resource and should be limited to use during"]
                #[doc = r" development or debugging."]
                pub fn execute_debug(
                    self,
                ) -> Result<crate::schemas::ListCapsulesResponse, Box<dyn ::std::error::Error>>
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
                    Ok(req.send()?.error_for_status()?.json()?)
                }
                fn _path(&self) -> String {
                    let mut output =
                        "https://www.googleapis.com/fitness_internal/v0dogfood/".to_owned();
                    output.push_str("users/");
                    {
                        let var_as_str = &self.user_id;
                        output.extend(::percent_encoding::utf8_percent_encode(
                            &var_as_str,
                            crate::SIMPLE,
                        ));
                    }
                    output.push_str("/capsules");
                    output
                }
                fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
                    let req = self.reqwest.request(::reqwest::Method::GET, path);
                    let req = req.query(&[("shouldInitialize", &self.should_initialize)]);
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
            pub struct UpdateRequestBuilder<'a, A> {
                pub(crate) reqwest: &'a ::reqwest::Client,
                pub(crate) auth: &'a ::std::sync::Mutex<A>,
                request: crate::schemas::Capsule,
                user_id: String,
                capsule_id: String,
                alt: Option<crate::params::Alt>,
                fields: Option<String>,
                key: Option<String>,
                oauth_token: Option<String>,
                pretty_print: Option<bool>,
                quota_user: Option<String>,
                user_ip: Option<String>,
            }
            impl<'a, A: yup_oauth2::GetToken> UpdateRequestBuilder<'a, A> {
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
                ) -> Result<crate::schemas::Capsule, Box<dyn ::std::error::Error>> {
                    self.execute_fields::<_, &str>(None)
                }
                #[doc = r" Execute the given operation. This will provide a `fields`"]
                #[doc = r" selector of `*`. This will include every attribute of the"]
                #[doc = r" response resource and should be limited to use during"]
                #[doc = r" development or debugging."]
                pub fn execute_debug(
                    self,
                ) -> Result<crate::schemas::Capsule, Box<dyn ::std::error::Error>> {
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
                        "https://www.googleapis.com/fitness_internal/v0dogfood/".to_owned();
                    output.push_str("users/");
                    {
                        let var_as_str = &self.user_id;
                        output.extend(::percent_encoding::utf8_percent_encode(
                            &var_as_str,
                            crate::SIMPLE,
                        ));
                    }
                    output.push_str("/capsules/");
                    {
                        let var_as_str = &self.capsule_id;
                        output.extend(::percent_encoding::utf8_percent_encode(
                            &var_as_str,
                            crate::SIMPLE,
                        ));
                    }
                    output
                }
                fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
                    let req = self.reqwest.request(::reqwest::Method::PUT, path);
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
        pub mod goals {
            pub mod params {}
            pub struct GoalsActions<'a, A> {
                pub(crate) reqwest: &'a reqwest::Client,
                pub(crate) auth: &'a std::sync::Mutex<A>,
            }
            impl<'a, A: yup_oauth2::GetToken> GoalsActions<'a, A> {
                #[doc = ""]
                pub fn create(
                    &self,
                    request: crate::schemas::Goal,
                    user_id: impl Into<String>,
                ) -> CreateRequestBuilder<A> {
                    CreateRequestBuilder {
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
                pub fn list(&self, user_id: impl Into<String>) -> ListRequestBuilder<A> {
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
                        changed_since_millis: None,
                        include_cancelled: None,
                        include_completed: None,
                    }
                }
                #[doc = ""]
                pub fn update(
                    &self,
                    request: crate::schemas::Goal,
                    user_id: impl Into<String>,
                    goal_id: impl Into<String>,
                ) -> UpdateRequestBuilder<A> {
                    UpdateRequestBuilder {
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
                        goal_id: goal_id.into(),
                    }
                }
            }
            #[derive(Debug, Clone)]
            pub struct CreateRequestBuilder<'a, A> {
                pub(crate) reqwest: &'a ::reqwest::Client,
                pub(crate) auth: &'a ::std::sync::Mutex<A>,
                request: crate::schemas::Goal,
                user_id: String,
                alt: Option<crate::params::Alt>,
                fields: Option<String>,
                key: Option<String>,
                oauth_token: Option<String>,
                pretty_print: Option<bool>,
                quota_user: Option<String>,
                user_ip: Option<String>,
            }
            impl<'a, A: yup_oauth2::GetToken> CreateRequestBuilder<'a, A> {
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
                ) -> Result<crate::schemas::Goal, Box<dyn ::std::error::Error>> {
                    self.execute_fields::<_, &str>(None)
                }
                #[doc = r" Execute the given operation. This will provide a `fields`"]
                #[doc = r" selector of `*`. This will include every attribute of the"]
                #[doc = r" response resource and should be limited to use during"]
                #[doc = r" development or debugging."]
                pub fn execute_debug(
                    self,
                ) -> Result<crate::schemas::Goal, Box<dyn ::std::error::Error>> {
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
                        "https://www.googleapis.com/fitness_internal/v0dogfood/".to_owned();
                    output.push_str("users/");
                    {
                        let var_as_str = &self.user_id;
                        output.extend(::percent_encoding::utf8_percent_encode(
                            &var_as_str,
                            crate::SIMPLE,
                        ));
                    }
                    output.push_str("/goals");
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
            pub struct ListRequestBuilder<'a, A> {
                pub(crate) reqwest: &'a ::reqwest::Client,
                pub(crate) auth: &'a ::std::sync::Mutex<A>,
                user_id: String,
                changed_since_millis: Option<String>,
                include_cancelled: Option<bool>,
                include_completed: Option<bool>,
                alt: Option<crate::params::Alt>,
                fields: Option<String>,
                key: Option<String>,
                oauth_token: Option<String>,
                pretty_print: Option<bool>,
                quota_user: Option<String>,
                user_ip: Option<String>,
            }
            impl<'a, A: yup_oauth2::GetToken> ListRequestBuilder<'a, A> {
                #[doc = ""]
                pub fn changed_since_millis(mut self, value: impl Into<String>) -> Self {
                    self.changed_since_millis = Some(value.into());
                    self
                }
                #[doc = ""]
                pub fn include_cancelled(mut self, value: bool) -> Self {
                    self.include_cancelled = Some(value);
                    self
                }
                #[doc = ""]
                pub fn include_completed(mut self, value: bool) -> Self {
                    self.include_completed = Some(value);
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
                ) -> Result<crate::schemas::ListGoalsResponse, Box<dyn ::std::error::Error>>
                {
                    self.execute_fields::<_, &str>(None)
                }
                #[doc = r" Execute the given operation. This will provide a `fields`"]
                #[doc = r" selector of `*`. This will include every attribute of the"]
                #[doc = r" response resource and should be limited to use during"]
                #[doc = r" development or debugging."]
                pub fn execute_debug(
                    self,
                ) -> Result<crate::schemas::ListGoalsResponse, Box<dyn ::std::error::Error>>
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
                    Ok(req.send()?.error_for_status()?.json()?)
                }
                fn _path(&self) -> String {
                    let mut output =
                        "https://www.googleapis.com/fitness_internal/v0dogfood/".to_owned();
                    output.push_str("users/");
                    {
                        let var_as_str = &self.user_id;
                        output.extend(::percent_encoding::utf8_percent_encode(
                            &var_as_str,
                            crate::SIMPLE,
                        ));
                    }
                    output.push_str("/goals");
                    output
                }
                fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
                    let req = self.reqwest.request(::reqwest::Method::GET, path);
                    let req = req.query(&[("changedSinceMillis", &self.changed_since_millis)]);
                    let req = req.query(&[("includeCancelled", &self.include_cancelled)]);
                    let req = req.query(&[("includeCompleted", &self.include_completed)]);
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
            pub struct UpdateRequestBuilder<'a, A> {
                pub(crate) reqwest: &'a ::reqwest::Client,
                pub(crate) auth: &'a ::std::sync::Mutex<A>,
                request: crate::schemas::Goal,
                user_id: String,
                goal_id: String,
                alt: Option<crate::params::Alt>,
                fields: Option<String>,
                key: Option<String>,
                oauth_token: Option<String>,
                pretty_print: Option<bool>,
                quota_user: Option<String>,
                user_ip: Option<String>,
            }
            impl<'a, A: yup_oauth2::GetToken> UpdateRequestBuilder<'a, A> {
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
                ) -> Result<crate::schemas::Goal, Box<dyn ::std::error::Error>> {
                    self.execute_fields::<_, &str>(None)
                }
                #[doc = r" Execute the given operation. This will provide a `fields`"]
                #[doc = r" selector of `*`. This will include every attribute of the"]
                #[doc = r" response resource and should be limited to use during"]
                #[doc = r" development or debugging."]
                pub fn execute_debug(
                    self,
                ) -> Result<crate::schemas::Goal, Box<dyn ::std::error::Error>> {
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
                        "https://www.googleapis.com/fitness_internal/v0dogfood/".to_owned();
                    output.push_str("users/");
                    {
                        let var_as_str = &self.user_id;
                        output.extend(::percent_encoding::utf8_percent_encode(
                            &var_as_str,
                            crate::SIMPLE,
                        ));
                    }
                    output.push_str("/goals/");
                    {
                        let var_as_str = &self.goal_id;
                        output.extend(::percent_encoding::utf8_percent_encode(
                            &var_as_str,
                            crate::SIMPLE,
                        ));
                    }
                    output
                }
                fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
                    let req = self.reqwest.request(::reqwest::Method::PUT, path);
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
        pub mod goalsv_2 {
            pub mod params {}
            pub struct Goalsv2Actions<'a, A> {
                pub(crate) reqwest: &'a reqwest::Client,
                pub(crate) auth: &'a std::sync::Mutex<A>,
            }
            impl<'a, A: yup_oauth2::GetToken> Goalsv2Actions<'a, A> {
                #[doc = "Create a goalv2"]
                pub fn create(
                    &self,
                    request: crate::schemas::GoalV2,
                    user_id: impl Into<String>,
                ) -> CreateRequestBuilder<A> {
                    CreateRequestBuilder {
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
                #[doc = "Calls Glycogen to get a list of all active V2 goals."]
                pub fn list(&self, user_id: impl Into<String>) -> ListRequestBuilder<A> {
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
                    }
                }
                #[doc = "Cancel a goalv2"]
                pub fn update_cancel(
                    &self,
                    request: crate::schemas::GoalV2,
                    user_id: impl Into<String>,
                ) -> UpdateCancelRequestBuilder<A> {
                    UpdateCancelRequestBuilder {
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
                        cancel_time_nanos: None,
                    }
                }
                #[doc = "Cancel a goalv2 and create another goalv2"]
                pub fn update_cancel_create(
                    &self,
                    request: crate::schemas::UpdateGoalV2Request,
                    user_id: impl Into<String>,
                ) -> UpdateCancelCreateRequestBuilder<A> {
                    UpdateCancelCreateRequestBuilder {
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
            }
            #[derive(Debug, Clone)]
            pub struct CreateRequestBuilder<'a, A> {
                pub(crate) reqwest: &'a ::reqwest::Client,
                pub(crate) auth: &'a ::std::sync::Mutex<A>,
                request: crate::schemas::GoalV2,
                user_id: String,
                alt: Option<crate::params::Alt>,
                fields: Option<String>,
                key: Option<String>,
                oauth_token: Option<String>,
                pretty_print: Option<bool>,
                quota_user: Option<String>,
                user_ip: Option<String>,
            }
            impl<'a, A: yup_oauth2::GetToken> CreateRequestBuilder<'a, A> {
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
                ) -> Result<crate::schemas::GoalV2, Box<dyn ::std::error::Error>> {
                    self.execute_fields::<_, &str>(None)
                }
                #[doc = r" Execute the given operation. This will provide a `fields`"]
                #[doc = r" selector of `*`. This will include every attribute of the"]
                #[doc = r" response resource and should be limited to use during"]
                #[doc = r" development or debugging."]
                pub fn execute_debug(
                    self,
                ) -> Result<crate::schemas::GoalV2, Box<dyn ::std::error::Error>> {
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
                        "https://www.googleapis.com/fitness_internal/v0dogfood/".to_owned();
                    output.push_str("users/");
                    {
                        let var_as_str = &self.user_id;
                        output.extend(::percent_encoding::utf8_percent_encode(
                            &var_as_str,
                            crate::SIMPLE,
                        ));
                    }
                    output.push_str("/goalsv2");
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
            pub struct ListRequestBuilder<'a, A> {
                pub(crate) reqwest: &'a ::reqwest::Client,
                pub(crate) auth: &'a ::std::sync::Mutex<A>,
                user_id: String,
                alt: Option<crate::params::Alt>,
                fields: Option<String>,
                key: Option<String>,
                oauth_token: Option<String>,
                pretty_print: Option<bool>,
                quota_user: Option<String>,
                user_ip: Option<String>,
            }
            impl<'a, A: yup_oauth2::GetToken> ListRequestBuilder<'a, A> {
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
                ) -> Result<crate::schemas::ListGoalsV2Response, Box<dyn ::std::error::Error>>
                {
                    self.execute_fields::<_, &str>(None)
                }
                #[doc = r" Execute the given operation. This will provide a `fields`"]
                #[doc = r" selector of `*`. This will include every attribute of the"]
                #[doc = r" response resource and should be limited to use during"]
                #[doc = r" development or debugging."]
                pub fn execute_debug(
                    self,
                ) -> Result<crate::schemas::ListGoalsV2Response, Box<dyn ::std::error::Error>>
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
                    Ok(req.send()?.error_for_status()?.json()?)
                }
                fn _path(&self) -> String {
                    let mut output =
                        "https://www.googleapis.com/fitness_internal/v0dogfood/".to_owned();
                    output.push_str("users/");
                    {
                        let var_as_str = &self.user_id;
                        output.extend(::percent_encoding::utf8_percent_encode(
                            &var_as_str,
                            crate::SIMPLE,
                        ));
                    }
                    output.push_str("/goalsv2");
                    output
                }
                fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
                    let req = self.reqwest.request(::reqwest::Method::GET, path);
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
            pub struct UpdateCancelRequestBuilder<'a, A> {
                pub(crate) reqwest: &'a ::reqwest::Client,
                pub(crate) auth: &'a ::std::sync::Mutex<A>,
                request: crate::schemas::GoalV2,
                user_id: String,
                cancel_time_nanos: Option<i64>,
                alt: Option<crate::params::Alt>,
                fields: Option<String>,
                key: Option<String>,
                oauth_token: Option<String>,
                pretty_print: Option<bool>,
                quota_user: Option<String>,
                user_ip: Option<String>,
            }
            impl<'a, A: yup_oauth2::GetToken> UpdateCancelRequestBuilder<'a, A> {
                #[doc = "New effective end time of the goal, defaults to now if unset."]
                pub fn cancel_time_nanos(mut self, value: i64) -> Self {
                    self.cancel_time_nanos = Some(value);
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
                pub fn execute(self) -> Result<(), Box<dyn ::std::error::Error>> {
                    let req = self._request(&self._path());
                    let req = req.json(&self.request);
                    req.send()?.error_for_status()?;
                    Ok(())
                }
                fn _path(&self) -> String {
                    let mut output =
                        "https://www.googleapis.com/fitness_internal/v0dogfood/".to_owned();
                    output.push_str("users/");
                    {
                        let var_as_str = &self.user_id;
                        output.extend(::percent_encoding::utf8_percent_encode(
                            &var_as_str,
                            crate::SIMPLE,
                        ));
                    }
                    output.push_str("/goalsv2/cancel");
                    output
                }
                fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
                    let req = self.reqwest.request(::reqwest::Method::PUT, path);
                    let req = req.query(&[("cancelTimeNanos", &self.cancel_time_nanos)]);
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
            pub struct UpdateCancelCreateRequestBuilder<'a, A> {
                pub(crate) reqwest: &'a ::reqwest::Client,
                pub(crate) auth: &'a ::std::sync::Mutex<A>,
                request: crate::schemas::UpdateGoalV2Request,
                user_id: String,
                alt: Option<crate::params::Alt>,
                fields: Option<String>,
                key: Option<String>,
                oauth_token: Option<String>,
                pretty_print: Option<bool>,
                quota_user: Option<String>,
                user_ip: Option<String>,
            }
            impl<'a, A: yup_oauth2::GetToken> UpdateCancelCreateRequestBuilder<'a, A> {
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
                ) -> Result<crate::schemas::GoalV2, Box<dyn ::std::error::Error>> {
                    self.execute_fields::<_, &str>(None)
                }
                #[doc = r" Execute the given operation. This will provide a `fields`"]
                #[doc = r" selector of `*`. This will include every attribute of the"]
                #[doc = r" response resource and should be limited to use during"]
                #[doc = r" development or debugging."]
                pub fn execute_debug(
                    self,
                ) -> Result<crate::schemas::GoalV2, Box<dyn ::std::error::Error>> {
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
                        "https://www.googleapis.com/fitness_internal/v0dogfood/".to_owned();
                    output.push_str("users/");
                    {
                        let var_as_str = &self.user_id;
                        output.extend(::percent_encoding::utf8_percent_encode(
                            &var_as_str,
                            crate::SIMPLE,
                        ));
                    }
                    output.push_str("/goalsv2/cancelCreate");
                    output
                }
                fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
                    let req = self.reqwest.request(::reqwest::Method::PUT, path);
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
        pub mod notifications {
            pub mod params {
                #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
                pub enum ListStatus {
                    Accepted,
                    Blocked,
                    DeprecatedComplete,
                    Dismissed,
                    LocalOnly,
                    NoStatus,
                    SystemCancel,
                    Visible,
                }
                impl ListStatus {
                    pub fn as_str(self) -> &'static str {
                        match self {
                            ListStatus::Accepted => "accepted",
                            ListStatus::Blocked => "blocked",
                            ListStatus::DeprecatedComplete => "deprecatedComplete",
                            ListStatus::Dismissed => "dismissed",
                            ListStatus::LocalOnly => "localOnly",
                            ListStatus::NoStatus => "noStatus",
                            ListStatus::SystemCancel => "systemCancel",
                            ListStatus::Visible => "visible",
                        }
                    }
                }
                impl ::std::fmt::Display for ListStatus {
                    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                        f.write_str(self.as_str())
                    }
                }
                impl ::serde::Serialize for ListStatus {
                    fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
                    where
                        S: ::serde::ser::Serializer,
                    {
                        serializer.serialize_str(self.as_str())
                    }
                }
                impl<'de> ::serde::Deserialize<'de> for ListStatus {
                    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
                    where
                        D: ::serde::de::Deserializer<'de>,
                    {
                        let value: &'de str = <&str>::deserialize(deserializer)?;
                        Ok(match value {
                            "accepted" => ListStatus::Accepted,
                            "blocked" => ListStatus::Blocked,
                            "deprecatedComplete" => ListStatus::DeprecatedComplete,
                            "dismissed" => ListStatus::Dismissed,
                            "localOnly" => ListStatus::LocalOnly,
                            "noStatus" => ListStatus::NoStatus,
                            "systemCancel" => ListStatus::SystemCancel,
                            "visible" => ListStatus::Visible,
                            _ => {
                                return Err(::serde::de::Error::custom(format!(
                                    "invalid enum for #name: {}",
                                    value
                                )))
                            }
                        })
                    }
                }
                impl ::field_selector::FieldSelector for ListStatus {
                    fn field_selector_with_ident(ident: &str, selector: &mut String) {
                        match selector.chars().rev().nth(0) {
                            Some(',') | None => {}
                            _ => selector.push_str(","),
                        }
                        selector.push_str(ident);
                    }
                }
            }
            pub struct NotificationsActions<'a, A> {
                pub(crate) reqwest: &'a reqwest::Client,
                pub(crate) auth: &'a std::sync::Mutex<A>,
            }
            impl<'a, A: yup_oauth2::GetToken> NotificationsActions<'a, A> {
                #[doc = ""]
                pub fn create(
                    &self,
                    request: crate::schemas::Notification,
                    user_id: impl Into<String>,
                ) -> CreateRequestBuilder<A> {
                    CreateRequestBuilder {
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
                pub fn get_status(
                    &self,
                    user_id: impl Into<String>,
                    notification_id: impl Into<String>,
                ) -> GetStatusRequestBuilder<A> {
                    GetStatusRequestBuilder {
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
                        notification_id: notification_id.into(),
                    }
                }
                #[doc = "Returns all notifications for a user. If the 'status' parameter is set, only the notifications with a matching status will be returned."]
                pub fn list(&self, user_id: impl Into<String>) -> ListRequestBuilder<A> {
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
                        force_generate_source_names: None,
                        include_stored_notifications: None,
                        locale: None,
                        status: None,
                    }
                }
                #[doc = ""]
                pub fn update(
                    &self,
                    request: crate::schemas::Notification,
                    user_id: impl Into<String>,
                    notification_id: impl Into<String>,
                ) -> UpdateRequestBuilder<A> {
                    UpdateRequestBuilder {
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
                        notification_id: notification_id.into(),
                    }
                }
                #[doc = ""]
                pub fn update_record_user_action(
                    &self,
                    request: crate::schemas::NotificationStatus,
                    user_id: impl Into<String>,
                    notification_id: impl Into<String>,
                ) -> UpdateRecordUserActionRequestBuilder<A> {
                    UpdateRecordUserActionRequestBuilder {
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
                        notification_id: notification_id.into(),
                    }
                }
            }
            #[derive(Debug, Clone)]
            pub struct CreateRequestBuilder<'a, A> {
                pub(crate) reqwest: &'a ::reqwest::Client,
                pub(crate) auth: &'a ::std::sync::Mutex<A>,
                request: crate::schemas::Notification,
                user_id: String,
                alt: Option<crate::params::Alt>,
                fields: Option<String>,
                key: Option<String>,
                oauth_token: Option<String>,
                pretty_print: Option<bool>,
                quota_user: Option<String>,
                user_ip: Option<String>,
            }
            impl<'a, A: yup_oauth2::GetToken> CreateRequestBuilder<'a, A> {
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
                ) -> Result<crate::schemas::Notification, Box<dyn ::std::error::Error>>
                {
                    self.execute_fields::<_, &str>(None)
                }
                #[doc = r" Execute the given operation. This will provide a `fields`"]
                #[doc = r" selector of `*`. This will include every attribute of the"]
                #[doc = r" response resource and should be limited to use during"]
                #[doc = r" development or debugging."]
                pub fn execute_debug(
                    self,
                ) -> Result<crate::schemas::Notification, Box<dyn ::std::error::Error>>
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
                        "https://www.googleapis.com/fitness_internal/v0dogfood/".to_owned();
                    output.push_str("users/");
                    {
                        let var_as_str = &self.user_id;
                        output.extend(::percent_encoding::utf8_percent_encode(
                            &var_as_str,
                            crate::SIMPLE,
                        ));
                    }
                    output.push_str("/notifications");
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
            pub struct GetStatusRequestBuilder<'a, A> {
                pub(crate) reqwest: &'a ::reqwest::Client,
                pub(crate) auth: &'a ::std::sync::Mutex<A>,
                user_id: String,
                notification_id: String,
                alt: Option<crate::params::Alt>,
                fields: Option<String>,
                key: Option<String>,
                oauth_token: Option<String>,
                pretty_print: Option<bool>,
                quota_user: Option<String>,
                user_ip: Option<String>,
            }
            impl<'a, A: yup_oauth2::GetToken> GetStatusRequestBuilder<'a, A> {
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
                ) -> Result<crate::schemas::NotificationStatus, Box<dyn ::std::error::Error>>
                {
                    self.execute_fields::<_, &str>(None)
                }
                #[doc = r" Execute the given operation. This will provide a `fields`"]
                #[doc = r" selector of `*`. This will include every attribute of the"]
                #[doc = r" response resource and should be limited to use during"]
                #[doc = r" development or debugging."]
                pub fn execute_debug(
                    self,
                ) -> Result<crate::schemas::NotificationStatus, Box<dyn ::std::error::Error>>
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
                    Ok(req.send()?.error_for_status()?.json()?)
                }
                fn _path(&self) -> String {
                    let mut output =
                        "https://www.googleapis.com/fitness_internal/v0dogfood/".to_owned();
                    output.push_str("users/");
                    {
                        let var_as_str = &self.user_id;
                        output.extend(::percent_encoding::utf8_percent_encode(
                            &var_as_str,
                            crate::SIMPLE,
                        ));
                    }
                    output.push_str("/notifications/");
                    {
                        let var_as_str = &self.notification_id;
                        output.extend(::percent_encoding::utf8_percent_encode(
                            &var_as_str,
                            crate::SIMPLE,
                        ));
                    }
                    output.push_str("/status");
                    output
                }
                fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
                    let req = self.reqwest.request(::reqwest::Method::GET, path);
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
            pub struct ListRequestBuilder<'a, A> {
                pub(crate) reqwest: &'a ::reqwest::Client,
                pub(crate) auth: &'a ::std::sync::Mutex<A>,
                user_id: String,
                force_generate_source_names: Option<Vec<String>>,
                include_stored_notifications: Option<bool>,
                locale: Option<String>,
                status: Option<crate::resources::users::notifications::params::ListStatus>,
                alt: Option<crate::params::Alt>,
                fields: Option<String>,
                key: Option<String>,
                oauth_token: Option<String>,
                pretty_print: Option<bool>,
                quota_user: Option<String>,
                user_ip: Option<String>,
            }
            impl<'a, A: yup_oauth2::GetToken> ListRequestBuilder<'a, A> {
                #[doc = ""]
                pub fn force_generate_source_names(
                    mut self,
                    value: impl Into<Vec<String>>,
                ) -> Self {
                    self.force_generate_source_names = Some(value.into());
                    self
                }
                #[doc = "if set, also returns all notifications that have been acted upon by user"]
                pub fn include_stored_notifications(mut self, value: bool) -> Self {
                    self.include_stored_notifications = Some(value);
                    self
                }
                #[doc = "TODO(ricebin): deprecate; use header.accept_language"]
                pub fn locale(mut self, value: impl Into<String>) -> Self {
                    self.locale = Some(value.into());
                    self
                }
                #[doc = ""]
                pub fn status(
                    mut self,
                    value: crate::resources::users::notifications::params::ListStatus,
                ) -> Self {
                    self.status = Some(value);
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
                ) -> Result<crate::schemas::ListNotificationsResponse, Box<dyn ::std::error::Error>>
                {
                    self.execute_fields::<_, &str>(None)
                }
                #[doc = r" Execute the given operation. This will provide a `fields`"]
                #[doc = r" selector of `*`. This will include every attribute of the"]
                #[doc = r" response resource and should be limited to use during"]
                #[doc = r" development or debugging."]
                pub fn execute_debug(
                    self,
                ) -> Result<crate::schemas::ListNotificationsResponse, Box<dyn ::std::error::Error>>
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
                    Ok(req.send()?.error_for_status()?.json()?)
                }
                fn _path(&self) -> String {
                    let mut output =
                        "https://www.googleapis.com/fitness_internal/v0dogfood/".to_owned();
                    output.push_str("users/");
                    {
                        let var_as_str = &self.user_id;
                        output.extend(::percent_encoding::utf8_percent_encode(
                            &var_as_str,
                            crate::SIMPLE,
                        ));
                    }
                    output.push_str("/notifications");
                    output
                }
                fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
                    let req = self.reqwest.request(::reqwest::Method::GET, path);
                    let req = req.query(&[(
                        "forceGenerateSourceNames",
                        &self.force_generate_source_names,
                    )]);
                    let req = req.query(&[(
                        "includeStoredNotifications",
                        &self.include_stored_notifications,
                    )]);
                    let req = req.query(&[("locale", &self.locale)]);
                    let req = req.query(&[("status", &self.status)]);
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
            pub struct UpdateRequestBuilder<'a, A> {
                pub(crate) reqwest: &'a ::reqwest::Client,
                pub(crate) auth: &'a ::std::sync::Mutex<A>,
                request: crate::schemas::Notification,
                user_id: String,
                notification_id: String,
                alt: Option<crate::params::Alt>,
                fields: Option<String>,
                key: Option<String>,
                oauth_token: Option<String>,
                pretty_print: Option<bool>,
                quota_user: Option<String>,
                user_ip: Option<String>,
            }
            impl<'a, A: yup_oauth2::GetToken> UpdateRequestBuilder<'a, A> {
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
                ) -> Result<crate::schemas::Notification, Box<dyn ::std::error::Error>>
                {
                    self.execute_fields::<_, &str>(None)
                }
                #[doc = r" Execute the given operation. This will provide a `fields`"]
                #[doc = r" selector of `*`. This will include every attribute of the"]
                #[doc = r" response resource and should be limited to use during"]
                #[doc = r" development or debugging."]
                pub fn execute_debug(
                    self,
                ) -> Result<crate::schemas::Notification, Box<dyn ::std::error::Error>>
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
                        "https://www.googleapis.com/fitness_internal/v0dogfood/".to_owned();
                    output.push_str("users/");
                    {
                        let var_as_str = &self.user_id;
                        output.extend(::percent_encoding::utf8_percent_encode(
                            &var_as_str,
                            crate::SIMPLE,
                        ));
                    }
                    output.push_str("/notifications/");
                    {
                        let var_as_str = &self.notification_id;
                        output.extend(::percent_encoding::utf8_percent_encode(
                            &var_as_str,
                            crate::SIMPLE,
                        ));
                    }
                    output
                }
                fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
                    let req = self.reqwest.request(::reqwest::Method::PUT, path);
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
            pub struct UpdateRecordUserActionRequestBuilder<'a, A> {
                pub(crate) reqwest: &'a ::reqwest::Client,
                pub(crate) auth: &'a ::std::sync::Mutex<A>,
                request: crate::schemas::NotificationStatus,
                user_id: String,
                notification_id: String,
                alt: Option<crate::params::Alt>,
                fields: Option<String>,
                key: Option<String>,
                oauth_token: Option<String>,
                pretty_print: Option<bool>,
                quota_user: Option<String>,
                user_ip: Option<String>,
            }
            impl<'a, A: yup_oauth2::GetToken> UpdateRecordUserActionRequestBuilder<'a, A> {
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
                        "https://www.googleapis.com/fitness_internal/v0dogfood/".to_owned();
                    output.push_str("users/");
                    {
                        let var_as_str = &self.user_id;
                        output.extend(::percent_encoding::utf8_percent_encode(
                            &var_as_str,
                            crate::SIMPLE,
                        ));
                    }
                    output.push_str("/notifications/");
                    {
                        let var_as_str = &self.notification_id;
                        output.extend(::percent_encoding::utf8_percent_encode(
                            &var_as_str,
                            crate::SIMPLE,
                        ));
                    }
                    output.push_str("/recordUserAction");
                    output
                }
                fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
                    let req = self.reqwest.request(::reqwest::Method::PUT, path);
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
        pub mod notifications_v_2 {
            pub mod params {
                #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
                pub enum ListStatus {
                    Accepted,
                    Blocked,
                    DeprecatedComplete,
                    Dismissed,
                    LocalOnly,
                    NoStatus,
                    SystemCancel,
                    Visible,
                }
                impl ListStatus {
                    pub fn as_str(self) -> &'static str {
                        match self {
                            ListStatus::Accepted => "accepted",
                            ListStatus::Blocked => "blocked",
                            ListStatus::DeprecatedComplete => "deprecatedComplete",
                            ListStatus::Dismissed => "dismissed",
                            ListStatus::LocalOnly => "localOnly",
                            ListStatus::NoStatus => "noStatus",
                            ListStatus::SystemCancel => "systemCancel",
                            ListStatus::Visible => "visible",
                        }
                    }
                }
                impl ::std::fmt::Display for ListStatus {
                    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                        f.write_str(self.as_str())
                    }
                }
                impl ::serde::Serialize for ListStatus {
                    fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
                    where
                        S: ::serde::ser::Serializer,
                    {
                        serializer.serialize_str(self.as_str())
                    }
                }
                impl<'de> ::serde::Deserialize<'de> for ListStatus {
                    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
                    where
                        D: ::serde::de::Deserializer<'de>,
                    {
                        let value: &'de str = <&str>::deserialize(deserializer)?;
                        Ok(match value {
                            "accepted" => ListStatus::Accepted,
                            "blocked" => ListStatus::Blocked,
                            "deprecatedComplete" => ListStatus::DeprecatedComplete,
                            "dismissed" => ListStatus::Dismissed,
                            "localOnly" => ListStatus::LocalOnly,
                            "noStatus" => ListStatus::NoStatus,
                            "systemCancel" => ListStatus::SystemCancel,
                            "visible" => ListStatus::Visible,
                            _ => {
                                return Err(::serde::de::Error::custom(format!(
                                    "invalid enum for #name: {}",
                                    value
                                )))
                            }
                        })
                    }
                }
                impl ::field_selector::FieldSelector for ListStatus {
                    fn field_selector_with_ident(ident: &str, selector: &mut String) {
                        match selector.chars().rev().nth(0) {
                            Some(',') | None => {}
                            _ => selector.push_str(","),
                        }
                        selector.push_str(ident);
                    }
                }
            }
            pub struct NotificationsV2Actions<'a, A> {
                pub(crate) reqwest: &'a reqwest::Client,
                pub(crate) auth: &'a std::sync::Mutex<A>,
            }
            impl<'a, A: yup_oauth2::GetToken> NotificationsV2Actions<'a, A> {
                #[doc = "Called by Comaneci and newer clients."]
                pub fn list(&self, user_id: impl Into<String>) -> ListRequestBuilder<A> {
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
                        force_generate_source_names: None,
                        include_stored_notifications: None,
                        locale: None,
                        status: None,
                    }
                }
            }
            #[derive(Debug, Clone)]
            pub struct ListRequestBuilder<'a, A> {
                pub(crate) reqwest: &'a ::reqwest::Client,
                pub(crate) auth: &'a ::std::sync::Mutex<A>,
                user_id: String,
                force_generate_source_names: Option<Vec<String>>,
                include_stored_notifications: Option<bool>,
                locale: Option<String>,
                status: Option<crate::resources::users::notifications_v_2::params::ListStatus>,
                alt: Option<crate::params::Alt>,
                fields: Option<String>,
                key: Option<String>,
                oauth_token: Option<String>,
                pretty_print: Option<bool>,
                quota_user: Option<String>,
                user_ip: Option<String>,
            }
            impl<'a, A: yup_oauth2::GetToken> ListRequestBuilder<'a, A> {
                #[doc = ""]
                pub fn force_generate_source_names(
                    mut self,
                    value: impl Into<Vec<String>>,
                ) -> Self {
                    self.force_generate_source_names = Some(value.into());
                    self
                }
                #[doc = "if set, also returns all notifications that have been acted upon by user"]
                pub fn include_stored_notifications(mut self, value: bool) -> Self {
                    self.include_stored_notifications = Some(value);
                    self
                }
                #[doc = "TODO(ricebin): deprecate; use header.accept_language"]
                pub fn locale(mut self, value: impl Into<String>) -> Self {
                    self.locale = Some(value.into());
                    self
                }
                #[doc = ""]
                pub fn status(
                    mut self,
                    value: crate::resources::users::notifications_v_2::params::ListStatus,
                ) -> Self {
                    self.status = Some(value);
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
                ) -> Result<crate::schemas::ListNotificationsResponse, Box<dyn ::std::error::Error>>
                {
                    self.execute_fields::<_, &str>(None)
                }
                #[doc = r" Execute the given operation. This will provide a `fields`"]
                #[doc = r" selector of `*`. This will include every attribute of the"]
                #[doc = r" response resource and should be limited to use during"]
                #[doc = r" development or debugging."]
                pub fn execute_debug(
                    self,
                ) -> Result<crate::schemas::ListNotificationsResponse, Box<dyn ::std::error::Error>>
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
                    Ok(req.send()?.error_for_status()?.json()?)
                }
                fn _path(&self) -> String {
                    let mut output =
                        "https://www.googleapis.com/fitness_internal/v0dogfood/".to_owned();
                    output.push_str("users/");
                    {
                        let var_as_str = &self.user_id;
                        output.extend(::percent_encoding::utf8_percent_encode(
                            &var_as_str,
                            crate::SIMPLE,
                        ));
                    }
                    output.push_str("/notifications_v2");
                    output
                }
                fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
                    let req = self.reqwest.request(::reqwest::Method::GET, path);
                    let req = req.query(&[(
                        "forceGenerateSourceNames",
                        &self.force_generate_source_names,
                    )]);
                    let req = req.query(&[(
                        "includeStoredNotifications",
                        &self.include_stored_notifications,
                    )]);
                    let req = req.query(&[("locale", &self.locale)]);
                    let req = req.query(&[("status", &self.status)]);
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
        pub mod sessions {
            pub mod params {}
            pub struct SessionsActions<'a, A> {
                pub(crate) reqwest: &'a reqwest::Client,
                pub(crate) auth: &'a std::sync::Mutex<A>,
            }
            impl<'a, A: yup_oauth2::GetToken> SessionsActions<'a, A> {
                #[doc = ""]
                pub fn list(&self, user_id: impl Into<String>) -> ListRequestBuilder<A> {
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
                        annotation_data_stream_id: None,
                        annotation_data_type: None,
                        deprecated_include_post_filler_sessions: None,
                        deprecated_include_pre_filler_sessions: None,
                        end_time_millis: None,
                        language_code: None,
                        start_time_millis: None,
                    }
                }
                #[doc = "Deletes sessions (from Glycogen) and segments and inserts new."]
                pub fn update_edit(
                    &self,
                    request: crate::schemas::TimelineSession,
                    user_id: impl Into<String>,
                ) -> UpdateEditRequestBuilder<A> {
                    UpdateEditRequestBuilder {
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
                        end_time_millis: None,
                        hide: None,
                        start_time_millis: None,
                    }
                }
            }
            #[derive(Debug, Clone)]
            pub struct ListRequestBuilder<'a, A> {
                pub(crate) reqwest: &'a ::reqwest::Client,
                pub(crate) auth: &'a ::std::sync::Mutex<A>,
                user_id: String,
                annotation_data_stream_id: Option<Vec<String>>,
                annotation_data_type: Option<Vec<String>>,
                deprecated_include_post_filler_sessions: Option<bool>,
                deprecated_include_pre_filler_sessions: Option<bool>,
                end_time_millis: Option<String>,
                language_code: Option<String>,
                start_time_millis: Option<String>,
                alt: Option<crate::params::Alt>,
                fields: Option<String>,
                key: Option<String>,
                oauth_token: Option<String>,
                pretty_print: Option<bool>,
                quota_user: Option<String>,
                user_ip: Option<String>,
            }
            impl<'a, A: yup_oauth2::GetToken> ListRequestBuilder<'a, A> {
                #[doc = ""]
                pub fn annotation_data_stream_id(mut self, value: impl Into<Vec<String>>) -> Self {
                    self.annotation_data_stream_id = Some(value.into());
                    self
                }
                #[doc = "DEPRECATED, any values supplied will be ignored. see NonBucketingSessionAnnotator or BucketingSessionAnnotator for supported DataTypes com.google.step_count.delta com.google.calories.expended com.google.distance.delta com.google.heart_rate.bpm"]
                pub fn annotation_data_type(mut self, value: impl Into<Vec<String>>) -> Self {
                    self.annotation_data_type = Some(value.into());
                    self
                }
                #[doc = ""]
                pub fn deprecated_include_post_filler_sessions(mut self, value: bool) -> Self {
                    self.deprecated_include_post_filler_sessions = Some(value);
                    self
                }
                #[doc = "DEPRECATED, will always default to true."]
                pub fn deprecated_include_pre_filler_sessions(mut self, value: bool) -> Self {
                    self.deprecated_include_pre_filler_sessions = Some(value);
                    self
                }
                #[doc = "@required"]
                pub fn end_time_millis(mut self, value: impl Into<String>) -> Self {
                    self.end_time_millis = Some(value.into());
                    self
                }
                #[doc = "DEPRECATED, any value supplied will be ignored."]
                pub fn language_code(mut self, value: impl Into<String>) -> Self {
                    self.language_code = Some(value.into());
                    self
                }
                #[doc = "@required"]
                pub fn start_time_millis(mut self, value: impl Into<String>) -> Self {
                    self.start_time_millis = Some(value.into());
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
                ) -> Result<crate::schemas::ListSessionsResponse, Box<dyn ::std::error::Error>>
                {
                    self.execute_fields::<_, &str>(None)
                }
                #[doc = r" Execute the given operation. This will provide a `fields`"]
                #[doc = r" selector of `*`. This will include every attribute of the"]
                #[doc = r" response resource and should be limited to use during"]
                #[doc = r" development or debugging."]
                pub fn execute_debug(
                    self,
                ) -> Result<crate::schemas::ListSessionsResponse, Box<dyn ::std::error::Error>>
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
                    Ok(req.send()?.error_for_status()?.json()?)
                }
                fn _path(&self) -> String {
                    let mut output =
                        "https://www.googleapis.com/fitness_internal/v0dogfood/".to_owned();
                    output.push_str("users/");
                    {
                        let var_as_str = &self.user_id;
                        output.extend(::percent_encoding::utf8_percent_encode(
                            &var_as_str,
                            crate::SIMPLE,
                        ));
                    }
                    output.push_str("/sessions");
                    output
                }
                fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
                    let req = self.reqwest.request(::reqwest::Method::GET, path);
                    let req =
                        req.query(&[("annotationDataStreamId", &self.annotation_data_stream_id)]);
                    let req = req.query(&[("annotationDataType", &self.annotation_data_type)]);
                    let req = req.query(&[(
                        "deprecatedIncludePostFillerSessions",
                        &self.deprecated_include_post_filler_sessions,
                    )]);
                    let req = req.query(&[(
                        "deprecatedIncludePreFillerSessions",
                        &self.deprecated_include_pre_filler_sessions,
                    )]);
                    let req = req.query(&[("endTimeMillis", &self.end_time_millis)]);
                    let req = req.query(&[("languageCode", &self.language_code)]);
                    let req = req.query(&[("startTimeMillis", &self.start_time_millis)]);
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
            pub struct UpdateEditRequestBuilder<'a, A> {
                pub(crate) reqwest: &'a ::reqwest::Client,
                pub(crate) auth: &'a ::std::sync::Mutex<A>,
                request: crate::schemas::TimelineSession,
                user_id: String,
                end_time_millis: Option<String>,
                hide: Option<bool>,
                start_time_millis: Option<String>,
                alt: Option<crate::params::Alt>,
                fields: Option<String>,
                key: Option<String>,
                oauth_token: Option<String>,
                pretty_print: Option<bool>,
                quota_user: Option<String>,
                user_ip: Option<String>,
            }
            impl<'a, A: yup_oauth2::GetToken> UpdateEditRequestBuilder<'a, A> {
                #[doc = "@required"]
                pub fn end_time_millis(mut self, value: impl Into<String>) -> Self {
                    self.end_time_millis = Some(value.into());
                    self
                }
                #[doc = ""]
                pub fn hide(mut self, value: bool) -> Self {
                    self.hide = Some(value);
                    self
                }
                #[doc = "@required"]
                pub fn start_time_millis(mut self, value: impl Into<String>) -> Self {
                    self.start_time_millis = Some(value.into());
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
                pub fn execute(self) -> Result<(), Box<dyn ::std::error::Error>> {
                    let req = self._request(&self._path());
                    let req = req.json(&self.request);
                    req.send()?.error_for_status()?;
                    Ok(())
                }
                fn _path(&self) -> String {
                    let mut output =
                        "https://www.googleapis.com/fitness_internal/v0dogfood/".to_owned();
                    output.push_str("users/");
                    {
                        let var_as_str = &self.user_id;
                        output.extend(::percent_encoding::utf8_percent_encode(
                            &var_as_str,
                            crate::SIMPLE,
                        ));
                    }
                    output.push_str("/sessions/edit");
                    output
                }
                fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
                    let req = self.reqwest.request(::reqwest::Method::PUT, path);
                    let req = req.query(&[("endTimeMillis", &self.end_time_millis)]);
                    let req = req.query(&[("hide", &self.hide)]);
                    let req = req.query(&[("startTimeMillis", &self.start_time_millis)]);
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
