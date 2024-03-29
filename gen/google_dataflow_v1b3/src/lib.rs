pub mod schemas {
    #[derive(
        Debug, Clone, PartialEq, PartialOrd, Default, :: serde :: Deserialize, :: serde :: Serialize,
    )]
    pub struct ApproximateProgress {
        #[doc = "Obsolete."]
        #[serde(rename = "percentComplete", default)]
        pub percent_complete: ::std::option::Option<f32>,
        #[doc = "Obsolete."]
        #[serde(rename = "position", default)]
        pub position: ::std::option::Option<crate::schemas::Position>,
        #[doc = "Obsolete."]
        #[serde(rename = "remainingTime", default)]
        pub remaining_time: ::std::option::Option<String>,
    }
    impl ::field_selector::FieldSelector for ApproximateProgress {
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
    pub struct ApproximateReportedProgress {
        #[doc = "Total amount of parallelism in the portion of input of this task that has\nalready been consumed and is no longer active. In the first two examples\nabove (see remaining_parallelism), the value should be 29 or 2\nrespectively.  The sum of remaining_parallelism and consumed_parallelism\nshould equal the total amount of parallelism in this work item.  If\nspecified, must be finite."]
        #[serde(rename = "consumedParallelism", default)]
        pub consumed_parallelism: ::std::option::Option<crate::schemas::ReportedParallelism>,
        #[doc = "Completion as fraction of the input consumed, from 0.0 (beginning, nothing\nconsumed), to 1.0 (end of the input, entire input consumed)."]
        #[serde(rename = "fractionConsumed", default)]
        pub fraction_consumed: ::std::option::Option<f64>,
        #[doc = "A Position within the work to represent a progress."]
        #[serde(rename = "position", default)]
        pub position: ::std::option::Option<crate::schemas::Position>,
        #[doc = "Total amount of parallelism in the input of this task that remains,\n(i.e. can be delegated to this task and any new tasks via dynamic\nsplitting). Always at least 1 for non-finished work items and 0 for\nfinished.\n\n\"Amount of parallelism\" refers to how many non-empty parts of the input\ncan be read in parallel. This does not necessarily equal number\nof records. An input that can be read in parallel down to the\nindividual records is called \"perfectly splittable\".\nAn example of non-perfectly parallelizable input is a block-compressed\nfile format where a block of records has to be read as a whole,\nbut different blocks can be read in parallel.\n\nExamples:\n\n* If we are processing record #30 (starting at 1) out of 50 in a perfectly\n  splittable 50-record input, this value should be 21 (20 remaining + 1\n  current).\n* If we are reading through block 3 in a block-compressed file consisting\n  of 5 blocks, this value should be 3 (since blocks 4 and 5 can be\n  processed in parallel by new tasks via dynamic splitting and the current\n  task remains processing block 3).\n* If we are reading through the last block in a block-compressed file,\n  or reading or processing the last record in a perfectly splittable\n  input, this value should be 1, because apart from the current task, no\n  additional remainder can be split off."]
        #[serde(rename = "remainingParallelism", default)]
        pub remaining_parallelism: ::std::option::Option<crate::schemas::ReportedParallelism>,
    }
    impl ::field_selector::FieldSelector for ApproximateReportedProgress {
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
    pub struct ApproximateSplitRequest {
        #[doc = "A fraction at which to split the work item, from 0.0 (beginning of the\ninput) to 1.0 (end of the input)."]
        #[serde(rename = "fractionConsumed", default)]
        pub fraction_consumed: ::std::option::Option<f64>,
        #[doc = "The fraction of the remainder of work to split the work item at, from 0.0\n(split at the current position) to 1.0 (end of the input)."]
        #[serde(rename = "fractionOfRemainder", default)]
        pub fraction_of_remainder: ::std::option::Option<f64>,
        #[doc = "A Position at which to split the work item."]
        #[serde(rename = "position", default)]
        pub position: ::std::option::Option<crate::schemas::Position>,
    }
    impl ::field_selector::FieldSelector for ApproximateSplitRequest {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum AutoscalingEventEventType {
        #[doc = "The ACTUATION_FAILURE type should be used when we want to report\nan error to the user indicating why the current number of workers\nin the pool could not be changed.\nDisplayed in the current status and history widgets."]
        ActuationFailure,
        #[doc = "The CURRENT_NUM_WORKERS_CHANGED type should be used when actual worker\npool size has been changed, but the target_num_workers has not changed."]
        CurrentNumWorkersChanged,
        #[doc = "Used when we want to report to the user a reason why we are\nnot currently adjusting the number of workers.\nShould specify both target_num_workers, current_num_workers and a\ndecision_message."]
        NoChange,
        #[doc = "The TARGET_NUM_WORKERS_CHANGED type should be used when the target\nworker pool size has changed at the start of an actuation. An event\nshould always be specified as TARGET_NUM_WORKERS_CHANGED if it reflects\na change in the target_num_workers."]
        TargetNumWorkersChanged,
        #[doc = "Default type for the enum.  Value should never be returned."]
        TypeUnknown,
    }
    impl AutoscalingEventEventType {
        pub fn as_str(self) -> &'static str {
            match self {
                AutoscalingEventEventType::ActuationFailure => "ACTUATION_FAILURE",
                AutoscalingEventEventType::CurrentNumWorkersChanged => {
                    "CURRENT_NUM_WORKERS_CHANGED"
                }
                AutoscalingEventEventType::NoChange => "NO_CHANGE",
                AutoscalingEventEventType::TargetNumWorkersChanged => "TARGET_NUM_WORKERS_CHANGED",
                AutoscalingEventEventType::TypeUnknown => "TYPE_UNKNOWN",
            }
        }
    }
    impl ::std::fmt::Display for AutoscalingEventEventType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for AutoscalingEventEventType {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for AutoscalingEventEventType {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "ACTUATION_FAILURE" => AutoscalingEventEventType::ActuationFailure,
                "CURRENT_NUM_WORKERS_CHANGED" => {
                    AutoscalingEventEventType::CurrentNumWorkersChanged
                }
                "NO_CHANGE" => AutoscalingEventEventType::NoChange,
                "TARGET_NUM_WORKERS_CHANGED" => AutoscalingEventEventType::TargetNumWorkersChanged,
                "TYPE_UNKNOWN" => AutoscalingEventEventType::TypeUnknown,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::field_selector::FieldSelector for AutoscalingEventEventType {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct AutoscalingEvent {
        #[doc = "The current number of workers the job has."]
        #[serde(rename = "currentNumWorkers", default)]
        #[serde(with = "crate::parsed_string")]
        pub current_num_workers: ::std::option::Option<i64>,
        #[doc = "A message describing why the system decided to adjust the current\nnumber of workers, why it failed, or why the system decided to\nnot make any changes to the number of workers."]
        #[serde(rename = "description", default)]
        pub description: ::std::option::Option<crate::schemas::StructuredMessage>,
        #[doc = "The type of autoscaling event to report."]
        #[serde(rename = "eventType", default)]
        pub event_type: ::std::option::Option<crate::schemas::AutoscalingEventEventType>,
        #[doc = "The target number of workers the worker pool wants to resize to use."]
        #[serde(rename = "targetNumWorkers", default)]
        #[serde(with = "crate::parsed_string")]
        pub target_num_workers: ::std::option::Option<i64>,
        #[doc = "The time this event was emitted to indicate a new target or current\nnum_workers value."]
        #[serde(rename = "time", default)]
        pub time: ::std::option::Option<String>,
        #[doc = "A short and friendly name for the worker pool this event refers to,\npopulated from the value of PoolStageRelation::user_pool_name."]
        #[serde(rename = "workerPool", default)]
        pub worker_pool: ::std::option::Option<String>,
    }
    impl ::field_selector::FieldSelector for AutoscalingEvent {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum AutoscalingSettingsAlgorithm {
        #[doc = "Increase worker count over time to reduce job execution time."]
        AutoscalingAlgorithmBasic,
        #[doc = "Disable autoscaling."]
        AutoscalingAlgorithmNone,
        #[doc = "The algorithm is unknown, or unspecified."]
        AutoscalingAlgorithmUnknown,
    }
    impl AutoscalingSettingsAlgorithm {
        pub fn as_str(self) -> &'static str {
            match self {
                AutoscalingSettingsAlgorithm::AutoscalingAlgorithmBasic => {
                    "AUTOSCALING_ALGORITHM_BASIC"
                }
                AutoscalingSettingsAlgorithm::AutoscalingAlgorithmNone => {
                    "AUTOSCALING_ALGORITHM_NONE"
                }
                AutoscalingSettingsAlgorithm::AutoscalingAlgorithmUnknown => {
                    "AUTOSCALING_ALGORITHM_UNKNOWN"
                }
            }
        }
    }
    impl ::std::fmt::Display for AutoscalingSettingsAlgorithm {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for AutoscalingSettingsAlgorithm {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for AutoscalingSettingsAlgorithm {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "AUTOSCALING_ALGORITHM_BASIC" => {
                    AutoscalingSettingsAlgorithm::AutoscalingAlgorithmBasic
                }
                "AUTOSCALING_ALGORITHM_NONE" => {
                    AutoscalingSettingsAlgorithm::AutoscalingAlgorithmNone
                }
                "AUTOSCALING_ALGORITHM_UNKNOWN" => {
                    AutoscalingSettingsAlgorithm::AutoscalingAlgorithmUnknown
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
    impl ::field_selector::FieldSelector for AutoscalingSettingsAlgorithm {
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
    pub struct AutoscalingSettings {
        #[doc = "The algorithm to use for autoscaling."]
        #[serde(rename = "algorithm", default)]
        pub algorithm: ::std::option::Option<crate::schemas::AutoscalingSettingsAlgorithm>,
        #[doc = "The maximum number of workers to cap scaling at."]
        #[serde(rename = "maxNumWorkers", default)]
        pub max_num_workers: ::std::option::Option<i32>,
    }
    impl ::field_selector::FieldSelector for AutoscalingSettings {
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
    pub struct BigQueryIODetails {
        #[doc = "Dataset accessed in the connection."]
        #[serde(rename = "dataset", default)]
        pub dataset: ::std::option::Option<String>,
        #[doc = "Project accessed in the connection."]
        #[serde(rename = "projectId", default)]
        pub project_id: ::std::option::Option<String>,
        #[doc = "Query used to access data in the connection."]
        #[serde(rename = "query", default)]
        pub query: ::std::option::Option<String>,
        #[doc = "Table accessed in the connection."]
        #[serde(rename = "table", default)]
        pub table: ::std::option::Option<String>,
    }
    impl ::field_selector::FieldSelector for BigQueryIODetails {
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
    pub struct BigTableIODetails {
        #[doc = "InstanceId accessed in the connection."]
        #[serde(rename = "instanceId", default)]
        pub instance_id: ::std::option::Option<String>,
        #[doc = "ProjectId accessed in the connection."]
        #[serde(rename = "projectId", default)]
        pub project_id: ::std::option::Option<String>,
        #[doc = "TableId accessed in the connection."]
        #[serde(rename = "tableId", default)]
        pub table_id: ::std::option::Option<String>,
    }
    impl ::field_selector::FieldSelector for BigTableIODetails {
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
    pub struct ComponentSource {
        #[doc = "Dataflow service generated name for this source."]
        #[serde(rename = "name", default)]
        pub name: ::std::option::Option<String>,
        #[doc = "User name for the original user transform or collection with which this\nsource is most closely associated."]
        #[serde(rename = "originalTransformOrCollection", default)]
        pub original_transform_or_collection: ::std::option::Option<String>,
        #[doc = "Human-readable name for this transform; may be user or system generated."]
        #[serde(rename = "userName", default)]
        pub user_name: ::std::option::Option<String>,
    }
    impl ::field_selector::FieldSelector for ComponentSource {
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
    pub struct ComponentTransform {
        #[doc = "Dataflow service generated name for this source."]
        #[serde(rename = "name", default)]
        pub name: ::std::option::Option<String>,
        #[doc = "User name for the original user transform with which this transform is\nmost closely associated."]
        #[serde(rename = "originalTransform", default)]
        pub original_transform: ::std::option::Option<String>,
        #[doc = "Human-readable name for this transform; may be user or system generated."]
        #[serde(rename = "userName", default)]
        pub user_name: ::std::option::Option<String>,
    }
    impl ::field_selector::FieldSelector for ComponentTransform {
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
    pub struct ComputationTopology {
        #[doc = "The ID of the computation."]
        #[serde(rename = "computationId", default)]
        pub computation_id: ::std::option::Option<String>,
        #[doc = "The inputs to the computation."]
        #[serde(rename = "inputs", default)]
        pub inputs: ::std::option::Option<Vec<crate::schemas::StreamLocation>>,
        #[doc = "The key ranges processed by the computation."]
        #[serde(rename = "keyRanges", default)]
        pub key_ranges: ::std::option::Option<Vec<crate::schemas::KeyRangeLocation>>,
        #[doc = "The outputs from the computation."]
        #[serde(rename = "outputs", default)]
        pub outputs: ::std::option::Option<Vec<crate::schemas::StreamLocation>>,
        #[doc = "The state family values."]
        #[serde(rename = "stateFamilies", default)]
        pub state_families: ::std::option::Option<Vec<crate::schemas::StateFamilyConfig>>,
        #[doc = "The system stage name."]
        #[serde(rename = "systemStageName", default)]
        pub system_stage_name: ::std::option::Option<String>,
    }
    impl ::field_selector::FieldSelector for ComputationTopology {
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
    pub struct ConcatPosition {
        #[doc = "Index of the inner source."]
        #[serde(rename = "index", default)]
        pub index: ::std::option::Option<i32>,
        #[doc = "Position within the inner source."]
        #[serde(rename = "position", default)]
        pub position: ::std::option::Option<Box<crate::schemas::Position>>,
    }
    impl ::field_selector::FieldSelector for ConcatPosition {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum CounterMetadataKind {
        #[doc = "Aggregated value represents the logical 'and' of all contributed values."]
        And,
        #[doc = "Aggregated value captures statistics about a distribution."]
        Distribution,
        #[doc = "Counter aggregation kind was not set."]
        Invalid,
        #[doc = "Aggregated value tracks the latest value of a variable."]
        LatestValue,
        #[doc = "Aggregated value is the max of all contributed values."]
        Max,
        #[doc = "Aggregated value is the mean of all contributed values."]
        Mean,
        #[doc = "Aggregated value is the min of all contributed values."]
        Min,
        #[doc = "Aggregated value represents the logical 'or' of all contributed values."]
        Or,
        #[doc = "Aggregated value is a set of unique contributed values."]
        Set,
        #[doc = "Aggregated value is the sum of all contributed values."]
        Sum,
    }
    impl CounterMetadataKind {
        pub fn as_str(self) -> &'static str {
            match self {
                CounterMetadataKind::And => "AND",
                CounterMetadataKind::Distribution => "DISTRIBUTION",
                CounterMetadataKind::Invalid => "INVALID",
                CounterMetadataKind::LatestValue => "LATEST_VALUE",
                CounterMetadataKind::Max => "MAX",
                CounterMetadataKind::Mean => "MEAN",
                CounterMetadataKind::Min => "MIN",
                CounterMetadataKind::Or => "OR",
                CounterMetadataKind::Set => "SET",
                CounterMetadataKind::Sum => "SUM",
            }
        }
    }
    impl ::std::fmt::Display for CounterMetadataKind {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for CounterMetadataKind {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for CounterMetadataKind {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "AND" => CounterMetadataKind::And,
                "DISTRIBUTION" => CounterMetadataKind::Distribution,
                "INVALID" => CounterMetadataKind::Invalid,
                "LATEST_VALUE" => CounterMetadataKind::LatestValue,
                "MAX" => CounterMetadataKind::Max,
                "MEAN" => CounterMetadataKind::Mean,
                "MIN" => CounterMetadataKind::Min,
                "OR" => CounterMetadataKind::Or,
                "SET" => CounterMetadataKind::Set,
                "SUM" => CounterMetadataKind::Sum,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::field_selector::FieldSelector for CounterMetadataKind {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum CounterMetadataStandardUnits {
        #[doc = "Counter returns a value in bytes."]
        Bytes,
        #[doc = "Counter returns a value in bytes per second."]
        BytesPerSec,
        #[doc = "Counter returns a value in microseconds."]
        Microseconds,
        #[doc = "Counter returns a value in milliseconds."]
        Milliseconds,
        #[doc = "Counter returns a value in nanoseconds."]
        Nanoseconds,
        #[doc = "Counter returns a timestamp in milliseconds."]
        TimestampMsec,
        #[doc = "Counter returns a timestamp in nanoseconds."]
        TimestampNsec,
        #[doc = "Counter returns a timestamp in microseconds."]
        TimestampUsec,
    }
    impl CounterMetadataStandardUnits {
        pub fn as_str(self) -> &'static str {
            match self {
                CounterMetadataStandardUnits::Bytes => "BYTES",
                CounterMetadataStandardUnits::BytesPerSec => "BYTES_PER_SEC",
                CounterMetadataStandardUnits::Microseconds => "MICROSECONDS",
                CounterMetadataStandardUnits::Milliseconds => "MILLISECONDS",
                CounterMetadataStandardUnits::Nanoseconds => "NANOSECONDS",
                CounterMetadataStandardUnits::TimestampMsec => "TIMESTAMP_MSEC",
                CounterMetadataStandardUnits::TimestampNsec => "TIMESTAMP_NSEC",
                CounterMetadataStandardUnits::TimestampUsec => "TIMESTAMP_USEC",
            }
        }
    }
    impl ::std::fmt::Display for CounterMetadataStandardUnits {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for CounterMetadataStandardUnits {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for CounterMetadataStandardUnits {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "BYTES" => CounterMetadataStandardUnits::Bytes,
                "BYTES_PER_SEC" => CounterMetadataStandardUnits::BytesPerSec,
                "MICROSECONDS" => CounterMetadataStandardUnits::Microseconds,
                "MILLISECONDS" => CounterMetadataStandardUnits::Milliseconds,
                "NANOSECONDS" => CounterMetadataStandardUnits::Nanoseconds,
                "TIMESTAMP_MSEC" => CounterMetadataStandardUnits::TimestampMsec,
                "TIMESTAMP_NSEC" => CounterMetadataStandardUnits::TimestampNsec,
                "TIMESTAMP_USEC" => CounterMetadataStandardUnits::TimestampUsec,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::field_selector::FieldSelector for CounterMetadataStandardUnits {
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
    pub struct CounterMetadata {
        #[doc = "Human-readable description of the counter semantics."]
        #[serde(rename = "description", default)]
        pub description: ::std::option::Option<String>,
        #[doc = "Counter aggregation kind."]
        #[serde(rename = "kind", default)]
        pub kind: ::std::option::Option<crate::schemas::CounterMetadataKind>,
        #[doc = "A string referring to the unit type."]
        #[serde(rename = "otherUnits", default)]
        pub other_units: ::std::option::Option<String>,
        #[doc = "System defined Units, see above enum."]
        #[serde(rename = "standardUnits", default)]
        pub standard_units: ::std::option::Option<crate::schemas::CounterMetadataStandardUnits>,
    }
    impl ::field_selector::FieldSelector for CounterMetadata {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum CounterStructuredNameOrigin {
        #[doc = "Counter was created by the Dataflow system."]
        System,
        #[doc = "Counter was created by the user."]
        User,
    }
    impl CounterStructuredNameOrigin {
        pub fn as_str(self) -> &'static str {
            match self {
                CounterStructuredNameOrigin::System => "SYSTEM",
                CounterStructuredNameOrigin::User => "USER",
            }
        }
    }
    impl ::std::fmt::Display for CounterStructuredNameOrigin {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for CounterStructuredNameOrigin {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for CounterStructuredNameOrigin {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "SYSTEM" => CounterStructuredNameOrigin::System,
                "USER" => CounterStructuredNameOrigin::User,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::field_selector::FieldSelector for CounterStructuredNameOrigin {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum CounterStructuredNamePortion {
        #[doc = "Counter portion has not been set."]
        All,
        #[doc = "Counter reports a key."]
        Key,
        #[doc = "Counter reports a value."]
        Value,
    }
    impl CounterStructuredNamePortion {
        pub fn as_str(self) -> &'static str {
            match self {
                CounterStructuredNamePortion::All => "ALL",
                CounterStructuredNamePortion::Key => "KEY",
                CounterStructuredNamePortion::Value => "VALUE",
            }
        }
    }
    impl ::std::fmt::Display for CounterStructuredNamePortion {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for CounterStructuredNamePortion {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for CounterStructuredNamePortion {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "ALL" => CounterStructuredNamePortion::All,
                "KEY" => CounterStructuredNamePortion::Key,
                "VALUE" => CounterStructuredNamePortion::Value,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::field_selector::FieldSelector for CounterStructuredNamePortion {
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
    pub struct CounterStructuredName {
        #[doc = "Name of the optimized step being executed by the workers."]
        #[serde(rename = "componentStepName", default)]
        pub component_step_name: ::std::option::Option<String>,
        #[doc = "Name of the stage. An execution step contains multiple component steps."]
        #[serde(rename = "executionStepName", default)]
        pub execution_step_name: ::std::option::Option<String>,
        #[doc = "Index of an input collection that's being read from/written to as a side\ninput.\nThe index identifies a step's side inputs starting by 1 (e.g. the first\nside input has input_index 1, the third has input_index 3).\nSide inputs are identified by a pair of (original_step_name, input_index).\nThis field helps uniquely identify them."]
        #[serde(rename = "inputIndex", default)]
        pub input_index: ::std::option::Option<i32>,
        #[doc = "Counter name. Not necessarily globally-unique, but unique within the\ncontext of the other fields.\nRequired."]
        #[serde(rename = "name", default)]
        pub name: ::std::option::Option<String>,
        #[doc = "One of the standard Origins defined above."]
        #[serde(rename = "origin", default)]
        pub origin: ::std::option::Option<crate::schemas::CounterStructuredNameOrigin>,
        #[doc = "A string containing a more specific namespace of the counter's origin."]
        #[serde(rename = "originNamespace", default)]
        pub origin_namespace: ::std::option::Option<String>,
        #[doc = "The step name requesting an operation, such as GBK.\nI.e. the ParDo causing a read/write from shuffle to occur, or a\nread from side inputs."]
        #[serde(rename = "originalRequestingStepName", default)]
        pub original_requesting_step_name: ::std::option::Option<String>,
        #[doc = "System generated name of the original step in the user's graph, before\noptimization."]
        #[serde(rename = "originalStepName", default)]
        pub original_step_name: ::std::option::Option<String>,
        #[doc = "Portion of this counter, either key or value."]
        #[serde(rename = "portion", default)]
        pub portion: ::std::option::Option<crate::schemas::CounterStructuredNamePortion>,
        #[doc = "ID of a particular worker."]
        #[serde(rename = "workerId", default)]
        pub worker_id: ::std::option::Option<String>,
    }
    impl ::field_selector::FieldSelector for CounterStructuredName {
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
    pub struct CounterStructuredNameAndMetadata {
        #[doc = "Metadata associated with a counter"]
        #[serde(rename = "metadata", default)]
        pub metadata: ::std::option::Option<crate::schemas::CounterMetadata>,
        #[doc = "Structured name of the counter."]
        #[serde(rename = "name", default)]
        pub name: ::std::option::Option<crate::schemas::CounterStructuredName>,
    }
    impl ::field_selector::FieldSelector for CounterStructuredNameAndMetadata {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct CounterUpdate {
        #[doc = "Boolean value for And, Or."]
        #[serde(rename = "boolean", default)]
        pub boolean: ::std::option::Option<bool>,
        #[doc = "True if this counter is reported as the total cumulative aggregate\nvalue accumulated since the worker started working on this WorkItem.\nBy default this is false, indicating that this counter is reported\nas a delta."]
        #[serde(rename = "cumulative", default)]
        pub cumulative: ::std::option::Option<bool>,
        #[doc = "Distribution data"]
        #[serde(rename = "distribution", default)]
        pub distribution: ::std::option::Option<crate::schemas::DistributionUpdate>,
        #[doc = "Floating point value for Sum, Max, Min."]
        #[serde(rename = "floatingPoint", default)]
        pub floating_point: ::std::option::Option<f64>,
        #[doc = "List of floating point numbers, for Set."]
        #[serde(rename = "floatingPointList", default)]
        pub floating_point_list: ::std::option::Option<crate::schemas::FloatingPointList>,
        #[doc = "Floating point mean aggregation value for Mean."]
        #[serde(rename = "floatingPointMean", default)]
        pub floating_point_mean: ::std::option::Option<crate::schemas::FloatingPointMean>,
        #[doc = "Integer value for Sum, Max, Min."]
        #[serde(rename = "integer", default)]
        pub integer: ::std::option::Option<crate::schemas::SplitInt64>,
        #[doc = "Gauge data"]
        #[serde(rename = "integerGauge", default)]
        pub integer_gauge: ::std::option::Option<crate::schemas::IntegerGauge>,
        #[doc = "List of integers, for Set."]
        #[serde(rename = "integerList", default)]
        pub integer_list: ::std::option::Option<crate::schemas::IntegerList>,
        #[doc = "Integer mean aggregation value for Mean."]
        #[serde(rename = "integerMean", default)]
        pub integer_mean: ::std::option::Option<crate::schemas::IntegerMean>,
        #[doc = "Value for internally-defined counters used by the Dataflow service."]
        #[serde(rename = "internal", default)]
        pub internal: ::std::option::Option<::serde_json::Value>,
        #[doc = "Counter name and aggregation type."]
        #[serde(rename = "nameAndKind", default)]
        pub name_and_kind: ::std::option::Option<crate::schemas::NameAndKind>,
        #[doc = "The service-generated short identifier for this counter.\nThe short_id -> (name, metadata) mapping is constant for the lifetime of\na job."]
        #[serde(rename = "shortId", default)]
        #[serde(with = "crate::parsed_string")]
        pub short_id: ::std::option::Option<i64>,
        #[doc = "List of strings, for Set."]
        #[serde(rename = "stringList", default)]
        pub string_list: ::std::option::Option<crate::schemas::StringList>,
        #[doc = "Counter structured name and metadata."]
        #[serde(rename = "structuredNameAndMetadata", default)]
        pub structured_name_and_metadata:
            ::std::option::Option<crate::schemas::CounterStructuredNameAndMetadata>,
    }
    impl ::field_selector::FieldSelector for CounterUpdate {
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
    pub struct Cputime {
        #[doc = "Average CPU utilization rate (% non-idle cpu / second) since previous\nsample."]
        #[serde(rename = "rate", default)]
        pub rate: ::std::option::Option<f64>,
        #[doc = "Timestamp of the measurement."]
        #[serde(rename = "timestamp", default)]
        pub timestamp: ::std::option::Option<String>,
        #[doc = "Total active CPU time across all cores (ie., non-idle) in milliseconds\nsince start-up."]
        #[serde(rename = "totalMs", default)]
        #[serde(with = "crate::parsed_string")]
        pub total_ms: ::std::option::Option<u64>,
    }
    impl ::field_selector::FieldSelector for Cputime {
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
    pub struct CreateJobFromTemplateRequest {
        #[doc = "The runtime environment for the job."]
        #[serde(rename = "environment", default)]
        pub environment: ::std::option::Option<crate::schemas::RuntimeEnvironment>,
        #[doc = "Required. A Cloud Storage path to the template from which to\ncreate the job.\nMust be a valid Cloud Storage URL, beginning with `gs://`."]
        #[serde(rename = "gcsPath", default)]
        pub gcs_path: ::std::option::Option<String>,
        #[doc = "Required. The job name to use for the created job."]
        #[serde(rename = "jobName", default)]
        pub job_name: ::std::option::Option<String>,
        #[doc = "The [regional endpoint]\n(https://cloud.google.com/dataflow/docs/concepts/regional-endpoints) to\nwhich to direct the request."]
        #[serde(rename = "location", default)]
        pub location: ::std::option::Option<String>,
        #[doc = "The runtime parameters to pass to the job."]
        #[serde(rename = "parameters", default)]
        pub parameters: ::std::option::Option<::std::collections::BTreeMap<String, String>>,
    }
    impl ::field_selector::FieldSelector for CreateJobFromTemplateRequest {
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
    pub struct CustomSourceLocation {
        #[doc = "Whether this source is stateful."]
        #[serde(rename = "stateful", default)]
        pub stateful: ::std::option::Option<bool>,
    }
    impl ::field_selector::FieldSelector for CustomSourceLocation {
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
    pub struct DataDiskAssignment {
        #[doc = "Mounted data disks. The order is important a data disk's 0-based index in\nthis list defines which persistent directory the disk is mounted to, for\nexample the list of { \"myproject-1014-104817-4c2-harness-0-disk-0\" },\n{ \"myproject-1014-104817-4c2-harness-0-disk-1\" }."]
        #[serde(rename = "dataDisks", default)]
        pub data_disks: ::std::option::Option<Vec<String>>,
        #[doc = "VM instance name the data disks mounted to, for example\n\"myproject-1014-104817-4c2-harness-0\"."]
        #[serde(rename = "vmInstance", default)]
        pub vm_instance: ::std::option::Option<String>,
    }
    impl ::field_selector::FieldSelector for DataDiskAssignment {
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
    pub struct DatastoreIODetails {
        #[doc = "Namespace used in the connection."]
        #[serde(rename = "namespace", default)]
        pub namespace: ::std::option::Option<String>,
        #[doc = "ProjectId accessed in the connection."]
        #[serde(rename = "projectId", default)]
        pub project_id: ::std::option::Option<String>,
    }
    impl ::field_selector::FieldSelector for DatastoreIODetails {
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
        Copy,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct DeleteSnapshotResponse;
    impl ::field_selector::FieldSelector for DeleteSnapshotResponse {
        fn field_selector_with_ident(_ident: &str, _selector: &mut String) {}
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum DerivedSourceDerivationMode {
        #[doc = "Produce a Source based on the Source being split."]
        SourceDerivationModeChildOfCurrent,
        #[doc = "Produce a completely independent Source with no base."]
        SourceDerivationModeIndependent,
        #[doc = "Produce a Source based on the base of the Source being split."]
        SourceDerivationModeSiblingOfCurrent,
        #[doc = "The source derivation is unknown, or unspecified."]
        SourceDerivationModeUnknown,
    }
    impl DerivedSourceDerivationMode {
        pub fn as_str(self) -> &'static str {
            match self {
                DerivedSourceDerivationMode::SourceDerivationModeChildOfCurrent => {
                    "SOURCE_DERIVATION_MODE_CHILD_OF_CURRENT"
                }
                DerivedSourceDerivationMode::SourceDerivationModeIndependent => {
                    "SOURCE_DERIVATION_MODE_INDEPENDENT"
                }
                DerivedSourceDerivationMode::SourceDerivationModeSiblingOfCurrent => {
                    "SOURCE_DERIVATION_MODE_SIBLING_OF_CURRENT"
                }
                DerivedSourceDerivationMode::SourceDerivationModeUnknown => {
                    "SOURCE_DERIVATION_MODE_UNKNOWN"
                }
            }
        }
    }
    impl ::std::fmt::Display for DerivedSourceDerivationMode {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for DerivedSourceDerivationMode {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for DerivedSourceDerivationMode {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "SOURCE_DERIVATION_MODE_CHILD_OF_CURRENT" => {
                    DerivedSourceDerivationMode::SourceDerivationModeChildOfCurrent
                }
                "SOURCE_DERIVATION_MODE_INDEPENDENT" => {
                    DerivedSourceDerivationMode::SourceDerivationModeIndependent
                }
                "SOURCE_DERIVATION_MODE_SIBLING_OF_CURRENT" => {
                    DerivedSourceDerivationMode::SourceDerivationModeSiblingOfCurrent
                }
                "SOURCE_DERIVATION_MODE_UNKNOWN" => {
                    DerivedSourceDerivationMode::SourceDerivationModeUnknown
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
    impl ::field_selector::FieldSelector for DerivedSourceDerivationMode {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct DerivedSource {
        #[doc = "What source to base the produced source on (if any)."]
        #[serde(rename = "derivationMode", default)]
        pub derivation_mode: ::std::option::Option<crate::schemas::DerivedSourceDerivationMode>,
        #[doc = "Specification of the source."]
        #[serde(rename = "source", default)]
        pub source: ::std::option::Option<crate::schemas::Source>,
    }
    impl ::field_selector::FieldSelector for DerivedSource {
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
    pub struct Disk {
        #[doc = "Disk storage type, as defined by Google Compute Engine.  This\nmust be a disk type appropriate to the project and zone in which\nthe workers will run.  If unknown or unspecified, the service\nwill attempt to choose a reasonable default.\n\nFor example, the standard persistent disk type is a resource name\ntypically ending in \"pd-standard\".  If SSD persistent disks are\navailable, the resource name typically ends with \"pd-ssd\".  The\nactual valid values are defined the Google Compute Engine API,\nnot by the Cloud Dataflow API; consult the Google Compute Engine\ndocumentation for more information about determining the set of\navailable disk types for a particular project and zone.\n\nGoogle Compute Engine Disk types are local to a particular\nproject in a particular zone, and so the resource name will\ntypically look something like this:\n\ncompute.googleapis.com/projects/project-id/zones/zone/diskTypes/pd-standard"]
        #[serde(rename = "diskType", default)]
        pub disk_type: ::std::option::Option<String>,
        #[doc = "Directory in a VM where disk is mounted."]
        #[serde(rename = "mountPoint", default)]
        pub mount_point: ::std::option::Option<String>,
        #[doc = "Size of disk in GB.  If zero or unspecified, the service will\nattempt to choose a reasonable default."]
        #[serde(rename = "sizeGb", default)]
        pub size_gb: ::std::option::Option<i32>,
    }
    impl ::field_selector::FieldSelector for Disk {
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
    pub struct DisplayData {
        #[doc = "Contains value if the data is of a boolean type."]
        #[serde(rename = "boolValue", default)]
        pub bool_value: ::std::option::Option<bool>,
        #[doc = "Contains value if the data is of duration type."]
        #[serde(rename = "durationValue", default)]
        pub duration_value: ::std::option::Option<String>,
        #[doc = "Contains value if the data is of float type."]
        #[serde(rename = "floatValue", default)]
        pub float_value: ::std::option::Option<f32>,
        #[doc = "Contains value if the data is of int64 type."]
        #[serde(rename = "int64Value", default)]
        #[serde(with = "crate::parsed_string")]
        pub int_64_value: ::std::option::Option<i64>,
        #[doc = "Contains value if the data is of java class type."]
        #[serde(rename = "javaClassValue", default)]
        pub java_class_value: ::std::option::Option<String>,
        #[doc = "The key identifying the display data.\nThis is intended to be used as a label for the display data\nwhen viewed in a dax monitoring system."]
        #[serde(rename = "key", default)]
        pub key: ::std::option::Option<String>,
        #[doc = "An optional label to display in a dax UI for the element."]
        #[serde(rename = "label", default)]
        pub label: ::std::option::Option<String>,
        #[doc = "The namespace for the key. This is usually a class name or programming\nlanguage namespace (i.e. python module) which defines the display data.\nThis allows a dax monitoring system to specially handle the data\nand perform custom rendering."]
        #[serde(rename = "namespace", default)]
        pub namespace: ::std::option::Option<String>,
        #[doc = "A possible additional shorter value to display.\nFor example a java_class_name_value of com.mypackage.MyDoFn\nwill be stored with MyDoFn as the short_str_value and\ncom.mypackage.MyDoFn as the java_class_name value.\nshort_str_value can be displayed and java_class_name_value\nwill be displayed as a tooltip."]
        #[serde(rename = "shortStrValue", default)]
        pub short_str_value: ::std::option::Option<String>,
        #[doc = "Contains value if the data is of string type."]
        #[serde(rename = "strValue", default)]
        pub str_value: ::std::option::Option<String>,
        #[doc = "Contains value if the data is of timestamp type."]
        #[serde(rename = "timestampValue", default)]
        pub timestamp_value: ::std::option::Option<String>,
        #[doc = "An optional full URL."]
        #[serde(rename = "url", default)]
        pub url: ::std::option::Option<String>,
    }
    impl ::field_selector::FieldSelector for DisplayData {
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
    pub struct DistributionUpdate {
        #[doc = "The count of the number of elements present in the distribution."]
        #[serde(rename = "count", default)]
        pub count: ::std::option::Option<crate::schemas::SplitInt64>,
        #[doc = "(Optional) Histogram of value counts for the distribution."]
        #[serde(rename = "histogram", default)]
        pub histogram: ::std::option::Option<crate::schemas::Histogram>,
        #[doc = "The maximum value present in the distribution."]
        #[serde(rename = "max", default)]
        pub max: ::std::option::Option<crate::schemas::SplitInt64>,
        #[doc = "The minimum value present in the distribution."]
        #[serde(rename = "min", default)]
        pub min: ::std::option::Option<crate::schemas::SplitInt64>,
        #[doc = "Use an int64 since we'd prefer the added precision. If overflow is a common\nproblem we can detect it and use an additional int64 or a double."]
        #[serde(rename = "sum", default)]
        pub sum: ::std::option::Option<crate::schemas::SplitInt64>,
        #[doc = "Use a double since the sum of squares is likely to overflow int64."]
        #[serde(rename = "sumOfSquares", default)]
        pub sum_of_squares: ::std::option::Option<f64>,
    }
    impl ::field_selector::FieldSelector for DistributionUpdate {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct DynamicSourceSplit {
        #[doc = "Primary part (continued to be processed by worker).\nSpecified relative to the previously-current source.\nBecomes current."]
        #[serde(rename = "primary", default)]
        pub primary: ::std::option::Option<crate::schemas::DerivedSource>,
        #[doc = "Residual part (returned to the pool of work).\nSpecified relative to the previously-current source."]
        #[serde(rename = "residual", default)]
        pub residual: ::std::option::Option<crate::schemas::DerivedSource>,
    }
    impl ::field_selector::FieldSelector for DynamicSourceSplit {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum EnvironmentFlexResourceSchedulingGoal {
        #[doc = "Optimize for lower cost."]
        FlexrsCostOptimized,
        #[doc = "Optimize for lower execution time."]
        FlexrsSpeedOptimized,
        #[doc = "Run in the default mode."]
        FlexrsUnspecified,
    }
    impl EnvironmentFlexResourceSchedulingGoal {
        pub fn as_str(self) -> &'static str {
            match self {
                EnvironmentFlexResourceSchedulingGoal::FlexrsCostOptimized => {
                    "FLEXRS_COST_OPTIMIZED"
                }
                EnvironmentFlexResourceSchedulingGoal::FlexrsSpeedOptimized => {
                    "FLEXRS_SPEED_OPTIMIZED"
                }
                EnvironmentFlexResourceSchedulingGoal::FlexrsUnspecified => "FLEXRS_UNSPECIFIED",
            }
        }
    }
    impl ::std::fmt::Display for EnvironmentFlexResourceSchedulingGoal {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for EnvironmentFlexResourceSchedulingGoal {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for EnvironmentFlexResourceSchedulingGoal {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "FLEXRS_COST_OPTIMIZED" => {
                    EnvironmentFlexResourceSchedulingGoal::FlexrsCostOptimized
                }
                "FLEXRS_SPEED_OPTIMIZED" => {
                    EnvironmentFlexResourceSchedulingGoal::FlexrsSpeedOptimized
                }
                "FLEXRS_UNSPECIFIED" => EnvironmentFlexResourceSchedulingGoal::FlexrsUnspecified,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::field_selector::FieldSelector for EnvironmentFlexResourceSchedulingGoal {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct Environment {
        #[doc = "The type of cluster manager API to use.  If unknown or\nunspecified, the service will attempt to choose a reasonable\ndefault.  This should be in the form of the API service name,\ne.g. \"compute.googleapis.com\"."]
        #[serde(rename = "clusterManagerApiService", default)]
        pub cluster_manager_api_service: ::std::option::Option<String>,
        #[doc = "The dataset for the current project where various workflow\nrelated tables are stored.\n\nThe supported resource type is:\n\nGoogle BigQuery:\nbigquery.googleapis.com/{dataset}"]
        #[serde(rename = "dataset", default)]
        pub dataset: ::std::option::Option<String>,
        #[doc = "The list of experiments to enable."]
        #[serde(rename = "experiments", default)]
        pub experiments: ::std::option::Option<Vec<String>>,
        #[doc = "Which Flexible Resource Scheduling mode to run in."]
        #[serde(rename = "flexResourceSchedulingGoal", default)]
        pub flex_resource_scheduling_goal:
            ::std::option::Option<crate::schemas::EnvironmentFlexResourceSchedulingGoal>,
        #[doc = "Experimental settings."]
        #[serde(rename = "internalExperiments", default)]
        pub internal_experiments:
            ::std::option::Option<::std::collections::BTreeMap<String, ::serde_json::Value>>,
        #[doc = "The Cloud Dataflow SDK pipeline options specified by the user. These\noptions are passed through the service and are used to recreate the\nSDK pipeline options on the worker in a language agnostic and platform\nindependent way."]
        #[serde(rename = "sdkPipelineOptions", default)]
        pub sdk_pipeline_options:
            ::std::option::Option<::std::collections::BTreeMap<String, ::serde_json::Value>>,
        #[doc = "Identity to run virtual machines as. Defaults to the default account."]
        #[serde(rename = "serviceAccountEmail", default)]
        pub service_account_email: ::std::option::Option<String>,
        #[doc = "If set, contains the Cloud KMS key identifier used to encrypt data\nat rest, AKA a Customer Managed Encryption Key (CMEK).\n\nFormat:\nprojects/PROJECT_ID/locations/LOCATION/keyRings/KEY_RING/cryptoKeys/KEY"]
        #[serde(rename = "serviceKmsKeyName", default)]
        pub service_kms_key_name: ::std::option::Option<String>,
        #[doc = "The prefix of the resources the system should use for temporary\nstorage.  The system will append the suffix \"/temp-{JOBNAME} to\nthis resource prefix, where {JOBNAME} is the value of the\njob_name field.  The resulting bucket and object prefix is used\nas the prefix of the resources used to store temporary data\nneeded during the job execution.  NOTE: This will override the\nvalue in taskrunner_settings.\nThe supported resource type is:\n\nGoogle Cloud Storage:\n\nstorage.googleapis.com/{bucket}/{object}\nbucket.storage.googleapis.com/{object}"]
        #[serde(rename = "tempStoragePrefix", default)]
        pub temp_storage_prefix: ::std::option::Option<String>,
        #[doc = "A description of the process that generated the request."]
        #[serde(rename = "userAgent", default)]
        pub user_agent:
            ::std::option::Option<::std::collections::BTreeMap<String, ::serde_json::Value>>,
        #[doc = "A structure describing which components and their versions of the service\nare required in order to run the job."]
        #[serde(rename = "version", default)]
        pub version:
            ::std::option::Option<::std::collections::BTreeMap<String, ::serde_json::Value>>,
        #[doc = "The worker pools. At least one \"harness\" worker pool must be\nspecified in order for the job to have workers."]
        #[serde(rename = "workerPools", default)]
        pub worker_pools: ::std::option::Option<Vec<crate::schemas::WorkerPool>>,
    }
    impl ::field_selector::FieldSelector for Environment {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum ExecutionStageStateExecutionStageState {
        #[doc = "`JOB_STATE_CANCELLED` indicates that the job has been explicitly\ncancelled. This is a terminal job state. This state may only be\nset via a Cloud Dataflow `UpdateJob` call, and only if the job has not\nyet reached another terminal state."]
        JobStateCancelled,
        #[doc = "`JOB_STATE_CANCELLING` indicates that the job has been explicitly cancelled\nand is in the process of stopping.  Jobs that are cancelling may only\ntransition to `JOB_STATE_CANCELLED` or `JOB_STATE_FAILED`."]
        JobStateCancelling,
        #[doc = "`JOB_STATE_DONE` indicates that the job has successfully completed.\nThis is a terminal job state.  This state may be set by the Cloud Dataflow\nservice, as a transition from `JOB_STATE_RUNNING`. It may also be set via a\nCloud Dataflow `UpdateJob` call, if the job has not yet reached a terminal\nstate."]
        JobStateDone,
        #[doc = "`JOB_STATE_DRAINED` indicates that the job has been drained.\nA drained job terminated by stopping pulling from its input sources and\nprocessing any data that remained in-flight when draining was requested.\nThis state is a terminal state, may only be set by the Cloud Dataflow\nservice, and only as a transition from `JOB_STATE_DRAINING`."]
        JobStateDrained,
        #[doc = "`JOB_STATE_DRAINING` indicates that the job is in the process of draining.\nA draining job has stopped pulling from its input sources and is processing\nany data that remains in-flight. This state may be set via a Cloud Dataflow\n`UpdateJob` call, but only as a transition from `JOB_STATE_RUNNING`. Jobs\nthat are draining may only transition to `JOB_STATE_DRAINED`,\n`JOB_STATE_CANCELLED`, or `JOB_STATE_FAILED`."]
        JobStateDraining,
        #[doc = "`JOB_STATE_FAILED` indicates that the job has failed.  This is a\nterminal job state.  This state may only be set by the Cloud Dataflow\nservice, and only as a transition from `JOB_STATE_RUNNING`."]
        JobStateFailed,
        #[doc = "`JOB_STATE_PENDING` indicates that the job has been created but is not yet\nrunning.  Jobs that are pending may only transition to `JOB_STATE_RUNNING`,\nor `JOB_STATE_FAILED`."]
        JobStatePending,
        #[doc = "`JOB_STATE_QUEUED` indicates that the job has been created but is being\ndelayed until launch. Jobs that are queued may only transition to\n`JOB_STATE_PENDING` or `JOB_STATE_CANCELLED`."]
        JobStateQueued,
        #[doc = "`JOB_STATE_RUNNING` indicates that the job is currently running."]
        JobStateRunning,
        #[doc = "`JOB_STATE_STOPPED` indicates that the job has not\nyet started to run."]
        JobStateStopped,
        #[doc = "The job's run state isn't specified."]
        JobStateUnknown,
        #[doc = "`JOB_STATE_UPDATED` indicates that the job was successfully updated,\nmeaning that this job was stopped and another job was started, inheriting\nstate from this one. This is a terminal job state. This state may only be\nset by the Cloud Dataflow service, and only as a transition from\n`JOB_STATE_RUNNING`."]
        JobStateUpdated,
    }
    impl ExecutionStageStateExecutionStageState {
        pub fn as_str(self) -> &'static str {
            match self {
                ExecutionStageStateExecutionStageState::JobStateCancelled => "JOB_STATE_CANCELLED",
                ExecutionStageStateExecutionStageState::JobStateCancelling => {
                    "JOB_STATE_CANCELLING"
                }
                ExecutionStageStateExecutionStageState::JobStateDone => "JOB_STATE_DONE",
                ExecutionStageStateExecutionStageState::JobStateDrained => "JOB_STATE_DRAINED",
                ExecutionStageStateExecutionStageState::JobStateDraining => "JOB_STATE_DRAINING",
                ExecutionStageStateExecutionStageState::JobStateFailed => "JOB_STATE_FAILED",
                ExecutionStageStateExecutionStageState::JobStatePending => "JOB_STATE_PENDING",
                ExecutionStageStateExecutionStageState::JobStateQueued => "JOB_STATE_QUEUED",
                ExecutionStageStateExecutionStageState::JobStateRunning => "JOB_STATE_RUNNING",
                ExecutionStageStateExecutionStageState::JobStateStopped => "JOB_STATE_STOPPED",
                ExecutionStageStateExecutionStageState::JobStateUnknown => "JOB_STATE_UNKNOWN",
                ExecutionStageStateExecutionStageState::JobStateUpdated => "JOB_STATE_UPDATED",
            }
        }
    }
    impl ::std::fmt::Display for ExecutionStageStateExecutionStageState {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for ExecutionStageStateExecutionStageState {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for ExecutionStageStateExecutionStageState {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "JOB_STATE_CANCELLED" => ExecutionStageStateExecutionStageState::JobStateCancelled,
                "JOB_STATE_CANCELLING" => {
                    ExecutionStageStateExecutionStageState::JobStateCancelling
                }
                "JOB_STATE_DONE" => ExecutionStageStateExecutionStageState::JobStateDone,
                "JOB_STATE_DRAINED" => ExecutionStageStateExecutionStageState::JobStateDrained,
                "JOB_STATE_DRAINING" => ExecutionStageStateExecutionStageState::JobStateDraining,
                "JOB_STATE_FAILED" => ExecutionStageStateExecutionStageState::JobStateFailed,
                "JOB_STATE_PENDING" => ExecutionStageStateExecutionStageState::JobStatePending,
                "JOB_STATE_QUEUED" => ExecutionStageStateExecutionStageState::JobStateQueued,
                "JOB_STATE_RUNNING" => ExecutionStageStateExecutionStageState::JobStateRunning,
                "JOB_STATE_STOPPED" => ExecutionStageStateExecutionStageState::JobStateStopped,
                "JOB_STATE_UNKNOWN" => ExecutionStageStateExecutionStageState::JobStateUnknown,
                "JOB_STATE_UPDATED" => ExecutionStageStateExecutionStageState::JobStateUpdated,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::field_selector::FieldSelector for ExecutionStageStateExecutionStageState {
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
    pub struct ExecutionStageState {
        #[doc = "The time at which the stage transitioned to this state."]
        #[serde(rename = "currentStateTime", default)]
        pub current_state_time: ::std::option::Option<String>,
        #[doc = "The name of the execution stage."]
        #[serde(rename = "executionStageName", default)]
        pub execution_stage_name: ::std::option::Option<String>,
        #[doc = "Executions stage states allow the same set of values as JobState."]
        #[serde(rename = "executionStageState", default)]
        pub execution_stage_state:
            ::std::option::Option<crate::schemas::ExecutionStageStateExecutionStageState>,
    }
    impl ::field_selector::FieldSelector for ExecutionStageState {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum ExecutionStageSummaryKind {
        #[doc = "Constructs from a constant value, such as with Create.of."]
        ConstantKind,
        #[doc = "Flatten transform."]
        FlattenKind,
        #[doc = "Group By Key transform."]
        GroupByKeyKind,
        #[doc = "ParDo transform."]
        ParDoKind,
        #[doc = "Read transform."]
        ReadKind,
        #[doc = "Opening or closing a shuffle session, often as part of a GroupByKey."]
        ShuffleKind,
        #[doc = "Creates a Singleton view of a collection."]
        SingletonKind,
        #[doc = "Unrecognized transform type."]
        UnknownKind,
        #[doc = "Write transform."]
        WriteKind,
    }
    impl ExecutionStageSummaryKind {
        pub fn as_str(self) -> &'static str {
            match self {
                ExecutionStageSummaryKind::ConstantKind => "CONSTANT_KIND",
                ExecutionStageSummaryKind::FlattenKind => "FLATTEN_KIND",
                ExecutionStageSummaryKind::GroupByKeyKind => "GROUP_BY_KEY_KIND",
                ExecutionStageSummaryKind::ParDoKind => "PAR_DO_KIND",
                ExecutionStageSummaryKind::ReadKind => "READ_KIND",
                ExecutionStageSummaryKind::ShuffleKind => "SHUFFLE_KIND",
                ExecutionStageSummaryKind::SingletonKind => "SINGLETON_KIND",
                ExecutionStageSummaryKind::UnknownKind => "UNKNOWN_KIND",
                ExecutionStageSummaryKind::WriteKind => "WRITE_KIND",
            }
        }
    }
    impl ::std::fmt::Display for ExecutionStageSummaryKind {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for ExecutionStageSummaryKind {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for ExecutionStageSummaryKind {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "CONSTANT_KIND" => ExecutionStageSummaryKind::ConstantKind,
                "FLATTEN_KIND" => ExecutionStageSummaryKind::FlattenKind,
                "GROUP_BY_KEY_KIND" => ExecutionStageSummaryKind::GroupByKeyKind,
                "PAR_DO_KIND" => ExecutionStageSummaryKind::ParDoKind,
                "READ_KIND" => ExecutionStageSummaryKind::ReadKind,
                "SHUFFLE_KIND" => ExecutionStageSummaryKind::ShuffleKind,
                "SINGLETON_KIND" => ExecutionStageSummaryKind::SingletonKind,
                "UNKNOWN_KIND" => ExecutionStageSummaryKind::UnknownKind,
                "WRITE_KIND" => ExecutionStageSummaryKind::WriteKind,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::field_selector::FieldSelector for ExecutionStageSummaryKind {
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
    pub struct ExecutionStageSummary {
        #[doc = "Collections produced and consumed by component transforms of this stage."]
        #[serde(rename = "componentSource", default)]
        pub component_source: ::std::option::Option<Vec<crate::schemas::ComponentSource>>,
        #[doc = "Transforms that comprise this execution stage."]
        #[serde(rename = "componentTransform", default)]
        pub component_transform: ::std::option::Option<Vec<crate::schemas::ComponentTransform>>,
        #[doc = "Dataflow service generated id for this stage."]
        #[serde(rename = "id", default)]
        pub id: ::std::option::Option<String>,
        #[doc = "Input sources for this stage."]
        #[serde(rename = "inputSource", default)]
        pub input_source: ::std::option::Option<Vec<crate::schemas::StageSource>>,
        #[doc = "Type of tranform this stage is executing."]
        #[serde(rename = "kind", default)]
        pub kind: ::std::option::Option<crate::schemas::ExecutionStageSummaryKind>,
        #[doc = "Dataflow service generated name for this stage."]
        #[serde(rename = "name", default)]
        pub name: ::std::option::Option<String>,
        #[doc = "Output sources for this stage."]
        #[serde(rename = "outputSource", default)]
        pub output_source: ::std::option::Option<Vec<crate::schemas::StageSource>>,
    }
    impl ::field_selector::FieldSelector for ExecutionStageSummary {
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
    pub struct FailedLocation {
        #[doc = "The name of the [regional endpoint]\n(https://cloud.google.com/dataflow/docs/concepts/regional-endpoints) that\nfailed to respond."]
        #[serde(rename = "name", default)]
        pub name: ::std::option::Option<String>,
    }
    impl ::field_selector::FieldSelector for FailedLocation {
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
    pub struct FileIODetails {
        #[doc = "File Pattern used to access files by the connector."]
        #[serde(rename = "filePattern", default)]
        pub file_pattern: ::std::option::Option<String>,
    }
    impl ::field_selector::FieldSelector for FileIODetails {
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
    pub struct FlattenInstruction {
        #[doc = "Describes the inputs to the flatten instruction."]
        #[serde(rename = "inputs", default)]
        pub inputs: ::std::option::Option<Vec<crate::schemas::InstructionInput>>,
    }
    impl ::field_selector::FieldSelector for FlattenInstruction {
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
    pub struct FloatingPointList {
        #[doc = "Elements of the list."]
        #[serde(rename = "elements", default)]
        pub elements: ::std::option::Option<Vec<f64>>,
    }
    impl ::field_selector::FieldSelector for FloatingPointList {
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
    pub struct FloatingPointMean {
        #[doc = "The number of values being aggregated."]
        #[serde(rename = "count", default)]
        pub count: ::std::option::Option<crate::schemas::SplitInt64>,
        #[doc = "The sum of all values being aggregated."]
        #[serde(rename = "sum", default)]
        pub sum: ::std::option::Option<f64>,
    }
    impl ::field_selector::FieldSelector for FloatingPointMean {
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
    pub struct GetDebugConfigRequest {
        #[doc = "The internal component id for which debug configuration is\nrequested."]
        #[serde(rename = "componentId", default)]
        pub component_id: ::std::option::Option<String>,
        #[doc = "The [regional endpoint]\n(https://cloud.google.com/dataflow/docs/concepts/regional-endpoints) that\ncontains the job specified by job_id."]
        #[serde(rename = "location", default)]
        pub location: ::std::option::Option<String>,
        #[doc = "The worker id, i.e., VM hostname."]
        #[serde(rename = "workerId", default)]
        pub worker_id: ::std::option::Option<String>,
    }
    impl ::field_selector::FieldSelector for GetDebugConfigRequest {
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
    pub struct GetDebugConfigResponse {
        #[doc = "The encoded debug configuration for the requested component."]
        #[serde(rename = "config", default)]
        pub config: ::std::option::Option<String>,
    }
    impl ::field_selector::FieldSelector for GetDebugConfigResponse {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct GetTemplateResponse {
        #[doc = "The template metadata describing the template name, available\nparameters, etc."]
        #[serde(rename = "metadata", default)]
        pub metadata: ::std::option::Option<crate::schemas::TemplateMetadata>,
        #[doc = "The status of the get template request. Any problems with the\nrequest will be indicated in the error_details."]
        #[serde(rename = "status", default)]
        pub status: ::std::option::Option<crate::schemas::Status>,
    }
    impl ::field_selector::FieldSelector for GetTemplateResponse {
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
    pub struct Histogram {
        #[doc = "Counts of values in each bucket. For efficiency, prefix and trailing\nbuckets with count = 0 are elided. Buckets can store the full range of\nvalues of an unsigned long, with ULLONG_MAX falling into the 59th bucket\nwith range [1e19, 2e19)."]
        #[serde(rename = "bucketCounts", default)]
        pub bucket_counts: ::std::option::Option<Vec<i64>>,
        #[doc = "Starting index of first stored bucket. The non-inclusive upper-bound of\nthe ith bucket is given by:\npow(10,(i-first_bucket_offset)/3) * (1,2,5)[(i-first_bucket_offset)%3]"]
        #[serde(rename = "firstBucketOffset", default)]
        pub first_bucket_offset: ::std::option::Option<i32>,
    }
    impl ::field_selector::FieldSelector for Histogram {
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
    pub struct HotKeyDetection {
        #[doc = "The age of the hot key measured from when it was first detected."]
        #[serde(rename = "hotKeyAge", default)]
        pub hot_key_age: ::std::option::Option<String>,
        #[doc = "System-defined name of the step containing this hot key.\nUnique across the workflow."]
        #[serde(rename = "systemName", default)]
        pub system_name: ::std::option::Option<String>,
        #[doc = "User-provided name of the step that contains this hot key."]
        #[serde(rename = "userStepName", default)]
        pub user_step_name: ::std::option::Option<String>,
    }
    impl ::field_selector::FieldSelector for HotKeyDetection {
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
    pub struct InstructionInput {
        #[doc = "The output index (origin zero) within the producer."]
        #[serde(rename = "outputNum", default)]
        pub output_num: ::std::option::Option<i32>,
        #[doc = "The index (origin zero) of the parallel instruction that produces\nthe output to be consumed by this input.  This index is relative\nto the list of instructions in this input's instruction's\ncontaining MapTask."]
        #[serde(rename = "producerInstructionIndex", default)]
        pub producer_instruction_index: ::std::option::Option<i32>,
    }
    impl ::field_selector::FieldSelector for InstructionInput {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct InstructionOutput {
        #[doc = "The codec to use to encode data being written via this output."]
        #[serde(rename = "codec", default)]
        pub codec: ::std::option::Option<::std::collections::BTreeMap<String, ::serde_json::Value>>,
        #[doc = "The user-provided name of this output."]
        #[serde(rename = "name", default)]
        pub name: ::std::option::Option<String>,
        #[doc = "For system-generated byte and mean byte metrics, certain instructions\nshould only report the key size."]
        #[serde(rename = "onlyCountKeyBytes", default)]
        pub only_count_key_bytes: ::std::option::Option<bool>,
        #[doc = "For system-generated byte and mean byte metrics, certain instructions\nshould only report the value size."]
        #[serde(rename = "onlyCountValueBytes", default)]
        pub only_count_value_bytes: ::std::option::Option<bool>,
        #[doc = "System-defined name for this output in the original workflow graph.\nOutputs that do not contribute to an original instruction do not set this."]
        #[serde(rename = "originalName", default)]
        pub original_name: ::std::option::Option<String>,
        #[doc = "System-defined name of this output.\nUnique across the workflow."]
        #[serde(rename = "systemName", default)]
        pub system_name: ::std::option::Option<String>,
    }
    impl ::field_selector::FieldSelector for InstructionOutput {
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
    pub struct IntegerGauge {
        #[doc = "The time at which this value was measured. Measured as msecs from epoch."]
        #[serde(rename = "timestamp", default)]
        pub timestamp: ::std::option::Option<String>,
        #[doc = "The value of the variable represented by this gauge."]
        #[serde(rename = "value", default)]
        pub value: ::std::option::Option<crate::schemas::SplitInt64>,
    }
    impl ::field_selector::FieldSelector for IntegerGauge {
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
    pub struct IntegerList {
        #[doc = "Elements of the list."]
        #[serde(rename = "elements", default)]
        pub elements: ::std::option::Option<Vec<crate::schemas::SplitInt64>>,
    }
    impl ::field_selector::FieldSelector for IntegerList {
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
    pub struct IntegerMean {
        #[doc = "The number of values being aggregated."]
        #[serde(rename = "count", default)]
        pub count: ::std::option::Option<crate::schemas::SplitInt64>,
        #[doc = "The sum of all values being aggregated."]
        #[serde(rename = "sum", default)]
        pub sum: ::std::option::Option<crate::schemas::SplitInt64>,
    }
    impl ::field_selector::FieldSelector for IntegerMean {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum JobCurrentState {
        #[doc = "`JOB_STATE_CANCELLED` indicates that the job has been explicitly\ncancelled. This is a terminal job state. This state may only be\nset via a Cloud Dataflow `UpdateJob` call, and only if the job has not\nyet reached another terminal state."]
        JobStateCancelled,
        #[doc = "`JOB_STATE_CANCELLING` indicates that the job has been explicitly cancelled\nand is in the process of stopping.  Jobs that are cancelling may only\ntransition to `JOB_STATE_CANCELLED` or `JOB_STATE_FAILED`."]
        JobStateCancelling,
        #[doc = "`JOB_STATE_DONE` indicates that the job has successfully completed.\nThis is a terminal job state.  This state may be set by the Cloud Dataflow\nservice, as a transition from `JOB_STATE_RUNNING`. It may also be set via a\nCloud Dataflow `UpdateJob` call, if the job has not yet reached a terminal\nstate."]
        JobStateDone,
        #[doc = "`JOB_STATE_DRAINED` indicates that the job has been drained.\nA drained job terminated by stopping pulling from its input sources and\nprocessing any data that remained in-flight when draining was requested.\nThis state is a terminal state, may only be set by the Cloud Dataflow\nservice, and only as a transition from `JOB_STATE_DRAINING`."]
        JobStateDrained,
        #[doc = "`JOB_STATE_DRAINING` indicates that the job is in the process of draining.\nA draining job has stopped pulling from its input sources and is processing\nany data that remains in-flight. This state may be set via a Cloud Dataflow\n`UpdateJob` call, but only as a transition from `JOB_STATE_RUNNING`. Jobs\nthat are draining may only transition to `JOB_STATE_DRAINED`,\n`JOB_STATE_CANCELLED`, or `JOB_STATE_FAILED`."]
        JobStateDraining,
        #[doc = "`JOB_STATE_FAILED` indicates that the job has failed.  This is a\nterminal job state.  This state may only be set by the Cloud Dataflow\nservice, and only as a transition from `JOB_STATE_RUNNING`."]
        JobStateFailed,
        #[doc = "`JOB_STATE_PENDING` indicates that the job has been created but is not yet\nrunning.  Jobs that are pending may only transition to `JOB_STATE_RUNNING`,\nor `JOB_STATE_FAILED`."]
        JobStatePending,
        #[doc = "`JOB_STATE_QUEUED` indicates that the job has been created but is being\ndelayed until launch. Jobs that are queued may only transition to\n`JOB_STATE_PENDING` or `JOB_STATE_CANCELLED`."]
        JobStateQueued,
        #[doc = "`JOB_STATE_RUNNING` indicates that the job is currently running."]
        JobStateRunning,
        #[doc = "`JOB_STATE_STOPPED` indicates that the job has not\nyet started to run."]
        JobStateStopped,
        #[doc = "The job's run state isn't specified."]
        JobStateUnknown,
        #[doc = "`JOB_STATE_UPDATED` indicates that the job was successfully updated,\nmeaning that this job was stopped and another job was started, inheriting\nstate from this one. This is a terminal job state. This state may only be\nset by the Cloud Dataflow service, and only as a transition from\n`JOB_STATE_RUNNING`."]
        JobStateUpdated,
    }
    impl JobCurrentState {
        pub fn as_str(self) -> &'static str {
            match self {
                JobCurrentState::JobStateCancelled => "JOB_STATE_CANCELLED",
                JobCurrentState::JobStateCancelling => "JOB_STATE_CANCELLING",
                JobCurrentState::JobStateDone => "JOB_STATE_DONE",
                JobCurrentState::JobStateDrained => "JOB_STATE_DRAINED",
                JobCurrentState::JobStateDraining => "JOB_STATE_DRAINING",
                JobCurrentState::JobStateFailed => "JOB_STATE_FAILED",
                JobCurrentState::JobStatePending => "JOB_STATE_PENDING",
                JobCurrentState::JobStateQueued => "JOB_STATE_QUEUED",
                JobCurrentState::JobStateRunning => "JOB_STATE_RUNNING",
                JobCurrentState::JobStateStopped => "JOB_STATE_STOPPED",
                JobCurrentState::JobStateUnknown => "JOB_STATE_UNKNOWN",
                JobCurrentState::JobStateUpdated => "JOB_STATE_UPDATED",
            }
        }
    }
    impl ::std::fmt::Display for JobCurrentState {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for JobCurrentState {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for JobCurrentState {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "JOB_STATE_CANCELLED" => JobCurrentState::JobStateCancelled,
                "JOB_STATE_CANCELLING" => JobCurrentState::JobStateCancelling,
                "JOB_STATE_DONE" => JobCurrentState::JobStateDone,
                "JOB_STATE_DRAINED" => JobCurrentState::JobStateDrained,
                "JOB_STATE_DRAINING" => JobCurrentState::JobStateDraining,
                "JOB_STATE_FAILED" => JobCurrentState::JobStateFailed,
                "JOB_STATE_PENDING" => JobCurrentState::JobStatePending,
                "JOB_STATE_QUEUED" => JobCurrentState::JobStateQueued,
                "JOB_STATE_RUNNING" => JobCurrentState::JobStateRunning,
                "JOB_STATE_STOPPED" => JobCurrentState::JobStateStopped,
                "JOB_STATE_UNKNOWN" => JobCurrentState::JobStateUnknown,
                "JOB_STATE_UPDATED" => JobCurrentState::JobStateUpdated,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::field_selector::FieldSelector for JobCurrentState {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum JobType {
        #[doc = "A batch job with a well-defined end point: data is read, data is\nprocessed, data is written, and the job is done."]
        JobTypeBatch,
        #[doc = "A continuously streaming job with no end: data is read,\nprocessed, and written continuously."]
        JobTypeStreaming,
        #[doc = "The type of the job is unspecified, or unknown."]
        JobTypeUnknown,
    }
    impl JobType {
        pub fn as_str(self) -> &'static str {
            match self {
                JobType::JobTypeBatch => "JOB_TYPE_BATCH",
                JobType::JobTypeStreaming => "JOB_TYPE_STREAMING",
                JobType::JobTypeUnknown => "JOB_TYPE_UNKNOWN",
            }
        }
    }
    impl ::std::fmt::Display for JobType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for JobType {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for JobType {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "JOB_TYPE_BATCH" => JobType::JobTypeBatch,
                "JOB_TYPE_STREAMING" => JobType::JobTypeStreaming,
                "JOB_TYPE_UNKNOWN" => JobType::JobTypeUnknown,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::field_selector::FieldSelector for JobType {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum JobRequestedState {
        #[doc = "`JOB_STATE_CANCELLED` indicates that the job has been explicitly\ncancelled. This is a terminal job state. This state may only be\nset via a Cloud Dataflow `UpdateJob` call, and only if the job has not\nyet reached another terminal state."]
        JobStateCancelled,
        #[doc = "`JOB_STATE_CANCELLING` indicates that the job has been explicitly cancelled\nand is in the process of stopping.  Jobs that are cancelling may only\ntransition to `JOB_STATE_CANCELLED` or `JOB_STATE_FAILED`."]
        JobStateCancelling,
        #[doc = "`JOB_STATE_DONE` indicates that the job has successfully completed.\nThis is a terminal job state.  This state may be set by the Cloud Dataflow\nservice, as a transition from `JOB_STATE_RUNNING`. It may also be set via a\nCloud Dataflow `UpdateJob` call, if the job has not yet reached a terminal\nstate."]
        JobStateDone,
        #[doc = "`JOB_STATE_DRAINED` indicates that the job has been drained.\nA drained job terminated by stopping pulling from its input sources and\nprocessing any data that remained in-flight when draining was requested.\nThis state is a terminal state, may only be set by the Cloud Dataflow\nservice, and only as a transition from `JOB_STATE_DRAINING`."]
        JobStateDrained,
        #[doc = "`JOB_STATE_DRAINING` indicates that the job is in the process of draining.\nA draining job has stopped pulling from its input sources and is processing\nany data that remains in-flight. This state may be set via a Cloud Dataflow\n`UpdateJob` call, but only as a transition from `JOB_STATE_RUNNING`. Jobs\nthat are draining may only transition to `JOB_STATE_DRAINED`,\n`JOB_STATE_CANCELLED`, or `JOB_STATE_FAILED`."]
        JobStateDraining,
        #[doc = "`JOB_STATE_FAILED` indicates that the job has failed.  This is a\nterminal job state.  This state may only be set by the Cloud Dataflow\nservice, and only as a transition from `JOB_STATE_RUNNING`."]
        JobStateFailed,
        #[doc = "`JOB_STATE_PENDING` indicates that the job has been created but is not yet\nrunning.  Jobs that are pending may only transition to `JOB_STATE_RUNNING`,\nor `JOB_STATE_FAILED`."]
        JobStatePending,
        #[doc = "`JOB_STATE_QUEUED` indicates that the job has been created but is being\ndelayed until launch. Jobs that are queued may only transition to\n`JOB_STATE_PENDING` or `JOB_STATE_CANCELLED`."]
        JobStateQueued,
        #[doc = "`JOB_STATE_RUNNING` indicates that the job is currently running."]
        JobStateRunning,
        #[doc = "`JOB_STATE_STOPPED` indicates that the job has not\nyet started to run."]
        JobStateStopped,
        #[doc = "The job's run state isn't specified."]
        JobStateUnknown,
        #[doc = "`JOB_STATE_UPDATED` indicates that the job was successfully updated,\nmeaning that this job was stopped and another job was started, inheriting\nstate from this one. This is a terminal job state. This state may only be\nset by the Cloud Dataflow service, and only as a transition from\n`JOB_STATE_RUNNING`."]
        JobStateUpdated,
    }
    impl JobRequestedState {
        pub fn as_str(self) -> &'static str {
            match self {
                JobRequestedState::JobStateCancelled => "JOB_STATE_CANCELLED",
                JobRequestedState::JobStateCancelling => "JOB_STATE_CANCELLING",
                JobRequestedState::JobStateDone => "JOB_STATE_DONE",
                JobRequestedState::JobStateDrained => "JOB_STATE_DRAINED",
                JobRequestedState::JobStateDraining => "JOB_STATE_DRAINING",
                JobRequestedState::JobStateFailed => "JOB_STATE_FAILED",
                JobRequestedState::JobStatePending => "JOB_STATE_PENDING",
                JobRequestedState::JobStateQueued => "JOB_STATE_QUEUED",
                JobRequestedState::JobStateRunning => "JOB_STATE_RUNNING",
                JobRequestedState::JobStateStopped => "JOB_STATE_STOPPED",
                JobRequestedState::JobStateUnknown => "JOB_STATE_UNKNOWN",
                JobRequestedState::JobStateUpdated => "JOB_STATE_UPDATED",
            }
        }
    }
    impl ::std::fmt::Display for JobRequestedState {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for JobRequestedState {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for JobRequestedState {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "JOB_STATE_CANCELLED" => JobRequestedState::JobStateCancelled,
                "JOB_STATE_CANCELLING" => JobRequestedState::JobStateCancelling,
                "JOB_STATE_DONE" => JobRequestedState::JobStateDone,
                "JOB_STATE_DRAINED" => JobRequestedState::JobStateDrained,
                "JOB_STATE_DRAINING" => JobRequestedState::JobStateDraining,
                "JOB_STATE_FAILED" => JobRequestedState::JobStateFailed,
                "JOB_STATE_PENDING" => JobRequestedState::JobStatePending,
                "JOB_STATE_QUEUED" => JobRequestedState::JobStateQueued,
                "JOB_STATE_RUNNING" => JobRequestedState::JobStateRunning,
                "JOB_STATE_STOPPED" => JobRequestedState::JobStateStopped,
                "JOB_STATE_UNKNOWN" => JobRequestedState::JobStateUnknown,
                "JOB_STATE_UPDATED" => JobRequestedState::JobStateUpdated,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::field_selector::FieldSelector for JobRequestedState {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct Job {
        #[doc = "The client's unique identifier of the job, re-used across retried attempts.\nIf this field is set, the service will ensure its uniqueness.\nThe request to create a job will fail if the service has knowledge of a\npreviously submitted job with the same client's ID and job name.\nThe caller may use this field to ensure idempotence of job\ncreation across retried attempts to create a job.\nBy default, the field is empty and, in that case, the service ignores it."]
        #[serde(rename = "clientRequestId", default)]
        pub client_request_id: ::std::option::Option<String>,
        #[doc = "The timestamp when the job was initially created. Immutable and set by the\nCloud Dataflow service."]
        #[serde(rename = "createTime", default)]
        pub create_time: ::std::option::Option<String>,
        #[doc = "If this is specified, the job's initial state is populated from the given\nsnapshot."]
        #[serde(rename = "createdFromSnapshotId", default)]
        pub created_from_snapshot_id: ::std::option::Option<String>,
        #[doc = "The current state of the job.\n\nJobs are created in the `JOB_STATE_STOPPED` state unless otherwise\nspecified.\n\nA job in the `JOB_STATE_RUNNING` state may asynchronously enter a\nterminal state. After a job has reached a terminal state, no\nfurther state updates may be made.\n\nThis field may be mutated by the Cloud Dataflow service;\ncallers cannot mutate it."]
        #[serde(rename = "currentState", default)]
        pub current_state: ::std::option::Option<crate::schemas::JobCurrentState>,
        #[doc = "The timestamp associated with the current state."]
        #[serde(rename = "currentStateTime", default)]
        pub current_state_time: ::std::option::Option<String>,
        #[doc = "The environment for the job."]
        #[serde(rename = "environment", default)]
        pub environment: ::std::option::Option<crate::schemas::Environment>,
        #[doc = "Deprecated."]
        #[serde(rename = "executionInfo", default)]
        pub execution_info: ::std::option::Option<crate::schemas::JobExecutionInfo>,
        #[doc = "The unique ID of this job.\n\nThis field is set by the Cloud Dataflow service when the Job is\ncreated, and is immutable for the life of the job."]
        #[serde(rename = "id", default)]
        pub id: ::std::option::Option<String>,
        #[doc = "This field is populated by the Dataflow service to support filtering jobs\nby the metadata values provided here. Populated for ListJobs and all GetJob\nviews SUMMARY and higher."]
        #[serde(rename = "jobMetadata", default)]
        pub job_metadata: ::std::option::Option<crate::schemas::JobMetadata>,
        #[doc = "User-defined labels for this job.\n\nThe labels map can contain no more than 64 entries.  Entries of the labels\nmap are UTF8 strings that comply with the following restrictions:\n\n* Keys must conform to regexp:  \\p{Ll}\\p{Lo}{0,62}\n* Values must conform to regexp:  [\\p{Ll}\\p{Lo}\\p{N}_-]{0,63}\n* Both keys and values are additionally constrained to be <= 128 bytes in\n  size."]
        #[serde(rename = "labels", default)]
        pub labels: ::std::option::Option<::std::collections::BTreeMap<String, String>>,
        #[doc = "The [regional endpoint]\n(https://cloud.google.com/dataflow/docs/concepts/regional-endpoints) that\ncontains this job."]
        #[serde(rename = "location", default)]
        pub location: ::std::option::Option<String>,
        #[doc = "The user-specified Cloud Dataflow job name.\n\nOnly one Job with a given name may exist in a project at any\ngiven time. If a caller attempts to create a Job with the same\nname as an already-existing Job, the attempt returns the\nexisting Job.\n\nThe name must match the regular expression\n`[a-z]([-a-z0-9]{0,38}[a-z0-9])?`"]
        #[serde(rename = "name", default)]
        pub name: ::std::option::Option<String>,
        #[doc = "Preliminary field: The format of this data may change at any time.\nA description of the user pipeline and stages through which it is executed.\nCreated by Cloud Dataflow service.  Only retrieved with\nJOB_VIEW_DESCRIPTION or JOB_VIEW_ALL."]
        #[serde(rename = "pipelineDescription", default)]
        pub pipeline_description: ::std::option::Option<crate::schemas::PipelineDescription>,
        #[doc = "The ID of the Cloud Platform project that the job belongs to."]
        #[serde(rename = "projectId", default)]
        pub project_id: ::std::option::Option<String>,
        #[doc = "The type of Cloud Dataflow job."]
        #[serde(rename = "type", default)]
        pub r#type: ::std::option::Option<crate::schemas::JobType>,
        #[doc = "If this job is an update of an existing job, this field is the job ID\nof the job it replaced.\n\nWhen sending a `CreateJobRequest`, you can update a job by specifying it\nhere. The job named here is stopped, and its intermediate state is\ntransferred to this job."]
        #[serde(rename = "replaceJobId", default)]
        pub replace_job_id: ::std::option::Option<String>,
        #[doc = "If another job is an update of this job (and thus, this job is in\n`JOB_STATE_UPDATED`), this field contains the ID of that job."]
        #[serde(rename = "replacedByJobId", default)]
        pub replaced_by_job_id: ::std::option::Option<String>,
        #[doc = "The job's requested state.\n\n`UpdateJob` may be used to switch between the `JOB_STATE_STOPPED` and\n`JOB_STATE_RUNNING` states, by setting requested_state.  `UpdateJob` may\nalso be used to directly set a job's requested state to\n`JOB_STATE_CANCELLED` or `JOB_STATE_DONE`, irrevocably terminating the\njob if it has not already reached a terminal state."]
        #[serde(rename = "requestedState", default)]
        pub requested_state: ::std::option::Option<crate::schemas::JobRequestedState>,
        #[doc = "This field may be mutated by the Cloud Dataflow service;\ncallers cannot mutate it."]
        #[serde(rename = "stageStates", default)]
        pub stage_states: ::std::option::Option<Vec<crate::schemas::ExecutionStageState>>,
        #[doc = "The timestamp when the job was started (transitioned to JOB_STATE_PENDING).\nFlexible resource scheduling jobs are started with some delay after job\ncreation, so start_time is unset before start and is updated when the\njob is started by the Cloud Dataflow service. For other jobs, start_time\nalways equals to create_time and is immutable and set by the Cloud Dataflow\nservice."]
        #[serde(rename = "startTime", default)]
        pub start_time: ::std::option::Option<String>,
        #[doc = "Exactly one of step or steps_location should be specified.\n\nThe top-level steps that constitute the entire job."]
        #[serde(rename = "steps", default)]
        pub steps: ::std::option::Option<Vec<crate::schemas::Step>>,
        #[doc = "The GCS location where the steps are stored."]
        #[serde(rename = "stepsLocation", default)]
        pub steps_location: ::std::option::Option<String>,
        #[doc = "A set of files the system should be aware of that are used\nfor temporary storage. These temporary files will be\nremoved on job completion.\nNo duplicates are allowed.\nNo file patterns are supported.\n\nThe supported files are:\n\nGoogle Cloud Storage:\n\nstorage.googleapis.com/{bucket}/{object}\nbucket.storage.googleapis.com/{object}"]
        #[serde(rename = "tempFiles", default)]
        pub temp_files: ::std::option::Option<Vec<String>>,
        #[doc = "The map of transform name prefixes of the job to be replaced to the\ncorresponding name prefixes of the new job."]
        #[serde(rename = "transformNameMapping", default)]
        pub transform_name_mapping:
            ::std::option::Option<::std::collections::BTreeMap<String, String>>,
    }
    impl ::field_selector::FieldSelector for Job {
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
    pub struct JobExecutionInfo {
        #[doc = "A mapping from each stage to the information about that stage."]
        #[serde(rename = "stages", default)]
        pub stages: ::std::option::Option<
            ::std::collections::BTreeMap<String, crate::schemas::JobExecutionStageInfo>,
        >,
    }
    impl ::field_selector::FieldSelector for JobExecutionInfo {
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
    pub struct JobExecutionStageInfo {
        #[doc = "The steps associated with the execution stage.\nNote that stages may have several steps, and that a given step\nmight be run by more than one stage."]
        #[serde(rename = "stepName", default)]
        pub step_name: ::std::option::Option<Vec<String>>,
    }
    impl ::field_selector::FieldSelector for JobExecutionStageInfo {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum JobMessageMessageImportance {
        #[doc = "The message is at the 'basic' level: useful for keeping\ntrack of the execution of a Dataflow pipeline.  Typically,\nDataflow pipeline runners display log messages at this level by\ndefault, and these messages are displayed by default in the\nDataflow monitoring UI."]
        JobMessageBasic,
        #[doc = "The message is at the 'debug' level: typically only useful for\nsoftware engineers working on the code the job is running.\nTypically, Dataflow pipeline runners do not display log messages\nat this level by default."]
        JobMessageDebug,
        #[doc = "The message is at the 'detailed' level: somewhat verbose, but\npotentially useful to users.  Typically, Dataflow pipeline\nrunners do not display log messages at this level by default.\nThese messages are displayed by default in the Dataflow\nmonitoring UI."]
        JobMessageDetailed,
        #[doc = "The message is at the 'error' level: indicating a condition\npreventing a job from succeeding.  Typically, Dataflow pipeline\nrunners display log messages at this level by default, and these\nmessages are displayed by default in the Dataflow monitoring UI."]
        JobMessageError,
        #[doc = "The message importance isn't specified, or is unknown."]
        JobMessageImportanceUnknown,
        #[doc = "The message is at the 'warning' level: indicating a condition\npertaining to a job which may require human intervention.\nTypically, Dataflow pipeline runners display log messages at this\nlevel by default, and these messages are displayed by default in\nthe Dataflow monitoring UI."]
        JobMessageWarning,
    }
    impl JobMessageMessageImportance {
        pub fn as_str(self) -> &'static str {
            match self {
                JobMessageMessageImportance::JobMessageBasic => "JOB_MESSAGE_BASIC",
                JobMessageMessageImportance::JobMessageDebug => "JOB_MESSAGE_DEBUG",
                JobMessageMessageImportance::JobMessageDetailed => "JOB_MESSAGE_DETAILED",
                JobMessageMessageImportance::JobMessageError => "JOB_MESSAGE_ERROR",
                JobMessageMessageImportance::JobMessageImportanceUnknown => {
                    "JOB_MESSAGE_IMPORTANCE_UNKNOWN"
                }
                JobMessageMessageImportance::JobMessageWarning => "JOB_MESSAGE_WARNING",
            }
        }
    }
    impl ::std::fmt::Display for JobMessageMessageImportance {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for JobMessageMessageImportance {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for JobMessageMessageImportance {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "JOB_MESSAGE_BASIC" => JobMessageMessageImportance::JobMessageBasic,
                "JOB_MESSAGE_DEBUG" => JobMessageMessageImportance::JobMessageDebug,
                "JOB_MESSAGE_DETAILED" => JobMessageMessageImportance::JobMessageDetailed,
                "JOB_MESSAGE_ERROR" => JobMessageMessageImportance::JobMessageError,
                "JOB_MESSAGE_IMPORTANCE_UNKNOWN" => {
                    JobMessageMessageImportance::JobMessageImportanceUnknown
                }
                "JOB_MESSAGE_WARNING" => JobMessageMessageImportance::JobMessageWarning,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::field_selector::FieldSelector for JobMessageMessageImportance {
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
    pub struct JobMessage {
        #[doc = "Deprecated."]
        #[serde(rename = "id", default)]
        pub id: ::std::option::Option<String>,
        #[doc = "Importance level of the message."]
        #[serde(rename = "messageImportance", default)]
        pub message_importance: ::std::option::Option<crate::schemas::JobMessageMessageImportance>,
        #[doc = "The text of the message."]
        #[serde(rename = "messageText", default)]
        pub message_text: ::std::option::Option<String>,
        #[doc = "The timestamp of the message."]
        #[serde(rename = "time", default)]
        pub time: ::std::option::Option<String>,
    }
    impl ::field_selector::FieldSelector for JobMessage {
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
    pub struct JobMetadata {
        #[doc = "Identification of a BigTable source used in the Dataflow job."]
        #[serde(rename = "bigTableDetails", default)]
        pub big_table_details: ::std::option::Option<Vec<crate::schemas::BigTableIODetails>>,
        #[doc = "Identification of a BigQuery source used in the Dataflow job."]
        #[serde(rename = "bigqueryDetails", default)]
        pub bigquery_details: ::std::option::Option<Vec<crate::schemas::BigQueryIODetails>>,
        #[doc = "Identification of a Datastore source used in the Dataflow job."]
        #[serde(rename = "datastoreDetails", default)]
        pub datastore_details: ::std::option::Option<Vec<crate::schemas::DatastoreIODetails>>,
        #[doc = "Identification of a File source used in the Dataflow job."]
        #[serde(rename = "fileDetails", default)]
        pub file_details: ::std::option::Option<Vec<crate::schemas::FileIODetails>>,
        #[doc = "Identification of a PubSub source used in the Dataflow job."]
        #[serde(rename = "pubsubDetails", default)]
        pub pubsub_details: ::std::option::Option<Vec<crate::schemas::PubSubIODetails>>,
        #[doc = "The SDK version used to run the job."]
        #[serde(rename = "sdkVersion", default)]
        pub sdk_version: ::std::option::Option<crate::schemas::SdkVersion>,
        #[doc = "Identification of a Spanner source used in the Dataflow job."]
        #[serde(rename = "spannerDetails", default)]
        pub spanner_details: ::std::option::Option<Vec<crate::schemas::SpannerIODetails>>,
    }
    impl ::field_selector::FieldSelector for JobMetadata {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct JobMetrics {
        #[doc = "Timestamp as of which metric values are current."]
        #[serde(rename = "metricTime", default)]
        pub metric_time: ::std::option::Option<String>,
        #[doc = "All metrics for this job."]
        #[serde(rename = "metrics", default)]
        pub metrics: ::std::option::Option<Vec<crate::schemas::MetricUpdate>>,
    }
    impl ::field_selector::FieldSelector for JobMetrics {
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
    pub struct KeyRangeDataDiskAssignment {
        #[doc = "The name of the data disk where data for this range is stored.\nThis name is local to the Google Cloud Platform project and uniquely\nidentifies the disk within that project, for example\n\"myproject-1014-104817-4c2-harness-0-disk-1\"."]
        #[serde(rename = "dataDisk", default)]
        pub data_disk: ::std::option::Option<String>,
        #[doc = "The end (exclusive) of the key range."]
        #[serde(rename = "end", default)]
        pub end: ::std::option::Option<String>,
        #[doc = "The start (inclusive) of the key range."]
        #[serde(rename = "start", default)]
        pub start: ::std::option::Option<String>,
    }
    impl ::field_selector::FieldSelector for KeyRangeDataDiskAssignment {
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
    pub struct KeyRangeLocation {
        #[doc = "The name of the data disk where data for this range is stored.\nThis name is local to the Google Cloud Platform project and uniquely\nidentifies the disk within that project, for example\n\"myproject-1014-104817-4c2-harness-0-disk-1\"."]
        #[serde(rename = "dataDisk", default)]
        pub data_disk: ::std::option::Option<String>,
        #[doc = "The physical location of this range assignment to be used for\nstreaming computation cross-worker message delivery."]
        #[serde(rename = "deliveryEndpoint", default)]
        pub delivery_endpoint: ::std::option::Option<String>,
        #[doc = "DEPRECATED. The location of the persistent state for this range, as a\npersistent directory in the worker local filesystem."]
        #[serde(rename = "deprecatedPersistentDirectory", default)]
        pub deprecated_persistent_directory: ::std::option::Option<String>,
        #[doc = "The end (exclusive) of the key range."]
        #[serde(rename = "end", default)]
        pub end: ::std::option::Option<String>,
        #[doc = "The start (inclusive) of the key range."]
        #[serde(rename = "start", default)]
        pub start: ::std::option::Option<String>,
    }
    impl ::field_selector::FieldSelector for KeyRangeLocation {
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
    pub struct LaunchTemplateParameters {
        #[doc = "The runtime environment for the job."]
        #[serde(rename = "environment", default)]
        pub environment: ::std::option::Option<crate::schemas::RuntimeEnvironment>,
        #[doc = "Required. The job name to use for the created job."]
        #[serde(rename = "jobName", default)]
        pub job_name: ::std::option::Option<String>,
        #[doc = "The runtime parameters to pass to the job."]
        #[serde(rename = "parameters", default)]
        pub parameters: ::std::option::Option<::std::collections::BTreeMap<String, String>>,
        #[doc = "Only applicable when updating a pipeline. Map of transform name prefixes of\nthe job to be replaced to the corresponding name prefixes of the new job."]
        #[serde(rename = "transformNameMapping", default)]
        pub transform_name_mapping:
            ::std::option::Option<::std::collections::BTreeMap<String, String>>,
        #[doc = "If set, replace the existing pipeline with the name specified by jobName\nwith this pipeline, preserving state."]
        #[serde(rename = "update", default)]
        pub update: ::std::option::Option<bool>,
    }
    impl ::field_selector::FieldSelector for LaunchTemplateParameters {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct LaunchTemplateResponse {
        #[doc = "The job that was launched, if the request was not a dry run and\nthe job was successfully launched."]
        #[serde(rename = "job", default)]
        pub job: ::std::option::Option<crate::schemas::Job>,
    }
    impl ::field_selector::FieldSelector for LaunchTemplateResponse {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct LeaseWorkItemRequest {
        #[doc = "The current timestamp at the worker."]
        #[serde(rename = "currentWorkerTime", default)]
        pub current_worker_time: ::std::option::Option<String>,
        #[doc = "The [regional endpoint]\n(https://cloud.google.com/dataflow/docs/concepts/regional-endpoints) that\ncontains the WorkItem's job."]
        #[serde(rename = "location", default)]
        pub location: ::std::option::Option<String>,
        #[doc = "The initial lease period."]
        #[serde(rename = "requestedLeaseDuration", default)]
        pub requested_lease_duration: ::std::option::Option<String>,
        #[doc = "Untranslated bag-of-bytes WorkRequest from UnifiedWorker."]
        #[serde(rename = "unifiedWorkerRequest", default)]
        pub unified_worker_request:
            ::std::option::Option<::std::collections::BTreeMap<String, ::serde_json::Value>>,
        #[doc = "Filter for WorkItem type."]
        #[serde(rename = "workItemTypes", default)]
        pub work_item_types: ::std::option::Option<Vec<String>>,
        #[doc = "Worker capabilities. WorkItems might be limited to workers with specific\ncapabilities."]
        #[serde(rename = "workerCapabilities", default)]
        pub worker_capabilities: ::std::option::Option<Vec<String>>,
        #[doc = "Identifies the worker leasing work -- typically the ID of the\nvirtual machine running the worker."]
        #[serde(rename = "workerId", default)]
        pub worker_id: ::std::option::Option<String>,
    }
    impl ::field_selector::FieldSelector for LeaseWorkItemRequest {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct LeaseWorkItemResponse {
        #[doc = "Untranslated bag-of-bytes WorkResponse for UnifiedWorker."]
        #[serde(rename = "unifiedWorkerResponse", default)]
        pub unified_worker_response:
            ::std::option::Option<::std::collections::BTreeMap<String, ::serde_json::Value>>,
        #[doc = "A list of the leased WorkItems."]
        #[serde(rename = "workItems", default)]
        pub work_items: ::std::option::Option<Vec<crate::schemas::WorkItem>>,
    }
    impl ::field_selector::FieldSelector for LeaseWorkItemResponse {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct ListJobMessagesResponse {
        #[doc = "Autoscaling events in ascending timestamp order."]
        #[serde(rename = "autoscalingEvents", default)]
        pub autoscaling_events: ::std::option::Option<Vec<crate::schemas::AutoscalingEvent>>,
        #[doc = "Messages in ascending timestamp order."]
        #[serde(rename = "jobMessages", default)]
        pub job_messages: ::std::option::Option<Vec<crate::schemas::JobMessage>>,
        #[doc = "The token to obtain the next page of results if there are more."]
        #[serde(rename = "nextPageToken", default)]
        pub next_page_token: ::std::option::Option<String>,
    }
    impl ::field_selector::FieldSelector for ListJobMessagesResponse {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct ListJobsResponse {
        #[doc = "Zero or more messages describing the [regional endpoints]\n(https://cloud.google.com/dataflow/docs/concepts/regional-endpoints) that\nfailed to respond."]
        #[serde(rename = "failedLocation", default)]
        pub failed_location: ::std::option::Option<Vec<crate::schemas::FailedLocation>>,
        #[doc = "A subset of the requested job information."]
        #[serde(rename = "jobs", default)]
        pub jobs: ::std::option::Option<Vec<crate::schemas::Job>>,
        #[doc = "Set if there may be more results than fit in this response."]
        #[serde(rename = "nextPageToken", default)]
        pub next_page_token: ::std::option::Option<String>,
    }
    impl ::field_selector::FieldSelector for ListJobsResponse {
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
    pub struct ListSnapshotsResponse {
        #[doc = "Returned snapshots."]
        #[serde(rename = "snapshots", default)]
        pub snapshots: ::std::option::Option<Vec<crate::schemas::Snapshot>>,
    }
    impl ::field_selector::FieldSelector for ListSnapshotsResponse {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct MapTask {
        #[doc = "Counter prefix that can be used to prefix counters. Not currently used in\nDataflow."]
        #[serde(rename = "counterPrefix", default)]
        pub counter_prefix: ::std::option::Option<String>,
        #[doc = "The instructions in the MapTask."]
        #[serde(rename = "instructions", default)]
        pub instructions: ::std::option::Option<Vec<crate::schemas::ParallelInstruction>>,
        #[doc = "System-defined name of the stage containing this MapTask.\nUnique across the workflow."]
        #[serde(rename = "stageName", default)]
        pub stage_name: ::std::option::Option<String>,
        #[doc = "System-defined name of this MapTask.\nUnique across the workflow."]
        #[serde(rename = "systemName", default)]
        pub system_name: ::std::option::Option<String>,
    }
    impl ::field_selector::FieldSelector for MapTask {
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
    pub struct MetricShortId {
        #[doc = "The index of the corresponding metric in\nthe ReportWorkItemStatusRequest. Required."]
        #[serde(rename = "metricIndex", default)]
        pub metric_index: ::std::option::Option<i32>,
        #[doc = "The service-generated short identifier for the metric."]
        #[serde(rename = "shortId", default)]
        #[serde(with = "crate::parsed_string")]
        pub short_id: ::std::option::Option<i64>,
    }
    impl ::field_selector::FieldSelector for MetricShortId {
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
    pub struct MetricStructuredName {
        #[doc = "Zero or more labeled fields which identify the part of the job this\nmetric is associated with, such as the name of a step or collection.\n\nFor example, built-in counters associated with steps will have\ncontext['step'] = <step-name>. Counters associated with PCollections\nin the SDK will have context['pcollection'] = <pcollection-name>."]
        #[serde(rename = "context", default)]
        pub context: ::std::option::Option<::std::collections::BTreeMap<String, String>>,
        #[doc = "Worker-defined metric name."]
        #[serde(rename = "name", default)]
        pub name: ::std::option::Option<String>,
        #[doc = "Origin (namespace) of metric name. May be blank for user-define metrics;\nwill be \"dataflow\" for metrics defined by the Dataflow service or SDK."]
        #[serde(rename = "origin", default)]
        pub origin: ::std::option::Option<String>,
    }
    impl ::field_selector::FieldSelector for MetricStructuredName {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct MetricUpdate {
        #[doc = "True if this metric is reported as the total cumulative aggregate\nvalue accumulated since the worker started working on this WorkItem.\nBy default this is false, indicating that this metric is reported\nas a delta that is not associated with any WorkItem."]
        #[serde(rename = "cumulative", default)]
        pub cumulative: ::std::option::Option<bool>,
        #[doc = "A struct value describing properties of a distribution of numeric values."]
        #[serde(rename = "distribution", default)]
        pub distribution: ::std::option::Option<::serde_json::Value>,
        #[doc = "A struct value describing properties of a Gauge.\nMetrics of gauge type show the value of a metric across time, and is\naggregated based on the newest value."]
        #[serde(rename = "gauge", default)]
        pub gauge: ::std::option::Option<::serde_json::Value>,
        #[doc = "Worker-computed aggregate value for internal use by the Dataflow\nservice."]
        #[serde(rename = "internal", default)]
        pub internal: ::std::option::Option<::serde_json::Value>,
        #[doc = "Metric aggregation kind.  The possible metric aggregation kinds are\n\"Sum\", \"Max\", \"Min\", \"Mean\", \"Set\", \"And\", \"Or\", and \"Distribution\".\nThe specified aggregation kind is case-insensitive.\n\nIf omitted, this is not an aggregated value but instead\na single metric sample value."]
        #[serde(rename = "kind", default)]
        pub kind: ::std::option::Option<String>,
        #[doc = "Worker-computed aggregate value for the \"Mean\" aggregation kind.\nThis holds the count of the aggregated values and is used in combination\nwith mean_sum above to obtain the actual mean aggregate value.\nThe only possible value type is Long."]
        #[serde(rename = "meanCount", default)]
        pub mean_count: ::std::option::Option<::serde_json::Value>,
        #[doc = "Worker-computed aggregate value for the \"Mean\" aggregation kind.\nThis holds the sum of the aggregated values and is used in combination\nwith mean_count below to obtain the actual mean aggregate value.\nThe only possible value types are Long and Double."]
        #[serde(rename = "meanSum", default)]
        pub mean_sum: ::std::option::Option<::serde_json::Value>,
        #[doc = "Name of the metric."]
        #[serde(rename = "name", default)]
        pub name: ::std::option::Option<crate::schemas::MetricStructuredName>,
        #[doc = "Worker-computed aggregate value for aggregation kinds \"Sum\", \"Max\", \"Min\",\n\"And\", and \"Or\".  The possible value types are Long, Double, and Boolean."]
        #[serde(rename = "scalar", default)]
        pub scalar: ::std::option::Option<::serde_json::Value>,
        #[doc = "Worker-computed aggregate value for the \"Set\" aggregation kind.  The only\npossible value type is a list of Values whose type can be Long, Double,\nor String, according to the metric's type.  All Values in the list must\nbe of the same type."]
        #[serde(rename = "set", default)]
        pub set: ::std::option::Option<::serde_json::Value>,
        #[doc = "Timestamp associated with the metric value. Optional when workers are\nreporting work progress; it will be filled in responses from the\nmetrics API."]
        #[serde(rename = "updateTime", default)]
        pub update_time: ::std::option::Option<String>,
    }
    impl ::field_selector::FieldSelector for MetricUpdate {
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
    pub struct MountedDataDisk {
        #[doc = "The name of the data disk.\nThis name is local to the Google Cloud Platform project and uniquely\nidentifies the disk within that project, for example\n\"myproject-1014-104817-4c2-harness-0-disk-1\"."]
        #[serde(rename = "dataDisk", default)]
        pub data_disk: ::std::option::Option<String>,
    }
    impl ::field_selector::FieldSelector for MountedDataDisk {
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
    pub struct MultiOutputInfo {
        #[doc = "The id of the tag the user code will emit to this output by; this\nshould correspond to the tag of some SideInputInfo."]
        #[serde(rename = "tag", default)]
        pub tag: ::std::option::Option<String>,
    }
    impl ::field_selector::FieldSelector for MultiOutputInfo {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum NameAndKindKind {
        #[doc = "Aggregated value represents the logical 'and' of all contributed values."]
        And,
        #[doc = "Aggregated value captures statistics about a distribution."]
        Distribution,
        #[doc = "Counter aggregation kind was not set."]
        Invalid,
        #[doc = "Aggregated value tracks the latest value of a variable."]
        LatestValue,
        #[doc = "Aggregated value is the max of all contributed values."]
        Max,
        #[doc = "Aggregated value is the mean of all contributed values."]
        Mean,
        #[doc = "Aggregated value is the min of all contributed values."]
        Min,
        #[doc = "Aggregated value represents the logical 'or' of all contributed values."]
        Or,
        #[doc = "Aggregated value is a set of unique contributed values."]
        Set,
        #[doc = "Aggregated value is the sum of all contributed values."]
        Sum,
    }
    impl NameAndKindKind {
        pub fn as_str(self) -> &'static str {
            match self {
                NameAndKindKind::And => "AND",
                NameAndKindKind::Distribution => "DISTRIBUTION",
                NameAndKindKind::Invalid => "INVALID",
                NameAndKindKind::LatestValue => "LATEST_VALUE",
                NameAndKindKind::Max => "MAX",
                NameAndKindKind::Mean => "MEAN",
                NameAndKindKind::Min => "MIN",
                NameAndKindKind::Or => "OR",
                NameAndKindKind::Set => "SET",
                NameAndKindKind::Sum => "SUM",
            }
        }
    }
    impl ::std::fmt::Display for NameAndKindKind {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for NameAndKindKind {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for NameAndKindKind {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "AND" => NameAndKindKind::And,
                "DISTRIBUTION" => NameAndKindKind::Distribution,
                "INVALID" => NameAndKindKind::Invalid,
                "LATEST_VALUE" => NameAndKindKind::LatestValue,
                "MAX" => NameAndKindKind::Max,
                "MEAN" => NameAndKindKind::Mean,
                "MIN" => NameAndKindKind::Min,
                "OR" => NameAndKindKind::Or,
                "SET" => NameAndKindKind::Set,
                "SUM" => NameAndKindKind::Sum,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::field_selector::FieldSelector for NameAndKindKind {
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
    pub struct NameAndKind {
        #[doc = "Counter aggregation kind."]
        #[serde(rename = "kind", default)]
        pub kind: ::std::option::Option<crate::schemas::NameAndKindKind>,
        #[doc = "Name of the counter."]
        #[serde(rename = "name", default)]
        pub name: ::std::option::Option<String>,
    }
    impl ::field_selector::FieldSelector for NameAndKind {
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
    pub struct Package {
        #[doc = "The resource to read the package from. The supported resource type is:\n\nGoogle Cloud Storage:\n\nstorage.googleapis.com/{bucket}\nbucket.storage.googleapis.com/"]
        #[serde(rename = "location", default)]
        pub location: ::std::option::Option<String>,
        #[doc = "The name of the package."]
        #[serde(rename = "name", default)]
        pub name: ::std::option::Option<String>,
    }
    impl ::field_selector::FieldSelector for Package {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct ParDoInstruction {
        #[doc = "The input."]
        #[serde(rename = "input", default)]
        pub input: ::std::option::Option<crate::schemas::InstructionInput>,
        #[doc = "Information about each of the outputs, if user_fn is a  MultiDoFn."]
        #[serde(rename = "multiOutputInfos", default)]
        pub multi_output_infos: ::std::option::Option<Vec<crate::schemas::MultiOutputInfo>>,
        #[doc = "The number of outputs."]
        #[serde(rename = "numOutputs", default)]
        pub num_outputs: ::std::option::Option<i32>,
        #[doc = "Zero or more side inputs."]
        #[serde(rename = "sideInputs", default)]
        pub side_inputs: ::std::option::Option<Vec<crate::schemas::SideInputInfo>>,
        #[doc = "The user function to invoke."]
        #[serde(rename = "userFn", default)]
        pub user_fn:
            ::std::option::Option<::std::collections::BTreeMap<String, ::serde_json::Value>>,
    }
    impl ::field_selector::FieldSelector for ParDoInstruction {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct ParallelInstruction {
        #[doc = "Additional information for Flatten instructions."]
        #[serde(rename = "flatten", default)]
        pub flatten: ::std::option::Option<crate::schemas::FlattenInstruction>,
        #[doc = "User-provided name of this operation."]
        #[serde(rename = "name", default)]
        pub name: ::std::option::Option<String>,
        #[doc = "System-defined name for the operation in the original workflow graph."]
        #[serde(rename = "originalName", default)]
        pub original_name: ::std::option::Option<String>,
        #[doc = "Describes the outputs of the instruction."]
        #[serde(rename = "outputs", default)]
        pub outputs: ::std::option::Option<Vec<crate::schemas::InstructionOutput>>,
        #[doc = "Additional information for ParDo instructions."]
        #[serde(rename = "parDo", default)]
        pub par_do: ::std::option::Option<crate::schemas::ParDoInstruction>,
        #[doc = "Additional information for PartialGroupByKey instructions."]
        #[serde(rename = "partialGroupByKey", default)]
        pub partial_group_by_key:
            ::std::option::Option<crate::schemas::PartialGroupByKeyInstruction>,
        #[doc = "Additional information for Read instructions."]
        #[serde(rename = "read", default)]
        pub read: ::std::option::Option<crate::schemas::ReadInstruction>,
        #[doc = "System-defined name of this operation.\nUnique across the workflow."]
        #[serde(rename = "systemName", default)]
        pub system_name: ::std::option::Option<String>,
        #[doc = "Additional information for Write instructions."]
        #[serde(rename = "write", default)]
        pub write: ::std::option::Option<crate::schemas::WriteInstruction>,
    }
    impl ::field_selector::FieldSelector for ParallelInstruction {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct Parameter {
        #[doc = "Key or name for this parameter."]
        #[serde(rename = "key", default)]
        pub key: ::std::option::Option<String>,
        #[doc = "Value for this parameter."]
        #[serde(rename = "value", default)]
        pub value: ::std::option::Option<::serde_json::Value>,
    }
    impl ::field_selector::FieldSelector for Parameter {
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
    pub struct ParameterMetadata {
        #[doc = "Required. The help text to display for the parameter."]
        #[serde(rename = "helpText", default)]
        pub help_text: ::std::option::Option<String>,
        #[doc = "Optional. Whether the parameter is optional. Defaults to false."]
        #[serde(rename = "isOptional", default)]
        pub is_optional: ::std::option::Option<bool>,
        #[doc = "Required. The label to display for the parameter."]
        #[serde(rename = "label", default)]
        pub label: ::std::option::Option<String>,
        #[doc = "Required. The name of the parameter."]
        #[serde(rename = "name", default)]
        pub name: ::std::option::Option<String>,
        #[doc = "Optional. Regexes that the parameter must match."]
        #[serde(rename = "regexes", default)]
        pub regexes: ::std::option::Option<Vec<String>>,
    }
    impl ::field_selector::FieldSelector for ParameterMetadata {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct PartialGroupByKeyInstruction {
        #[doc = "Describes the input to the partial group-by-key instruction."]
        #[serde(rename = "input", default)]
        pub input: ::std::option::Option<crate::schemas::InstructionInput>,
        #[doc = "The codec to use for interpreting an element in the input PTable."]
        #[serde(rename = "inputElementCodec", default)]
        pub input_element_codec:
            ::std::option::Option<::std::collections::BTreeMap<String, ::serde_json::Value>>,
        #[doc = "If this instruction includes a combining function this is the name of the\nintermediate store between the GBK and the CombineValues."]
        #[serde(rename = "originalCombineValuesInputStoreName", default)]
        pub original_combine_values_input_store_name: ::std::option::Option<String>,
        #[doc = "If this instruction includes a combining function, this is the name of the\nCombineValues instruction lifted into this instruction."]
        #[serde(rename = "originalCombineValuesStepName", default)]
        pub original_combine_values_step_name: ::std::option::Option<String>,
        #[doc = "Zero or more side inputs."]
        #[serde(rename = "sideInputs", default)]
        pub side_inputs: ::std::option::Option<Vec<crate::schemas::SideInputInfo>>,
        #[doc = "The value combining function to invoke."]
        #[serde(rename = "valueCombiningFn", default)]
        pub value_combining_fn:
            ::std::option::Option<::std::collections::BTreeMap<String, ::serde_json::Value>>,
    }
    impl ::field_selector::FieldSelector for PartialGroupByKeyInstruction {
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
    pub struct PipelineDescription {
        #[doc = "Pipeline level display data."]
        #[serde(rename = "displayData", default)]
        pub display_data: ::std::option::Option<Vec<crate::schemas::DisplayData>>,
        #[doc = "Description of each stage of execution of the pipeline."]
        #[serde(rename = "executionPipelineStage", default)]
        pub execution_pipeline_stage:
            ::std::option::Option<Vec<crate::schemas::ExecutionStageSummary>>,
        #[doc = "Description of each transform in the pipeline and collections between them."]
        #[serde(rename = "originalPipelineTransform", default)]
        pub original_pipeline_transform:
            ::std::option::Option<Vec<crate::schemas::TransformSummary>>,
    }
    impl ::field_selector::FieldSelector for PipelineDescription {
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
    pub struct Position {
        #[doc = "Position is a byte offset."]
        #[serde(rename = "byteOffset", default)]
        #[serde(with = "crate::parsed_string")]
        pub byte_offset: ::std::option::Option<i64>,
        #[doc = "CloudPosition is a concat position."]
        #[serde(rename = "concatPosition", default)]
        pub concat_position: ::std::option::Option<Box<crate::schemas::ConcatPosition>>,
        #[doc = "Position is past all other positions. Also useful for the end\nposition of an unbounded range."]
        #[serde(rename = "end", default)]
        pub end: ::std::option::Option<bool>,
        #[doc = "Position is a string key, ordered lexicographically."]
        #[serde(rename = "key", default)]
        pub key: ::std::option::Option<String>,
        #[doc = "Position is a record index."]
        #[serde(rename = "recordIndex", default)]
        #[serde(with = "crate::parsed_string")]
        pub record_index: ::std::option::Option<i64>,
        #[doc = "CloudPosition is a base64 encoded BatchShufflePosition (with FIXED\nsharding)."]
        #[serde(rename = "shufflePosition", default)]
        pub shuffle_position: ::std::option::Option<String>,
    }
    impl ::field_selector::FieldSelector for Position {
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
    pub struct PubSubIODetails {
        #[doc = "Subscription used in the connection."]
        #[serde(rename = "subscription", default)]
        pub subscription: ::std::option::Option<String>,
        #[doc = "Topic accessed in the connection."]
        #[serde(rename = "topic", default)]
        pub topic: ::std::option::Option<String>,
    }
    impl ::field_selector::FieldSelector for PubSubIODetails {
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
    pub struct PubsubLocation {
        #[doc = "Indicates whether the pipeline allows late-arriving data."]
        #[serde(rename = "dropLateData", default)]
        pub drop_late_data: ::std::option::Option<bool>,
        #[doc = "If set, contains a pubsub label from which to extract record ids.\nIf left empty, record deduplication will be strictly best effort."]
        #[serde(rename = "idLabel", default)]
        pub id_label: ::std::option::Option<String>,
        #[doc = "A pubsub subscription, in the form of\n\"pubsub.googleapis.com/subscriptions/<project-id>/<subscription-name>\""]
        #[serde(rename = "subscription", default)]
        pub subscription: ::std::option::Option<String>,
        #[doc = "If set, contains a pubsub label from which to extract record timestamps.\nIf left empty, record timestamps will be generated upon arrival."]
        #[serde(rename = "timestampLabel", default)]
        pub timestamp_label: ::std::option::Option<String>,
        #[doc = "A pubsub topic, in the form of\n\"pubsub.googleapis.com/topics/<project-id>/<topic-name>\""]
        #[serde(rename = "topic", default)]
        pub topic: ::std::option::Option<String>,
        #[doc = "If set, specifies the pubsub subscription that will be used for tracking\ncustom time timestamps for watermark estimation."]
        #[serde(rename = "trackingSubscription", default)]
        pub tracking_subscription: ::std::option::Option<String>,
        #[doc = "If true, then the client has requested to get pubsub attributes."]
        #[serde(rename = "withAttributes", default)]
        pub with_attributes: ::std::option::Option<bool>,
    }
    impl ::field_selector::FieldSelector for PubsubLocation {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct ReadInstruction {
        #[doc = "The source to read from."]
        #[serde(rename = "source", default)]
        pub source: ::std::option::Option<crate::schemas::Source>,
    }
    impl ::field_selector::FieldSelector for ReadInstruction {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct ReportWorkItemStatusRequest {
        #[doc = "The current timestamp at the worker."]
        #[serde(rename = "currentWorkerTime", default)]
        pub current_worker_time: ::std::option::Option<String>,
        #[doc = "The [regional endpoint]\n(https://cloud.google.com/dataflow/docs/concepts/regional-endpoints) that\ncontains the WorkItem's job."]
        #[serde(rename = "location", default)]
        pub location: ::std::option::Option<String>,
        #[doc = "Untranslated bag-of-bytes WorkProgressUpdateRequest from UnifiedWorker."]
        #[serde(rename = "unifiedWorkerRequest", default)]
        pub unified_worker_request:
            ::std::option::Option<::std::collections::BTreeMap<String, ::serde_json::Value>>,
        #[doc = "The order is unimportant, except that the order of the\nWorkItemServiceState messages in the ReportWorkItemStatusResponse\ncorresponds to the order of WorkItemStatus messages here."]
        #[serde(rename = "workItemStatuses", default)]
        pub work_item_statuses: ::std::option::Option<Vec<crate::schemas::WorkItemStatus>>,
        #[doc = "The ID of the worker reporting the WorkItem status.  If this\ndoes not match the ID of the worker which the Dataflow service\nbelieves currently has the lease on the WorkItem, the report\nwill be dropped (with an error response)."]
        #[serde(rename = "workerId", default)]
        pub worker_id: ::std::option::Option<String>,
    }
    impl ::field_selector::FieldSelector for ReportWorkItemStatusRequest {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct ReportWorkItemStatusResponse {
        #[doc = "Untranslated bag-of-bytes WorkProgressUpdateResponse for UnifiedWorker."]
        #[serde(rename = "unifiedWorkerResponse", default)]
        pub unified_worker_response:
            ::std::option::Option<::std::collections::BTreeMap<String, ::serde_json::Value>>,
        #[doc = "A set of messages indicating the service-side state for each\nWorkItem whose status was reported, in the same order as the\nWorkItemStatus messages in the ReportWorkItemStatusRequest which\nresulting in this response."]
        #[serde(rename = "workItemServiceStates", default)]
        pub work_item_service_states:
            ::std::option::Option<Vec<crate::schemas::WorkItemServiceState>>,
    }
    impl ::field_selector::FieldSelector for ReportWorkItemStatusResponse {
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
    pub struct ReportedParallelism {
        #[doc = "Specifies whether the parallelism is infinite. If true, \"value\" is\nignored.\nInfinite parallelism means the service will assume that the work item\ncan always be split into more non-empty work items by dynamic splitting.\nThis is a work-around for lack of support for infinity by the current\nJSON-based Java RPC stack."]
        #[serde(rename = "isInfinite", default)]
        pub is_infinite: ::std::option::Option<bool>,
        #[doc = "Specifies the level of parallelism in case it is finite."]
        #[serde(rename = "value", default)]
        pub value: ::std::option::Option<f64>,
    }
    impl ::field_selector::FieldSelector for ReportedParallelism {
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
    pub struct ResourceUtilizationReport {
        #[doc = "CPU utilization samples."]
        #[serde(rename = "cpuTime", default)]
        pub cpu_time: ::std::option::Option<Vec<crate::schemas::Cputime>>,
    }
    impl ::field_selector::FieldSelector for ResourceUtilizationReport {
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
        Copy,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct ResourceUtilizationReportResponse;
    impl ::field_selector::FieldSelector for ResourceUtilizationReportResponse {
        fn field_selector_with_ident(_ident: &str, _selector: &mut String) {}
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
    pub struct RuntimeEnvironment {
        #[doc = "Additional experiment flags for the job."]
        #[serde(rename = "additionalExperiments", default)]
        pub additional_experiments: ::std::option::Option<Vec<String>>,
        #[doc = "Additional user labels to be specified for the job.\nKeys and values should follow the restrictions specified in the [labeling\nrestrictions](https://cloud.google.com/compute/docs/labeling-resources#restrictions)\npage."]
        #[serde(rename = "additionalUserLabels", default)]
        pub additional_user_labels:
            ::std::option::Option<::std::collections::BTreeMap<String, String>>,
        #[doc = "Whether to bypass the safety checks for the job's temporary directory.\nUse with caution."]
        #[serde(rename = "bypassTempDirValidation", default)]
        pub bypass_temp_dir_validation: ::std::option::Option<bool>,
        #[doc = "Optional. Name for the Cloud KMS key for the job.\nKey format is:\nprojects/<project>/locations/<location>/keyRings/<keyring>/cryptoKeys/<key>"]
        #[serde(rename = "kmsKeyName", default)]
        pub kms_key_name: ::std::option::Option<String>,
        #[doc = "The machine type to use for the job. Defaults to the value from the\ntemplate if not specified."]
        #[serde(rename = "machineType", default)]
        pub machine_type: ::std::option::Option<String>,
        #[doc = "The maximum number of Google Compute Engine instances to be made\navailable to your pipeline during execution, from 1 to 1000."]
        #[serde(rename = "maxWorkers", default)]
        pub max_workers: ::std::option::Option<i32>,
        #[doc = "Network to which VMs will be assigned.  If empty or unspecified,\nthe service will use the network \"default\"."]
        #[serde(rename = "network", default)]
        pub network: ::std::option::Option<String>,
        #[doc = "The initial number of Google Compute Engine instnaces for the job."]
        #[serde(rename = "numWorkers", default)]
        pub num_workers: ::std::option::Option<i32>,
        #[doc = "The email address of the service account to run the job as."]
        #[serde(rename = "serviceAccountEmail", default)]
        pub service_account_email: ::std::option::Option<String>,
        #[doc = "Subnetwork to which VMs will be assigned, if desired.  Expected to be of\nthe form \"regions/REGION/subnetworks/SUBNETWORK\"."]
        #[serde(rename = "subnetwork", default)]
        pub subnetwork: ::std::option::Option<String>,
        #[doc = "The Cloud Storage path to use for temporary files.\nMust be a valid Cloud Storage URL, beginning with `gs://`."]
        #[serde(rename = "tempLocation", default)]
        pub temp_location: ::std::option::Option<String>,
        #[doc = "The Compute Engine [availability\nzone](https://cloud.google.com/compute/docs/regions-zones/regions-zones)\nfor launching worker instances to run your pipeline."]
        #[serde(rename = "zone", default)]
        pub zone: ::std::option::Option<String>,
    }
    impl ::field_selector::FieldSelector for RuntimeEnvironment {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum SdkVersionSdkSupportStatus {
        #[doc = "This version of the SDK is deprecated and will eventually be no\nlonger supported."]
        Deprecated,
        #[doc = "A newer version of the SDK family exists, and an update is recommended."]
        Stale,
        #[doc = "This is a known version of an SDK, and is supported."]
        Supported,
        #[doc = "Cloud Dataflow is unaware of this version."]
        Unknown,
        #[doc = "Support for this SDK version has ended and it should no longer be used."]
        Unsupported,
    }
    impl SdkVersionSdkSupportStatus {
        pub fn as_str(self) -> &'static str {
            match self {
                SdkVersionSdkSupportStatus::Deprecated => "DEPRECATED",
                SdkVersionSdkSupportStatus::Stale => "STALE",
                SdkVersionSdkSupportStatus::Supported => "SUPPORTED",
                SdkVersionSdkSupportStatus::Unknown => "UNKNOWN",
                SdkVersionSdkSupportStatus::Unsupported => "UNSUPPORTED",
            }
        }
    }
    impl ::std::fmt::Display for SdkVersionSdkSupportStatus {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for SdkVersionSdkSupportStatus {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for SdkVersionSdkSupportStatus {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "DEPRECATED" => SdkVersionSdkSupportStatus::Deprecated,
                "STALE" => SdkVersionSdkSupportStatus::Stale,
                "SUPPORTED" => SdkVersionSdkSupportStatus::Supported,
                "UNKNOWN" => SdkVersionSdkSupportStatus::Unknown,
                "UNSUPPORTED" => SdkVersionSdkSupportStatus::Unsupported,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::field_selector::FieldSelector for SdkVersionSdkSupportStatus {
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
    pub struct SdkVersion {
        #[doc = "The support status for this SDK version."]
        #[serde(rename = "sdkSupportStatus", default)]
        pub sdk_support_status: ::std::option::Option<crate::schemas::SdkVersionSdkSupportStatus>,
        #[doc = "The version of the SDK used to run the job."]
        #[serde(rename = "version", default)]
        pub version: ::std::option::Option<String>,
        #[doc = "A readable string describing the version of the SDK."]
        #[serde(rename = "versionDisplayName", default)]
        pub version_display_name: ::std::option::Option<String>,
    }
    impl ::field_selector::FieldSelector for SdkVersion {
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
    pub struct SendDebugCaptureRequest {
        #[doc = "The internal component id for which debug information is sent."]
        #[serde(rename = "componentId", default)]
        pub component_id: ::std::option::Option<String>,
        #[doc = "The encoded debug information."]
        #[serde(rename = "data", default)]
        pub data: ::std::option::Option<String>,
        #[doc = "The [regional endpoint]\n(https://cloud.google.com/dataflow/docs/concepts/regional-endpoints) that\ncontains the job specified by job_id."]
        #[serde(rename = "location", default)]
        pub location: ::std::option::Option<String>,
        #[doc = "The worker id, i.e., VM hostname."]
        #[serde(rename = "workerId", default)]
        pub worker_id: ::std::option::Option<String>,
    }
    impl ::field_selector::FieldSelector for SendDebugCaptureRequest {
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
        Copy,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct SendDebugCaptureResponse;
    impl ::field_selector::FieldSelector for SendDebugCaptureResponse {
        fn field_selector_with_ident(_ident: &str, _selector: &mut String) {}
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct SendWorkerMessagesRequest {
        #[doc = "The [regional endpoint]\n(https://cloud.google.com/dataflow/docs/concepts/regional-endpoints) that\ncontains the job."]
        #[serde(rename = "location", default)]
        pub location: ::std::option::Option<String>,
        #[doc = "The WorkerMessages to send."]
        #[serde(rename = "workerMessages", default)]
        pub worker_messages: ::std::option::Option<Vec<crate::schemas::WorkerMessage>>,
    }
    impl ::field_selector::FieldSelector for SendWorkerMessagesRequest {
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
    pub struct SendWorkerMessagesResponse {
        #[doc = "The servers response to the worker messages."]
        #[serde(rename = "workerMessageResponses", default)]
        pub worker_message_responses:
            ::std::option::Option<Vec<crate::schemas::WorkerMessageResponse>>,
    }
    impl ::field_selector::FieldSelector for SendWorkerMessagesResponse {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct SeqMapTask {
        #[doc = "Information about each of the inputs."]
        #[serde(rename = "inputs", default)]
        pub inputs: ::std::option::Option<Vec<crate::schemas::SideInputInfo>>,
        #[doc = "The user-provided name of the SeqDo operation."]
        #[serde(rename = "name", default)]
        pub name: ::std::option::Option<String>,
        #[doc = "Information about each of the outputs."]
        #[serde(rename = "outputInfos", default)]
        pub output_infos: ::std::option::Option<Vec<crate::schemas::SeqMapTaskOutputInfo>>,
        #[doc = "System-defined name of the stage containing the SeqDo operation.\nUnique across the workflow."]
        #[serde(rename = "stageName", default)]
        pub stage_name: ::std::option::Option<String>,
        #[doc = "System-defined name of the SeqDo operation.\nUnique across the workflow."]
        #[serde(rename = "systemName", default)]
        pub system_name: ::std::option::Option<String>,
        #[doc = "The user function to invoke."]
        #[serde(rename = "userFn", default)]
        pub user_fn:
            ::std::option::Option<::std::collections::BTreeMap<String, ::serde_json::Value>>,
    }
    impl ::field_selector::FieldSelector for SeqMapTask {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct SeqMapTaskOutputInfo {
        #[doc = "The sink to write the output value to."]
        #[serde(rename = "sink", default)]
        pub sink: ::std::option::Option<crate::schemas::Sink>,
        #[doc = "The id of the TupleTag the user code will tag the output value by."]
        #[serde(rename = "tag", default)]
        pub tag: ::std::option::Option<String>,
    }
    impl ::field_selector::FieldSelector for SeqMapTaskOutputInfo {
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
    pub struct ShellTask {
        #[doc = "The shell command to run."]
        #[serde(rename = "command", default)]
        pub command: ::std::option::Option<String>,
        #[doc = "Exit code for the task."]
        #[serde(rename = "exitCode", default)]
        pub exit_code: ::std::option::Option<i32>,
    }
    impl ::field_selector::FieldSelector for ShellTask {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct SideInputInfo {
        #[doc = "How to interpret the source element(s) as a side input value."]
        #[serde(rename = "kind", default)]
        pub kind: ::std::option::Option<::std::collections::BTreeMap<String, ::serde_json::Value>>,
        #[doc = "The source(s) to read element(s) from to get the value of this side input.\nIf more than one source, then the elements are taken from the\nsources, in the specified order if order matters.\nAt least one source is required."]
        #[serde(rename = "sources", default)]
        pub sources: ::std::option::Option<Vec<crate::schemas::Source>>,
        #[doc = "The id of the tag the user code will access this side input by;\nthis should correspond to the tag of some MultiOutputInfo."]
        #[serde(rename = "tag", default)]
        pub tag: ::std::option::Option<String>,
    }
    impl ::field_selector::FieldSelector for SideInputInfo {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct Sink {
        #[doc = "The codec to use to encode data written to the sink."]
        #[serde(rename = "codec", default)]
        pub codec: ::std::option::Option<::std::collections::BTreeMap<String, ::serde_json::Value>>,
        #[doc = "The sink to write to, plus its parameters."]
        #[serde(rename = "spec", default)]
        pub spec: ::std::option::Option<::std::collections::BTreeMap<String, ::serde_json::Value>>,
    }
    impl ::field_selector::FieldSelector for Sink {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum SnapshotState {
        #[doc = "Snapshot has been deleted."]
        Deleted,
        #[doc = "Snapshot failed to be created."]
        Failed,
        #[doc = "Snapshot intent to create has been persisted, snapshotting of state has not\nyet started."]
        Pending,
        #[doc = "Snapshot has been created and is ready to be used."]
        Ready,
        #[doc = "Snapshotting is being performed."]
        Running,
        #[doc = "Unknown state."]
        UnknownSnapshotState,
    }
    impl SnapshotState {
        pub fn as_str(self) -> &'static str {
            match self {
                SnapshotState::Deleted => "DELETED",
                SnapshotState::Failed => "FAILED",
                SnapshotState::Pending => "PENDING",
                SnapshotState::Ready => "READY",
                SnapshotState::Running => "RUNNING",
                SnapshotState::UnknownSnapshotState => "UNKNOWN_SNAPSHOT_STATE",
            }
        }
    }
    impl ::std::fmt::Display for SnapshotState {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for SnapshotState {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for SnapshotState {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "DELETED" => SnapshotState::Deleted,
                "FAILED" => SnapshotState::Failed,
                "PENDING" => SnapshotState::Pending,
                "READY" => SnapshotState::Ready,
                "RUNNING" => SnapshotState::Running,
                "UNKNOWN_SNAPSHOT_STATE" => SnapshotState::UnknownSnapshotState,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::field_selector::FieldSelector for SnapshotState {
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
    pub struct Snapshot {
        #[doc = "The time this snapshot was created."]
        #[serde(rename = "creationTime", default)]
        pub creation_time: ::std::option::Option<String>,
        #[doc = "The unique ID of this snapshot."]
        #[serde(rename = "id", default)]
        pub id: ::std::option::Option<String>,
        #[doc = "The project this snapshot belongs to."]
        #[serde(rename = "projectId", default)]
        pub project_id: ::std::option::Option<String>,
        #[doc = "The job this snapshot was created from."]
        #[serde(rename = "sourceJobId", default)]
        pub source_job_id: ::std::option::Option<String>,
        #[doc = "State of the snapshot."]
        #[serde(rename = "state", default)]
        pub state: ::std::option::Option<crate::schemas::SnapshotState>,
        #[doc = "The time after which this snapshot will be automatically deleted."]
        #[serde(rename = "ttl", default)]
        pub ttl: ::std::option::Option<String>,
    }
    impl ::field_selector::FieldSelector for Snapshot {
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
    pub struct SnapshotJobRequest {
        #[doc = "The location that contains this job."]
        #[serde(rename = "location", default)]
        pub location: ::std::option::Option<String>,
        #[doc = "TTL for the snapshot."]
        #[serde(rename = "ttl", default)]
        pub ttl: ::std::option::Option<String>,
    }
    impl ::field_selector::FieldSelector for SnapshotJobRequest {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct Source {
        #[doc = "While splitting, sources may specify the produced bundles\nas differences against another source, in order to save backend-side\nmemory and allow bigger jobs. For details, see SourceSplitRequest.\nTo support this use case, the full set of parameters of the source\nis logically obtained by taking the latest explicitly specified value\nof each parameter in the order:\nbase_specs (later items win), spec (overrides anything in base_specs)."]
        #[serde(rename = "baseSpecs", default)]
        pub base_specs:
            ::std::option::Option<Vec<::std::collections::BTreeMap<String, ::serde_json::Value>>>,
        #[doc = "The codec to use to decode data read from the source."]
        #[serde(rename = "codec", default)]
        pub codec: ::std::option::Option<::std::collections::BTreeMap<String, ::serde_json::Value>>,
        #[doc = "Setting this value to true hints to the framework that the source\ndoesn't need splitting, and using SourceSplitRequest on it would\nyield SOURCE_SPLIT_OUTCOME_USE_CURRENT.\n\nE.g. a file splitter may set this to true when splitting a single file\ninto a set of byte ranges of appropriate size, and set this\nto false when splitting a filepattern into individual files.\nHowever, for efficiency, a file splitter may decide to produce\nfile subranges directly from the filepattern to avoid a splitting\nround-trip.\n\nSee SourceSplitRequest for an overview of the splitting process.\n\nThis field is meaningful only in the Source objects populated\nby the user (e.g. when filling in a DerivedSource).\nSource objects supplied by the framework to the user don't have\nthis field populated."]
        #[serde(rename = "doesNotNeedSplitting", default)]
        pub does_not_need_splitting: ::std::option::Option<bool>,
        #[doc = "Optionally, metadata for this source can be supplied right away,\navoiding a SourceGetMetadataOperation roundtrip\n(see SourceOperationRequest).\n\nThis field is meaningful only in the Source objects populated\nby the user (e.g. when filling in a DerivedSource).\nSource objects supplied by the framework to the user don't have\nthis field populated."]
        #[serde(rename = "metadata", default)]
        pub metadata: ::std::option::Option<crate::schemas::SourceMetadata>,
        #[doc = "The source to read from, plus its parameters."]
        #[serde(rename = "spec", default)]
        pub spec: ::std::option::Option<::std::collections::BTreeMap<String, ::serde_json::Value>>,
    }
    impl ::field_selector::FieldSelector for Source {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct SourceFork {
        #[doc = "DEPRECATED"]
        #[serde(rename = "primary", default)]
        pub primary: ::std::option::Option<crate::schemas::SourceSplitShard>,
        #[doc = "DEPRECATED"]
        #[serde(rename = "primarySource", default)]
        pub primary_source: ::std::option::Option<crate::schemas::DerivedSource>,
        #[doc = "DEPRECATED"]
        #[serde(rename = "residual", default)]
        pub residual: ::std::option::Option<crate::schemas::SourceSplitShard>,
        #[doc = "DEPRECATED"]
        #[serde(rename = "residualSource", default)]
        pub residual_source: ::std::option::Option<crate::schemas::DerivedSource>,
    }
    impl ::field_selector::FieldSelector for SourceFork {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct SourceGetMetadataRequest {
        #[doc = "Specification of the source whose metadata should be computed."]
        #[serde(rename = "source", default)]
        pub source: ::std::option::Option<crate::schemas::Source>,
    }
    impl ::field_selector::FieldSelector for SourceGetMetadataRequest {
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
    pub struct SourceGetMetadataResponse {
        #[doc = "The computed metadata."]
        #[serde(rename = "metadata", default)]
        pub metadata: ::std::option::Option<crate::schemas::SourceMetadata>,
    }
    impl ::field_selector::FieldSelector for SourceGetMetadataResponse {
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
    pub struct SourceMetadata {
        #[doc = "An estimate of the total size (in bytes) of the data that would be\nread from this source.  This estimate is in terms of external storage\nsize, before any decompression or other processing done by the reader."]
        #[serde(rename = "estimatedSizeBytes", default)]
        #[serde(with = "crate::parsed_string")]
        pub estimated_size_bytes: ::std::option::Option<i64>,
        #[doc = "Specifies that the size of this source is known to be infinite\n(this is a streaming source)."]
        #[serde(rename = "infinite", default)]
        pub infinite: ::std::option::Option<bool>,
        #[doc = "Whether this source is known to produce key/value pairs with\nthe (encoded) keys in lexicographically sorted order."]
        #[serde(rename = "producesSortedKeys", default)]
        pub produces_sorted_keys: ::std::option::Option<bool>,
    }
    impl ::field_selector::FieldSelector for SourceMetadata {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct SourceOperationRequest {
        #[doc = "Information about a request to get metadata about a source."]
        #[serde(rename = "getMetadata", default)]
        pub get_metadata: ::std::option::Option<crate::schemas::SourceGetMetadataRequest>,
        #[doc = "User-provided name of the Read instruction for this source."]
        #[serde(rename = "name", default)]
        pub name: ::std::option::Option<String>,
        #[doc = "System-defined name for the Read instruction for this source\nin the original workflow graph."]
        #[serde(rename = "originalName", default)]
        pub original_name: ::std::option::Option<String>,
        #[doc = "Information about a request to split a source."]
        #[serde(rename = "split", default)]
        pub split: ::std::option::Option<crate::schemas::SourceSplitRequest>,
        #[doc = "System-defined name of the stage containing the source operation.\nUnique across the workflow."]
        #[serde(rename = "stageName", default)]
        pub stage_name: ::std::option::Option<String>,
        #[doc = "System-defined name of the Read instruction for this source.\nUnique across the workflow."]
        #[serde(rename = "systemName", default)]
        pub system_name: ::std::option::Option<String>,
    }
    impl ::field_selector::FieldSelector for SourceOperationRequest {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct SourceOperationResponse {
        #[doc = "A response to a request to get metadata about a source."]
        #[serde(rename = "getMetadata", default)]
        pub get_metadata: ::std::option::Option<crate::schemas::SourceGetMetadataResponse>,
        #[doc = "A response to a request to split a source."]
        #[serde(rename = "split", default)]
        pub split: ::std::option::Option<crate::schemas::SourceSplitResponse>,
    }
    impl ::field_selector::FieldSelector for SourceOperationResponse {
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
    pub struct SourceSplitOptions {
        #[doc = "The source should be split into a set of bundles where the estimated size\nof each is approximately this many bytes."]
        #[serde(rename = "desiredBundleSizeBytes", default)]
        #[serde(with = "crate::parsed_string")]
        pub desired_bundle_size_bytes: ::std::option::Option<i64>,
        #[doc = "DEPRECATED in favor of desired_bundle_size_bytes."]
        #[serde(rename = "desiredShardSizeBytes", default)]
        #[serde(with = "crate::parsed_string")]
        pub desired_shard_size_bytes: ::std::option::Option<i64>,
    }
    impl ::field_selector::FieldSelector for SourceSplitOptions {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct SourceSplitRequest {
        #[doc = "Hints for tuning the splitting process."]
        #[serde(rename = "options", default)]
        pub options: ::std::option::Option<crate::schemas::SourceSplitOptions>,
        #[doc = "Specification of the source to be split."]
        #[serde(rename = "source", default)]
        pub source: ::std::option::Option<crate::schemas::Source>,
    }
    impl ::field_selector::FieldSelector for SourceSplitRequest {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum SourceSplitResponseOutcome {
        #[doc = "Splitting produced a list of bundles."]
        SourceSplitOutcomeSplittingHappened,
        #[doc = "The source split outcome is unknown, or unspecified."]
        SourceSplitOutcomeUnknown,
        #[doc = "The current source should be processed \"as is\" without splitting."]
        SourceSplitOutcomeUseCurrent,
    }
    impl SourceSplitResponseOutcome {
        pub fn as_str(self) -> &'static str {
            match self {
                SourceSplitResponseOutcome::SourceSplitOutcomeSplittingHappened => {
                    "SOURCE_SPLIT_OUTCOME_SPLITTING_HAPPENED"
                }
                SourceSplitResponseOutcome::SourceSplitOutcomeUnknown => {
                    "SOURCE_SPLIT_OUTCOME_UNKNOWN"
                }
                SourceSplitResponseOutcome::SourceSplitOutcomeUseCurrent => {
                    "SOURCE_SPLIT_OUTCOME_USE_CURRENT"
                }
            }
        }
    }
    impl ::std::fmt::Display for SourceSplitResponseOutcome {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for SourceSplitResponseOutcome {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for SourceSplitResponseOutcome {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "SOURCE_SPLIT_OUTCOME_SPLITTING_HAPPENED" => {
                    SourceSplitResponseOutcome::SourceSplitOutcomeSplittingHappened
                }
                "SOURCE_SPLIT_OUTCOME_UNKNOWN" => {
                    SourceSplitResponseOutcome::SourceSplitOutcomeUnknown
                }
                "SOURCE_SPLIT_OUTCOME_USE_CURRENT" => {
                    SourceSplitResponseOutcome::SourceSplitOutcomeUseCurrent
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
    impl ::field_selector::FieldSelector for SourceSplitResponseOutcome {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct SourceSplitResponse {
        #[doc = "If outcome is SPLITTING_HAPPENED, then this is a list of bundles\ninto which the source was split. Otherwise this field is ignored.\nThis list can be empty, which means the source represents an empty input."]
        #[serde(rename = "bundles", default)]
        pub bundles: ::std::option::Option<Vec<crate::schemas::DerivedSource>>,
        #[doc = "Indicates whether splitting happened and produced a list of bundles.\nIf this is USE_CURRENT_SOURCE_AS_IS, the current source should\nbe processed \"as is\" without splitting. \"bundles\" is ignored in this case.\nIf this is SPLITTING_HAPPENED, then \"bundles\" contains a list of\nbundles into which the source was split."]
        #[serde(rename = "outcome", default)]
        pub outcome: ::std::option::Option<crate::schemas::SourceSplitResponseOutcome>,
        #[doc = "DEPRECATED in favor of bundles."]
        #[serde(rename = "shards", default)]
        pub shards: ::std::option::Option<Vec<crate::schemas::SourceSplitShard>>,
    }
    impl ::field_selector::FieldSelector for SourceSplitResponse {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum SourceSplitShardDerivationMode {
        #[doc = "Produce a Source based on the Source being split."]
        SourceDerivationModeChildOfCurrent,
        #[doc = "Produce a completely independent Source with no base."]
        SourceDerivationModeIndependent,
        #[doc = "Produce a Source based on the base of the Source being split."]
        SourceDerivationModeSiblingOfCurrent,
        #[doc = "The source derivation is unknown, or unspecified."]
        SourceDerivationModeUnknown,
    }
    impl SourceSplitShardDerivationMode {
        pub fn as_str(self) -> &'static str {
            match self {
                SourceSplitShardDerivationMode::SourceDerivationModeChildOfCurrent => {
                    "SOURCE_DERIVATION_MODE_CHILD_OF_CURRENT"
                }
                SourceSplitShardDerivationMode::SourceDerivationModeIndependent => {
                    "SOURCE_DERIVATION_MODE_INDEPENDENT"
                }
                SourceSplitShardDerivationMode::SourceDerivationModeSiblingOfCurrent => {
                    "SOURCE_DERIVATION_MODE_SIBLING_OF_CURRENT"
                }
                SourceSplitShardDerivationMode::SourceDerivationModeUnknown => {
                    "SOURCE_DERIVATION_MODE_UNKNOWN"
                }
            }
        }
    }
    impl ::std::fmt::Display for SourceSplitShardDerivationMode {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for SourceSplitShardDerivationMode {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for SourceSplitShardDerivationMode {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "SOURCE_DERIVATION_MODE_CHILD_OF_CURRENT" => {
                    SourceSplitShardDerivationMode::SourceDerivationModeChildOfCurrent
                }
                "SOURCE_DERIVATION_MODE_INDEPENDENT" => {
                    SourceSplitShardDerivationMode::SourceDerivationModeIndependent
                }
                "SOURCE_DERIVATION_MODE_SIBLING_OF_CURRENT" => {
                    SourceSplitShardDerivationMode::SourceDerivationModeSiblingOfCurrent
                }
                "SOURCE_DERIVATION_MODE_UNKNOWN" => {
                    SourceSplitShardDerivationMode::SourceDerivationModeUnknown
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
    impl ::field_selector::FieldSelector for SourceSplitShardDerivationMode {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct SourceSplitShard {
        #[doc = "DEPRECATED"]
        #[serde(rename = "derivationMode", default)]
        pub derivation_mode: ::std::option::Option<crate::schemas::SourceSplitShardDerivationMode>,
        #[doc = "DEPRECATED"]
        #[serde(rename = "source", default)]
        pub source: ::std::option::Option<crate::schemas::Source>,
    }
    impl ::field_selector::FieldSelector for SourceSplitShard {
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
    pub struct SpannerIODetails {
        #[doc = "DatabaseId accessed in the connection."]
        #[serde(rename = "databaseId", default)]
        pub database_id: ::std::option::Option<String>,
        #[doc = "InstanceId accessed in the connection."]
        #[serde(rename = "instanceId", default)]
        pub instance_id: ::std::option::Option<String>,
        #[doc = "ProjectId accessed in the connection."]
        #[serde(rename = "projectId", default)]
        pub project_id: ::std::option::Option<String>,
    }
    impl ::field_selector::FieldSelector for SpannerIODetails {
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
    pub struct SplitInt64 {
        #[doc = "The high order bits, including the sign: n >> 32."]
        #[serde(rename = "highBits", default)]
        pub high_bits: ::std::option::Option<i32>,
        #[doc = "The low order bits: n & 0xffffffff."]
        #[serde(rename = "lowBits", default)]
        pub low_bits: ::std::option::Option<u32>,
    }
    impl ::field_selector::FieldSelector for SplitInt64 {
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
    pub struct StageSource {
        #[doc = "Dataflow service generated name for this source."]
        #[serde(rename = "name", default)]
        pub name: ::std::option::Option<String>,
        #[doc = "User name for the original user transform or collection with which this\nsource is most closely associated."]
        #[serde(rename = "originalTransformOrCollection", default)]
        pub original_transform_or_collection: ::std::option::Option<String>,
        #[doc = "Size of the source, if measurable."]
        #[serde(rename = "sizeBytes", default)]
        #[serde(with = "crate::parsed_string")]
        pub size_bytes: ::std::option::Option<i64>,
        #[doc = "Human-readable name for this source; may be user or system generated."]
        #[serde(rename = "userName", default)]
        pub user_name: ::std::option::Option<String>,
    }
    impl ::field_selector::FieldSelector for StageSource {
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
    pub struct StateFamilyConfig {
        #[doc = "If true, this family corresponds to a read operation."]
        #[serde(rename = "isRead", default)]
        pub is_read: ::std::option::Option<bool>,
        #[doc = "The state family value."]
        #[serde(rename = "stateFamily", default)]
        pub state_family: ::std::option::Option<String>,
    }
    impl ::field_selector::FieldSelector for StateFamilyConfig {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct Status {
        #[doc = "The status code, which should be an enum value of google.rpc.Code."]
        #[serde(rename = "code", default)]
        pub code: ::std::option::Option<i32>,
        #[doc = "A list of messages that carry the error details.  There is a common set of\nmessage types for APIs to use."]
        #[serde(rename = "details", default)]
        pub details:
            ::std::option::Option<Vec<::std::collections::BTreeMap<String, ::serde_json::Value>>>,
        #[doc = "A developer-facing error message, which should be in English. Any\nuser-facing error message should be localized and sent in the\ngoogle.rpc.Status.details field, or localized by the client."]
        #[serde(rename = "message", default)]
        pub message: ::std::option::Option<String>,
    }
    impl ::field_selector::FieldSelector for Status {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct Step {
        #[doc = "The kind of step in the Cloud Dataflow job."]
        #[serde(rename = "kind", default)]
        pub kind: ::std::option::Option<String>,
        #[doc = "The name that identifies the step. This must be unique for each\nstep with respect to all other steps in the Cloud Dataflow job."]
        #[serde(rename = "name", default)]
        pub name: ::std::option::Option<String>,
        #[doc = "Named properties associated with the step. Each kind of\npredefined step has its own required set of properties.\nMust be provided on Create.  Only retrieved with JOB_VIEW_ALL."]
        #[serde(rename = "properties", default)]
        pub properties:
            ::std::option::Option<::std::collections::BTreeMap<String, ::serde_json::Value>>,
    }
    impl ::field_selector::FieldSelector for Step {
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
    pub struct StreamLocation {
        #[doc = "The stream is a custom source."]
        #[serde(rename = "customSourceLocation", default)]
        pub custom_source_location: ::std::option::Option<crate::schemas::CustomSourceLocation>,
        #[doc = "The stream is a pubsub stream."]
        #[serde(rename = "pubsubLocation", default)]
        pub pubsub_location: ::std::option::Option<crate::schemas::PubsubLocation>,
        #[doc = "The stream is a streaming side input."]
        #[serde(rename = "sideInputLocation", default)]
        pub side_input_location: ::std::option::Option<crate::schemas::StreamingSideInputLocation>,
        #[doc = "The stream is part of another computation within the current\nstreaming Dataflow job."]
        #[serde(rename = "streamingStageLocation", default)]
        pub streaming_stage_location: ::std::option::Option<crate::schemas::StreamingStageLocation>,
    }
    impl ::field_selector::FieldSelector for StreamLocation {
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
    pub struct StreamingApplianceSnapshotConfig {
        #[doc = "Indicates which endpoint is used to import appliance state."]
        #[serde(rename = "importStateEndpoint", default)]
        pub import_state_endpoint: ::std::option::Option<String>,
        #[doc = "If set, indicates the snapshot id for the snapshot being performed."]
        #[serde(rename = "snapshotId", default)]
        pub snapshot_id: ::std::option::Option<String>,
    }
    impl ::field_selector::FieldSelector for StreamingApplianceSnapshotConfig {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct StreamingComputationConfig {
        #[doc = "Unique identifier for this computation."]
        #[serde(rename = "computationId", default)]
        pub computation_id: ::std::option::Option<String>,
        #[doc = "Instructions that comprise the computation."]
        #[serde(rename = "instructions", default)]
        pub instructions: ::std::option::Option<Vec<crate::schemas::ParallelInstruction>>,
        #[doc = "Stage name of this computation."]
        #[serde(rename = "stageName", default)]
        pub stage_name: ::std::option::Option<String>,
        #[doc = "System defined name for this computation."]
        #[serde(rename = "systemName", default)]
        pub system_name: ::std::option::Option<String>,
        #[doc = "Map from user name of stateful transforms in this stage to their state\nfamily."]
        #[serde(rename = "transformUserNameToStateFamily", default)]
        pub transform_user_name_to_state_family:
            ::std::option::Option<::std::collections::BTreeMap<String, String>>,
    }
    impl ::field_selector::FieldSelector for StreamingComputationConfig {
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
    pub struct StreamingComputationRanges {
        #[doc = "The ID of the computation."]
        #[serde(rename = "computationId", default)]
        pub computation_id: ::std::option::Option<String>,
        #[doc = "Data disk assignments for ranges from this computation."]
        #[serde(rename = "rangeAssignments", default)]
        pub range_assignments:
            ::std::option::Option<Vec<crate::schemas::KeyRangeDataDiskAssignment>>,
    }
    impl ::field_selector::FieldSelector for StreamingComputationRanges {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum StreamingComputationTaskTaskType {
        #[doc = "Start processing specified streaming computation range(s)."]
        StreamingComputationTaskStart,
        #[doc = "Stop processing specified streaming computation range(s)."]
        StreamingComputationTaskStop,
        #[doc = "The streaming computation task is unknown, or unspecified."]
        StreamingComputationTaskUnknown,
    }
    impl StreamingComputationTaskTaskType {
        pub fn as_str(self) -> &'static str {
            match self {
                StreamingComputationTaskTaskType::StreamingComputationTaskStart => {
                    "STREAMING_COMPUTATION_TASK_START"
                }
                StreamingComputationTaskTaskType::StreamingComputationTaskStop => {
                    "STREAMING_COMPUTATION_TASK_STOP"
                }
                StreamingComputationTaskTaskType::StreamingComputationTaskUnknown => {
                    "STREAMING_COMPUTATION_TASK_UNKNOWN"
                }
            }
        }
    }
    impl ::std::fmt::Display for StreamingComputationTaskTaskType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for StreamingComputationTaskTaskType {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for StreamingComputationTaskTaskType {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "STREAMING_COMPUTATION_TASK_START" => {
                    StreamingComputationTaskTaskType::StreamingComputationTaskStart
                }
                "STREAMING_COMPUTATION_TASK_STOP" => {
                    StreamingComputationTaskTaskType::StreamingComputationTaskStop
                }
                "STREAMING_COMPUTATION_TASK_UNKNOWN" => {
                    StreamingComputationTaskTaskType::StreamingComputationTaskUnknown
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
    impl ::field_selector::FieldSelector for StreamingComputationTaskTaskType {
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
    pub struct StreamingComputationTask {
        #[doc = "Contains ranges of a streaming computation this task should apply to."]
        #[serde(rename = "computationRanges", default)]
        pub computation_ranges:
            ::std::option::Option<Vec<crate::schemas::StreamingComputationRanges>>,
        #[doc = "Describes the set of data disks this task should apply to."]
        #[serde(rename = "dataDisks", default)]
        pub data_disks: ::std::option::Option<Vec<crate::schemas::MountedDataDisk>>,
        #[doc = "A type of streaming computation task."]
        #[serde(rename = "taskType", default)]
        pub task_type: ::std::option::Option<crate::schemas::StreamingComputationTaskTaskType>,
    }
    impl ::field_selector::FieldSelector for StreamingComputationTask {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct StreamingConfigTask {
        #[doc = "Maximum size for work item commit supported windmill storage layer."]
        #[serde(rename = "maxWorkItemCommitBytes", default)]
        #[serde(with = "crate::parsed_string")]
        pub max_work_item_commit_bytes: ::std::option::Option<i64>,
        #[doc = "Set of computation configuration information."]
        #[serde(rename = "streamingComputationConfigs", default)]
        pub streaming_computation_configs:
            ::std::option::Option<Vec<crate::schemas::StreamingComputationConfig>>,
        #[doc = "Map from user step names to state families."]
        #[serde(rename = "userStepToStateFamilyNameMap", default)]
        pub user_step_to_state_family_name_map:
            ::std::option::Option<::std::collections::BTreeMap<String, String>>,
        #[doc = "If present, the worker must use this endpoint to communicate with Windmill\nService dispatchers, otherwise the worker must continue to use whatever\nendpoint it had been using."]
        #[serde(rename = "windmillServiceEndpoint", default)]
        pub windmill_service_endpoint: ::std::option::Option<String>,
        #[doc = "If present, the worker must use this port to communicate with Windmill\nService dispatchers. Only applicable when windmill_service_endpoint is\nspecified."]
        #[serde(rename = "windmillServicePort", default)]
        #[serde(with = "crate::parsed_string")]
        pub windmill_service_port: ::std::option::Option<i64>,
    }
    impl ::field_selector::FieldSelector for StreamingConfigTask {
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
    pub struct StreamingSetupTask {
        #[doc = "The user has requested drain."]
        #[serde(rename = "drain", default)]
        pub drain: ::std::option::Option<bool>,
        #[doc = "The TCP port on which the worker should listen for messages from\nother streaming computation workers."]
        #[serde(rename = "receiveWorkPort", default)]
        pub receive_work_port: ::std::option::Option<i32>,
        #[doc = "Configures streaming appliance snapshot."]
        #[serde(rename = "snapshotConfig", default)]
        pub snapshot_config:
            ::std::option::Option<crate::schemas::StreamingApplianceSnapshotConfig>,
        #[doc = "The global topology of the streaming Dataflow job."]
        #[serde(rename = "streamingComputationTopology", default)]
        pub streaming_computation_topology: ::std::option::Option<crate::schemas::TopologyConfig>,
        #[doc = "The TCP port used by the worker to communicate with the Dataflow\nworker harness."]
        #[serde(rename = "workerHarnessPort", default)]
        pub worker_harness_port: ::std::option::Option<i32>,
    }
    impl ::field_selector::FieldSelector for StreamingSetupTask {
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
    pub struct StreamingSideInputLocation {
        #[doc = "Identifies the state family where this side input is stored."]
        #[serde(rename = "stateFamily", default)]
        pub state_family: ::std::option::Option<String>,
        #[doc = "Identifies the particular side input within the streaming Dataflow job."]
        #[serde(rename = "tag", default)]
        pub tag: ::std::option::Option<String>,
    }
    impl ::field_selector::FieldSelector for StreamingSideInputLocation {
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
    pub struct StreamingStageLocation {
        #[doc = "Identifies the particular stream within the streaming Dataflow\njob."]
        #[serde(rename = "streamId", default)]
        pub stream_id: ::std::option::Option<String>,
    }
    impl ::field_selector::FieldSelector for StreamingStageLocation {
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
    pub struct StringList {
        #[doc = "Elements of the list."]
        #[serde(rename = "elements", default)]
        pub elements: ::std::option::Option<Vec<String>>,
    }
    impl ::field_selector::FieldSelector for StringList {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct StructuredMessage {
        #[doc = "Identifier for this message type.  Used by external systems to\ninternationalize or personalize message."]
        #[serde(rename = "messageKey", default)]
        pub message_key: ::std::option::Option<String>,
        #[doc = "Human-readable version of message."]
        #[serde(rename = "messageText", default)]
        pub message_text: ::std::option::Option<String>,
        #[doc = "The structured data associated with this message."]
        #[serde(rename = "parameters", default)]
        pub parameters: ::std::option::Option<Vec<crate::schemas::Parameter>>,
    }
    impl ::field_selector::FieldSelector for StructuredMessage {
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
    pub struct TaskRunnerSettings {
        #[doc = "Whether to also send taskrunner log info to stderr."]
        #[serde(rename = "alsologtostderr", default)]
        pub alsologtostderr: ::std::option::Option<bool>,
        #[doc = "The location on the worker for task-specific subdirectories."]
        #[serde(rename = "baseTaskDir", default)]
        pub base_task_dir: ::std::option::Option<String>,
        #[doc = "The base URL for the taskrunner to use when accessing Google Cloud APIs.\n\nWhen workers access Google Cloud APIs, they logically do so via\nrelative URLs.  If this field is specified, it supplies the base\nURL to use for resolving these relative URLs.  The normative\nalgorithm used is defined by RFC 1808, \"Relative Uniform Resource\nLocators\".\n\nIf not specified, the default value is \"http://www.googleapis.com/\""]
        #[serde(rename = "baseUrl", default)]
        pub base_url: ::std::option::Option<String>,
        #[doc = "The file to store preprocessing commands in."]
        #[serde(rename = "commandlinesFileName", default)]
        pub commandlines_file_name: ::std::option::Option<String>,
        #[doc = "Whether to continue taskrunner if an exception is hit."]
        #[serde(rename = "continueOnException", default)]
        pub continue_on_exception: ::std::option::Option<bool>,
        #[doc = "The API version of endpoint, e.g. \"v1b3\""]
        #[serde(rename = "dataflowApiVersion", default)]
        pub dataflow_api_version: ::std::option::Option<String>,
        #[doc = "The command to launch the worker harness."]
        #[serde(rename = "harnessCommand", default)]
        pub harness_command: ::std::option::Option<String>,
        #[doc = "The suggested backend language."]
        #[serde(rename = "languageHint", default)]
        pub language_hint: ::std::option::Option<String>,
        #[doc = "The directory on the VM to store logs."]
        #[serde(rename = "logDir", default)]
        pub log_dir: ::std::option::Option<String>,
        #[doc = "Whether to send taskrunner log info to Google Compute Engine VM serial\nconsole."]
        #[serde(rename = "logToSerialconsole", default)]
        pub log_to_serialconsole: ::std::option::Option<bool>,
        #[doc = "Indicates where to put logs.  If this is not specified, the logs\nwill not be uploaded.\n\nThe supported resource type is:\n\nGoogle Cloud Storage:\nstorage.googleapis.com/{bucket}/{object}\nbucket.storage.googleapis.com/{object}"]
        #[serde(rename = "logUploadLocation", default)]
        pub log_upload_location: ::std::option::Option<String>,
        #[doc = "The OAuth2 scopes to be requested by the taskrunner in order to\naccess the Cloud Dataflow API."]
        #[serde(rename = "oauthScopes", default)]
        pub oauth_scopes: ::std::option::Option<Vec<String>>,
        #[doc = "The settings to pass to the parallel worker harness."]
        #[serde(rename = "parallelWorkerSettings", default)]
        pub parallel_worker_settings: ::std::option::Option<crate::schemas::WorkerSettings>,
        #[doc = "The streaming worker main class name."]
        #[serde(rename = "streamingWorkerMainClass", default)]
        pub streaming_worker_main_class: ::std::option::Option<String>,
        #[doc = "The UNIX group ID on the worker VM to use for tasks launched by\ntaskrunner; e.g. \"wheel\"."]
        #[serde(rename = "taskGroup", default)]
        pub task_group: ::std::option::Option<String>,
        #[doc = "The UNIX user ID on the worker VM to use for tasks launched by\ntaskrunner; e.g. \"root\"."]
        #[serde(rename = "taskUser", default)]
        pub task_user: ::std::option::Option<String>,
        #[doc = "The prefix of the resources the taskrunner should use for\ntemporary storage.\n\nThe supported resource type is:\n\nGoogle Cloud Storage:\nstorage.googleapis.com/{bucket}/{object}\nbucket.storage.googleapis.com/{object}"]
        #[serde(rename = "tempStoragePrefix", default)]
        pub temp_storage_prefix: ::std::option::Option<String>,
        #[doc = "The ID string of the VM."]
        #[serde(rename = "vmId", default)]
        pub vm_id: ::std::option::Option<String>,
        #[doc = "The file to store the workflow in."]
        #[serde(rename = "workflowFileName", default)]
        pub workflow_file_name: ::std::option::Option<String>,
    }
    impl ::field_selector::FieldSelector for TaskRunnerSettings {
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
    pub struct TemplateMetadata {
        #[doc = "Optional. A description of the template."]
        #[serde(rename = "description", default)]
        pub description: ::std::option::Option<String>,
        #[doc = "Required. The name of the template."]
        #[serde(rename = "name", default)]
        pub name: ::std::option::Option<String>,
        #[doc = "The parameters for the template."]
        #[serde(rename = "parameters", default)]
        pub parameters: ::std::option::Option<Vec<crate::schemas::ParameterMetadata>>,
    }
    impl ::field_selector::FieldSelector for TemplateMetadata {
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
    pub struct TopologyConfig {
        #[doc = "The computations associated with a streaming Dataflow job."]
        #[serde(rename = "computations", default)]
        pub computations: ::std::option::Option<Vec<crate::schemas::ComputationTopology>>,
        #[doc = "The disks assigned to a streaming Dataflow job."]
        #[serde(rename = "dataDiskAssignments", default)]
        pub data_disk_assignments: ::std::option::Option<Vec<crate::schemas::DataDiskAssignment>>,
        #[doc = "The size (in bits) of keys that will be assigned to source messages."]
        #[serde(rename = "forwardingKeyBits", default)]
        pub forwarding_key_bits: ::std::option::Option<i32>,
        #[doc = "Version number for persistent state."]
        #[serde(rename = "persistentStateVersion", default)]
        pub persistent_state_version: ::std::option::Option<i32>,
        #[doc = "Maps user stage names to stable computation names."]
        #[serde(rename = "userStageToComputationNameMap", default)]
        pub user_stage_to_computation_name_map:
            ::std::option::Option<::std::collections::BTreeMap<String, String>>,
    }
    impl ::field_selector::FieldSelector for TopologyConfig {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum TransformSummaryKind {
        #[doc = "Constructs from a constant value, such as with Create.of."]
        ConstantKind,
        #[doc = "Flatten transform."]
        FlattenKind,
        #[doc = "Group By Key transform."]
        GroupByKeyKind,
        #[doc = "ParDo transform."]
        ParDoKind,
        #[doc = "Read transform."]
        ReadKind,
        #[doc = "Opening or closing a shuffle session, often as part of a GroupByKey."]
        ShuffleKind,
        #[doc = "Creates a Singleton view of a collection."]
        SingletonKind,
        #[doc = "Unrecognized transform type."]
        UnknownKind,
        #[doc = "Write transform."]
        WriteKind,
    }
    impl TransformSummaryKind {
        pub fn as_str(self) -> &'static str {
            match self {
                TransformSummaryKind::ConstantKind => "CONSTANT_KIND",
                TransformSummaryKind::FlattenKind => "FLATTEN_KIND",
                TransformSummaryKind::GroupByKeyKind => "GROUP_BY_KEY_KIND",
                TransformSummaryKind::ParDoKind => "PAR_DO_KIND",
                TransformSummaryKind::ReadKind => "READ_KIND",
                TransformSummaryKind::ShuffleKind => "SHUFFLE_KIND",
                TransformSummaryKind::SingletonKind => "SINGLETON_KIND",
                TransformSummaryKind::UnknownKind => "UNKNOWN_KIND",
                TransformSummaryKind::WriteKind => "WRITE_KIND",
            }
        }
    }
    impl ::std::fmt::Display for TransformSummaryKind {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for TransformSummaryKind {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for TransformSummaryKind {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "CONSTANT_KIND" => TransformSummaryKind::ConstantKind,
                "FLATTEN_KIND" => TransformSummaryKind::FlattenKind,
                "GROUP_BY_KEY_KIND" => TransformSummaryKind::GroupByKeyKind,
                "PAR_DO_KIND" => TransformSummaryKind::ParDoKind,
                "READ_KIND" => TransformSummaryKind::ReadKind,
                "SHUFFLE_KIND" => TransformSummaryKind::ShuffleKind,
                "SINGLETON_KIND" => TransformSummaryKind::SingletonKind,
                "UNKNOWN_KIND" => TransformSummaryKind::UnknownKind,
                "WRITE_KIND" => TransformSummaryKind::WriteKind,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::field_selector::FieldSelector for TransformSummaryKind {
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
    pub struct TransformSummary {
        #[doc = "Transform-specific display data."]
        #[serde(rename = "displayData", default)]
        pub display_data: ::std::option::Option<Vec<crate::schemas::DisplayData>>,
        #[doc = "SDK generated id of this transform instance."]
        #[serde(rename = "id", default)]
        pub id: ::std::option::Option<String>,
        #[doc = "User names for all collection inputs to this transform."]
        #[serde(rename = "inputCollectionName", default)]
        pub input_collection_name: ::std::option::Option<Vec<String>>,
        #[doc = "Type of transform."]
        #[serde(rename = "kind", default)]
        pub kind: ::std::option::Option<crate::schemas::TransformSummaryKind>,
        #[doc = "User provided name for this transform instance."]
        #[serde(rename = "name", default)]
        pub name: ::std::option::Option<String>,
        #[doc = "User  names for all collection outputs to this transform."]
        #[serde(rename = "outputCollectionName", default)]
        pub output_collection_name: ::std::option::Option<Vec<String>>,
    }
    impl ::field_selector::FieldSelector for TransformSummary {
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
    pub struct ValidateResponse {
        #[doc = "Will be empty if validation succeeds."]
        #[serde(rename = "errorMessage", default)]
        pub error_message: ::std::option::Option<String>,
    }
    impl ::field_selector::FieldSelector for ValidateResponse {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct WorkItem {
        #[doc = "Work item-specific configuration as an opaque blob."]
        #[serde(rename = "configuration", default)]
        pub configuration: ::std::option::Option<String>,
        #[doc = "Identifies this WorkItem."]
        #[serde(rename = "id", default)]
        #[serde(with = "crate::parsed_string")]
        pub id: ::std::option::Option<i64>,
        #[doc = "The initial index to use when reporting the status of the WorkItem."]
        #[serde(rename = "initialReportIndex", default)]
        #[serde(with = "crate::parsed_string")]
        pub initial_report_index: ::std::option::Option<i64>,
        #[doc = "Identifies the workflow job this WorkItem belongs to."]
        #[serde(rename = "jobId", default)]
        pub job_id: ::std::option::Option<String>,
        #[doc = "Time when the lease on this Work will expire."]
        #[serde(rename = "leaseExpireTime", default)]
        pub lease_expire_time: ::std::option::Option<String>,
        #[doc = "Additional information for MapTask WorkItems."]
        #[serde(rename = "mapTask", default)]
        pub map_task: ::std::option::Option<crate::schemas::MapTask>,
        #[doc = "Any required packages that need to be fetched in order to execute\nthis WorkItem."]
        #[serde(rename = "packages", default)]
        pub packages: ::std::option::Option<Vec<crate::schemas::Package>>,
        #[doc = "Identifies the cloud project this WorkItem belongs to."]
        #[serde(rename = "projectId", default)]
        pub project_id: ::std::option::Option<String>,
        #[doc = "Recommended reporting interval."]
        #[serde(rename = "reportStatusInterval", default)]
        pub report_status_interval: ::std::option::Option<String>,
        #[doc = "Additional information for SeqMapTask WorkItems."]
        #[serde(rename = "seqMapTask", default)]
        pub seq_map_task: ::std::option::Option<crate::schemas::SeqMapTask>,
        #[doc = "Additional information for ShellTask WorkItems."]
        #[serde(rename = "shellTask", default)]
        pub shell_task: ::std::option::Option<crate::schemas::ShellTask>,
        #[doc = "Additional information for source operation WorkItems."]
        #[serde(rename = "sourceOperationTask", default)]
        pub source_operation_task: ::std::option::Option<crate::schemas::SourceOperationRequest>,
        #[doc = "Additional information for StreamingComputationTask WorkItems."]
        #[serde(rename = "streamingComputationTask", default)]
        pub streaming_computation_task:
            ::std::option::Option<crate::schemas::StreamingComputationTask>,
        #[doc = "Additional information for StreamingConfigTask WorkItems."]
        #[serde(rename = "streamingConfigTask", default)]
        pub streaming_config_task: ::std::option::Option<crate::schemas::StreamingConfigTask>,
        #[doc = "Additional information for StreamingSetupTask WorkItems."]
        #[serde(rename = "streamingSetupTask", default)]
        pub streaming_setup_task: ::std::option::Option<crate::schemas::StreamingSetupTask>,
    }
    impl ::field_selector::FieldSelector for WorkItem {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct WorkItemServiceState {
        #[doc = "Other data returned by the service, specific to the particular\nworker harness."]
        #[serde(rename = "harnessData", default)]
        pub harness_data:
            ::std::option::Option<::std::collections::BTreeMap<String, ::serde_json::Value>>,
        #[doc = "A hot key is a symptom of poor data distribution in which there are enough\nelements mapped to a single key to impact pipeline performance. When\npresent, this field includes metadata associated with any hot key."]
        #[serde(rename = "hotKeyDetection", default)]
        pub hot_key_detection: ::std::option::Option<crate::schemas::HotKeyDetection>,
        #[doc = "Time at which the current lease will expire."]
        #[serde(rename = "leaseExpireTime", default)]
        pub lease_expire_time: ::std::option::Option<String>,
        #[doc = "The short ids that workers should use in subsequent metric updates.\nWorkers should strive to use short ids whenever possible, but it is ok\nto request the short_id again if a worker lost track of it\n(e.g. if the worker is recovering from a crash).\nNOTE: it is possible that the response may have short ids for a subset\nof the metrics."]
        #[serde(rename = "metricShortId", default)]
        pub metric_short_id: ::std::option::Option<Vec<crate::schemas::MetricShortId>>,
        #[doc = "The index value to use for the next report sent by the worker.\nNote: If the report call fails for whatever reason, the worker should\nreuse this index for subsequent report attempts."]
        #[serde(rename = "nextReportIndex", default)]
        #[serde(with = "crate::parsed_string")]
        pub next_report_index: ::std::option::Option<i64>,
        #[doc = "New recommended reporting interval."]
        #[serde(rename = "reportStatusInterval", default)]
        pub report_status_interval: ::std::option::Option<String>,
        #[doc = "The progress point in the WorkItem where the Dataflow service\nsuggests that the worker truncate the task."]
        #[serde(rename = "splitRequest", default)]
        pub split_request: ::std::option::Option<crate::schemas::ApproximateSplitRequest>,
        #[doc = "DEPRECATED in favor of split_request."]
        #[serde(rename = "suggestedStopPoint", default)]
        pub suggested_stop_point: ::std::option::Option<crate::schemas::ApproximateProgress>,
        #[doc = "Obsolete, always empty."]
        #[serde(rename = "suggestedStopPosition", default)]
        pub suggested_stop_position: ::std::option::Option<crate::schemas::Position>,
    }
    impl ::field_selector::FieldSelector for WorkItemServiceState {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct WorkItemStatus {
        #[doc = "True if the WorkItem was completed (successfully or unsuccessfully)."]
        #[serde(rename = "completed", default)]
        pub completed: ::std::option::Option<bool>,
        #[doc = "Worker output counters for this WorkItem."]
        #[serde(rename = "counterUpdates", default)]
        pub counter_updates: ::std::option::Option<Vec<crate::schemas::CounterUpdate>>,
        #[doc = "See documentation of stop_position."]
        #[serde(rename = "dynamicSourceSplit", default)]
        pub dynamic_source_split: ::std::option::Option<crate::schemas::DynamicSourceSplit>,
        #[doc = "Specifies errors which occurred during processing.  If errors are\nprovided, and completed = true, then the WorkItem is considered\nto have failed."]
        #[serde(rename = "errors", default)]
        pub errors: ::std::option::Option<Vec<crate::schemas::Status>>,
        #[doc = "DEPRECATED in favor of counter_updates."]
        #[serde(rename = "metricUpdates", default)]
        pub metric_updates: ::std::option::Option<Vec<crate::schemas::MetricUpdate>>,
        #[doc = "DEPRECATED in favor of reported_progress."]
        #[serde(rename = "progress", default)]
        pub progress: ::std::option::Option<crate::schemas::ApproximateProgress>,
        #[doc = "The report index.  When a WorkItem is leased, the lease will\ncontain an initial report index.  When a WorkItem's status is\nreported to the system, the report should be sent with\nthat report index, and the response will contain the index the\nworker should use for the next report.  Reports received with\nunexpected index values will be rejected by the service.\n\nIn order to preserve idempotency, the worker should not alter the\ncontents of a report, even if the worker must submit the same\nreport multiple times before getting back a response.  The worker\nshould not submit a subsequent report until the response for the\nprevious report had been received from the service."]
        #[serde(rename = "reportIndex", default)]
        #[serde(with = "crate::parsed_string")]
        pub report_index: ::std::option::Option<i64>,
        #[doc = "The worker's progress through this WorkItem."]
        #[serde(rename = "reportedProgress", default)]
        pub reported_progress: ::std::option::Option<crate::schemas::ApproximateReportedProgress>,
        #[doc = "Amount of time the worker requests for its lease."]
        #[serde(rename = "requestedLeaseDuration", default)]
        pub requested_lease_duration: ::std::option::Option<String>,
        #[doc = "DEPRECATED in favor of dynamic_source_split."]
        #[serde(rename = "sourceFork", default)]
        pub source_fork: ::std::option::Option<crate::schemas::SourceFork>,
        #[doc = "If the work item represented a SourceOperationRequest, and the work\nis completed, contains the result of the operation."]
        #[serde(rename = "sourceOperationResponse", default)]
        pub source_operation_response:
            ::std::option::Option<crate::schemas::SourceOperationResponse>,
        #[doc = "A worker may split an active map task in two parts, \"primary\" and\n\"residual\", continuing to process the primary part and returning the\nresidual part into the pool of available work.\nThis event is called a \"dynamic split\" and is critical to the dynamic\nwork rebalancing feature. The two obtained sub-tasks are called\n\"parts\" of the split.\nThe parts, if concatenated, must represent the same input as would\nbe read by the current task if the split did not happen.\nThe exact way in which the original task is decomposed into the two\nparts is specified either as a position demarcating them\n(stop_position), or explicitly as two DerivedSources, if this\ntask consumes a user-defined source type (dynamic_source_split).\n\nThe \"current\" task is adjusted as a result of the split: after a task\nwith range [A, B) sends a stop_position update at C, its range is\nconsidered to be [A, C), e.g.:\n\n* Progress should be interpreted relative to the new range, e.g.\n  \"75% completed\" means \"75% of [A, C) completed\"\n* The worker should interpret proposed_stop_position relative to the\n  new range, e.g. \"split at 68%\" should be interpreted as\n  \"split at 68% of [A, C)\".\n* If the worker chooses to split again using stop_position, only\n  stop_positions in [A, C) will be accepted.\n* Etc.\n  dynamic_source_split has similar semantics: e.g., if a task with\n  source S splits using dynamic_source_split into {P, R}\n  (where P and R must be together equivalent to S), then subsequent\n  progress and proposed_stop_position should be interpreted relative\n  to P, and in a potential subsequent dynamic_source_split into {P', R'},\n  P' and R' must be together equivalent to P, etc."]
        #[serde(rename = "stopPosition", default)]
        pub stop_position: ::std::option::Option<crate::schemas::Position>,
        #[doc = "Total time the worker spent being throttled by external systems."]
        #[serde(rename = "totalThrottlerWaitTimeSeconds", default)]
        pub total_throttler_wait_time_seconds: ::std::option::Option<f64>,
        #[doc = "Identifies the WorkItem."]
        #[serde(rename = "workItemId", default)]
        pub work_item_id: ::std::option::Option<String>,
    }
    impl ::field_selector::FieldSelector for WorkItemStatus {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct WorkerHealthReport {
        #[doc = "A message describing any unusual health reports."]
        #[serde(rename = "msg", default)]
        pub msg: ::std::option::Option<String>,
        #[doc = "The pods running on the worker. See:\nhttp://kubernetes.io/v1.1/docs/api-reference/v1/definitions.html#_v1_pod\n\nThis field is used by the worker to send the status of the indvidual\ncontainers running on each worker."]
        #[serde(rename = "pods", default)]
        pub pods:
            ::std::option::Option<Vec<::std::collections::BTreeMap<String, ::serde_json::Value>>>,
        #[doc = "The interval at which the worker is sending health reports.\nThe default value of 0 should be interpreted as the field is not being\nexplicitly set by the worker."]
        #[serde(rename = "reportInterval", default)]
        pub report_interval: ::std::option::Option<String>,
        #[doc = "Whether the VM is in a permanently broken state.\nBroken VMs should be abandoned or deleted ASAP to avoid assigning or\ncompleting any work."]
        #[serde(rename = "vmIsBroken", default)]
        pub vm_is_broken: ::std::option::Option<bool>,
        #[doc = "Whether the VM is currently healthy."]
        #[serde(rename = "vmIsHealthy", default)]
        pub vm_is_healthy: ::std::option::Option<bool>,
        #[doc = "The time the VM was booted."]
        #[serde(rename = "vmStartupTime", default)]
        pub vm_startup_time: ::std::option::Option<String>,
    }
    impl ::field_selector::FieldSelector for WorkerHealthReport {
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
    pub struct WorkerHealthReportResponse {
        #[doc = "A positive value indicates the worker should change its reporting interval\nto the specified value.\n\nThe default value of zero means no change in report rate is requested by\nthe server."]
        #[serde(rename = "reportInterval", default)]
        pub report_interval: ::std::option::Option<String>,
    }
    impl ::field_selector::FieldSelector for WorkerHealthReportResponse {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum WorkerLifecycleEventEvent {
        #[doc = "Our container code starts running. Multiple containers could be\ndistinguished with WorkerMessage.labels if desired."]
        ContainerStart,
        #[doc = "The worker has a functional external network connection."]
        NetworkUp,
        #[doc = "The time the VM started."]
        OsStart,
        #[doc = "Finished installing SDK."]
        SdkInstallFinish,
        #[doc = "For applicable SDKs, started installation of SDK and worker packages."]
        SdkInstallStart,
        #[doc = "Finished downloading all staging files."]
        StagingFilesDownloadFinish,
        #[doc = "Started downloading staging files."]
        StagingFilesDownloadStart,
        #[doc = "Invalid event."]
        UnknownEvent,
    }
    impl WorkerLifecycleEventEvent {
        pub fn as_str(self) -> &'static str {
            match self {
                WorkerLifecycleEventEvent::ContainerStart => "CONTAINER_START",
                WorkerLifecycleEventEvent::NetworkUp => "NETWORK_UP",
                WorkerLifecycleEventEvent::OsStart => "OS_START",
                WorkerLifecycleEventEvent::SdkInstallFinish => "SDK_INSTALL_FINISH",
                WorkerLifecycleEventEvent::SdkInstallStart => "SDK_INSTALL_START",
                WorkerLifecycleEventEvent::StagingFilesDownloadFinish => {
                    "STAGING_FILES_DOWNLOAD_FINISH"
                }
                WorkerLifecycleEventEvent::StagingFilesDownloadStart => {
                    "STAGING_FILES_DOWNLOAD_START"
                }
                WorkerLifecycleEventEvent::UnknownEvent => "UNKNOWN_EVENT",
            }
        }
    }
    impl ::std::fmt::Display for WorkerLifecycleEventEvent {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for WorkerLifecycleEventEvent {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for WorkerLifecycleEventEvent {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "CONTAINER_START" => WorkerLifecycleEventEvent::ContainerStart,
                "NETWORK_UP" => WorkerLifecycleEventEvent::NetworkUp,
                "OS_START" => WorkerLifecycleEventEvent::OsStart,
                "SDK_INSTALL_FINISH" => WorkerLifecycleEventEvent::SdkInstallFinish,
                "SDK_INSTALL_START" => WorkerLifecycleEventEvent::SdkInstallStart,
                "STAGING_FILES_DOWNLOAD_FINISH" => {
                    WorkerLifecycleEventEvent::StagingFilesDownloadFinish
                }
                "STAGING_FILES_DOWNLOAD_START" => {
                    WorkerLifecycleEventEvent::StagingFilesDownloadStart
                }
                "UNKNOWN_EVENT" => WorkerLifecycleEventEvent::UnknownEvent,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::field_selector::FieldSelector for WorkerLifecycleEventEvent {
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
    pub struct WorkerLifecycleEvent {
        #[doc = "The start time of this container. All events will report this so that\nevents can be grouped together across container/VM restarts."]
        #[serde(rename = "containerStartTime", default)]
        pub container_start_time: ::std::option::Option<String>,
        #[doc = "The event being reported."]
        #[serde(rename = "event", default)]
        pub event: ::std::option::Option<crate::schemas::WorkerLifecycleEventEvent>,
        #[doc = "Other stats that can accompany an event. E.g.\n{ \"downloaded_bytes\" : \"123456\" }"]
        #[serde(rename = "metadata", default)]
        pub metadata: ::std::option::Option<::std::collections::BTreeMap<String, String>>,
    }
    impl ::field_selector::FieldSelector for WorkerLifecycleEvent {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct WorkerMessage {
        #[doc = "Labels are used to group WorkerMessages.\nFor example, a worker_message about a particular container\nmight have the labels:\n{ \"JOB_ID\": \"2015-04-22\",\n\"WORKER_ID\": \"wordcount-vm-2015\u{2026}\"\n\"CONTAINER_TYPE\": \"worker\",\n\"CONTAINER_ID\": \"ac1234def\"}\nLabel tags typically correspond to Label enum values. However, for ease\nof development other strings can be used as tags. LABEL_UNSPECIFIED should\nnot be used here."]
        #[serde(rename = "labels", default)]
        pub labels: ::std::option::Option<::std::collections::BTreeMap<String, String>>,
        #[doc = "The timestamp of the worker_message."]
        #[serde(rename = "time", default)]
        pub time: ::std::option::Option<String>,
        #[doc = "The health of a worker."]
        #[serde(rename = "workerHealthReport", default)]
        pub worker_health_report: ::std::option::Option<crate::schemas::WorkerHealthReport>,
        #[doc = "Record of worker lifecycle events."]
        #[serde(rename = "workerLifecycleEvent", default)]
        pub worker_lifecycle_event: ::std::option::Option<crate::schemas::WorkerLifecycleEvent>,
        #[doc = "A worker message code."]
        #[serde(rename = "workerMessageCode", default)]
        pub worker_message_code: ::std::option::Option<crate::schemas::WorkerMessageCode>,
        #[doc = "Resource metrics reported by workers."]
        #[serde(rename = "workerMetrics", default)]
        pub worker_metrics: ::std::option::Option<crate::schemas::ResourceUtilizationReport>,
        #[doc = "Shutdown notice by workers."]
        #[serde(rename = "workerShutdownNotice", default)]
        pub worker_shutdown_notice: ::std::option::Option<crate::schemas::WorkerShutdownNotice>,
    }
    impl ::field_selector::FieldSelector for WorkerMessage {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct WorkerMessageCode {
        #[doc = "The code is a string intended for consumption by a machine that identifies\nthe type of message being sent.\nExamples:\n\n1. \"HARNESS_STARTED\" might be used to indicate the worker harness has\n   started.\n1. \"GCS_DOWNLOAD_ERROR\" might be used to indicate an error downloading\n   a GCS file as part of the boot process of one of the worker containers.\n\nThis is a string and not an enum to make it easy to add new codes without\nwaiting for an API change."]
        #[serde(rename = "code", default)]
        pub code: ::std::option::Option<String>,
        #[doc = "Parameters contains specific information about the code.\n\nThis is a struct to allow parameters of different types.\n\nExamples:\n\n1. For a \"HARNESS_STARTED\" message parameters might provide the name\n   of the worker and additional data like timing information.\n1. For a \"GCS_DOWNLOAD_ERROR\" parameters might contain fields listing\n   the GCS objects being downloaded and fields containing errors.\n\nIn general complex data structures should be avoided. If a worker\nneeds to send a specific and complicated data structure then please\nconsider defining a new proto and adding it to the data oneof in\nWorkerMessageResponse.\n\nConventions:\nParameters should only be used for information that isn't typically passed\nas a label.\nhostname and other worker identifiers should almost always be passed\nas labels since they will be included on most messages."]
        #[serde(rename = "parameters", default)]
        pub parameters:
            ::std::option::Option<::std::collections::BTreeMap<String, ::serde_json::Value>>,
    }
    impl ::field_selector::FieldSelector for WorkerMessageCode {
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
    pub struct WorkerMessageResponse {
        #[doc = "The service's response to a worker's health report."]
        #[serde(rename = "workerHealthReportResponse", default)]
        pub worker_health_report_response:
            ::std::option::Option<crate::schemas::WorkerHealthReportResponse>,
        #[doc = "Service's response to reporting worker metrics (currently empty)."]
        #[serde(rename = "workerMetricsResponse", default)]
        pub worker_metrics_response:
            ::std::option::Option<crate::schemas::ResourceUtilizationReportResponse>,
        #[doc = "Service's response to shutdown notice (currently empty)."]
        #[serde(rename = "workerShutdownNoticeResponse", default)]
        pub worker_shutdown_notice_response:
            ::std::option::Option<crate::schemas::WorkerShutdownNoticeResponse>,
    }
    impl ::field_selector::FieldSelector for WorkerMessageResponse {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum WorkerPoolDefaultPackageSet {
        #[doc = "Stage packages typically useful to workers written in Java."]
        DefaultPackageSetJava,
        #[doc = "Indicates that no packages should be staged at the worker unless\nexplicitly specified by the job."]
        DefaultPackageSetNone,
        #[doc = "Stage pacakges typically useful to workers written in Python."]
        DefaultPackageSetPython,
        #[doc = "The default set of packages to stage is unknown, or unspecified."]
        DefaultPackageSetUnknown,
    }
    impl WorkerPoolDefaultPackageSet {
        pub fn as_str(self) -> &'static str {
            match self {
                WorkerPoolDefaultPackageSet::DefaultPackageSetJava => "DEFAULT_PACKAGE_SET_JAVA",
                WorkerPoolDefaultPackageSet::DefaultPackageSetNone => "DEFAULT_PACKAGE_SET_NONE",
                WorkerPoolDefaultPackageSet::DefaultPackageSetPython => {
                    "DEFAULT_PACKAGE_SET_PYTHON"
                }
                WorkerPoolDefaultPackageSet::DefaultPackageSetUnknown => {
                    "DEFAULT_PACKAGE_SET_UNKNOWN"
                }
            }
        }
    }
    impl ::std::fmt::Display for WorkerPoolDefaultPackageSet {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for WorkerPoolDefaultPackageSet {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for WorkerPoolDefaultPackageSet {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "DEFAULT_PACKAGE_SET_JAVA" => WorkerPoolDefaultPackageSet::DefaultPackageSetJava,
                "DEFAULT_PACKAGE_SET_NONE" => WorkerPoolDefaultPackageSet::DefaultPackageSetNone,
                "DEFAULT_PACKAGE_SET_PYTHON" => {
                    WorkerPoolDefaultPackageSet::DefaultPackageSetPython
                }
                "DEFAULT_PACKAGE_SET_UNKNOWN" => {
                    WorkerPoolDefaultPackageSet::DefaultPackageSetUnknown
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
    impl ::field_selector::FieldSelector for WorkerPoolDefaultPackageSet {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum WorkerPoolIpConfiguration {
        #[doc = "Workers should have private IP addresses."]
        WorkerIpPrivate,
        #[doc = "Workers should have public IP addresses."]
        WorkerIpPublic,
        #[doc = "The configuration is unknown, or unspecified."]
        WorkerIpUnspecified,
    }
    impl WorkerPoolIpConfiguration {
        pub fn as_str(self) -> &'static str {
            match self {
                WorkerPoolIpConfiguration::WorkerIpPrivate => "WORKER_IP_PRIVATE",
                WorkerPoolIpConfiguration::WorkerIpPublic => "WORKER_IP_PUBLIC",
                WorkerPoolIpConfiguration::WorkerIpUnspecified => "WORKER_IP_UNSPECIFIED",
            }
        }
    }
    impl ::std::fmt::Display for WorkerPoolIpConfiguration {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for WorkerPoolIpConfiguration {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for WorkerPoolIpConfiguration {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "WORKER_IP_PRIVATE" => WorkerPoolIpConfiguration::WorkerIpPrivate,
                "WORKER_IP_PUBLIC" => WorkerPoolIpConfiguration::WorkerIpPublic,
                "WORKER_IP_UNSPECIFIED" => WorkerPoolIpConfiguration::WorkerIpUnspecified,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::field_selector::FieldSelector for WorkerPoolIpConfiguration {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum WorkerPoolTeardownPolicy {
        #[doc = "Always teardown the resource."]
        TeardownAlways,
        #[doc = "Never teardown the resource. This is useful for debugging and\ndevelopment."]
        TeardownNever,
        #[doc = "Teardown the resource on success. This is useful for debugging\nfailures."]
        TeardownOnSuccess,
        #[doc = "The teardown policy isn't specified, or is unknown."]
        TeardownPolicyUnknown,
    }
    impl WorkerPoolTeardownPolicy {
        pub fn as_str(self) -> &'static str {
            match self {
                WorkerPoolTeardownPolicy::TeardownAlways => "TEARDOWN_ALWAYS",
                WorkerPoolTeardownPolicy::TeardownNever => "TEARDOWN_NEVER",
                WorkerPoolTeardownPolicy::TeardownOnSuccess => "TEARDOWN_ON_SUCCESS",
                WorkerPoolTeardownPolicy::TeardownPolicyUnknown => "TEARDOWN_POLICY_UNKNOWN",
            }
        }
    }
    impl ::std::fmt::Display for WorkerPoolTeardownPolicy {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for WorkerPoolTeardownPolicy {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for WorkerPoolTeardownPolicy {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "TEARDOWN_ALWAYS" => WorkerPoolTeardownPolicy::TeardownAlways,
                "TEARDOWN_NEVER" => WorkerPoolTeardownPolicy::TeardownNever,
                "TEARDOWN_ON_SUCCESS" => WorkerPoolTeardownPolicy::TeardownOnSuccess,
                "TEARDOWN_POLICY_UNKNOWN" => WorkerPoolTeardownPolicy::TeardownPolicyUnknown,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::field_selector::FieldSelector for WorkerPoolTeardownPolicy {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct WorkerPool {
        #[doc = "Settings for autoscaling of this WorkerPool."]
        #[serde(rename = "autoscalingSettings", default)]
        pub autoscaling_settings: ::std::option::Option<crate::schemas::AutoscalingSettings>,
        #[doc = "Data disks that are used by a VM in this workflow."]
        #[serde(rename = "dataDisks", default)]
        pub data_disks: ::std::option::Option<Vec<crate::schemas::Disk>>,
        #[doc = "The default package set to install.  This allows the service to\nselect a default set of packages which are useful to worker\nharnesses written in a particular language."]
        #[serde(rename = "defaultPackageSet", default)]
        pub default_package_set: ::std::option::Option<crate::schemas::WorkerPoolDefaultPackageSet>,
        #[doc = "Size of root disk for VMs, in GB.  If zero or unspecified, the service will\nattempt to choose a reasonable default."]
        #[serde(rename = "diskSizeGb", default)]
        pub disk_size_gb: ::std::option::Option<i32>,
        #[doc = "Fully qualified source image for disks."]
        #[serde(rename = "diskSourceImage", default)]
        pub disk_source_image: ::std::option::Option<String>,
        #[doc = "Type of root disk for VMs.  If empty or unspecified, the service will\nattempt to choose a reasonable default."]
        #[serde(rename = "diskType", default)]
        pub disk_type: ::std::option::Option<String>,
        #[doc = "Configuration for VM IPs."]
        #[serde(rename = "ipConfiguration", default)]
        pub ip_configuration: ::std::option::Option<crate::schemas::WorkerPoolIpConfiguration>,
        #[doc = "The kind of the worker pool; currently only `harness` and `shuffle`\nare supported."]
        #[serde(rename = "kind", default)]
        pub kind: ::std::option::Option<String>,
        #[doc = "Machine type (e.g. \"n1-standard-1\").  If empty or unspecified, the\nservice will attempt to choose a reasonable default."]
        #[serde(rename = "machineType", default)]
        pub machine_type: ::std::option::Option<String>,
        #[doc = "Metadata to set on the Google Compute Engine VMs."]
        #[serde(rename = "metadata", default)]
        pub metadata: ::std::option::Option<::std::collections::BTreeMap<String, String>>,
        #[doc = "Network to which VMs will be assigned.  If empty or unspecified,\nthe service will use the network \"default\"."]
        #[serde(rename = "network", default)]
        pub network: ::std::option::Option<String>,
        #[doc = "The number of threads per worker harness. If empty or unspecified, the\nservice will choose a number of threads (according to the number of cores\non the selected machine type for batch, or 1 by convention for streaming)."]
        #[serde(rename = "numThreadsPerWorker", default)]
        pub num_threads_per_worker: ::std::option::Option<i32>,
        #[doc = "Number of Google Compute Engine workers in this pool needed to\nexecute the job.  If zero or unspecified, the service will\nattempt to choose a reasonable default."]
        #[serde(rename = "numWorkers", default)]
        pub num_workers: ::std::option::Option<i32>,
        #[doc = "The action to take on host maintenance, as defined by the Google\nCompute Engine API."]
        #[serde(rename = "onHostMaintenance", default)]
        pub on_host_maintenance: ::std::option::Option<String>,
        #[doc = "Packages to be installed on workers."]
        #[serde(rename = "packages", default)]
        pub packages: ::std::option::Option<Vec<crate::schemas::Package>>,
        #[doc = "Extra arguments for this worker pool."]
        #[serde(rename = "poolArgs", default)]
        pub pool_args:
            ::std::option::Option<::std::collections::BTreeMap<String, ::serde_json::Value>>,
        #[doc = "Subnetwork to which VMs will be assigned, if desired.  Expected to be of\nthe form \"regions/REGION/subnetworks/SUBNETWORK\"."]
        #[serde(rename = "subnetwork", default)]
        pub subnetwork: ::std::option::Option<String>,
        #[doc = "Settings passed through to Google Compute Engine workers when\nusing the standard Dataflow task runner.  Users should ignore\nthis field."]
        #[serde(rename = "taskrunnerSettings", default)]
        pub taskrunner_settings: ::std::option::Option<crate::schemas::TaskRunnerSettings>,
        #[doc = "Sets the policy for determining when to turndown worker pool.\nAllowed values are: `TEARDOWN_ALWAYS`, `TEARDOWN_ON_SUCCESS`, and\n`TEARDOWN_NEVER`.\n`TEARDOWN_ALWAYS` means workers are always torn down regardless of whether\nthe job succeeds. `TEARDOWN_ON_SUCCESS` means workers are torn down\nif the job succeeds. `TEARDOWN_NEVER` means the workers are never torn\ndown.\n\nIf the workers are not torn down by the service, they will\ncontinue to run and use Google Compute Engine VM resources in the\nuser's project until they are explicitly terminated by the user.\nBecause of this, Google recommends using the `TEARDOWN_ALWAYS`\npolicy except for small, manually supervised test jobs.\n\nIf unknown or unspecified, the service will attempt to choose a reasonable\ndefault."]
        #[serde(rename = "teardownPolicy", default)]
        pub teardown_policy: ::std::option::Option<crate::schemas::WorkerPoolTeardownPolicy>,
        #[doc = "Required. Docker container image that executes the Cloud Dataflow worker\nharness, residing in Google Container Registry."]
        #[serde(rename = "workerHarnessContainerImage", default)]
        pub worker_harness_container_image: ::std::option::Option<String>,
        #[doc = "Zone to run the worker pools in.  If empty or unspecified, the service\nwill attempt to choose a reasonable default."]
        #[serde(rename = "zone", default)]
        pub zone: ::std::option::Option<String>,
    }
    impl ::field_selector::FieldSelector for WorkerPool {
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
    pub struct WorkerSettings {
        #[doc = "The base URL for accessing Google Cloud APIs.\n\nWhen workers access Google Cloud APIs, they logically do so via\nrelative URLs.  If this field is specified, it supplies the base\nURL to use for resolving these relative URLs.  The normative\nalgorithm used is defined by RFC 1808, \"Relative Uniform Resource\nLocators\".\n\nIf not specified, the default value is \"http://www.googleapis.com/\""]
        #[serde(rename = "baseUrl", default)]
        pub base_url: ::std::option::Option<String>,
        #[doc = "Whether to send work progress updates to the service."]
        #[serde(rename = "reportingEnabled", default)]
        pub reporting_enabled: ::std::option::Option<bool>,
        #[doc = "The Cloud Dataflow service path relative to the root URL, for example,\n\"dataflow/v1b3/projects\"."]
        #[serde(rename = "servicePath", default)]
        pub service_path: ::std::option::Option<String>,
        #[doc = "The Shuffle service path relative to the root URL, for example,\n\"shuffle/v1beta1\"."]
        #[serde(rename = "shuffleServicePath", default)]
        pub shuffle_service_path: ::std::option::Option<String>,
        #[doc = "The prefix of the resources the system should use for temporary\nstorage.\n\nThe supported resource type is:\n\nGoogle Cloud Storage:\n\nstorage.googleapis.com/{bucket}/{object}\nbucket.storage.googleapis.com/{object}"]
        #[serde(rename = "tempStoragePrefix", default)]
        pub temp_storage_prefix: ::std::option::Option<String>,
        #[doc = "The ID of the worker running this pipeline."]
        #[serde(rename = "workerId", default)]
        pub worker_id: ::std::option::Option<String>,
    }
    impl ::field_selector::FieldSelector for WorkerSettings {
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
    pub struct WorkerShutdownNotice {
        #[doc = "The reason for the worker shutdown.\nCurrent possible values are:\n\"UNKNOWN\": shutdown reason is unknown.\n\"PREEMPTION\": shutdown reason is preemption.\nOther possible reasons may be added in the future."]
        #[serde(rename = "reason", default)]
        pub reason: ::std::option::Option<String>,
    }
    impl ::field_selector::FieldSelector for WorkerShutdownNotice {
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
        Copy,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct WorkerShutdownNoticeResponse;
    impl ::field_selector::FieldSelector for WorkerShutdownNoticeResponse {
        fn field_selector_with_ident(_ident: &str, _selector: &mut String) {}
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct WriteInstruction {
        #[doc = "The input."]
        #[serde(rename = "input", default)]
        pub input: ::std::option::Option<crate::schemas::InstructionInput>,
        #[doc = "The sink to write to."]
        #[serde(rename = "sink", default)]
        pub sink: ::std::option::Option<crate::schemas::Sink>,
    }
    impl ::field_selector::FieldSelector for WriteInstruction {
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
        #[doc = "Media download with context-dependent Content-Type"]
        Media,
        #[doc = "Responses with Content-Type of application/x-protobuf"]
        Proto,
    }
    impl Alt {
        pub fn as_str(self) -> &'static str {
            match self {
                Alt::Json => "json",
                Alt::Media => "media",
                Alt::Proto => "proto",
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
                "media" => Alt::Media,
                "proto" => Alt::Proto,
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
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum Xgafv {
        #[doc = "v1 error format"]
        _1,
        #[doc = "v2 error format"]
        _2,
    }
    impl Xgafv {
        pub fn as_str(self) -> &'static str {
            match self {
                Xgafv::_1 => "1",
                Xgafv::_2 => "2",
            }
        }
    }
    impl ::std::fmt::Display for Xgafv {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for Xgafv {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for Xgafv {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "1" => Xgafv::_1,
                "2" => Xgafv::_2,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::field_selector::FieldSelector for Xgafv {
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
    #[doc = "Actions that can be performed on the projects resource"]
    pub fn projects(&self) -> crate::resources::projects::ProjectsActions<A> {
        crate::resources::projects::ProjectsActions {
            reqwest: &self.reqwest,
            auth: &self.auth,
        }
    }
}
pub mod resources {
    pub mod projects {
        pub mod params {}
        pub struct ProjectsActions<'a, A> {
            pub(crate) reqwest: &'a reqwest::Client,
            pub(crate) auth: &'a std::sync::Mutex<A>,
        }
        impl<'a, A: yup_oauth2::GetToken> ProjectsActions<'a, A> {
            #[doc = "Deletes a snapshot."]
            pub fn delete_snapshots(
                &self,
                project_id: impl Into<String>,
            ) -> DeleteSnapshotsRequestBuilder<A> {
                DeleteSnapshotsRequestBuilder {
                    reqwest: &self.reqwest,
                    auth: &self.auth,
                    access_token: None,
                    alt: None,
                    callback: None,
                    fields: None,
                    key: None,
                    oauth_token: None,
                    pretty_print: None,
                    quota_user: None,
                    upload_protocol: None,
                    upload_type: None,
                    xgafv: None,
                    project_id: project_id.into(),
                    location: None,
                    snapshot_id: None,
                }
            }
            #[doc = "Send a worker_message to the service."]
            pub fn worker_messages(
                &self,
                request: crate::schemas::SendWorkerMessagesRequest,
                project_id: impl Into<String>,
            ) -> WorkerMessagesRequestBuilder<A> {
                WorkerMessagesRequestBuilder {
                    reqwest: &self.reqwest,
                    auth: &self.auth,
                    request,
                    access_token: None,
                    alt: None,
                    callback: None,
                    fields: None,
                    key: None,
                    oauth_token: None,
                    pretty_print: None,
                    quota_user: None,
                    upload_protocol: None,
                    upload_type: None,
                    xgafv: None,
                    project_id: project_id.into(),
                }
            }
            #[doc = "Actions that can be performed on the jobs resource"]
            pub fn jobs(&self) -> crate::resources::projects::jobs::JobsActions<A> {
                crate::resources::projects::jobs::JobsActions {
                    reqwest: &self.reqwest,
                    auth: &self.auth,
                }
            }
            #[doc = "Actions that can be performed on the locations resource"]
            pub fn locations(&self) -> crate::resources::projects::locations::LocationsActions<A> {
                crate::resources::projects::locations::LocationsActions {
                    reqwest: &self.reqwest,
                    auth: &self.auth,
                }
            }
            #[doc = "Actions that can be performed on the snapshots resource"]
            pub fn snapshots(&self) -> crate::resources::projects::snapshots::SnapshotsActions<A> {
                crate::resources::projects::snapshots::SnapshotsActions {
                    reqwest: &self.reqwest,
                    auth: &self.auth,
                }
            }
            #[doc = "Actions that can be performed on the templates resource"]
            pub fn templates(&self) -> crate::resources::projects::templates::TemplatesActions<A> {
                crate::resources::projects::templates::TemplatesActions {
                    reqwest: &self.reqwest,
                    auth: &self.auth,
                }
            }
        }
        #[derive(Debug, Clone)]
        pub struct DeleteSnapshotsRequestBuilder<'a, A> {
            pub(crate) reqwest: &'a ::reqwest::Client,
            pub(crate) auth: &'a ::std::sync::Mutex<A>,
            project_id: String,
            location: Option<String>,
            snapshot_id: Option<String>,
            access_token: Option<String>,
            alt: Option<crate::params::Alt>,
            callback: Option<String>,
            fields: Option<String>,
            key: Option<String>,
            oauth_token: Option<String>,
            pretty_print: Option<bool>,
            quota_user: Option<String>,
            upload_protocol: Option<String>,
            upload_type: Option<String>,
            xgafv: Option<crate::params::Xgafv>,
        }
        impl<'a, A: yup_oauth2::GetToken> DeleteSnapshotsRequestBuilder<'a, A> {
            #[doc = "The location that contains this snapshot."]
            pub fn location(mut self, value: impl Into<String>) -> Self {
                self.location = Some(value.into());
                self
            }
            #[doc = "The ID of the snapshot."]
            pub fn snapshot_id(mut self, value: impl Into<String>) -> Self {
                self.snapshot_id = Some(value.into());
                self
            }
            #[doc = "OAuth access token."]
            pub fn access_token(mut self, value: impl Into<String>) -> Self {
                self.access_token = Some(value.into());
                self
            }
            #[doc = "JSONP"]
            pub fn callback(mut self, value: impl Into<String>) -> Self {
                self.callback = Some(value.into());
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
            #[doc = "Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters."]
            pub fn quota_user(mut self, value: impl Into<String>) -> Self {
                self.quota_user = Some(value.into());
                self
            }
            #[doc = "Upload protocol for media (e.g. \"raw\", \"multipart\")."]
            pub fn upload_protocol(mut self, value: impl Into<String>) -> Self {
                self.upload_protocol = Some(value.into());
                self
            }
            #[doc = "Legacy upload protocol for media (e.g. \"media\", \"multipart\")."]
            pub fn upload_type(mut self, value: impl Into<String>) -> Self {
                self.upload_type = Some(value.into());
                self
            }
            #[doc = "V1 error format."]
            pub fn xgafv(mut self, value: crate::params::Xgafv) -> Self {
                self.xgafv = Some(value);
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
            ) -> Result<crate::schemas::DeleteSnapshotResponse, Box<dyn ::std::error::Error>>
            {
                self.execute_fields::<_, &str>(None)
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub fn execute_debug(
                self,
            ) -> Result<crate::schemas::DeleteSnapshotResponse, Box<dyn ::std::error::Error>>
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
                let mut output = "https://dataflow.googleapis.com/".to_owned();
                output.push_str("v1b3/projects/");
                {
                    let var_as_str = &self.project_id;
                    output.extend(::percent_encoding::utf8_percent_encode(
                        &var_as_str,
                        crate::SIMPLE,
                    ));
                }
                output.push_str("/snapshots");
                output
            }
            fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
                let req = self.reqwest.request(::reqwest::Method::DELETE, path);
                let req = req.query(&[("location", &self.location)]);
                let req = req.query(&[("snapshotId", &self.snapshot_id)]);
                let req = req.query(&[("access_token", &self.access_token)]);
                let req = req.query(&[("alt", &self.alt)]);
                let req = req.query(&[("callback", &self.callback)]);
                let req = req.query(&[("fields", &self.fields)]);
                let req = req.query(&[("key", &self.key)]);
                let req = req.query(&[("oauth_token", &self.oauth_token)]);
                let req = req.query(&[("prettyPrint", &self.pretty_print)]);
                let req = req.query(&[("quotaUser", &self.quota_user)]);
                let req = req.query(&[("upload_protocol", &self.upload_protocol)]);
                let req = req.query(&[("uploadType", &self.upload_type)]);
                let req = req.query(&[("$.xgafv", &self.xgafv)]);
                let mut auth = self.auth.lock().unwrap();
                let fut = auth.token(vec!["https://www.googleapis.com/auth/cloud-platform"]);
                let mut runtime = ::tokio::runtime::Runtime::new().unwrap();
                let token = runtime.block_on(fut).unwrap().access_token;
                let req = req.bearer_auth(&token);
                req
            }
        }
        #[derive(Debug, Clone)]
        pub struct WorkerMessagesRequestBuilder<'a, A> {
            pub(crate) reqwest: &'a ::reqwest::Client,
            pub(crate) auth: &'a ::std::sync::Mutex<A>,
            request: crate::schemas::SendWorkerMessagesRequest,
            project_id: String,
            access_token: Option<String>,
            alt: Option<crate::params::Alt>,
            callback: Option<String>,
            fields: Option<String>,
            key: Option<String>,
            oauth_token: Option<String>,
            pretty_print: Option<bool>,
            quota_user: Option<String>,
            upload_protocol: Option<String>,
            upload_type: Option<String>,
            xgafv: Option<crate::params::Xgafv>,
        }
        impl<'a, A: yup_oauth2::GetToken> WorkerMessagesRequestBuilder<'a, A> {
            #[doc = "OAuth access token."]
            pub fn access_token(mut self, value: impl Into<String>) -> Self {
                self.access_token = Some(value.into());
                self
            }
            #[doc = "JSONP"]
            pub fn callback(mut self, value: impl Into<String>) -> Self {
                self.callback = Some(value.into());
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
            #[doc = "Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters."]
            pub fn quota_user(mut self, value: impl Into<String>) -> Self {
                self.quota_user = Some(value.into());
                self
            }
            #[doc = "Upload protocol for media (e.g. \"raw\", \"multipart\")."]
            pub fn upload_protocol(mut self, value: impl Into<String>) -> Self {
                self.upload_protocol = Some(value.into());
                self
            }
            #[doc = "Legacy upload protocol for media (e.g. \"media\", \"multipart\")."]
            pub fn upload_type(mut self, value: impl Into<String>) -> Self {
                self.upload_type = Some(value.into());
                self
            }
            #[doc = "V1 error format."]
            pub fn xgafv(mut self, value: crate::params::Xgafv) -> Self {
                self.xgafv = Some(value);
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
            ) -> Result<crate::schemas::SendWorkerMessagesResponse, Box<dyn ::std::error::Error>>
            {
                self.execute_fields::<_, &str>(None)
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub fn execute_debug(
                self,
            ) -> Result<crate::schemas::SendWorkerMessagesResponse, Box<dyn ::std::error::Error>>
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
                let mut output = "https://dataflow.googleapis.com/".to_owned();
                output.push_str("v1b3/projects/");
                {
                    let var_as_str = &self.project_id;
                    output.extend(::percent_encoding::utf8_percent_encode(
                        &var_as_str,
                        crate::SIMPLE,
                    ));
                }
                output.push_str("/WorkerMessages");
                output
            }
            fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
                let req = self.reqwest.request(::reqwest::Method::POST, path);
                let req = req.query(&[("access_token", &self.access_token)]);
                let req = req.query(&[("alt", &self.alt)]);
                let req = req.query(&[("callback", &self.callback)]);
                let req = req.query(&[("fields", &self.fields)]);
                let req = req.query(&[("key", &self.key)]);
                let req = req.query(&[("oauth_token", &self.oauth_token)]);
                let req = req.query(&[("prettyPrint", &self.pretty_print)]);
                let req = req.query(&[("quotaUser", &self.quota_user)]);
                let req = req.query(&[("upload_protocol", &self.upload_protocol)]);
                let req = req.query(&[("uploadType", &self.upload_type)]);
                let req = req.query(&[("$.xgafv", &self.xgafv)]);
                let mut auth = self.auth.lock().unwrap();
                let fut = auth.token(vec!["https://www.googleapis.com/auth/cloud-platform"]);
                let mut runtime = ::tokio::runtime::Runtime::new().unwrap();
                let token = runtime.block_on(fut).unwrap().access_token;
                let req = req.bearer_auth(&token);
                req
            }
        }
        pub mod jobs {
            pub mod params {
                #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
                pub enum AggregatedFilter {
                    Active,
                    All,
                    Terminated,
                    Unknown,
                }
                impl AggregatedFilter {
                    pub fn as_str(self) -> &'static str {
                        match self {
                            AggregatedFilter::Active => "ACTIVE",
                            AggregatedFilter::All => "ALL",
                            AggregatedFilter::Terminated => "TERMINATED",
                            AggregatedFilter::Unknown => "UNKNOWN",
                        }
                    }
                }
                impl ::std::fmt::Display for AggregatedFilter {
                    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                        f.write_str(self.as_str())
                    }
                }
                impl ::serde::Serialize for AggregatedFilter {
                    fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
                    where
                        S: ::serde::ser::Serializer,
                    {
                        serializer.serialize_str(self.as_str())
                    }
                }
                impl<'de> ::serde::Deserialize<'de> for AggregatedFilter {
                    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
                    where
                        D: ::serde::de::Deserializer<'de>,
                    {
                        let value: &'de str = <&str>::deserialize(deserializer)?;
                        Ok(match value {
                            "ACTIVE" => AggregatedFilter::Active,
                            "ALL" => AggregatedFilter::All,
                            "TERMINATED" => AggregatedFilter::Terminated,
                            "UNKNOWN" => AggregatedFilter::Unknown,
                            _ => {
                                return Err(::serde::de::Error::custom(format!(
                                    "invalid enum for #name: {}",
                                    value
                                )))
                            }
                        })
                    }
                }
                impl ::field_selector::FieldSelector for AggregatedFilter {
                    fn field_selector_with_ident(ident: &str, selector: &mut String) {
                        match selector.chars().rev().nth(0) {
                            Some(',') | None => {}
                            _ => selector.push_str(","),
                        }
                        selector.push_str(ident);
                    }
                }
                #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
                pub enum AggregatedView {
                    JobViewAll,
                    JobViewDescription,
                    JobViewSummary,
                    JobViewUnknown,
                }
                impl AggregatedView {
                    pub fn as_str(self) -> &'static str {
                        match self {
                            AggregatedView::JobViewAll => "JOB_VIEW_ALL",
                            AggregatedView::JobViewDescription => "JOB_VIEW_DESCRIPTION",
                            AggregatedView::JobViewSummary => "JOB_VIEW_SUMMARY",
                            AggregatedView::JobViewUnknown => "JOB_VIEW_UNKNOWN",
                        }
                    }
                }
                impl ::std::fmt::Display for AggregatedView {
                    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                        f.write_str(self.as_str())
                    }
                }
                impl ::serde::Serialize for AggregatedView {
                    fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
                    where
                        S: ::serde::ser::Serializer,
                    {
                        serializer.serialize_str(self.as_str())
                    }
                }
                impl<'de> ::serde::Deserialize<'de> for AggregatedView {
                    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
                    where
                        D: ::serde::de::Deserializer<'de>,
                    {
                        let value: &'de str = <&str>::deserialize(deserializer)?;
                        Ok(match value {
                            "JOB_VIEW_ALL" => AggregatedView::JobViewAll,
                            "JOB_VIEW_DESCRIPTION" => AggregatedView::JobViewDescription,
                            "JOB_VIEW_SUMMARY" => AggregatedView::JobViewSummary,
                            "JOB_VIEW_UNKNOWN" => AggregatedView::JobViewUnknown,
                            _ => {
                                return Err(::serde::de::Error::custom(format!(
                                    "invalid enum for #name: {}",
                                    value
                                )))
                            }
                        })
                    }
                }
                impl ::field_selector::FieldSelector for AggregatedView {
                    fn field_selector_with_ident(ident: &str, selector: &mut String) {
                        match selector.chars().rev().nth(0) {
                            Some(',') | None => {}
                            _ => selector.push_str(","),
                        }
                        selector.push_str(ident);
                    }
                }
                #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
                pub enum CreateView {
                    JobViewAll,
                    JobViewDescription,
                    JobViewSummary,
                    JobViewUnknown,
                }
                impl CreateView {
                    pub fn as_str(self) -> &'static str {
                        match self {
                            CreateView::JobViewAll => "JOB_VIEW_ALL",
                            CreateView::JobViewDescription => "JOB_VIEW_DESCRIPTION",
                            CreateView::JobViewSummary => "JOB_VIEW_SUMMARY",
                            CreateView::JobViewUnknown => "JOB_VIEW_UNKNOWN",
                        }
                    }
                }
                impl ::std::fmt::Display for CreateView {
                    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                        f.write_str(self.as_str())
                    }
                }
                impl ::serde::Serialize for CreateView {
                    fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
                    where
                        S: ::serde::ser::Serializer,
                    {
                        serializer.serialize_str(self.as_str())
                    }
                }
                impl<'de> ::serde::Deserialize<'de> for CreateView {
                    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
                    where
                        D: ::serde::de::Deserializer<'de>,
                    {
                        let value: &'de str = <&str>::deserialize(deserializer)?;
                        Ok(match value {
                            "JOB_VIEW_ALL" => CreateView::JobViewAll,
                            "JOB_VIEW_DESCRIPTION" => CreateView::JobViewDescription,
                            "JOB_VIEW_SUMMARY" => CreateView::JobViewSummary,
                            "JOB_VIEW_UNKNOWN" => CreateView::JobViewUnknown,
                            _ => {
                                return Err(::serde::de::Error::custom(format!(
                                    "invalid enum for #name: {}",
                                    value
                                )))
                            }
                        })
                    }
                }
                impl ::field_selector::FieldSelector for CreateView {
                    fn field_selector_with_ident(ident: &str, selector: &mut String) {
                        match selector.chars().rev().nth(0) {
                            Some(',') | None => {}
                            _ => selector.push_str(","),
                        }
                        selector.push_str(ident);
                    }
                }
                #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
                pub enum GetView {
                    JobViewAll,
                    JobViewDescription,
                    JobViewSummary,
                    JobViewUnknown,
                }
                impl GetView {
                    pub fn as_str(self) -> &'static str {
                        match self {
                            GetView::JobViewAll => "JOB_VIEW_ALL",
                            GetView::JobViewDescription => "JOB_VIEW_DESCRIPTION",
                            GetView::JobViewSummary => "JOB_VIEW_SUMMARY",
                            GetView::JobViewUnknown => "JOB_VIEW_UNKNOWN",
                        }
                    }
                }
                impl ::std::fmt::Display for GetView {
                    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                        f.write_str(self.as_str())
                    }
                }
                impl ::serde::Serialize for GetView {
                    fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
                    where
                        S: ::serde::ser::Serializer,
                    {
                        serializer.serialize_str(self.as_str())
                    }
                }
                impl<'de> ::serde::Deserialize<'de> for GetView {
                    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
                    where
                        D: ::serde::de::Deserializer<'de>,
                    {
                        let value: &'de str = <&str>::deserialize(deserializer)?;
                        Ok(match value {
                            "JOB_VIEW_ALL" => GetView::JobViewAll,
                            "JOB_VIEW_DESCRIPTION" => GetView::JobViewDescription,
                            "JOB_VIEW_SUMMARY" => GetView::JobViewSummary,
                            "JOB_VIEW_UNKNOWN" => GetView::JobViewUnknown,
                            _ => {
                                return Err(::serde::de::Error::custom(format!(
                                    "invalid enum for #name: {}",
                                    value
                                )))
                            }
                        })
                    }
                }
                impl ::field_selector::FieldSelector for GetView {
                    fn field_selector_with_ident(ident: &str, selector: &mut String) {
                        match selector.chars().rev().nth(0) {
                            Some(',') | None => {}
                            _ => selector.push_str(","),
                        }
                        selector.push_str(ident);
                    }
                }
                #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
                pub enum ListFilter {
                    Active,
                    All,
                    Terminated,
                    Unknown,
                }
                impl ListFilter {
                    pub fn as_str(self) -> &'static str {
                        match self {
                            ListFilter::Active => "ACTIVE",
                            ListFilter::All => "ALL",
                            ListFilter::Terminated => "TERMINATED",
                            ListFilter::Unknown => "UNKNOWN",
                        }
                    }
                }
                impl ::std::fmt::Display for ListFilter {
                    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                        f.write_str(self.as_str())
                    }
                }
                impl ::serde::Serialize for ListFilter {
                    fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
                    where
                        S: ::serde::ser::Serializer,
                    {
                        serializer.serialize_str(self.as_str())
                    }
                }
                impl<'de> ::serde::Deserialize<'de> for ListFilter {
                    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
                    where
                        D: ::serde::de::Deserializer<'de>,
                    {
                        let value: &'de str = <&str>::deserialize(deserializer)?;
                        Ok(match value {
                            "ACTIVE" => ListFilter::Active,
                            "ALL" => ListFilter::All,
                            "TERMINATED" => ListFilter::Terminated,
                            "UNKNOWN" => ListFilter::Unknown,
                            _ => {
                                return Err(::serde::de::Error::custom(format!(
                                    "invalid enum for #name: {}",
                                    value
                                )))
                            }
                        })
                    }
                }
                impl ::field_selector::FieldSelector for ListFilter {
                    fn field_selector_with_ident(ident: &str, selector: &mut String) {
                        match selector.chars().rev().nth(0) {
                            Some(',') | None => {}
                            _ => selector.push_str(","),
                        }
                        selector.push_str(ident);
                    }
                }
                #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
                pub enum ListView {
                    JobViewAll,
                    JobViewDescription,
                    JobViewSummary,
                    JobViewUnknown,
                }
                impl ListView {
                    pub fn as_str(self) -> &'static str {
                        match self {
                            ListView::JobViewAll => "JOB_VIEW_ALL",
                            ListView::JobViewDescription => "JOB_VIEW_DESCRIPTION",
                            ListView::JobViewSummary => "JOB_VIEW_SUMMARY",
                            ListView::JobViewUnknown => "JOB_VIEW_UNKNOWN",
                        }
                    }
                }
                impl ::std::fmt::Display for ListView {
                    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                        f.write_str(self.as_str())
                    }
                }
                impl ::serde::Serialize for ListView {
                    fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
                    where
                        S: ::serde::ser::Serializer,
                    {
                        serializer.serialize_str(self.as_str())
                    }
                }
                impl<'de> ::serde::Deserialize<'de> for ListView {
                    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
                    where
                        D: ::serde::de::Deserializer<'de>,
                    {
                        let value: &'de str = <&str>::deserialize(deserializer)?;
                        Ok(match value {
                            "JOB_VIEW_ALL" => ListView::JobViewAll,
                            "JOB_VIEW_DESCRIPTION" => ListView::JobViewDescription,
                            "JOB_VIEW_SUMMARY" => ListView::JobViewSummary,
                            "JOB_VIEW_UNKNOWN" => ListView::JobViewUnknown,
                            _ => {
                                return Err(::serde::de::Error::custom(format!(
                                    "invalid enum for #name: {}",
                                    value
                                )))
                            }
                        })
                    }
                }
                impl ::field_selector::FieldSelector for ListView {
                    fn field_selector_with_ident(ident: &str, selector: &mut String) {
                        match selector.chars().rev().nth(0) {
                            Some(',') | None => {}
                            _ => selector.push_str(","),
                        }
                        selector.push_str(ident);
                    }
                }
            }
            pub struct JobsActions<'a, A> {
                pub(crate) reqwest: &'a reqwest::Client,
                pub(crate) auth: &'a std::sync::Mutex<A>,
            }
            impl<'a, A: yup_oauth2::GetToken> JobsActions<'a, A> {
                #[doc = "List the jobs of a project across all regions."]
                pub fn aggregated(
                    &self,
                    project_id: impl Into<String>,
                ) -> AggregatedRequestBuilder<A> {
                    AggregatedRequestBuilder {
                        reqwest: &self.reqwest,
                        auth: &self.auth,
                        access_token: None,
                        alt: None,
                        callback: None,
                        fields: None,
                        key: None,
                        oauth_token: None,
                        pretty_print: None,
                        quota_user: None,
                        upload_protocol: None,
                        upload_type: None,
                        xgafv: None,
                        project_id: project_id.into(),
                        filter: None,
                        location: None,
                        page_size: None,
                        page_token: None,
                        view: None,
                    }
                }
                #[doc = "Creates a Cloud Dataflow job.\n\nTo create a job, we recommend using `projects.locations.jobs.create` with a\n[regional endpoint]\n(https://cloud.google.com/dataflow/docs/concepts/regional-endpoints). Using\n`projects.jobs.create` is not recommended, as your job will always start\nin `us-central1`."]
                pub fn create(
                    &self,
                    request: crate::schemas::Job,
                    project_id: impl Into<String>,
                ) -> CreateRequestBuilder<A> {
                    CreateRequestBuilder {
                        reqwest: &self.reqwest,
                        auth: &self.auth,
                        request,
                        access_token: None,
                        alt: None,
                        callback: None,
                        fields: None,
                        key: None,
                        oauth_token: None,
                        pretty_print: None,
                        quota_user: None,
                        upload_protocol: None,
                        upload_type: None,
                        xgafv: None,
                        project_id: project_id.into(),
                        location: None,
                        replace_job_id: None,
                        view: None,
                    }
                }
                #[doc = "Gets the state of the specified Cloud Dataflow job.\n\nTo get the state of a job, we recommend using `projects.locations.jobs.get`\nwith a [regional endpoint]\n(https://cloud.google.com/dataflow/docs/concepts/regional-endpoints). Using\n`projects.jobs.get` is not recommended, as you can only get the state of\njobs that are running in `us-central1`."]
                pub fn get(
                    &self,
                    project_id: impl Into<String>,
                    job_id: impl Into<String>,
                ) -> GetRequestBuilder<A> {
                    GetRequestBuilder {
                        reqwest: &self.reqwest,
                        auth: &self.auth,
                        access_token: None,
                        alt: None,
                        callback: None,
                        fields: None,
                        key: None,
                        oauth_token: None,
                        pretty_print: None,
                        quota_user: None,
                        upload_protocol: None,
                        upload_type: None,
                        xgafv: None,
                        project_id: project_id.into(),
                        job_id: job_id.into(),
                        location: None,
                        view: None,
                    }
                }
                #[doc = "Request the job status.\n\nTo request the status of a job, we recommend using\n`projects.locations.jobs.getMetrics` with a [regional endpoint]\n(https://cloud.google.com/dataflow/docs/concepts/regional-endpoints). Using\n`projects.jobs.getMetrics` is not recommended, as you can only request the\nstatus of jobs that are running in `us-central1`."]
                pub fn get_metrics(
                    &self,
                    project_id: impl Into<String>,
                    job_id: impl Into<String>,
                ) -> GetMetricsRequestBuilder<A> {
                    GetMetricsRequestBuilder {
                        reqwest: &self.reqwest,
                        auth: &self.auth,
                        access_token: None,
                        alt: None,
                        callback: None,
                        fields: None,
                        key: None,
                        oauth_token: None,
                        pretty_print: None,
                        quota_user: None,
                        upload_protocol: None,
                        upload_type: None,
                        xgafv: None,
                        project_id: project_id.into(),
                        job_id: job_id.into(),
                        location: None,
                        start_time: None,
                    }
                }
                #[doc = "List the jobs of a project.\n\nTo list the jobs of a project in a region, we recommend using\n`projects.locations.jobs.get` with a [regional endpoint]\n(https://cloud.google.com/dataflow/docs/concepts/regional-endpoints). To\nlist the all jobs across all regions, use `projects.jobs.aggregated`. Using\n`projects.jobs.list` is not recommended, as you can only get the list of\njobs that are running in `us-central1`."]
                pub fn list(&self, project_id: impl Into<String>) -> ListRequestBuilder<A> {
                    ListRequestBuilder {
                        reqwest: &self.reqwest,
                        auth: &self.auth,
                        access_token: None,
                        alt: None,
                        callback: None,
                        fields: None,
                        key: None,
                        oauth_token: None,
                        pretty_print: None,
                        quota_user: None,
                        upload_protocol: None,
                        upload_type: None,
                        xgafv: None,
                        project_id: project_id.into(),
                        filter: None,
                        location: None,
                        page_size: None,
                        page_token: None,
                        view: None,
                    }
                }
                #[doc = "Snapshot the state of a streaming job."]
                pub fn snapshot(
                    &self,
                    request: crate::schemas::SnapshotJobRequest,
                    project_id: impl Into<String>,
                    job_id: impl Into<String>,
                ) -> SnapshotRequestBuilder<A> {
                    SnapshotRequestBuilder {
                        reqwest: &self.reqwest,
                        auth: &self.auth,
                        request,
                        access_token: None,
                        alt: None,
                        callback: None,
                        fields: None,
                        key: None,
                        oauth_token: None,
                        pretty_print: None,
                        quota_user: None,
                        upload_protocol: None,
                        upload_type: None,
                        xgafv: None,
                        project_id: project_id.into(),
                        job_id: job_id.into(),
                    }
                }
                #[doc = "Updates the state of an existing Cloud Dataflow job.\n\nTo update the state of an existing job, we recommend using\n`projects.locations.jobs.update` with a [regional endpoint]\n(https://cloud.google.com/dataflow/docs/concepts/regional-endpoints). Using\n`projects.jobs.update` is not recommended, as you can only update the state\nof jobs that are running in `us-central1`."]
                pub fn update(
                    &self,
                    request: crate::schemas::Job,
                    project_id: impl Into<String>,
                    job_id: impl Into<String>,
                ) -> UpdateRequestBuilder<A> {
                    UpdateRequestBuilder {
                        reqwest: &self.reqwest,
                        auth: &self.auth,
                        request,
                        access_token: None,
                        alt: None,
                        callback: None,
                        fields: None,
                        key: None,
                        oauth_token: None,
                        pretty_print: None,
                        quota_user: None,
                        upload_protocol: None,
                        upload_type: None,
                        xgafv: None,
                        project_id: project_id.into(),
                        job_id: job_id.into(),
                        location: None,
                    }
                }
                #[doc = "Actions that can be performed on the debug resource"]
                pub fn debug(&self) -> crate::resources::projects::jobs::debug::DebugActions<A> {
                    crate::resources::projects::jobs::debug::DebugActions {
                        reqwest: &self.reqwest,
                        auth: &self.auth,
                    }
                }
                #[doc = "Actions that can be performed on the messages resource"]
                pub fn messages(
                    &self,
                ) -> crate::resources::projects::jobs::messages::MessagesActions<A>
                {
                    crate::resources::projects::jobs::messages::MessagesActions {
                        reqwest: &self.reqwest,
                        auth: &self.auth,
                    }
                }
                #[doc = "Actions that can be performed on the work_items resource"]
                pub fn work_items(
                    &self,
                ) -> crate::resources::projects::jobs::work_items::WorkItemsActions<A>
                {
                    crate::resources::projects::jobs::work_items::WorkItemsActions {
                        reqwest: &self.reqwest,
                        auth: &self.auth,
                    }
                }
            }
            #[derive(Debug, Clone)]
            pub struct AggregatedRequestBuilder<'a, A> {
                pub(crate) reqwest: &'a ::reqwest::Client,
                pub(crate) auth: &'a ::std::sync::Mutex<A>,
                project_id: String,
                filter: Option<crate::resources::projects::jobs::params::AggregatedFilter>,
                location: Option<String>,
                page_size: Option<i32>,
                page_token: Option<String>,
                view: Option<crate::resources::projects::jobs::params::AggregatedView>,
                access_token: Option<String>,
                alt: Option<crate::params::Alt>,
                callback: Option<String>,
                fields: Option<String>,
                key: Option<String>,
                oauth_token: Option<String>,
                pretty_print: Option<bool>,
                quota_user: Option<String>,
                upload_protocol: Option<String>,
                upload_type: Option<String>,
                xgafv: Option<crate::params::Xgafv>,
            }
            impl<'a, A: yup_oauth2::GetToken> AggregatedRequestBuilder<'a, A> {
                #[doc = "The kind of filter to use."]
                pub fn filter(
                    mut self,
                    value: crate::resources::projects::jobs::params::AggregatedFilter,
                ) -> Self {
                    self.filter = Some(value);
                    self
                }
                #[doc = "The [regional endpoint]\n(https://cloud.google.com/dataflow/docs/concepts/regional-endpoints) that\ncontains this job."]
                pub fn location(mut self, value: impl Into<String>) -> Self {
                    self.location = Some(value.into());
                    self
                }
                #[doc = "If there are many jobs, limit response to at most this many.\nThe actual number of jobs returned will be the lesser of max_responses\nand an unspecified server-defined limit."]
                pub fn page_size(mut self, value: i32) -> Self {
                    self.page_size = Some(value);
                    self
                }
                #[doc = "Set this to the 'next_page_token' field of a previous response\nto request additional results in a long list."]
                pub fn page_token(mut self, value: impl Into<String>) -> Self {
                    self.page_token = Some(value.into());
                    self
                }
                #[doc = "Level of information requested in response. Default is `JOB_VIEW_SUMMARY`."]
                pub fn view(
                    mut self,
                    value: crate::resources::projects::jobs::params::AggregatedView,
                ) -> Self {
                    self.view = Some(value);
                    self
                }
                #[doc = "OAuth access token."]
                pub fn access_token(mut self, value: impl Into<String>) -> Self {
                    self.access_token = Some(value.into());
                    self
                }
                #[doc = "JSONP"]
                pub fn callback(mut self, value: impl Into<String>) -> Self {
                    self.callback = Some(value.into());
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
                #[doc = "Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters."]
                pub fn quota_user(mut self, value: impl Into<String>) -> Self {
                    self.quota_user = Some(value.into());
                    self
                }
                #[doc = "Upload protocol for media (e.g. \"raw\", \"multipart\")."]
                pub fn upload_protocol(mut self, value: impl Into<String>) -> Self {
                    self.upload_protocol = Some(value.into());
                    self
                }
                #[doc = "Legacy upload protocol for media (e.g. \"media\", \"multipart\")."]
                pub fn upload_type(mut self, value: impl Into<String>) -> Self {
                    self.upload_type = Some(value.into());
                    self
                }
                #[doc = "V1 error format."]
                pub fn xgafv(mut self, value: crate::params::Xgafv) -> Self {
                    self.xgafv = Some(value);
                    self
                }
                #[doc = r" Return an iterator that iterates over all `#prop_ident`. The"]
                #[doc = r" items yielded by the iterator are chosen by the caller of this"]
                #[doc = r" method and must implement `Deserialize` and `FieldSelector`. The"]
                #[doc = r" populated fields in the yielded items will be determined by the"]
                #[doc = r" `FieldSelector` implementation."]
                pub fn iter_failed_location<T>(mut self) -> crate::iter::PageItemIter<Self, T>
                where
                    T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
                {
                    let mut fields = concat!("nextPageToken,", "failedLocation").to_owned();
                    let items_fields = T::field_selector();
                    if !items_fields.is_empty() {
                        fields.push_str("(");
                        fields.push_str(&items_fields);
                        fields.push_str(")");
                    }
                    self.fields = Some(fields);
                    crate::iter::PageItemIter::new(self, "failedLocation")
                }
                #[doc = r" Return an iterator that iterates over all `#prop_ident`. The"]
                #[doc = r" items yielded by the iterator are `#items_type`. The populated"]
                #[doc = r" fields in `#items_type` will be the default fields populated by"]
                #[doc = r" the server."]
                pub fn iter_failed_location_standard(
                    mut self,
                ) -> crate::iter::PageItemIter<Self, crate::schemas::FailedLocation>
                {
                    self.fields = Some(concat!("nextPageToken,", "failedLocation").to_owned());
                    crate::iter::PageItemIter::new(self, "failedLocation")
                }
                #[doc = r" Return an iterator that iterates over all `#prop_ident`. The"]
                #[doc = r" items yielded by the iterator are `#items_type`. The populated"]
                #[doc = r" fields in `#items_type` will be all fields available. This should"]
                #[doc = r" primarily be used during developement and debugging as fetching"]
                #[doc = r" all fields can be expensive both in bandwidth and server"]
                #[doc = r" resources."]
                pub fn iter_failed_location_debug(
                    mut self,
                ) -> crate::iter::PageItemIter<Self, crate::schemas::FailedLocation>
                {
                    self.fields =
                        Some(concat!("nextPageToken,", "failedLocation", "(*)").to_owned());
                    crate::iter::PageItemIter::new(self, "failedLocation")
                }
                #[doc = r" Return an iterator that iterates over all `#prop_ident`. The"]
                #[doc = r" items yielded by the iterator are chosen by the caller of this"]
                #[doc = r" method and must implement `Deserialize` and `FieldSelector`. The"]
                #[doc = r" populated fields in the yielded items will be determined by the"]
                #[doc = r" `FieldSelector` implementation."]
                pub fn iter_jobs<T>(mut self) -> crate::iter::PageItemIter<Self, T>
                where
                    T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
                {
                    let mut fields = concat!("nextPageToken,", "jobs").to_owned();
                    let items_fields = T::field_selector();
                    if !items_fields.is_empty() {
                        fields.push_str("(");
                        fields.push_str(&items_fields);
                        fields.push_str(")");
                    }
                    self.fields = Some(fields);
                    crate::iter::PageItemIter::new(self, "jobs")
                }
                #[doc = r" Return an iterator that iterates over all `#prop_ident`. The"]
                #[doc = r" items yielded by the iterator are `#items_type`. The populated"]
                #[doc = r" fields in `#items_type` will be the default fields populated by"]
                #[doc = r" the server."]
                pub fn iter_jobs_standard(
                    mut self,
                ) -> crate::iter::PageItemIter<Self, crate::schemas::Job> {
                    self.fields = Some(concat!("nextPageToken,", "jobs").to_owned());
                    crate::iter::PageItemIter::new(self, "jobs")
                }
                #[doc = r" Return an iterator that iterates over all `#prop_ident`. The"]
                #[doc = r" items yielded by the iterator are `#items_type`. The populated"]
                #[doc = r" fields in `#items_type` will be all fields available. This should"]
                #[doc = r" primarily be used during developement and debugging as fetching"]
                #[doc = r" all fields can be expensive both in bandwidth and server"]
                #[doc = r" resources."]
                pub fn iter_jobs_debug(
                    mut self,
                ) -> crate::iter::PageItemIter<Self, crate::schemas::Job> {
                    self.fields = Some(concat!("nextPageToken,", "jobs", "(*)").to_owned());
                    crate::iter::PageItemIter::new(self, "jobs")
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
                ) -> crate::iter::PageIter<Self, crate::schemas::ListJobsResponse> {
                    crate::iter::PageIter::new(self)
                }
                pub fn iter_debug(
                    mut self,
                ) -> crate::iter::PageIter<Self, crate::schemas::ListJobsResponse> {
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
                ) -> Result<crate::schemas::ListJobsResponse, Box<dyn ::std::error::Error>>
                {
                    self.execute_fields::<_, &str>(None)
                }
                #[doc = r" Execute the given operation. This will provide a `fields`"]
                #[doc = r" selector of `*`. This will include every attribute of the"]
                #[doc = r" response resource and should be limited to use during"]
                #[doc = r" development or debugging."]
                pub fn execute_debug(
                    self,
                ) -> Result<crate::schemas::ListJobsResponse, Box<dyn ::std::error::Error>>
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
                    let mut output = "https://dataflow.googleapis.com/".to_owned();
                    output.push_str("v1b3/projects/");
                    {
                        let var_as_str = &self.project_id;
                        output.extend(::percent_encoding::utf8_percent_encode(
                            &var_as_str,
                            crate::SIMPLE,
                        ));
                    }
                    output.push_str("/jobs:aggregated");
                    output
                }
                fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
                    let req = self.reqwest.request(::reqwest::Method::GET, path);
                    let req = req.query(&[("filter", &self.filter)]);
                    let req = req.query(&[("location", &self.location)]);
                    let req = req.query(&[("pageSize", &self.page_size)]);
                    let req = req.query(&[("pageToken", &self.page_token)]);
                    let req = req.query(&[("view", &self.view)]);
                    let req = req.query(&[("access_token", &self.access_token)]);
                    let req = req.query(&[("alt", &self.alt)]);
                    let req = req.query(&[("callback", &self.callback)]);
                    let req = req.query(&[("fields", &self.fields)]);
                    let req = req.query(&[("key", &self.key)]);
                    let req = req.query(&[("oauth_token", &self.oauth_token)]);
                    let req = req.query(&[("prettyPrint", &self.pretty_print)]);
                    let req = req.query(&[("quotaUser", &self.quota_user)]);
                    let req = req.query(&[("upload_protocol", &self.upload_protocol)]);
                    let req = req.query(&[("uploadType", &self.upload_type)]);
                    let req = req.query(&[("$.xgafv", &self.xgafv)]);
                    let mut auth = self.auth.lock().unwrap();
                    let fut = auth.token(vec!["https://www.googleapis.com/auth/compute.readonly"]);
                    let mut runtime = ::tokio::runtime::Runtime::new().unwrap();
                    let token = runtime.block_on(fut).unwrap().access_token;
                    let req = req.bearer_auth(&token);
                    req
                }
            }
            impl<'a, A: yup_oauth2::GetToken> crate::iter::IterableMethod for AggregatedRequestBuilder<'a, A> {
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
            pub struct CreateRequestBuilder<'a, A> {
                pub(crate) reqwest: &'a ::reqwest::Client,
                pub(crate) auth: &'a ::std::sync::Mutex<A>,
                request: crate::schemas::Job,
                project_id: String,
                location: Option<String>,
                replace_job_id: Option<String>,
                view: Option<crate::resources::projects::jobs::params::CreateView>,
                access_token: Option<String>,
                alt: Option<crate::params::Alt>,
                callback: Option<String>,
                fields: Option<String>,
                key: Option<String>,
                oauth_token: Option<String>,
                pretty_print: Option<bool>,
                quota_user: Option<String>,
                upload_protocol: Option<String>,
                upload_type: Option<String>,
                xgafv: Option<crate::params::Xgafv>,
            }
            impl<'a, A: yup_oauth2::GetToken> CreateRequestBuilder<'a, A> {
                #[doc = "The [regional endpoint]\n(https://cloud.google.com/dataflow/docs/concepts/regional-endpoints) that\ncontains this job."]
                pub fn location(mut self, value: impl Into<String>) -> Self {
                    self.location = Some(value.into());
                    self
                }
                #[doc = "Deprecated. This field is now in the Job message."]
                pub fn replace_job_id(mut self, value: impl Into<String>) -> Self {
                    self.replace_job_id = Some(value.into());
                    self
                }
                #[doc = "The level of information requested in response."]
                pub fn view(
                    mut self,
                    value: crate::resources::projects::jobs::params::CreateView,
                ) -> Self {
                    self.view = Some(value);
                    self
                }
                #[doc = "OAuth access token."]
                pub fn access_token(mut self, value: impl Into<String>) -> Self {
                    self.access_token = Some(value.into());
                    self
                }
                #[doc = "JSONP"]
                pub fn callback(mut self, value: impl Into<String>) -> Self {
                    self.callback = Some(value.into());
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
                #[doc = "Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters."]
                pub fn quota_user(mut self, value: impl Into<String>) -> Self {
                    self.quota_user = Some(value.into());
                    self
                }
                #[doc = "Upload protocol for media (e.g. \"raw\", \"multipart\")."]
                pub fn upload_protocol(mut self, value: impl Into<String>) -> Self {
                    self.upload_protocol = Some(value.into());
                    self
                }
                #[doc = "Legacy upload protocol for media (e.g. \"media\", \"multipart\")."]
                pub fn upload_type(mut self, value: impl Into<String>) -> Self {
                    self.upload_type = Some(value.into());
                    self
                }
                #[doc = "V1 error format."]
                pub fn xgafv(mut self, value: crate::params::Xgafv) -> Self {
                    self.xgafv = Some(value);
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
                ) -> Result<crate::schemas::Job, Box<dyn ::std::error::Error>> {
                    self.execute_fields::<_, &str>(None)
                }
                #[doc = r" Execute the given operation. This will provide a `fields`"]
                #[doc = r" selector of `*`. This will include every attribute of the"]
                #[doc = r" response resource and should be limited to use during"]
                #[doc = r" development or debugging."]
                pub fn execute_debug(
                    self,
                ) -> Result<crate::schemas::Job, Box<dyn ::std::error::Error>> {
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
                    let mut output = "https://dataflow.googleapis.com/".to_owned();
                    output.push_str("v1b3/projects/");
                    {
                        let var_as_str = &self.project_id;
                        output.extend(::percent_encoding::utf8_percent_encode(
                            &var_as_str,
                            crate::SIMPLE,
                        ));
                    }
                    output.push_str("/jobs");
                    output
                }
                fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
                    let req = self.reqwest.request(::reqwest::Method::POST, path);
                    let req = req.query(&[("location", &self.location)]);
                    let req = req.query(&[("replaceJobId", &self.replace_job_id)]);
                    let req = req.query(&[("view", &self.view)]);
                    let req = req.query(&[("access_token", &self.access_token)]);
                    let req = req.query(&[("alt", &self.alt)]);
                    let req = req.query(&[("callback", &self.callback)]);
                    let req = req.query(&[("fields", &self.fields)]);
                    let req = req.query(&[("key", &self.key)]);
                    let req = req.query(&[("oauth_token", &self.oauth_token)]);
                    let req = req.query(&[("prettyPrint", &self.pretty_print)]);
                    let req = req.query(&[("quotaUser", &self.quota_user)]);
                    let req = req.query(&[("upload_protocol", &self.upload_protocol)]);
                    let req = req.query(&[("uploadType", &self.upload_type)]);
                    let req = req.query(&[("$.xgafv", &self.xgafv)]);
                    let mut auth = self.auth.lock().unwrap();
                    let fut = auth.token(vec!["https://www.googleapis.com/auth/cloud-platform"]);
                    let mut runtime = ::tokio::runtime::Runtime::new().unwrap();
                    let token = runtime.block_on(fut).unwrap().access_token;
                    let req = req.bearer_auth(&token);
                    req
                }
            }
            #[derive(Debug, Clone)]
            pub struct GetRequestBuilder<'a, A> {
                pub(crate) reqwest: &'a ::reqwest::Client,
                pub(crate) auth: &'a ::std::sync::Mutex<A>,
                project_id: String,
                job_id: String,
                location: Option<String>,
                view: Option<crate::resources::projects::jobs::params::GetView>,
                access_token: Option<String>,
                alt: Option<crate::params::Alt>,
                callback: Option<String>,
                fields: Option<String>,
                key: Option<String>,
                oauth_token: Option<String>,
                pretty_print: Option<bool>,
                quota_user: Option<String>,
                upload_protocol: Option<String>,
                upload_type: Option<String>,
                xgafv: Option<crate::params::Xgafv>,
            }
            impl<'a, A: yup_oauth2::GetToken> GetRequestBuilder<'a, A> {
                #[doc = "The [regional endpoint]\n(https://cloud.google.com/dataflow/docs/concepts/regional-endpoints) that\ncontains this job."]
                pub fn location(mut self, value: impl Into<String>) -> Self {
                    self.location = Some(value.into());
                    self
                }
                #[doc = "The level of information requested in response."]
                pub fn view(
                    mut self,
                    value: crate::resources::projects::jobs::params::GetView,
                ) -> Self {
                    self.view = Some(value);
                    self
                }
                #[doc = "OAuth access token."]
                pub fn access_token(mut self, value: impl Into<String>) -> Self {
                    self.access_token = Some(value.into());
                    self
                }
                #[doc = "JSONP"]
                pub fn callback(mut self, value: impl Into<String>) -> Self {
                    self.callback = Some(value.into());
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
                #[doc = "Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters."]
                pub fn quota_user(mut self, value: impl Into<String>) -> Self {
                    self.quota_user = Some(value.into());
                    self
                }
                #[doc = "Upload protocol for media (e.g. \"raw\", \"multipart\")."]
                pub fn upload_protocol(mut self, value: impl Into<String>) -> Self {
                    self.upload_protocol = Some(value.into());
                    self
                }
                #[doc = "Legacy upload protocol for media (e.g. \"media\", \"multipart\")."]
                pub fn upload_type(mut self, value: impl Into<String>) -> Self {
                    self.upload_type = Some(value.into());
                    self
                }
                #[doc = "V1 error format."]
                pub fn xgafv(mut self, value: crate::params::Xgafv) -> Self {
                    self.xgafv = Some(value);
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
                ) -> Result<crate::schemas::Job, Box<dyn ::std::error::Error>> {
                    self.execute_fields::<_, &str>(None)
                }
                #[doc = r" Execute the given operation. This will provide a `fields`"]
                #[doc = r" selector of `*`. This will include every attribute of the"]
                #[doc = r" response resource and should be limited to use during"]
                #[doc = r" development or debugging."]
                pub fn execute_debug(
                    self,
                ) -> Result<crate::schemas::Job, Box<dyn ::std::error::Error>> {
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
                    let mut output = "https://dataflow.googleapis.com/".to_owned();
                    output.push_str("v1b3/projects/");
                    {
                        let var_as_str = &self.project_id;
                        output.extend(::percent_encoding::utf8_percent_encode(
                            &var_as_str,
                            crate::SIMPLE,
                        ));
                    }
                    output.push_str("/jobs/");
                    {
                        let var_as_str = &self.job_id;
                        output.extend(::percent_encoding::utf8_percent_encode(
                            &var_as_str,
                            crate::SIMPLE,
                        ));
                    }
                    output
                }
                fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
                    let req = self.reqwest.request(::reqwest::Method::GET, path);
                    let req = req.query(&[("location", &self.location)]);
                    let req = req.query(&[("view", &self.view)]);
                    let req = req.query(&[("access_token", &self.access_token)]);
                    let req = req.query(&[("alt", &self.alt)]);
                    let req = req.query(&[("callback", &self.callback)]);
                    let req = req.query(&[("fields", &self.fields)]);
                    let req = req.query(&[("key", &self.key)]);
                    let req = req.query(&[("oauth_token", &self.oauth_token)]);
                    let req = req.query(&[("prettyPrint", &self.pretty_print)]);
                    let req = req.query(&[("quotaUser", &self.quota_user)]);
                    let req = req.query(&[("upload_protocol", &self.upload_protocol)]);
                    let req = req.query(&[("uploadType", &self.upload_type)]);
                    let req = req.query(&[("$.xgafv", &self.xgafv)]);
                    let mut auth = self.auth.lock().unwrap();
                    let fut = auth.token(vec!["https://www.googleapis.com/auth/compute.readonly"]);
                    let mut runtime = ::tokio::runtime::Runtime::new().unwrap();
                    let token = runtime.block_on(fut).unwrap().access_token;
                    let req = req.bearer_auth(&token);
                    req
                }
            }
            #[derive(Debug, Clone)]
            pub struct GetMetricsRequestBuilder<'a, A> {
                pub(crate) reqwest: &'a ::reqwest::Client,
                pub(crate) auth: &'a ::std::sync::Mutex<A>,
                project_id: String,
                job_id: String,
                location: Option<String>,
                start_time: Option<String>,
                access_token: Option<String>,
                alt: Option<crate::params::Alt>,
                callback: Option<String>,
                fields: Option<String>,
                key: Option<String>,
                oauth_token: Option<String>,
                pretty_print: Option<bool>,
                quota_user: Option<String>,
                upload_protocol: Option<String>,
                upload_type: Option<String>,
                xgafv: Option<crate::params::Xgafv>,
            }
            impl<'a, A: yup_oauth2::GetToken> GetMetricsRequestBuilder<'a, A> {
                #[doc = "The [regional endpoint]\n(https://cloud.google.com/dataflow/docs/concepts/regional-endpoints) that\ncontains the job specified by job_id."]
                pub fn location(mut self, value: impl Into<String>) -> Self {
                    self.location = Some(value.into());
                    self
                }
                #[doc = "Return only metric data that has changed since this time.\nDefault is to return all information about all metrics for the job."]
                pub fn start_time(mut self, value: impl Into<String>) -> Self {
                    self.start_time = Some(value.into());
                    self
                }
                #[doc = "OAuth access token."]
                pub fn access_token(mut self, value: impl Into<String>) -> Self {
                    self.access_token = Some(value.into());
                    self
                }
                #[doc = "JSONP"]
                pub fn callback(mut self, value: impl Into<String>) -> Self {
                    self.callback = Some(value.into());
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
                #[doc = "Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters."]
                pub fn quota_user(mut self, value: impl Into<String>) -> Self {
                    self.quota_user = Some(value.into());
                    self
                }
                #[doc = "Upload protocol for media (e.g. \"raw\", \"multipart\")."]
                pub fn upload_protocol(mut self, value: impl Into<String>) -> Self {
                    self.upload_protocol = Some(value.into());
                    self
                }
                #[doc = "Legacy upload protocol for media (e.g. \"media\", \"multipart\")."]
                pub fn upload_type(mut self, value: impl Into<String>) -> Self {
                    self.upload_type = Some(value.into());
                    self
                }
                #[doc = "V1 error format."]
                pub fn xgafv(mut self, value: crate::params::Xgafv) -> Self {
                    self.xgafv = Some(value);
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
                ) -> Result<crate::schemas::JobMetrics, Box<dyn ::std::error::Error>>
                {
                    self.execute_fields::<_, &str>(None)
                }
                #[doc = r" Execute the given operation. This will provide a `fields`"]
                #[doc = r" selector of `*`. This will include every attribute of the"]
                #[doc = r" response resource and should be limited to use during"]
                #[doc = r" development or debugging."]
                pub fn execute_debug(
                    self,
                ) -> Result<crate::schemas::JobMetrics, Box<dyn ::std::error::Error>>
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
                    let mut output = "https://dataflow.googleapis.com/".to_owned();
                    output.push_str("v1b3/projects/");
                    {
                        let var_as_str = &self.project_id;
                        output.extend(::percent_encoding::utf8_percent_encode(
                            &var_as_str,
                            crate::SIMPLE,
                        ));
                    }
                    output.push_str("/jobs/");
                    {
                        let var_as_str = &self.job_id;
                        output.extend(::percent_encoding::utf8_percent_encode(
                            &var_as_str,
                            crate::SIMPLE,
                        ));
                    }
                    output.push_str("/metrics");
                    output
                }
                fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
                    let req = self.reqwest.request(::reqwest::Method::GET, path);
                    let req = req.query(&[("location", &self.location)]);
                    let req = req.query(&[("startTime", &self.start_time)]);
                    let req = req.query(&[("access_token", &self.access_token)]);
                    let req = req.query(&[("alt", &self.alt)]);
                    let req = req.query(&[("callback", &self.callback)]);
                    let req = req.query(&[("fields", &self.fields)]);
                    let req = req.query(&[("key", &self.key)]);
                    let req = req.query(&[("oauth_token", &self.oauth_token)]);
                    let req = req.query(&[("prettyPrint", &self.pretty_print)]);
                    let req = req.query(&[("quotaUser", &self.quota_user)]);
                    let req = req.query(&[("upload_protocol", &self.upload_protocol)]);
                    let req = req.query(&[("uploadType", &self.upload_type)]);
                    let req = req.query(&[("$.xgafv", &self.xgafv)]);
                    let mut auth = self.auth.lock().unwrap();
                    let fut = auth.token(vec!["https://www.googleapis.com/auth/compute.readonly"]);
                    let mut runtime = ::tokio::runtime::Runtime::new().unwrap();
                    let token = runtime.block_on(fut).unwrap().access_token;
                    let req = req.bearer_auth(&token);
                    req
                }
            }
            #[derive(Debug, Clone)]
            pub struct ListRequestBuilder<'a, A> {
                pub(crate) reqwest: &'a ::reqwest::Client,
                pub(crate) auth: &'a ::std::sync::Mutex<A>,
                project_id: String,
                filter: Option<crate::resources::projects::jobs::params::ListFilter>,
                location: Option<String>,
                page_size: Option<i32>,
                page_token: Option<String>,
                view: Option<crate::resources::projects::jobs::params::ListView>,
                access_token: Option<String>,
                alt: Option<crate::params::Alt>,
                callback: Option<String>,
                fields: Option<String>,
                key: Option<String>,
                oauth_token: Option<String>,
                pretty_print: Option<bool>,
                quota_user: Option<String>,
                upload_protocol: Option<String>,
                upload_type: Option<String>,
                xgafv: Option<crate::params::Xgafv>,
            }
            impl<'a, A: yup_oauth2::GetToken> ListRequestBuilder<'a, A> {
                #[doc = "The kind of filter to use."]
                pub fn filter(
                    mut self,
                    value: crate::resources::projects::jobs::params::ListFilter,
                ) -> Self {
                    self.filter = Some(value);
                    self
                }
                #[doc = "The [regional endpoint]\n(https://cloud.google.com/dataflow/docs/concepts/regional-endpoints) that\ncontains this job."]
                pub fn location(mut self, value: impl Into<String>) -> Self {
                    self.location = Some(value.into());
                    self
                }
                #[doc = "If there are many jobs, limit response to at most this many.\nThe actual number of jobs returned will be the lesser of max_responses\nand an unspecified server-defined limit."]
                pub fn page_size(mut self, value: i32) -> Self {
                    self.page_size = Some(value);
                    self
                }
                #[doc = "Set this to the 'next_page_token' field of a previous response\nto request additional results in a long list."]
                pub fn page_token(mut self, value: impl Into<String>) -> Self {
                    self.page_token = Some(value.into());
                    self
                }
                #[doc = "Level of information requested in response. Default is `JOB_VIEW_SUMMARY`."]
                pub fn view(
                    mut self,
                    value: crate::resources::projects::jobs::params::ListView,
                ) -> Self {
                    self.view = Some(value);
                    self
                }
                #[doc = "OAuth access token."]
                pub fn access_token(mut self, value: impl Into<String>) -> Self {
                    self.access_token = Some(value.into());
                    self
                }
                #[doc = "JSONP"]
                pub fn callback(mut self, value: impl Into<String>) -> Self {
                    self.callback = Some(value.into());
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
                #[doc = "Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters."]
                pub fn quota_user(mut self, value: impl Into<String>) -> Self {
                    self.quota_user = Some(value.into());
                    self
                }
                #[doc = "Upload protocol for media (e.g. \"raw\", \"multipart\")."]
                pub fn upload_protocol(mut self, value: impl Into<String>) -> Self {
                    self.upload_protocol = Some(value.into());
                    self
                }
                #[doc = "Legacy upload protocol for media (e.g. \"media\", \"multipart\")."]
                pub fn upload_type(mut self, value: impl Into<String>) -> Self {
                    self.upload_type = Some(value.into());
                    self
                }
                #[doc = "V1 error format."]
                pub fn xgafv(mut self, value: crate::params::Xgafv) -> Self {
                    self.xgafv = Some(value);
                    self
                }
                #[doc = r" Return an iterator that iterates over all `#prop_ident`. The"]
                #[doc = r" items yielded by the iterator are chosen by the caller of this"]
                #[doc = r" method and must implement `Deserialize` and `FieldSelector`. The"]
                #[doc = r" populated fields in the yielded items will be determined by the"]
                #[doc = r" `FieldSelector` implementation."]
                pub fn iter_failed_location<T>(mut self) -> crate::iter::PageItemIter<Self, T>
                where
                    T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
                {
                    let mut fields = concat!("nextPageToken,", "failedLocation").to_owned();
                    let items_fields = T::field_selector();
                    if !items_fields.is_empty() {
                        fields.push_str("(");
                        fields.push_str(&items_fields);
                        fields.push_str(")");
                    }
                    self.fields = Some(fields);
                    crate::iter::PageItemIter::new(self, "failedLocation")
                }
                #[doc = r" Return an iterator that iterates over all `#prop_ident`. The"]
                #[doc = r" items yielded by the iterator are `#items_type`. The populated"]
                #[doc = r" fields in `#items_type` will be the default fields populated by"]
                #[doc = r" the server."]
                pub fn iter_failed_location_standard(
                    mut self,
                ) -> crate::iter::PageItemIter<Self, crate::schemas::FailedLocation>
                {
                    self.fields = Some(concat!("nextPageToken,", "failedLocation").to_owned());
                    crate::iter::PageItemIter::new(self, "failedLocation")
                }
                #[doc = r" Return an iterator that iterates over all `#prop_ident`. The"]
                #[doc = r" items yielded by the iterator are `#items_type`. The populated"]
                #[doc = r" fields in `#items_type` will be all fields available. This should"]
                #[doc = r" primarily be used during developement and debugging as fetching"]
                #[doc = r" all fields can be expensive both in bandwidth and server"]
                #[doc = r" resources."]
                pub fn iter_failed_location_debug(
                    mut self,
                ) -> crate::iter::PageItemIter<Self, crate::schemas::FailedLocation>
                {
                    self.fields =
                        Some(concat!("nextPageToken,", "failedLocation", "(*)").to_owned());
                    crate::iter::PageItemIter::new(self, "failedLocation")
                }
                #[doc = r" Return an iterator that iterates over all `#prop_ident`. The"]
                #[doc = r" items yielded by the iterator are chosen by the caller of this"]
                #[doc = r" method and must implement `Deserialize` and `FieldSelector`. The"]
                #[doc = r" populated fields in the yielded items will be determined by the"]
                #[doc = r" `FieldSelector` implementation."]
                pub fn iter_jobs<T>(mut self) -> crate::iter::PageItemIter<Self, T>
                where
                    T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
                {
                    let mut fields = concat!("nextPageToken,", "jobs").to_owned();
                    let items_fields = T::field_selector();
                    if !items_fields.is_empty() {
                        fields.push_str("(");
                        fields.push_str(&items_fields);
                        fields.push_str(")");
                    }
                    self.fields = Some(fields);
                    crate::iter::PageItemIter::new(self, "jobs")
                }
                #[doc = r" Return an iterator that iterates over all `#prop_ident`. The"]
                #[doc = r" items yielded by the iterator are `#items_type`. The populated"]
                #[doc = r" fields in `#items_type` will be the default fields populated by"]
                #[doc = r" the server."]
                pub fn iter_jobs_standard(
                    mut self,
                ) -> crate::iter::PageItemIter<Self, crate::schemas::Job> {
                    self.fields = Some(concat!("nextPageToken,", "jobs").to_owned());
                    crate::iter::PageItemIter::new(self, "jobs")
                }
                #[doc = r" Return an iterator that iterates over all `#prop_ident`. The"]
                #[doc = r" items yielded by the iterator are `#items_type`. The populated"]
                #[doc = r" fields in `#items_type` will be all fields available. This should"]
                #[doc = r" primarily be used during developement and debugging as fetching"]
                #[doc = r" all fields can be expensive both in bandwidth and server"]
                #[doc = r" resources."]
                pub fn iter_jobs_debug(
                    mut self,
                ) -> crate::iter::PageItemIter<Self, crate::schemas::Job> {
                    self.fields = Some(concat!("nextPageToken,", "jobs", "(*)").to_owned());
                    crate::iter::PageItemIter::new(self, "jobs")
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
                ) -> crate::iter::PageIter<Self, crate::schemas::ListJobsResponse> {
                    crate::iter::PageIter::new(self)
                }
                pub fn iter_debug(
                    mut self,
                ) -> crate::iter::PageIter<Self, crate::schemas::ListJobsResponse> {
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
                ) -> Result<crate::schemas::ListJobsResponse, Box<dyn ::std::error::Error>>
                {
                    self.execute_fields::<_, &str>(None)
                }
                #[doc = r" Execute the given operation. This will provide a `fields`"]
                #[doc = r" selector of `*`. This will include every attribute of the"]
                #[doc = r" response resource and should be limited to use during"]
                #[doc = r" development or debugging."]
                pub fn execute_debug(
                    self,
                ) -> Result<crate::schemas::ListJobsResponse, Box<dyn ::std::error::Error>>
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
                    let mut output = "https://dataflow.googleapis.com/".to_owned();
                    output.push_str("v1b3/projects/");
                    {
                        let var_as_str = &self.project_id;
                        output.extend(::percent_encoding::utf8_percent_encode(
                            &var_as_str,
                            crate::SIMPLE,
                        ));
                    }
                    output.push_str("/jobs");
                    output
                }
                fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
                    let req = self.reqwest.request(::reqwest::Method::GET, path);
                    let req = req.query(&[("filter", &self.filter)]);
                    let req = req.query(&[("location", &self.location)]);
                    let req = req.query(&[("pageSize", &self.page_size)]);
                    let req = req.query(&[("pageToken", &self.page_token)]);
                    let req = req.query(&[("view", &self.view)]);
                    let req = req.query(&[("access_token", &self.access_token)]);
                    let req = req.query(&[("alt", &self.alt)]);
                    let req = req.query(&[("callback", &self.callback)]);
                    let req = req.query(&[("fields", &self.fields)]);
                    let req = req.query(&[("key", &self.key)]);
                    let req = req.query(&[("oauth_token", &self.oauth_token)]);
                    let req = req.query(&[("prettyPrint", &self.pretty_print)]);
                    let req = req.query(&[("quotaUser", &self.quota_user)]);
                    let req = req.query(&[("upload_protocol", &self.upload_protocol)]);
                    let req = req.query(&[("uploadType", &self.upload_type)]);
                    let req = req.query(&[("$.xgafv", &self.xgafv)]);
                    let mut auth = self.auth.lock().unwrap();
                    let fut = auth.token(vec!["https://www.googleapis.com/auth/compute.readonly"]);
                    let mut runtime = ::tokio::runtime::Runtime::new().unwrap();
                    let token = runtime.block_on(fut).unwrap().access_token;
                    let req = req.bearer_auth(&token);
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
            #[derive(Debug, Clone)]
            pub struct SnapshotRequestBuilder<'a, A> {
                pub(crate) reqwest: &'a ::reqwest::Client,
                pub(crate) auth: &'a ::std::sync::Mutex<A>,
                request: crate::schemas::SnapshotJobRequest,
                project_id: String,
                job_id: String,
                access_token: Option<String>,
                alt: Option<crate::params::Alt>,
                callback: Option<String>,
                fields: Option<String>,
                key: Option<String>,
                oauth_token: Option<String>,
                pretty_print: Option<bool>,
                quota_user: Option<String>,
                upload_protocol: Option<String>,
                upload_type: Option<String>,
                xgafv: Option<crate::params::Xgafv>,
            }
            impl<'a, A: yup_oauth2::GetToken> SnapshotRequestBuilder<'a, A> {
                #[doc = "OAuth access token."]
                pub fn access_token(mut self, value: impl Into<String>) -> Self {
                    self.access_token = Some(value.into());
                    self
                }
                #[doc = "JSONP"]
                pub fn callback(mut self, value: impl Into<String>) -> Self {
                    self.callback = Some(value.into());
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
                #[doc = "Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters."]
                pub fn quota_user(mut self, value: impl Into<String>) -> Self {
                    self.quota_user = Some(value.into());
                    self
                }
                #[doc = "Upload protocol for media (e.g. \"raw\", \"multipart\")."]
                pub fn upload_protocol(mut self, value: impl Into<String>) -> Self {
                    self.upload_protocol = Some(value.into());
                    self
                }
                #[doc = "Legacy upload protocol for media (e.g. \"media\", \"multipart\")."]
                pub fn upload_type(mut self, value: impl Into<String>) -> Self {
                    self.upload_type = Some(value.into());
                    self
                }
                #[doc = "V1 error format."]
                pub fn xgafv(mut self, value: crate::params::Xgafv) -> Self {
                    self.xgafv = Some(value);
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
                ) -> Result<crate::schemas::Snapshot, Box<dyn ::std::error::Error>>
                {
                    self.execute_fields::<_, &str>(None)
                }
                #[doc = r" Execute the given operation. This will provide a `fields`"]
                #[doc = r" selector of `*`. This will include every attribute of the"]
                #[doc = r" response resource and should be limited to use during"]
                #[doc = r" development or debugging."]
                pub fn execute_debug(
                    self,
                ) -> Result<crate::schemas::Snapshot, Box<dyn ::std::error::Error>>
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
                    let mut output = "https://dataflow.googleapis.com/".to_owned();
                    output.push_str("v1b3/projects/");
                    {
                        let var_as_str = &self.project_id;
                        output.extend(::percent_encoding::utf8_percent_encode(
                            &var_as_str,
                            crate::SIMPLE,
                        ));
                    }
                    output.push_str("/jobs/");
                    {
                        let var_as_str = &self.job_id;
                        output.extend(::percent_encoding::utf8_percent_encode(
                            &var_as_str,
                            crate::SIMPLE,
                        ));
                    }
                    output.push_str(":snapshot");
                    output
                }
                fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
                    let req = self.reqwest.request(::reqwest::Method::POST, path);
                    let req = req.query(&[("access_token", &self.access_token)]);
                    let req = req.query(&[("alt", &self.alt)]);
                    let req = req.query(&[("callback", &self.callback)]);
                    let req = req.query(&[("fields", &self.fields)]);
                    let req = req.query(&[("key", &self.key)]);
                    let req = req.query(&[("oauth_token", &self.oauth_token)]);
                    let req = req.query(&[("prettyPrint", &self.pretty_print)]);
                    let req = req.query(&[("quotaUser", &self.quota_user)]);
                    let req = req.query(&[("upload_protocol", &self.upload_protocol)]);
                    let req = req.query(&[("uploadType", &self.upload_type)]);
                    let req = req.query(&[("$.xgafv", &self.xgafv)]);
                    let mut auth = self.auth.lock().unwrap();
                    let fut = auth.token(vec!["https://www.googleapis.com/auth/cloud-platform"]);
                    let mut runtime = ::tokio::runtime::Runtime::new().unwrap();
                    let token = runtime.block_on(fut).unwrap().access_token;
                    let req = req.bearer_auth(&token);
                    req
                }
            }
            #[derive(Debug, Clone)]
            pub struct UpdateRequestBuilder<'a, A> {
                pub(crate) reqwest: &'a ::reqwest::Client,
                pub(crate) auth: &'a ::std::sync::Mutex<A>,
                request: crate::schemas::Job,
                project_id: String,
                job_id: String,
                location: Option<String>,
                access_token: Option<String>,
                alt: Option<crate::params::Alt>,
                callback: Option<String>,
                fields: Option<String>,
                key: Option<String>,
                oauth_token: Option<String>,
                pretty_print: Option<bool>,
                quota_user: Option<String>,
                upload_protocol: Option<String>,
                upload_type: Option<String>,
                xgafv: Option<crate::params::Xgafv>,
            }
            impl<'a, A: yup_oauth2::GetToken> UpdateRequestBuilder<'a, A> {
                #[doc = "The [regional endpoint]\n(https://cloud.google.com/dataflow/docs/concepts/regional-endpoints) that\ncontains this job."]
                pub fn location(mut self, value: impl Into<String>) -> Self {
                    self.location = Some(value.into());
                    self
                }
                #[doc = "OAuth access token."]
                pub fn access_token(mut self, value: impl Into<String>) -> Self {
                    self.access_token = Some(value.into());
                    self
                }
                #[doc = "JSONP"]
                pub fn callback(mut self, value: impl Into<String>) -> Self {
                    self.callback = Some(value.into());
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
                #[doc = "Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters."]
                pub fn quota_user(mut self, value: impl Into<String>) -> Self {
                    self.quota_user = Some(value.into());
                    self
                }
                #[doc = "Upload protocol for media (e.g. \"raw\", \"multipart\")."]
                pub fn upload_protocol(mut self, value: impl Into<String>) -> Self {
                    self.upload_protocol = Some(value.into());
                    self
                }
                #[doc = "Legacy upload protocol for media (e.g. \"media\", \"multipart\")."]
                pub fn upload_type(mut self, value: impl Into<String>) -> Self {
                    self.upload_type = Some(value.into());
                    self
                }
                #[doc = "V1 error format."]
                pub fn xgafv(mut self, value: crate::params::Xgafv) -> Self {
                    self.xgafv = Some(value);
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
                ) -> Result<crate::schemas::Job, Box<dyn ::std::error::Error>> {
                    self.execute_fields::<_, &str>(None)
                }
                #[doc = r" Execute the given operation. This will provide a `fields`"]
                #[doc = r" selector of `*`. This will include every attribute of the"]
                #[doc = r" response resource and should be limited to use during"]
                #[doc = r" development or debugging."]
                pub fn execute_debug(
                    self,
                ) -> Result<crate::schemas::Job, Box<dyn ::std::error::Error>> {
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
                    let mut output = "https://dataflow.googleapis.com/".to_owned();
                    output.push_str("v1b3/projects/");
                    {
                        let var_as_str = &self.project_id;
                        output.extend(::percent_encoding::utf8_percent_encode(
                            &var_as_str,
                            crate::SIMPLE,
                        ));
                    }
                    output.push_str("/jobs/");
                    {
                        let var_as_str = &self.job_id;
                        output.extend(::percent_encoding::utf8_percent_encode(
                            &var_as_str,
                            crate::SIMPLE,
                        ));
                    }
                    output
                }
                fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
                    let req = self.reqwest.request(::reqwest::Method::PUT, path);
                    let req = req.query(&[("location", &self.location)]);
                    let req = req.query(&[("access_token", &self.access_token)]);
                    let req = req.query(&[("alt", &self.alt)]);
                    let req = req.query(&[("callback", &self.callback)]);
                    let req = req.query(&[("fields", &self.fields)]);
                    let req = req.query(&[("key", &self.key)]);
                    let req = req.query(&[("oauth_token", &self.oauth_token)]);
                    let req = req.query(&[("prettyPrint", &self.pretty_print)]);
                    let req = req.query(&[("quotaUser", &self.quota_user)]);
                    let req = req.query(&[("upload_protocol", &self.upload_protocol)]);
                    let req = req.query(&[("uploadType", &self.upload_type)]);
                    let req = req.query(&[("$.xgafv", &self.xgafv)]);
                    let mut auth = self.auth.lock().unwrap();
                    let fut = auth.token(vec!["https://www.googleapis.com/auth/cloud-platform"]);
                    let mut runtime = ::tokio::runtime::Runtime::new().unwrap();
                    let token = runtime.block_on(fut).unwrap().access_token;
                    let req = req.bearer_auth(&token);
                    req
                }
            }
            pub mod debug {
                pub mod params {}
                pub struct DebugActions<'a, A> {
                    pub(crate) reqwest: &'a reqwest::Client,
                    pub(crate) auth: &'a std::sync::Mutex<A>,
                }
                impl<'a, A: yup_oauth2::GetToken> DebugActions<'a, A> {
                    #[doc = "Get encoded debug configuration for component. Not cacheable."]
                    pub fn get_config(
                        &self,
                        request: crate::schemas::GetDebugConfigRequest,
                        project_id: impl Into<String>,
                        job_id: impl Into<String>,
                    ) -> GetConfigRequestBuilder<A> {
                        GetConfigRequestBuilder {
                            reqwest: &self.reqwest,
                            auth: &self.auth,
                            request,
                            access_token: None,
                            alt: None,
                            callback: None,
                            fields: None,
                            key: None,
                            oauth_token: None,
                            pretty_print: None,
                            quota_user: None,
                            upload_protocol: None,
                            upload_type: None,
                            xgafv: None,
                            project_id: project_id.into(),
                            job_id: job_id.into(),
                        }
                    }
                    #[doc = "Send encoded debug capture data for component."]
                    pub fn send_capture(
                        &self,
                        request: crate::schemas::SendDebugCaptureRequest,
                        project_id: impl Into<String>,
                        job_id: impl Into<String>,
                    ) -> SendCaptureRequestBuilder<A> {
                        SendCaptureRequestBuilder {
                            reqwest: &self.reqwest,
                            auth: &self.auth,
                            request,
                            access_token: None,
                            alt: None,
                            callback: None,
                            fields: None,
                            key: None,
                            oauth_token: None,
                            pretty_print: None,
                            quota_user: None,
                            upload_protocol: None,
                            upload_type: None,
                            xgafv: None,
                            project_id: project_id.into(),
                            job_id: job_id.into(),
                        }
                    }
                }
                #[derive(Debug, Clone)]
                pub struct GetConfigRequestBuilder<'a, A> {
                    pub(crate) reqwest: &'a ::reqwest::Client,
                    pub(crate) auth: &'a ::std::sync::Mutex<A>,
                    request: crate::schemas::GetDebugConfigRequest,
                    project_id: String,
                    job_id: String,
                    access_token: Option<String>,
                    alt: Option<crate::params::Alt>,
                    callback: Option<String>,
                    fields: Option<String>,
                    key: Option<String>,
                    oauth_token: Option<String>,
                    pretty_print: Option<bool>,
                    quota_user: Option<String>,
                    upload_protocol: Option<String>,
                    upload_type: Option<String>,
                    xgafv: Option<crate::params::Xgafv>,
                }
                impl<'a, A: yup_oauth2::GetToken> GetConfigRequestBuilder<'a, A> {
                    #[doc = "OAuth access token."]
                    pub fn access_token(mut self, value: impl Into<String>) -> Self {
                        self.access_token = Some(value.into());
                        self
                    }
                    #[doc = "JSONP"]
                    pub fn callback(mut self, value: impl Into<String>) -> Self {
                        self.callback = Some(value.into());
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
                    #[doc = "Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters."]
                    pub fn quota_user(mut self, value: impl Into<String>) -> Self {
                        self.quota_user = Some(value.into());
                        self
                    }
                    #[doc = "Upload protocol for media (e.g. \"raw\", \"multipart\")."]
                    pub fn upload_protocol(mut self, value: impl Into<String>) -> Self {
                        self.upload_protocol = Some(value.into());
                        self
                    }
                    #[doc = "Legacy upload protocol for media (e.g. \"media\", \"multipart\")."]
                    pub fn upload_type(mut self, value: impl Into<String>) -> Self {
                        self.upload_type = Some(value.into());
                        self
                    }
                    #[doc = "V1 error format."]
                    pub fn xgafv(mut self, value: crate::params::Xgafv) -> Self {
                        self.xgafv = Some(value);
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
                    ) -> Result<crate::schemas::GetDebugConfigResponse, Box<dyn ::std::error::Error>>
                    {
                        self.execute_fields::<_, &str>(None)
                    }
                    #[doc = r" Execute the given operation. This will provide a `fields`"]
                    #[doc = r" selector of `*`. This will include every attribute of the"]
                    #[doc = r" response resource and should be limited to use during"]
                    #[doc = r" development or debugging."]
                    pub fn execute_debug(
                        self,
                    ) -> Result<crate::schemas::GetDebugConfigResponse, Box<dyn ::std::error::Error>>
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
                        let mut output = "https://dataflow.googleapis.com/".to_owned();
                        output.push_str("v1b3/projects/");
                        {
                            let var_as_str = &self.project_id;
                            output.extend(::percent_encoding::utf8_percent_encode(
                                &var_as_str,
                                crate::SIMPLE,
                            ));
                        }
                        output.push_str("/jobs/");
                        {
                            let var_as_str = &self.job_id;
                            output.extend(::percent_encoding::utf8_percent_encode(
                                &var_as_str,
                                crate::SIMPLE,
                            ));
                        }
                        output.push_str("/debug/getConfig");
                        output
                    }
                    fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
                        let req = self.reqwest.request(::reqwest::Method::POST, path);
                        let req = req.query(&[("access_token", &self.access_token)]);
                        let req = req.query(&[("alt", &self.alt)]);
                        let req = req.query(&[("callback", &self.callback)]);
                        let req = req.query(&[("fields", &self.fields)]);
                        let req = req.query(&[("key", &self.key)]);
                        let req = req.query(&[("oauth_token", &self.oauth_token)]);
                        let req = req.query(&[("prettyPrint", &self.pretty_print)]);
                        let req = req.query(&[("quotaUser", &self.quota_user)]);
                        let req = req.query(&[("upload_protocol", &self.upload_protocol)]);
                        let req = req.query(&[("uploadType", &self.upload_type)]);
                        let req = req.query(&[("$.xgafv", &self.xgafv)]);
                        let mut auth = self.auth.lock().unwrap();
                        let fut =
                            auth.token(vec!["https://www.googleapis.com/auth/cloud-platform"]);
                        let mut runtime = ::tokio::runtime::Runtime::new().unwrap();
                        let token = runtime.block_on(fut).unwrap().access_token;
                        let req = req.bearer_auth(&token);
                        req
                    }
                }
                #[derive(Debug, Clone)]
                pub struct SendCaptureRequestBuilder<'a, A> {
                    pub(crate) reqwest: &'a ::reqwest::Client,
                    pub(crate) auth: &'a ::std::sync::Mutex<A>,
                    request: crate::schemas::SendDebugCaptureRequest,
                    project_id: String,
                    job_id: String,
                    access_token: Option<String>,
                    alt: Option<crate::params::Alt>,
                    callback: Option<String>,
                    fields: Option<String>,
                    key: Option<String>,
                    oauth_token: Option<String>,
                    pretty_print: Option<bool>,
                    quota_user: Option<String>,
                    upload_protocol: Option<String>,
                    upload_type: Option<String>,
                    xgafv: Option<crate::params::Xgafv>,
                }
                impl<'a, A: yup_oauth2::GetToken> SendCaptureRequestBuilder<'a, A> {
                    #[doc = "OAuth access token."]
                    pub fn access_token(mut self, value: impl Into<String>) -> Self {
                        self.access_token = Some(value.into());
                        self
                    }
                    #[doc = "JSONP"]
                    pub fn callback(mut self, value: impl Into<String>) -> Self {
                        self.callback = Some(value.into());
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
                    #[doc = "Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters."]
                    pub fn quota_user(mut self, value: impl Into<String>) -> Self {
                        self.quota_user = Some(value.into());
                        self
                    }
                    #[doc = "Upload protocol for media (e.g. \"raw\", \"multipart\")."]
                    pub fn upload_protocol(mut self, value: impl Into<String>) -> Self {
                        self.upload_protocol = Some(value.into());
                        self
                    }
                    #[doc = "Legacy upload protocol for media (e.g. \"media\", \"multipart\")."]
                    pub fn upload_type(mut self, value: impl Into<String>) -> Self {
                        self.upload_type = Some(value.into());
                        self
                    }
                    #[doc = "V1 error format."]
                    pub fn xgafv(mut self, value: crate::params::Xgafv) -> Self {
                        self.xgafv = Some(value);
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
                        crate::schemas::SendDebugCaptureResponse,
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
                        crate::schemas::SendDebugCaptureResponse,
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
                        let req = req.json(&self.request);
                        Ok(req.send()?.error_for_status()?.json()?)
                    }
                    fn _path(&self) -> String {
                        let mut output = "https://dataflow.googleapis.com/".to_owned();
                        output.push_str("v1b3/projects/");
                        {
                            let var_as_str = &self.project_id;
                            output.extend(::percent_encoding::utf8_percent_encode(
                                &var_as_str,
                                crate::SIMPLE,
                            ));
                        }
                        output.push_str("/jobs/");
                        {
                            let var_as_str = &self.job_id;
                            output.extend(::percent_encoding::utf8_percent_encode(
                                &var_as_str,
                                crate::SIMPLE,
                            ));
                        }
                        output.push_str("/debug/sendCapture");
                        output
                    }
                    fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
                        let req = self.reqwest.request(::reqwest::Method::POST, path);
                        let req = req.query(&[("access_token", &self.access_token)]);
                        let req = req.query(&[("alt", &self.alt)]);
                        let req = req.query(&[("callback", &self.callback)]);
                        let req = req.query(&[("fields", &self.fields)]);
                        let req = req.query(&[("key", &self.key)]);
                        let req = req.query(&[("oauth_token", &self.oauth_token)]);
                        let req = req.query(&[("prettyPrint", &self.pretty_print)]);
                        let req = req.query(&[("quotaUser", &self.quota_user)]);
                        let req = req.query(&[("upload_protocol", &self.upload_protocol)]);
                        let req = req.query(&[("uploadType", &self.upload_type)]);
                        let req = req.query(&[("$.xgafv", &self.xgafv)]);
                        let mut auth = self.auth.lock().unwrap();
                        let fut =
                            auth.token(vec!["https://www.googleapis.com/auth/cloud-platform"]);
                        let mut runtime = ::tokio::runtime::Runtime::new().unwrap();
                        let token = runtime.block_on(fut).unwrap().access_token;
                        let req = req.bearer_auth(&token);
                        req
                    }
                }
            }
            pub mod messages {
                pub mod params {
                    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
                    pub enum ListMinimumImportance {
                        JobMessageBasic,
                        JobMessageDebug,
                        JobMessageDetailed,
                        JobMessageError,
                        JobMessageImportanceUnknown,
                        JobMessageWarning,
                    }
                    impl ListMinimumImportance {
                        pub fn as_str(self) -> &'static str {
                            match self {
                                ListMinimumImportance::JobMessageBasic => "JOB_MESSAGE_BASIC",
                                ListMinimumImportance::JobMessageDebug => "JOB_MESSAGE_DEBUG",
                                ListMinimumImportance::JobMessageDetailed => "JOB_MESSAGE_DETAILED",
                                ListMinimumImportance::JobMessageError => "JOB_MESSAGE_ERROR",
                                ListMinimumImportance::JobMessageImportanceUnknown => {
                                    "JOB_MESSAGE_IMPORTANCE_UNKNOWN"
                                }
                                ListMinimumImportance::JobMessageWarning => "JOB_MESSAGE_WARNING",
                            }
                        }
                    }
                    impl ::std::fmt::Display for ListMinimumImportance {
                        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                            f.write_str(self.as_str())
                        }
                    }
                    impl ::serde::Serialize for ListMinimumImportance {
                        fn serialize<S>(
                            &self,
                            serializer: S,
                        ) -> ::std::result::Result<S::Ok, S::Error>
                        where
                            S: ::serde::ser::Serializer,
                        {
                            serializer.serialize_str(self.as_str())
                        }
                    }
                    impl<'de> ::serde::Deserialize<'de> for ListMinimumImportance {
                        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
                        where
                            D: ::serde::de::Deserializer<'de>,
                        {
                            let value: &'de str = <&str>::deserialize(deserializer)?;
                            Ok(match value {
                                "JOB_MESSAGE_BASIC" => ListMinimumImportance::JobMessageBasic,
                                "JOB_MESSAGE_DEBUG" => ListMinimumImportance::JobMessageDebug,
                                "JOB_MESSAGE_DETAILED" => ListMinimumImportance::JobMessageDetailed,
                                "JOB_MESSAGE_ERROR" => ListMinimumImportance::JobMessageError,
                                "JOB_MESSAGE_IMPORTANCE_UNKNOWN" => {
                                    ListMinimumImportance::JobMessageImportanceUnknown
                                }
                                "JOB_MESSAGE_WARNING" => ListMinimumImportance::JobMessageWarning,
                                _ => {
                                    return Err(::serde::de::Error::custom(format!(
                                        "invalid enum for #name: {}",
                                        value
                                    )))
                                }
                            })
                        }
                    }
                    impl ::field_selector::FieldSelector for ListMinimumImportance {
                        fn field_selector_with_ident(ident: &str, selector: &mut String) {
                            match selector.chars().rev().nth(0) {
                                Some(',') | None => {}
                                _ => selector.push_str(","),
                            }
                            selector.push_str(ident);
                        }
                    }
                }
                pub struct MessagesActions<'a, A> {
                    pub(crate) reqwest: &'a reqwest::Client,
                    pub(crate) auth: &'a std::sync::Mutex<A>,
                }
                impl<'a, A: yup_oauth2::GetToken> MessagesActions<'a, A> {
                    #[doc = "Request the job status.\n\nTo request the status of a job, we recommend using\n`projects.locations.jobs.messages.list` with a [regional endpoint]\n(https://cloud.google.com/dataflow/docs/concepts/regional-endpoints). Using\n`projects.jobs.messages.list` is not recommended, as you can only request\nthe status of jobs that are running in `us-central1`."]
                    pub fn list(
                        &self,
                        project_id: impl Into<String>,
                        job_id: impl Into<String>,
                    ) -> ListRequestBuilder<A> {
                        ListRequestBuilder {
                            reqwest: &self.reqwest,
                            auth: &self.auth,
                            access_token: None,
                            alt: None,
                            callback: None,
                            fields: None,
                            key: None,
                            oauth_token: None,
                            pretty_print: None,
                            quota_user: None,
                            upload_protocol: None,
                            upload_type: None,
                            xgafv: None,
                            project_id: project_id.into(),
                            job_id: job_id.into(),
                            end_time: None,
                            location: None,
                            minimum_importance: None,
                            page_size: None,
                            page_token: None,
                            start_time: None,
                        }
                    }
                }
                #[derive(Debug, Clone)]
                pub struct ListRequestBuilder<'a, A> {
                    pub(crate) reqwest: &'a ::reqwest::Client,
                    pub(crate) auth: &'a ::std::sync::Mutex<A>,
                    project_id: String,
                    job_id: String,
                    end_time: Option<String>,
                    location: Option<String>,
                    minimum_importance: Option<
                        crate::resources::projects::jobs::messages::params::ListMinimumImportance,
                    >,
                    page_size: Option<i32>,
                    page_token: Option<String>,
                    start_time: Option<String>,
                    access_token: Option<String>,
                    alt: Option<crate::params::Alt>,
                    callback: Option<String>,
                    fields: Option<String>,
                    key: Option<String>,
                    oauth_token: Option<String>,
                    pretty_print: Option<bool>,
                    quota_user: Option<String>,
                    upload_protocol: Option<String>,
                    upload_type: Option<String>,
                    xgafv: Option<crate::params::Xgafv>,
                }
                impl<'a, A: yup_oauth2::GetToken> ListRequestBuilder<'a, A> {
                    #[doc = "Return only messages with timestamps < end_time. The default is now\n(i.e. return up to the latest messages available)."]
                    pub fn end_time(mut self, value: impl Into<String>) -> Self {
                        self.end_time = Some(value.into());
                        self
                    }
                    #[doc = "The [regional endpoint]\n(https://cloud.google.com/dataflow/docs/concepts/regional-endpoints) that\ncontains the job specified by job_id."]
                    pub fn location(mut self, value: impl Into<String>) -> Self {
                        self.location = Some(value.into());
                        self
                    }
                    #[doc = "Filter to only get messages with importance >= level"]
                    pub fn minimum_importance(
                        mut self,
                        value : crate :: resources :: projects :: jobs :: messages :: params :: ListMinimumImportance,
                    ) -> Self {
                        self.minimum_importance = Some(value);
                        self
                    }
                    #[doc = "If specified, determines the maximum number of messages to\nreturn.  If unspecified, the service may choose an appropriate\ndefault, or may return an arbitrarily large number of results."]
                    pub fn page_size(mut self, value: i32) -> Self {
                        self.page_size = Some(value);
                        self
                    }
                    #[doc = "If supplied, this should be the value of next_page_token returned\nby an earlier call. This will cause the next page of results to\nbe returned."]
                    pub fn page_token(mut self, value: impl Into<String>) -> Self {
                        self.page_token = Some(value.into());
                        self
                    }
                    #[doc = "If specified, return only messages with timestamps >= start_time.\nThe default is the job creation time (i.e. beginning of messages)."]
                    pub fn start_time(mut self, value: impl Into<String>) -> Self {
                        self.start_time = Some(value.into());
                        self
                    }
                    #[doc = "OAuth access token."]
                    pub fn access_token(mut self, value: impl Into<String>) -> Self {
                        self.access_token = Some(value.into());
                        self
                    }
                    #[doc = "JSONP"]
                    pub fn callback(mut self, value: impl Into<String>) -> Self {
                        self.callback = Some(value.into());
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
                    #[doc = "Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters."]
                    pub fn quota_user(mut self, value: impl Into<String>) -> Self {
                        self.quota_user = Some(value.into());
                        self
                    }
                    #[doc = "Upload protocol for media (e.g. \"raw\", \"multipart\")."]
                    pub fn upload_protocol(mut self, value: impl Into<String>) -> Self {
                        self.upload_protocol = Some(value.into());
                        self
                    }
                    #[doc = "Legacy upload protocol for media (e.g. \"media\", \"multipart\")."]
                    pub fn upload_type(mut self, value: impl Into<String>) -> Self {
                        self.upload_type = Some(value.into());
                        self
                    }
                    #[doc = "V1 error format."]
                    pub fn xgafv(mut self, value: crate::params::Xgafv) -> Self {
                        self.xgafv = Some(value);
                        self
                    }
                    #[doc = r" Return an iterator that iterates over all `#prop_ident`. The"]
                    #[doc = r" items yielded by the iterator are chosen by the caller of this"]
                    #[doc = r" method and must implement `Deserialize` and `FieldSelector`. The"]
                    #[doc = r" populated fields in the yielded items will be determined by the"]
                    #[doc = r" `FieldSelector` implementation."]
                    pub fn iter_autoscaling_events<T>(
                        mut self,
                    ) -> crate::iter::PageItemIter<Self, T>
                    where
                        T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
                    {
                        let mut fields = concat!("nextPageToken,", "autoscalingEvents").to_owned();
                        let items_fields = T::field_selector();
                        if !items_fields.is_empty() {
                            fields.push_str("(");
                            fields.push_str(&items_fields);
                            fields.push_str(")");
                        }
                        self.fields = Some(fields);
                        crate::iter::PageItemIter::new(self, "autoscalingEvents")
                    }
                    #[doc = r" Return an iterator that iterates over all `#prop_ident`. The"]
                    #[doc = r" items yielded by the iterator are `#items_type`. The populated"]
                    #[doc = r" fields in `#items_type` will be the default fields populated by"]
                    #[doc = r" the server."]
                    pub fn iter_autoscaling_events_standard(
                        mut self,
                    ) -> crate::iter::PageItemIter<Self, crate::schemas::AutoscalingEvent>
                    {
                        self.fields =
                            Some(concat!("nextPageToken,", "autoscalingEvents").to_owned());
                        crate::iter::PageItemIter::new(self, "autoscalingEvents")
                    }
                    #[doc = r" Return an iterator that iterates over all `#prop_ident`. The"]
                    #[doc = r" items yielded by the iterator are `#items_type`. The populated"]
                    #[doc = r" fields in `#items_type` will be all fields available. This should"]
                    #[doc = r" primarily be used during developement and debugging as fetching"]
                    #[doc = r" all fields can be expensive both in bandwidth and server"]
                    #[doc = r" resources."]
                    pub fn iter_autoscaling_events_debug(
                        mut self,
                    ) -> crate::iter::PageItemIter<Self, crate::schemas::AutoscalingEvent>
                    {
                        self.fields =
                            Some(concat!("nextPageToken,", "autoscalingEvents", "(*)").to_owned());
                        crate::iter::PageItemIter::new(self, "autoscalingEvents")
                    }
                    #[doc = r" Return an iterator that iterates over all `#prop_ident`. The"]
                    #[doc = r" items yielded by the iterator are chosen by the caller of this"]
                    #[doc = r" method and must implement `Deserialize` and `FieldSelector`. The"]
                    #[doc = r" populated fields in the yielded items will be determined by the"]
                    #[doc = r" `FieldSelector` implementation."]
                    pub fn iter_job_messages<T>(mut self) -> crate::iter::PageItemIter<Self, T>
                    where
                        T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
                    {
                        let mut fields = concat!("nextPageToken,", "jobMessages").to_owned();
                        let items_fields = T::field_selector();
                        if !items_fields.is_empty() {
                            fields.push_str("(");
                            fields.push_str(&items_fields);
                            fields.push_str(")");
                        }
                        self.fields = Some(fields);
                        crate::iter::PageItemIter::new(self, "jobMessages")
                    }
                    #[doc = r" Return an iterator that iterates over all `#prop_ident`. The"]
                    #[doc = r" items yielded by the iterator are `#items_type`. The populated"]
                    #[doc = r" fields in `#items_type` will be the default fields populated by"]
                    #[doc = r" the server."]
                    pub fn iter_job_messages_standard(
                        mut self,
                    ) -> crate::iter::PageItemIter<Self, crate::schemas::JobMessage>
                    {
                        self.fields = Some(concat!("nextPageToken,", "jobMessages").to_owned());
                        crate::iter::PageItemIter::new(self, "jobMessages")
                    }
                    #[doc = r" Return an iterator that iterates over all `#prop_ident`. The"]
                    #[doc = r" items yielded by the iterator are `#items_type`. The populated"]
                    #[doc = r" fields in `#items_type` will be all fields available. This should"]
                    #[doc = r" primarily be used during developement and debugging as fetching"]
                    #[doc = r" all fields can be expensive both in bandwidth and server"]
                    #[doc = r" resources."]
                    pub fn iter_job_messages_debug(
                        mut self,
                    ) -> crate::iter::PageItemIter<Self, crate::schemas::JobMessage>
                    {
                        self.fields =
                            Some(concat!("nextPageToken,", "jobMessages", "(*)").to_owned());
                        crate::iter::PageItemIter::new(self, "jobMessages")
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
                    ) -> crate::iter::PageIter<Self, crate::schemas::ListJobMessagesResponse>
                    {
                        crate::iter::PageIter::new(self)
                    }
                    pub fn iter_debug(
                        mut self,
                    ) -> crate::iter::PageIter<Self, crate::schemas::ListJobMessagesResponse>
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
                    ) -> Result<crate::schemas::ListJobMessagesResponse, Box<dyn ::std::error::Error>>
                    {
                        self.execute_fields::<_, &str>(None)
                    }
                    #[doc = r" Execute the given operation. This will provide a `fields`"]
                    #[doc = r" selector of `*`. This will include every attribute of the"]
                    #[doc = r" response resource and should be limited to use during"]
                    #[doc = r" development or debugging."]
                    pub fn execute_debug(
                        self,
                    ) -> Result<crate::schemas::ListJobMessagesResponse, Box<dyn ::std::error::Error>>
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
                        let mut output = "https://dataflow.googleapis.com/".to_owned();
                        output.push_str("v1b3/projects/");
                        {
                            let var_as_str = &self.project_id;
                            output.extend(::percent_encoding::utf8_percent_encode(
                                &var_as_str,
                                crate::SIMPLE,
                            ));
                        }
                        output.push_str("/jobs/");
                        {
                            let var_as_str = &self.job_id;
                            output.extend(::percent_encoding::utf8_percent_encode(
                                &var_as_str,
                                crate::SIMPLE,
                            ));
                        }
                        output.push_str("/messages");
                        output
                    }
                    fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
                        let req = self.reqwest.request(::reqwest::Method::GET, path);
                        let req = req.query(&[("endTime", &self.end_time)]);
                        let req = req.query(&[("location", &self.location)]);
                        let req = req.query(&[("minimumImportance", &self.minimum_importance)]);
                        let req = req.query(&[("pageSize", &self.page_size)]);
                        let req = req.query(&[("pageToken", &self.page_token)]);
                        let req = req.query(&[("startTime", &self.start_time)]);
                        let req = req.query(&[("access_token", &self.access_token)]);
                        let req = req.query(&[("alt", &self.alt)]);
                        let req = req.query(&[("callback", &self.callback)]);
                        let req = req.query(&[("fields", &self.fields)]);
                        let req = req.query(&[("key", &self.key)]);
                        let req = req.query(&[("oauth_token", &self.oauth_token)]);
                        let req = req.query(&[("prettyPrint", &self.pretty_print)]);
                        let req = req.query(&[("quotaUser", &self.quota_user)]);
                        let req = req.query(&[("upload_protocol", &self.upload_protocol)]);
                        let req = req.query(&[("uploadType", &self.upload_type)]);
                        let req = req.query(&[("$.xgafv", &self.xgafv)]);
                        let mut auth = self.auth.lock().unwrap();
                        let fut =
                            auth.token(vec!["https://www.googleapis.com/auth/compute.readonly"]);
                        let mut runtime = ::tokio::runtime::Runtime::new().unwrap();
                        let token = runtime.block_on(fut).unwrap().access_token;
                        let req = req.bearer_auth(&token);
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
            pub mod work_items {
                pub mod params {}
                pub struct WorkItemsActions<'a, A> {
                    pub(crate) reqwest: &'a reqwest::Client,
                    pub(crate) auth: &'a std::sync::Mutex<A>,
                }
                impl<'a, A: yup_oauth2::GetToken> WorkItemsActions<'a, A> {
                    #[doc = "Leases a dataflow WorkItem to run."]
                    pub fn lease(
                        &self,
                        request: crate::schemas::LeaseWorkItemRequest,
                        project_id: impl Into<String>,
                        job_id: impl Into<String>,
                    ) -> LeaseRequestBuilder<A> {
                        LeaseRequestBuilder {
                            reqwest: &self.reqwest,
                            auth: &self.auth,
                            request,
                            access_token: None,
                            alt: None,
                            callback: None,
                            fields: None,
                            key: None,
                            oauth_token: None,
                            pretty_print: None,
                            quota_user: None,
                            upload_protocol: None,
                            upload_type: None,
                            xgafv: None,
                            project_id: project_id.into(),
                            job_id: job_id.into(),
                        }
                    }
                    #[doc = "Reports the status of dataflow WorkItems leased by a worker."]
                    pub fn report_status(
                        &self,
                        request: crate::schemas::ReportWorkItemStatusRequest,
                        project_id: impl Into<String>,
                        job_id: impl Into<String>,
                    ) -> ReportStatusRequestBuilder<A> {
                        ReportStatusRequestBuilder {
                            reqwest: &self.reqwest,
                            auth: &self.auth,
                            request,
                            access_token: None,
                            alt: None,
                            callback: None,
                            fields: None,
                            key: None,
                            oauth_token: None,
                            pretty_print: None,
                            quota_user: None,
                            upload_protocol: None,
                            upload_type: None,
                            xgafv: None,
                            project_id: project_id.into(),
                            job_id: job_id.into(),
                        }
                    }
                }
                #[derive(Debug, Clone)]
                pub struct LeaseRequestBuilder<'a, A> {
                    pub(crate) reqwest: &'a ::reqwest::Client,
                    pub(crate) auth: &'a ::std::sync::Mutex<A>,
                    request: crate::schemas::LeaseWorkItemRequest,
                    project_id: String,
                    job_id: String,
                    access_token: Option<String>,
                    alt: Option<crate::params::Alt>,
                    callback: Option<String>,
                    fields: Option<String>,
                    key: Option<String>,
                    oauth_token: Option<String>,
                    pretty_print: Option<bool>,
                    quota_user: Option<String>,
                    upload_protocol: Option<String>,
                    upload_type: Option<String>,
                    xgafv: Option<crate::params::Xgafv>,
                }
                impl<'a, A: yup_oauth2::GetToken> LeaseRequestBuilder<'a, A> {
                    #[doc = "OAuth access token."]
                    pub fn access_token(mut self, value: impl Into<String>) -> Self {
                        self.access_token = Some(value.into());
                        self
                    }
                    #[doc = "JSONP"]
                    pub fn callback(mut self, value: impl Into<String>) -> Self {
                        self.callback = Some(value.into());
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
                    #[doc = "Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters."]
                    pub fn quota_user(mut self, value: impl Into<String>) -> Self {
                        self.quota_user = Some(value.into());
                        self
                    }
                    #[doc = "Upload protocol for media (e.g. \"raw\", \"multipart\")."]
                    pub fn upload_protocol(mut self, value: impl Into<String>) -> Self {
                        self.upload_protocol = Some(value.into());
                        self
                    }
                    #[doc = "Legacy upload protocol for media (e.g. \"media\", \"multipart\")."]
                    pub fn upload_type(mut self, value: impl Into<String>) -> Self {
                        self.upload_type = Some(value.into());
                        self
                    }
                    #[doc = "V1 error format."]
                    pub fn xgafv(mut self, value: crate::params::Xgafv) -> Self {
                        self.xgafv = Some(value);
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
                    ) -> Result<crate::schemas::LeaseWorkItemResponse, Box<dyn ::std::error::Error>>
                    {
                        self.execute_fields::<_, &str>(None)
                    }
                    #[doc = r" Execute the given operation. This will provide a `fields`"]
                    #[doc = r" selector of `*`. This will include every attribute of the"]
                    #[doc = r" response resource and should be limited to use during"]
                    #[doc = r" development or debugging."]
                    pub fn execute_debug(
                        self,
                    ) -> Result<crate::schemas::LeaseWorkItemResponse, Box<dyn ::std::error::Error>>
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
                        let mut output = "https://dataflow.googleapis.com/".to_owned();
                        output.push_str("v1b3/projects/");
                        {
                            let var_as_str = &self.project_id;
                            output.extend(::percent_encoding::utf8_percent_encode(
                                &var_as_str,
                                crate::SIMPLE,
                            ));
                        }
                        output.push_str("/jobs/");
                        {
                            let var_as_str = &self.job_id;
                            output.extend(::percent_encoding::utf8_percent_encode(
                                &var_as_str,
                                crate::SIMPLE,
                            ));
                        }
                        output.push_str("/workItems:lease");
                        output
                    }
                    fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
                        let req = self.reqwest.request(::reqwest::Method::POST, path);
                        let req = req.query(&[("access_token", &self.access_token)]);
                        let req = req.query(&[("alt", &self.alt)]);
                        let req = req.query(&[("callback", &self.callback)]);
                        let req = req.query(&[("fields", &self.fields)]);
                        let req = req.query(&[("key", &self.key)]);
                        let req = req.query(&[("oauth_token", &self.oauth_token)]);
                        let req = req.query(&[("prettyPrint", &self.pretty_print)]);
                        let req = req.query(&[("quotaUser", &self.quota_user)]);
                        let req = req.query(&[("upload_protocol", &self.upload_protocol)]);
                        let req = req.query(&[("uploadType", &self.upload_type)]);
                        let req = req.query(&[("$.xgafv", &self.xgafv)]);
                        let mut auth = self.auth.lock().unwrap();
                        let fut =
                            auth.token(vec!["https://www.googleapis.com/auth/cloud-platform"]);
                        let mut runtime = ::tokio::runtime::Runtime::new().unwrap();
                        let token = runtime.block_on(fut).unwrap().access_token;
                        let req = req.bearer_auth(&token);
                        req
                    }
                }
                #[derive(Debug, Clone)]
                pub struct ReportStatusRequestBuilder<'a, A> {
                    pub(crate) reqwest: &'a ::reqwest::Client,
                    pub(crate) auth: &'a ::std::sync::Mutex<A>,
                    request: crate::schemas::ReportWorkItemStatusRequest,
                    project_id: String,
                    job_id: String,
                    access_token: Option<String>,
                    alt: Option<crate::params::Alt>,
                    callback: Option<String>,
                    fields: Option<String>,
                    key: Option<String>,
                    oauth_token: Option<String>,
                    pretty_print: Option<bool>,
                    quota_user: Option<String>,
                    upload_protocol: Option<String>,
                    upload_type: Option<String>,
                    xgafv: Option<crate::params::Xgafv>,
                }
                impl<'a, A: yup_oauth2::GetToken> ReportStatusRequestBuilder<'a, A> {
                    #[doc = "OAuth access token."]
                    pub fn access_token(mut self, value: impl Into<String>) -> Self {
                        self.access_token = Some(value.into());
                        self
                    }
                    #[doc = "JSONP"]
                    pub fn callback(mut self, value: impl Into<String>) -> Self {
                        self.callback = Some(value.into());
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
                    #[doc = "Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters."]
                    pub fn quota_user(mut self, value: impl Into<String>) -> Self {
                        self.quota_user = Some(value.into());
                        self
                    }
                    #[doc = "Upload protocol for media (e.g. \"raw\", \"multipart\")."]
                    pub fn upload_protocol(mut self, value: impl Into<String>) -> Self {
                        self.upload_protocol = Some(value.into());
                        self
                    }
                    #[doc = "Legacy upload protocol for media (e.g. \"media\", \"multipart\")."]
                    pub fn upload_type(mut self, value: impl Into<String>) -> Self {
                        self.upload_type = Some(value.into());
                        self
                    }
                    #[doc = "V1 error format."]
                    pub fn xgafv(mut self, value: crate::params::Xgafv) -> Self {
                        self.xgafv = Some(value);
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
                        crate::schemas::ReportWorkItemStatusResponse,
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
                        crate::schemas::ReportWorkItemStatusResponse,
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
                        let req = req.json(&self.request);
                        Ok(req.send()?.error_for_status()?.json()?)
                    }
                    fn _path(&self) -> String {
                        let mut output = "https://dataflow.googleapis.com/".to_owned();
                        output.push_str("v1b3/projects/");
                        {
                            let var_as_str = &self.project_id;
                            output.extend(::percent_encoding::utf8_percent_encode(
                                &var_as_str,
                                crate::SIMPLE,
                            ));
                        }
                        output.push_str("/jobs/");
                        {
                            let var_as_str = &self.job_id;
                            output.extend(::percent_encoding::utf8_percent_encode(
                                &var_as_str,
                                crate::SIMPLE,
                            ));
                        }
                        output.push_str("/workItems:reportStatus");
                        output
                    }
                    fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
                        let req = self.reqwest.request(::reqwest::Method::POST, path);
                        let req = req.query(&[("access_token", &self.access_token)]);
                        let req = req.query(&[("alt", &self.alt)]);
                        let req = req.query(&[("callback", &self.callback)]);
                        let req = req.query(&[("fields", &self.fields)]);
                        let req = req.query(&[("key", &self.key)]);
                        let req = req.query(&[("oauth_token", &self.oauth_token)]);
                        let req = req.query(&[("prettyPrint", &self.pretty_print)]);
                        let req = req.query(&[("quotaUser", &self.quota_user)]);
                        let req = req.query(&[("upload_protocol", &self.upload_protocol)]);
                        let req = req.query(&[("uploadType", &self.upload_type)]);
                        let req = req.query(&[("$.xgafv", &self.xgafv)]);
                        let mut auth = self.auth.lock().unwrap();
                        let fut =
                            auth.token(vec!["https://www.googleapis.com/auth/cloud-platform"]);
                        let mut runtime = ::tokio::runtime::Runtime::new().unwrap();
                        let token = runtime.block_on(fut).unwrap().access_token;
                        let req = req.bearer_auth(&token);
                        req
                    }
                }
            }
        }
        pub mod locations {
            pub mod params {}
            pub struct LocationsActions<'a, A> {
                pub(crate) reqwest: &'a reqwest::Client,
                pub(crate) auth: &'a std::sync::Mutex<A>,
            }
            impl<'a, A: yup_oauth2::GetToken> LocationsActions<'a, A> {
                #[doc = "Send a worker_message to the service."]
                pub fn worker_messages(
                    &self,
                    request: crate::schemas::SendWorkerMessagesRequest,
                    project_id: impl Into<String>,
                    location: impl Into<String>,
                ) -> WorkerMessagesRequestBuilder<A> {
                    WorkerMessagesRequestBuilder {
                        reqwest: &self.reqwest,
                        auth: &self.auth,
                        request,
                        access_token: None,
                        alt: None,
                        callback: None,
                        fields: None,
                        key: None,
                        oauth_token: None,
                        pretty_print: None,
                        quota_user: None,
                        upload_protocol: None,
                        upload_type: None,
                        xgafv: None,
                        project_id: project_id.into(),
                        location: location.into(),
                    }
                }
                #[doc = "Actions that can be performed on the jobs resource"]
                pub fn jobs(&self) -> crate::resources::projects::locations::jobs::JobsActions<A> {
                    crate::resources::projects::locations::jobs::JobsActions {
                        reqwest: &self.reqwest,
                        auth: &self.auth,
                    }
                }
                #[doc = "Actions that can be performed on the snapshots resource"]
                pub fn snapshots(
                    &self,
                ) -> crate::resources::projects::locations::snapshots::SnapshotsActions<A>
                {
                    crate::resources::projects::locations::snapshots::SnapshotsActions {
                        reqwest: &self.reqwest,
                        auth: &self.auth,
                    }
                }
                #[doc = "Actions that can be performed on the sql resource"]
                pub fn sql(&self) -> crate::resources::projects::locations::sql::SqlActions<A> {
                    crate::resources::projects::locations::sql::SqlActions {
                        reqwest: &self.reqwest,
                        auth: &self.auth,
                    }
                }
                #[doc = "Actions that can be performed on the templates resource"]
                pub fn templates(
                    &self,
                ) -> crate::resources::projects::locations::templates::TemplatesActions<A>
                {
                    crate::resources::projects::locations::templates::TemplatesActions {
                        reqwest: &self.reqwest,
                        auth: &self.auth,
                    }
                }
            }
            #[derive(Debug, Clone)]
            pub struct WorkerMessagesRequestBuilder<'a, A> {
                pub(crate) reqwest: &'a ::reqwest::Client,
                pub(crate) auth: &'a ::std::sync::Mutex<A>,
                request: crate::schemas::SendWorkerMessagesRequest,
                project_id: String,
                location: String,
                access_token: Option<String>,
                alt: Option<crate::params::Alt>,
                callback: Option<String>,
                fields: Option<String>,
                key: Option<String>,
                oauth_token: Option<String>,
                pretty_print: Option<bool>,
                quota_user: Option<String>,
                upload_protocol: Option<String>,
                upload_type: Option<String>,
                xgafv: Option<crate::params::Xgafv>,
            }
            impl<'a, A: yup_oauth2::GetToken> WorkerMessagesRequestBuilder<'a, A> {
                #[doc = "OAuth access token."]
                pub fn access_token(mut self, value: impl Into<String>) -> Self {
                    self.access_token = Some(value.into());
                    self
                }
                #[doc = "JSONP"]
                pub fn callback(mut self, value: impl Into<String>) -> Self {
                    self.callback = Some(value.into());
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
                #[doc = "Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters."]
                pub fn quota_user(mut self, value: impl Into<String>) -> Self {
                    self.quota_user = Some(value.into());
                    self
                }
                #[doc = "Upload protocol for media (e.g. \"raw\", \"multipart\")."]
                pub fn upload_protocol(mut self, value: impl Into<String>) -> Self {
                    self.upload_protocol = Some(value.into());
                    self
                }
                #[doc = "Legacy upload protocol for media (e.g. \"media\", \"multipart\")."]
                pub fn upload_type(mut self, value: impl Into<String>) -> Self {
                    self.upload_type = Some(value.into());
                    self
                }
                #[doc = "V1 error format."]
                pub fn xgafv(mut self, value: crate::params::Xgafv) -> Self {
                    self.xgafv = Some(value);
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
                ) -> Result<crate::schemas::SendWorkerMessagesResponse, Box<dyn ::std::error::Error>>
                {
                    self.execute_fields::<_, &str>(None)
                }
                #[doc = r" Execute the given operation. This will provide a `fields`"]
                #[doc = r" selector of `*`. This will include every attribute of the"]
                #[doc = r" response resource and should be limited to use during"]
                #[doc = r" development or debugging."]
                pub fn execute_debug(
                    self,
                ) -> Result<crate::schemas::SendWorkerMessagesResponse, Box<dyn ::std::error::Error>>
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
                    let mut output = "https://dataflow.googleapis.com/".to_owned();
                    output.push_str("v1b3/projects/");
                    {
                        let var_as_str = &self.project_id;
                        output.extend(::percent_encoding::utf8_percent_encode(
                            &var_as_str,
                            crate::SIMPLE,
                        ));
                    }
                    output.push_str("/locations/");
                    {
                        let var_as_str = &self.location;
                        output.extend(::percent_encoding::utf8_percent_encode(
                            &var_as_str,
                            crate::SIMPLE,
                        ));
                    }
                    output.push_str("/WorkerMessages");
                    output
                }
                fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
                    let req = self.reqwest.request(::reqwest::Method::POST, path);
                    let req = req.query(&[("access_token", &self.access_token)]);
                    let req = req.query(&[("alt", &self.alt)]);
                    let req = req.query(&[("callback", &self.callback)]);
                    let req = req.query(&[("fields", &self.fields)]);
                    let req = req.query(&[("key", &self.key)]);
                    let req = req.query(&[("oauth_token", &self.oauth_token)]);
                    let req = req.query(&[("prettyPrint", &self.pretty_print)]);
                    let req = req.query(&[("quotaUser", &self.quota_user)]);
                    let req = req.query(&[("upload_protocol", &self.upload_protocol)]);
                    let req = req.query(&[("uploadType", &self.upload_type)]);
                    let req = req.query(&[("$.xgafv", &self.xgafv)]);
                    let mut auth = self.auth.lock().unwrap();
                    let fut = auth.token(vec!["https://www.googleapis.com/auth/cloud-platform"]);
                    let mut runtime = ::tokio::runtime::Runtime::new().unwrap();
                    let token = runtime.block_on(fut).unwrap().access_token;
                    let req = req.bearer_auth(&token);
                    req
                }
            }
            pub mod jobs {
                pub mod params {
                    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
                    pub enum CreateView {
                        JobViewAll,
                        JobViewDescription,
                        JobViewSummary,
                        JobViewUnknown,
                    }
                    impl CreateView {
                        pub fn as_str(self) -> &'static str {
                            match self {
                                CreateView::JobViewAll => "JOB_VIEW_ALL",
                                CreateView::JobViewDescription => "JOB_VIEW_DESCRIPTION",
                                CreateView::JobViewSummary => "JOB_VIEW_SUMMARY",
                                CreateView::JobViewUnknown => "JOB_VIEW_UNKNOWN",
                            }
                        }
                    }
                    impl ::std::fmt::Display for CreateView {
                        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                            f.write_str(self.as_str())
                        }
                    }
                    impl ::serde::Serialize for CreateView {
                        fn serialize<S>(
                            &self,
                            serializer: S,
                        ) -> ::std::result::Result<S::Ok, S::Error>
                        where
                            S: ::serde::ser::Serializer,
                        {
                            serializer.serialize_str(self.as_str())
                        }
                    }
                    impl<'de> ::serde::Deserialize<'de> for CreateView {
                        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
                        where
                            D: ::serde::de::Deserializer<'de>,
                        {
                            let value: &'de str = <&str>::deserialize(deserializer)?;
                            Ok(match value {
                                "JOB_VIEW_ALL" => CreateView::JobViewAll,
                                "JOB_VIEW_DESCRIPTION" => CreateView::JobViewDescription,
                                "JOB_VIEW_SUMMARY" => CreateView::JobViewSummary,
                                "JOB_VIEW_UNKNOWN" => CreateView::JobViewUnknown,
                                _ => {
                                    return Err(::serde::de::Error::custom(format!(
                                        "invalid enum for #name: {}",
                                        value
                                    )))
                                }
                            })
                        }
                    }
                    impl ::field_selector::FieldSelector for CreateView {
                        fn field_selector_with_ident(ident: &str, selector: &mut String) {
                            match selector.chars().rev().nth(0) {
                                Some(',') | None => {}
                                _ => selector.push_str(","),
                            }
                            selector.push_str(ident);
                        }
                    }
                    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
                    pub enum GetView {
                        JobViewAll,
                        JobViewDescription,
                        JobViewSummary,
                        JobViewUnknown,
                    }
                    impl GetView {
                        pub fn as_str(self) -> &'static str {
                            match self {
                                GetView::JobViewAll => "JOB_VIEW_ALL",
                                GetView::JobViewDescription => "JOB_VIEW_DESCRIPTION",
                                GetView::JobViewSummary => "JOB_VIEW_SUMMARY",
                                GetView::JobViewUnknown => "JOB_VIEW_UNKNOWN",
                            }
                        }
                    }
                    impl ::std::fmt::Display for GetView {
                        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                            f.write_str(self.as_str())
                        }
                    }
                    impl ::serde::Serialize for GetView {
                        fn serialize<S>(
                            &self,
                            serializer: S,
                        ) -> ::std::result::Result<S::Ok, S::Error>
                        where
                            S: ::serde::ser::Serializer,
                        {
                            serializer.serialize_str(self.as_str())
                        }
                    }
                    impl<'de> ::serde::Deserialize<'de> for GetView {
                        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
                        where
                            D: ::serde::de::Deserializer<'de>,
                        {
                            let value: &'de str = <&str>::deserialize(deserializer)?;
                            Ok(match value {
                                "JOB_VIEW_ALL" => GetView::JobViewAll,
                                "JOB_VIEW_DESCRIPTION" => GetView::JobViewDescription,
                                "JOB_VIEW_SUMMARY" => GetView::JobViewSummary,
                                "JOB_VIEW_UNKNOWN" => GetView::JobViewUnknown,
                                _ => {
                                    return Err(::serde::de::Error::custom(format!(
                                        "invalid enum for #name: {}",
                                        value
                                    )))
                                }
                            })
                        }
                    }
                    impl ::field_selector::FieldSelector for GetView {
                        fn field_selector_with_ident(ident: &str, selector: &mut String) {
                            match selector.chars().rev().nth(0) {
                                Some(',') | None => {}
                                _ => selector.push_str(","),
                            }
                            selector.push_str(ident);
                        }
                    }
                    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
                    pub enum ListFilter {
                        Active,
                        All,
                        Terminated,
                        Unknown,
                    }
                    impl ListFilter {
                        pub fn as_str(self) -> &'static str {
                            match self {
                                ListFilter::Active => "ACTIVE",
                                ListFilter::All => "ALL",
                                ListFilter::Terminated => "TERMINATED",
                                ListFilter::Unknown => "UNKNOWN",
                            }
                        }
                    }
                    impl ::std::fmt::Display for ListFilter {
                        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                            f.write_str(self.as_str())
                        }
                    }
                    impl ::serde::Serialize for ListFilter {
                        fn serialize<S>(
                            &self,
                            serializer: S,
                        ) -> ::std::result::Result<S::Ok, S::Error>
                        where
                            S: ::serde::ser::Serializer,
                        {
                            serializer.serialize_str(self.as_str())
                        }
                    }
                    impl<'de> ::serde::Deserialize<'de> for ListFilter {
                        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
                        where
                            D: ::serde::de::Deserializer<'de>,
                        {
                            let value: &'de str = <&str>::deserialize(deserializer)?;
                            Ok(match value {
                                "ACTIVE" => ListFilter::Active,
                                "ALL" => ListFilter::All,
                                "TERMINATED" => ListFilter::Terminated,
                                "UNKNOWN" => ListFilter::Unknown,
                                _ => {
                                    return Err(::serde::de::Error::custom(format!(
                                        "invalid enum for #name: {}",
                                        value
                                    )))
                                }
                            })
                        }
                    }
                    impl ::field_selector::FieldSelector for ListFilter {
                        fn field_selector_with_ident(ident: &str, selector: &mut String) {
                            match selector.chars().rev().nth(0) {
                                Some(',') | None => {}
                                _ => selector.push_str(","),
                            }
                            selector.push_str(ident);
                        }
                    }
                    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
                    pub enum ListView {
                        JobViewAll,
                        JobViewDescription,
                        JobViewSummary,
                        JobViewUnknown,
                    }
                    impl ListView {
                        pub fn as_str(self) -> &'static str {
                            match self {
                                ListView::JobViewAll => "JOB_VIEW_ALL",
                                ListView::JobViewDescription => "JOB_VIEW_DESCRIPTION",
                                ListView::JobViewSummary => "JOB_VIEW_SUMMARY",
                                ListView::JobViewUnknown => "JOB_VIEW_UNKNOWN",
                            }
                        }
                    }
                    impl ::std::fmt::Display for ListView {
                        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                            f.write_str(self.as_str())
                        }
                    }
                    impl ::serde::Serialize for ListView {
                        fn serialize<S>(
                            &self,
                            serializer: S,
                        ) -> ::std::result::Result<S::Ok, S::Error>
                        where
                            S: ::serde::ser::Serializer,
                        {
                            serializer.serialize_str(self.as_str())
                        }
                    }
                    impl<'de> ::serde::Deserialize<'de> for ListView {
                        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
                        where
                            D: ::serde::de::Deserializer<'de>,
                        {
                            let value: &'de str = <&str>::deserialize(deserializer)?;
                            Ok(match value {
                                "JOB_VIEW_ALL" => ListView::JobViewAll,
                                "JOB_VIEW_DESCRIPTION" => ListView::JobViewDescription,
                                "JOB_VIEW_SUMMARY" => ListView::JobViewSummary,
                                "JOB_VIEW_UNKNOWN" => ListView::JobViewUnknown,
                                _ => {
                                    return Err(::serde::de::Error::custom(format!(
                                        "invalid enum for #name: {}",
                                        value
                                    )))
                                }
                            })
                        }
                    }
                    impl ::field_selector::FieldSelector for ListView {
                        fn field_selector_with_ident(ident: &str, selector: &mut String) {
                            match selector.chars().rev().nth(0) {
                                Some(',') | None => {}
                                _ => selector.push_str(","),
                            }
                            selector.push_str(ident);
                        }
                    }
                }
                pub struct JobsActions<'a, A> {
                    pub(crate) reqwest: &'a reqwest::Client,
                    pub(crate) auth: &'a std::sync::Mutex<A>,
                }
                impl<'a, A: yup_oauth2::GetToken> JobsActions<'a, A> {
                    #[doc = "Creates a Cloud Dataflow job.\n\nTo create a job, we recommend using `projects.locations.jobs.create` with a\n[regional endpoint]\n(https://cloud.google.com/dataflow/docs/concepts/regional-endpoints). Using\n`projects.jobs.create` is not recommended, as your job will always start\nin `us-central1`."]
                    pub fn create(
                        &self,
                        request: crate::schemas::Job,
                        project_id: impl Into<String>,
                        location: impl Into<String>,
                    ) -> CreateRequestBuilder<A> {
                        CreateRequestBuilder {
                            reqwest: &self.reqwest,
                            auth: &self.auth,
                            request,
                            access_token: None,
                            alt: None,
                            callback: None,
                            fields: None,
                            key: None,
                            oauth_token: None,
                            pretty_print: None,
                            quota_user: None,
                            upload_protocol: None,
                            upload_type: None,
                            xgafv: None,
                            project_id: project_id.into(),
                            location: location.into(),
                            replace_job_id: None,
                            view: None,
                        }
                    }
                    #[doc = "Gets the state of the specified Cloud Dataflow job.\n\nTo get the state of a job, we recommend using `projects.locations.jobs.get`\nwith a [regional endpoint]\n(https://cloud.google.com/dataflow/docs/concepts/regional-endpoints). Using\n`projects.jobs.get` is not recommended, as you can only get the state of\njobs that are running in `us-central1`."]
                    pub fn get(
                        &self,
                        project_id: impl Into<String>,
                        location: impl Into<String>,
                        job_id: impl Into<String>,
                    ) -> GetRequestBuilder<A> {
                        GetRequestBuilder {
                            reqwest: &self.reqwest,
                            auth: &self.auth,
                            access_token: None,
                            alt: None,
                            callback: None,
                            fields: None,
                            key: None,
                            oauth_token: None,
                            pretty_print: None,
                            quota_user: None,
                            upload_protocol: None,
                            upload_type: None,
                            xgafv: None,
                            project_id: project_id.into(),
                            location: location.into(),
                            job_id: job_id.into(),
                            view: None,
                        }
                    }
                    #[doc = "Request the job status.\n\nTo request the status of a job, we recommend using\n`projects.locations.jobs.getMetrics` with a [regional endpoint]\n(https://cloud.google.com/dataflow/docs/concepts/regional-endpoints). Using\n`projects.jobs.getMetrics` is not recommended, as you can only request the\nstatus of jobs that are running in `us-central1`."]
                    pub fn get_metrics(
                        &self,
                        project_id: impl Into<String>,
                        location: impl Into<String>,
                        job_id: impl Into<String>,
                    ) -> GetMetricsRequestBuilder<A> {
                        GetMetricsRequestBuilder {
                            reqwest: &self.reqwest,
                            auth: &self.auth,
                            access_token: None,
                            alt: None,
                            callback: None,
                            fields: None,
                            key: None,
                            oauth_token: None,
                            pretty_print: None,
                            quota_user: None,
                            upload_protocol: None,
                            upload_type: None,
                            xgafv: None,
                            project_id: project_id.into(),
                            location: location.into(),
                            job_id: job_id.into(),
                            start_time: None,
                        }
                    }
                    #[doc = "List the jobs of a project.\n\nTo list the jobs of a project in a region, we recommend using\n`projects.locations.jobs.get` with a [regional endpoint]\n(https://cloud.google.com/dataflow/docs/concepts/regional-endpoints). To\nlist the all jobs across all regions, use `projects.jobs.aggregated`. Using\n`projects.jobs.list` is not recommended, as you can only get the list of\njobs that are running in `us-central1`."]
                    pub fn list(
                        &self,
                        project_id: impl Into<String>,
                        location: impl Into<String>,
                    ) -> ListRequestBuilder<A> {
                        ListRequestBuilder {
                            reqwest: &self.reqwest,
                            auth: &self.auth,
                            access_token: None,
                            alt: None,
                            callback: None,
                            fields: None,
                            key: None,
                            oauth_token: None,
                            pretty_print: None,
                            quota_user: None,
                            upload_protocol: None,
                            upload_type: None,
                            xgafv: None,
                            project_id: project_id.into(),
                            location: location.into(),
                            filter: None,
                            page_size: None,
                            page_token: None,
                            view: None,
                        }
                    }
                    #[doc = "Snapshot the state of a streaming job."]
                    pub fn snapshot(
                        &self,
                        request: crate::schemas::SnapshotJobRequest,
                        project_id: impl Into<String>,
                        location: impl Into<String>,
                        job_id: impl Into<String>,
                    ) -> SnapshotRequestBuilder<A> {
                        SnapshotRequestBuilder {
                            reqwest: &self.reqwest,
                            auth: &self.auth,
                            request,
                            access_token: None,
                            alt: None,
                            callback: None,
                            fields: None,
                            key: None,
                            oauth_token: None,
                            pretty_print: None,
                            quota_user: None,
                            upload_protocol: None,
                            upload_type: None,
                            xgafv: None,
                            project_id: project_id.into(),
                            location: location.into(),
                            job_id: job_id.into(),
                        }
                    }
                    #[doc = "Updates the state of an existing Cloud Dataflow job.\n\nTo update the state of an existing job, we recommend using\n`projects.locations.jobs.update` with a [regional endpoint]\n(https://cloud.google.com/dataflow/docs/concepts/regional-endpoints). Using\n`projects.jobs.update` is not recommended, as you can only update the state\nof jobs that are running in `us-central1`."]
                    pub fn update(
                        &self,
                        request: crate::schemas::Job,
                        project_id: impl Into<String>,
                        location: impl Into<String>,
                        job_id: impl Into<String>,
                    ) -> UpdateRequestBuilder<A> {
                        UpdateRequestBuilder {
                            reqwest: &self.reqwest,
                            auth: &self.auth,
                            request,
                            access_token: None,
                            alt: None,
                            callback: None,
                            fields: None,
                            key: None,
                            oauth_token: None,
                            pretty_print: None,
                            quota_user: None,
                            upload_protocol: None,
                            upload_type: None,
                            xgafv: None,
                            project_id: project_id.into(),
                            location: location.into(),
                            job_id: job_id.into(),
                        }
                    }
                    #[doc = "Actions that can be performed on the debug resource"]
                    pub fn debug(
                        &self,
                    ) -> crate::resources::projects::locations::jobs::debug::DebugActions<A>
                    {
                        crate::resources::projects::locations::jobs::debug::DebugActions {
                            reqwest: &self.reqwest,
                            auth: &self.auth,
                        }
                    }
                    #[doc = "Actions that can be performed on the messages resource"]
                    pub fn messages(
                        &self,
                    ) -> crate::resources::projects::locations::jobs::messages::MessagesActions<A>
                    {
                        crate::resources::projects::locations::jobs::messages::MessagesActions {
                            reqwest: &self.reqwest,
                            auth: &self.auth,
                        }
                    }
                    #[doc = "Actions that can be performed on the snapshots resource"]
                    pub fn snapshots(
                        &self,
                    ) -> crate::resources::projects::locations::jobs::snapshots::SnapshotsActions<A>
                    {
                        crate::resources::projects::locations::jobs::snapshots::SnapshotsActions {
                            reqwest: &self.reqwest,
                            auth: &self.auth,
                        }
                    }
                    #[doc = "Actions that can be performed on the work_items resource"]
                    pub fn work_items(
                        &self,
                    ) -> crate::resources::projects::locations::jobs::work_items::WorkItemsActions<A>
                    {
                        crate::resources::projects::locations::jobs::work_items::WorkItemsActions {
                            reqwest: &self.reqwest,
                            auth: &self.auth,
                        }
                    }
                }
                #[derive(Debug, Clone)]
                pub struct CreateRequestBuilder<'a, A> {
                    pub(crate) reqwest: &'a ::reqwest::Client,
                    pub(crate) auth: &'a ::std::sync::Mutex<A>,
                    request: crate::schemas::Job,
                    project_id: String,
                    location: String,
                    replace_job_id: Option<String>,
                    view: Option<crate::resources::projects::locations::jobs::params::CreateView>,
                    access_token: Option<String>,
                    alt: Option<crate::params::Alt>,
                    callback: Option<String>,
                    fields: Option<String>,
                    key: Option<String>,
                    oauth_token: Option<String>,
                    pretty_print: Option<bool>,
                    quota_user: Option<String>,
                    upload_protocol: Option<String>,
                    upload_type: Option<String>,
                    xgafv: Option<crate::params::Xgafv>,
                }
                impl<'a, A: yup_oauth2::GetToken> CreateRequestBuilder<'a, A> {
                    #[doc = "Deprecated. This field is now in the Job message."]
                    pub fn replace_job_id(mut self, value: impl Into<String>) -> Self {
                        self.replace_job_id = Some(value.into());
                        self
                    }
                    #[doc = "The level of information requested in response."]
                    pub fn view(
                        mut self,
                        value: crate::resources::projects::locations::jobs::params::CreateView,
                    ) -> Self {
                        self.view = Some(value);
                        self
                    }
                    #[doc = "OAuth access token."]
                    pub fn access_token(mut self, value: impl Into<String>) -> Self {
                        self.access_token = Some(value.into());
                        self
                    }
                    #[doc = "JSONP"]
                    pub fn callback(mut self, value: impl Into<String>) -> Self {
                        self.callback = Some(value.into());
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
                    #[doc = "Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters."]
                    pub fn quota_user(mut self, value: impl Into<String>) -> Self {
                        self.quota_user = Some(value.into());
                        self
                    }
                    #[doc = "Upload protocol for media (e.g. \"raw\", \"multipart\")."]
                    pub fn upload_protocol(mut self, value: impl Into<String>) -> Self {
                        self.upload_protocol = Some(value.into());
                        self
                    }
                    #[doc = "Legacy upload protocol for media (e.g. \"media\", \"multipart\")."]
                    pub fn upload_type(mut self, value: impl Into<String>) -> Self {
                        self.upload_type = Some(value.into());
                        self
                    }
                    #[doc = "V1 error format."]
                    pub fn xgafv(mut self, value: crate::params::Xgafv) -> Self {
                        self.xgafv = Some(value);
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
                    ) -> Result<crate::schemas::Job, Box<dyn ::std::error::Error>>
                    {
                        self.execute_fields::<_, &str>(None)
                    }
                    #[doc = r" Execute the given operation. This will provide a `fields`"]
                    #[doc = r" selector of `*`. This will include every attribute of the"]
                    #[doc = r" response resource and should be limited to use during"]
                    #[doc = r" development or debugging."]
                    pub fn execute_debug(
                        self,
                    ) -> Result<crate::schemas::Job, Box<dyn ::std::error::Error>>
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
                        let mut output = "https://dataflow.googleapis.com/".to_owned();
                        output.push_str("v1b3/projects/");
                        {
                            let var_as_str = &self.project_id;
                            output.extend(::percent_encoding::utf8_percent_encode(
                                &var_as_str,
                                crate::SIMPLE,
                            ));
                        }
                        output.push_str("/locations/");
                        {
                            let var_as_str = &self.location;
                            output.extend(::percent_encoding::utf8_percent_encode(
                                &var_as_str,
                                crate::SIMPLE,
                            ));
                        }
                        output.push_str("/jobs");
                        output
                    }
                    fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
                        let req = self.reqwest.request(::reqwest::Method::POST, path);
                        let req = req.query(&[("replaceJobId", &self.replace_job_id)]);
                        let req = req.query(&[("view", &self.view)]);
                        let req = req.query(&[("access_token", &self.access_token)]);
                        let req = req.query(&[("alt", &self.alt)]);
                        let req = req.query(&[("callback", &self.callback)]);
                        let req = req.query(&[("fields", &self.fields)]);
                        let req = req.query(&[("key", &self.key)]);
                        let req = req.query(&[("oauth_token", &self.oauth_token)]);
                        let req = req.query(&[("prettyPrint", &self.pretty_print)]);
                        let req = req.query(&[("quotaUser", &self.quota_user)]);
                        let req = req.query(&[("upload_protocol", &self.upload_protocol)]);
                        let req = req.query(&[("uploadType", &self.upload_type)]);
                        let req = req.query(&[("$.xgafv", &self.xgafv)]);
                        let mut auth = self.auth.lock().unwrap();
                        let fut =
                            auth.token(vec!["https://www.googleapis.com/auth/cloud-platform"]);
                        let mut runtime = ::tokio::runtime::Runtime::new().unwrap();
                        let token = runtime.block_on(fut).unwrap().access_token;
                        let req = req.bearer_auth(&token);
                        req
                    }
                }
                #[derive(Debug, Clone)]
                pub struct GetRequestBuilder<'a, A> {
                    pub(crate) reqwest: &'a ::reqwest::Client,
                    pub(crate) auth: &'a ::std::sync::Mutex<A>,
                    project_id: String,
                    location: String,
                    job_id: String,
                    view: Option<crate::resources::projects::locations::jobs::params::GetView>,
                    access_token: Option<String>,
                    alt: Option<crate::params::Alt>,
                    callback: Option<String>,
                    fields: Option<String>,
                    key: Option<String>,
                    oauth_token: Option<String>,
                    pretty_print: Option<bool>,
                    quota_user: Option<String>,
                    upload_protocol: Option<String>,
                    upload_type: Option<String>,
                    xgafv: Option<crate::params::Xgafv>,
                }
                impl<'a, A: yup_oauth2::GetToken> GetRequestBuilder<'a, A> {
                    #[doc = "The level of information requested in response."]
                    pub fn view(
                        mut self,
                        value: crate::resources::projects::locations::jobs::params::GetView,
                    ) -> Self {
                        self.view = Some(value);
                        self
                    }
                    #[doc = "OAuth access token."]
                    pub fn access_token(mut self, value: impl Into<String>) -> Self {
                        self.access_token = Some(value.into());
                        self
                    }
                    #[doc = "JSONP"]
                    pub fn callback(mut self, value: impl Into<String>) -> Self {
                        self.callback = Some(value.into());
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
                    #[doc = "Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters."]
                    pub fn quota_user(mut self, value: impl Into<String>) -> Self {
                        self.quota_user = Some(value.into());
                        self
                    }
                    #[doc = "Upload protocol for media (e.g. \"raw\", \"multipart\")."]
                    pub fn upload_protocol(mut self, value: impl Into<String>) -> Self {
                        self.upload_protocol = Some(value.into());
                        self
                    }
                    #[doc = "Legacy upload protocol for media (e.g. \"media\", \"multipart\")."]
                    pub fn upload_type(mut self, value: impl Into<String>) -> Self {
                        self.upload_type = Some(value.into());
                        self
                    }
                    #[doc = "V1 error format."]
                    pub fn xgafv(mut self, value: crate::params::Xgafv) -> Self {
                        self.xgafv = Some(value);
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
                    ) -> Result<crate::schemas::Job, Box<dyn ::std::error::Error>>
                    {
                        self.execute_fields::<_, &str>(None)
                    }
                    #[doc = r" Execute the given operation. This will provide a `fields`"]
                    #[doc = r" selector of `*`. This will include every attribute of the"]
                    #[doc = r" response resource and should be limited to use during"]
                    #[doc = r" development or debugging."]
                    pub fn execute_debug(
                        self,
                    ) -> Result<crate::schemas::Job, Box<dyn ::std::error::Error>>
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
                        let mut output = "https://dataflow.googleapis.com/".to_owned();
                        output.push_str("v1b3/projects/");
                        {
                            let var_as_str = &self.project_id;
                            output.extend(::percent_encoding::utf8_percent_encode(
                                &var_as_str,
                                crate::SIMPLE,
                            ));
                        }
                        output.push_str("/locations/");
                        {
                            let var_as_str = &self.location;
                            output.extend(::percent_encoding::utf8_percent_encode(
                                &var_as_str,
                                crate::SIMPLE,
                            ));
                        }
                        output.push_str("/jobs/");
                        {
                            let var_as_str = &self.job_id;
                            output.extend(::percent_encoding::utf8_percent_encode(
                                &var_as_str,
                                crate::SIMPLE,
                            ));
                        }
                        output
                    }
                    fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
                        let req = self.reqwest.request(::reqwest::Method::GET, path);
                        let req = req.query(&[("view", &self.view)]);
                        let req = req.query(&[("access_token", &self.access_token)]);
                        let req = req.query(&[("alt", &self.alt)]);
                        let req = req.query(&[("callback", &self.callback)]);
                        let req = req.query(&[("fields", &self.fields)]);
                        let req = req.query(&[("key", &self.key)]);
                        let req = req.query(&[("oauth_token", &self.oauth_token)]);
                        let req = req.query(&[("prettyPrint", &self.pretty_print)]);
                        let req = req.query(&[("quotaUser", &self.quota_user)]);
                        let req = req.query(&[("upload_protocol", &self.upload_protocol)]);
                        let req = req.query(&[("uploadType", &self.upload_type)]);
                        let req = req.query(&[("$.xgafv", &self.xgafv)]);
                        let mut auth = self.auth.lock().unwrap();
                        let fut =
                            auth.token(vec!["https://www.googleapis.com/auth/compute.readonly"]);
                        let mut runtime = ::tokio::runtime::Runtime::new().unwrap();
                        let token = runtime.block_on(fut).unwrap().access_token;
                        let req = req.bearer_auth(&token);
                        req
                    }
                }
                #[derive(Debug, Clone)]
                pub struct GetMetricsRequestBuilder<'a, A> {
                    pub(crate) reqwest: &'a ::reqwest::Client,
                    pub(crate) auth: &'a ::std::sync::Mutex<A>,
                    project_id: String,
                    location: String,
                    job_id: String,
                    start_time: Option<String>,
                    access_token: Option<String>,
                    alt: Option<crate::params::Alt>,
                    callback: Option<String>,
                    fields: Option<String>,
                    key: Option<String>,
                    oauth_token: Option<String>,
                    pretty_print: Option<bool>,
                    quota_user: Option<String>,
                    upload_protocol: Option<String>,
                    upload_type: Option<String>,
                    xgafv: Option<crate::params::Xgafv>,
                }
                impl<'a, A: yup_oauth2::GetToken> GetMetricsRequestBuilder<'a, A> {
                    #[doc = "Return only metric data that has changed since this time.\nDefault is to return all information about all metrics for the job."]
                    pub fn start_time(mut self, value: impl Into<String>) -> Self {
                        self.start_time = Some(value.into());
                        self
                    }
                    #[doc = "OAuth access token."]
                    pub fn access_token(mut self, value: impl Into<String>) -> Self {
                        self.access_token = Some(value.into());
                        self
                    }
                    #[doc = "JSONP"]
                    pub fn callback(mut self, value: impl Into<String>) -> Self {
                        self.callback = Some(value.into());
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
                    #[doc = "Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters."]
                    pub fn quota_user(mut self, value: impl Into<String>) -> Self {
                        self.quota_user = Some(value.into());
                        self
                    }
                    #[doc = "Upload protocol for media (e.g. \"raw\", \"multipart\")."]
                    pub fn upload_protocol(mut self, value: impl Into<String>) -> Self {
                        self.upload_protocol = Some(value.into());
                        self
                    }
                    #[doc = "Legacy upload protocol for media (e.g. \"media\", \"multipart\")."]
                    pub fn upload_type(mut self, value: impl Into<String>) -> Self {
                        self.upload_type = Some(value.into());
                        self
                    }
                    #[doc = "V1 error format."]
                    pub fn xgafv(mut self, value: crate::params::Xgafv) -> Self {
                        self.xgafv = Some(value);
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
                    ) -> Result<crate::schemas::JobMetrics, Box<dyn ::std::error::Error>>
                    {
                        self.execute_fields::<_, &str>(None)
                    }
                    #[doc = r" Execute the given operation. This will provide a `fields`"]
                    #[doc = r" selector of `*`. This will include every attribute of the"]
                    #[doc = r" response resource and should be limited to use during"]
                    #[doc = r" development or debugging."]
                    pub fn execute_debug(
                        self,
                    ) -> Result<crate::schemas::JobMetrics, Box<dyn ::std::error::Error>>
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
                        let mut output = "https://dataflow.googleapis.com/".to_owned();
                        output.push_str("v1b3/projects/");
                        {
                            let var_as_str = &self.project_id;
                            output.extend(::percent_encoding::utf8_percent_encode(
                                &var_as_str,
                                crate::SIMPLE,
                            ));
                        }
                        output.push_str("/locations/");
                        {
                            let var_as_str = &self.location;
                            output.extend(::percent_encoding::utf8_percent_encode(
                                &var_as_str,
                                crate::SIMPLE,
                            ));
                        }
                        output.push_str("/jobs/");
                        {
                            let var_as_str = &self.job_id;
                            output.extend(::percent_encoding::utf8_percent_encode(
                                &var_as_str,
                                crate::SIMPLE,
                            ));
                        }
                        output.push_str("/metrics");
                        output
                    }
                    fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
                        let req = self.reqwest.request(::reqwest::Method::GET, path);
                        let req = req.query(&[("startTime", &self.start_time)]);
                        let req = req.query(&[("access_token", &self.access_token)]);
                        let req = req.query(&[("alt", &self.alt)]);
                        let req = req.query(&[("callback", &self.callback)]);
                        let req = req.query(&[("fields", &self.fields)]);
                        let req = req.query(&[("key", &self.key)]);
                        let req = req.query(&[("oauth_token", &self.oauth_token)]);
                        let req = req.query(&[("prettyPrint", &self.pretty_print)]);
                        let req = req.query(&[("quotaUser", &self.quota_user)]);
                        let req = req.query(&[("upload_protocol", &self.upload_protocol)]);
                        let req = req.query(&[("uploadType", &self.upload_type)]);
                        let req = req.query(&[("$.xgafv", &self.xgafv)]);
                        let mut auth = self.auth.lock().unwrap();
                        let fut =
                            auth.token(vec!["https://www.googleapis.com/auth/compute.readonly"]);
                        let mut runtime = ::tokio::runtime::Runtime::new().unwrap();
                        let token = runtime.block_on(fut).unwrap().access_token;
                        let req = req.bearer_auth(&token);
                        req
                    }
                }
                #[derive(Debug, Clone)]
                pub struct ListRequestBuilder<'a, A> {
                    pub(crate) reqwest: &'a ::reqwest::Client,
                    pub(crate) auth: &'a ::std::sync::Mutex<A>,
                    project_id: String,
                    location: String,
                    filter: Option<crate::resources::projects::locations::jobs::params::ListFilter>,
                    page_size: Option<i32>,
                    page_token: Option<String>,
                    view: Option<crate::resources::projects::locations::jobs::params::ListView>,
                    access_token: Option<String>,
                    alt: Option<crate::params::Alt>,
                    callback: Option<String>,
                    fields: Option<String>,
                    key: Option<String>,
                    oauth_token: Option<String>,
                    pretty_print: Option<bool>,
                    quota_user: Option<String>,
                    upload_protocol: Option<String>,
                    upload_type: Option<String>,
                    xgafv: Option<crate::params::Xgafv>,
                }
                impl<'a, A: yup_oauth2::GetToken> ListRequestBuilder<'a, A> {
                    #[doc = "The kind of filter to use."]
                    pub fn filter(
                        mut self,
                        value: crate::resources::projects::locations::jobs::params::ListFilter,
                    ) -> Self {
                        self.filter = Some(value);
                        self
                    }
                    #[doc = "If there are many jobs, limit response to at most this many.\nThe actual number of jobs returned will be the lesser of max_responses\nand an unspecified server-defined limit."]
                    pub fn page_size(mut self, value: i32) -> Self {
                        self.page_size = Some(value);
                        self
                    }
                    #[doc = "Set this to the 'next_page_token' field of a previous response\nto request additional results in a long list."]
                    pub fn page_token(mut self, value: impl Into<String>) -> Self {
                        self.page_token = Some(value.into());
                        self
                    }
                    #[doc = "Level of information requested in response. Default is `JOB_VIEW_SUMMARY`."]
                    pub fn view(
                        mut self,
                        value: crate::resources::projects::locations::jobs::params::ListView,
                    ) -> Self {
                        self.view = Some(value);
                        self
                    }
                    #[doc = "OAuth access token."]
                    pub fn access_token(mut self, value: impl Into<String>) -> Self {
                        self.access_token = Some(value.into());
                        self
                    }
                    #[doc = "JSONP"]
                    pub fn callback(mut self, value: impl Into<String>) -> Self {
                        self.callback = Some(value.into());
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
                    #[doc = "Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters."]
                    pub fn quota_user(mut self, value: impl Into<String>) -> Self {
                        self.quota_user = Some(value.into());
                        self
                    }
                    #[doc = "Upload protocol for media (e.g. \"raw\", \"multipart\")."]
                    pub fn upload_protocol(mut self, value: impl Into<String>) -> Self {
                        self.upload_protocol = Some(value.into());
                        self
                    }
                    #[doc = "Legacy upload protocol for media (e.g. \"media\", \"multipart\")."]
                    pub fn upload_type(mut self, value: impl Into<String>) -> Self {
                        self.upload_type = Some(value.into());
                        self
                    }
                    #[doc = "V1 error format."]
                    pub fn xgafv(mut self, value: crate::params::Xgafv) -> Self {
                        self.xgafv = Some(value);
                        self
                    }
                    #[doc = r" Return an iterator that iterates over all `#prop_ident`. The"]
                    #[doc = r" items yielded by the iterator are chosen by the caller of this"]
                    #[doc = r" method and must implement `Deserialize` and `FieldSelector`. The"]
                    #[doc = r" populated fields in the yielded items will be determined by the"]
                    #[doc = r" `FieldSelector` implementation."]
                    pub fn iter_failed_location<T>(mut self) -> crate::iter::PageItemIter<Self, T>
                    where
                        T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
                    {
                        let mut fields = concat!("nextPageToken,", "failedLocation").to_owned();
                        let items_fields = T::field_selector();
                        if !items_fields.is_empty() {
                            fields.push_str("(");
                            fields.push_str(&items_fields);
                            fields.push_str(")");
                        }
                        self.fields = Some(fields);
                        crate::iter::PageItemIter::new(self, "failedLocation")
                    }
                    #[doc = r" Return an iterator that iterates over all `#prop_ident`. The"]
                    #[doc = r" items yielded by the iterator are `#items_type`. The populated"]
                    #[doc = r" fields in `#items_type` will be the default fields populated by"]
                    #[doc = r" the server."]
                    pub fn iter_failed_location_standard(
                        mut self,
                    ) -> crate::iter::PageItemIter<Self, crate::schemas::FailedLocation>
                    {
                        self.fields = Some(concat!("nextPageToken,", "failedLocation").to_owned());
                        crate::iter::PageItemIter::new(self, "failedLocation")
                    }
                    #[doc = r" Return an iterator that iterates over all `#prop_ident`. The"]
                    #[doc = r" items yielded by the iterator are `#items_type`. The populated"]
                    #[doc = r" fields in `#items_type` will be all fields available. This should"]
                    #[doc = r" primarily be used during developement and debugging as fetching"]
                    #[doc = r" all fields can be expensive both in bandwidth and server"]
                    #[doc = r" resources."]
                    pub fn iter_failed_location_debug(
                        mut self,
                    ) -> crate::iter::PageItemIter<Self, crate::schemas::FailedLocation>
                    {
                        self.fields =
                            Some(concat!("nextPageToken,", "failedLocation", "(*)").to_owned());
                        crate::iter::PageItemIter::new(self, "failedLocation")
                    }
                    #[doc = r" Return an iterator that iterates over all `#prop_ident`. The"]
                    #[doc = r" items yielded by the iterator are chosen by the caller of this"]
                    #[doc = r" method and must implement `Deserialize` and `FieldSelector`. The"]
                    #[doc = r" populated fields in the yielded items will be determined by the"]
                    #[doc = r" `FieldSelector` implementation."]
                    pub fn iter_jobs<T>(mut self) -> crate::iter::PageItemIter<Self, T>
                    where
                        T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
                    {
                        let mut fields = concat!("nextPageToken,", "jobs").to_owned();
                        let items_fields = T::field_selector();
                        if !items_fields.is_empty() {
                            fields.push_str("(");
                            fields.push_str(&items_fields);
                            fields.push_str(")");
                        }
                        self.fields = Some(fields);
                        crate::iter::PageItemIter::new(self, "jobs")
                    }
                    #[doc = r" Return an iterator that iterates over all `#prop_ident`. The"]
                    #[doc = r" items yielded by the iterator are `#items_type`. The populated"]
                    #[doc = r" fields in `#items_type` will be the default fields populated by"]
                    #[doc = r" the server."]
                    pub fn iter_jobs_standard(
                        mut self,
                    ) -> crate::iter::PageItemIter<Self, crate::schemas::Job> {
                        self.fields = Some(concat!("nextPageToken,", "jobs").to_owned());
                        crate::iter::PageItemIter::new(self, "jobs")
                    }
                    #[doc = r" Return an iterator that iterates over all `#prop_ident`. The"]
                    #[doc = r" items yielded by the iterator are `#items_type`. The populated"]
                    #[doc = r" fields in `#items_type` will be all fields available. This should"]
                    #[doc = r" primarily be used during developement and debugging as fetching"]
                    #[doc = r" all fields can be expensive both in bandwidth and server"]
                    #[doc = r" resources."]
                    pub fn iter_jobs_debug(
                        mut self,
                    ) -> crate::iter::PageItemIter<Self, crate::schemas::Job> {
                        self.fields = Some(concat!("nextPageToken,", "jobs", "(*)").to_owned());
                        crate::iter::PageItemIter::new(self, "jobs")
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
                    ) -> crate::iter::PageIter<Self, crate::schemas::ListJobsResponse>
                    {
                        crate::iter::PageIter::new(self)
                    }
                    pub fn iter_debug(
                        mut self,
                    ) -> crate::iter::PageIter<Self, crate::schemas::ListJobsResponse>
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
                    ) -> Result<crate::schemas::ListJobsResponse, Box<dyn ::std::error::Error>>
                    {
                        self.execute_fields::<_, &str>(None)
                    }
                    #[doc = r" Execute the given operation. This will provide a `fields`"]
                    #[doc = r" selector of `*`. This will include every attribute of the"]
                    #[doc = r" response resource and should be limited to use during"]
                    #[doc = r" development or debugging."]
                    pub fn execute_debug(
                        self,
                    ) -> Result<crate::schemas::ListJobsResponse, Box<dyn ::std::error::Error>>
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
                        let mut output = "https://dataflow.googleapis.com/".to_owned();
                        output.push_str("v1b3/projects/");
                        {
                            let var_as_str = &self.project_id;
                            output.extend(::percent_encoding::utf8_percent_encode(
                                &var_as_str,
                                crate::SIMPLE,
                            ));
                        }
                        output.push_str("/locations/");
                        {
                            let var_as_str = &self.location;
                            output.extend(::percent_encoding::utf8_percent_encode(
                                &var_as_str,
                                crate::SIMPLE,
                            ));
                        }
                        output.push_str("/jobs");
                        output
                    }
                    fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
                        let req = self.reqwest.request(::reqwest::Method::GET, path);
                        let req = req.query(&[("filter", &self.filter)]);
                        let req = req.query(&[("pageSize", &self.page_size)]);
                        let req = req.query(&[("pageToken", &self.page_token)]);
                        let req = req.query(&[("view", &self.view)]);
                        let req = req.query(&[("access_token", &self.access_token)]);
                        let req = req.query(&[("alt", &self.alt)]);
                        let req = req.query(&[("callback", &self.callback)]);
                        let req = req.query(&[("fields", &self.fields)]);
                        let req = req.query(&[("key", &self.key)]);
                        let req = req.query(&[("oauth_token", &self.oauth_token)]);
                        let req = req.query(&[("prettyPrint", &self.pretty_print)]);
                        let req = req.query(&[("quotaUser", &self.quota_user)]);
                        let req = req.query(&[("upload_protocol", &self.upload_protocol)]);
                        let req = req.query(&[("uploadType", &self.upload_type)]);
                        let req = req.query(&[("$.xgafv", &self.xgafv)]);
                        let mut auth = self.auth.lock().unwrap();
                        let fut =
                            auth.token(vec!["https://www.googleapis.com/auth/compute.readonly"]);
                        let mut runtime = ::tokio::runtime::Runtime::new().unwrap();
                        let token = runtime.block_on(fut).unwrap().access_token;
                        let req = req.bearer_auth(&token);
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
                #[derive(Debug, Clone)]
                pub struct SnapshotRequestBuilder<'a, A> {
                    pub(crate) reqwest: &'a ::reqwest::Client,
                    pub(crate) auth: &'a ::std::sync::Mutex<A>,
                    request: crate::schemas::SnapshotJobRequest,
                    project_id: String,
                    location: String,
                    job_id: String,
                    access_token: Option<String>,
                    alt: Option<crate::params::Alt>,
                    callback: Option<String>,
                    fields: Option<String>,
                    key: Option<String>,
                    oauth_token: Option<String>,
                    pretty_print: Option<bool>,
                    quota_user: Option<String>,
                    upload_protocol: Option<String>,
                    upload_type: Option<String>,
                    xgafv: Option<crate::params::Xgafv>,
                }
                impl<'a, A: yup_oauth2::GetToken> SnapshotRequestBuilder<'a, A> {
                    #[doc = "OAuth access token."]
                    pub fn access_token(mut self, value: impl Into<String>) -> Self {
                        self.access_token = Some(value.into());
                        self
                    }
                    #[doc = "JSONP"]
                    pub fn callback(mut self, value: impl Into<String>) -> Self {
                        self.callback = Some(value.into());
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
                    #[doc = "Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters."]
                    pub fn quota_user(mut self, value: impl Into<String>) -> Self {
                        self.quota_user = Some(value.into());
                        self
                    }
                    #[doc = "Upload protocol for media (e.g. \"raw\", \"multipart\")."]
                    pub fn upload_protocol(mut self, value: impl Into<String>) -> Self {
                        self.upload_protocol = Some(value.into());
                        self
                    }
                    #[doc = "Legacy upload protocol for media (e.g. \"media\", \"multipart\")."]
                    pub fn upload_type(mut self, value: impl Into<String>) -> Self {
                        self.upload_type = Some(value.into());
                        self
                    }
                    #[doc = "V1 error format."]
                    pub fn xgafv(mut self, value: crate::params::Xgafv) -> Self {
                        self.xgafv = Some(value);
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
                    ) -> Result<crate::schemas::Snapshot, Box<dyn ::std::error::Error>>
                    {
                        self.execute_fields::<_, &str>(None)
                    }
                    #[doc = r" Execute the given operation. This will provide a `fields`"]
                    #[doc = r" selector of `*`. This will include every attribute of the"]
                    #[doc = r" response resource and should be limited to use during"]
                    #[doc = r" development or debugging."]
                    pub fn execute_debug(
                        self,
                    ) -> Result<crate::schemas::Snapshot, Box<dyn ::std::error::Error>>
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
                        let mut output = "https://dataflow.googleapis.com/".to_owned();
                        output.push_str("v1b3/projects/");
                        {
                            let var_as_str = &self.project_id;
                            output.extend(::percent_encoding::utf8_percent_encode(
                                &var_as_str,
                                crate::SIMPLE,
                            ));
                        }
                        output.push_str("/locations/");
                        {
                            let var_as_str = &self.location;
                            output.extend(::percent_encoding::utf8_percent_encode(
                                &var_as_str,
                                crate::SIMPLE,
                            ));
                        }
                        output.push_str("/jobs/");
                        {
                            let var_as_str = &self.job_id;
                            output.extend(::percent_encoding::utf8_percent_encode(
                                &var_as_str,
                                crate::SIMPLE,
                            ));
                        }
                        output.push_str(":snapshot");
                        output
                    }
                    fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
                        let req = self.reqwest.request(::reqwest::Method::POST, path);
                        let req = req.query(&[("access_token", &self.access_token)]);
                        let req = req.query(&[("alt", &self.alt)]);
                        let req = req.query(&[("callback", &self.callback)]);
                        let req = req.query(&[("fields", &self.fields)]);
                        let req = req.query(&[("key", &self.key)]);
                        let req = req.query(&[("oauth_token", &self.oauth_token)]);
                        let req = req.query(&[("prettyPrint", &self.pretty_print)]);
                        let req = req.query(&[("quotaUser", &self.quota_user)]);
                        let req = req.query(&[("upload_protocol", &self.upload_protocol)]);
                        let req = req.query(&[("uploadType", &self.upload_type)]);
                        let req = req.query(&[("$.xgafv", &self.xgafv)]);
                        let mut auth = self.auth.lock().unwrap();
                        let fut =
                            auth.token(vec!["https://www.googleapis.com/auth/cloud-platform"]);
                        let mut runtime = ::tokio::runtime::Runtime::new().unwrap();
                        let token = runtime.block_on(fut).unwrap().access_token;
                        let req = req.bearer_auth(&token);
                        req
                    }
                }
                #[derive(Debug, Clone)]
                pub struct UpdateRequestBuilder<'a, A> {
                    pub(crate) reqwest: &'a ::reqwest::Client,
                    pub(crate) auth: &'a ::std::sync::Mutex<A>,
                    request: crate::schemas::Job,
                    project_id: String,
                    location: String,
                    job_id: String,
                    access_token: Option<String>,
                    alt: Option<crate::params::Alt>,
                    callback: Option<String>,
                    fields: Option<String>,
                    key: Option<String>,
                    oauth_token: Option<String>,
                    pretty_print: Option<bool>,
                    quota_user: Option<String>,
                    upload_protocol: Option<String>,
                    upload_type: Option<String>,
                    xgafv: Option<crate::params::Xgafv>,
                }
                impl<'a, A: yup_oauth2::GetToken> UpdateRequestBuilder<'a, A> {
                    #[doc = "OAuth access token."]
                    pub fn access_token(mut self, value: impl Into<String>) -> Self {
                        self.access_token = Some(value.into());
                        self
                    }
                    #[doc = "JSONP"]
                    pub fn callback(mut self, value: impl Into<String>) -> Self {
                        self.callback = Some(value.into());
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
                    #[doc = "Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters."]
                    pub fn quota_user(mut self, value: impl Into<String>) -> Self {
                        self.quota_user = Some(value.into());
                        self
                    }
                    #[doc = "Upload protocol for media (e.g. \"raw\", \"multipart\")."]
                    pub fn upload_protocol(mut self, value: impl Into<String>) -> Self {
                        self.upload_protocol = Some(value.into());
                        self
                    }
                    #[doc = "Legacy upload protocol for media (e.g. \"media\", \"multipart\")."]
                    pub fn upload_type(mut self, value: impl Into<String>) -> Self {
                        self.upload_type = Some(value.into());
                        self
                    }
                    #[doc = "V1 error format."]
                    pub fn xgafv(mut self, value: crate::params::Xgafv) -> Self {
                        self.xgafv = Some(value);
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
                    ) -> Result<crate::schemas::Job, Box<dyn ::std::error::Error>>
                    {
                        self.execute_fields::<_, &str>(None)
                    }
                    #[doc = r" Execute the given operation. This will provide a `fields`"]
                    #[doc = r" selector of `*`. This will include every attribute of the"]
                    #[doc = r" response resource and should be limited to use during"]
                    #[doc = r" development or debugging."]
                    pub fn execute_debug(
                        self,
                    ) -> Result<crate::schemas::Job, Box<dyn ::std::error::Error>>
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
                        let mut output = "https://dataflow.googleapis.com/".to_owned();
                        output.push_str("v1b3/projects/");
                        {
                            let var_as_str = &self.project_id;
                            output.extend(::percent_encoding::utf8_percent_encode(
                                &var_as_str,
                                crate::SIMPLE,
                            ));
                        }
                        output.push_str("/locations/");
                        {
                            let var_as_str = &self.location;
                            output.extend(::percent_encoding::utf8_percent_encode(
                                &var_as_str,
                                crate::SIMPLE,
                            ));
                        }
                        output.push_str("/jobs/");
                        {
                            let var_as_str = &self.job_id;
                            output.extend(::percent_encoding::utf8_percent_encode(
                                &var_as_str,
                                crate::SIMPLE,
                            ));
                        }
                        output
                    }
                    fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
                        let req = self.reqwest.request(::reqwest::Method::PUT, path);
                        let req = req.query(&[("access_token", &self.access_token)]);
                        let req = req.query(&[("alt", &self.alt)]);
                        let req = req.query(&[("callback", &self.callback)]);
                        let req = req.query(&[("fields", &self.fields)]);
                        let req = req.query(&[("key", &self.key)]);
                        let req = req.query(&[("oauth_token", &self.oauth_token)]);
                        let req = req.query(&[("prettyPrint", &self.pretty_print)]);
                        let req = req.query(&[("quotaUser", &self.quota_user)]);
                        let req = req.query(&[("upload_protocol", &self.upload_protocol)]);
                        let req = req.query(&[("uploadType", &self.upload_type)]);
                        let req = req.query(&[("$.xgafv", &self.xgafv)]);
                        let mut auth = self.auth.lock().unwrap();
                        let fut =
                            auth.token(vec!["https://www.googleapis.com/auth/cloud-platform"]);
                        let mut runtime = ::tokio::runtime::Runtime::new().unwrap();
                        let token = runtime.block_on(fut).unwrap().access_token;
                        let req = req.bearer_auth(&token);
                        req
                    }
                }
                pub mod debug {
                    pub mod params {}
                    pub struct DebugActions<'a, A> {
                        pub(crate) reqwest: &'a reqwest::Client,
                        pub(crate) auth: &'a std::sync::Mutex<A>,
                    }
                    impl<'a, A: yup_oauth2::GetToken> DebugActions<'a, A> {
                        #[doc = "Get encoded debug configuration for component. Not cacheable."]
                        pub fn get_config(
                            &self,
                            request: crate::schemas::GetDebugConfigRequest,
                            project_id: impl Into<String>,
                            location: impl Into<String>,
                            job_id: impl Into<String>,
                        ) -> GetConfigRequestBuilder<A> {
                            GetConfigRequestBuilder {
                                reqwest: &self.reqwest,
                                auth: &self.auth,
                                request,
                                access_token: None,
                                alt: None,
                                callback: None,
                                fields: None,
                                key: None,
                                oauth_token: None,
                                pretty_print: None,
                                quota_user: None,
                                upload_protocol: None,
                                upload_type: None,
                                xgafv: None,
                                project_id: project_id.into(),
                                location: location.into(),
                                job_id: job_id.into(),
                            }
                        }
                        #[doc = "Send encoded debug capture data for component."]
                        pub fn send_capture(
                            &self,
                            request: crate::schemas::SendDebugCaptureRequest,
                            project_id: impl Into<String>,
                            location: impl Into<String>,
                            job_id: impl Into<String>,
                        ) -> SendCaptureRequestBuilder<A> {
                            SendCaptureRequestBuilder {
                                reqwest: &self.reqwest,
                                auth: &self.auth,
                                request,
                                access_token: None,
                                alt: None,
                                callback: None,
                                fields: None,
                                key: None,
                                oauth_token: None,
                                pretty_print: None,
                                quota_user: None,
                                upload_protocol: None,
                                upload_type: None,
                                xgafv: None,
                                project_id: project_id.into(),
                                location: location.into(),
                                job_id: job_id.into(),
                            }
                        }
                    }
                    #[derive(Debug, Clone)]
                    pub struct GetConfigRequestBuilder<'a, A> {
                        pub(crate) reqwest: &'a ::reqwest::Client,
                        pub(crate) auth: &'a ::std::sync::Mutex<A>,
                        request: crate::schemas::GetDebugConfigRequest,
                        project_id: String,
                        location: String,
                        job_id: String,
                        access_token: Option<String>,
                        alt: Option<crate::params::Alt>,
                        callback: Option<String>,
                        fields: Option<String>,
                        key: Option<String>,
                        oauth_token: Option<String>,
                        pretty_print: Option<bool>,
                        quota_user: Option<String>,
                        upload_protocol: Option<String>,
                        upload_type: Option<String>,
                        xgafv: Option<crate::params::Xgafv>,
                    }
                    impl<'a, A: yup_oauth2::GetToken> GetConfigRequestBuilder<'a, A> {
                        #[doc = "OAuth access token."]
                        pub fn access_token(mut self, value: impl Into<String>) -> Self {
                            self.access_token = Some(value.into());
                            self
                        }
                        #[doc = "JSONP"]
                        pub fn callback(mut self, value: impl Into<String>) -> Self {
                            self.callback = Some(value.into());
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
                        #[doc = "Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters."]
                        pub fn quota_user(mut self, value: impl Into<String>) -> Self {
                            self.quota_user = Some(value.into());
                            self
                        }
                        #[doc = "Upload protocol for media (e.g. \"raw\", \"multipart\")."]
                        pub fn upload_protocol(mut self, value: impl Into<String>) -> Self {
                            self.upload_protocol = Some(value.into());
                            self
                        }
                        #[doc = "Legacy upload protocol for media (e.g. \"media\", \"multipart\")."]
                        pub fn upload_type(mut self, value: impl Into<String>) -> Self {
                            self.upload_type = Some(value.into());
                            self
                        }
                        #[doc = "V1 error format."]
                        pub fn xgafv(mut self, value: crate::params::Xgafv) -> Self {
                            self.xgafv = Some(value);
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
                            crate::schemas::GetDebugConfigResponse,
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
                            crate::schemas::GetDebugConfigResponse,
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
                            let req = req.json(&self.request);
                            Ok(req.send()?.error_for_status()?.json()?)
                        }
                        fn _path(&self) -> String {
                            let mut output = "https://dataflow.googleapis.com/".to_owned();
                            output.push_str("v1b3/projects/");
                            {
                                let var_as_str = &self.project_id;
                                output.extend(::percent_encoding::utf8_percent_encode(
                                    &var_as_str,
                                    crate::SIMPLE,
                                ));
                            }
                            output.push_str("/locations/");
                            {
                                let var_as_str = &self.location;
                                output.extend(::percent_encoding::utf8_percent_encode(
                                    &var_as_str,
                                    crate::SIMPLE,
                                ));
                            }
                            output.push_str("/jobs/");
                            {
                                let var_as_str = &self.job_id;
                                output.extend(::percent_encoding::utf8_percent_encode(
                                    &var_as_str,
                                    crate::SIMPLE,
                                ));
                            }
                            output.push_str("/debug/getConfig");
                            output
                        }
                        fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
                            let req = self.reqwest.request(::reqwest::Method::POST, path);
                            let req = req.query(&[("access_token", &self.access_token)]);
                            let req = req.query(&[("alt", &self.alt)]);
                            let req = req.query(&[("callback", &self.callback)]);
                            let req = req.query(&[("fields", &self.fields)]);
                            let req = req.query(&[("key", &self.key)]);
                            let req = req.query(&[("oauth_token", &self.oauth_token)]);
                            let req = req.query(&[("prettyPrint", &self.pretty_print)]);
                            let req = req.query(&[("quotaUser", &self.quota_user)]);
                            let req = req.query(&[("upload_protocol", &self.upload_protocol)]);
                            let req = req.query(&[("uploadType", &self.upload_type)]);
                            let req = req.query(&[("$.xgafv", &self.xgafv)]);
                            let mut auth = self.auth.lock().unwrap();
                            let fut =
                                auth.token(vec!["https://www.googleapis.com/auth/cloud-platform"]);
                            let mut runtime = ::tokio::runtime::Runtime::new().unwrap();
                            let token = runtime.block_on(fut).unwrap().access_token;
                            let req = req.bearer_auth(&token);
                            req
                        }
                    }
                    #[derive(Debug, Clone)]
                    pub struct SendCaptureRequestBuilder<'a, A> {
                        pub(crate) reqwest: &'a ::reqwest::Client,
                        pub(crate) auth: &'a ::std::sync::Mutex<A>,
                        request: crate::schemas::SendDebugCaptureRequest,
                        project_id: String,
                        location: String,
                        job_id: String,
                        access_token: Option<String>,
                        alt: Option<crate::params::Alt>,
                        callback: Option<String>,
                        fields: Option<String>,
                        key: Option<String>,
                        oauth_token: Option<String>,
                        pretty_print: Option<bool>,
                        quota_user: Option<String>,
                        upload_protocol: Option<String>,
                        upload_type: Option<String>,
                        xgafv: Option<crate::params::Xgafv>,
                    }
                    impl<'a, A: yup_oauth2::GetToken> SendCaptureRequestBuilder<'a, A> {
                        #[doc = "OAuth access token."]
                        pub fn access_token(mut self, value: impl Into<String>) -> Self {
                            self.access_token = Some(value.into());
                            self
                        }
                        #[doc = "JSONP"]
                        pub fn callback(mut self, value: impl Into<String>) -> Self {
                            self.callback = Some(value.into());
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
                        #[doc = "Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters."]
                        pub fn quota_user(mut self, value: impl Into<String>) -> Self {
                            self.quota_user = Some(value.into());
                            self
                        }
                        #[doc = "Upload protocol for media (e.g. \"raw\", \"multipart\")."]
                        pub fn upload_protocol(mut self, value: impl Into<String>) -> Self {
                            self.upload_protocol = Some(value.into());
                            self
                        }
                        #[doc = "Legacy upload protocol for media (e.g. \"media\", \"multipart\")."]
                        pub fn upload_type(mut self, value: impl Into<String>) -> Self {
                            self.upload_type = Some(value.into());
                            self
                        }
                        #[doc = "V1 error format."]
                        pub fn xgafv(mut self, value: crate::params::Xgafv) -> Self {
                            self.xgafv = Some(value);
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
                            crate::schemas::SendDebugCaptureResponse,
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
                            crate::schemas::SendDebugCaptureResponse,
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
                            let req = req.json(&self.request);
                            Ok(req.send()?.error_for_status()?.json()?)
                        }
                        fn _path(&self) -> String {
                            let mut output = "https://dataflow.googleapis.com/".to_owned();
                            output.push_str("v1b3/projects/");
                            {
                                let var_as_str = &self.project_id;
                                output.extend(::percent_encoding::utf8_percent_encode(
                                    &var_as_str,
                                    crate::SIMPLE,
                                ));
                            }
                            output.push_str("/locations/");
                            {
                                let var_as_str = &self.location;
                                output.extend(::percent_encoding::utf8_percent_encode(
                                    &var_as_str,
                                    crate::SIMPLE,
                                ));
                            }
                            output.push_str("/jobs/");
                            {
                                let var_as_str = &self.job_id;
                                output.extend(::percent_encoding::utf8_percent_encode(
                                    &var_as_str,
                                    crate::SIMPLE,
                                ));
                            }
                            output.push_str("/debug/sendCapture");
                            output
                        }
                        fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
                            let req = self.reqwest.request(::reqwest::Method::POST, path);
                            let req = req.query(&[("access_token", &self.access_token)]);
                            let req = req.query(&[("alt", &self.alt)]);
                            let req = req.query(&[("callback", &self.callback)]);
                            let req = req.query(&[("fields", &self.fields)]);
                            let req = req.query(&[("key", &self.key)]);
                            let req = req.query(&[("oauth_token", &self.oauth_token)]);
                            let req = req.query(&[("prettyPrint", &self.pretty_print)]);
                            let req = req.query(&[("quotaUser", &self.quota_user)]);
                            let req = req.query(&[("upload_protocol", &self.upload_protocol)]);
                            let req = req.query(&[("uploadType", &self.upload_type)]);
                            let req = req.query(&[("$.xgafv", &self.xgafv)]);
                            let mut auth = self.auth.lock().unwrap();
                            let fut =
                                auth.token(vec!["https://www.googleapis.com/auth/cloud-platform"]);
                            let mut runtime = ::tokio::runtime::Runtime::new().unwrap();
                            let token = runtime.block_on(fut).unwrap().access_token;
                            let req = req.bearer_auth(&token);
                            req
                        }
                    }
                }
                pub mod messages {
                    pub mod params {
                        #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
                        pub enum ListMinimumImportance {
                            JobMessageBasic,
                            JobMessageDebug,
                            JobMessageDetailed,
                            JobMessageError,
                            JobMessageImportanceUnknown,
                            JobMessageWarning,
                        }
                        impl ListMinimumImportance {
                            pub fn as_str(self) -> &'static str {
                                match self {
                                    ListMinimumImportance::JobMessageBasic => "JOB_MESSAGE_BASIC",
                                    ListMinimumImportance::JobMessageDebug => "JOB_MESSAGE_DEBUG",
                                    ListMinimumImportance::JobMessageDetailed => {
                                        "JOB_MESSAGE_DETAILED"
                                    }
                                    ListMinimumImportance::JobMessageError => "JOB_MESSAGE_ERROR",
                                    ListMinimumImportance::JobMessageImportanceUnknown => {
                                        "JOB_MESSAGE_IMPORTANCE_UNKNOWN"
                                    }
                                    ListMinimumImportance::JobMessageWarning => {
                                        "JOB_MESSAGE_WARNING"
                                    }
                                }
                            }
                        }
                        impl ::std::fmt::Display for ListMinimumImportance {
                            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                                f.write_str(self.as_str())
                            }
                        }
                        impl ::serde::Serialize for ListMinimumImportance {
                            fn serialize<S>(
                                &self,
                                serializer: S,
                            ) -> ::std::result::Result<S::Ok, S::Error>
                            where
                                S: ::serde::ser::Serializer,
                            {
                                serializer.serialize_str(self.as_str())
                            }
                        }
                        impl<'de> ::serde::Deserialize<'de> for ListMinimumImportance {
                            fn deserialize<D>(
                                deserializer: D,
                            ) -> ::std::result::Result<Self, D::Error>
                            where
                                D: ::serde::de::Deserializer<'de>,
                            {
                                let value: &'de str = <&str>::deserialize(deserializer)?;
                                Ok(match value {
                                    "JOB_MESSAGE_BASIC" => ListMinimumImportance::JobMessageBasic,
                                    "JOB_MESSAGE_DEBUG" => ListMinimumImportance::JobMessageDebug,
                                    "JOB_MESSAGE_DETAILED" => {
                                        ListMinimumImportance::JobMessageDetailed
                                    }
                                    "JOB_MESSAGE_ERROR" => ListMinimumImportance::JobMessageError,
                                    "JOB_MESSAGE_IMPORTANCE_UNKNOWN" => {
                                        ListMinimumImportance::JobMessageImportanceUnknown
                                    }
                                    "JOB_MESSAGE_WARNING" => {
                                        ListMinimumImportance::JobMessageWarning
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
                        impl ::field_selector::FieldSelector for ListMinimumImportance {
                            fn field_selector_with_ident(ident: &str, selector: &mut String) {
                                match selector.chars().rev().nth(0) {
                                    Some(',') | None => {}
                                    _ => selector.push_str(","),
                                }
                                selector.push_str(ident);
                            }
                        }
                    }
                    pub struct MessagesActions<'a, A> {
                        pub(crate) reqwest: &'a reqwest::Client,
                        pub(crate) auth: &'a std::sync::Mutex<A>,
                    }
                    impl<'a, A: yup_oauth2::GetToken> MessagesActions<'a, A> {
                        #[doc = "Request the job status.\n\nTo request the status of a job, we recommend using\n`projects.locations.jobs.messages.list` with a [regional endpoint]\n(https://cloud.google.com/dataflow/docs/concepts/regional-endpoints). Using\n`projects.jobs.messages.list` is not recommended, as you can only request\nthe status of jobs that are running in `us-central1`."]
                        pub fn list(
                            &self,
                            project_id: impl Into<String>,
                            location: impl Into<String>,
                            job_id: impl Into<String>,
                        ) -> ListRequestBuilder<A> {
                            ListRequestBuilder {
                                reqwest: &self.reqwest,
                                auth: &self.auth,
                                access_token: None,
                                alt: None,
                                callback: None,
                                fields: None,
                                key: None,
                                oauth_token: None,
                                pretty_print: None,
                                quota_user: None,
                                upload_protocol: None,
                                upload_type: None,
                                xgafv: None,
                                project_id: project_id.into(),
                                location: location.into(),
                                job_id: job_id.into(),
                                end_time: None,
                                minimum_importance: None,
                                page_size: None,
                                page_token: None,
                                start_time: None,
                            }
                        }
                    }
                    #[derive(Debug, Clone)]
                    pub struct ListRequestBuilder < 'a , A > { pub ( crate ) reqwest : & 'a :: reqwest :: Client , pub ( crate ) auth : & 'a :: std :: sync :: Mutex < A > , project_id : String , location : String , job_id : String , end_time : Option < String > , minimum_importance : Option < crate :: resources :: projects :: locations :: jobs :: messages :: params :: ListMinimumImportance > , page_size : Option < i32 > , page_token : Option < String > , start_time : Option < String > , access_token : Option < String > , alt : Option < crate :: params :: Alt > , callback : Option < String > , fields : Option < String > , key : Option < String > , oauth_token : Option < String > , pretty_print : Option < bool > , quota_user : Option < String > , upload_protocol : Option < String > , upload_type : Option < String > , xgafv : Option < crate :: params :: Xgafv > , }
                    impl<'a, A: yup_oauth2::GetToken> ListRequestBuilder<'a, A> {
                        #[doc = "Return only messages with timestamps < end_time. The default is now\n(i.e. return up to the latest messages available)."]
                        pub fn end_time(mut self, value: impl Into<String>) -> Self {
                            self.end_time = Some(value.into());
                            self
                        }
                        #[doc = "Filter to only get messages with importance >= level"]
                        pub fn minimum_importance(
                            mut self,
                            value : crate :: resources :: projects :: locations :: jobs :: messages :: params :: ListMinimumImportance,
                        ) -> Self {
                            self.minimum_importance = Some(value);
                            self
                        }
                        #[doc = "If specified, determines the maximum number of messages to\nreturn.  If unspecified, the service may choose an appropriate\ndefault, or may return an arbitrarily large number of results."]
                        pub fn page_size(mut self, value: i32) -> Self {
                            self.page_size = Some(value);
                            self
                        }
                        #[doc = "If supplied, this should be the value of next_page_token returned\nby an earlier call. This will cause the next page of results to\nbe returned."]
                        pub fn page_token(mut self, value: impl Into<String>) -> Self {
                            self.page_token = Some(value.into());
                            self
                        }
                        #[doc = "If specified, return only messages with timestamps >= start_time.\nThe default is the job creation time (i.e. beginning of messages)."]
                        pub fn start_time(mut self, value: impl Into<String>) -> Self {
                            self.start_time = Some(value.into());
                            self
                        }
                        #[doc = "OAuth access token."]
                        pub fn access_token(mut self, value: impl Into<String>) -> Self {
                            self.access_token = Some(value.into());
                            self
                        }
                        #[doc = "JSONP"]
                        pub fn callback(mut self, value: impl Into<String>) -> Self {
                            self.callback = Some(value.into());
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
                        #[doc = "Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters."]
                        pub fn quota_user(mut self, value: impl Into<String>) -> Self {
                            self.quota_user = Some(value.into());
                            self
                        }
                        #[doc = "Upload protocol for media (e.g. \"raw\", \"multipart\")."]
                        pub fn upload_protocol(mut self, value: impl Into<String>) -> Self {
                            self.upload_protocol = Some(value.into());
                            self
                        }
                        #[doc = "Legacy upload protocol for media (e.g. \"media\", \"multipart\")."]
                        pub fn upload_type(mut self, value: impl Into<String>) -> Self {
                            self.upload_type = Some(value.into());
                            self
                        }
                        #[doc = "V1 error format."]
                        pub fn xgafv(mut self, value: crate::params::Xgafv) -> Self {
                            self.xgafv = Some(value);
                            self
                        }
                        #[doc = r" Return an iterator that iterates over all `#prop_ident`. The"]
                        #[doc = r" items yielded by the iterator are chosen by the caller of this"]
                        #[doc = r" method and must implement `Deserialize` and `FieldSelector`. The"]
                        #[doc = r" populated fields in the yielded items will be determined by the"]
                        #[doc = r" `FieldSelector` implementation."]
                        pub fn iter_autoscaling_events<T>(
                            mut self,
                        ) -> crate::iter::PageItemIter<Self, T>
                        where
                            T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
                        {
                            let mut fields =
                                concat!("nextPageToken,", "autoscalingEvents").to_owned();
                            let items_fields = T::field_selector();
                            if !items_fields.is_empty() {
                                fields.push_str("(");
                                fields.push_str(&items_fields);
                                fields.push_str(")");
                            }
                            self.fields = Some(fields);
                            crate::iter::PageItemIter::new(self, "autoscalingEvents")
                        }
                        #[doc = r" Return an iterator that iterates over all `#prop_ident`. The"]
                        #[doc = r" items yielded by the iterator are `#items_type`. The populated"]
                        #[doc = r" fields in `#items_type` will be the default fields populated by"]
                        #[doc = r" the server."]
                        pub fn iter_autoscaling_events_standard(
                            mut self,
                        ) -> crate::iter::PageItemIter<Self, crate::schemas::AutoscalingEvent>
                        {
                            self.fields =
                                Some(concat!("nextPageToken,", "autoscalingEvents").to_owned());
                            crate::iter::PageItemIter::new(self, "autoscalingEvents")
                        }
                        #[doc = r" Return an iterator that iterates over all `#prop_ident`. The"]
                        #[doc = r" items yielded by the iterator are `#items_type`. The populated"]
                        #[doc = r" fields in `#items_type` will be all fields available. This should"]
                        #[doc = r" primarily be used during developement and debugging as fetching"]
                        #[doc = r" all fields can be expensive both in bandwidth and server"]
                        #[doc = r" resources."]
                        pub fn iter_autoscaling_events_debug(
                            mut self,
                        ) -> crate::iter::PageItemIter<Self, crate::schemas::AutoscalingEvent>
                        {
                            self.fields = Some(
                                concat!("nextPageToken,", "autoscalingEvents", "(*)").to_owned(),
                            );
                            crate::iter::PageItemIter::new(self, "autoscalingEvents")
                        }
                        #[doc = r" Return an iterator that iterates over all `#prop_ident`. The"]
                        #[doc = r" items yielded by the iterator are chosen by the caller of this"]
                        #[doc = r" method and must implement `Deserialize` and `FieldSelector`. The"]
                        #[doc = r" populated fields in the yielded items will be determined by the"]
                        #[doc = r" `FieldSelector` implementation."]
                        pub fn iter_job_messages<T>(mut self) -> crate::iter::PageItemIter<Self, T>
                        where
                            T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
                        {
                            let mut fields = concat!("nextPageToken,", "jobMessages").to_owned();
                            let items_fields = T::field_selector();
                            if !items_fields.is_empty() {
                                fields.push_str("(");
                                fields.push_str(&items_fields);
                                fields.push_str(")");
                            }
                            self.fields = Some(fields);
                            crate::iter::PageItemIter::new(self, "jobMessages")
                        }
                        #[doc = r" Return an iterator that iterates over all `#prop_ident`. The"]
                        #[doc = r" items yielded by the iterator are `#items_type`. The populated"]
                        #[doc = r" fields in `#items_type` will be the default fields populated by"]
                        #[doc = r" the server."]
                        pub fn iter_job_messages_standard(
                            mut self,
                        ) -> crate::iter::PageItemIter<Self, crate::schemas::JobMessage>
                        {
                            self.fields = Some(concat!("nextPageToken,", "jobMessages").to_owned());
                            crate::iter::PageItemIter::new(self, "jobMessages")
                        }
                        #[doc = r" Return an iterator that iterates over all `#prop_ident`. The"]
                        #[doc = r" items yielded by the iterator are `#items_type`. The populated"]
                        #[doc = r" fields in `#items_type` will be all fields available. This should"]
                        #[doc = r" primarily be used during developement and debugging as fetching"]
                        #[doc = r" all fields can be expensive both in bandwidth and server"]
                        #[doc = r" resources."]
                        pub fn iter_job_messages_debug(
                            mut self,
                        ) -> crate::iter::PageItemIter<Self, crate::schemas::JobMessage>
                        {
                            self.fields =
                                Some(concat!("nextPageToken,", "jobMessages", "(*)").to_owned());
                            crate::iter::PageItemIter::new(self, "jobMessages")
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
                        ) -> crate::iter::PageIter<Self, crate::schemas::ListJobMessagesResponse>
                        {
                            crate::iter::PageIter::new(self)
                        }
                        pub fn iter_debug(
                            mut self,
                        ) -> crate::iter::PageIter<Self, crate::schemas::ListJobMessagesResponse>
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
                            crate::schemas::ListJobMessagesResponse,
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
                            crate::schemas::ListJobMessagesResponse,
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
                            let mut output = "https://dataflow.googleapis.com/".to_owned();
                            output.push_str("v1b3/projects/");
                            {
                                let var_as_str = &self.project_id;
                                output.extend(::percent_encoding::utf8_percent_encode(
                                    &var_as_str,
                                    crate::SIMPLE,
                                ));
                            }
                            output.push_str("/locations/");
                            {
                                let var_as_str = &self.location;
                                output.extend(::percent_encoding::utf8_percent_encode(
                                    &var_as_str,
                                    crate::SIMPLE,
                                ));
                            }
                            output.push_str("/jobs/");
                            {
                                let var_as_str = &self.job_id;
                                output.extend(::percent_encoding::utf8_percent_encode(
                                    &var_as_str,
                                    crate::SIMPLE,
                                ));
                            }
                            output.push_str("/messages");
                            output
                        }
                        fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
                            let req = self.reqwest.request(::reqwest::Method::GET, path);
                            let req = req.query(&[("endTime", &self.end_time)]);
                            let req = req.query(&[("minimumImportance", &self.minimum_importance)]);
                            let req = req.query(&[("pageSize", &self.page_size)]);
                            let req = req.query(&[("pageToken", &self.page_token)]);
                            let req = req.query(&[("startTime", &self.start_time)]);
                            let req = req.query(&[("access_token", &self.access_token)]);
                            let req = req.query(&[("alt", &self.alt)]);
                            let req = req.query(&[("callback", &self.callback)]);
                            let req = req.query(&[("fields", &self.fields)]);
                            let req = req.query(&[("key", &self.key)]);
                            let req = req.query(&[("oauth_token", &self.oauth_token)]);
                            let req = req.query(&[("prettyPrint", &self.pretty_print)]);
                            let req = req.query(&[("quotaUser", &self.quota_user)]);
                            let req = req.query(&[("upload_protocol", &self.upload_protocol)]);
                            let req = req.query(&[("uploadType", &self.upload_type)]);
                            let req = req.query(&[("$.xgafv", &self.xgafv)]);
                            let mut auth = self.auth.lock().unwrap();
                            let fut = auth
                                .token(vec!["https://www.googleapis.com/auth/compute.readonly"]);
                            let mut runtime = ::tokio::runtime::Runtime::new().unwrap();
                            let token = runtime.block_on(fut).unwrap().access_token;
                            let req = req.bearer_auth(&token);
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
                pub mod snapshots {
                    pub mod params {}
                    pub struct SnapshotsActions<'a, A> {
                        pub(crate) reqwest: &'a reqwest::Client,
                        pub(crate) auth: &'a std::sync::Mutex<A>,
                    }
                    impl<'a, A: yup_oauth2::GetToken> SnapshotsActions<'a, A> {
                        #[doc = "Lists snapshots."]
                        pub fn list(
                            &self,
                            project_id: impl Into<String>,
                            location: impl Into<String>,
                            job_id: impl Into<String>,
                        ) -> ListRequestBuilder<A> {
                            ListRequestBuilder {
                                reqwest: &self.reqwest,
                                auth: &self.auth,
                                access_token: None,
                                alt: None,
                                callback: None,
                                fields: None,
                                key: None,
                                oauth_token: None,
                                pretty_print: None,
                                quota_user: None,
                                upload_protocol: None,
                                upload_type: None,
                                xgafv: None,
                                project_id: project_id.into(),
                                location: location.into(),
                                job_id: job_id.into(),
                            }
                        }
                    }
                    #[derive(Debug, Clone)]
                    pub struct ListRequestBuilder<'a, A> {
                        pub(crate) reqwest: &'a ::reqwest::Client,
                        pub(crate) auth: &'a ::std::sync::Mutex<A>,
                        project_id: String,
                        location: String,
                        job_id: String,
                        access_token: Option<String>,
                        alt: Option<crate::params::Alt>,
                        callback: Option<String>,
                        fields: Option<String>,
                        key: Option<String>,
                        oauth_token: Option<String>,
                        pretty_print: Option<bool>,
                        quota_user: Option<String>,
                        upload_protocol: Option<String>,
                        upload_type: Option<String>,
                        xgafv: Option<crate::params::Xgafv>,
                    }
                    impl<'a, A: yup_oauth2::GetToken> ListRequestBuilder<'a, A> {
                        #[doc = "OAuth access token."]
                        pub fn access_token(mut self, value: impl Into<String>) -> Self {
                            self.access_token = Some(value.into());
                            self
                        }
                        #[doc = "JSONP"]
                        pub fn callback(mut self, value: impl Into<String>) -> Self {
                            self.callback = Some(value.into());
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
                        #[doc = "Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters."]
                        pub fn quota_user(mut self, value: impl Into<String>) -> Self {
                            self.quota_user = Some(value.into());
                            self
                        }
                        #[doc = "Upload protocol for media (e.g. \"raw\", \"multipart\")."]
                        pub fn upload_protocol(mut self, value: impl Into<String>) -> Self {
                            self.upload_protocol = Some(value.into());
                            self
                        }
                        #[doc = "Legacy upload protocol for media (e.g. \"media\", \"multipart\")."]
                        pub fn upload_type(mut self, value: impl Into<String>) -> Self {
                            self.upload_type = Some(value.into());
                            self
                        }
                        #[doc = "V1 error format."]
                        pub fn xgafv(mut self, value: crate::params::Xgafv) -> Self {
                            self.xgafv = Some(value);
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
                            crate::schemas::ListSnapshotsResponse,
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
                            crate::schemas::ListSnapshotsResponse,
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
                            let mut output = "https://dataflow.googleapis.com/".to_owned();
                            output.push_str("v1b3/projects/");
                            {
                                let var_as_str = &self.project_id;
                                output.extend(::percent_encoding::utf8_percent_encode(
                                    &var_as_str,
                                    crate::SIMPLE,
                                ));
                            }
                            output.push_str("/locations/");
                            {
                                let var_as_str = &self.location;
                                output.extend(::percent_encoding::utf8_percent_encode(
                                    &var_as_str,
                                    crate::SIMPLE,
                                ));
                            }
                            output.push_str("/jobs/");
                            {
                                let var_as_str = &self.job_id;
                                output.extend(::percent_encoding::utf8_percent_encode(
                                    &var_as_str,
                                    crate::SIMPLE,
                                ));
                            }
                            output.push_str("/snapshots");
                            output
                        }
                        fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
                            let req = self.reqwest.request(::reqwest::Method::GET, path);
                            let req = req.query(&[("access_token", &self.access_token)]);
                            let req = req.query(&[("alt", &self.alt)]);
                            let req = req.query(&[("callback", &self.callback)]);
                            let req = req.query(&[("fields", &self.fields)]);
                            let req = req.query(&[("key", &self.key)]);
                            let req = req.query(&[("oauth_token", &self.oauth_token)]);
                            let req = req.query(&[("prettyPrint", &self.pretty_print)]);
                            let req = req.query(&[("quotaUser", &self.quota_user)]);
                            let req = req.query(&[("upload_protocol", &self.upload_protocol)]);
                            let req = req.query(&[("uploadType", &self.upload_type)]);
                            let req = req.query(&[("$.xgafv", &self.xgafv)]);
                            let mut auth = self.auth.lock().unwrap();
                            let fut = auth
                                .token(vec!["https://www.googleapis.com/auth/compute.readonly"]);
                            let mut runtime = ::tokio::runtime::Runtime::new().unwrap();
                            let token = runtime.block_on(fut).unwrap().access_token;
                            let req = req.bearer_auth(&token);
                            req
                        }
                    }
                }
                pub mod work_items {
                    pub mod params {}
                    pub struct WorkItemsActions<'a, A> {
                        pub(crate) reqwest: &'a reqwest::Client,
                        pub(crate) auth: &'a std::sync::Mutex<A>,
                    }
                    impl<'a, A: yup_oauth2::GetToken> WorkItemsActions<'a, A> {
                        #[doc = "Leases a dataflow WorkItem to run."]
                        pub fn lease(
                            &self,
                            request: crate::schemas::LeaseWorkItemRequest,
                            project_id: impl Into<String>,
                            location: impl Into<String>,
                            job_id: impl Into<String>,
                        ) -> LeaseRequestBuilder<A> {
                            LeaseRequestBuilder {
                                reqwest: &self.reqwest,
                                auth: &self.auth,
                                request,
                                access_token: None,
                                alt: None,
                                callback: None,
                                fields: None,
                                key: None,
                                oauth_token: None,
                                pretty_print: None,
                                quota_user: None,
                                upload_protocol: None,
                                upload_type: None,
                                xgafv: None,
                                project_id: project_id.into(),
                                location: location.into(),
                                job_id: job_id.into(),
                            }
                        }
                        #[doc = "Reports the status of dataflow WorkItems leased by a worker."]
                        pub fn report_status(
                            &self,
                            request: crate::schemas::ReportWorkItemStatusRequest,
                            project_id: impl Into<String>,
                            location: impl Into<String>,
                            job_id: impl Into<String>,
                        ) -> ReportStatusRequestBuilder<A> {
                            ReportStatusRequestBuilder {
                                reqwest: &self.reqwest,
                                auth: &self.auth,
                                request,
                                access_token: None,
                                alt: None,
                                callback: None,
                                fields: None,
                                key: None,
                                oauth_token: None,
                                pretty_print: None,
                                quota_user: None,
                                upload_protocol: None,
                                upload_type: None,
                                xgafv: None,
                                project_id: project_id.into(),
                                location: location.into(),
                                job_id: job_id.into(),
                            }
                        }
                    }
                    #[derive(Debug, Clone)]
                    pub struct LeaseRequestBuilder<'a, A> {
                        pub(crate) reqwest: &'a ::reqwest::Client,
                        pub(crate) auth: &'a ::std::sync::Mutex<A>,
                        request: crate::schemas::LeaseWorkItemRequest,
                        project_id: String,
                        location: String,
                        job_id: String,
                        access_token: Option<String>,
                        alt: Option<crate::params::Alt>,
                        callback: Option<String>,
                        fields: Option<String>,
                        key: Option<String>,
                        oauth_token: Option<String>,
                        pretty_print: Option<bool>,
                        quota_user: Option<String>,
                        upload_protocol: Option<String>,
                        upload_type: Option<String>,
                        xgafv: Option<crate::params::Xgafv>,
                    }
                    impl<'a, A: yup_oauth2::GetToken> LeaseRequestBuilder<'a, A> {
                        #[doc = "OAuth access token."]
                        pub fn access_token(mut self, value: impl Into<String>) -> Self {
                            self.access_token = Some(value.into());
                            self
                        }
                        #[doc = "JSONP"]
                        pub fn callback(mut self, value: impl Into<String>) -> Self {
                            self.callback = Some(value.into());
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
                        #[doc = "Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters."]
                        pub fn quota_user(mut self, value: impl Into<String>) -> Self {
                            self.quota_user = Some(value.into());
                            self
                        }
                        #[doc = "Upload protocol for media (e.g. \"raw\", \"multipart\")."]
                        pub fn upload_protocol(mut self, value: impl Into<String>) -> Self {
                            self.upload_protocol = Some(value.into());
                            self
                        }
                        #[doc = "Legacy upload protocol for media (e.g. \"media\", \"multipart\")."]
                        pub fn upload_type(mut self, value: impl Into<String>) -> Self {
                            self.upload_type = Some(value.into());
                            self
                        }
                        #[doc = "V1 error format."]
                        pub fn xgafv(mut self, value: crate::params::Xgafv) -> Self {
                            self.xgafv = Some(value);
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
                            crate::schemas::LeaseWorkItemResponse,
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
                            crate::schemas::LeaseWorkItemResponse,
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
                            let req = req.json(&self.request);
                            Ok(req.send()?.error_for_status()?.json()?)
                        }
                        fn _path(&self) -> String {
                            let mut output = "https://dataflow.googleapis.com/".to_owned();
                            output.push_str("v1b3/projects/");
                            {
                                let var_as_str = &self.project_id;
                                output.extend(::percent_encoding::utf8_percent_encode(
                                    &var_as_str,
                                    crate::SIMPLE,
                                ));
                            }
                            output.push_str("/locations/");
                            {
                                let var_as_str = &self.location;
                                output.extend(::percent_encoding::utf8_percent_encode(
                                    &var_as_str,
                                    crate::SIMPLE,
                                ));
                            }
                            output.push_str("/jobs/");
                            {
                                let var_as_str = &self.job_id;
                                output.extend(::percent_encoding::utf8_percent_encode(
                                    &var_as_str,
                                    crate::SIMPLE,
                                ));
                            }
                            output.push_str("/workItems:lease");
                            output
                        }
                        fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
                            let req = self.reqwest.request(::reqwest::Method::POST, path);
                            let req = req.query(&[("access_token", &self.access_token)]);
                            let req = req.query(&[("alt", &self.alt)]);
                            let req = req.query(&[("callback", &self.callback)]);
                            let req = req.query(&[("fields", &self.fields)]);
                            let req = req.query(&[("key", &self.key)]);
                            let req = req.query(&[("oauth_token", &self.oauth_token)]);
                            let req = req.query(&[("prettyPrint", &self.pretty_print)]);
                            let req = req.query(&[("quotaUser", &self.quota_user)]);
                            let req = req.query(&[("upload_protocol", &self.upload_protocol)]);
                            let req = req.query(&[("uploadType", &self.upload_type)]);
                            let req = req.query(&[("$.xgafv", &self.xgafv)]);
                            let mut auth = self.auth.lock().unwrap();
                            let fut =
                                auth.token(vec!["https://www.googleapis.com/auth/cloud-platform"]);
                            let mut runtime = ::tokio::runtime::Runtime::new().unwrap();
                            let token = runtime.block_on(fut).unwrap().access_token;
                            let req = req.bearer_auth(&token);
                            req
                        }
                    }
                    #[derive(Debug, Clone)]
                    pub struct ReportStatusRequestBuilder<'a, A> {
                        pub(crate) reqwest: &'a ::reqwest::Client,
                        pub(crate) auth: &'a ::std::sync::Mutex<A>,
                        request: crate::schemas::ReportWorkItemStatusRequest,
                        project_id: String,
                        location: String,
                        job_id: String,
                        access_token: Option<String>,
                        alt: Option<crate::params::Alt>,
                        callback: Option<String>,
                        fields: Option<String>,
                        key: Option<String>,
                        oauth_token: Option<String>,
                        pretty_print: Option<bool>,
                        quota_user: Option<String>,
                        upload_protocol: Option<String>,
                        upload_type: Option<String>,
                        xgafv: Option<crate::params::Xgafv>,
                    }
                    impl<'a, A: yup_oauth2::GetToken> ReportStatusRequestBuilder<'a, A> {
                        #[doc = "OAuth access token."]
                        pub fn access_token(mut self, value: impl Into<String>) -> Self {
                            self.access_token = Some(value.into());
                            self
                        }
                        #[doc = "JSONP"]
                        pub fn callback(mut self, value: impl Into<String>) -> Self {
                            self.callback = Some(value.into());
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
                        #[doc = "Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters."]
                        pub fn quota_user(mut self, value: impl Into<String>) -> Self {
                            self.quota_user = Some(value.into());
                            self
                        }
                        #[doc = "Upload protocol for media (e.g. \"raw\", \"multipart\")."]
                        pub fn upload_protocol(mut self, value: impl Into<String>) -> Self {
                            self.upload_protocol = Some(value.into());
                            self
                        }
                        #[doc = "Legacy upload protocol for media (e.g. \"media\", \"multipart\")."]
                        pub fn upload_type(mut self, value: impl Into<String>) -> Self {
                            self.upload_type = Some(value.into());
                            self
                        }
                        #[doc = "V1 error format."]
                        pub fn xgafv(mut self, value: crate::params::Xgafv) -> Self {
                            self.xgafv = Some(value);
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
                            crate::schemas::ReportWorkItemStatusResponse,
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
                            crate::schemas::ReportWorkItemStatusResponse,
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
                            let req = req.json(&self.request);
                            Ok(req.send()?.error_for_status()?.json()?)
                        }
                        fn _path(&self) -> String {
                            let mut output = "https://dataflow.googleapis.com/".to_owned();
                            output.push_str("v1b3/projects/");
                            {
                                let var_as_str = &self.project_id;
                                output.extend(::percent_encoding::utf8_percent_encode(
                                    &var_as_str,
                                    crate::SIMPLE,
                                ));
                            }
                            output.push_str("/locations/");
                            {
                                let var_as_str = &self.location;
                                output.extend(::percent_encoding::utf8_percent_encode(
                                    &var_as_str,
                                    crate::SIMPLE,
                                ));
                            }
                            output.push_str("/jobs/");
                            {
                                let var_as_str = &self.job_id;
                                output.extend(::percent_encoding::utf8_percent_encode(
                                    &var_as_str,
                                    crate::SIMPLE,
                                ));
                            }
                            output.push_str("/workItems:reportStatus");
                            output
                        }
                        fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
                            let req = self.reqwest.request(::reqwest::Method::POST, path);
                            let req = req.query(&[("access_token", &self.access_token)]);
                            let req = req.query(&[("alt", &self.alt)]);
                            let req = req.query(&[("callback", &self.callback)]);
                            let req = req.query(&[("fields", &self.fields)]);
                            let req = req.query(&[("key", &self.key)]);
                            let req = req.query(&[("oauth_token", &self.oauth_token)]);
                            let req = req.query(&[("prettyPrint", &self.pretty_print)]);
                            let req = req.query(&[("quotaUser", &self.quota_user)]);
                            let req = req.query(&[("upload_protocol", &self.upload_protocol)]);
                            let req = req.query(&[("uploadType", &self.upload_type)]);
                            let req = req.query(&[("$.xgafv", &self.xgafv)]);
                            let mut auth = self.auth.lock().unwrap();
                            let fut =
                                auth.token(vec!["https://www.googleapis.com/auth/cloud-platform"]);
                            let mut runtime = ::tokio::runtime::Runtime::new().unwrap();
                            let token = runtime.block_on(fut).unwrap().access_token;
                            let req = req.bearer_auth(&token);
                            req
                        }
                    }
                }
            }
            pub mod snapshots {
                pub mod params {}
                pub struct SnapshotsActions<'a, A> {
                    pub(crate) reqwest: &'a reqwest::Client,
                    pub(crate) auth: &'a std::sync::Mutex<A>,
                }
                impl<'a, A: yup_oauth2::GetToken> SnapshotsActions<'a, A> {
                    #[doc = "Deletes a snapshot."]
                    pub fn delete(
                        &self,
                        project_id: impl Into<String>,
                        location: impl Into<String>,
                        snapshot_id: impl Into<String>,
                    ) -> DeleteRequestBuilder<A> {
                        DeleteRequestBuilder {
                            reqwest: &self.reqwest,
                            auth: &self.auth,
                            access_token: None,
                            alt: None,
                            callback: None,
                            fields: None,
                            key: None,
                            oauth_token: None,
                            pretty_print: None,
                            quota_user: None,
                            upload_protocol: None,
                            upload_type: None,
                            xgafv: None,
                            project_id: project_id.into(),
                            location: location.into(),
                            snapshot_id: snapshot_id.into(),
                        }
                    }
                    #[doc = "Gets information about a snapshot."]
                    pub fn get(
                        &self,
                        project_id: impl Into<String>,
                        location: impl Into<String>,
                        snapshot_id: impl Into<String>,
                    ) -> GetRequestBuilder<A> {
                        GetRequestBuilder {
                            reqwest: &self.reqwest,
                            auth: &self.auth,
                            access_token: None,
                            alt: None,
                            callback: None,
                            fields: None,
                            key: None,
                            oauth_token: None,
                            pretty_print: None,
                            quota_user: None,
                            upload_protocol: None,
                            upload_type: None,
                            xgafv: None,
                            project_id: project_id.into(),
                            location: location.into(),
                            snapshot_id: snapshot_id.into(),
                        }
                    }
                    #[doc = "Lists snapshots."]
                    pub fn list(
                        &self,
                        project_id: impl Into<String>,
                        location: impl Into<String>,
                    ) -> ListRequestBuilder<A> {
                        ListRequestBuilder {
                            reqwest: &self.reqwest,
                            auth: &self.auth,
                            access_token: None,
                            alt: None,
                            callback: None,
                            fields: None,
                            key: None,
                            oauth_token: None,
                            pretty_print: None,
                            quota_user: None,
                            upload_protocol: None,
                            upload_type: None,
                            xgafv: None,
                            project_id: project_id.into(),
                            location: location.into(),
                            job_id: None,
                        }
                    }
                }
                #[derive(Debug, Clone)]
                pub struct DeleteRequestBuilder<'a, A> {
                    pub(crate) reqwest: &'a ::reqwest::Client,
                    pub(crate) auth: &'a ::std::sync::Mutex<A>,
                    project_id: String,
                    location: String,
                    snapshot_id: String,
                    access_token: Option<String>,
                    alt: Option<crate::params::Alt>,
                    callback: Option<String>,
                    fields: Option<String>,
                    key: Option<String>,
                    oauth_token: Option<String>,
                    pretty_print: Option<bool>,
                    quota_user: Option<String>,
                    upload_protocol: Option<String>,
                    upload_type: Option<String>,
                    xgafv: Option<crate::params::Xgafv>,
                }
                impl<'a, A: yup_oauth2::GetToken> DeleteRequestBuilder<'a, A> {
                    #[doc = "OAuth access token."]
                    pub fn access_token(mut self, value: impl Into<String>) -> Self {
                        self.access_token = Some(value.into());
                        self
                    }
                    #[doc = "JSONP"]
                    pub fn callback(mut self, value: impl Into<String>) -> Self {
                        self.callback = Some(value.into());
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
                    #[doc = "Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters."]
                    pub fn quota_user(mut self, value: impl Into<String>) -> Self {
                        self.quota_user = Some(value.into());
                        self
                    }
                    #[doc = "Upload protocol for media (e.g. \"raw\", \"multipart\")."]
                    pub fn upload_protocol(mut self, value: impl Into<String>) -> Self {
                        self.upload_protocol = Some(value.into());
                        self
                    }
                    #[doc = "Legacy upload protocol for media (e.g. \"media\", \"multipart\")."]
                    pub fn upload_type(mut self, value: impl Into<String>) -> Self {
                        self.upload_type = Some(value.into());
                        self
                    }
                    #[doc = "V1 error format."]
                    pub fn xgafv(mut self, value: crate::params::Xgafv) -> Self {
                        self.xgafv = Some(value);
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
                    ) -> Result<crate::schemas::DeleteSnapshotResponse, Box<dyn ::std::error::Error>>
                    {
                        self.execute_fields::<_, &str>(None)
                    }
                    #[doc = r" Execute the given operation. This will provide a `fields`"]
                    #[doc = r" selector of `*`. This will include every attribute of the"]
                    #[doc = r" response resource and should be limited to use during"]
                    #[doc = r" development or debugging."]
                    pub fn execute_debug(
                        self,
                    ) -> Result<crate::schemas::DeleteSnapshotResponse, Box<dyn ::std::error::Error>>
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
                        let mut output = "https://dataflow.googleapis.com/".to_owned();
                        output.push_str("v1b3/projects/");
                        {
                            let var_as_str = &self.project_id;
                            output.extend(::percent_encoding::utf8_percent_encode(
                                &var_as_str,
                                crate::SIMPLE,
                            ));
                        }
                        output.push_str("/locations/");
                        {
                            let var_as_str = &self.location;
                            output.extend(::percent_encoding::utf8_percent_encode(
                                &var_as_str,
                                crate::SIMPLE,
                            ));
                        }
                        output.push_str("/snapshots/");
                        {
                            let var_as_str = &self.snapshot_id;
                            output.extend(::percent_encoding::utf8_percent_encode(
                                &var_as_str,
                                crate::SIMPLE,
                            ));
                        }
                        output
                    }
                    fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
                        let req = self.reqwest.request(::reqwest::Method::DELETE, path);
                        let req = req.query(&[("access_token", &self.access_token)]);
                        let req = req.query(&[("alt", &self.alt)]);
                        let req = req.query(&[("callback", &self.callback)]);
                        let req = req.query(&[("fields", &self.fields)]);
                        let req = req.query(&[("key", &self.key)]);
                        let req = req.query(&[("oauth_token", &self.oauth_token)]);
                        let req = req.query(&[("prettyPrint", &self.pretty_print)]);
                        let req = req.query(&[("quotaUser", &self.quota_user)]);
                        let req = req.query(&[("upload_protocol", &self.upload_protocol)]);
                        let req = req.query(&[("uploadType", &self.upload_type)]);
                        let req = req.query(&[("$.xgafv", &self.xgafv)]);
                        let mut auth = self.auth.lock().unwrap();
                        let fut =
                            auth.token(vec!["https://www.googleapis.com/auth/cloud-platform"]);
                        let mut runtime = ::tokio::runtime::Runtime::new().unwrap();
                        let token = runtime.block_on(fut).unwrap().access_token;
                        let req = req.bearer_auth(&token);
                        req
                    }
                }
                #[derive(Debug, Clone)]
                pub struct GetRequestBuilder<'a, A> {
                    pub(crate) reqwest: &'a ::reqwest::Client,
                    pub(crate) auth: &'a ::std::sync::Mutex<A>,
                    project_id: String,
                    location: String,
                    snapshot_id: String,
                    access_token: Option<String>,
                    alt: Option<crate::params::Alt>,
                    callback: Option<String>,
                    fields: Option<String>,
                    key: Option<String>,
                    oauth_token: Option<String>,
                    pretty_print: Option<bool>,
                    quota_user: Option<String>,
                    upload_protocol: Option<String>,
                    upload_type: Option<String>,
                    xgafv: Option<crate::params::Xgafv>,
                }
                impl<'a, A: yup_oauth2::GetToken> GetRequestBuilder<'a, A> {
                    #[doc = "OAuth access token."]
                    pub fn access_token(mut self, value: impl Into<String>) -> Self {
                        self.access_token = Some(value.into());
                        self
                    }
                    #[doc = "JSONP"]
                    pub fn callback(mut self, value: impl Into<String>) -> Self {
                        self.callback = Some(value.into());
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
                    #[doc = "Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters."]
                    pub fn quota_user(mut self, value: impl Into<String>) -> Self {
                        self.quota_user = Some(value.into());
                        self
                    }
                    #[doc = "Upload protocol for media (e.g. \"raw\", \"multipart\")."]
                    pub fn upload_protocol(mut self, value: impl Into<String>) -> Self {
                        self.upload_protocol = Some(value.into());
                        self
                    }
                    #[doc = "Legacy upload protocol for media (e.g. \"media\", \"multipart\")."]
                    pub fn upload_type(mut self, value: impl Into<String>) -> Self {
                        self.upload_type = Some(value.into());
                        self
                    }
                    #[doc = "V1 error format."]
                    pub fn xgafv(mut self, value: crate::params::Xgafv) -> Self {
                        self.xgafv = Some(value);
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
                    ) -> Result<crate::schemas::Snapshot, Box<dyn ::std::error::Error>>
                    {
                        self.execute_fields::<_, &str>(None)
                    }
                    #[doc = r" Execute the given operation. This will provide a `fields`"]
                    #[doc = r" selector of `*`. This will include every attribute of the"]
                    #[doc = r" response resource and should be limited to use during"]
                    #[doc = r" development or debugging."]
                    pub fn execute_debug(
                        self,
                    ) -> Result<crate::schemas::Snapshot, Box<dyn ::std::error::Error>>
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
                        let mut output = "https://dataflow.googleapis.com/".to_owned();
                        output.push_str("v1b3/projects/");
                        {
                            let var_as_str = &self.project_id;
                            output.extend(::percent_encoding::utf8_percent_encode(
                                &var_as_str,
                                crate::SIMPLE,
                            ));
                        }
                        output.push_str("/locations/");
                        {
                            let var_as_str = &self.location;
                            output.extend(::percent_encoding::utf8_percent_encode(
                                &var_as_str,
                                crate::SIMPLE,
                            ));
                        }
                        output.push_str("/snapshots/");
                        {
                            let var_as_str = &self.snapshot_id;
                            output.extend(::percent_encoding::utf8_percent_encode(
                                &var_as_str,
                                crate::SIMPLE,
                            ));
                        }
                        output
                    }
                    fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
                        let req = self.reqwest.request(::reqwest::Method::GET, path);
                        let req = req.query(&[("access_token", &self.access_token)]);
                        let req = req.query(&[("alt", &self.alt)]);
                        let req = req.query(&[("callback", &self.callback)]);
                        let req = req.query(&[("fields", &self.fields)]);
                        let req = req.query(&[("key", &self.key)]);
                        let req = req.query(&[("oauth_token", &self.oauth_token)]);
                        let req = req.query(&[("prettyPrint", &self.pretty_print)]);
                        let req = req.query(&[("quotaUser", &self.quota_user)]);
                        let req = req.query(&[("upload_protocol", &self.upload_protocol)]);
                        let req = req.query(&[("uploadType", &self.upload_type)]);
                        let req = req.query(&[("$.xgafv", &self.xgafv)]);
                        let mut auth = self.auth.lock().unwrap();
                        let fut =
                            auth.token(vec!["https://www.googleapis.com/auth/compute.readonly"]);
                        let mut runtime = ::tokio::runtime::Runtime::new().unwrap();
                        let token = runtime.block_on(fut).unwrap().access_token;
                        let req = req.bearer_auth(&token);
                        req
                    }
                }
                #[derive(Debug, Clone)]
                pub struct ListRequestBuilder<'a, A> {
                    pub(crate) reqwest: &'a ::reqwest::Client,
                    pub(crate) auth: &'a ::std::sync::Mutex<A>,
                    project_id: String,
                    location: String,
                    job_id: Option<String>,
                    access_token: Option<String>,
                    alt: Option<crate::params::Alt>,
                    callback: Option<String>,
                    fields: Option<String>,
                    key: Option<String>,
                    oauth_token: Option<String>,
                    pretty_print: Option<bool>,
                    quota_user: Option<String>,
                    upload_protocol: Option<String>,
                    upload_type: Option<String>,
                    xgafv: Option<crate::params::Xgafv>,
                }
                impl<'a, A: yup_oauth2::GetToken> ListRequestBuilder<'a, A> {
                    #[doc = "If specified, list snapshots created from this job."]
                    pub fn job_id(mut self, value: impl Into<String>) -> Self {
                        self.job_id = Some(value.into());
                        self
                    }
                    #[doc = "OAuth access token."]
                    pub fn access_token(mut self, value: impl Into<String>) -> Self {
                        self.access_token = Some(value.into());
                        self
                    }
                    #[doc = "JSONP"]
                    pub fn callback(mut self, value: impl Into<String>) -> Self {
                        self.callback = Some(value.into());
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
                    #[doc = "Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters."]
                    pub fn quota_user(mut self, value: impl Into<String>) -> Self {
                        self.quota_user = Some(value.into());
                        self
                    }
                    #[doc = "Upload protocol for media (e.g. \"raw\", \"multipart\")."]
                    pub fn upload_protocol(mut self, value: impl Into<String>) -> Self {
                        self.upload_protocol = Some(value.into());
                        self
                    }
                    #[doc = "Legacy upload protocol for media (e.g. \"media\", \"multipart\")."]
                    pub fn upload_type(mut self, value: impl Into<String>) -> Self {
                        self.upload_type = Some(value.into());
                        self
                    }
                    #[doc = "V1 error format."]
                    pub fn xgafv(mut self, value: crate::params::Xgafv) -> Self {
                        self.xgafv = Some(value);
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
                    ) -> Result<crate::schemas::ListSnapshotsResponse, Box<dyn ::std::error::Error>>
                    {
                        self.execute_fields::<_, &str>(None)
                    }
                    #[doc = r" Execute the given operation. This will provide a `fields`"]
                    #[doc = r" selector of `*`. This will include every attribute of the"]
                    #[doc = r" response resource and should be limited to use during"]
                    #[doc = r" development or debugging."]
                    pub fn execute_debug(
                        self,
                    ) -> Result<crate::schemas::ListSnapshotsResponse, Box<dyn ::std::error::Error>>
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
                        let mut output = "https://dataflow.googleapis.com/".to_owned();
                        output.push_str("v1b3/projects/");
                        {
                            let var_as_str = &self.project_id;
                            output.extend(::percent_encoding::utf8_percent_encode(
                                &var_as_str,
                                crate::SIMPLE,
                            ));
                        }
                        output.push_str("/locations/");
                        {
                            let var_as_str = &self.location;
                            output.extend(::percent_encoding::utf8_percent_encode(
                                &var_as_str,
                                crate::SIMPLE,
                            ));
                        }
                        output.push_str("/snapshots");
                        output
                    }
                    fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
                        let req = self.reqwest.request(::reqwest::Method::GET, path);
                        let req = req.query(&[("jobId", &self.job_id)]);
                        let req = req.query(&[("access_token", &self.access_token)]);
                        let req = req.query(&[("alt", &self.alt)]);
                        let req = req.query(&[("callback", &self.callback)]);
                        let req = req.query(&[("fields", &self.fields)]);
                        let req = req.query(&[("key", &self.key)]);
                        let req = req.query(&[("oauth_token", &self.oauth_token)]);
                        let req = req.query(&[("prettyPrint", &self.pretty_print)]);
                        let req = req.query(&[("quotaUser", &self.quota_user)]);
                        let req = req.query(&[("upload_protocol", &self.upload_protocol)]);
                        let req = req.query(&[("uploadType", &self.upload_type)]);
                        let req = req.query(&[("$.xgafv", &self.xgafv)]);
                        let mut auth = self.auth.lock().unwrap();
                        let fut =
                            auth.token(vec!["https://www.googleapis.com/auth/compute.readonly"]);
                        let mut runtime = ::tokio::runtime::Runtime::new().unwrap();
                        let token = runtime.block_on(fut).unwrap().access_token;
                        let req = req.bearer_auth(&token);
                        req
                    }
                }
            }
            pub mod sql {
                pub mod params {}
                pub struct SqlActions<'a, A> {
                    pub(crate) reqwest: &'a reqwest::Client,
                    pub(crate) auth: &'a std::sync::Mutex<A>,
                }
                impl<'a, A: yup_oauth2::GetToken> SqlActions<'a, A> {
                    #[doc = "Validates a GoogleSQL query for Cloud Dataflow syntax. Will always\nconfirm the given query parses correctly, and if able to look up\nschema information from DataCatalog, will validate that the query\nanalyzes properly as well."]
                    pub fn validate(
                        &self,
                        project_id: impl Into<String>,
                        location: impl Into<String>,
                    ) -> ValidateRequestBuilder<A> {
                        ValidateRequestBuilder {
                            reqwest: &self.reqwest,
                            auth: &self.auth,
                            access_token: None,
                            alt: None,
                            callback: None,
                            fields: None,
                            key: None,
                            oauth_token: None,
                            pretty_print: None,
                            quota_user: None,
                            upload_protocol: None,
                            upload_type: None,
                            xgafv: None,
                            project_id: project_id.into(),
                            location: location.into(),
                            query: None,
                        }
                    }
                }
                #[derive(Debug, Clone)]
                pub struct ValidateRequestBuilder<'a, A> {
                    pub(crate) reqwest: &'a ::reqwest::Client,
                    pub(crate) auth: &'a ::std::sync::Mutex<A>,
                    project_id: String,
                    location: String,
                    query: Option<String>,
                    access_token: Option<String>,
                    alt: Option<crate::params::Alt>,
                    callback: Option<String>,
                    fields: Option<String>,
                    key: Option<String>,
                    oauth_token: Option<String>,
                    pretty_print: Option<bool>,
                    quota_user: Option<String>,
                    upload_protocol: Option<String>,
                    upload_type: Option<String>,
                    xgafv: Option<crate::params::Xgafv>,
                }
                impl<'a, A: yup_oauth2::GetToken> ValidateRequestBuilder<'a, A> {
                    #[doc = "The sql query to validate."]
                    pub fn query(mut self, value: impl Into<String>) -> Self {
                        self.query = Some(value.into());
                        self
                    }
                    #[doc = "OAuth access token."]
                    pub fn access_token(mut self, value: impl Into<String>) -> Self {
                        self.access_token = Some(value.into());
                        self
                    }
                    #[doc = "JSONP"]
                    pub fn callback(mut self, value: impl Into<String>) -> Self {
                        self.callback = Some(value.into());
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
                    #[doc = "Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters."]
                    pub fn quota_user(mut self, value: impl Into<String>) -> Self {
                        self.quota_user = Some(value.into());
                        self
                    }
                    #[doc = "Upload protocol for media (e.g. \"raw\", \"multipart\")."]
                    pub fn upload_protocol(mut self, value: impl Into<String>) -> Self {
                        self.upload_protocol = Some(value.into());
                        self
                    }
                    #[doc = "Legacy upload protocol for media (e.g. \"media\", \"multipart\")."]
                    pub fn upload_type(mut self, value: impl Into<String>) -> Self {
                        self.upload_type = Some(value.into());
                        self
                    }
                    #[doc = "V1 error format."]
                    pub fn xgafv(mut self, value: crate::params::Xgafv) -> Self {
                        self.xgafv = Some(value);
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
                    ) -> Result<crate::schemas::ValidateResponse, Box<dyn ::std::error::Error>>
                    {
                        self.execute_fields::<_, &str>(None)
                    }
                    #[doc = r" Execute the given operation. This will provide a `fields`"]
                    #[doc = r" selector of `*`. This will include every attribute of the"]
                    #[doc = r" response resource and should be limited to use during"]
                    #[doc = r" development or debugging."]
                    pub fn execute_debug(
                        self,
                    ) -> Result<crate::schemas::ValidateResponse, Box<dyn ::std::error::Error>>
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
                        let mut output = "https://dataflow.googleapis.com/".to_owned();
                        output.push_str("v1b3/projects/");
                        {
                            let var_as_str = &self.project_id;
                            output.extend(::percent_encoding::utf8_percent_encode(
                                &var_as_str,
                                crate::SIMPLE,
                            ));
                        }
                        output.push_str("/locations/");
                        {
                            let var_as_str = &self.location;
                            output.extend(::percent_encoding::utf8_percent_encode(
                                &var_as_str,
                                crate::SIMPLE,
                            ));
                        }
                        output.push_str("/sql:validate");
                        output
                    }
                    fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
                        let req = self.reqwest.request(::reqwest::Method::GET, path);
                        let req = req.query(&[("query", &self.query)]);
                        let req = req.query(&[("access_token", &self.access_token)]);
                        let req = req.query(&[("alt", &self.alt)]);
                        let req = req.query(&[("callback", &self.callback)]);
                        let req = req.query(&[("fields", &self.fields)]);
                        let req = req.query(&[("key", &self.key)]);
                        let req = req.query(&[("oauth_token", &self.oauth_token)]);
                        let req = req.query(&[("prettyPrint", &self.pretty_print)]);
                        let req = req.query(&[("quotaUser", &self.quota_user)]);
                        let req = req.query(&[("upload_protocol", &self.upload_protocol)]);
                        let req = req.query(&[("uploadType", &self.upload_type)]);
                        let req = req.query(&[("$.xgafv", &self.xgafv)]);
                        let mut auth = self.auth.lock().unwrap();
                        let fut =
                            auth.token(vec!["https://www.googleapis.com/auth/cloud-platform"]);
                        let mut runtime = ::tokio::runtime::Runtime::new().unwrap();
                        let token = runtime.block_on(fut).unwrap().access_token;
                        let req = req.bearer_auth(&token);
                        req
                    }
                }
            }
            pub mod templates {
                pub mod params {
                    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
                    pub enum GetView {
                        MetadataOnly,
                    }
                    impl GetView {
                        pub fn as_str(self) -> &'static str {
                            match self {
                                GetView::MetadataOnly => "METADATA_ONLY",
                            }
                        }
                    }
                    impl ::std::fmt::Display for GetView {
                        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                            f.write_str(self.as_str())
                        }
                    }
                    impl ::serde::Serialize for GetView {
                        fn serialize<S>(
                            &self,
                            serializer: S,
                        ) -> ::std::result::Result<S::Ok, S::Error>
                        where
                            S: ::serde::ser::Serializer,
                        {
                            serializer.serialize_str(self.as_str())
                        }
                    }
                    impl<'de> ::serde::Deserialize<'de> for GetView {
                        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
                        where
                            D: ::serde::de::Deserializer<'de>,
                        {
                            let value: &'de str = <&str>::deserialize(deserializer)?;
                            Ok(match value {
                                "METADATA_ONLY" => GetView::MetadataOnly,
                                _ => {
                                    return Err(::serde::de::Error::custom(format!(
                                        "invalid enum for #name: {}",
                                        value
                                    )))
                                }
                            })
                        }
                    }
                    impl ::field_selector::FieldSelector for GetView {
                        fn field_selector_with_ident(ident: &str, selector: &mut String) {
                            match selector.chars().rev().nth(0) {
                                Some(',') | None => {}
                                _ => selector.push_str(","),
                            }
                            selector.push_str(ident);
                        }
                    }
                }
                pub struct TemplatesActions<'a, A> {
                    pub(crate) reqwest: &'a reqwest::Client,
                    pub(crate) auth: &'a std::sync::Mutex<A>,
                }
                impl<'a, A: yup_oauth2::GetToken> TemplatesActions<'a, A> {
                    #[doc = "Creates a Cloud Dataflow job from a template."]
                    pub fn create(
                        &self,
                        request: crate::schemas::CreateJobFromTemplateRequest,
                        project_id: impl Into<String>,
                        location: impl Into<String>,
                    ) -> CreateRequestBuilder<A> {
                        CreateRequestBuilder {
                            reqwest: &self.reqwest,
                            auth: &self.auth,
                            request,
                            access_token: None,
                            alt: None,
                            callback: None,
                            fields: None,
                            key: None,
                            oauth_token: None,
                            pretty_print: None,
                            quota_user: None,
                            upload_protocol: None,
                            upload_type: None,
                            xgafv: None,
                            project_id: project_id.into(),
                            location: location.into(),
                        }
                    }
                    #[doc = "Get the template associated with a template."]
                    pub fn get(
                        &self,
                        project_id: impl Into<String>,
                        location: impl Into<String>,
                    ) -> GetRequestBuilder<A> {
                        GetRequestBuilder {
                            reqwest: &self.reqwest,
                            auth: &self.auth,
                            access_token: None,
                            alt: None,
                            callback: None,
                            fields: None,
                            key: None,
                            oauth_token: None,
                            pretty_print: None,
                            quota_user: None,
                            upload_protocol: None,
                            upload_type: None,
                            xgafv: None,
                            project_id: project_id.into(),
                            location: location.into(),
                            gcs_path: None,
                            view: None,
                        }
                    }
                    #[doc = "Launch a template."]
                    pub fn launch(
                        &self,
                        request: crate::schemas::LaunchTemplateParameters,
                        project_id: impl Into<String>,
                        location: impl Into<String>,
                    ) -> LaunchRequestBuilder<A> {
                        LaunchRequestBuilder {
                            reqwest: &self.reqwest,
                            auth: &self.auth,
                            request,
                            access_token: None,
                            alt: None,
                            callback: None,
                            fields: None,
                            key: None,
                            oauth_token: None,
                            pretty_print: None,
                            quota_user: None,
                            upload_protocol: None,
                            upload_type: None,
                            xgafv: None,
                            project_id: project_id.into(),
                            location: location.into(),
                            dynamic_template_gcs_path: None,
                            dynamic_template_staging_location: None,
                            gcs_path: None,
                            validate_only: None,
                        }
                    }
                }
                #[derive(Debug, Clone)]
                pub struct CreateRequestBuilder<'a, A> {
                    pub(crate) reqwest: &'a ::reqwest::Client,
                    pub(crate) auth: &'a ::std::sync::Mutex<A>,
                    request: crate::schemas::CreateJobFromTemplateRequest,
                    project_id: String,
                    location: String,
                    access_token: Option<String>,
                    alt: Option<crate::params::Alt>,
                    callback: Option<String>,
                    fields: Option<String>,
                    key: Option<String>,
                    oauth_token: Option<String>,
                    pretty_print: Option<bool>,
                    quota_user: Option<String>,
                    upload_protocol: Option<String>,
                    upload_type: Option<String>,
                    xgafv: Option<crate::params::Xgafv>,
                }
                impl<'a, A: yup_oauth2::GetToken> CreateRequestBuilder<'a, A> {
                    #[doc = "OAuth access token."]
                    pub fn access_token(mut self, value: impl Into<String>) -> Self {
                        self.access_token = Some(value.into());
                        self
                    }
                    #[doc = "JSONP"]
                    pub fn callback(mut self, value: impl Into<String>) -> Self {
                        self.callback = Some(value.into());
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
                    #[doc = "Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters."]
                    pub fn quota_user(mut self, value: impl Into<String>) -> Self {
                        self.quota_user = Some(value.into());
                        self
                    }
                    #[doc = "Upload protocol for media (e.g. \"raw\", \"multipart\")."]
                    pub fn upload_protocol(mut self, value: impl Into<String>) -> Self {
                        self.upload_protocol = Some(value.into());
                        self
                    }
                    #[doc = "Legacy upload protocol for media (e.g. \"media\", \"multipart\")."]
                    pub fn upload_type(mut self, value: impl Into<String>) -> Self {
                        self.upload_type = Some(value.into());
                        self
                    }
                    #[doc = "V1 error format."]
                    pub fn xgafv(mut self, value: crate::params::Xgafv) -> Self {
                        self.xgafv = Some(value);
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
                    ) -> Result<crate::schemas::Job, Box<dyn ::std::error::Error>>
                    {
                        self.execute_fields::<_, &str>(None)
                    }
                    #[doc = r" Execute the given operation. This will provide a `fields`"]
                    #[doc = r" selector of `*`. This will include every attribute of the"]
                    #[doc = r" response resource and should be limited to use during"]
                    #[doc = r" development or debugging."]
                    pub fn execute_debug(
                        self,
                    ) -> Result<crate::schemas::Job, Box<dyn ::std::error::Error>>
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
                        let mut output = "https://dataflow.googleapis.com/".to_owned();
                        output.push_str("v1b3/projects/");
                        {
                            let var_as_str = &self.project_id;
                            output.extend(::percent_encoding::utf8_percent_encode(
                                &var_as_str,
                                crate::SIMPLE,
                            ));
                        }
                        output.push_str("/locations/");
                        {
                            let var_as_str = &self.location;
                            output.extend(::percent_encoding::utf8_percent_encode(
                                &var_as_str,
                                crate::SIMPLE,
                            ));
                        }
                        output.push_str("/templates");
                        output
                    }
                    fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
                        let req = self.reqwest.request(::reqwest::Method::POST, path);
                        let req = req.query(&[("access_token", &self.access_token)]);
                        let req = req.query(&[("alt", &self.alt)]);
                        let req = req.query(&[("callback", &self.callback)]);
                        let req = req.query(&[("fields", &self.fields)]);
                        let req = req.query(&[("key", &self.key)]);
                        let req = req.query(&[("oauth_token", &self.oauth_token)]);
                        let req = req.query(&[("prettyPrint", &self.pretty_print)]);
                        let req = req.query(&[("quotaUser", &self.quota_user)]);
                        let req = req.query(&[("upload_protocol", &self.upload_protocol)]);
                        let req = req.query(&[("uploadType", &self.upload_type)]);
                        let req = req.query(&[("$.xgafv", &self.xgafv)]);
                        let mut auth = self.auth.lock().unwrap();
                        let fut =
                            auth.token(vec!["https://www.googleapis.com/auth/cloud-platform"]);
                        let mut runtime = ::tokio::runtime::Runtime::new().unwrap();
                        let token = runtime.block_on(fut).unwrap().access_token;
                        let req = req.bearer_auth(&token);
                        req
                    }
                }
                #[derive(Debug, Clone)]
                pub struct GetRequestBuilder<'a, A> {
                    pub(crate) reqwest: &'a ::reqwest::Client,
                    pub(crate) auth: &'a ::std::sync::Mutex<A>,
                    project_id: String,
                    location: String,
                    gcs_path: Option<String>,
                    view: Option<crate::resources::projects::locations::templates::params::GetView>,
                    access_token: Option<String>,
                    alt: Option<crate::params::Alt>,
                    callback: Option<String>,
                    fields: Option<String>,
                    key: Option<String>,
                    oauth_token: Option<String>,
                    pretty_print: Option<bool>,
                    quota_user: Option<String>,
                    upload_protocol: Option<String>,
                    upload_type: Option<String>,
                    xgafv: Option<crate::params::Xgafv>,
                }
                impl<'a, A: yup_oauth2::GetToken> GetRequestBuilder<'a, A> {
                    #[doc = "Required. A Cloud Storage path to the template from which to\ncreate the job.\nMust be valid Cloud Storage URL, beginning with 'gs://'."]
                    pub fn gcs_path(mut self, value: impl Into<String>) -> Self {
                        self.gcs_path = Some(value.into());
                        self
                    }
                    #[doc = "The view to retrieve. Defaults to METADATA_ONLY."]
                    pub fn view(
                        mut self,
                        value: crate::resources::projects::locations::templates::params::GetView,
                    ) -> Self {
                        self.view = Some(value);
                        self
                    }
                    #[doc = "OAuth access token."]
                    pub fn access_token(mut self, value: impl Into<String>) -> Self {
                        self.access_token = Some(value.into());
                        self
                    }
                    #[doc = "JSONP"]
                    pub fn callback(mut self, value: impl Into<String>) -> Self {
                        self.callback = Some(value.into());
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
                    #[doc = "Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters."]
                    pub fn quota_user(mut self, value: impl Into<String>) -> Self {
                        self.quota_user = Some(value.into());
                        self
                    }
                    #[doc = "Upload protocol for media (e.g. \"raw\", \"multipart\")."]
                    pub fn upload_protocol(mut self, value: impl Into<String>) -> Self {
                        self.upload_protocol = Some(value.into());
                        self
                    }
                    #[doc = "Legacy upload protocol for media (e.g. \"media\", \"multipart\")."]
                    pub fn upload_type(mut self, value: impl Into<String>) -> Self {
                        self.upload_type = Some(value.into());
                        self
                    }
                    #[doc = "V1 error format."]
                    pub fn xgafv(mut self, value: crate::params::Xgafv) -> Self {
                        self.xgafv = Some(value);
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
                    ) -> Result<crate::schemas::GetTemplateResponse, Box<dyn ::std::error::Error>>
                    {
                        self.execute_fields::<_, &str>(None)
                    }
                    #[doc = r" Execute the given operation. This will provide a `fields`"]
                    #[doc = r" selector of `*`. This will include every attribute of the"]
                    #[doc = r" response resource and should be limited to use during"]
                    #[doc = r" development or debugging."]
                    pub fn execute_debug(
                        self,
                    ) -> Result<crate::schemas::GetTemplateResponse, Box<dyn ::std::error::Error>>
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
                        let mut output = "https://dataflow.googleapis.com/".to_owned();
                        output.push_str("v1b3/projects/");
                        {
                            let var_as_str = &self.project_id;
                            output.extend(::percent_encoding::utf8_percent_encode(
                                &var_as_str,
                                crate::SIMPLE,
                            ));
                        }
                        output.push_str("/locations/");
                        {
                            let var_as_str = &self.location;
                            output.extend(::percent_encoding::utf8_percent_encode(
                                &var_as_str,
                                crate::SIMPLE,
                            ));
                        }
                        output.push_str("/templates:get");
                        output
                    }
                    fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
                        let req = self.reqwest.request(::reqwest::Method::GET, path);
                        let req = req.query(&[("gcsPath", &self.gcs_path)]);
                        let req = req.query(&[("view", &self.view)]);
                        let req = req.query(&[("access_token", &self.access_token)]);
                        let req = req.query(&[("alt", &self.alt)]);
                        let req = req.query(&[("callback", &self.callback)]);
                        let req = req.query(&[("fields", &self.fields)]);
                        let req = req.query(&[("key", &self.key)]);
                        let req = req.query(&[("oauth_token", &self.oauth_token)]);
                        let req = req.query(&[("prettyPrint", &self.pretty_print)]);
                        let req = req.query(&[("quotaUser", &self.quota_user)]);
                        let req = req.query(&[("upload_protocol", &self.upload_protocol)]);
                        let req = req.query(&[("uploadType", &self.upload_type)]);
                        let req = req.query(&[("$.xgafv", &self.xgafv)]);
                        let mut auth = self.auth.lock().unwrap();
                        let fut =
                            auth.token(vec!["https://www.googleapis.com/auth/compute.readonly"]);
                        let mut runtime = ::tokio::runtime::Runtime::new().unwrap();
                        let token = runtime.block_on(fut).unwrap().access_token;
                        let req = req.bearer_auth(&token);
                        req
                    }
                }
                #[derive(Debug, Clone)]
                pub struct LaunchRequestBuilder<'a, A> {
                    pub(crate) reqwest: &'a ::reqwest::Client,
                    pub(crate) auth: &'a ::std::sync::Mutex<A>,
                    request: crate::schemas::LaunchTemplateParameters,
                    project_id: String,
                    location: String,
                    dynamic_template_gcs_path: Option<String>,
                    dynamic_template_staging_location: Option<String>,
                    gcs_path: Option<String>,
                    validate_only: Option<bool>,
                    access_token: Option<String>,
                    alt: Option<crate::params::Alt>,
                    callback: Option<String>,
                    fields: Option<String>,
                    key: Option<String>,
                    oauth_token: Option<String>,
                    pretty_print: Option<bool>,
                    quota_user: Option<String>,
                    upload_protocol: Option<String>,
                    upload_type: Option<String>,
                    xgafv: Option<crate::params::Xgafv>,
                }
                impl<'a, A: yup_oauth2::GetToken> LaunchRequestBuilder<'a, A> {
                    #[doc = "Path to dynamic template spec file on GCS.\nThe file must be a Json serialized DynamicTemplateFieSpec object."]
                    pub fn dynamic_template_gcs_path(mut self, value: impl Into<String>) -> Self {
                        self.dynamic_template_gcs_path = Some(value.into());
                        self
                    }
                    #[doc = "Cloud Storage path for staging dependencies.\nMust be a valid Cloud Storage URL, beginning with `gs://`."]
                    pub fn dynamic_template_staging_location(
                        mut self,
                        value: impl Into<String>,
                    ) -> Self {
                        self.dynamic_template_staging_location = Some(value.into());
                        self
                    }
                    #[doc = "A Cloud Storage path to the template from which to create\nthe job.\nMust be valid Cloud Storage URL, beginning with 'gs://'."]
                    pub fn gcs_path(mut self, value: impl Into<String>) -> Self {
                        self.gcs_path = Some(value.into());
                        self
                    }
                    #[doc = "If true, the request is validated but not actually executed.\nDefaults to false."]
                    pub fn validate_only(mut self, value: bool) -> Self {
                        self.validate_only = Some(value);
                        self
                    }
                    #[doc = "OAuth access token."]
                    pub fn access_token(mut self, value: impl Into<String>) -> Self {
                        self.access_token = Some(value.into());
                        self
                    }
                    #[doc = "JSONP"]
                    pub fn callback(mut self, value: impl Into<String>) -> Self {
                        self.callback = Some(value.into());
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
                    #[doc = "Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters."]
                    pub fn quota_user(mut self, value: impl Into<String>) -> Self {
                        self.quota_user = Some(value.into());
                        self
                    }
                    #[doc = "Upload protocol for media (e.g. \"raw\", \"multipart\")."]
                    pub fn upload_protocol(mut self, value: impl Into<String>) -> Self {
                        self.upload_protocol = Some(value.into());
                        self
                    }
                    #[doc = "Legacy upload protocol for media (e.g. \"media\", \"multipart\")."]
                    pub fn upload_type(mut self, value: impl Into<String>) -> Self {
                        self.upload_type = Some(value.into());
                        self
                    }
                    #[doc = "V1 error format."]
                    pub fn xgafv(mut self, value: crate::params::Xgafv) -> Self {
                        self.xgafv = Some(value);
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
                    ) -> Result<crate::schemas::LaunchTemplateResponse, Box<dyn ::std::error::Error>>
                    {
                        self.execute_fields::<_, &str>(None)
                    }
                    #[doc = r" Execute the given operation. This will provide a `fields`"]
                    #[doc = r" selector of `*`. This will include every attribute of the"]
                    #[doc = r" response resource and should be limited to use during"]
                    #[doc = r" development or debugging."]
                    pub fn execute_debug(
                        self,
                    ) -> Result<crate::schemas::LaunchTemplateResponse, Box<dyn ::std::error::Error>>
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
                        let mut output = "https://dataflow.googleapis.com/".to_owned();
                        output.push_str("v1b3/projects/");
                        {
                            let var_as_str = &self.project_id;
                            output.extend(::percent_encoding::utf8_percent_encode(
                                &var_as_str,
                                crate::SIMPLE,
                            ));
                        }
                        output.push_str("/locations/");
                        {
                            let var_as_str = &self.location;
                            output.extend(::percent_encoding::utf8_percent_encode(
                                &var_as_str,
                                crate::SIMPLE,
                            ));
                        }
                        output.push_str("/templates:launch");
                        output
                    }
                    fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
                        let req = self.reqwest.request(::reqwest::Method::POST, path);
                        let req = req
                            .query(&[("dynamicTemplate.gcsPath", &self.dynamic_template_gcs_path)]);
                        let req = req.query(&[(
                            "dynamicTemplate.stagingLocation",
                            &self.dynamic_template_staging_location,
                        )]);
                        let req = req.query(&[("gcsPath", &self.gcs_path)]);
                        let req = req.query(&[("validateOnly", &self.validate_only)]);
                        let req = req.query(&[("access_token", &self.access_token)]);
                        let req = req.query(&[("alt", &self.alt)]);
                        let req = req.query(&[("callback", &self.callback)]);
                        let req = req.query(&[("fields", &self.fields)]);
                        let req = req.query(&[("key", &self.key)]);
                        let req = req.query(&[("oauth_token", &self.oauth_token)]);
                        let req = req.query(&[("prettyPrint", &self.pretty_print)]);
                        let req = req.query(&[("quotaUser", &self.quota_user)]);
                        let req = req.query(&[("upload_protocol", &self.upload_protocol)]);
                        let req = req.query(&[("uploadType", &self.upload_type)]);
                        let req = req.query(&[("$.xgafv", &self.xgafv)]);
                        let mut auth = self.auth.lock().unwrap();
                        let fut =
                            auth.token(vec!["https://www.googleapis.com/auth/cloud-platform"]);
                        let mut runtime = ::tokio::runtime::Runtime::new().unwrap();
                        let token = runtime.block_on(fut).unwrap().access_token;
                        let req = req.bearer_auth(&token);
                        req
                    }
                }
            }
        }
        pub mod snapshots {
            pub mod params {}
            pub struct SnapshotsActions<'a, A> {
                pub(crate) reqwest: &'a reqwest::Client,
                pub(crate) auth: &'a std::sync::Mutex<A>,
            }
            impl<'a, A: yup_oauth2::GetToken> SnapshotsActions<'a, A> {
                #[doc = "Gets information about a snapshot."]
                pub fn get(
                    &self,
                    project_id: impl Into<String>,
                    snapshot_id: impl Into<String>,
                ) -> GetRequestBuilder<A> {
                    GetRequestBuilder {
                        reqwest: &self.reqwest,
                        auth: &self.auth,
                        access_token: None,
                        alt: None,
                        callback: None,
                        fields: None,
                        key: None,
                        oauth_token: None,
                        pretty_print: None,
                        quota_user: None,
                        upload_protocol: None,
                        upload_type: None,
                        xgafv: None,
                        project_id: project_id.into(),
                        snapshot_id: snapshot_id.into(),
                        location: None,
                    }
                }
                #[doc = "Lists snapshots."]
                pub fn list(&self, project_id: impl Into<String>) -> ListRequestBuilder<A> {
                    ListRequestBuilder {
                        reqwest: &self.reqwest,
                        auth: &self.auth,
                        access_token: None,
                        alt: None,
                        callback: None,
                        fields: None,
                        key: None,
                        oauth_token: None,
                        pretty_print: None,
                        quota_user: None,
                        upload_protocol: None,
                        upload_type: None,
                        xgafv: None,
                        project_id: project_id.into(),
                        job_id: None,
                        location: None,
                    }
                }
            }
            #[derive(Debug, Clone)]
            pub struct GetRequestBuilder<'a, A> {
                pub(crate) reqwest: &'a ::reqwest::Client,
                pub(crate) auth: &'a ::std::sync::Mutex<A>,
                project_id: String,
                snapshot_id: String,
                location: Option<String>,
                access_token: Option<String>,
                alt: Option<crate::params::Alt>,
                callback: Option<String>,
                fields: Option<String>,
                key: Option<String>,
                oauth_token: Option<String>,
                pretty_print: Option<bool>,
                quota_user: Option<String>,
                upload_protocol: Option<String>,
                upload_type: Option<String>,
                xgafv: Option<crate::params::Xgafv>,
            }
            impl<'a, A: yup_oauth2::GetToken> GetRequestBuilder<'a, A> {
                #[doc = "The location that contains this snapshot."]
                pub fn location(mut self, value: impl Into<String>) -> Self {
                    self.location = Some(value.into());
                    self
                }
                #[doc = "OAuth access token."]
                pub fn access_token(mut self, value: impl Into<String>) -> Self {
                    self.access_token = Some(value.into());
                    self
                }
                #[doc = "JSONP"]
                pub fn callback(mut self, value: impl Into<String>) -> Self {
                    self.callback = Some(value.into());
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
                #[doc = "Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters."]
                pub fn quota_user(mut self, value: impl Into<String>) -> Self {
                    self.quota_user = Some(value.into());
                    self
                }
                #[doc = "Upload protocol for media (e.g. \"raw\", \"multipart\")."]
                pub fn upload_protocol(mut self, value: impl Into<String>) -> Self {
                    self.upload_protocol = Some(value.into());
                    self
                }
                #[doc = "Legacy upload protocol for media (e.g. \"media\", \"multipart\")."]
                pub fn upload_type(mut self, value: impl Into<String>) -> Self {
                    self.upload_type = Some(value.into());
                    self
                }
                #[doc = "V1 error format."]
                pub fn xgafv(mut self, value: crate::params::Xgafv) -> Self {
                    self.xgafv = Some(value);
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
                ) -> Result<crate::schemas::Snapshot, Box<dyn ::std::error::Error>>
                {
                    self.execute_fields::<_, &str>(None)
                }
                #[doc = r" Execute the given operation. This will provide a `fields`"]
                #[doc = r" selector of `*`. This will include every attribute of the"]
                #[doc = r" response resource and should be limited to use during"]
                #[doc = r" development or debugging."]
                pub fn execute_debug(
                    self,
                ) -> Result<crate::schemas::Snapshot, Box<dyn ::std::error::Error>>
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
                    let mut output = "https://dataflow.googleapis.com/".to_owned();
                    output.push_str("v1b3/projects/");
                    {
                        let var_as_str = &self.project_id;
                        output.extend(::percent_encoding::utf8_percent_encode(
                            &var_as_str,
                            crate::SIMPLE,
                        ));
                    }
                    output.push_str("/snapshots/");
                    {
                        let var_as_str = &self.snapshot_id;
                        output.extend(::percent_encoding::utf8_percent_encode(
                            &var_as_str,
                            crate::SIMPLE,
                        ));
                    }
                    output
                }
                fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
                    let req = self.reqwest.request(::reqwest::Method::GET, path);
                    let req = req.query(&[("location", &self.location)]);
                    let req = req.query(&[("access_token", &self.access_token)]);
                    let req = req.query(&[("alt", &self.alt)]);
                    let req = req.query(&[("callback", &self.callback)]);
                    let req = req.query(&[("fields", &self.fields)]);
                    let req = req.query(&[("key", &self.key)]);
                    let req = req.query(&[("oauth_token", &self.oauth_token)]);
                    let req = req.query(&[("prettyPrint", &self.pretty_print)]);
                    let req = req.query(&[("quotaUser", &self.quota_user)]);
                    let req = req.query(&[("upload_protocol", &self.upload_protocol)]);
                    let req = req.query(&[("uploadType", &self.upload_type)]);
                    let req = req.query(&[("$.xgafv", &self.xgafv)]);
                    let mut auth = self.auth.lock().unwrap();
                    let fut = auth.token(vec!["https://www.googleapis.com/auth/compute.readonly"]);
                    let mut runtime = ::tokio::runtime::Runtime::new().unwrap();
                    let token = runtime.block_on(fut).unwrap().access_token;
                    let req = req.bearer_auth(&token);
                    req
                }
            }
            #[derive(Debug, Clone)]
            pub struct ListRequestBuilder<'a, A> {
                pub(crate) reqwest: &'a ::reqwest::Client,
                pub(crate) auth: &'a ::std::sync::Mutex<A>,
                project_id: String,
                job_id: Option<String>,
                location: Option<String>,
                access_token: Option<String>,
                alt: Option<crate::params::Alt>,
                callback: Option<String>,
                fields: Option<String>,
                key: Option<String>,
                oauth_token: Option<String>,
                pretty_print: Option<bool>,
                quota_user: Option<String>,
                upload_protocol: Option<String>,
                upload_type: Option<String>,
                xgafv: Option<crate::params::Xgafv>,
            }
            impl<'a, A: yup_oauth2::GetToken> ListRequestBuilder<'a, A> {
                #[doc = "If specified, list snapshots created from this job."]
                pub fn job_id(mut self, value: impl Into<String>) -> Self {
                    self.job_id = Some(value.into());
                    self
                }
                #[doc = "The location to list snapshots in."]
                pub fn location(mut self, value: impl Into<String>) -> Self {
                    self.location = Some(value.into());
                    self
                }
                #[doc = "OAuth access token."]
                pub fn access_token(mut self, value: impl Into<String>) -> Self {
                    self.access_token = Some(value.into());
                    self
                }
                #[doc = "JSONP"]
                pub fn callback(mut self, value: impl Into<String>) -> Self {
                    self.callback = Some(value.into());
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
                #[doc = "Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters."]
                pub fn quota_user(mut self, value: impl Into<String>) -> Self {
                    self.quota_user = Some(value.into());
                    self
                }
                #[doc = "Upload protocol for media (e.g. \"raw\", \"multipart\")."]
                pub fn upload_protocol(mut self, value: impl Into<String>) -> Self {
                    self.upload_protocol = Some(value.into());
                    self
                }
                #[doc = "Legacy upload protocol for media (e.g. \"media\", \"multipart\")."]
                pub fn upload_type(mut self, value: impl Into<String>) -> Self {
                    self.upload_type = Some(value.into());
                    self
                }
                #[doc = "V1 error format."]
                pub fn xgafv(mut self, value: crate::params::Xgafv) -> Self {
                    self.xgafv = Some(value);
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
                ) -> Result<crate::schemas::ListSnapshotsResponse, Box<dyn ::std::error::Error>>
                {
                    self.execute_fields::<_, &str>(None)
                }
                #[doc = r" Execute the given operation. This will provide a `fields`"]
                #[doc = r" selector of `*`. This will include every attribute of the"]
                #[doc = r" response resource and should be limited to use during"]
                #[doc = r" development or debugging."]
                pub fn execute_debug(
                    self,
                ) -> Result<crate::schemas::ListSnapshotsResponse, Box<dyn ::std::error::Error>>
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
                    let mut output = "https://dataflow.googleapis.com/".to_owned();
                    output.push_str("v1b3/projects/");
                    {
                        let var_as_str = &self.project_id;
                        output.extend(::percent_encoding::utf8_percent_encode(
                            &var_as_str,
                            crate::SIMPLE,
                        ));
                    }
                    output.push_str("/snapshots");
                    output
                }
                fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
                    let req = self.reqwest.request(::reqwest::Method::GET, path);
                    let req = req.query(&[("jobId", &self.job_id)]);
                    let req = req.query(&[("location", &self.location)]);
                    let req = req.query(&[("access_token", &self.access_token)]);
                    let req = req.query(&[("alt", &self.alt)]);
                    let req = req.query(&[("callback", &self.callback)]);
                    let req = req.query(&[("fields", &self.fields)]);
                    let req = req.query(&[("key", &self.key)]);
                    let req = req.query(&[("oauth_token", &self.oauth_token)]);
                    let req = req.query(&[("prettyPrint", &self.pretty_print)]);
                    let req = req.query(&[("quotaUser", &self.quota_user)]);
                    let req = req.query(&[("upload_protocol", &self.upload_protocol)]);
                    let req = req.query(&[("uploadType", &self.upload_type)]);
                    let req = req.query(&[("$.xgafv", &self.xgafv)]);
                    let mut auth = self.auth.lock().unwrap();
                    let fut = auth.token(vec!["https://www.googleapis.com/auth/compute.readonly"]);
                    let mut runtime = ::tokio::runtime::Runtime::new().unwrap();
                    let token = runtime.block_on(fut).unwrap().access_token;
                    let req = req.bearer_auth(&token);
                    req
                }
            }
        }
        pub mod templates {
            pub mod params {
                #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
                pub enum GetView {
                    MetadataOnly,
                }
                impl GetView {
                    pub fn as_str(self) -> &'static str {
                        match self {
                            GetView::MetadataOnly => "METADATA_ONLY",
                        }
                    }
                }
                impl ::std::fmt::Display for GetView {
                    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                        f.write_str(self.as_str())
                    }
                }
                impl ::serde::Serialize for GetView {
                    fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
                    where
                        S: ::serde::ser::Serializer,
                    {
                        serializer.serialize_str(self.as_str())
                    }
                }
                impl<'de> ::serde::Deserialize<'de> for GetView {
                    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
                    where
                        D: ::serde::de::Deserializer<'de>,
                    {
                        let value: &'de str = <&str>::deserialize(deserializer)?;
                        Ok(match value {
                            "METADATA_ONLY" => GetView::MetadataOnly,
                            _ => {
                                return Err(::serde::de::Error::custom(format!(
                                    "invalid enum for #name: {}",
                                    value
                                )))
                            }
                        })
                    }
                }
                impl ::field_selector::FieldSelector for GetView {
                    fn field_selector_with_ident(ident: &str, selector: &mut String) {
                        match selector.chars().rev().nth(0) {
                            Some(',') | None => {}
                            _ => selector.push_str(","),
                        }
                        selector.push_str(ident);
                    }
                }
            }
            pub struct TemplatesActions<'a, A> {
                pub(crate) reqwest: &'a reqwest::Client,
                pub(crate) auth: &'a std::sync::Mutex<A>,
            }
            impl<'a, A: yup_oauth2::GetToken> TemplatesActions<'a, A> {
                #[doc = "Creates a Cloud Dataflow job from a template."]
                pub fn create(
                    &self,
                    request: crate::schemas::CreateJobFromTemplateRequest,
                    project_id: impl Into<String>,
                ) -> CreateRequestBuilder<A> {
                    CreateRequestBuilder {
                        reqwest: &self.reqwest,
                        auth: &self.auth,
                        request,
                        access_token: None,
                        alt: None,
                        callback: None,
                        fields: None,
                        key: None,
                        oauth_token: None,
                        pretty_print: None,
                        quota_user: None,
                        upload_protocol: None,
                        upload_type: None,
                        xgafv: None,
                        project_id: project_id.into(),
                    }
                }
                #[doc = "Get the template associated with a template."]
                pub fn get(&self, project_id: impl Into<String>) -> GetRequestBuilder<A> {
                    GetRequestBuilder {
                        reqwest: &self.reqwest,
                        auth: &self.auth,
                        access_token: None,
                        alt: None,
                        callback: None,
                        fields: None,
                        key: None,
                        oauth_token: None,
                        pretty_print: None,
                        quota_user: None,
                        upload_protocol: None,
                        upload_type: None,
                        xgafv: None,
                        project_id: project_id.into(),
                        gcs_path: None,
                        location: None,
                        view: None,
                    }
                }
                #[doc = "Launch a template."]
                pub fn launch(
                    &self,
                    request: crate::schemas::LaunchTemplateParameters,
                    project_id: impl Into<String>,
                ) -> LaunchRequestBuilder<A> {
                    LaunchRequestBuilder {
                        reqwest: &self.reqwest,
                        auth: &self.auth,
                        request,
                        access_token: None,
                        alt: None,
                        callback: None,
                        fields: None,
                        key: None,
                        oauth_token: None,
                        pretty_print: None,
                        quota_user: None,
                        upload_protocol: None,
                        upload_type: None,
                        xgafv: None,
                        project_id: project_id.into(),
                        dynamic_template_gcs_path: None,
                        dynamic_template_staging_location: None,
                        gcs_path: None,
                        location: None,
                        validate_only: None,
                    }
                }
            }
            #[derive(Debug, Clone)]
            pub struct CreateRequestBuilder<'a, A> {
                pub(crate) reqwest: &'a ::reqwest::Client,
                pub(crate) auth: &'a ::std::sync::Mutex<A>,
                request: crate::schemas::CreateJobFromTemplateRequest,
                project_id: String,
                access_token: Option<String>,
                alt: Option<crate::params::Alt>,
                callback: Option<String>,
                fields: Option<String>,
                key: Option<String>,
                oauth_token: Option<String>,
                pretty_print: Option<bool>,
                quota_user: Option<String>,
                upload_protocol: Option<String>,
                upload_type: Option<String>,
                xgafv: Option<crate::params::Xgafv>,
            }
            impl<'a, A: yup_oauth2::GetToken> CreateRequestBuilder<'a, A> {
                #[doc = "OAuth access token."]
                pub fn access_token(mut self, value: impl Into<String>) -> Self {
                    self.access_token = Some(value.into());
                    self
                }
                #[doc = "JSONP"]
                pub fn callback(mut self, value: impl Into<String>) -> Self {
                    self.callback = Some(value.into());
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
                #[doc = "Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters."]
                pub fn quota_user(mut self, value: impl Into<String>) -> Self {
                    self.quota_user = Some(value.into());
                    self
                }
                #[doc = "Upload protocol for media (e.g. \"raw\", \"multipart\")."]
                pub fn upload_protocol(mut self, value: impl Into<String>) -> Self {
                    self.upload_protocol = Some(value.into());
                    self
                }
                #[doc = "Legacy upload protocol for media (e.g. \"media\", \"multipart\")."]
                pub fn upload_type(mut self, value: impl Into<String>) -> Self {
                    self.upload_type = Some(value.into());
                    self
                }
                #[doc = "V1 error format."]
                pub fn xgafv(mut self, value: crate::params::Xgafv) -> Self {
                    self.xgafv = Some(value);
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
                ) -> Result<crate::schemas::Job, Box<dyn ::std::error::Error>> {
                    self.execute_fields::<_, &str>(None)
                }
                #[doc = r" Execute the given operation. This will provide a `fields`"]
                #[doc = r" selector of `*`. This will include every attribute of the"]
                #[doc = r" response resource and should be limited to use during"]
                #[doc = r" development or debugging."]
                pub fn execute_debug(
                    self,
                ) -> Result<crate::schemas::Job, Box<dyn ::std::error::Error>> {
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
                    let mut output = "https://dataflow.googleapis.com/".to_owned();
                    output.push_str("v1b3/projects/");
                    {
                        let var_as_str = &self.project_id;
                        output.extend(::percent_encoding::utf8_percent_encode(
                            &var_as_str,
                            crate::SIMPLE,
                        ));
                    }
                    output.push_str("/templates");
                    output
                }
                fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
                    let req = self.reqwest.request(::reqwest::Method::POST, path);
                    let req = req.query(&[("access_token", &self.access_token)]);
                    let req = req.query(&[("alt", &self.alt)]);
                    let req = req.query(&[("callback", &self.callback)]);
                    let req = req.query(&[("fields", &self.fields)]);
                    let req = req.query(&[("key", &self.key)]);
                    let req = req.query(&[("oauth_token", &self.oauth_token)]);
                    let req = req.query(&[("prettyPrint", &self.pretty_print)]);
                    let req = req.query(&[("quotaUser", &self.quota_user)]);
                    let req = req.query(&[("upload_protocol", &self.upload_protocol)]);
                    let req = req.query(&[("uploadType", &self.upload_type)]);
                    let req = req.query(&[("$.xgafv", &self.xgafv)]);
                    let mut auth = self.auth.lock().unwrap();
                    let fut = auth.token(vec!["https://www.googleapis.com/auth/cloud-platform"]);
                    let mut runtime = ::tokio::runtime::Runtime::new().unwrap();
                    let token = runtime.block_on(fut).unwrap().access_token;
                    let req = req.bearer_auth(&token);
                    req
                }
            }
            #[derive(Debug, Clone)]
            pub struct GetRequestBuilder<'a, A> {
                pub(crate) reqwest: &'a ::reqwest::Client,
                pub(crate) auth: &'a ::std::sync::Mutex<A>,
                project_id: String,
                gcs_path: Option<String>,
                location: Option<String>,
                view: Option<crate::resources::projects::templates::params::GetView>,
                access_token: Option<String>,
                alt: Option<crate::params::Alt>,
                callback: Option<String>,
                fields: Option<String>,
                key: Option<String>,
                oauth_token: Option<String>,
                pretty_print: Option<bool>,
                quota_user: Option<String>,
                upload_protocol: Option<String>,
                upload_type: Option<String>,
                xgafv: Option<crate::params::Xgafv>,
            }
            impl<'a, A: yup_oauth2::GetToken> GetRequestBuilder<'a, A> {
                #[doc = "Required. A Cloud Storage path to the template from which to\ncreate the job.\nMust be valid Cloud Storage URL, beginning with 'gs://'."]
                pub fn gcs_path(mut self, value: impl Into<String>) -> Self {
                    self.gcs_path = Some(value.into());
                    self
                }
                #[doc = "The [regional endpoint]\n(https://cloud.google.com/dataflow/docs/concepts/regional-endpoints) to\nwhich to direct the request."]
                pub fn location(mut self, value: impl Into<String>) -> Self {
                    self.location = Some(value.into());
                    self
                }
                #[doc = "The view to retrieve. Defaults to METADATA_ONLY."]
                pub fn view(
                    mut self,
                    value: crate::resources::projects::templates::params::GetView,
                ) -> Self {
                    self.view = Some(value);
                    self
                }
                #[doc = "OAuth access token."]
                pub fn access_token(mut self, value: impl Into<String>) -> Self {
                    self.access_token = Some(value.into());
                    self
                }
                #[doc = "JSONP"]
                pub fn callback(mut self, value: impl Into<String>) -> Self {
                    self.callback = Some(value.into());
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
                #[doc = "Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters."]
                pub fn quota_user(mut self, value: impl Into<String>) -> Self {
                    self.quota_user = Some(value.into());
                    self
                }
                #[doc = "Upload protocol for media (e.g. \"raw\", \"multipart\")."]
                pub fn upload_protocol(mut self, value: impl Into<String>) -> Self {
                    self.upload_protocol = Some(value.into());
                    self
                }
                #[doc = "Legacy upload protocol for media (e.g. \"media\", \"multipart\")."]
                pub fn upload_type(mut self, value: impl Into<String>) -> Self {
                    self.upload_type = Some(value.into());
                    self
                }
                #[doc = "V1 error format."]
                pub fn xgafv(mut self, value: crate::params::Xgafv) -> Self {
                    self.xgafv = Some(value);
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
                ) -> Result<crate::schemas::GetTemplateResponse, Box<dyn ::std::error::Error>>
                {
                    self.execute_fields::<_, &str>(None)
                }
                #[doc = r" Execute the given operation. This will provide a `fields`"]
                #[doc = r" selector of `*`. This will include every attribute of the"]
                #[doc = r" response resource and should be limited to use during"]
                #[doc = r" development or debugging."]
                pub fn execute_debug(
                    self,
                ) -> Result<crate::schemas::GetTemplateResponse, Box<dyn ::std::error::Error>>
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
                    let mut output = "https://dataflow.googleapis.com/".to_owned();
                    output.push_str("v1b3/projects/");
                    {
                        let var_as_str = &self.project_id;
                        output.extend(::percent_encoding::utf8_percent_encode(
                            &var_as_str,
                            crate::SIMPLE,
                        ));
                    }
                    output.push_str("/templates:get");
                    output
                }
                fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
                    let req = self.reqwest.request(::reqwest::Method::GET, path);
                    let req = req.query(&[("gcsPath", &self.gcs_path)]);
                    let req = req.query(&[("location", &self.location)]);
                    let req = req.query(&[("view", &self.view)]);
                    let req = req.query(&[("access_token", &self.access_token)]);
                    let req = req.query(&[("alt", &self.alt)]);
                    let req = req.query(&[("callback", &self.callback)]);
                    let req = req.query(&[("fields", &self.fields)]);
                    let req = req.query(&[("key", &self.key)]);
                    let req = req.query(&[("oauth_token", &self.oauth_token)]);
                    let req = req.query(&[("prettyPrint", &self.pretty_print)]);
                    let req = req.query(&[("quotaUser", &self.quota_user)]);
                    let req = req.query(&[("upload_protocol", &self.upload_protocol)]);
                    let req = req.query(&[("uploadType", &self.upload_type)]);
                    let req = req.query(&[("$.xgafv", &self.xgafv)]);
                    let mut auth = self.auth.lock().unwrap();
                    let fut = auth.token(vec!["https://www.googleapis.com/auth/compute.readonly"]);
                    let mut runtime = ::tokio::runtime::Runtime::new().unwrap();
                    let token = runtime.block_on(fut).unwrap().access_token;
                    let req = req.bearer_auth(&token);
                    req
                }
            }
            #[derive(Debug, Clone)]
            pub struct LaunchRequestBuilder<'a, A> {
                pub(crate) reqwest: &'a ::reqwest::Client,
                pub(crate) auth: &'a ::std::sync::Mutex<A>,
                request: crate::schemas::LaunchTemplateParameters,
                project_id: String,
                dynamic_template_gcs_path: Option<String>,
                dynamic_template_staging_location: Option<String>,
                gcs_path: Option<String>,
                location: Option<String>,
                validate_only: Option<bool>,
                access_token: Option<String>,
                alt: Option<crate::params::Alt>,
                callback: Option<String>,
                fields: Option<String>,
                key: Option<String>,
                oauth_token: Option<String>,
                pretty_print: Option<bool>,
                quota_user: Option<String>,
                upload_protocol: Option<String>,
                upload_type: Option<String>,
                xgafv: Option<crate::params::Xgafv>,
            }
            impl<'a, A: yup_oauth2::GetToken> LaunchRequestBuilder<'a, A> {
                #[doc = "Path to dynamic template spec file on GCS.\nThe file must be a Json serialized DynamicTemplateFieSpec object."]
                pub fn dynamic_template_gcs_path(mut self, value: impl Into<String>) -> Self {
                    self.dynamic_template_gcs_path = Some(value.into());
                    self
                }
                #[doc = "Cloud Storage path for staging dependencies.\nMust be a valid Cloud Storage URL, beginning with `gs://`."]
                pub fn dynamic_template_staging_location(
                    mut self,
                    value: impl Into<String>,
                ) -> Self {
                    self.dynamic_template_staging_location = Some(value.into());
                    self
                }
                #[doc = "A Cloud Storage path to the template from which to create\nthe job.\nMust be valid Cloud Storage URL, beginning with 'gs://'."]
                pub fn gcs_path(mut self, value: impl Into<String>) -> Self {
                    self.gcs_path = Some(value.into());
                    self
                }
                #[doc = "The [regional endpoint]\n(https://cloud.google.com/dataflow/docs/concepts/regional-endpoints) to\nwhich to direct the request."]
                pub fn location(mut self, value: impl Into<String>) -> Self {
                    self.location = Some(value.into());
                    self
                }
                #[doc = "If true, the request is validated but not actually executed.\nDefaults to false."]
                pub fn validate_only(mut self, value: bool) -> Self {
                    self.validate_only = Some(value);
                    self
                }
                #[doc = "OAuth access token."]
                pub fn access_token(mut self, value: impl Into<String>) -> Self {
                    self.access_token = Some(value.into());
                    self
                }
                #[doc = "JSONP"]
                pub fn callback(mut self, value: impl Into<String>) -> Self {
                    self.callback = Some(value.into());
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
                #[doc = "Available to use for quota purposes for server-side applications. Can be any arbitrary string assigned to a user, but should not exceed 40 characters."]
                pub fn quota_user(mut self, value: impl Into<String>) -> Self {
                    self.quota_user = Some(value.into());
                    self
                }
                #[doc = "Upload protocol for media (e.g. \"raw\", \"multipart\")."]
                pub fn upload_protocol(mut self, value: impl Into<String>) -> Self {
                    self.upload_protocol = Some(value.into());
                    self
                }
                #[doc = "Legacy upload protocol for media (e.g. \"media\", \"multipart\")."]
                pub fn upload_type(mut self, value: impl Into<String>) -> Self {
                    self.upload_type = Some(value.into());
                    self
                }
                #[doc = "V1 error format."]
                pub fn xgafv(mut self, value: crate::params::Xgafv) -> Self {
                    self.xgafv = Some(value);
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
                ) -> Result<crate::schemas::LaunchTemplateResponse, Box<dyn ::std::error::Error>>
                {
                    self.execute_fields::<_, &str>(None)
                }
                #[doc = r" Execute the given operation. This will provide a `fields`"]
                #[doc = r" selector of `*`. This will include every attribute of the"]
                #[doc = r" response resource and should be limited to use during"]
                #[doc = r" development or debugging."]
                pub fn execute_debug(
                    self,
                ) -> Result<crate::schemas::LaunchTemplateResponse, Box<dyn ::std::error::Error>>
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
                    let mut output = "https://dataflow.googleapis.com/".to_owned();
                    output.push_str("v1b3/projects/");
                    {
                        let var_as_str = &self.project_id;
                        output.extend(::percent_encoding::utf8_percent_encode(
                            &var_as_str,
                            crate::SIMPLE,
                        ));
                    }
                    output.push_str("/templates:launch");
                    output
                }
                fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
                    let req = self.reqwest.request(::reqwest::Method::POST, path);
                    let req =
                        req.query(&[("dynamicTemplate.gcsPath", &self.dynamic_template_gcs_path)]);
                    let req = req.query(&[(
                        "dynamicTemplate.stagingLocation",
                        &self.dynamic_template_staging_location,
                    )]);
                    let req = req.query(&[("gcsPath", &self.gcs_path)]);
                    let req = req.query(&[("location", &self.location)]);
                    let req = req.query(&[("validateOnly", &self.validate_only)]);
                    let req = req.query(&[("access_token", &self.access_token)]);
                    let req = req.query(&[("alt", &self.alt)]);
                    let req = req.query(&[("callback", &self.callback)]);
                    let req = req.query(&[("fields", &self.fields)]);
                    let req = req.query(&[("key", &self.key)]);
                    let req = req.query(&[("oauth_token", &self.oauth_token)]);
                    let req = req.query(&[("prettyPrint", &self.pretty_print)]);
                    let req = req.query(&[("quotaUser", &self.quota_user)]);
                    let req = req.query(&[("upload_protocol", &self.upload_protocol)]);
                    let req = req.query(&[("uploadType", &self.upload_type)]);
                    let req = req.query(&[("$.xgafv", &self.xgafv)]);
                    let mut auth = self.auth.lock().unwrap();
                    let fut = auth.token(vec!["https://www.googleapis.com/auth/cloud-platform"]);
                    let mut runtime = ::tokio::runtime::Runtime::new().unwrap();
                    let token = runtime.block_on(fut).unwrap().access_token;
                    let req = req.bearer_auth(&token);
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
