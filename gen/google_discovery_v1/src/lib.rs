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
    pub struct DirectoryListItemsItemsIcons {
        #[doc = "The URL of the 16x16 icon."]
        #[serde(rename = "x16", default)]
        pub x_16: ::std::option::Option<String>,
        #[doc = "The URL of the 32x32 icon."]
        #[serde(rename = "x32", default)]
        pub x_32: ::std::option::Option<String>,
    }
    impl ::field_selector::FieldSelector for DirectoryListItemsItemsIcons {
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
    pub struct DirectoryListItemsItems {
        #[doc = "The description of this API."]
        #[serde(rename = "description", default)]
        pub description: ::std::option::Option<String>,
        #[doc = "A link to the discovery document."]
        #[serde(rename = "discoveryLink", default)]
        pub discovery_link: ::std::option::Option<String>,
        #[doc = "The URL for the discovery REST document."]
        #[serde(rename = "discoveryRestUrl", default)]
        pub discovery_rest_url: ::std::option::Option<String>,
        #[doc = "A link to human readable documentation for the API."]
        #[serde(rename = "documentationLink", default)]
        pub documentation_link: ::std::option::Option<String>,
        #[doc = "Links to 16x16 and 32x32 icons representing the API."]
        #[serde(rename = "icons", default)]
        pub icons: ::std::option::Option<crate::schemas::DirectoryListItemsItemsIcons>,
        #[doc = "The id of this API."]
        #[serde(rename = "id", default)]
        pub id: ::std::option::Option<String>,
        #[doc = "The kind for this response."]
        #[serde(rename = "kind", default)]
        pub kind: ::std::option::Option<String>,
        #[doc = "Labels for the status of this API, such as labs or deprecated."]
        #[serde(rename = "labels", default)]
        pub labels: ::std::option::Option<Vec<String>>,
        #[doc = "The name of the API."]
        #[serde(rename = "name", default)]
        pub name: ::std::option::Option<String>,
        #[doc = "True if this version is the preferred version to use."]
        #[serde(rename = "preferred", default)]
        pub preferred: ::std::option::Option<bool>,
        #[doc = "The title of this API."]
        #[serde(rename = "title", default)]
        pub title: ::std::option::Option<String>,
        #[doc = "The version of the API."]
        #[serde(rename = "version", default)]
        pub version: ::std::option::Option<String>,
    }
    impl ::field_selector::FieldSelector for DirectoryListItemsItems {
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
    pub struct DirectoryList {
        #[doc = "Indicate the version of the Discovery API used to generate this doc."]
        #[serde(rename = "discoveryVersion", default)]
        pub discovery_version: ::std::option::Option<String>,
        #[doc = "The individual directory entries. One entry per api/version pair."]
        #[serde(rename = "items", default)]
        pub items: ::std::option::Option<Vec<crate::schemas::DirectoryListItemsItems>>,
        #[doc = "The kind for this response."]
        #[serde(rename = "kind", default)]
        pub kind: ::std::option::Option<String>,
    }
    impl ::field_selector::FieldSelector for DirectoryList {
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
    pub struct JsonSchemaAnnotations {
        #[doc = "A list of methods for which this property is required on requests."]
        #[serde(rename = "required", default)]
        pub required: ::std::option::Option<Vec<String>>,
    }
    impl ::field_selector::FieldSelector for JsonSchemaAnnotations {
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
    pub struct JsonSchemaVariantMapItems {
        #[serde(rename = "$ref", default)]
        pub r#ref: ::std::option::Option<String>,
        #[serde(rename = "type_value", default)]
        pub type_value: ::std::option::Option<String>,
    }
    impl ::field_selector::FieldSelector for JsonSchemaVariantMapItems {
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
    pub struct JsonSchemaVariant {
        #[doc = "The name of the type discriminant property."]
        #[serde(rename = "discriminant", default)]
        pub discriminant: ::std::option::Option<String>,
        #[doc = "The map of discriminant value to schema to use for parsing.."]
        #[serde(rename = "map", default)]
        pub map: ::std::option::Option<Vec<crate::schemas::JsonSchemaVariantMapItems>>,
    }
    impl ::field_selector::FieldSelector for JsonSchemaVariant {
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
    pub struct JsonSchema {
        #[doc = "If this is a schema for an object, this property is the schema for any additional properties with dynamic keys on this object."]
        #[serde(rename = "additionalProperties", default)]
        pub additional_properties: ::std::option::Option<Box<crate::schemas::JsonSchema>>,
        #[doc = "Additional information about this property."]
        #[serde(rename = "annotations", default)]
        pub annotations: ::std::option::Option<crate::schemas::JsonSchemaAnnotations>,
        #[doc = "The default value of this property (if one exists)."]
        #[serde(rename = "default", default)]
        pub default: ::std::option::Option<String>,
        #[doc = "A description of this object."]
        #[serde(rename = "description", default)]
        pub description: ::std::option::Option<String>,
        #[doc = "The descriptions for the enums. Each position maps to the corresponding value in the \"enum\" array."]
        #[serde(rename = "enumDescriptions", default)]
        pub enum_descriptions: ::std::option::Option<Vec<String>>,
        #[doc = "An additional regular expression or key that helps constrain the value. For more details see: http://tools.ietf.org/html/draft-zyp-json-schema-03#section-5.23"]
        #[serde(rename = "format", default)]
        pub format: ::std::option::Option<String>,
        #[doc = "Unique identifier for this schema."]
        #[serde(rename = "id", default)]
        pub id: ::std::option::Option<String>,
        #[doc = "If this is a schema for an array, this property is the schema for each element in the array."]
        #[serde(rename = "items", default)]
        pub items: ::std::option::Option<Box<crate::schemas::JsonSchema>>,
        #[doc = "Whether this parameter goes in the query or the path for REST requests."]
        #[serde(rename = "location", default)]
        pub location: ::std::option::Option<String>,
        #[doc = "The maximum value of this parameter."]
        #[serde(rename = "maximum", default)]
        pub maximum: ::std::option::Option<String>,
        #[doc = "The minimum value of this parameter."]
        #[serde(rename = "minimum", default)]
        pub minimum: ::std::option::Option<String>,
        #[doc = "The regular expression this parameter must conform to. Uses Java 6 regex format: http://docs.oracle.com/javase/6/docs/api/java/util/regex/Pattern.html"]
        #[serde(rename = "pattern", default)]
        pub pattern: ::std::option::Option<String>,
        #[doc = "If this is a schema for an object, list the schema for each property of this object."]
        #[serde(rename = "properties", default)]
        pub properties:
            ::std::option::Option<::std::collections::BTreeMap<String, crate::schemas::JsonSchema>>,
        #[doc = "Values this parameter may take (if it is an enum)."]
        #[serde(rename = "enum", default)]
        pub r#enum: ::std::option::Option<Vec<String>>,
        #[doc = "A reference to another schema. The value of this property is the \"id\" of another schema."]
        #[serde(rename = "$ref", default)]
        pub r#ref: ::std::option::Option<String>,
        #[doc = "The value type for this schema. A list of values can be found here: http://tools.ietf.org/html/draft-zyp-json-schema-03#section-5.1"]
        #[serde(rename = "type", default)]
        pub r#type: ::std::option::Option<String>,
        #[doc = "The value is read-only, generated by the service. The value cannot be modified by the client. If the value is included in a POST, PUT, or PATCH request, it is ignored by the service."]
        #[serde(rename = "readOnly", default)]
        pub read_only: ::std::option::Option<bool>,
        #[doc = "Whether this parameter may appear multiple times."]
        #[serde(rename = "repeated", default)]
        pub repeated: ::std::option::Option<bool>,
        #[doc = "Whether the parameter is required."]
        #[serde(rename = "required", default)]
        pub required: ::std::option::Option<bool>,
        #[doc = "In a variant data type, the value of one property is used to determine how to interpret the entire entity. Its value must exist in a map of descriminant values to schema names."]
        #[serde(rename = "variant", default)]
        pub variant: ::std::option::Option<crate::schemas::JsonSchemaVariant>,
    }
    impl ::field_selector::FieldSelector for JsonSchema {
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
    pub struct RestDescriptionAuthOauth2ScopesAdditionalProperties {
        #[doc = "Description of scope."]
        #[serde(rename = "description", default)]
        pub description: ::std::option::Option<String>,
    }
    impl ::field_selector::FieldSelector for RestDescriptionAuthOauth2ScopesAdditionalProperties {
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
    pub struct RestDescriptionAuthOauth2 {
        #[doc = "Available OAuth 2.0 scopes."]
        #[serde(rename = "scopes", default)]
        pub scopes: ::std::option::Option<
            ::std::collections::BTreeMap<
                String,
                crate::schemas::RestDescriptionAuthOauth2ScopesAdditionalProperties,
            >,
        >,
    }
    impl ::field_selector::FieldSelector for RestDescriptionAuthOauth2 {
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
    pub struct RestDescriptionAuth {
        #[doc = "OAuth 2.0 authentication information."]
        #[serde(rename = "oauth2", default)]
        pub oauth_2: ::std::option::Option<crate::schemas::RestDescriptionAuthOauth2>,
    }
    impl ::field_selector::FieldSelector for RestDescriptionAuth {
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
    pub struct RestDescriptionIcons {
        #[doc = "The URL of the 16x16 icon."]
        #[serde(rename = "x16", default)]
        pub x_16: ::std::option::Option<String>,
        #[doc = "The URL of the 32x32 icon."]
        #[serde(rename = "x32", default)]
        pub x_32: ::std::option::Option<String>,
    }
    impl ::field_selector::FieldSelector for RestDescriptionIcons {
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
    pub struct RestDescription {
        #[doc = "Authentication information."]
        #[serde(rename = "auth", default)]
        pub auth: ::std::option::Option<crate::schemas::RestDescriptionAuth>,
        #[doc = "[DEPRECATED] The base path for REST requests."]
        #[serde(rename = "basePath", default)]
        pub base_path: ::std::option::Option<String>,
        #[doc = "[DEPRECATED] The base URL for REST requests."]
        #[serde(rename = "baseUrl", default)]
        pub base_url: ::std::option::Option<String>,
        #[doc = "The path for REST batch requests."]
        #[serde(rename = "batchPath", default)]
        pub batch_path: ::std::option::Option<String>,
        #[doc = "Indicates how the API name should be capitalized and split into various parts. Useful for generating pretty class names."]
        #[serde(rename = "canonicalName", default)]
        pub canonical_name: ::std::option::Option<String>,
        #[doc = "The description of this API."]
        #[serde(rename = "description", default)]
        pub description: ::std::option::Option<String>,
        #[doc = "Indicate the version of the Discovery API used to generate this doc."]
        #[serde(rename = "discoveryVersion", default)]
        pub discovery_version: ::std::option::Option<String>,
        #[doc = "A link to human readable documentation for the API."]
        #[serde(rename = "documentationLink", default)]
        pub documentation_link: ::std::option::Option<String>,
        #[doc = "The ETag for this response."]
        #[serde(rename = "etag", default)]
        pub etag: ::std::option::Option<String>,
        #[doc = "Enable exponential backoff for suitable methods in the generated clients."]
        #[serde(rename = "exponentialBackoffDefault", default)]
        pub exponential_backoff_default: ::std::option::Option<bool>,
        #[doc = "A list of supported features for this API."]
        #[serde(rename = "features", default)]
        pub features: ::std::option::Option<Vec<String>>,
        #[doc = "Links to 16x16 and 32x32 icons representing the API."]
        #[serde(rename = "icons", default)]
        pub icons: ::std::option::Option<crate::schemas::RestDescriptionIcons>,
        #[doc = "The ID of this API."]
        #[serde(rename = "id", default)]
        pub id: ::std::option::Option<String>,
        #[doc = "The kind for this response."]
        #[serde(rename = "kind", default)]
        pub kind: ::std::option::Option<String>,
        #[doc = "Labels for the status of this API, such as labs or deprecated."]
        #[serde(rename = "labels", default)]
        pub labels: ::std::option::Option<Vec<String>>,
        #[doc = "API-level methods for this API."]
        #[serde(rename = "methods", default)]
        pub methods:
            ::std::option::Option<::std::collections::BTreeMap<String, crate::schemas::RestMethod>>,
        #[doc = "The name of this API."]
        #[serde(rename = "name", default)]
        pub name: ::std::option::Option<String>,
        #[doc = "The domain of the owner of this API. Together with the ownerName and a packagePath values, this can be used to generate a library for this API which would have a unique fully qualified name."]
        #[serde(rename = "ownerDomain", default)]
        pub owner_domain: ::std::option::Option<String>,
        #[doc = "The name of the owner of this API. See ownerDomain."]
        #[serde(rename = "ownerName", default)]
        pub owner_name: ::std::option::Option<String>,
        #[doc = "The package of the owner of this API. See ownerDomain."]
        #[serde(rename = "packagePath", default)]
        pub package_path: ::std::option::Option<String>,
        #[doc = "Common parameters that apply across all apis."]
        #[serde(rename = "parameters", default)]
        pub parameters:
            ::std::option::Option<::std::collections::BTreeMap<String, crate::schemas::JsonSchema>>,
        #[doc = "The protocol described by this document."]
        #[serde(rename = "protocol", default)]
        pub protocol: ::std::option::Option<String>,
        #[doc = "The resources in this API."]
        #[serde(rename = "resources", default)]
        pub resources: ::std::option::Option<
            ::std::collections::BTreeMap<String, crate::schemas::RestResource>,
        >,
        #[doc = "The version of this API."]
        #[serde(rename = "revision", default)]
        pub revision: ::std::option::Option<String>,
        #[doc = "The root URL under which all API services live."]
        #[serde(rename = "rootUrl", default)]
        pub root_url: ::std::option::Option<String>,
        #[doc = "The schemas for this API."]
        #[serde(rename = "schemas", default)]
        pub schemas:
            ::std::option::Option<::std::collections::BTreeMap<String, crate::schemas::JsonSchema>>,
        #[doc = "The base path for all REST requests."]
        #[serde(rename = "servicePath", default)]
        pub service_path: ::std::option::Option<String>,
        #[doc = "The title of this API."]
        #[serde(rename = "title", default)]
        pub title: ::std::option::Option<String>,
        #[doc = "The version of this API."]
        #[serde(rename = "version", default)]
        pub version: ::std::option::Option<String>,
        #[serde(rename = "version_module", default)]
        pub version_module: ::std::option::Option<bool>,
    }
    impl ::field_selector::FieldSelector for RestDescription {
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
    pub struct RestMethodMediaUploadProtocolsResumable {
        #[doc = "True if this endpoint supports uploading multipart media."]
        #[serde(rename = "multipart", default)]
        pub multipart: ::std::option::Option<bool>,
        #[doc = "The URI path to be used for upload. Should be used in conjunction with the basePath property at the api-level."]
        #[serde(rename = "path", default)]
        pub path: ::std::option::Option<String>,
    }
    impl ::field_selector::FieldSelector for RestMethodMediaUploadProtocolsResumable {
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
    pub struct RestMethodMediaUploadProtocolsSimple {
        #[doc = "True if this endpoint supports upload multipart media."]
        #[serde(rename = "multipart", default)]
        pub multipart: ::std::option::Option<bool>,
        #[doc = "The URI path to be used for upload. Should be used in conjunction with the basePath property at the api-level."]
        #[serde(rename = "path", default)]
        pub path: ::std::option::Option<String>,
    }
    impl ::field_selector::FieldSelector for RestMethodMediaUploadProtocolsSimple {
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
    pub struct RestMethodMediaUploadProtocols {
        #[doc = "Supports the Resumable Media Upload protocol."]
        #[serde(rename = "resumable", default)]
        pub resumable:
            ::std::option::Option<crate::schemas::RestMethodMediaUploadProtocolsResumable>,
        #[doc = "Supports uploading as a single HTTP request."]
        #[serde(rename = "simple", default)]
        pub simple: ::std::option::Option<crate::schemas::RestMethodMediaUploadProtocolsSimple>,
    }
    impl ::field_selector::FieldSelector for RestMethodMediaUploadProtocols {
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
    pub struct RestMethodMediaUpload {
        #[doc = "MIME Media Ranges for acceptable media uploads to this method."]
        #[serde(rename = "accept", default)]
        pub accept: ::std::option::Option<Vec<String>>,
        #[doc = "Maximum size of a media upload, such as \"1MB\", \"2GB\" or \"3TB\"."]
        #[serde(rename = "maxSize", default)]
        pub max_size: ::std::option::Option<String>,
        #[doc = "Supported upload protocols."]
        #[serde(rename = "protocols", default)]
        pub protocols: ::std::option::Option<crate::schemas::RestMethodMediaUploadProtocols>,
    }
    impl ::field_selector::FieldSelector for RestMethodMediaUpload {
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
    pub struct RestMethodRequest {
        #[doc = "parameter name."]
        #[serde(rename = "parameterName", default)]
        pub parameter_name: ::std::option::Option<String>,
        #[doc = "Schema ID for the request schema."]
        #[serde(rename = "$ref", default)]
        pub r#ref: ::std::option::Option<String>,
    }
    impl ::field_selector::FieldSelector for RestMethodRequest {
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
    pub struct RestMethodResponse {
        #[doc = "Schema ID for the response schema."]
        #[serde(rename = "$ref", default)]
        pub r#ref: ::std::option::Option<String>,
    }
    impl ::field_selector::FieldSelector for RestMethodResponse {
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
    pub struct RestMethod {
        #[doc = "Description of this method."]
        #[serde(rename = "description", default)]
        pub description: ::std::option::Option<String>,
        #[doc = "Whether this method requires an ETag to be specified. The ETag is sent as an HTTP If-Match or If-None-Match header."]
        #[serde(rename = "etagRequired", default)]
        pub etag_required: ::std::option::Option<bool>,
        #[doc = "HTTP method used by this method."]
        #[serde(rename = "httpMethod", default)]
        pub http_method: ::std::option::Option<String>,
        #[doc = "A unique ID for this method. This property can be used to match methods between different versions of Discovery."]
        #[serde(rename = "id", default)]
        pub id: ::std::option::Option<String>,
        #[doc = "Media upload parameters."]
        #[serde(rename = "mediaUpload", default)]
        pub media_upload: ::std::option::Option<crate::schemas::RestMethodMediaUpload>,
        #[doc = "Ordered list of required parameters, serves as a hint to clients on how to structure their method signatures. The array is ordered such that the \"most-significant\" parameter appears first."]
        #[serde(rename = "parameterOrder", default)]
        pub parameter_order: ::std::option::Option<Vec<String>>,
        #[doc = "Details for all parameters in this method."]
        #[serde(rename = "parameters", default)]
        pub parameters:
            ::std::option::Option<::std::collections::BTreeMap<String, crate::schemas::JsonSchema>>,
        #[doc = "The URI path of this REST method. Should be used in conjunction with the basePath property at the api-level."]
        #[serde(rename = "path", default)]
        pub path: ::std::option::Option<String>,
        #[doc = "The schema for the request."]
        #[serde(rename = "request", default)]
        pub request: ::std::option::Option<crate::schemas::RestMethodRequest>,
        #[doc = "The schema for the response."]
        #[serde(rename = "response", default)]
        pub response: ::std::option::Option<crate::schemas::RestMethodResponse>,
        #[doc = "OAuth 2.0 scopes applicable to this method."]
        #[serde(rename = "scopes", default)]
        pub scopes: ::std::option::Option<Vec<String>>,
        #[doc = "Whether this method supports media downloads."]
        #[serde(rename = "supportsMediaDownload", default)]
        pub supports_media_download: ::std::option::Option<bool>,
        #[doc = "Whether this method supports media uploads."]
        #[serde(rename = "supportsMediaUpload", default)]
        pub supports_media_upload: ::std::option::Option<bool>,
        #[doc = "Whether this method supports subscriptions."]
        #[serde(rename = "supportsSubscription", default)]
        pub supports_subscription: ::std::option::Option<bool>,
        #[doc = "Indicates that downloads from this method should use the download service URL (i.e. \"/download\"). Only applies if the method supports media download."]
        #[serde(rename = "useMediaDownloadService", default)]
        pub use_media_download_service: ::std::option::Option<bool>,
    }
    impl ::field_selector::FieldSelector for RestMethod {
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
    pub struct RestResource {
        #[doc = "Methods on this resource."]
        #[serde(rename = "methods", default)]
        pub methods:
            ::std::option::Option<::std::collections::BTreeMap<String, crate::schemas::RestMethod>>,
        #[doc = "Sub-resources on this resource."]
        #[serde(rename = "resources", default)]
        pub resources: ::std::option::Option<
            ::std::collections::BTreeMap<String, crate::schemas::RestResource>,
        >,
    }
    impl ::field_selector::FieldSelector for RestResource {
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
    #[doc = "Actions that can be performed on the apis resource"]
    pub fn apis(&self) -> crate::resources::apis::ApisActions<A> {
        crate::resources::apis::ApisActions {
            reqwest: &self.reqwest,
            auth: &self.auth,
        }
    }
}
pub mod resources {
    pub mod apis {
        pub mod params {}
        pub struct ApisActions<'a, A> {
            pub(crate) reqwest: &'a reqwest::Client,
            pub(crate) auth: &'a std::sync::Mutex<A>,
        }
        impl<'a, A: yup_oauth2::GetToken> ApisActions<'a, A> {
            #[doc = "Retrieve the description of a particular version of an api."]
            pub fn get_rest(
                &self,
                api: impl Into<String>,
                version: impl Into<String>,
            ) -> GetRestRequestBuilder<A> {
                GetRestRequestBuilder {
                    reqwest: &self.reqwest,
                    auth: &self.auth,
                    alt: None,
                    fields: None,
                    key: None,
                    oauth_token: None,
                    pretty_print: None,
                    quota_user: None,
                    user_ip: None,
                    api: api.into(),
                    version: version.into(),
                }
            }
            #[doc = "Retrieve the list of APIs supported at this endpoint."]
            pub fn list(&self) -> ListRequestBuilder<A> {
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
                    name: None,
                    preferred: None,
                }
            }
        }
        #[derive(Debug, Clone)]
        pub struct GetRestRequestBuilder<'a, A> {
            pub(crate) reqwest: &'a ::reqwest::Client,
            pub(crate) auth: &'a ::std::sync::Mutex<A>,
            api: String,
            version: String,
            alt: Option<crate::params::Alt>,
            fields: Option<String>,
            key: Option<String>,
            oauth_token: Option<String>,
            pretty_print: Option<bool>,
            quota_user: Option<String>,
            user_ip: Option<String>,
        }
        impl<'a, A: yup_oauth2::GetToken> GetRestRequestBuilder<'a, A> {
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
            ) -> Result<crate::schemas::RestDescription, Box<dyn ::std::error::Error>> {
                self.execute_fields::<_, &str>(None)
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub fn execute_debug(
                self,
            ) -> Result<crate::schemas::RestDescription, Box<dyn ::std::error::Error>> {
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
                let mut output = "https://www.googleapis.com/discovery/v1/".to_owned();
                output.push_str("apis/");
                {
                    let var_as_str = &self.api;
                    output.extend(::percent_encoding::utf8_percent_encode(
                        &var_as_str,
                        crate::SIMPLE,
                    ));
                }
                output.push_str("/");
                {
                    let var_as_str = &self.version;
                    output.extend(::percent_encoding::utf8_percent_encode(
                        &var_as_str,
                        crate::SIMPLE,
                    ));
                }
                output.push_str("/rest");
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
            name: Option<String>,
            preferred: Option<bool>,
            alt: Option<crate::params::Alt>,
            fields: Option<String>,
            key: Option<String>,
            oauth_token: Option<String>,
            pretty_print: Option<bool>,
            quota_user: Option<String>,
            user_ip: Option<String>,
        }
        impl<'a, A: yup_oauth2::GetToken> ListRequestBuilder<'a, A> {
            #[doc = "Only include APIs with the given name."]
            pub fn name(mut self, value: impl Into<String>) -> Self {
                self.name = Some(value.into());
                self
            }
            #[doc = "Return only the preferred version of an API."]
            pub fn preferred(mut self, value: bool) -> Self {
                self.preferred = Some(value);
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
            ) -> Result<crate::schemas::DirectoryList, Box<dyn ::std::error::Error>> {
                self.execute_fields::<_, &str>(None)
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub fn execute_debug(
                self,
            ) -> Result<crate::schemas::DirectoryList, Box<dyn ::std::error::Error>> {
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
                let mut output = "https://www.googleapis.com/discovery/v1/".to_owned();
                output.push_str("apis");
                output
            }
            fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
                let req = self.reqwest.request(::reqwest::Method::GET, path);
                let req = req.query(&[("name", &self.name)]);
                let req = req.query(&[("preferred", &self.preferred)]);
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
