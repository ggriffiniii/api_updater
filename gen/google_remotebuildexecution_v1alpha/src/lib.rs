pub mod schemas {
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
    pub struct BuildBazelRemoteExecutionV2Action {
        #[doc = "The digest of the Command\nto run, which MUST be present in the\nContentAddressableStorage."]
        #[serde(rename = "commandDigest", default)]
        pub command_digest:
            ::std::option::Option<crate::schemas::BuildBazelRemoteExecutionV2Digest>,
        #[doc = "If true, then the `Action`'s result cannot be cached, and in-flight\nrequests for the same `Action` may not be merged."]
        #[serde(rename = "doNotCache", default)]
        pub do_not_cache: ::std::option::Option<bool>,
        #[doc = "The digest of the root\nDirectory for the input\nfiles. The files in the directory tree are available in the correct\nlocation on the build machine before the command is executed. The root\ndirectory, as well as every subdirectory and content blob referred to, MUST\nbe in the\nContentAddressableStorage."]
        #[serde(rename = "inputRootDigest", default)]
        pub input_root_digest:
            ::std::option::Option<crate::schemas::BuildBazelRemoteExecutionV2Digest>,
        #[doc = "A timeout after which the execution should be killed. If the timeout is\nabsent, then the client is specifying that the execution should continue\nas long as the server will let it. The server SHOULD impose a timeout if\nthe client does not specify one, however, if the client does specify a\ntimeout that is longer than the server's maximum timeout, the server MUST\nreject the request.\n\nThe timeout is a part of the\nAction message, and\ntherefore two `Actions` with different timeouts are different, even if they\nare otherwise identical. This is because, if they were not, running an\n`Action` with a lower timeout than is required might result in a cache hit\nfrom an execution run with a longer timeout, hiding the fact that the\ntimeout is too short. By encoding it directly in the `Action`, a lower\ntimeout will result in a cache miss and the execution timeout will fail\nimmediately, rather than whenever the cache entry gets evicted."]
        #[serde(rename = "timeout", default)]
        pub timeout: ::std::option::Option<String>,
    }
    impl ::field_selector::FieldSelector for BuildBazelRemoteExecutionV2Action {
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
    pub struct BuildBazelRemoteExecutionV2ActionResult {
        #[doc = "The details of the execution that originally produced this result."]
        #[serde(rename = "executionMetadata", default)]
        pub execution_metadata: ::std::option::Option<
            crate::schemas::BuildBazelRemoteExecutionV2ExecutedActionMetadata,
        >,
        #[doc = "The exit code of the command."]
        #[serde(rename = "exitCode", default)]
        pub exit_code: ::std::option::Option<i32>,
        #[doc = "The output directories of the action. For each output directory requested\nin the `output_directories` field of the Action, if the corresponding\ndirectory existed after the action completed, a single entry will be\npresent in the output list, which will contain the digest of a\nTree message containing the\ndirectory tree, and the path equal exactly to the corresponding Action\noutput_directories member.\n\nAs an example, suppose the Action had an output directory `a/b/dir` and the\nexecution produced the following contents in `a/b/dir`: a file named `bar`\nand a directory named `foo` with an executable file named `baz`. Then,\noutput_directory will contain (hashes shortened for readability):\n\n````textjson\n// OutputDirectory proto:\n{\n  path: \"a/b/dir\"\n  tree_digest: {\n    hash: \"4a73bc9d03...\",\n    size: 55\n  }\n}\n// Tree proto with hash \"4a73bc9d03...\" and size 55:\n{\n  root: {\n    files: [\n      {\n        name: \"bar\",\n        digest: {\n          hash: \"4a73bc9d03...\",\n          size: 65534\n        }\n      }\n    ],\n    directories: [\n      {\n        name: \"foo\",\n        digest: {\n          hash: \"4cf2eda940...\",\n          size: 43\n        }\n      }\n    ]\n  }\n  children : {\n    // (Directory proto with hash \"4cf2eda940...\" and size 43)\n    files: [\n      {\n        name: \"baz\",\n        digest: {\n          hash: \"b2c941073e...\",\n          size: 1294,\n        },\n        is_executable: true\n      }\n    ]\n  }\n}\n````\n\nIf an output of the same name was found, but was not a directory, the\nserver will return a FAILED_PRECONDITION."]
        #[serde(rename = "outputDirectories", default)]
        pub output_directories:
            ::std::option::Option<Vec<crate::schemas::BuildBazelRemoteExecutionV2OutputDirectory>>,
        #[doc = "The output directories of the action that are symbolic links to other\ndirectories. Those may be links to other output directories, or input\ndirectories, or even absolute paths outside of the working directory,\nif the server supports\nSymlinkAbsolutePathStrategy.ALLOWED.\nFor each output directory requested in the `output_directories` field of\nthe Action, if the directory existed after the action completed, a\nsingle entry will be present either in this field, or in the\n`output_directories` field, if the directory was not a symbolic link.\n\nIf an output of the same name was found, but was a symbolic link to a file\ninstead of a directory, the server will return a FAILED_PRECONDITION.\nIf the action does not produce the requested output, then that output\nwill be omitted from the list. The server is free to arrange the output\nlist as desired; clients MUST NOT assume that the output list is sorted."]
        #[serde(rename = "outputDirectorySymlinks", default)]
        pub output_directory_symlinks:
            ::std::option::Option<Vec<crate::schemas::BuildBazelRemoteExecutionV2OutputSymlink>>,
        #[doc = "The output files of the action that are symbolic links to other files. Those\nmay be links to other output files, or input files, or even absolute paths\noutside of the working directory, if the server supports\nSymlinkAbsolutePathStrategy.ALLOWED.\nFor each output file requested in the `output_files` field of the Action,\nif the corresponding file existed after\nthe action completed, a single entry will be present either in this field,\nor in the `output_files` field, if the file was not a symbolic link.\n\nIf an output symbolic link of the same name was found, but its target\ntype was not a regular file, the server will return a FAILED_PRECONDITION.\nIf the action does not produce the requested output, then that output\nwill be omitted from the list. The server is free to arrange the output\nlist as desired; clients MUST NOT assume that the output list is sorted."]
        #[serde(rename = "outputFileSymlinks", default)]
        pub output_file_symlinks:
            ::std::option::Option<Vec<crate::schemas::BuildBazelRemoteExecutionV2OutputSymlink>>,
        #[doc = "The output files of the action. For each output file requested in the\n`output_files` field of the Action, if the corresponding file existed after\nthe action completed, a single entry will be present either in this field,\nor the `output_file_symlinks` field if the file was a symbolic link to\nanother file.\n\nIf an output of the same name was found, but was a directory rather\nthan a regular file, the server will return a FAILED_PRECONDITION.\nIf the action does not produce the requested output, then that output\nwill be omitted from the list. The server is free to arrange the output\nlist as desired; clients MUST NOT assume that the output list is sorted."]
        #[serde(rename = "outputFiles", default)]
        pub output_files:
            ::std::option::Option<Vec<crate::schemas::BuildBazelRemoteExecutionV2OutputFile>>,
        #[doc = "The digest for a blob containing the standard error of the action, which\ncan be retrieved from the\nContentAddressableStorage."]
        #[serde(rename = "stderrDigest", default)]
        pub stderr_digest: ::std::option::Option<crate::schemas::BuildBazelRemoteExecutionV2Digest>,
        #[doc = "The standard error buffer of the action. The server SHOULD NOT inline\nstderr unless requested by the client in the\nGetActionResultRequest\nmessage. The server MAY omit inlining, even if requested, and MUST do so if inlining\nwould cause the response to exceed message size limits."]
        #[serde(rename = "stderrRaw", default)]
        pub stderr_raw: ::std::option::Option<crate::bytes::Bytes>,
        #[doc = "The digest for a blob containing the standard output of the action, which\ncan be retrieved from the\nContentAddressableStorage."]
        #[serde(rename = "stdoutDigest", default)]
        pub stdout_digest: ::std::option::Option<crate::schemas::BuildBazelRemoteExecutionV2Digest>,
        #[doc = "The standard output buffer of the action. The server SHOULD NOT inline\nstdout unless requested by the client in the\nGetActionResultRequest\nmessage. The server MAY omit inlining, even if requested, and MUST do so if inlining\nwould cause the response to exceed message size limits."]
        #[serde(rename = "stdoutRaw", default)]
        pub stdout_raw: ::std::option::Option<crate::bytes::Bytes>,
    }
    impl ::field_selector::FieldSelector for BuildBazelRemoteExecutionV2ActionResult {
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
    pub struct BuildBazelRemoteExecutionV2Command {
        #[doc = "The arguments to the command. The first argument must be the path to the\nexecutable, which must be either a relative path, in which case it is\nevaluated with respect to the input root, or an absolute path."]
        #[serde(rename = "arguments", default)]
        pub arguments: ::std::option::Option<Vec<String>>,
        #[doc = "The environment variables to set when running the program. The worker may\nprovide its own default environment variables; these defaults can be\noverridden using this field. Additional variables can also be specified.\n\nIn order to ensure that equivalent\nCommands always hash to the same\nvalue, the environment variables MUST be lexicographically sorted by name.\nSorting of strings is done by code point, equivalently, by the UTF-8 bytes."]
        #[serde(rename = "environmentVariables", default)]
        pub environment_variables: ::std::option::Option<
            Vec<crate::schemas::BuildBazelRemoteExecutionV2CommandEnvironmentVariable>,
        >,
        #[doc = "A list of the output directories that the client expects to retrieve from\nthe action. Only the listed directories will be returned (an entire\ndirectory structure will be returned as a\nTree message digest, see\nOutputDirectory), as\nwell as files listed in `output_files`. Other files or directories that\nmay be created during command execution are discarded.\n\nThe paths are relative to the working directory of the action execution.\nThe paths are specified using a single forward slash (`/`) as a path\nseparator, even if the execution platform natively uses a different\nseparator. The path MUST NOT include a trailing slash, nor a leading slash,\nbeing a relative path. The special value of empty string is allowed,\nalthough not recommended, and can be used to capture the entire working\ndirectory tree, including inputs.\n\nIn order to ensure consistent hashing of the same Action, the output paths\nMUST be sorted lexicographically by code point (or, equivalently, by UTF-8\nbytes).\n\nAn output directory cannot be duplicated or have the same path as any of\nthe listed output files. An output directory is allowed to be a parent of\nanother output directory.\n\nDirectories leading up to the output directories (but not the output\ndirectories themselves) are created by the worker prior to execution, even\nif they are not explicitly part of the input root."]
        #[serde(rename = "outputDirectories", default)]
        pub output_directories: ::std::option::Option<Vec<String>>,
        #[doc = "A list of the output files that the client expects to retrieve from the\naction. Only the listed files, as well as directories listed in\n`output_directories`, will be returned to the client as output.\nOther files or directories that may be created during command execution\nare discarded.\n\nThe paths are relative to the working directory of the action execution.\nThe paths are specified using a single forward slash (`/`) as a path\nseparator, even if the execution platform natively uses a different\nseparator. The path MUST NOT include a trailing slash, nor a leading slash,\nbeing a relative path.\n\nIn order to ensure consistent hashing of the same Action, the output paths\nMUST be sorted lexicographically by code point (or, equivalently, by UTF-8\nbytes).\n\nAn output file cannot be duplicated, be a parent of another output file, or\nhave the same path as any of the listed output directories.\n\nDirectories leading up to the output files are created by the worker prior\nto execution, even if they are not explicitly part of the input root."]
        #[serde(rename = "outputFiles", default)]
        pub output_files: ::std::option::Option<Vec<String>>,
        #[doc = "The platform requirements for the execution environment. The server MAY\nchoose to execute the action on any worker satisfying the requirements, so\nthe client SHOULD ensure that running the action on any such worker will\nhave the same result.\nA detailed lexicon for this can be found in the accompanying platform.md."]
        #[serde(rename = "platform", default)]
        pub platform: ::std::option::Option<crate::schemas::BuildBazelRemoteExecutionV2Platform>,
        #[doc = "The working directory, relative to the input root, for the command to run\nin. It must be a directory which exists in the input tree. If it is left\nempty, then the action is run in the input root."]
        #[serde(rename = "workingDirectory", default)]
        pub working_directory: ::std::option::Option<String>,
    }
    impl ::field_selector::FieldSelector for BuildBazelRemoteExecutionV2Command {
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
    pub struct BuildBazelRemoteExecutionV2CommandEnvironmentVariable {
        #[doc = "The variable name."]
        #[serde(rename = "name", default)]
        pub name: ::std::option::Option<String>,
        #[doc = "The variable value."]
        #[serde(rename = "value", default)]
        pub value: ::std::option::Option<String>,
    }
    impl ::field_selector::FieldSelector for BuildBazelRemoteExecutionV2CommandEnvironmentVariable {
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
    pub struct BuildBazelRemoteExecutionV2Digest {
        #[doc = "The hash. In the case of SHA-256, it will always be a lowercase hex string\nexactly 64 characters long."]
        #[serde(rename = "hash", default)]
        pub hash: ::std::option::Option<String>,
        #[doc = "The size of the blob, in bytes."]
        #[serde(rename = "sizeBytes", default)]
        #[serde(with = "crate::parsed_string")]
        pub size_bytes: ::std::option::Option<i64>,
    }
    impl ::field_selector::FieldSelector for BuildBazelRemoteExecutionV2Digest {
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
    pub struct BuildBazelRemoteExecutionV2Directory {
        #[doc = "The subdirectories in the directory."]
        #[serde(rename = "directories", default)]
        pub directories:
            ::std::option::Option<Vec<crate::schemas::BuildBazelRemoteExecutionV2DirectoryNode>>,
        #[doc = "The files in the directory."]
        #[serde(rename = "files", default)]
        pub files: ::std::option::Option<Vec<crate::schemas::BuildBazelRemoteExecutionV2FileNode>>,
        #[doc = "The symlinks in the directory."]
        #[serde(rename = "symlinks", default)]
        pub symlinks:
            ::std::option::Option<Vec<crate::schemas::BuildBazelRemoteExecutionV2SymlinkNode>>,
    }
    impl ::field_selector::FieldSelector for BuildBazelRemoteExecutionV2Directory {
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
    pub struct BuildBazelRemoteExecutionV2DirectoryNode {
        #[doc = "The digest of the\nDirectory object\nrepresented. See Digest\nfor information about how to take the digest of a proto message."]
        #[serde(rename = "digest", default)]
        pub digest: ::std::option::Option<crate::schemas::BuildBazelRemoteExecutionV2Digest>,
        #[doc = "The name of the directory."]
        #[serde(rename = "name", default)]
        pub name: ::std::option::Option<String>,
    }
    impl ::field_selector::FieldSelector for BuildBazelRemoteExecutionV2DirectoryNode {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum BuildBazelRemoteExecutionV2ExecuteOperationMetadataStage {
        #[doc = "Checking the result against the cache."]
        CacheCheck,
        #[doc = "Finished execution."]
        Completed,
        #[doc = "Currently being executed by a worker."]
        Executing,
        #[doc = "Currently idle, awaiting a free machine to execute."]
        Queued,
        #[doc = "Invalid value."]
        Unknown,
    }
    impl BuildBazelRemoteExecutionV2ExecuteOperationMetadataStage {
        pub fn as_str(self) -> &'static str {
            match self {
                BuildBazelRemoteExecutionV2ExecuteOperationMetadataStage::CacheCheck => {
                    "CACHE_CHECK"
                }
                BuildBazelRemoteExecutionV2ExecuteOperationMetadataStage::Completed => "COMPLETED",
                BuildBazelRemoteExecutionV2ExecuteOperationMetadataStage::Executing => "EXECUTING",
                BuildBazelRemoteExecutionV2ExecuteOperationMetadataStage::Queued => "QUEUED",
                BuildBazelRemoteExecutionV2ExecuteOperationMetadataStage::Unknown => "UNKNOWN",
            }
        }
    }
    impl ::std::fmt::Display for BuildBazelRemoteExecutionV2ExecuteOperationMetadataStage {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for BuildBazelRemoteExecutionV2ExecuteOperationMetadataStage {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for BuildBazelRemoteExecutionV2ExecuteOperationMetadataStage {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "CACHE_CHECK" => {
                    BuildBazelRemoteExecutionV2ExecuteOperationMetadataStage::CacheCheck
                }
                "COMPLETED" => BuildBazelRemoteExecutionV2ExecuteOperationMetadataStage::Completed,
                "EXECUTING" => BuildBazelRemoteExecutionV2ExecuteOperationMetadataStage::Executing,
                "QUEUED" => BuildBazelRemoteExecutionV2ExecuteOperationMetadataStage::Queued,
                "UNKNOWN" => BuildBazelRemoteExecutionV2ExecuteOperationMetadataStage::Unknown,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::field_selector::FieldSelector for BuildBazelRemoteExecutionV2ExecuteOperationMetadataStage {
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
    pub struct BuildBazelRemoteExecutionV2ExecuteOperationMetadata {
        #[doc = "The digest of the Action\nbeing executed."]
        #[serde(rename = "actionDigest", default)]
        pub action_digest: ::std::option::Option<crate::schemas::BuildBazelRemoteExecutionV2Digest>,
        #[doc = "The current stage of execution."]
        #[serde(rename = "stage", default)]
        pub stage: ::std::option::Option<
            crate::schemas::BuildBazelRemoteExecutionV2ExecuteOperationMetadataStage,
        >,
        #[doc = "If set, the client can use this name with\nByteStream.Read to stream the\nstandard error."]
        #[serde(rename = "stderrStreamName", default)]
        pub stderr_stream_name: ::std::option::Option<String>,
        #[doc = "If set, the client can use this name with\nByteStream.Read to stream the\nstandard output."]
        #[serde(rename = "stdoutStreamName", default)]
        pub stdout_stream_name: ::std::option::Option<String>,
    }
    impl ::field_selector::FieldSelector for BuildBazelRemoteExecutionV2ExecuteOperationMetadata {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct BuildBazelRemoteExecutionV2ExecuteResponse {
        #[doc = "True if the result was served from cache, false if it was executed."]
        #[serde(rename = "cachedResult", default)]
        pub cached_result: ::std::option::Option<bool>,
        #[doc = "Freeform informational message with details on the execution of the action\nthat may be displayed to the user upon failure or when requested explicitly."]
        #[serde(rename = "message", default)]
        pub message: ::std::option::Option<String>,
        #[doc = "The result of the action."]
        #[serde(rename = "result", default)]
        pub result: ::std::option::Option<crate::schemas::BuildBazelRemoteExecutionV2ActionResult>,
        #[doc = "An optional list of additional log outputs the server wishes to provide. A\nserver can use this to return execution-specific logs however it wishes.\nThis is intended primarily to make it easier for users to debug issues that\nmay be outside of the actual job execution, such as by identifying the\nworker executing the action or by providing logs from the worker's setup\nphase. The keys SHOULD be human readable so that a client can display them\nto a user."]
        #[serde(rename = "serverLogs", default)]
        pub server_logs: ::std::option::Option<
            ::std::collections::BTreeMap<
                String,
                crate::schemas::BuildBazelRemoteExecutionV2LogFile,
            >,
        >,
        #[doc = "If the status has a code other than `OK`, it indicates that the action did\nnot finish execution. For example, if the operation times out during\nexecution, the status will have a `DEADLINE_EXCEEDED` code. Servers MUST\nuse this field for errors in execution, rather than the error field on the\n`Operation` object.\n\nIf the status code is other than `OK`, then the result MUST NOT be cached.\nFor an error status, the `result` field is optional; the server may\npopulate the output-, stdout-, and stderr-related fields if it has any\ninformation available, such as the stdout and stderr of a timed-out action."]
        #[serde(rename = "status", default)]
        pub status: ::std::option::Option<crate::schemas::GoogleRpcStatus>,
    }
    impl ::field_selector::FieldSelector for BuildBazelRemoteExecutionV2ExecuteResponse {
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
    pub struct BuildBazelRemoteExecutionV2ExecutedActionMetadata {
        #[doc = "When the worker completed executing the action command."]
        #[serde(rename = "executionCompletedTimestamp", default)]
        pub execution_completed_timestamp: ::std::option::Option<String>,
        #[doc = "When the worker started executing the action command."]
        #[serde(rename = "executionStartTimestamp", default)]
        pub execution_start_timestamp: ::std::option::Option<String>,
        #[doc = "When the worker finished fetching action inputs."]
        #[serde(rename = "inputFetchCompletedTimestamp", default)]
        pub input_fetch_completed_timestamp: ::std::option::Option<String>,
        #[doc = "When the worker started fetching action inputs."]
        #[serde(rename = "inputFetchStartTimestamp", default)]
        pub input_fetch_start_timestamp: ::std::option::Option<String>,
        #[doc = "When the worker finished uploading action outputs."]
        #[serde(rename = "outputUploadCompletedTimestamp", default)]
        pub output_upload_completed_timestamp: ::std::option::Option<String>,
        #[doc = "When the worker started uploading action outputs."]
        #[serde(rename = "outputUploadStartTimestamp", default)]
        pub output_upload_start_timestamp: ::std::option::Option<String>,
        #[doc = "When was the action added to the queue."]
        #[serde(rename = "queuedTimestamp", default)]
        pub queued_timestamp: ::std::option::Option<String>,
        #[doc = "The name of the worker which ran the execution."]
        #[serde(rename = "worker", default)]
        pub worker: ::std::option::Option<String>,
        #[doc = "When the worker completed the action, including all stages."]
        #[serde(rename = "workerCompletedTimestamp", default)]
        pub worker_completed_timestamp: ::std::option::Option<String>,
        #[doc = "When the worker received the action."]
        #[serde(rename = "workerStartTimestamp", default)]
        pub worker_start_timestamp: ::std::option::Option<String>,
    }
    impl ::field_selector::FieldSelector for BuildBazelRemoteExecutionV2ExecutedActionMetadata {
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
    pub struct BuildBazelRemoteExecutionV2FileNode {
        #[doc = "The digest of the file's content."]
        #[serde(rename = "digest", default)]
        pub digest: ::std::option::Option<crate::schemas::BuildBazelRemoteExecutionV2Digest>,
        #[doc = "True if file is executable, false otherwise."]
        #[serde(rename = "isExecutable", default)]
        pub is_executable: ::std::option::Option<bool>,
        #[doc = "The name of the file."]
        #[serde(rename = "name", default)]
        pub name: ::std::option::Option<String>,
    }
    impl ::field_selector::FieldSelector for BuildBazelRemoteExecutionV2FileNode {
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
    pub struct BuildBazelRemoteExecutionV2LogFile {
        #[doc = "The digest of the log contents."]
        #[serde(rename = "digest", default)]
        pub digest: ::std::option::Option<crate::schemas::BuildBazelRemoteExecutionV2Digest>,
        #[doc = "This is a hint as to the purpose of the log, and is set to true if the log\nis human-readable text that can be usefully displayed to a user, and false\notherwise. For instance, if a command-line client wishes to print the\nserver logs to the terminal for a failed action, this allows it to avoid\ndisplaying a binary file."]
        #[serde(rename = "humanReadable", default)]
        pub human_readable: ::std::option::Option<bool>,
    }
    impl ::field_selector::FieldSelector for BuildBazelRemoteExecutionV2LogFile {
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
    pub struct BuildBazelRemoteExecutionV2OutputDirectory {
        #[doc = "The full path of the directory relative to the working directory. The path\nseparator is a forward slash `/`. Since this is a relative path, it MUST\nNOT begin with a leading forward slash. The empty string value is allowed,\nand it denotes the entire working directory."]
        #[serde(rename = "path", default)]
        pub path: ::std::option::Option<String>,
        #[doc = "The digest of the encoded\nTree proto containing the\ndirectory's contents."]
        #[serde(rename = "treeDigest", default)]
        pub tree_digest: ::std::option::Option<crate::schemas::BuildBazelRemoteExecutionV2Digest>,
    }
    impl ::field_selector::FieldSelector for BuildBazelRemoteExecutionV2OutputDirectory {
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
    pub struct BuildBazelRemoteExecutionV2OutputFile {
        #[doc = "The contents of the file if inlining was requested. The server SHOULD NOT inline\nfile contents unless requested by the client in the\nGetActionResultRequest\nmessage. The server MAY omit inlining, even if requested, and MUST do so if inlining\nwould cause the response to exceed message size limits."]
        #[serde(rename = "contents", default)]
        pub contents: ::std::option::Option<crate::bytes::Bytes>,
        #[doc = "The digest of the file's content."]
        #[serde(rename = "digest", default)]
        pub digest: ::std::option::Option<crate::schemas::BuildBazelRemoteExecutionV2Digest>,
        #[doc = "True if file is executable, false otherwise."]
        #[serde(rename = "isExecutable", default)]
        pub is_executable: ::std::option::Option<bool>,
        #[doc = "The full path of the file relative to the working directory, including the\nfilename. The path separator is a forward slash `/`. Since this is a\nrelative path, it MUST NOT begin with a leading forward slash."]
        #[serde(rename = "path", default)]
        pub path: ::std::option::Option<String>,
    }
    impl ::field_selector::FieldSelector for BuildBazelRemoteExecutionV2OutputFile {
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
    pub struct BuildBazelRemoteExecutionV2OutputSymlink {
        #[doc = "The full path of the symlink relative to the working directory, including the\nfilename. The path separator is a forward slash `/`. Since this is a\nrelative path, it MUST NOT begin with a leading forward slash."]
        #[serde(rename = "path", default)]
        pub path: ::std::option::Option<String>,
        #[doc = "The target path of the symlink. The path separator is a forward slash `/`.\nThe target path can be relative to the parent directory of the symlink or\nit can be an absolute path starting with `/`. Support for absolute paths\ncan be checked using the Capabilities\nAPI. The canonical form forbids the substrings `/./` and `//` in the target\npath. `..` components are allowed anywhere in the target path."]
        #[serde(rename = "target", default)]
        pub target: ::std::option::Option<String>,
    }
    impl ::field_selector::FieldSelector for BuildBazelRemoteExecutionV2OutputSymlink {
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
    pub struct BuildBazelRemoteExecutionV2Platform {
        #[doc = "The properties that make up this platform. In order to ensure that\nequivalent `Platform`s always hash to the same value, the properties MUST\nbe lexicographically sorted by name, and then by value. Sorting of strings\nis done by code point, equivalently, by the UTF-8 bytes."]
        #[serde(rename = "properties", default)]
        pub properties:
            ::std::option::Option<Vec<crate::schemas::BuildBazelRemoteExecutionV2PlatformProperty>>,
    }
    impl ::field_selector::FieldSelector for BuildBazelRemoteExecutionV2Platform {
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
    pub struct BuildBazelRemoteExecutionV2PlatformProperty {
        #[doc = "The property name."]
        #[serde(rename = "name", default)]
        pub name: ::std::option::Option<String>,
        #[doc = "The property value."]
        #[serde(rename = "value", default)]
        pub value: ::std::option::Option<String>,
    }
    impl ::field_selector::FieldSelector for BuildBazelRemoteExecutionV2PlatformProperty {
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
    pub struct BuildBazelRemoteExecutionV2RequestMetadata {
        #[doc = "An identifier that ties multiple requests to the same action.\nFor example, multiple requests to the CAS, Action Cache, and Execution\nAPI are used in order to compile foo.cc."]
        #[serde(rename = "actionId", default)]
        pub action_id: ::std::option::Option<String>,
        #[doc = "An identifier to tie multiple tool invocations together. For example,\nruns of foo_test, bar_test and baz_test on a post-submit of a given patch."]
        #[serde(rename = "correlatedInvocationsId", default)]
        pub correlated_invocations_id: ::std::option::Option<String>,
        #[doc = "The details for the tool invoking the requests."]
        #[serde(rename = "toolDetails", default)]
        pub tool_details:
            ::std::option::Option<crate::schemas::BuildBazelRemoteExecutionV2ToolDetails>,
        #[doc = "An identifier that ties multiple actions together to a final result.\nFor example, multiple actions are required to build and run foo_test."]
        #[serde(rename = "toolInvocationId", default)]
        pub tool_invocation_id: ::std::option::Option<String>,
    }
    impl ::field_selector::FieldSelector for BuildBazelRemoteExecutionV2RequestMetadata {
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
    pub struct BuildBazelRemoteExecutionV2SymlinkNode {
        #[doc = "The name of the symlink."]
        #[serde(rename = "name", default)]
        pub name: ::std::option::Option<String>,
        #[doc = "The target path of the symlink. The path separator is a forward slash `/`.\nThe target path can be relative to the parent directory of the symlink or\nit can be an absolute path starting with `/`. Support for absolute paths\ncan be checked using the Capabilities\nAPI. The canonical form forbids the substrings `/./` and `//` in the target\npath. `..` components are allowed anywhere in the target path."]
        #[serde(rename = "target", default)]
        pub target: ::std::option::Option<String>,
    }
    impl ::field_selector::FieldSelector for BuildBazelRemoteExecutionV2SymlinkNode {
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
    pub struct BuildBazelRemoteExecutionV2ToolDetails {
        #[doc = "Name of the tool, e.g. bazel."]
        #[serde(rename = "toolName", default)]
        pub tool_name: ::std::option::Option<String>,
        #[doc = "Version of the tool used for the request, e.g. 5.0.3."]
        #[serde(rename = "toolVersion", default)]
        pub tool_version: ::std::option::Option<String>,
    }
    impl ::field_selector::FieldSelector for BuildBazelRemoteExecutionV2ToolDetails {
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
    pub struct BuildBazelRemoteExecutionV2Tree {
        #[doc = "All the child directories: the directories referred to by the root and,\nrecursively, all its children. In order to reconstruct the directory tree,\nthe client must take the digests of each of the child directories and then\nbuild up a tree starting from the `root`."]
        #[serde(rename = "children", default)]
        pub children:
            ::std::option::Option<Vec<crate::schemas::BuildBazelRemoteExecutionV2Directory>>,
        #[doc = "The root directory in the tree."]
        #[serde(rename = "root", default)]
        pub root: ::std::option::Option<crate::schemas::BuildBazelRemoteExecutionV2Directory>,
    }
    impl ::field_selector::FieldSelector for BuildBazelRemoteExecutionV2Tree {
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
    pub struct GoogleDevtoolsRemotebuildbotCommandDurations {
        #[doc = "The time spent preparing the command to be run in a Docker container\n(includes pulling the Docker image, if necessary)."]
        #[serde(rename = "dockerPrep", default)]
        pub docker_prep: ::std::option::Option<String>,
        #[doc = "The time spent downloading the input files and constructing the working\ndirectory."]
        #[serde(rename = "download", default)]
        pub download: ::std::option::Option<String>,
        #[doc = "The time spent executing the command (i.e., doing useful work)."]
        #[serde(rename = "execution", default)]
        pub execution: ::std::option::Option<String>,
        #[doc = "The timestamp when preparation is done and bot starts downloading files."]
        #[serde(rename = "isoPrepDone", default)]
        pub iso_prep_done: ::std::option::Option<String>,
        #[doc = "The time spent completing the command, in total."]
        #[serde(rename = "overall", default)]
        pub overall: ::std::option::Option<String>,
        #[doc = "The time spent uploading the stdout logs."]
        #[serde(rename = "stdout", default)]
        pub stdout: ::std::option::Option<String>,
        #[doc = "The time spent uploading the output files."]
        #[serde(rename = "upload", default)]
        pub upload: ::std::option::Option<String>,
    }
    impl ::field_selector::FieldSelector for GoogleDevtoolsRemotebuildbotCommandDurations {
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
    pub struct GoogleDevtoolsRemotebuildbotCommandEvents {
        #[doc = "Indicates whether we are using a cached Docker image (true) or had to pull\nthe Docker image (false) for this command."]
        #[serde(rename = "dockerCacheHit", default)]
        pub docker_cache_hit: ::std::option::Option<bool>,
        #[doc = "The input cache miss ratio."]
        #[serde(rename = "inputCacheMiss", default)]
        pub input_cache_miss: ::std::option::Option<f32>,
        #[doc = "The number of errors reported."]
        #[serde(rename = "numErrors", default)]
        #[serde(with = "crate::parsed_string")]
        pub num_errors: ::std::option::Option<u64>,
        #[doc = "The number of warnings reported."]
        #[serde(rename = "numWarnings", default)]
        #[serde(with = "crate::parsed_string")]
        pub num_warnings: ::std::option::Option<u64>,
    }
    impl ::field_selector::FieldSelector for GoogleDevtoolsRemotebuildbotCommandEvents {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleDevtoolsRemotebuildbotCommandStatusCode {
        #[doc = "The command was aborted."]
        Aborted,
        #[doc = "The bot failed to do the cleanup, e.g. unable to delete the command\nworking directory or the command process."]
        CleanupError,
        #[doc = "The command had passed its expiry time while it was still running."]
        DeadlineExceeded,
        #[doc = "The bot failed to check docker images."]
        DockerImageExistError,
        #[doc = "The docker image cannot be found."]
        DockerImageNotFound,
        #[doc = "The bot doesn't have the permissions to pull docker images."]
        DockerImagePermissionDenied,
        #[doc = "The bot failed to pull docker image."]
        DockerImagePullError,
        #[doc = "The bot failed to login to docker."]
        DockerLoginError,
        #[doc = "There are issues with docker service/runtime."]
        DockerUnavailable,
        #[doc = "The bot failed to download the inputs."]
        DownloadInputsError,
        #[doc = "The inputs contain duplicate files."]
        DuplicateInputs,
        #[doc = "The command failed because of some invariants expected by the underlying\nsystem have been broken. This usually indicates a bug wit the system."]
        Internal,
        #[doc = "The command input was invalid."]
        InvalidArgument,
        #[doc = "The resources requested by the command were not found."]
        NotFound,
        #[doc = "The command succeeded."]
        Ok,
        #[doc = "The command failed due to permission errors."]
        PermissionDenied,
        #[doc = "Unknown error."]
        Unknown,
        #[doc = "The bot failed to upload the outputs."]
        UploadOutputsError,
        #[doc = "Working directory is not found."]
        WorkingDirNotFound,
        #[doc = "Working directory is not under the base directory"]
        WorkingDirNotInBaseDir,
    }
    impl GoogleDevtoolsRemotebuildbotCommandStatusCode {
        pub fn as_str(self) -> &'static str {
            match self {
                GoogleDevtoolsRemotebuildbotCommandStatusCode::Aborted => "ABORTED",
                GoogleDevtoolsRemotebuildbotCommandStatusCode::CleanupError => "CLEANUP_ERROR",
                GoogleDevtoolsRemotebuildbotCommandStatusCode::DeadlineExceeded => {
                    "DEADLINE_EXCEEDED"
                }
                GoogleDevtoolsRemotebuildbotCommandStatusCode::DockerImageExistError => {
                    "DOCKER_IMAGE_EXIST_ERROR"
                }
                GoogleDevtoolsRemotebuildbotCommandStatusCode::DockerImageNotFound => {
                    "DOCKER_IMAGE_NOT_FOUND"
                }
                GoogleDevtoolsRemotebuildbotCommandStatusCode::DockerImagePermissionDenied => {
                    "DOCKER_IMAGE_PERMISSION_DENIED"
                }
                GoogleDevtoolsRemotebuildbotCommandStatusCode::DockerImagePullError => {
                    "DOCKER_IMAGE_PULL_ERROR"
                }
                GoogleDevtoolsRemotebuildbotCommandStatusCode::DockerLoginError => {
                    "DOCKER_LOGIN_ERROR"
                }
                GoogleDevtoolsRemotebuildbotCommandStatusCode::DockerUnavailable => {
                    "DOCKER_UNAVAILABLE"
                }
                GoogleDevtoolsRemotebuildbotCommandStatusCode::DownloadInputsError => {
                    "DOWNLOAD_INPUTS_ERROR"
                }
                GoogleDevtoolsRemotebuildbotCommandStatusCode::DuplicateInputs => {
                    "DUPLICATE_INPUTS"
                }
                GoogleDevtoolsRemotebuildbotCommandStatusCode::Internal => "INTERNAL",
                GoogleDevtoolsRemotebuildbotCommandStatusCode::InvalidArgument => {
                    "INVALID_ARGUMENT"
                }
                GoogleDevtoolsRemotebuildbotCommandStatusCode::NotFound => "NOT_FOUND",
                GoogleDevtoolsRemotebuildbotCommandStatusCode::Ok => "OK",
                GoogleDevtoolsRemotebuildbotCommandStatusCode::PermissionDenied => {
                    "PERMISSION_DENIED"
                }
                GoogleDevtoolsRemotebuildbotCommandStatusCode::Unknown => "UNKNOWN",
                GoogleDevtoolsRemotebuildbotCommandStatusCode::UploadOutputsError => {
                    "UPLOAD_OUTPUTS_ERROR"
                }
                GoogleDevtoolsRemotebuildbotCommandStatusCode::WorkingDirNotFound => {
                    "WORKING_DIR_NOT_FOUND"
                }
                GoogleDevtoolsRemotebuildbotCommandStatusCode::WorkingDirNotInBaseDir => {
                    "WORKING_DIR_NOT_IN_BASE_DIR"
                }
            }
        }
    }
    impl ::std::fmt::Display for GoogleDevtoolsRemotebuildbotCommandStatusCode {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GoogleDevtoolsRemotebuildbotCommandStatusCode {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for GoogleDevtoolsRemotebuildbotCommandStatusCode {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "ABORTED" => GoogleDevtoolsRemotebuildbotCommandStatusCode::Aborted,
                "CLEANUP_ERROR" => GoogleDevtoolsRemotebuildbotCommandStatusCode::CleanupError,
                "DEADLINE_EXCEEDED" => {
                    GoogleDevtoolsRemotebuildbotCommandStatusCode::DeadlineExceeded
                }
                "DOCKER_IMAGE_EXIST_ERROR" => {
                    GoogleDevtoolsRemotebuildbotCommandStatusCode::DockerImageExistError
                }
                "DOCKER_IMAGE_NOT_FOUND" => {
                    GoogleDevtoolsRemotebuildbotCommandStatusCode::DockerImageNotFound
                }
                "DOCKER_IMAGE_PERMISSION_DENIED" => {
                    GoogleDevtoolsRemotebuildbotCommandStatusCode::DockerImagePermissionDenied
                }
                "DOCKER_IMAGE_PULL_ERROR" => {
                    GoogleDevtoolsRemotebuildbotCommandStatusCode::DockerImagePullError
                }
                "DOCKER_LOGIN_ERROR" => {
                    GoogleDevtoolsRemotebuildbotCommandStatusCode::DockerLoginError
                }
                "DOCKER_UNAVAILABLE" => {
                    GoogleDevtoolsRemotebuildbotCommandStatusCode::DockerUnavailable
                }
                "DOWNLOAD_INPUTS_ERROR" => {
                    GoogleDevtoolsRemotebuildbotCommandStatusCode::DownloadInputsError
                }
                "DUPLICATE_INPUTS" => {
                    GoogleDevtoolsRemotebuildbotCommandStatusCode::DuplicateInputs
                }
                "INTERNAL" => GoogleDevtoolsRemotebuildbotCommandStatusCode::Internal,
                "INVALID_ARGUMENT" => {
                    GoogleDevtoolsRemotebuildbotCommandStatusCode::InvalidArgument
                }
                "NOT_FOUND" => GoogleDevtoolsRemotebuildbotCommandStatusCode::NotFound,
                "OK" => GoogleDevtoolsRemotebuildbotCommandStatusCode::Ok,
                "PERMISSION_DENIED" => {
                    GoogleDevtoolsRemotebuildbotCommandStatusCode::PermissionDenied
                }
                "UNKNOWN" => GoogleDevtoolsRemotebuildbotCommandStatusCode::Unknown,
                "UPLOAD_OUTPUTS_ERROR" => {
                    GoogleDevtoolsRemotebuildbotCommandStatusCode::UploadOutputsError
                }
                "WORKING_DIR_NOT_FOUND" => {
                    GoogleDevtoolsRemotebuildbotCommandStatusCode::WorkingDirNotFound
                }
                "WORKING_DIR_NOT_IN_BASE_DIR" => {
                    GoogleDevtoolsRemotebuildbotCommandStatusCode::WorkingDirNotInBaseDir
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
    impl ::field_selector::FieldSelector for GoogleDevtoolsRemotebuildbotCommandStatusCode {
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
    pub struct GoogleDevtoolsRemotebuildbotCommandStatus {
        #[doc = "The status code."]
        #[serde(rename = "code", default)]
        pub code:
            ::std::option::Option<crate::schemas::GoogleDevtoolsRemotebuildbotCommandStatusCode>,
        #[doc = "The error message."]
        #[serde(rename = "message", default)]
        pub message: ::std::option::Option<String>,
    }
    impl ::field_selector::FieldSelector for GoogleDevtoolsRemotebuildbotCommandStatus {
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
    pub struct GoogleDevtoolsRemotebuildexecutionAdminV1AlphaAcceleratorConfig {
        #[doc = "The number of the guest accelerator cards exposed to this VM."]
        #[serde(rename = "acceleratorCount", default)]
        #[serde(with = "crate::parsed_string")]
        pub accelerator_count: ::std::option::Option<i64>,
        #[doc = "The type of accelerator to attach to this VM, e.g. \"nvidia-tesla-k80\" for\nnVidia Tesla K80."]
        #[serde(rename = "acceleratorType", default)]
        pub accelerator_type: ::std::option::Option<String>,
    }
    impl ::field_selector::FieldSelector
        for GoogleDevtoolsRemotebuildexecutionAdminV1AlphaAcceleratorConfig
    {
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
    pub struct GoogleDevtoolsRemotebuildexecutionAdminV1AlphaCreateInstanceRequest {
        #[doc = "Specifies the instance to create.\nThe name in the instance, if specified in the instance, is ignored."]
        #[serde(rename = "instance", default)]
        pub instance: ::std::option::Option<
            crate::schemas::GoogleDevtoolsRemotebuildexecutionAdminV1AlphaInstance,
        >,
        #[doc = "ID of the created instance.\nA valid `instance_id` must:\nbe 6-50 characters long,\ncontain only lowercase letters, digits, hyphens and underscores,\nstart with a lowercase letter, and\nend with a lowercase letter or a digit."]
        #[serde(rename = "instanceId", default)]
        pub instance_id: ::std::option::Option<String>,
        #[doc = "Resource name of the project containing the instance.\nFormat: `projects/[PROJECT_ID]`."]
        #[serde(rename = "parent", default)]
        pub parent: ::std::option::Option<String>,
    }
    impl ::field_selector::FieldSelector
        for GoogleDevtoolsRemotebuildexecutionAdminV1AlphaCreateInstanceRequest
    {
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
    pub struct GoogleDevtoolsRemotebuildexecutionAdminV1AlphaCreateWorkerPoolRequest {
        #[doc = "Resource name of the instance in which to create the new worker pool.\nFormat: `projects/[PROJECT_ID]/instances/[INSTANCE_ID]`."]
        #[serde(rename = "parent", default)]
        pub parent: ::std::option::Option<String>,
        #[doc = "ID of the created worker pool.\nA valid pool ID must:\nbe 6-50 characters long,\ncontain only lowercase letters, digits, hyphens and underscores,\nstart with a lowercase letter, and\nend with a lowercase letter or a digit."]
        #[serde(rename = "poolId", default)]
        pub pool_id: ::std::option::Option<String>,
        #[doc = "Specifies the worker pool to create.\nThe name in the worker pool, if specified, is ignored."]
        #[serde(rename = "workerPool", default)]
        pub worker_pool: ::std::option::Option<
            crate::schemas::GoogleDevtoolsRemotebuildexecutionAdminV1AlphaWorkerPool,
        >,
    }
    impl ::field_selector::FieldSelector
        for GoogleDevtoolsRemotebuildexecutionAdminV1AlphaCreateWorkerPoolRequest
    {
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
    pub struct GoogleDevtoolsRemotebuildexecutionAdminV1AlphaDeleteInstanceRequest {
        #[doc = "Name of the instance to delete.\nFormat: `projects/[PROJECT_ID]/instances/[INSTANCE_ID]`."]
        #[serde(rename = "name", default)]
        pub name: ::std::option::Option<String>,
    }
    impl ::field_selector::FieldSelector
        for GoogleDevtoolsRemotebuildexecutionAdminV1AlphaDeleteInstanceRequest
    {
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
    pub struct GoogleDevtoolsRemotebuildexecutionAdminV1AlphaDeleteWorkerPoolRequest {
        #[doc = "Name of the worker pool to delete.\nFormat:\n`projects/[PROJECT_ID]/instances/[INSTANCE_ID]/workerpools/[POOL_ID]`."]
        #[serde(rename = "name", default)]
        pub name: ::std::option::Option<String>,
    }
    impl ::field_selector::FieldSelector
        for GoogleDevtoolsRemotebuildexecutionAdminV1AlphaDeleteWorkerPoolRequest
    {
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
    pub struct GoogleDevtoolsRemotebuildexecutionAdminV1AlphaGetInstanceRequest {
        #[doc = "Name of the instance to retrieve.\nFormat: `projects/[PROJECT_ID]/instances/[INSTANCE_ID]`."]
        #[serde(rename = "name", default)]
        pub name: ::std::option::Option<String>,
    }
    impl ::field_selector::FieldSelector
        for GoogleDevtoolsRemotebuildexecutionAdminV1AlphaGetInstanceRequest
    {
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
    pub struct GoogleDevtoolsRemotebuildexecutionAdminV1AlphaGetWorkerPoolRequest {
        #[doc = "Name of the worker pool to retrieve.\nFormat:\n`projects/[PROJECT_ID]/instances/[INSTANCE_ID]/workerpools/[POOL_ID]`."]
        #[serde(rename = "name", default)]
        pub name: ::std::option::Option<String>,
    }
    impl ::field_selector::FieldSelector
        for GoogleDevtoolsRemotebuildexecutionAdminV1AlphaGetWorkerPoolRequest
    {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleDevtoolsRemotebuildexecutionAdminV1AlphaInstanceState {
        #[doc = "The instance is in state `CREATING` once `CreateInstance` is called and\nbefore the instance is ready for use."]
        Creating,
        #[doc = "An `INACTIVE` instance indicates that there is a problem that needs to be\nfixed. Such instances cannot be used for execution and instances that\nremain in this state for a significant period of time will be removed\npermanently."]
        Inactive,
        #[doc = "The instance is in state `RUNNING` when it is ready for use."]
        Running,
        #[doc = "Not a valid state, but the default value of the enum."]
        StateUnspecified,
    }
    impl GoogleDevtoolsRemotebuildexecutionAdminV1AlphaInstanceState {
        pub fn as_str(self) -> &'static str {
            match self {
                GoogleDevtoolsRemotebuildexecutionAdminV1AlphaInstanceState::Creating => "CREATING",
                GoogleDevtoolsRemotebuildexecutionAdminV1AlphaInstanceState::Inactive => "INACTIVE",
                GoogleDevtoolsRemotebuildexecutionAdminV1AlphaInstanceState::Running => "RUNNING",
                GoogleDevtoolsRemotebuildexecutionAdminV1AlphaInstanceState::StateUnspecified => {
                    "STATE_UNSPECIFIED"
                }
            }
        }
    }
    impl ::std::fmt::Display for GoogleDevtoolsRemotebuildexecutionAdminV1AlphaInstanceState {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GoogleDevtoolsRemotebuildexecutionAdminV1AlphaInstanceState {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de>
        for GoogleDevtoolsRemotebuildexecutionAdminV1AlphaInstanceState
    {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "CREATING" => GoogleDevtoolsRemotebuildexecutionAdminV1AlphaInstanceState::Creating,
                "INACTIVE" => GoogleDevtoolsRemotebuildexecutionAdminV1AlphaInstanceState::Inactive,
                "RUNNING" => GoogleDevtoolsRemotebuildexecutionAdminV1AlphaInstanceState::Running,
                "STATE_UNSPECIFIED" => {
                    GoogleDevtoolsRemotebuildexecutionAdminV1AlphaInstanceState::StateUnspecified
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
    impl ::field_selector::FieldSelector
        for GoogleDevtoolsRemotebuildexecutionAdminV1AlphaInstanceState
    {
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
    pub struct GoogleDevtoolsRemotebuildexecutionAdminV1AlphaInstance {
        #[doc = "The location is a GCP region. Currently only `us-central1` is supported."]
        #[serde(rename = "location", default)]
        pub location: ::std::option::Option<String>,
        #[doc = "Output only. Whether stack driver logging is enabled for the instance."]
        #[serde(rename = "loggingEnabled", default)]
        pub logging_enabled: ::std::option::Option<bool>,
        #[doc = "Output only. Instance resource name formatted as:\n`projects/[PROJECT_ID]/instances/[INSTANCE_ID]`.\nName should not be populated when creating an instance since it is provided\nin the `instance_id` field."]
        #[serde(rename = "name", default)]
        pub name: ::std::option::Option<String>,
        #[doc = "Output only. State of the instance."]
        #[serde(rename = "state", default)]
        pub state: ::std::option::Option<
            crate::schemas::GoogleDevtoolsRemotebuildexecutionAdminV1AlphaInstanceState,
        >,
    }
    impl ::field_selector::FieldSelector for GoogleDevtoolsRemotebuildexecutionAdminV1AlphaInstance {
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
    pub struct GoogleDevtoolsRemotebuildexecutionAdminV1AlphaListInstancesRequest {
        #[doc = "Resource name of the project.\nFormat: `projects/[PROJECT_ID]`."]
        #[serde(rename = "parent", default)]
        pub parent: ::std::option::Option<String>,
    }
    impl ::field_selector::FieldSelector
        for GoogleDevtoolsRemotebuildexecutionAdminV1AlphaListInstancesRequest
    {
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
    pub struct GoogleDevtoolsRemotebuildexecutionAdminV1AlphaListInstancesResponse {
        #[doc = "The list of instances in a given project."]
        #[serde(rename = "instances", default)]
        pub instances: ::std::option::Option<
            Vec<crate::schemas::GoogleDevtoolsRemotebuildexecutionAdminV1AlphaInstance>,
        >,
    }
    impl ::field_selector::FieldSelector
        for GoogleDevtoolsRemotebuildexecutionAdminV1AlphaListInstancesResponse
    {
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
    pub struct GoogleDevtoolsRemotebuildexecutionAdminV1AlphaListWorkerPoolsRequest {
        #[doc = "Optional. A filter expression that filters resources listed in\nthe response. The expression must specify the field name, a comparison\noperator, and the value that you want to use for filtering. The value\nmust be a string, a number, or a boolean. String values are\ncase-insensitive.\nThe comparison operator must be either `:`, `=`, `!=`, `>`, `>=`, `<=` or\n`<`.\nThe `:` operator can be used with string fields to match substrings.\nFor non-string fields it is equivalent to the `=` operator.\nThe `:*` comparison can be used to test  whether a key has been defined.\n\nYou can also filter on nested fields.\n\nTo filter on multiple expressions, you can separate expression using\n`AND` and `OR` operators, using parentheses to specify precedence. If\nneither operator is specified, `AND` is assumed.\n\nExamples:\n\nInclude only pools with more than 100 reserved workers:\n`(worker_count > 100) (worker_config.reserved = true)`\n\nInclude only pools with a certain label or machines of the n1-standard\nfamily:\n`worker_config.labels.key1 : * OR worker_config.machine_type: n1-standard`"]
        #[serde(rename = "filter", default)]
        pub filter: ::std::option::Option<String>,
        #[doc = "Resource name of the instance.\nFormat: `projects/[PROJECT_ID]/instances/[INSTANCE_ID]`."]
        #[serde(rename = "parent", default)]
        pub parent: ::std::option::Option<String>,
    }
    impl ::field_selector::FieldSelector
        for GoogleDevtoolsRemotebuildexecutionAdminV1AlphaListWorkerPoolsRequest
    {
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
    pub struct GoogleDevtoolsRemotebuildexecutionAdminV1AlphaListWorkerPoolsResponse {
        #[doc = "The list of worker pools in a given instance."]
        #[serde(rename = "workerPools", default)]
        pub worker_pools: ::std::option::Option<
            Vec<crate::schemas::GoogleDevtoolsRemotebuildexecutionAdminV1AlphaWorkerPool>,
        >,
    }
    impl ::field_selector::FieldSelector
        for GoogleDevtoolsRemotebuildexecutionAdminV1AlphaListWorkerPoolsResponse
    {
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
    pub struct GoogleDevtoolsRemotebuildexecutionAdminV1AlphaUpdateWorkerPoolRequest {
        #[doc = "The update mask applies to worker_pool. For the `FieldMask` definition,\nsee\nhttps://developers.google.com/protocol-buffers/docs/reference/google.protobuf#fieldmask\nIf an empty update_mask is provided, only the non-default valued field in\nthe worker pool field will be updated. Note that in order to update a field\nto the default value (zero, false, empty string) an explicit update_mask\nmust be provided."]
        #[serde(rename = "updateMask", default)]
        pub update_mask: ::std::option::Option<String>,
        #[doc = "Specifies the worker pool to update."]
        #[serde(rename = "workerPool", default)]
        pub worker_pool: ::std::option::Option<
            crate::schemas::GoogleDevtoolsRemotebuildexecutionAdminV1AlphaWorkerPool,
        >,
    }
    impl ::field_selector::FieldSelector
        for GoogleDevtoolsRemotebuildexecutionAdminV1AlphaUpdateWorkerPoolRequest
    {
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
    pub struct GoogleDevtoolsRemotebuildexecutionAdminV1AlphaWorkerConfig {
        #[doc = "The accelerator card attached to each VM."]
        #[serde(rename = "accelerator", default)]
        pub accelerator: ::std::option::Option<
            crate::schemas::GoogleDevtoolsRemotebuildexecutionAdminV1AlphaAcceleratorConfig,
        >,
        #[doc = "Required. Size of the disk attached to the worker, in GB.\nSee https://cloud.google.com/compute/docs/disks/"]
        #[serde(rename = "diskSizeGb", default)]
        #[serde(with = "crate::parsed_string")]
        pub disk_size_gb: ::std::option::Option<i64>,
        #[doc = "Required. Disk Type to use for the worker.\nSee [Storage\noptions](https://cloud.google.com/compute/docs/disks/#introduction).\nCurrently only `pd-standard` is supported."]
        #[serde(rename = "diskType", default)]
        pub disk_type: ::std::option::Option<String>,
        #[doc = "Labels associated with the workers.\nLabel keys and values can be no longer than 63 characters, can only contain\nlowercase letters, numeric characters, underscores and dashes.\nInternational letters are permitted. Label keys must start with a letter.\nLabel values are optional.\nThere can not be more than 64 labels per resource."]
        #[serde(rename = "labels", default)]
        pub labels: ::std::option::Option<::std::collections::BTreeMap<String, String>>,
        #[doc = "Required. Machine type of the worker, such as `n1-standard-2`.\nSee https://cloud.google.com/compute/docs/machine-types for a list of\nsupported machine types. Note that `f1-micro` and `g1-small` are not yet\nsupported."]
        #[serde(rename = "machineType", default)]
        pub machine_type: ::std::option::Option<String>,
        #[doc = "Minimum CPU platform to use when creating the worker.\nSee [CPU Platforms](https://cloud.google.com/compute/docs/cpu-platforms)."]
        #[serde(rename = "minCpuPlatform", default)]
        pub min_cpu_platform: ::std::option::Option<String>,
        #[doc = "Determines whether the worker is reserved (equivalent to a Compute Engine\non-demand VM and therefore won't be preempted).\nSee [Preemptible VMs](https://cloud.google.com/preemptible-vms/) for more\ndetails."]
        #[serde(rename = "reserved", default)]
        pub reserved: ::std::option::Option<bool>,
    }
    impl ::field_selector::FieldSelector
        for GoogleDevtoolsRemotebuildexecutionAdminV1AlphaWorkerConfig
    {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleDevtoolsRemotebuildexecutionAdminV1AlphaWorkerPoolState {
        #[doc = "The worker pool is in state `CREATING` once `CreateWorkerPool` is called\nand before all requested workers are ready."]
        Creating,
        #[doc = "The worker pool is in state `DELETING` once the `Delete` method is called\nand before the deletion completes."]
        Deleting,
        #[doc = "The worker pool is in state `INACTIVE` when the instance hosting the\nworker pool in not running."]
        Inactive,
        #[doc = "The worker pool is in state `RUNNING` when all its workers are ready for\nuse."]
        Running,
        #[doc = "Not a valid state, but the default value of the enum."]
        StateUnspecified,
        #[doc = "The worker pool is in state `UPDATING` once `UpdateWorkerPool` is called\nand before the new configuration has all the requested workers ready for\nuse, and no older configuration has any workers. At that point the state\ntransitions to `RUNNING`."]
        Updating,
    }
    impl GoogleDevtoolsRemotebuildexecutionAdminV1AlphaWorkerPoolState {
        pub fn as_str(self) -> &'static str {
            match self {
                GoogleDevtoolsRemotebuildexecutionAdminV1AlphaWorkerPoolState::Creating => {
                    "CREATING"
                }
                GoogleDevtoolsRemotebuildexecutionAdminV1AlphaWorkerPoolState::Deleting => {
                    "DELETING"
                }
                GoogleDevtoolsRemotebuildexecutionAdminV1AlphaWorkerPoolState::Inactive => {
                    "INACTIVE"
                }
                GoogleDevtoolsRemotebuildexecutionAdminV1AlphaWorkerPoolState::Running => "RUNNING",
                GoogleDevtoolsRemotebuildexecutionAdminV1AlphaWorkerPoolState::StateUnspecified => {
                    "STATE_UNSPECIFIED"
                }
                GoogleDevtoolsRemotebuildexecutionAdminV1AlphaWorkerPoolState::Updating => {
                    "UPDATING"
                }
            }
        }
    }
    impl ::std::fmt::Display for GoogleDevtoolsRemotebuildexecutionAdminV1AlphaWorkerPoolState {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GoogleDevtoolsRemotebuildexecutionAdminV1AlphaWorkerPoolState {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de>
        for GoogleDevtoolsRemotebuildexecutionAdminV1AlphaWorkerPoolState
    {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "CREATING" => {
                    GoogleDevtoolsRemotebuildexecutionAdminV1AlphaWorkerPoolState::Creating
                }
                "DELETING" => {
                    GoogleDevtoolsRemotebuildexecutionAdminV1AlphaWorkerPoolState::Deleting
                }
                "INACTIVE" => {
                    GoogleDevtoolsRemotebuildexecutionAdminV1AlphaWorkerPoolState::Inactive
                }
                "RUNNING" => GoogleDevtoolsRemotebuildexecutionAdminV1AlphaWorkerPoolState::Running,
                "STATE_UNSPECIFIED" => {
                    GoogleDevtoolsRemotebuildexecutionAdminV1AlphaWorkerPoolState::StateUnspecified
                }
                "UPDATING" => {
                    GoogleDevtoolsRemotebuildexecutionAdminV1AlphaWorkerPoolState::Updating
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
    impl ::field_selector::FieldSelector
        for GoogleDevtoolsRemotebuildexecutionAdminV1AlphaWorkerPoolState
    {
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
    pub struct GoogleDevtoolsRemotebuildexecutionAdminV1AlphaWorkerPool {
        #[doc = "WorkerPool resource name formatted as:\n`projects/[PROJECT_ID]/instances/[INSTANCE_ID]/workerpools/[POOL_ID]`.\nname should not be populated when creating a worker pool since it is\nprovided in the `poolId` field."]
        #[serde(rename = "name", default)]
        pub name: ::std::option::Option<String>,
        #[doc = "Output only. State of the worker pool."]
        #[serde(rename = "state", default)]
        pub state: ::std::option::Option<
            crate::schemas::GoogleDevtoolsRemotebuildexecutionAdminV1AlphaWorkerPoolState,
        >,
        #[doc = "Specifies the properties, such as machine type and disk size, used for\ncreating workers in a worker pool."]
        #[serde(rename = "workerConfig", default)]
        pub worker_config: ::std::option::Option<
            crate::schemas::GoogleDevtoolsRemotebuildexecutionAdminV1AlphaWorkerConfig,
        >,
        #[doc = "The desired number of workers in the worker pool. Must be a value between\n0 and 1000."]
        #[serde(rename = "workerCount", default)]
        #[serde(with = "crate::parsed_string")]
        pub worker_count: ::std::option::Option<i64>,
    }
    impl ::field_selector::FieldSelector for GoogleDevtoolsRemotebuildexecutionAdminV1AlphaWorkerPool {
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
    pub struct GoogleDevtoolsRemoteexecutionV1TestActionResult {
        #[doc = "The exit code of the command."]
        #[serde(rename = "exitCode", default)]
        pub exit_code: ::std::option::Option<i32>,
        #[doc = "The output directories of the action. For each output directory requested\nin the `output_directories` field of the Action, if the corresponding\ndirectory existed after the action completed, a single entry will be\npresent in the output list, which will contain the digest of\na Tree message containing\nthe directory tree, and the path equal exactly to the corresponding Action\noutput_directories member.\nAs an example, suppose the Action had an output directory `a/b/dir` and the\nexecution produced the following contents in `a/b/dir`: a file named `bar`\nand a directory named `foo` with an executable file named `baz`. Then,\noutput_directory will contain (hashes shortened for readability):\n\n````textjson\n// OutputDirectory proto:\n{\n  path: \"a/b/dir\"\n  tree_digest: {\n    hash: \"4a73bc9d03...\",\n    size: 55\n  }\n}\n// Tree proto with hash \"4a73bc9d03...\" and size 55:\n{\n  root: {\n    files: [\n      {\n        name: \"bar\",\n        digest: {\n          hash: \"4a73bc9d03...\",\n          size: 65534\n        }\n      }\n    ],\n    directories: [\n      {\n        name: \"foo\",\n        digest: {\n          hash: \"4cf2eda940...\",\n          size: 43\n        }\n      }\n    ]\n  }\n  children : {\n    // (Directory proto with hash \"4cf2eda940...\" and size 43)\n    files: [\n      {\n        name: \"baz\",\n        digest: {\n          hash: \"b2c941073e...\",\n          size: 1294,\n        },\n        is_executable: true\n      }\n    ]\n  }\n}\n````"]
        #[serde(rename = "outputDirectories", default)]
        pub output_directories: ::std::option::Option<
            Vec<crate::schemas::GoogleDevtoolsRemoteexecutionV1TestOutputDirectory>,
        >,
        #[doc = "The output files of the action. For each output file requested in the\n`output_files` field of the Action, if the corresponding file existed after\nthe action completed, a single entry will be present in the output list.\n\nIf the action does not produce the requested output, or produces a\ndirectory where a regular file is expected or vice versa, then that output\nwill be omitted from the list. The server is free to arrange the output\nlist as desired; clients MUST NOT assume that the output list is sorted."]
        #[serde(rename = "outputFiles", default)]
        pub output_files: ::std::option::Option<
            Vec<crate::schemas::GoogleDevtoolsRemoteexecutionV1TestOutputFile>,
        >,
        #[doc = "The digest for a blob containing the standard error of the action, which\ncan be retrieved from the\nContentAddressableStorage.\nSee `stderr_raw` for when this will be set."]
        #[serde(rename = "stderrDigest", default)]
        pub stderr_digest:
            ::std::option::Option<crate::schemas::GoogleDevtoolsRemoteexecutionV1TestDigest>,
        #[doc = "The standard error buffer of the action. The server will determine, based\non the size of the buffer, whether to return it in raw form or to return\na digest in `stderr_digest` that points to the buffer. If neither is set,\nthen the buffer is empty. The client SHOULD NOT assume it will get one of\nthe raw buffer or a digest on any given request and should be prepared to\nhandle either."]
        #[serde(rename = "stderrRaw", default)]
        pub stderr_raw: ::std::option::Option<crate::bytes::Bytes>,
        #[doc = "The digest for a blob containing the standard output of the action, which\ncan be retrieved from the\nContentAddressableStorage.\nSee `stdout_raw` for when this will be set."]
        #[serde(rename = "stdoutDigest", default)]
        pub stdout_digest:
            ::std::option::Option<crate::schemas::GoogleDevtoolsRemoteexecutionV1TestDigest>,
        #[doc = "The standard output buffer of the action. The server will determine, based\non the size of the buffer, whether to return it in raw form or to return\na digest in `stdout_digest` that points to the buffer. If neither is set,\nthen the buffer is empty. The client SHOULD NOT assume it will get one of\nthe raw buffer or a digest on any given request and should be prepared to\nhandle either."]
        #[serde(rename = "stdoutRaw", default)]
        pub stdout_raw: ::std::option::Option<crate::bytes::Bytes>,
    }
    impl ::field_selector::FieldSelector for GoogleDevtoolsRemoteexecutionV1TestActionResult {
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
    pub struct GoogleDevtoolsRemoteexecutionV1TestCommand {
        #[doc = "The arguments to the command. The first argument must be the path to the\nexecutable, which must be either a relative path, in which case it is\nevaluated with respect to the input root, or an absolute path.\n\nThe working directory will always be the input root."]
        #[serde(rename = "arguments", default)]
        pub arguments: ::std::option::Option<Vec<String>>,
        #[doc = "The environment variables to set when running the program. The worker may\nprovide its own default environment variables; these defaults can be\noverridden using this field. Additional variables can also be specified.\n\nIn order to ensure that equivalent `Command`s always hash to the same\nvalue, the environment variables MUST be lexicographically sorted by name.\nSorting of strings is done by code point, equivalently, by the UTF-8 bytes."]
        #[serde(rename = "environmentVariables", default)]
        pub environment_variables: ::std::option::Option<
            Vec<crate::schemas::GoogleDevtoolsRemoteexecutionV1TestCommandEnvironmentVariable>,
        >,
    }
    impl ::field_selector::FieldSelector for GoogleDevtoolsRemoteexecutionV1TestCommand {
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
    pub struct GoogleDevtoolsRemoteexecutionV1TestCommandEnvironmentVariable {
        #[doc = "The variable name."]
        #[serde(rename = "name", default)]
        pub name: ::std::option::Option<String>,
        #[doc = "The variable value."]
        #[serde(rename = "value", default)]
        pub value: ::std::option::Option<String>,
    }
    impl ::field_selector::FieldSelector
        for GoogleDevtoolsRemoteexecutionV1TestCommandEnvironmentVariable
    {
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
    pub struct GoogleDevtoolsRemoteexecutionV1TestDigest {
        #[doc = "The hash. In the case of SHA-256, it will always be a lowercase hex string\nexactly 64 characters long."]
        #[serde(rename = "hash", default)]
        pub hash: ::std::option::Option<String>,
        #[doc = "The size of the blob, in bytes."]
        #[serde(rename = "sizeBytes", default)]
        #[serde(with = "crate::parsed_string")]
        pub size_bytes: ::std::option::Option<i64>,
    }
    impl ::field_selector::FieldSelector for GoogleDevtoolsRemoteexecutionV1TestDigest {
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
    pub struct GoogleDevtoolsRemoteexecutionV1TestDirectory {
        #[doc = "The subdirectories in the directory."]
        #[serde(rename = "directories", default)]
        pub directories: ::std::option::Option<
            Vec<crate::schemas::GoogleDevtoolsRemoteexecutionV1TestDirectoryNode>,
        >,
        #[doc = "The files in the directory."]
        #[serde(rename = "files", default)]
        pub files:
            ::std::option::Option<Vec<crate::schemas::GoogleDevtoolsRemoteexecutionV1TestFileNode>>,
    }
    impl ::field_selector::FieldSelector for GoogleDevtoolsRemoteexecutionV1TestDirectory {
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
    pub struct GoogleDevtoolsRemoteexecutionV1TestDirectoryNode {
        #[doc = "The digest of the\nDirectory object\nrepresented. See Digest\nfor information about how to take the digest of a proto message."]
        #[serde(rename = "digest", default)]
        pub digest:
            ::std::option::Option<crate::schemas::GoogleDevtoolsRemoteexecutionV1TestDigest>,
        #[doc = "The name of the directory."]
        #[serde(rename = "name", default)]
        pub name: ::std::option::Option<String>,
    }
    impl ::field_selector::FieldSelector for GoogleDevtoolsRemoteexecutionV1TestDirectoryNode {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleDevtoolsRemoteexecutionV1TestExecuteOperationMetadataStage {
        #[doc = "Checking the result against the cache."]
        CacheCheck,
        #[doc = "Finished execution."]
        Completed,
        #[doc = "Currently being executed by a worker."]
        Executing,
        #[doc = "Currently idle, awaiting a free machine to execute."]
        Queued,
        Unknown,
    }
    impl GoogleDevtoolsRemoteexecutionV1TestExecuteOperationMetadataStage {
        pub fn as_str(self) -> &'static str {
            match self {
                GoogleDevtoolsRemoteexecutionV1TestExecuteOperationMetadataStage::CacheCheck => {
                    "CACHE_CHECK"
                }
                GoogleDevtoolsRemoteexecutionV1TestExecuteOperationMetadataStage::Completed => {
                    "COMPLETED"
                }
                GoogleDevtoolsRemoteexecutionV1TestExecuteOperationMetadataStage::Executing => {
                    "EXECUTING"
                }
                GoogleDevtoolsRemoteexecutionV1TestExecuteOperationMetadataStage::Queued => {
                    "QUEUED"
                }
                GoogleDevtoolsRemoteexecutionV1TestExecuteOperationMetadataStage::Unknown => {
                    "UNKNOWN"
                }
            }
        }
    }
    impl ::std::fmt::Display for GoogleDevtoolsRemoteexecutionV1TestExecuteOperationMetadataStage {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GoogleDevtoolsRemoteexecutionV1TestExecuteOperationMetadataStage {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de>
        for GoogleDevtoolsRemoteexecutionV1TestExecuteOperationMetadataStage
    {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "CACHE_CHECK" => {
                    GoogleDevtoolsRemoteexecutionV1TestExecuteOperationMetadataStage::CacheCheck
                }
                "COMPLETED" => {
                    GoogleDevtoolsRemoteexecutionV1TestExecuteOperationMetadataStage::Completed
                }
                "EXECUTING" => {
                    GoogleDevtoolsRemoteexecutionV1TestExecuteOperationMetadataStage::Executing
                }
                "QUEUED" => {
                    GoogleDevtoolsRemoteexecutionV1TestExecuteOperationMetadataStage::Queued
                }
                "UNKNOWN" => {
                    GoogleDevtoolsRemoteexecutionV1TestExecuteOperationMetadataStage::Unknown
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
    impl ::field_selector::FieldSelector
        for GoogleDevtoolsRemoteexecutionV1TestExecuteOperationMetadataStage
    {
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
    pub struct GoogleDevtoolsRemoteexecutionV1TestExecuteOperationMetadata {
        #[doc = "The digest of the Action\nbeing executed."]
        #[serde(rename = "actionDigest", default)]
        pub action_digest:
            ::std::option::Option<crate::schemas::GoogleDevtoolsRemoteexecutionV1TestDigest>,
        #[serde(rename = "stage", default)]
        pub stage: ::std::option::Option<
            crate::schemas::GoogleDevtoolsRemoteexecutionV1TestExecuteOperationMetadataStage,
        >,
        #[doc = "If set, the client can use this name with\nByteStream.Read to stream the\nstandard error."]
        #[serde(rename = "stderrStreamName", default)]
        pub stderr_stream_name: ::std::option::Option<String>,
        #[doc = "If set, the client can use this name with\nByteStream.Read to stream the\nstandard output."]
        #[serde(rename = "stdoutStreamName", default)]
        pub stdout_stream_name: ::std::option::Option<String>,
    }
    impl ::field_selector::FieldSelector
        for GoogleDevtoolsRemoteexecutionV1TestExecuteOperationMetadata
    {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct GoogleDevtoolsRemoteexecutionV1TestExecuteResponse {
        #[doc = "True if the result was served from cache, false if it was executed."]
        #[serde(rename = "cachedResult", default)]
        pub cached_result: ::std::option::Option<bool>,
        #[doc = "The result of the action."]
        #[serde(rename = "result", default)]
        pub result:
            ::std::option::Option<crate::schemas::GoogleDevtoolsRemoteexecutionV1TestActionResult>,
        #[doc = "An optional list of additional log outputs the server wishes to provide. A\nserver can use this to return execution-specific logs however it wishes.\nThis is intended primarily to make it easier for users to debug issues that\nmay be outside of the actual job execution, such as by identifying the\nworker executing the action or by providing logs from the worker's setup\nphase. The keys SHOULD be human readable so that a client can display them\nto a user."]
        #[serde(rename = "serverLogs", default)]
        pub server_logs: ::std::option::Option<
            ::std::collections::BTreeMap<
                String,
                crate::schemas::GoogleDevtoolsRemoteexecutionV1TestLogFile,
            >,
        >,
        #[doc = "If the status has a code other than `OK`, it indicates that the action did\nnot finish execution. For example, if the operation times out during\nexecution, the status will have a `DEADLINE_EXCEEDED` code. Servers MUST\nuse this field for errors in execution, rather than the error field on the\n`Operation` object.\n\nIf the status code is other than `OK`, then the result MUST NOT be cached.\nFor an error status, the `result` field is optional; the server may\npopulate the output-, stdout-, and stderr-related fields if it has any\ninformation available, such as the stdout and stderr of a timed-out action."]
        #[serde(rename = "status", default)]
        pub status: ::std::option::Option<crate::schemas::GoogleRpcStatus>,
    }
    impl ::field_selector::FieldSelector for GoogleDevtoolsRemoteexecutionV1TestExecuteResponse {
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
    pub struct GoogleDevtoolsRemoteexecutionV1TestFileNode {
        #[doc = "The digest of the file's content."]
        #[serde(rename = "digest", default)]
        pub digest:
            ::std::option::Option<crate::schemas::GoogleDevtoolsRemoteexecutionV1TestDigest>,
        #[doc = "True if file is executable, false otherwise."]
        #[serde(rename = "isExecutable", default)]
        pub is_executable: ::std::option::Option<bool>,
        #[doc = "The name of the file."]
        #[serde(rename = "name", default)]
        pub name: ::std::option::Option<String>,
    }
    impl ::field_selector::FieldSelector for GoogleDevtoolsRemoteexecutionV1TestFileNode {
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
    pub struct GoogleDevtoolsRemoteexecutionV1TestLogFile {
        #[doc = "The digest of the log contents."]
        #[serde(rename = "digest", default)]
        pub digest:
            ::std::option::Option<crate::schemas::GoogleDevtoolsRemoteexecutionV1TestDigest>,
        #[doc = "This is a hint as to the purpose of the log, and is set to true if the log\nis human-readable text that can be usefully displayed to a user, and false\notherwise. For instance, if a command-line client wishes to print the\nserver logs to the terminal for a failed action, this allows it to avoid\ndisplaying a binary file."]
        #[serde(rename = "humanReadable", default)]
        pub human_readable: ::std::option::Option<bool>,
    }
    impl ::field_selector::FieldSelector for GoogleDevtoolsRemoteexecutionV1TestLogFile {
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
    pub struct GoogleDevtoolsRemoteexecutionV1TestOutputDirectory {
        #[doc = "DEPRECATED: This field is deprecated and should no longer be used."]
        #[serde(rename = "digest", default)]
        pub digest:
            ::std::option::Option<crate::schemas::GoogleDevtoolsRemoteexecutionV1TestDigest>,
        #[doc = "The full path of the directory relative to the working directory. The path\nseparator is a forward slash `/`. Since this is a relative path, it MUST\nNOT begin with a leading forward slash. The empty string value is allowed,\nand it denotes the entire working directory."]
        #[serde(rename = "path", default)]
        pub path: ::std::option::Option<String>,
        #[doc = "The digest of the encoded\nTree proto containing the\ndirectory's contents."]
        #[serde(rename = "treeDigest", default)]
        pub tree_digest:
            ::std::option::Option<crate::schemas::GoogleDevtoolsRemoteexecutionV1TestDigest>,
    }
    impl ::field_selector::FieldSelector for GoogleDevtoolsRemoteexecutionV1TestOutputDirectory {
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
    pub struct GoogleDevtoolsRemoteexecutionV1TestOutputFile {
        #[doc = "The raw content of the file.\n\nThis field may be used by the server to provide the content of a file\ninline in an\nActionResult and\navoid requiring that the client make a separate call to\n[ContentAddressableStorage.GetBlob] to retrieve it.\n\nThe client SHOULD NOT assume that it will get raw content with any request,\nand always be prepared to retrieve it via `digest`."]
        #[serde(rename = "content", default)]
        pub content: ::std::option::Option<crate::bytes::Bytes>,
        #[doc = "The digest of the file's content."]
        #[serde(rename = "digest", default)]
        pub digest:
            ::std::option::Option<crate::schemas::GoogleDevtoolsRemoteexecutionV1TestDigest>,
        #[doc = "True if file is executable, false otherwise."]
        #[serde(rename = "isExecutable", default)]
        pub is_executable: ::std::option::Option<bool>,
        #[doc = "The full path of the file relative to the input root, including the\nfilename. The path separator is a forward slash `/`. Since this is a\nrelative path, it MUST NOT begin with a leading forward slash."]
        #[serde(rename = "path", default)]
        pub path: ::std::option::Option<String>,
    }
    impl ::field_selector::FieldSelector for GoogleDevtoolsRemoteexecutionV1TestOutputFile {
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
    pub struct GoogleDevtoolsRemoteexecutionV1TestRequestMetadata {
        #[doc = "An identifier that ties multiple requests to the same action.\nFor example, multiple requests to the CAS, Action Cache, and Execution\nAPI are used in order to compile foo.cc."]
        #[serde(rename = "actionId", default)]
        pub action_id: ::std::option::Option<String>,
        #[doc = "An identifier to tie multiple tool invocations together. For example,\nruns of foo_test, bar_test and baz_test on a post-submit of a given patch."]
        #[serde(rename = "correlatedInvocationsId", default)]
        pub correlated_invocations_id: ::std::option::Option<String>,
        #[doc = "The details for the tool invoking the requests."]
        #[serde(rename = "toolDetails", default)]
        pub tool_details:
            ::std::option::Option<crate::schemas::GoogleDevtoolsRemoteexecutionV1TestToolDetails>,
        #[doc = "An identifier that ties multiple actions together to a final result.\nFor example, multiple actions are required to build and run foo_test."]
        #[serde(rename = "toolInvocationId", default)]
        pub tool_invocation_id: ::std::option::Option<String>,
    }
    impl ::field_selector::FieldSelector for GoogleDevtoolsRemoteexecutionV1TestRequestMetadata {
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
    pub struct GoogleDevtoolsRemoteexecutionV1TestToolDetails {
        #[doc = "Name of the tool, e.g. bazel."]
        #[serde(rename = "toolName", default)]
        pub tool_name: ::std::option::Option<String>,
        #[doc = "Version of the tool used for the request, e.g. 5.0.3."]
        #[serde(rename = "toolVersion", default)]
        pub tool_version: ::std::option::Option<String>,
    }
    impl ::field_selector::FieldSelector for GoogleDevtoolsRemoteexecutionV1TestToolDetails {
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
    pub struct GoogleDevtoolsRemoteexecutionV1TestTree {
        #[doc = "All the child directories: the directories referred to by the root and,\nrecursively, all its children. In order to reconstruct the directory tree,\nthe client must take the digests of each of the child directories and then\nbuild up a tree starting from the `root`."]
        #[serde(rename = "children", default)]
        pub children: ::std::option::Option<
            Vec<crate::schemas::GoogleDevtoolsRemoteexecutionV1TestDirectory>,
        >,
        #[doc = "The root directory in the tree."]
        #[serde(rename = "root", default)]
        pub root:
            ::std::option::Option<crate::schemas::GoogleDevtoolsRemoteexecutionV1TestDirectory>,
    }
    impl ::field_selector::FieldSelector for GoogleDevtoolsRemoteexecutionV1TestTree {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum GoogleDevtoolsRemoteworkersV1Test2AdminTempCommand {
        #[doc = "Restart the bot without downloading a new version. `arg` will be a\nmessage to log."]
        BotRestart,
        #[doc = "Shut down the bot. `arg` will be a task resource name (similar to those\nin tasks.proto) that the bot can use to tell the server that it is\nterminating."]
        BotTerminate,
        #[doc = "Download and run a new version of the bot. `arg` will be a resource\naccessible via `ByteStream.Read` to obtain the new bot code."]
        BotUpdate,
        #[doc = "Restart the host computer. `arg` will be a message to log."]
        HostRestart,
        #[doc = "Illegal value."]
        Unspecified,
    }
    impl GoogleDevtoolsRemoteworkersV1Test2AdminTempCommand {
        pub fn as_str(self) -> &'static str {
            match self {
                GoogleDevtoolsRemoteworkersV1Test2AdminTempCommand::BotRestart => "BOT_RESTART",
                GoogleDevtoolsRemoteworkersV1Test2AdminTempCommand::BotTerminate => "BOT_TERMINATE",
                GoogleDevtoolsRemoteworkersV1Test2AdminTempCommand::BotUpdate => "BOT_UPDATE",
                GoogleDevtoolsRemoteworkersV1Test2AdminTempCommand::HostRestart => "HOST_RESTART",
                GoogleDevtoolsRemoteworkersV1Test2AdminTempCommand::Unspecified => "UNSPECIFIED",
            }
        }
    }
    impl ::std::fmt::Display for GoogleDevtoolsRemoteworkersV1Test2AdminTempCommand {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for GoogleDevtoolsRemoteworkersV1Test2AdminTempCommand {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for GoogleDevtoolsRemoteworkersV1Test2AdminTempCommand {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "BOT_RESTART" => GoogleDevtoolsRemoteworkersV1Test2AdminTempCommand::BotRestart,
                "BOT_TERMINATE" => GoogleDevtoolsRemoteworkersV1Test2AdminTempCommand::BotTerminate,
                "BOT_UPDATE" => GoogleDevtoolsRemoteworkersV1Test2AdminTempCommand::BotUpdate,
                "HOST_RESTART" => GoogleDevtoolsRemoteworkersV1Test2AdminTempCommand::HostRestart,
                "UNSPECIFIED" => GoogleDevtoolsRemoteworkersV1Test2AdminTempCommand::Unspecified,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::field_selector::FieldSelector for GoogleDevtoolsRemoteworkersV1Test2AdminTempCommand {
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
    pub struct GoogleDevtoolsRemoteworkersV1Test2AdminTemp {
        #[doc = "The argument to the admin action; see `Command` for semantics."]
        #[serde(rename = "arg", default)]
        pub arg: ::std::option::Option<String>,
        #[doc = "The admin action; see `Command` for legal values."]
        #[serde(rename = "command", default)]
        pub command: ::std::option::Option<
            crate::schemas::GoogleDevtoolsRemoteworkersV1Test2AdminTempCommand,
        >,
    }
    impl ::field_selector::FieldSelector for GoogleDevtoolsRemoteworkersV1Test2AdminTemp {
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
    pub struct GoogleDevtoolsRemoteworkersV1Test2Blob {
        #[doc = "The contents of the blob."]
        #[serde(rename = "contents", default)]
        pub contents: ::std::option::Option<crate::bytes::Bytes>,
        #[doc = "The digest of the blob. This should be verified by the receiver."]
        #[serde(rename = "digest", default)]
        pub digest: ::std::option::Option<crate::schemas::GoogleDevtoolsRemoteworkersV1Test2Digest>,
    }
    impl ::field_selector::FieldSelector for GoogleDevtoolsRemoteworkersV1Test2Blob {
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
    pub struct GoogleDevtoolsRemoteworkersV1Test2CommandOutputs {
        #[doc = "exit_code is only fully reliable if the status' code is OK. If the task\nexceeded its deadline or was cancelled, the process may still produce an\nexit code as it is cancelled, and this will be populated, but a successful\n(zero) is unlikely to be correct unless the status code is OK."]
        #[serde(rename = "exitCode", default)]
        pub exit_code: ::std::option::Option<i32>,
        #[doc = "The output files. The blob referenced by the digest should contain\none of the following (implementation-dependent):\n\n* A marshalled DirectoryMetadata of the returned filesystem\n* A LUCI-style .isolated file"]
        #[serde(rename = "outputs", default)]
        pub outputs:
            ::std::option::Option<crate::schemas::GoogleDevtoolsRemoteworkersV1Test2Digest>,
    }
    impl ::field_selector::FieldSelector for GoogleDevtoolsRemoteworkersV1Test2CommandOutputs {
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
    pub struct GoogleDevtoolsRemoteworkersV1Test2CommandOverhead {
        #[doc = "The elapsed time between calling Accept and Complete. The server will also\nhave its own idea of what this should be, but this excludes the overhead of\nthe RPCs and the bot response time."]
        #[serde(rename = "duration", default)]
        pub duration: ::std::option::Option<String>,
        #[doc = "The amount of time *not* spent executing the command (ie\nuploading/downloading files)."]
        #[serde(rename = "overhead", default)]
        pub overhead: ::std::option::Option<String>,
    }
    impl ::field_selector::FieldSelector for GoogleDevtoolsRemoteworkersV1Test2CommandOverhead {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct GoogleDevtoolsRemoteworkersV1Test2CommandResult {
        #[doc = "The elapsed time between calling Accept and Complete. The server will also\nhave its own idea of what this should be, but this excludes the overhead of\nthe RPCs and the bot response time."]
        #[serde(rename = "duration", default)]
        pub duration: ::std::option::Option<String>,
        #[doc = "The exit code of the process. An exit code of \"0\" should only be trusted if\n`status` has a code of OK (otherwise it may simply be unset)."]
        #[serde(rename = "exitCode", default)]
        pub exit_code: ::std::option::Option<i32>,
        #[doc = "Implementation-dependent metadata about the task. Both servers and bots\nmay define messages which can be encoded here; bots are free to provide\nmetadata in multiple formats, and servers are free to choose one or more\nof the values to process and ignore others. In particular, it is *not*\nconsidered an error for the bot to provide the server with a field that it\ndoesn't know about."]
        #[serde(rename = "metadata", default)]
        pub metadata:
            ::std::option::Option<Vec<::std::collections::BTreeMap<String, ::serde_json::Value>>>,
        #[doc = "The output files. The blob referenced by the digest should contain\none of the following (implementation-dependent):\n\n* A marshalled DirectoryMetadata of the returned filesystem\n* A LUCI-style .isolated file"]
        #[serde(rename = "outputs", default)]
        pub outputs:
            ::std::option::Option<crate::schemas::GoogleDevtoolsRemoteworkersV1Test2Digest>,
        #[doc = "The amount of time *not* spent executing the command (ie\nuploading/downloading files)."]
        #[serde(rename = "overhead", default)]
        pub overhead: ::std::option::Option<String>,
        #[doc = "An overall status for the command. For example, if the command timed out,\nthis might have a code of DEADLINE_EXCEEDED; if it was killed by the OS for\nmemory exhaustion, it might have a code of RESOURCE_EXHAUSTED."]
        #[serde(rename = "status", default)]
        pub status: ::std::option::Option<crate::schemas::GoogleRpcStatus>,
    }
    impl ::field_selector::FieldSelector for GoogleDevtoolsRemoteworkersV1Test2CommandResult {
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
    pub struct GoogleDevtoolsRemoteworkersV1Test2CommandTask {
        #[doc = "The expected outputs from the task."]
        #[serde(rename = "expectedOutputs", default)]
        pub expected_outputs: ::std::option::Option<
            crate::schemas::GoogleDevtoolsRemoteworkersV1Test2CommandTaskOutputs,
        >,
        #[doc = "The inputs to the task."]
        #[serde(rename = "inputs", default)]
        pub inputs: ::std::option::Option<
            crate::schemas::GoogleDevtoolsRemoteworkersV1Test2CommandTaskInputs,
        >,
        #[doc = "The timeouts of this task."]
        #[serde(rename = "timeouts", default)]
        pub timeouts: ::std::option::Option<
            crate::schemas::GoogleDevtoolsRemoteworkersV1Test2CommandTaskTimeouts,
        >,
    }
    impl ::field_selector::FieldSelector for GoogleDevtoolsRemoteworkersV1Test2CommandTask {
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
    pub struct GoogleDevtoolsRemoteworkersV1Test2CommandTaskInputs { # [ doc = "The command itself to run (e.g., argv).\n\nThis field should be passed directly to the underlying operating system,\nand so it must be sensible to that operating system. For example, on\nWindows, the first argument might be \"C:\\Windows\\System32\\ping.exe\" -\nthat is, using drive letters and backslashes. A command for a *nix\nsystem, on the other hand, would use forward slashes.\n\nAll other fields in the RWAPI must consistently use forward slashes,\nsince those fields may be interpretted by both the service and the bot." ] # [ serde ( rename = "arguments" , default ) ] pub arguments : :: std :: option :: Option < Vec < String > > , # [ doc = "All environment variables required by the task." ] # [ serde ( rename = "environmentVariables" , default ) ] pub environment_variables : :: std :: option :: Option < Vec < crate :: schemas :: GoogleDevtoolsRemoteworkersV1Test2CommandTaskInputsEnvironmentVariable > > , # [ doc = "The input filesystem to be set up prior to the task beginning. The\ncontents should be a repeated set of FileMetadata messages though other\nformats are allowed if better for the implementation (eg, a LUCI-style\n.isolated file).\n\nThis field is repeated since implementations might want to cache the\nmetadata, in which case it may be useful to break up portions of the\nfilesystem that change frequently (eg, specific input files) from those\nthat don't (eg, standard header files)." ] # [ serde ( rename = "files" , default ) ] pub files : :: std :: option :: Option < Vec < crate :: schemas :: GoogleDevtoolsRemoteworkersV1Test2Digest > > , # [ doc = "Inline contents for blobs expected to be needed by the bot to execute the\ntask. For example, contents of entries in `files` or blobs that are\nindirectly referenced by an entry there.\n\nThe bot should check against this list before downloading required task\ninputs to reduce the number of communications between itself and the\nremote CAS server." ] # [ serde ( rename = "inlineBlobs" , default ) ] pub inline_blobs : :: std :: option :: Option < Vec < crate :: schemas :: GoogleDevtoolsRemoteworkersV1Test2Blob > > , # [ doc = "Directory from which a command is executed. It is a relative directory\nwith respect to the bot's working directory (i.e., \"./\"). If it is\nnon-empty, then it must exist under \"./\". Otherwise, \"./\" will be used." ] # [ serde ( rename = "workingDirectory" , default ) ] pub working_directory : :: std :: option :: Option < String > , }
    impl ::field_selector::FieldSelector for GoogleDevtoolsRemoteworkersV1Test2CommandTaskInputs {
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
    pub struct GoogleDevtoolsRemoteworkersV1Test2CommandTaskInputsEnvironmentVariable {
        #[doc = "The envvar name."]
        #[serde(rename = "name", default)]
        pub name: ::std::option::Option<String>,
        #[doc = "The envvar value."]
        #[serde(rename = "value", default)]
        pub value: ::std::option::Option<String>,
    }
    impl ::field_selector::FieldSelector
        for GoogleDevtoolsRemoteworkersV1Test2CommandTaskInputsEnvironmentVariable
    {
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
    pub struct GoogleDevtoolsRemoteworkersV1Test2CommandTaskOutputs {
        #[doc = "A list of expected directories, relative to the execution root. All paths\nMUST be delimited by forward slashes."]
        #[serde(rename = "directories", default)]
        pub directories: ::std::option::Option<Vec<String>>,
        #[doc = "A list of expected files, relative to the execution root. All paths\nMUST be delimited by forward slashes."]
        #[serde(rename = "files", default)]
        pub files: ::std::option::Option<Vec<String>>,
        #[doc = "The destination to which any stderr should be sent. The method by which\nthe bot should send the stream contents to that destination is not\ndefined in this API. As examples, the destination could be a file\nreferenced in the `files` field in this message, or it could be a URI\nthat must be written via the ByteStream API."]
        #[serde(rename = "stderrDestination", default)]
        pub stderr_destination: ::std::option::Option<String>,
        #[doc = "The destination to which any stdout should be sent. The method by which\nthe bot should send the stream contents to that destination is not\ndefined in this API. As examples, the destination could be a file\nreferenced in the `files` field in this message, or it could be a URI\nthat must be written via the ByteStream API."]
        #[serde(rename = "stdoutDestination", default)]
        pub stdout_destination: ::std::option::Option<String>,
    }
    impl ::field_selector::FieldSelector for GoogleDevtoolsRemoteworkersV1Test2CommandTaskOutputs {
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
    pub struct GoogleDevtoolsRemoteworkersV1Test2CommandTaskTimeouts {
        #[doc = "This specifies the maximum time that the task can run, excluding the\ntime required to download inputs or upload outputs. That is, the worker\nwill terminate the task if it runs longer than this."]
        #[serde(rename = "execution", default)]
        pub execution: ::std::option::Option<String>,
        #[doc = "This specifies the maximum amount of time the task can be idle - that is,\ngo without generating some output in either stdout or stderr. If the\nprocess is silent for more than the specified time, the worker will\nterminate the task."]
        #[serde(rename = "idle", default)]
        pub idle: ::std::option::Option<String>,
        #[doc = "If the execution or IO timeouts are exceeded, the worker will try to\ngracefully terminate the task and return any existing logs. However,\ntasks may be hard-frozen in which case this process will fail. This\ntimeout specifies how long to wait for a terminated task to shut down\ngracefully (e.g. via SIGTERM) before we bring down the hammer (e.g.\nSIGKILL on *nix, CTRL_BREAK_EVENT on Windows)."]
        #[serde(rename = "shutdown", default)]
        pub shutdown: ::std::option::Option<String>,
    }
    impl ::field_selector::FieldSelector for GoogleDevtoolsRemoteworkersV1Test2CommandTaskTimeouts {
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
    pub struct GoogleDevtoolsRemoteworkersV1Test2Digest {
        #[doc = "A string-encoded hash (eg \"1a2b3c\", not the byte array [0x1a, 0x2b, 0x3c])\nusing an implementation-defined hash algorithm (eg SHA-256)."]
        #[serde(rename = "hash", default)]
        pub hash: ::std::option::Option<String>,
        #[doc = "The size of the contents. While this is not strictly required as part of an\nidentifier (after all, any given hash will have exactly one canonical\nsize), it's useful in almost all cases when one might want to send or\nretrieve blobs of content and is included here for this reason."]
        #[serde(rename = "sizeBytes", default)]
        #[serde(with = "crate::parsed_string")]
        pub size_bytes: ::std::option::Option<i64>,
    }
    impl ::field_selector::FieldSelector for GoogleDevtoolsRemoteworkersV1Test2Digest {
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
    pub struct GoogleDevtoolsRemoteworkersV1Test2Directory {
        #[doc = "Any subdirectories"]
        #[serde(rename = "directories", default)]
        pub directories: ::std::option::Option<
            Vec<crate::schemas::GoogleDevtoolsRemoteworkersV1Test2DirectoryMetadata>,
        >,
        #[doc = "The files in this directory"]
        #[serde(rename = "files", default)]
        pub files: ::std::option::Option<
            Vec<crate::schemas::GoogleDevtoolsRemoteworkersV1Test2FileMetadata>,
        >,
    }
    impl ::field_selector::FieldSelector for GoogleDevtoolsRemoteworkersV1Test2Directory {
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
    pub struct GoogleDevtoolsRemoteworkersV1Test2DirectoryMetadata {
        #[doc = "A pointer to the contents of the directory, in the form of a marshalled\nDirectory message."]
        #[serde(rename = "digest", default)]
        pub digest: ::std::option::Option<crate::schemas::GoogleDevtoolsRemoteworkersV1Test2Digest>,
        #[doc = "The path of the directory, as in FileMetadata.path."]
        #[serde(rename = "path", default)]
        pub path: ::std::option::Option<String>,
    }
    impl ::field_selector::FieldSelector for GoogleDevtoolsRemoteworkersV1Test2DirectoryMetadata {
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
    pub struct GoogleDevtoolsRemoteworkersV1Test2FileMetadata {
        #[doc = "If the file is small enough, its contents may also or alternatively be\nlisted here."]
        #[serde(rename = "contents", default)]
        pub contents: ::std::option::Option<crate::bytes::Bytes>,
        #[doc = "A pointer to the contents of the file. The method by which a client\nretrieves the contents from a CAS system is not defined here."]
        #[serde(rename = "digest", default)]
        pub digest: ::std::option::Option<crate::schemas::GoogleDevtoolsRemoteworkersV1Test2Digest>,
        #[doc = "Properties of the file"]
        #[serde(rename = "isExecutable", default)]
        pub is_executable: ::std::option::Option<bool>,
        #[doc = "The path of this file. If this message is part of the\nCommandOutputs.outputs fields, the path is relative to the execution root\nand must correspond to an entry in CommandTask.outputs.files. If this\nmessage is part of a Directory message, then the path is relative to the\nroot of that directory. All paths MUST be delimited by forward slashes."]
        #[serde(rename = "path", default)]
        pub path: ::std::option::Option<String>,
    }
    impl ::field_selector::FieldSelector for GoogleDevtoolsRemoteworkersV1Test2FileMetadata {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct GoogleLongrunningOperation {
        #[doc = "If the value is `false`, it means the operation is still in progress.\nIf `true`, the operation is completed, and either `error` or `response` is\navailable."]
        #[serde(rename = "done", default)]
        pub done: ::std::option::Option<bool>,
        #[doc = "The error result of the operation in case of failure or cancellation."]
        #[serde(rename = "error", default)]
        pub error: ::std::option::Option<crate::schemas::GoogleRpcStatus>,
        #[doc = "Service-specific metadata associated with the operation.  It typically\ncontains progress information and common metadata such as create time.\nSome services might not provide such metadata.  Any method that returns a\nlong-running operation should document the metadata type, if any."]
        #[serde(rename = "metadata", default)]
        pub metadata:
            ::std::option::Option<::std::collections::BTreeMap<String, ::serde_json::Value>>,
        #[doc = "The server-assigned name, which is only unique within the same service that\noriginally returns it. If you use the default HTTP mapping, the\n`name` should be a resource name ending with `operations/{unique_id}`."]
        #[serde(rename = "name", default)]
        pub name: ::std::option::Option<String>,
        #[doc = "The normal response of the operation in case of success.  If the original\nmethod returns no data on success, such as `Delete`, the response is\n`google.protobuf.Empty`.  If the original method is standard\n`Get`/`Create`/`Update`, the response should be the resource.  For other\nmethods, the response should have the type `XxxResponse`, where `Xxx`\nis the original method name.  For example, if the original method name\nis `TakeSnapshot()`, the inferred response type is\n`TakeSnapshotResponse`."]
        #[serde(rename = "response", default)]
        pub response:
            ::std::option::Option<::std::collections::BTreeMap<String, ::serde_json::Value>>,
    }
    impl ::field_selector::FieldSelector for GoogleLongrunningOperation {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(Debug, Clone, PartialEq, Default, :: serde :: Deserialize, :: serde :: Serialize)]
    pub struct GoogleRpcStatus {
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
    impl ::field_selector::FieldSelector for GoogleRpcStatus {
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
            #[doc = "Actions that can be performed on the instances resource"]
            pub fn instances(&self) -> crate::resources::projects::instances::InstancesActions<A> {
                crate::resources::projects::instances::InstancesActions {
                    reqwest: &self.reqwest,
                    auth: &self.auth,
                }
            }
            #[doc = "Actions that can be performed on the operations resource"]
            pub fn operations(
                &self,
            ) -> crate::resources::projects::operations::OperationsActions<A> {
                crate::resources::projects::operations::OperationsActions {
                    reqwest: &self.reqwest,
                    auth: &self.auth,
                }
            }
        }
        pub mod instances {
            pub mod params {}
            pub struct InstancesActions<'a, A> {
                pub(crate) reqwest: &'a reqwest::Client,
                pub(crate) auth: &'a std::sync::Mutex<A>,
            }
            impl<'a, A: yup_oauth2::GetToken> InstancesActions<'a, A> {
                #[doc = "Creates a new instance in the specified region.\nReturns a long running operation which contains an instance on completion.\nWhile the long running operation is in progress, any call to `GetInstance`\nreturns an instance in state `CREATING`."]
                pub fn create(
                    &self,
                    request : crate :: schemas :: GoogleDevtoolsRemotebuildexecutionAdminV1AlphaCreateInstanceRequest,
                    parent: impl Into<String>,
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
                        parent: parent.into(),
                    }
                }
                #[doc = "Deletes the specified instance.\nReturns a long running operation which contains a `google.protobuf.Empty`\nresponse on completion.\nDeleting an instance with worker pools in it will delete these worker\npools."]
                pub fn delete(&self, name: impl Into<String>) -> DeleteRequestBuilder<A> {
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
                        name: name.into(),
                    }
                }
                #[doc = "Returns the specified instance."]
                pub fn get(&self, name: impl Into<String>) -> GetRequestBuilder<A> {
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
                        name: name.into(),
                    }
                }
                #[doc = "Lists instances in a project."]
                pub fn list(&self, parent: impl Into<String>) -> ListRequestBuilder<A> {
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
                        parent: parent.into(),
                    }
                }
                #[doc = "Actions that can be performed on the workerpools resource"]
                pub fn workerpools(
                    &self,
                ) -> crate::resources::projects::instances::workerpools::WorkerpoolsActions<A>
                {
                    crate::resources::projects::instances::workerpools::WorkerpoolsActions {
                        reqwest: &self.reqwest,
                        auth: &self.auth,
                    }
                }
            }
            #[derive(Debug, Clone)]
            pub struct CreateRequestBuilder < 'a , A > { pub ( crate ) reqwest : & 'a :: reqwest :: Client , pub ( crate ) auth : & 'a :: std :: sync :: Mutex < A > , request : crate :: schemas :: GoogleDevtoolsRemotebuildexecutionAdminV1AlphaCreateInstanceRequest , parent : String , access_token : Option < String > , alt : Option < crate :: params :: Alt > , callback : Option < String > , fields : Option < String > , key : Option < String > , oauth_token : Option < String > , pretty_print : Option < bool > , quota_user : Option < String > , upload_protocol : Option < String > , upload_type : Option < String > , xgafv : Option < crate :: params :: Xgafv > , }
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
                ) -> Result<crate::schemas::GoogleLongrunningOperation, Box<dyn ::std::error::Error>>
                {
                    self.execute_fields::<_, &str>(None)
                }
                #[doc = r" Execute the given operation. This will provide a `fields`"]
                #[doc = r" selector of `*`. This will include every attribute of the"]
                #[doc = r" response resource and should be limited to use during"]
                #[doc = r" development or debugging."]
                pub fn execute_debug(
                    self,
                ) -> Result<crate::schemas::GoogleLongrunningOperation, Box<dyn ::std::error::Error>>
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
                        "https://admin-remotebuildexecution.googleapis.com/".to_owned();
                    output.push_str("v1alpha/");
                    {
                        let var_as_str = &self.parent;
                        output.extend(::percent_encoding::utf8_percent_encode(
                            &var_as_str,
                            crate::RESERVED,
                        ));
                    }
                    output.push_str("/instances");
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
            pub struct DeleteRequestBuilder<'a, A> {
                pub(crate) reqwest: &'a ::reqwest::Client,
                pub(crate) auth: &'a ::std::sync::Mutex<A>,
                name: String,
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
                ) -> Result<crate::schemas::GoogleLongrunningOperation, Box<dyn ::std::error::Error>>
                {
                    self.execute_fields::<_, &str>(None)
                }
                #[doc = r" Execute the given operation. This will provide a `fields`"]
                #[doc = r" selector of `*`. This will include every attribute of the"]
                #[doc = r" response resource and should be limited to use during"]
                #[doc = r" development or debugging."]
                pub fn execute_debug(
                    self,
                ) -> Result<crate::schemas::GoogleLongrunningOperation, Box<dyn ::std::error::Error>>
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
                        "https://admin-remotebuildexecution.googleapis.com/".to_owned();
                    output.push_str("v1alpha/");
                    {
                        let var_as_str = &self.name;
                        output.extend(::percent_encoding::utf8_percent_encode(
                            &var_as_str,
                            crate::RESERVED,
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
                name: String,
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
                ) -> Result<
                    crate::schemas::GoogleDevtoolsRemotebuildexecutionAdminV1AlphaInstance,
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
                    crate::schemas::GoogleDevtoolsRemotebuildexecutionAdminV1AlphaInstance,
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
                        "https://admin-remotebuildexecution.googleapis.com/".to_owned();
                    output.push_str("v1alpha/");
                    {
                        let var_as_str = &self.name;
                        output.extend(::percent_encoding::utf8_percent_encode(
                            &var_as_str,
                            crate::RESERVED,
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
                    let fut = auth.token(vec!["https://www.googleapis.com/auth/cloud-platform"]);
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
                parent: String,
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
                #[doc = r" the response resource."]pub fn execute_standard ( self ) -> Result < crate :: schemas :: GoogleDevtoolsRemotebuildexecutionAdminV1AlphaListInstancesResponse , Box < dyn :: std :: error :: Error >>{
                    self.execute_fields::<_, &str>(None)
                }
                #[doc = r" Execute the given operation. This will provide a `fields`"]
                #[doc = r" selector of `*`. This will include every attribute of the"]
                #[doc = r" response resource and should be limited to use during"]
                #[doc = r" development or debugging."]pub fn execute_debug ( self ) -> Result < crate :: schemas :: GoogleDevtoolsRemotebuildexecutionAdminV1AlphaListInstancesResponse , Box < dyn :: std :: error :: Error >>{
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
                        "https://admin-remotebuildexecution.googleapis.com/".to_owned();
                    output.push_str("v1alpha/");
                    {
                        let var_as_str = &self.parent;
                        output.extend(::percent_encoding::utf8_percent_encode(
                            &var_as_str,
                            crate::RESERVED,
                        ));
                    }
                    output.push_str("/instances");
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
                    let fut = auth.token(vec!["https://www.googleapis.com/auth/cloud-platform"]);
                    let mut runtime = ::tokio::runtime::Runtime::new().unwrap();
                    let token = runtime.block_on(fut).unwrap().access_token;
                    let req = req.bearer_auth(&token);
                    req
                }
            }
            pub mod workerpools {
                pub mod params {}
                pub struct WorkerpoolsActions<'a, A> {
                    pub(crate) reqwest: &'a reqwest::Client,
                    pub(crate) auth: &'a std::sync::Mutex<A>,
                }
                impl<'a, A: yup_oauth2::GetToken> WorkerpoolsActions<'a, A> {
                    #[doc = "Creates a new worker pool with a specified size and configuration.\nReturns a long running operation which contains a worker pool on\ncompletion. While the long running operation is in progress, any call to\n`GetWorkerPool` returns a worker pool in state `CREATING`."]
                    pub fn create(
                        &self,
                        request : crate :: schemas :: GoogleDevtoolsRemotebuildexecutionAdminV1AlphaCreateWorkerPoolRequest,
                        parent: impl Into<String>,
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
                            parent: parent.into(),
                        }
                    }
                    #[doc = "Deletes the specified worker pool.\nReturns a long running operation, which contains a `google.protobuf.Empty`\nresponse on completion.\nWhile the long running operation is in progress, any call to\n`GetWorkerPool` returns a worker pool in state `DELETING`."]
                    pub fn delete(&self, name: impl Into<String>) -> DeleteRequestBuilder<A> {
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
                            name: name.into(),
                        }
                    }
                    #[doc = "Returns the specified worker pool."]
                    pub fn get(&self, name: impl Into<String>) -> GetRequestBuilder<A> {
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
                            name: name.into(),
                        }
                    }
                    #[doc = "Lists worker pools in an instance."]
                    pub fn list(&self, parent: impl Into<String>) -> ListRequestBuilder<A> {
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
                            parent: parent.into(),
                            filter: None,
                        }
                    }
                    #[doc = "Updates an existing worker pool with a specified size and/or configuration.\nReturns a long running operation, which contains a worker pool on\ncompletion. While the long running operation is in progress, any call to\n`GetWorkerPool` returns a worker pool in state `UPDATING`."]
                    pub fn patch(
                        &self,
                        request : crate :: schemas :: GoogleDevtoolsRemotebuildexecutionAdminV1AlphaUpdateWorkerPoolRequest,
                        name: impl Into<String>,
                    ) -> PatchRequestBuilder<A> {
                        PatchRequestBuilder {
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
                            name: name.into(),
                        }
                    }
                }
                #[derive(Debug, Clone)]
                pub struct CreateRequestBuilder < 'a , A > { pub ( crate ) reqwest : & 'a :: reqwest :: Client , pub ( crate ) auth : & 'a :: std :: sync :: Mutex < A > , request : crate :: schemas :: GoogleDevtoolsRemotebuildexecutionAdminV1AlphaCreateWorkerPoolRequest , parent : String , access_token : Option < String > , alt : Option < crate :: params :: Alt > , callback : Option < String > , fields : Option < String > , key : Option < String > , oauth_token : Option < String > , pretty_print : Option < bool > , quota_user : Option < String > , upload_protocol : Option < String > , upload_type : Option < String > , xgafv : Option < crate :: params :: Xgafv > , }
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
                    ) -> Result<
                        crate::schemas::GoogleLongrunningOperation,
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
                        crate::schemas::GoogleLongrunningOperation,
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
                        let mut output =
                            "https://admin-remotebuildexecution.googleapis.com/".to_owned();
                        output.push_str("v1alpha/");
                        {
                            let var_as_str = &self.parent;
                            output.extend(::percent_encoding::utf8_percent_encode(
                                &var_as_str,
                                crate::RESERVED,
                            ));
                        }
                        output.push_str("/workerpools");
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
                pub struct DeleteRequestBuilder<'a, A> {
                    pub(crate) reqwest: &'a ::reqwest::Client,
                    pub(crate) auth: &'a ::std::sync::Mutex<A>,
                    name: String,
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
                    ) -> Result<
                        crate::schemas::GoogleLongrunningOperation,
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
                        crate::schemas::GoogleLongrunningOperation,
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
                            "https://admin-remotebuildexecution.googleapis.com/".to_owned();
                        output.push_str("v1alpha/");
                        {
                            let var_as_str = &self.name;
                            output.extend(::percent_encoding::utf8_percent_encode(
                                &var_as_str,
                                crate::RESERVED,
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
                    name: String,
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
                    ) -> Result<
                        crate::schemas::GoogleDevtoolsRemotebuildexecutionAdminV1AlphaWorkerPool,
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
                        crate::schemas::GoogleDevtoolsRemotebuildexecutionAdminV1AlphaWorkerPool,
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
                            "https://admin-remotebuildexecution.googleapis.com/".to_owned();
                        output.push_str("v1alpha/");
                        {
                            let var_as_str = &self.name;
                            output.extend(::percent_encoding::utf8_percent_encode(
                                &var_as_str,
                                crate::RESERVED,
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
                            auth.token(vec!["https://www.googleapis.com/auth/cloud-platform"]);
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
                    parent: String,
                    filter: Option<String>,
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
                    #[doc = "Optional. A filter expression that filters resources listed in\nthe response. The expression must specify the field name, a comparison\noperator, and the value that you want to use for filtering. The value\nmust be a string, a number, or a boolean. String values are\ncase-insensitive.\nThe comparison operator must be either `:`, `=`, `!=`, `>`, `>=`, `<=` or\n`<`.\nThe `:` operator can be used with string fields to match substrings.\nFor non-string fields it is equivalent to the `=` operator.\nThe `:*` comparison can be used to test  whether a key has been defined.\n\nYou can also filter on nested fields.\n\nTo filter on multiple expressions, you can separate expression using\n`AND` and `OR` operators, using parentheses to specify precedence. If\nneither operator is specified, `AND` is assumed.\n\nExamples:\n\nInclude only pools with more than 100 reserved workers:\n`(worker_count > 100) (worker_config.reserved = true)`\n\nInclude only pools with a certain label or machines of the n1-standard\nfamily:\n`worker_config.labels.key1 : * OR worker_config.machine_type: n1-standard`"]
                    pub fn filter(mut self, value: impl Into<String>) -> Self {
                        self.filter = Some(value.into());
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
                    #[doc = r" the response resource."]pub fn execute_standard ( self ) -> Result < crate :: schemas :: GoogleDevtoolsRemotebuildexecutionAdminV1AlphaListWorkerPoolsResponse , Box < dyn :: std :: error :: Error >>{
                        self.execute_fields::<_, &str>(None)
                    }
                    #[doc = r" Execute the given operation. This will provide a `fields`"]
                    #[doc = r" selector of `*`. This will include every attribute of the"]
                    #[doc = r" response resource and should be limited to use during"]
                    #[doc = r" development or debugging."]pub fn execute_debug ( self ) -> Result < crate :: schemas :: GoogleDevtoolsRemotebuildexecutionAdminV1AlphaListWorkerPoolsResponse , Box < dyn :: std :: error :: Error >>{
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
                            "https://admin-remotebuildexecution.googleapis.com/".to_owned();
                        output.push_str("v1alpha/");
                        {
                            let var_as_str = &self.parent;
                            output.extend(::percent_encoding::utf8_percent_encode(
                                &var_as_str,
                                crate::RESERVED,
                            ));
                        }
                        output.push_str("/workerpools");
                        output
                    }
                    fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
                        let req = self.reqwest.request(::reqwest::Method::GET, path);
                        let req = req.query(&[("filter", &self.filter)]);
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
                pub struct PatchRequestBuilder < 'a , A > { pub ( crate ) reqwest : & 'a :: reqwest :: Client , pub ( crate ) auth : & 'a :: std :: sync :: Mutex < A > , request : crate :: schemas :: GoogleDevtoolsRemotebuildexecutionAdminV1AlphaUpdateWorkerPoolRequest , name : String , access_token : Option < String > , alt : Option < crate :: params :: Alt > , callback : Option < String > , fields : Option < String > , key : Option < String > , oauth_token : Option < String > , pretty_print : Option < bool > , quota_user : Option < String > , upload_protocol : Option < String > , upload_type : Option < String > , xgafv : Option < crate :: params :: Xgafv > , }
                impl<'a, A: yup_oauth2::GetToken> PatchRequestBuilder<'a, A> {
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
                        crate::schemas::GoogleLongrunningOperation,
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
                        crate::schemas::GoogleLongrunningOperation,
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
                        let mut output =
                            "https://admin-remotebuildexecution.googleapis.com/".to_owned();
                        output.push_str("v1alpha/");
                        {
                            let var_as_str = &self.name;
                            output.extend(::percent_encoding::utf8_percent_encode(
                                &var_as_str,
                                crate::RESERVED,
                            ));
                        }
                        output
                    }
                    fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
                        let req = self.reqwest.request(::reqwest::Method::PATCH, path);
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
        pub mod operations {
            pub mod params {}
            pub struct OperationsActions<'a, A> {
                pub(crate) reqwest: &'a reqwest::Client,
                pub(crate) auth: &'a std::sync::Mutex<A>,
            }
            impl<'a, A: yup_oauth2::GetToken> OperationsActions<'a, A> {
                #[doc = "Gets the latest state of a long-running operation.  Clients can use this\nmethod to poll the operation result at intervals as recommended by the API\nservice."]
                pub fn get(&self, name: impl Into<String>) -> GetRequestBuilder<A> {
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
                        name: name.into(),
                    }
                }
            }
            #[derive(Debug, Clone)]
            pub struct GetRequestBuilder<'a, A> {
                pub(crate) reqwest: &'a ::reqwest::Client,
                pub(crate) auth: &'a ::std::sync::Mutex<A>,
                name: String,
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
                ) -> Result<crate::schemas::GoogleLongrunningOperation, Box<dyn ::std::error::Error>>
                {
                    self.execute_fields::<_, &str>(None)
                }
                #[doc = r" Execute the given operation. This will provide a `fields`"]
                #[doc = r" selector of `*`. This will include every attribute of the"]
                #[doc = r" response resource and should be limited to use during"]
                #[doc = r" development or debugging."]
                pub fn execute_debug(
                    self,
                ) -> Result<crate::schemas::GoogleLongrunningOperation, Box<dyn ::std::error::Error>>
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
                        "https://admin-remotebuildexecution.googleapis.com/".to_owned();
                    output.push_str("v1alpha/");
                    {
                        let var_as_str = &self.name;
                        output.extend(::percent_encoding::utf8_percent_encode(
                            &var_as_str,
                            crate::RESERVED,
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
