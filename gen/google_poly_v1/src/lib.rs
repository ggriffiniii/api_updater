pub mod schemas {
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum AssetLicense {
        #[doc = "Unlicensed: All Rights Reserved by the author. Unlicensed assets are\n**not** returned by List Assets."]
        AllRightsReserved,
        #[doc = "Creative Commons CC-BY 3.0. https://creativecommons.org/licenses/by/3.0/"]
        CreativeCommonsBy,
        #[doc = "Unknown license value."]
        Unknown,
    }
    impl AssetLicense {
        pub fn as_str(self) -> &'static str {
            match self {
                AssetLicense::AllRightsReserved => "ALL_RIGHTS_RESERVED",
                AssetLicense::CreativeCommonsBy => "CREATIVE_COMMONS_BY",
                AssetLicense::Unknown => "UNKNOWN",
            }
        }
    }
    impl ::std::fmt::Display for AssetLicense {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for AssetLicense {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for AssetLicense {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "ALL_RIGHTS_RESERVED" => AssetLicense::AllRightsReserved,
                "CREATIVE_COMMONS_BY" => AssetLicense::CreativeCommonsBy,
                "UNKNOWN" => AssetLicense::Unknown,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::field_selector::FieldSelector for AssetLicense {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum AssetVisibility {
        #[doc = "Access to the asset and its underlying files and resources is restricted to\nthe author.\n**Authentication:** You must supply an OAuth token that corresponds to the\nauthor's account."]
        Private,
        #[doc = "Access to the asset and its underlying files and resources is available\nto anyone."]
        Public,
        #[doc = "Access to the asset and its underlying files and resources is available to\nanyone with the asset's name. Unlisted assets are **not**\nreturned by List Assets."]
        Unlisted,
        #[doc = "Unknown (and invalid) visibility."]
        VisibilityUnspecified,
    }
    impl AssetVisibility {
        pub fn as_str(self) -> &'static str {
            match self {
                AssetVisibility::Private => "PRIVATE",
                AssetVisibility::Public => "PUBLIC",
                AssetVisibility::Unlisted => "UNLISTED",
                AssetVisibility::VisibilityUnspecified => "VISIBILITY_UNSPECIFIED",
            }
        }
    }
    impl ::std::fmt::Display for AssetVisibility {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for AssetVisibility {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for AssetVisibility {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "PRIVATE" => AssetVisibility::Private,
                "PUBLIC" => AssetVisibility::Public,
                "UNLISTED" => AssetVisibility::Unlisted,
                "VISIBILITY_UNSPECIFIED" => AssetVisibility::VisibilityUnspecified,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::field_selector::FieldSelector for AssetVisibility {
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
    pub struct Asset {
        #[doc = "The author's publicly visible name. Use this name when giving credit to the\nauthor. For more information, see [Licensing](/poly/discover/licensing)."]
        #[serde(rename = "authorName", default)]
        pub author_name: ::std::option::Option<String>,
        #[doc = "For published assets, the time when the asset was published.\nFor unpublished assets, the time when the asset was created."]
        #[serde(rename = "createTime", default)]
        pub create_time: ::std::option::Option<String>,
        #[doc = "The human-readable description, set by the asset's author."]
        #[serde(rename = "description", default)]
        pub description: ::std::option::Option<String>,
        #[doc = "The human-readable name, set by the asset's author."]
        #[serde(rename = "displayName", default)]
        pub display_name: ::std::option::Option<String>,
        #[doc = "A list of Formats where each\nformat describes one representation of the asset."]
        #[serde(rename = "formats", default)]
        pub formats: ::std::option::Option<Vec<crate::schemas::Format>>,
        #[doc = "Whether this asset has been curated by the Poly team."]
        #[serde(rename = "isCurated", default)]
        pub is_curated: ::std::option::Option<bool>,
        #[doc = "The license under which the author has made the asset available\nfor use, if any."]
        #[serde(rename = "license", default)]
        pub license: ::std::option::Option<crate::schemas::AssetLicense>,
        #[doc = "Application-defined opaque metadata for this asset. This field is only\nreturned when querying for the signed-in user's own assets, not for public\nassets. This string is limited to 1K chars. It is up to the creator of\nthe asset to define the format for this string (for example, JSON)."]
        #[serde(rename = "metadata", default)]
        pub metadata: ::std::option::Option<String>,
        #[doc = "The unique identifier for the asset in the form:\n`assets/{ASSET_ID}`."]
        #[serde(rename = "name", default)]
        pub name: ::std::option::Option<String>,
        #[doc = "Hints for displaying the asset. Note that these parameters are not\nimmutable; the author of an asset may change them post-publication."]
        #[serde(rename = "presentationParams", default)]
        pub presentation_params: ::std::option::Option<crate::schemas::PresentationParams>,
        #[doc = "The remix info for the asset."]
        #[serde(rename = "remixInfo", default)]
        pub remix_info: ::std::option::Option<crate::schemas::RemixInfo>,
        #[doc = "The thumbnail image for the asset."]
        #[serde(rename = "thumbnail", default)]
        pub thumbnail: ::std::option::Option<crate::schemas::File>,
        #[doc = "The time when the asset was last modified. For published assets, whose\ncontents are immutable, the update time changes only when metadata\nproperties, such as visibility, are updated."]
        #[serde(rename = "updateTime", default)]
        pub update_time: ::std::option::Option<String>,
        #[doc = "The visibility of the asset and who can access it."]
        #[serde(rename = "visibility", default)]
        pub visibility: ::std::option::Option<crate::schemas::AssetVisibility>,
    }
    impl ::field_selector::FieldSelector for Asset {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum AssetImportMessageCode {
        #[doc = "Unknown error code."]
        CodeUnspecified,
        #[doc = "Default materials are used in the model. This means that one or more\nfaces is using default materials either because no usemtl statement was\nspecified or because the requested material was not found due to a\nmissing material file or bad material name. This does not cover the case\nof missing textures."]
        DefaultMaterials,
        #[doc = "When generating the preview for the import, no geometry was found."]
        EmptyModel,
        #[doc = "The importer was not able to import the model before the expiration time."]
        Expired,
        #[doc = "Multiple files were encountered in addition to a ZIP archive. When\nuploading an archive only one file is permitted."]
        ExtraFilesWithArchive,
        #[doc = "The importer encountered a fatal error and was unable to import the\nmodel."]
        FatalError,
        #[doc = "The importer encountered a problem reading an image file."]
        ImageError,
        #[doc = "The import includes a file of an unsupported element type. The file path\nis specified."]
        InvalidElementType,
        #[doc = "The asset import did not include any file that we can import (i.e. an OBJ\nfile)."]
        NoImportableFile,
        #[doc = "A problem was encountered while parsing the OBJ file. The converter makes\na 'best effort' attempt to continue when encountering such issues. In\nsome cases the resulting preview model may still be acceptable. The\ndetails can be found in the parse error message."]
        ObjParseError,
    }
    impl AssetImportMessageCode {
        pub fn as_str(self) -> &'static str {
            match self {
                AssetImportMessageCode::CodeUnspecified => "CODE_UNSPECIFIED",
                AssetImportMessageCode::DefaultMaterials => "DEFAULT_MATERIALS",
                AssetImportMessageCode::EmptyModel => "EMPTY_MODEL",
                AssetImportMessageCode::Expired => "EXPIRED",
                AssetImportMessageCode::ExtraFilesWithArchive => "EXTRA_FILES_WITH_ARCHIVE",
                AssetImportMessageCode::FatalError => "FATAL_ERROR",
                AssetImportMessageCode::ImageError => "IMAGE_ERROR",
                AssetImportMessageCode::InvalidElementType => "INVALID_ELEMENT_TYPE",
                AssetImportMessageCode::NoImportableFile => "NO_IMPORTABLE_FILE",
                AssetImportMessageCode::ObjParseError => "OBJ_PARSE_ERROR",
            }
        }
    }
    impl ::std::fmt::Display for AssetImportMessageCode {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for AssetImportMessageCode {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for AssetImportMessageCode {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "CODE_UNSPECIFIED" => AssetImportMessageCode::CodeUnspecified,
                "DEFAULT_MATERIALS" => AssetImportMessageCode::DefaultMaterials,
                "EMPTY_MODEL" => AssetImportMessageCode::EmptyModel,
                "EXPIRED" => AssetImportMessageCode::Expired,
                "EXTRA_FILES_WITH_ARCHIVE" => AssetImportMessageCode::ExtraFilesWithArchive,
                "FATAL_ERROR" => AssetImportMessageCode::FatalError,
                "IMAGE_ERROR" => AssetImportMessageCode::ImageError,
                "INVALID_ELEMENT_TYPE" => AssetImportMessageCode::InvalidElementType,
                "NO_IMPORTABLE_FILE" => AssetImportMessageCode::NoImportableFile,
                "OBJ_PARSE_ERROR" => AssetImportMessageCode::ObjParseError,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::field_selector::FieldSelector for AssetImportMessageCode {
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
    pub struct AssetImportMessage {
        #[doc = "The code associated with this message."]
        #[serde(rename = "code", default)]
        pub code: ::std::option::Option<crate::schemas::AssetImportMessageCode>,
        #[doc = "An optional file path. Only present for those error codes that specify it."]
        #[serde(rename = "filePath", default)]
        pub file_path: ::std::option::Option<String>,
        #[doc = "An optional image error. Only present for INVALID_IMAGE_FILE."]
        #[serde(rename = "imageError", default)]
        pub image_error: ::std::option::Option<crate::schemas::ImageError>,
        #[doc = "An optional OBJ parse error. Only present for OBJ_PARSE_ERROR."]
        #[serde(rename = "objParseError", default)]
        pub obj_parse_error: ::std::option::Option<crate::schemas::ObjParseError>,
    }
    impl ::field_selector::FieldSelector for AssetImportMessage {
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
    pub struct File {
        #[doc = "The MIME content-type, such as `image/png`.\nFor more information, see\n[MIME\ntypes](//developer.mozilla.org/en-US/docs/Web/HTTP/Basics_of_HTTP/MIME_types)."]
        #[serde(rename = "contentType", default)]
        pub content_type: ::std::option::Option<String>,
        #[doc = "The path of the resource file relative to the\nroot file. For root or thumbnail files,\nthis is just the filename."]
        #[serde(rename = "relativePath", default)]
        pub relative_path: ::std::option::Option<String>,
        #[doc = "The URL where the file data can be retrieved."]
        #[serde(rename = "url", default)]
        pub url: ::std::option::Option<String>,
    }
    impl ::field_selector::FieldSelector for File {
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
    pub struct Format {
        #[doc = "Complexity stats about this representation of the asset."]
        #[serde(rename = "formatComplexity", default)]
        pub format_complexity: ::std::option::Option<crate::schemas::FormatComplexity>,
        #[doc = "A short string that identifies the format type of this representation.\nPossible values are: `FBX`, `GLTF`, `GLTF2`, `OBJ`, and `TILT`."]
        #[serde(rename = "formatType", default)]
        pub format_type: ::std::option::Option<String>,
        #[doc = "A list of dependencies of the root element. May include, but is not\nlimited to, materials, textures, and shader programs."]
        #[serde(rename = "resources", default)]
        pub resources: ::std::option::Option<Vec<crate::schemas::File>>,
        #[doc = "The root of the file hierarchy. This will always be populated.\nFor some format_types - such as `TILT`, which are\nself-contained - this is all of the data.\n\nOther types - such as `OBJ` - often reference other data elements.\nThese are contained in the resources field."]
        #[serde(rename = "root", default)]
        pub root: ::std::option::Option<crate::schemas::File>,
    }
    impl ::field_selector::FieldSelector for Format {
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
    pub struct FormatComplexity {
        #[doc = "A non-negative integer that represents the level of detail (LOD) of this\nformat relative to other formats of the same asset with the same\nformat_type.\nThis hint allows you to sort formats from the most-detailed (0) to\nleast-detailed (integers greater than 0)."]
        #[serde(rename = "lodHint", default)]
        pub lod_hint: ::std::option::Option<i32>,
        #[doc = "The estimated number of triangles."]
        #[serde(rename = "triangleCount", default)]
        #[serde(with = "crate::parsed_string")]
        pub triangle_count: ::std::option::Option<i64>,
    }
    impl ::field_selector::FieldSelector for FormatComplexity {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum ImageErrorCode {
        #[doc = "Unknown error code."]
        CodeUnspecified,
        #[doc = "The image size is too large."]
        ImageTooBig,
        #[doc = "We were unable to read the image file."]
        InvalidImage,
        #[doc = "The image data does not match the expected MIME type of the image."]
        WrongImageType,
    }
    impl ImageErrorCode {
        pub fn as_str(self) -> &'static str {
            match self {
                ImageErrorCode::CodeUnspecified => "CODE_UNSPECIFIED",
                ImageErrorCode::ImageTooBig => "IMAGE_TOO_BIG",
                ImageErrorCode::InvalidImage => "INVALID_IMAGE",
                ImageErrorCode::WrongImageType => "WRONG_IMAGE_TYPE",
            }
        }
    }
    impl ::std::fmt::Display for ImageErrorCode {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for ImageErrorCode {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for ImageErrorCode {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "CODE_UNSPECIFIED" => ImageErrorCode::CodeUnspecified,
                "IMAGE_TOO_BIG" => ImageErrorCode::ImageTooBig,
                "INVALID_IMAGE" => ImageErrorCode::InvalidImage,
                "WRONG_IMAGE_TYPE" => ImageErrorCode::WrongImageType,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::field_selector::FieldSelector for ImageErrorCode {
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
    pub struct ImageError {
        #[doc = "The type of image error encountered. Optional for older image errors."]
        #[serde(rename = "code", default)]
        pub code: ::std::option::Option<crate::schemas::ImageErrorCode>,
        #[doc = "The file path in the import of the image that was rejected."]
        #[serde(rename = "filePath", default)]
        pub file_path: ::std::option::Option<String>,
    }
    impl ::field_selector::FieldSelector for ImageError {
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
    pub struct ListAssetsResponse {
        #[doc = "A list of assets that match the criteria specified in the request."]
        #[serde(rename = "assets", default)]
        pub assets: ::std::option::Option<Vec<crate::schemas::Asset>>,
        #[doc = "The continuation token for retrieving the next page. If empty,\nindicates that there are no more pages. To get the next page, submit the\nsame request specifying this value as the\npage_token."]
        #[serde(rename = "nextPageToken", default)]
        pub next_page_token: ::std::option::Option<String>,
        #[doc = "The total number of assets in the list, without pagination."]
        #[serde(rename = "totalSize", default)]
        pub total_size: ::std::option::Option<i32>,
    }
    impl ::field_selector::FieldSelector for ListAssetsResponse {
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
    pub struct ListLikedAssetsResponse {
        #[doc = "A list of assets that match the criteria specified in the request."]
        #[serde(rename = "assets", default)]
        pub assets: ::std::option::Option<Vec<crate::schemas::Asset>>,
        #[doc = "The continuation token for retrieving the next page. If empty,\nindicates that there are no more pages. To get the next page, submit the\nsame request specifying this value as the\npage_token."]
        #[serde(rename = "nextPageToken", default)]
        pub next_page_token: ::std::option::Option<String>,
        #[doc = "The total number of assets in the list, without pagination."]
        #[serde(rename = "totalSize", default)]
        pub total_size: ::std::option::Option<i32>,
    }
    impl ::field_selector::FieldSelector for ListLikedAssetsResponse {
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
    pub struct ListUserAssetsResponse {
        #[doc = "The continuation token for retrieving the next page. If empty,\nindicates that there are no more pages. To get the next page, submit the\nsame request specifying this value as the\npage_token."]
        #[serde(rename = "nextPageToken", default)]
        pub next_page_token: ::std::option::Option<String>,
        #[doc = "The total number of assets in the list, without pagination."]
        #[serde(rename = "totalSize", default)]
        pub total_size: ::std::option::Option<i32>,
        #[doc = "A list of UserAssets matching the request."]
        #[serde(rename = "userAssets", default)]
        pub user_assets: ::std::option::Option<Vec<crate::schemas::UserAsset>>,
    }
    impl ::field_selector::FieldSelector for ListUserAssetsResponse {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum ObjParseErrorCode {
        #[doc = "Unknown error code."]
        CodeUnspecified,
        #[doc = "The specified file was not found in the import."]
        FileNotFound,
        #[doc = "A missing file was found at a different file path."]
        FileSubstitution,
        #[doc = "Vertex references are specified in an inconsistent style for a face (e.g.\nsome vertices specify texture vertices but some don't)."]
        InconsistentVertexRefs,
        #[doc = "The command is invalid."]
        InvalidCommand,
        #[doc = "The file path was invalid. Only relative paths are supported."]
        InvalidFilePath,
        #[doc = "A invalid number was specified."]
        InvalidNumber,
        #[doc = "The smoothing group is not valid."]
        InvalidSmoothingGroup,
        #[doc = "The specified texture option is not valid."]
        InvalidTextureOption,
        #[doc = "The specified parameter value was not recognized."]
        InvalidValue,
        #[doc = "An invalid vertex reference was specified."]
        InvalidVertexRef,
        #[doc = "A line in an OBJ or MTL file exceeded the maximum line length."]
        LineTooLong,
        #[doc = "An expected file name was not specified."]
        MissingFileName,
        #[doc = "A vertex reference does not specify a geometric vertex."]
        MissingGeometricVertex,
        #[doc = "An expected token was not found."]
        MissingToken,
        #[doc = "Vertex colors were specified for only some vertices of a face."]
        MissingVertexColors,
        #[doc = "Material parameters were specified before the first material definition."]
        NoMaterialDefined,
        #[doc = "The specified number was too large or small for its usage."]
        NumberOutOfRange,
        #[doc = "The vertex specified too few dimensions for its usage."]
        TooFewDimensions,
        #[doc = "The face specified too few vertices."]
        TooFewVertices,
        #[doc = "The vertex specified too many dimensions for its usage."]
        TooManyDimensions,
        #[doc = "The maximum number of problems to report was reached. Parsing continues,\nbut further problems will be ignored."]
        TooManyProblems,
        #[doc = "The specified material was not found in any material definition in the\nimport."]
        UnknownMaterial,
        #[doc = "This command is a valid OBJ command but is not supported. This error is\nonly generated for the first instance of such a command."]
        UnsupportedCommand,
        #[doc = "This line ended with unparsed token characters."]
        UnusedTokens,
        #[doc = "The specified vertex was not found."]
        VertexNotFound,
    }
    impl ObjParseErrorCode {
        pub fn as_str(self) -> &'static str {
            match self {
                ObjParseErrorCode::CodeUnspecified => "CODE_UNSPECIFIED",
                ObjParseErrorCode::FileNotFound => "FILE_NOT_FOUND",
                ObjParseErrorCode::FileSubstitution => "FILE_SUBSTITUTION",
                ObjParseErrorCode::InconsistentVertexRefs => "INCONSISTENT_VERTEX_REFS",
                ObjParseErrorCode::InvalidCommand => "INVALID_COMMAND",
                ObjParseErrorCode::InvalidFilePath => "INVALID_FILE_PATH",
                ObjParseErrorCode::InvalidNumber => "INVALID_NUMBER",
                ObjParseErrorCode::InvalidSmoothingGroup => "INVALID_SMOOTHING_GROUP",
                ObjParseErrorCode::InvalidTextureOption => "INVALID_TEXTURE_OPTION",
                ObjParseErrorCode::InvalidValue => "INVALID_VALUE",
                ObjParseErrorCode::InvalidVertexRef => "INVALID_VERTEX_REF",
                ObjParseErrorCode::LineTooLong => "LINE_TOO_LONG",
                ObjParseErrorCode::MissingFileName => "MISSING_FILE_NAME",
                ObjParseErrorCode::MissingGeometricVertex => "MISSING_GEOMETRIC_VERTEX",
                ObjParseErrorCode::MissingToken => "MISSING_TOKEN",
                ObjParseErrorCode::MissingVertexColors => "MISSING_VERTEX_COLORS",
                ObjParseErrorCode::NoMaterialDefined => "NO_MATERIAL_DEFINED",
                ObjParseErrorCode::NumberOutOfRange => "NUMBER_OUT_OF_RANGE",
                ObjParseErrorCode::TooFewDimensions => "TOO_FEW_DIMENSIONS",
                ObjParseErrorCode::TooFewVertices => "TOO_FEW_VERTICES",
                ObjParseErrorCode::TooManyDimensions => "TOO_MANY_DIMENSIONS",
                ObjParseErrorCode::TooManyProblems => "TOO_MANY_PROBLEMS",
                ObjParseErrorCode::UnknownMaterial => "UNKNOWN_MATERIAL",
                ObjParseErrorCode::UnsupportedCommand => "UNSUPPORTED_COMMAND",
                ObjParseErrorCode::UnusedTokens => "UNUSED_TOKENS",
                ObjParseErrorCode::VertexNotFound => "VERTEX_NOT_FOUND",
            }
        }
    }
    impl ::std::fmt::Display for ObjParseErrorCode {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for ObjParseErrorCode {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for ObjParseErrorCode {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "CODE_UNSPECIFIED" => ObjParseErrorCode::CodeUnspecified,
                "FILE_NOT_FOUND" => ObjParseErrorCode::FileNotFound,
                "FILE_SUBSTITUTION" => ObjParseErrorCode::FileSubstitution,
                "INCONSISTENT_VERTEX_REFS" => ObjParseErrorCode::InconsistentVertexRefs,
                "INVALID_COMMAND" => ObjParseErrorCode::InvalidCommand,
                "INVALID_FILE_PATH" => ObjParseErrorCode::InvalidFilePath,
                "INVALID_NUMBER" => ObjParseErrorCode::InvalidNumber,
                "INVALID_SMOOTHING_GROUP" => ObjParseErrorCode::InvalidSmoothingGroup,
                "INVALID_TEXTURE_OPTION" => ObjParseErrorCode::InvalidTextureOption,
                "INVALID_VALUE" => ObjParseErrorCode::InvalidValue,
                "INVALID_VERTEX_REF" => ObjParseErrorCode::InvalidVertexRef,
                "LINE_TOO_LONG" => ObjParseErrorCode::LineTooLong,
                "MISSING_FILE_NAME" => ObjParseErrorCode::MissingFileName,
                "MISSING_GEOMETRIC_VERTEX" => ObjParseErrorCode::MissingGeometricVertex,
                "MISSING_TOKEN" => ObjParseErrorCode::MissingToken,
                "MISSING_VERTEX_COLORS" => ObjParseErrorCode::MissingVertexColors,
                "NO_MATERIAL_DEFINED" => ObjParseErrorCode::NoMaterialDefined,
                "NUMBER_OUT_OF_RANGE" => ObjParseErrorCode::NumberOutOfRange,
                "TOO_FEW_DIMENSIONS" => ObjParseErrorCode::TooFewDimensions,
                "TOO_FEW_VERTICES" => ObjParseErrorCode::TooFewVertices,
                "TOO_MANY_DIMENSIONS" => ObjParseErrorCode::TooManyDimensions,
                "TOO_MANY_PROBLEMS" => ObjParseErrorCode::TooManyProblems,
                "UNKNOWN_MATERIAL" => ObjParseErrorCode::UnknownMaterial,
                "UNSUPPORTED_COMMAND" => ObjParseErrorCode::UnsupportedCommand,
                "UNUSED_TOKENS" => ObjParseErrorCode::UnusedTokens,
                "VERTEX_NOT_FOUND" => ObjParseErrorCode::VertexNotFound,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::field_selector::FieldSelector for ObjParseErrorCode {
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
    pub struct ObjParseError {
        #[doc = "The type of problem found (required)."]
        #[serde(rename = "code", default)]
        pub code: ::std::option::Option<crate::schemas::ObjParseErrorCode>,
        #[doc = "The ending character index at which the problem was found."]
        #[serde(rename = "endIndex", default)]
        pub end_index: ::std::option::Option<i32>,
        #[doc = "The file path in which the problem was found."]
        #[serde(rename = "filePath", default)]
        pub file_path: ::std::option::Option<String>,
        #[doc = "The text of the line. Note that this may be truncated if the line was very\nlong. This may not include the error if it occurs after line truncation."]
        #[serde(rename = "line", default)]
        pub line: ::std::option::Option<String>,
        #[doc = "Line number at which the problem was found."]
        #[serde(rename = "lineNumber", default)]
        pub line_number: ::std::option::Option<i32>,
        #[doc = "The starting character index at which the problem was found."]
        #[serde(rename = "startIndex", default)]
        pub start_index: ::std::option::Option<i32>,
    }
    impl ::field_selector::FieldSelector for ObjParseError {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
    pub enum PresentationParamsColorSpace {
        #[doc = "Colors should be converted to linear by assuming gamma = 2.0."]
        Gamma,
        #[doc = "Linear color values. Default."]
        Linear,
        #[doc = "Invalid color value."]
        Unknown,
    }
    impl PresentationParamsColorSpace {
        pub fn as_str(self) -> &'static str {
            match self {
                PresentationParamsColorSpace::Gamma => "GAMMA",
                PresentationParamsColorSpace::Linear => "LINEAR",
                PresentationParamsColorSpace::Unknown => "UNKNOWN",
            }
        }
    }
    impl ::std::fmt::Display for PresentationParamsColorSpace {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl ::serde::Serialize for PresentationParamsColorSpace {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: ::serde::ser::Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }
    impl<'de> ::serde::Deserialize<'de> for PresentationParamsColorSpace {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::de::Deserializer<'de>,
        {
            let value: &'de str = <&str>::deserialize(deserializer)?;
            Ok(match value {
                "GAMMA" => PresentationParamsColorSpace::Gamma,
                "LINEAR" => PresentationParamsColorSpace::Linear,
                "UNKNOWN" => PresentationParamsColorSpace::Unknown,
                _ => {
                    return Err(::serde::de::Error::custom(format!(
                        "invalid enum for #name: {}",
                        value
                    )))
                }
            })
        }
    }
    impl ::field_selector::FieldSelector for PresentationParamsColorSpace {
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
    pub struct PresentationParams {
        #[doc = "A background color which could be used for displaying the 3D asset in a\n'thumbnail' or 'palette' style view. Authors have the option to set this\nbackground color when publishing or editing their asset.\n\nThis is represented as a six-digit hexademical triplet specifying the\nRGB components of the background color, e.g. #FF0000 for Red."]
        #[serde(rename = "backgroundColor", default)]
        pub background_color: ::std::option::Option<String>,
        #[doc = "The materials' diffuse/albedo color. This does not apply to vertex colors\nor texture maps."]
        #[serde(rename = "colorSpace", default)]
        pub color_space: ::std::option::Option<crate::schemas::PresentationParamsColorSpace>,
        #[doc = "A rotation that should be applied to the object root to make it upright.\nMore precisely, this quaternion transforms from \"object space\" (the space\nin which the object is defined) to \"presentation space\", a coordinate\nsystem where +Y is up, +X is right, -Z is forward. For example, if\nthe object is the Eiffel Tower, in its local coordinate system the\nobject might be laid out such that the base of the tower is on the\nYZ plane and the tip of the tower is towards positive X. In this case\nthis quaternion would specify a rotation (of 90 degrees about the Z\naxis) such that in the presentation space the base of the tower is\naligned with the XZ plane, and the tip of the tower lies towards +Y.\n\nThis rotation is unrelated to the object's pose in the web preview,\nwhich is just a camera position setting and is *not* reflected in this\nrotation.\n\nPlease note: this is applicable only to the gLTF."]
        #[serde(rename = "orientingRotation", default)]
        pub orienting_rotation: ::std::option::Option<crate::schemas::Quaternion>,
    }
    impl ::field_selector::FieldSelector for PresentationParams {
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
    pub struct Quaternion {
        #[doc = "The scalar component."]
        #[serde(rename = "w", default)]
        pub w: ::std::option::Option<f64>,
        #[doc = "The x component."]
        #[serde(rename = "x", default)]
        pub x: ::std::option::Option<f64>,
        #[doc = "The y component."]
        #[serde(rename = "y", default)]
        pub y: ::std::option::Option<f64>,
        #[doc = "The z component."]
        #[serde(rename = "z", default)]
        pub z: ::std::option::Option<f64>,
    }
    impl ::field_selector::FieldSelector for Quaternion {
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
    pub struct RemixInfo {
        #[doc = "Resource ids for the sources of this remix, of the form:\n`assets/{ASSET_ID}`"]
        #[serde(rename = "sourceAsset", default)]
        pub source_asset: ::std::option::Option<Vec<String>>,
    }
    impl ::field_selector::FieldSelector for RemixInfo {
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
    pub struct StartAssetImportResponse {
        #[doc = "The id of newly created asset. If this is empty when the operation is\ncomplete it means the import failed. Please refer to the\nassetImportMessages field to understand what went wrong."]
        #[serde(rename = "assetId", default)]
        pub asset_id: ::std::option::Option<String>,
        #[doc = "The id of the asset import."]
        #[serde(rename = "assetImportId", default)]
        pub asset_import_id: ::std::option::Option<String>,
        #[doc = "The message from the asset import. This will contain any warnings\n(or - in the case of failure - errors) that occurred during import."]
        #[serde(rename = "assetImportMessages", default)]
        pub asset_import_messages: ::std::option::Option<Vec<crate::schemas::AssetImportMessage>>,
        #[doc = "The publish URL for the asset."]
        #[serde(rename = "publishUrl", default)]
        pub publish_url: ::std::option::Option<String>,
    }
    impl ::field_selector::FieldSelector for StartAssetImportResponse {
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
    pub struct UserAsset {
        #[doc = "An Asset."]
        #[serde(rename = "asset", default)]
        pub asset: ::std::option::Option<crate::schemas::Asset>,
    }
    impl ::field_selector::FieldSelector for UserAsset {
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
    #[doc = "Actions that can be performed on the assets resource"]
    pub fn assets(&self) -> crate::resources::assets::AssetsActions<A> {
        crate::resources::assets::AssetsActions {
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
    pub mod assets {
        pub mod params {
            #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
            pub enum ListMaxComplexity {
                Complex,
                ComplexityUnspecified,
                Medium,
                Simple,
            }
            impl ListMaxComplexity {
                pub fn as_str(self) -> &'static str {
                    match self {
                        ListMaxComplexity::Complex => "COMPLEX",
                        ListMaxComplexity::ComplexityUnspecified => "COMPLEXITY_UNSPECIFIED",
                        ListMaxComplexity::Medium => "MEDIUM",
                        ListMaxComplexity::Simple => "SIMPLE",
                    }
                }
            }
            impl ::std::fmt::Display for ListMaxComplexity {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                    f.write_str(self.as_str())
                }
            }
            impl ::serde::Serialize for ListMaxComplexity {
                fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
                where
                    S: ::serde::ser::Serializer,
                {
                    serializer.serialize_str(self.as_str())
                }
            }
            impl<'de> ::serde::Deserialize<'de> for ListMaxComplexity {
                fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
                where
                    D: ::serde::de::Deserializer<'de>,
                {
                    let value: &'de str = <&str>::deserialize(deserializer)?;
                    Ok(match value {
                        "COMPLEX" => ListMaxComplexity::Complex,
                        "COMPLEXITY_UNSPECIFIED" => ListMaxComplexity::ComplexityUnspecified,
                        "MEDIUM" => ListMaxComplexity::Medium,
                        "SIMPLE" => ListMaxComplexity::Simple,
                        _ => {
                            return Err(::serde::de::Error::custom(format!(
                                "invalid enum for #name: {}",
                                value
                            )))
                        }
                    })
                }
            }
            impl ::field_selector::FieldSelector for ListMaxComplexity {
                fn field_selector_with_ident(ident: &str, selector: &mut String) {
                    match selector.chars().rev().nth(0) {
                        Some(',') | None => {}
                        _ => selector.push_str(","),
                    }
                    selector.push_str(ident);
                }
            }
        }
        pub struct AssetsActions<'a, A> {
            pub(crate) reqwest: &'a reqwest::Client,
            pub(crate) auth: &'a std::sync::Mutex<A>,
        }
        impl<'a, A: yup_oauth2::GetToken> AssetsActions<'a, A> {
            #[doc = "Returns detailed information about an asset given its name.\nPRIVATE assets are returned only if\nthe currently authenticated user (via OAuth token) is the author of the\nasset."]
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
            #[doc = "Lists all public, remixable assets. These are assets with an access level\nof PUBLIC and published under the\nCC-By license."]
            pub fn list(&self) -> ListRequestBuilder<A> {
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
                    category: None,
                    curated: None,
                    format: None,
                    keywords: None,
                    max_complexity: None,
                    order_by: None,
                    page_size: None,
                    page_token: None,
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
            ) -> Result<crate::schemas::Asset, Box<dyn ::std::error::Error>> {
                self.execute_fields::<_, &str>(None)
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub fn execute_debug(
                self,
            ) -> Result<crate::schemas::Asset, Box<dyn ::std::error::Error>> {
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
                let mut output = "https://poly.googleapis.com/".to_owned();
                output.push_str("v1/");
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
                req
            }
        }
        #[derive(Debug, Clone)]
        pub struct ListRequestBuilder<'a, A> {
            pub(crate) reqwest: &'a ::reqwest::Client,
            pub(crate) auth: &'a ::std::sync::Mutex<A>,
            category: Option<String>,
            curated: Option<bool>,
            format: Option<String>,
            keywords: Option<String>,
            max_complexity: Option<crate::resources::assets::params::ListMaxComplexity>,
            order_by: Option<String>,
            page_size: Option<i32>,
            page_token: Option<String>,
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
            #[doc = "Filter assets based on the specified category. Supported values are:\n`animals`, `architecture`, `art`, `food`, `nature`, `objects`, `people`,\n`scenes`, `technology`, and `transport`."]
            pub fn category(mut self, value: impl Into<String>) -> Self {
                self.category = Some(value.into());
                self
            }
            #[doc = "Return only assets that have been curated by the Poly team."]
            pub fn curated(mut self, value: bool) -> Self {
                self.curated = Some(value);
                self
            }
            #[doc = "Return only assets with the matching format. Acceptable values are:\n`BLOCKS`, `FBX`, `GLTF`, `GLTF2`, `OBJ`, `TILT`."]
            pub fn format(mut self, value: impl Into<String>) -> Self {
                self.format = Some(value.into());
                self
            }
            #[doc = "One or more search terms to be matched against all text that Poly has\nindexed for assets, which includes display_name,\ndescription, and tags. Multiple keywords should be\nseparated by spaces."]
            pub fn keywords(mut self, value: impl Into<String>) -> Self {
                self.keywords = Some(value.into());
                self
            }
            #[doc = "Returns assets that are of the specified complexity or less. Defaults to\nCOMPLEX. For example, a request for\nMEDIUM assets also includes\nSIMPLE assets."]
            pub fn max_complexity(
                mut self,
                value: crate::resources::assets::params::ListMaxComplexity,
            ) -> Self {
                self.max_complexity = Some(value);
                self
            }
            #[doc = "Specifies an ordering for assets. Acceptable values are:\n`BEST`, `NEWEST`, `OLDEST`. Defaults to `BEST`, which ranks assets\nbased on a combination of popularity and other features."]
            pub fn order_by(mut self, value: impl Into<String>) -> Self {
                self.order_by = Some(value.into());
                self
            }
            #[doc = "The maximum number of assets to be returned. This value must be between `1`\nand `100`. Defaults to `20`."]
            pub fn page_size(mut self, value: i32) -> Self {
                self.page_size = Some(value);
                self
            }
            #[doc = "Specifies a continuation token from a previous search whose results were\nsplit into multiple pages. To get the next page, submit the same request\nspecifying the value from\nnext_page_token."]
            pub fn page_token(mut self, value: impl Into<String>) -> Self {
                self.page_token = Some(value.into());
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
            pub fn iter_assets<T>(mut self) -> crate::iter::PageItemIter<Self, T>
            where
                T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
            {
                let mut fields = concat!("nextPageToken,", "assets").to_owned();
                let items_fields = T::field_selector();
                if !items_fields.is_empty() {
                    fields.push_str("(");
                    fields.push_str(&items_fields);
                    fields.push_str(")");
                }
                self.fields = Some(fields);
                crate::iter::PageItemIter::new(self, "assets")
            }
            #[doc = r" Return an iterator that iterates over all `#prop_ident`. The"]
            #[doc = r" items yielded by the iterator are `#items_type`. The populated"]
            #[doc = r" fields in `#items_type` will be the default fields populated by"]
            #[doc = r" the server."]
            pub fn iter_assets_standard(
                mut self,
            ) -> crate::iter::PageItemIter<Self, crate::schemas::Asset> {
                self.fields = Some(concat!("nextPageToken,", "assets").to_owned());
                crate::iter::PageItemIter::new(self, "assets")
            }
            #[doc = r" Return an iterator that iterates over all `#prop_ident`. The"]
            #[doc = r" items yielded by the iterator are `#items_type`. The populated"]
            #[doc = r" fields in `#items_type` will be all fields available. This should"]
            #[doc = r" primarily be used during developement and debugging as fetching"]
            #[doc = r" all fields can be expensive both in bandwidth and server"]
            #[doc = r" resources."]
            pub fn iter_assets_debug(
                mut self,
            ) -> crate::iter::PageItemIter<Self, crate::schemas::Asset> {
                self.fields = Some(concat!("nextPageToken,", "assets", "(*)").to_owned());
                crate::iter::PageItemIter::new(self, "assets")
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
            ) -> crate::iter::PageIter<Self, crate::schemas::ListAssetsResponse> {
                crate::iter::PageIter::new(self)
            }
            pub fn iter_debug(
                mut self,
            ) -> crate::iter::PageIter<Self, crate::schemas::ListAssetsResponse> {
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
            ) -> Result<crate::schemas::ListAssetsResponse, Box<dyn ::std::error::Error>>
            {
                self.execute_fields::<_, &str>(None)
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub fn execute_debug(
                self,
            ) -> Result<crate::schemas::ListAssetsResponse, Box<dyn ::std::error::Error>>
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
                let mut output = "https://poly.googleapis.com/".to_owned();
                output.push_str("v1/assets");
                output
            }
            fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
                let req = self.reqwest.request(::reqwest::Method::GET, path);
                let req = req.query(&[("category", &self.category)]);
                let req = req.query(&[("curated", &self.curated)]);
                let req = req.query(&[("format", &self.format)]);
                let req = req.query(&[("keywords", &self.keywords)]);
                let req = req.query(&[("maxComplexity", &self.max_complexity)]);
                let req = req.query(&[("orderBy", &self.order_by)]);
                let req = req.query(&[("pageSize", &self.page_size)]);
                let req = req.query(&[("pageToken", &self.page_token)]);
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
    pub mod users {
        pub mod params {}
        pub struct UsersActions<'a, A> {
            pub(crate) reqwest: &'a reqwest::Client,
            pub(crate) auth: &'a std::sync::Mutex<A>,
        }
        impl<'a, A: yup_oauth2::GetToken> UsersActions<'a, A> {
            #[doc = "Actions that can be performed on the assets resource"]
            pub fn assets(&self) -> crate::resources::users::assets::AssetsActions<A> {
                crate::resources::users::assets::AssetsActions {
                    reqwest: &self.reqwest,
                    auth: &self.auth,
                }
            }
            #[doc = "Actions that can be performed on the likedassets resource"]
            pub fn likedassets(
                &self,
            ) -> crate::resources::users::likedassets::LikedassetsActions<A> {
                crate::resources::users::likedassets::LikedassetsActions {
                    reqwest: &self.reqwest,
                    auth: &self.auth,
                }
            }
        }
        pub mod assets {
            pub mod params {
                #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
                pub enum ListVisibility {
                    Private,
                    Published,
                    VisibilityUnspecified,
                }
                impl ListVisibility {
                    pub fn as_str(self) -> &'static str {
                        match self {
                            ListVisibility::Private => "PRIVATE",
                            ListVisibility::Published => "PUBLISHED",
                            ListVisibility::VisibilityUnspecified => "VISIBILITY_UNSPECIFIED",
                        }
                    }
                }
                impl ::std::fmt::Display for ListVisibility {
                    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                        f.write_str(self.as_str())
                    }
                }
                impl ::serde::Serialize for ListVisibility {
                    fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
                    where
                        S: ::serde::ser::Serializer,
                    {
                        serializer.serialize_str(self.as_str())
                    }
                }
                impl<'de> ::serde::Deserialize<'de> for ListVisibility {
                    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
                    where
                        D: ::serde::de::Deserializer<'de>,
                    {
                        let value: &'de str = <&str>::deserialize(deserializer)?;
                        Ok(match value {
                            "PRIVATE" => ListVisibility::Private,
                            "PUBLISHED" => ListVisibility::Published,
                            "VISIBILITY_UNSPECIFIED" => ListVisibility::VisibilityUnspecified,
                            _ => {
                                return Err(::serde::de::Error::custom(format!(
                                    "invalid enum for #name: {}",
                                    value
                                )))
                            }
                        })
                    }
                }
                impl ::field_selector::FieldSelector for ListVisibility {
                    fn field_selector_with_ident(ident: &str, selector: &mut String) {
                        match selector.chars().rev().nth(0) {
                            Some(',') | None => {}
                            _ => selector.push_str(","),
                        }
                        selector.push_str(ident);
                    }
                }
            }
            pub struct AssetsActions<'a, A> {
                pub(crate) reqwest: &'a reqwest::Client,
                pub(crate) auth: &'a std::sync::Mutex<A>,
            }
            impl<'a, A: yup_oauth2::GetToken> AssetsActions<'a, A> {
                #[doc = "Lists assets authored by the given user. Only the value 'me', representing\nthe currently-authenticated user, is supported. May include assets with an\naccess level of PRIVATE or\nUNLISTED and assets which are\nAll Rights Reserved for the\ncurrently-authenticated user."]
                pub fn list(&self, name: impl Into<String>) -> ListRequestBuilder<A> {
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
                        name: name.into(),
                        format: None,
                        order_by: None,
                        page_size: None,
                        page_token: None,
                        visibility: None,
                    }
                }
            }
            #[derive(Debug, Clone)]
            pub struct ListRequestBuilder<'a, A> {
                pub(crate) reqwest: &'a ::reqwest::Client,
                pub(crate) auth: &'a ::std::sync::Mutex<A>,
                name: String,
                format: Option<String>,
                order_by: Option<String>,
                page_size: Option<i32>,
                page_token: Option<String>,
                visibility: Option<crate::resources::users::assets::params::ListVisibility>,
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
                #[doc = "Return only assets with the matching format. Acceptable values are:\n`BLOCKS`, `FBX`, `GLTF`, `GLTF2`, `OBJ`, and `TILT`."]
                pub fn format(mut self, value: impl Into<String>) -> Self {
                    self.format = Some(value.into());
                    self
                }
                #[doc = "Specifies an ordering for assets. Acceptable values are:\n`BEST`, `NEWEST`, `OLDEST`. Defaults to `BEST`, which ranks assets\nbased on a combination of popularity and other features."]
                pub fn order_by(mut self, value: impl Into<String>) -> Self {
                    self.order_by = Some(value.into());
                    self
                }
                #[doc = "The maximum number of assets to be returned. This value must be between `1`\nand `100`. Defaults to `20`."]
                pub fn page_size(mut self, value: i32) -> Self {
                    self.page_size = Some(value);
                    self
                }
                #[doc = "Specifies a continuation token from a previous search whose results were\nsplit into multiple pages. To get the next page, submit the same request\nspecifying the value from\nnext_page_token."]
                pub fn page_token(mut self, value: impl Into<String>) -> Self {
                    self.page_token = Some(value.into());
                    self
                }
                #[doc = "The visibility of the assets to be returned.\nDefaults to\nVISIBILITY_UNSPECIFIED\nwhich returns all assets."]
                pub fn visibility(
                    mut self,
                    value: crate::resources::users::assets::params::ListVisibility,
                ) -> Self {
                    self.visibility = Some(value);
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
                pub fn iter_user_assets<T>(mut self) -> crate::iter::PageItemIter<Self, T>
                where
                    T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
                {
                    let mut fields = concat!("nextPageToken,", "userAssets").to_owned();
                    let items_fields = T::field_selector();
                    if !items_fields.is_empty() {
                        fields.push_str("(");
                        fields.push_str(&items_fields);
                        fields.push_str(")");
                    }
                    self.fields = Some(fields);
                    crate::iter::PageItemIter::new(self, "userAssets")
                }
                #[doc = r" Return an iterator that iterates over all `#prop_ident`. The"]
                #[doc = r" items yielded by the iterator are `#items_type`. The populated"]
                #[doc = r" fields in `#items_type` will be the default fields populated by"]
                #[doc = r" the server."]
                pub fn iter_user_assets_standard(
                    mut self,
                ) -> crate::iter::PageItemIter<Self, crate::schemas::UserAsset> {
                    self.fields = Some(concat!("nextPageToken,", "userAssets").to_owned());
                    crate::iter::PageItemIter::new(self, "userAssets")
                }
                #[doc = r" Return an iterator that iterates over all `#prop_ident`. The"]
                #[doc = r" items yielded by the iterator are `#items_type`. The populated"]
                #[doc = r" fields in `#items_type` will be all fields available. This should"]
                #[doc = r" primarily be used during developement and debugging as fetching"]
                #[doc = r" all fields can be expensive both in bandwidth and server"]
                #[doc = r" resources."]
                pub fn iter_user_assets_debug(
                    mut self,
                ) -> crate::iter::PageItemIter<Self, crate::schemas::UserAsset> {
                    self.fields = Some(concat!("nextPageToken,", "userAssets", "(*)").to_owned());
                    crate::iter::PageItemIter::new(self, "userAssets")
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
                ) -> crate::iter::PageIter<Self, crate::schemas::ListUserAssetsResponse>
                {
                    crate::iter::PageIter::new(self)
                }
                pub fn iter_debug(
                    mut self,
                ) -> crate::iter::PageIter<Self, crate::schemas::ListUserAssetsResponse>
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
                ) -> Result<crate::schemas::ListUserAssetsResponse, Box<dyn ::std::error::Error>>
                {
                    self.execute_fields::<_, &str>(None)
                }
                #[doc = r" Execute the given operation. This will provide a `fields`"]
                #[doc = r" selector of `*`. This will include every attribute of the"]
                #[doc = r" response resource and should be limited to use during"]
                #[doc = r" development or debugging."]
                pub fn execute_debug(
                    self,
                ) -> Result<crate::schemas::ListUserAssetsResponse, Box<dyn ::std::error::Error>>
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
                    let mut output = "https://poly.googleapis.com/".to_owned();
                    output.push_str("v1/");
                    {
                        let var_as_str = &self.name;
                        output.extend(::percent_encoding::utf8_percent_encode(
                            &var_as_str,
                            crate::RESERVED,
                        ));
                    }
                    output.push_str("/assets");
                    output
                }
                fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
                    let req = self.reqwest.request(::reqwest::Method::GET, path);
                    let req = req.query(&[("format", &self.format)]);
                    let req = req.query(&[("orderBy", &self.order_by)]);
                    let req = req.query(&[("pageSize", &self.page_size)]);
                    let req = req.query(&[("pageToken", &self.page_token)]);
                    let req = req.query(&[("visibility", &self.visibility)]);
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
        pub mod likedassets {
            pub mod params {}
            pub struct LikedassetsActions<'a, A> {
                pub(crate) reqwest: &'a reqwest::Client,
                pub(crate) auth: &'a std::sync::Mutex<A>,
            }
            impl<'a, A: yup_oauth2::GetToken> LikedassetsActions<'a, A> {
                #[doc = "Lists assets that the user has liked. Only the value 'me', representing\nthe currently-authenticated user, is supported. May include assets with an\naccess level of UNLISTED."]
                pub fn list(&self, name: impl Into<String>) -> ListRequestBuilder<A> {
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
                        name: name.into(),
                        format: None,
                        order_by: None,
                        page_size: None,
                        page_token: None,
                    }
                }
            }
            #[derive(Debug, Clone)]
            pub struct ListRequestBuilder<'a, A> {
                pub(crate) reqwest: &'a ::reqwest::Client,
                pub(crate) auth: &'a ::std::sync::Mutex<A>,
                name: String,
                format: Option<String>,
                order_by: Option<String>,
                page_size: Option<i32>,
                page_token: Option<String>,
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
                #[doc = "Return only assets with the matching format. Acceptable values are:\n`BLOCKS`, `FBX`, `GLTF`, `GLTF2`, `OBJ`, `TILT`."]
                pub fn format(mut self, value: impl Into<String>) -> Self {
                    self.format = Some(value.into());
                    self
                }
                #[doc = "Specifies an ordering for assets. Acceptable values are:\n`BEST`, `NEWEST`, `OLDEST`, 'LIKED_TIME'. Defaults to `LIKED_TIME`, which\nranks assets based on how recently they were liked."]
                pub fn order_by(mut self, value: impl Into<String>) -> Self {
                    self.order_by = Some(value.into());
                    self
                }
                #[doc = "The maximum number of assets to be returned. This value must be between `1`\nand `100`. Defaults to `20`."]
                pub fn page_size(mut self, value: i32) -> Self {
                    self.page_size = Some(value);
                    self
                }
                #[doc = "Specifies a continuation token from a previous search whose results were\nsplit into multiple pages. To get the next page, submit the same request\nspecifying the value from\nnext_page_token."]
                pub fn page_token(mut self, value: impl Into<String>) -> Self {
                    self.page_token = Some(value.into());
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
                pub fn iter_assets<T>(mut self) -> crate::iter::PageItemIter<Self, T>
                where
                    T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
                {
                    let mut fields = concat!("nextPageToken,", "assets").to_owned();
                    let items_fields = T::field_selector();
                    if !items_fields.is_empty() {
                        fields.push_str("(");
                        fields.push_str(&items_fields);
                        fields.push_str(")");
                    }
                    self.fields = Some(fields);
                    crate::iter::PageItemIter::new(self, "assets")
                }
                #[doc = r" Return an iterator that iterates over all `#prop_ident`. The"]
                #[doc = r" items yielded by the iterator are `#items_type`. The populated"]
                #[doc = r" fields in `#items_type` will be the default fields populated by"]
                #[doc = r" the server."]
                pub fn iter_assets_standard(
                    mut self,
                ) -> crate::iter::PageItemIter<Self, crate::schemas::Asset> {
                    self.fields = Some(concat!("nextPageToken,", "assets").to_owned());
                    crate::iter::PageItemIter::new(self, "assets")
                }
                #[doc = r" Return an iterator that iterates over all `#prop_ident`. The"]
                #[doc = r" items yielded by the iterator are `#items_type`. The populated"]
                #[doc = r" fields in `#items_type` will be all fields available. This should"]
                #[doc = r" primarily be used during developement and debugging as fetching"]
                #[doc = r" all fields can be expensive both in bandwidth and server"]
                #[doc = r" resources."]
                pub fn iter_assets_debug(
                    mut self,
                ) -> crate::iter::PageItemIter<Self, crate::schemas::Asset> {
                    self.fields = Some(concat!("nextPageToken,", "assets", "(*)").to_owned());
                    crate::iter::PageItemIter::new(self, "assets")
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
                ) -> crate::iter::PageIter<Self, crate::schemas::ListLikedAssetsResponse>
                {
                    crate::iter::PageIter::new(self)
                }
                pub fn iter_debug(
                    mut self,
                ) -> crate::iter::PageIter<Self, crate::schemas::ListLikedAssetsResponse>
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
                ) -> Result<crate::schemas::ListLikedAssetsResponse, Box<dyn ::std::error::Error>>
                {
                    self.execute_fields::<_, &str>(None)
                }
                #[doc = r" Execute the given operation. This will provide a `fields`"]
                #[doc = r" selector of `*`. This will include every attribute of the"]
                #[doc = r" response resource and should be limited to use during"]
                #[doc = r" development or debugging."]
                pub fn execute_debug(
                    self,
                ) -> Result<crate::schemas::ListLikedAssetsResponse, Box<dyn ::std::error::Error>>
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
                    let mut output = "https://poly.googleapis.com/".to_owned();
                    output.push_str("v1/");
                    {
                        let var_as_str = &self.name;
                        output.extend(::percent_encoding::utf8_percent_encode(
                            &var_as_str,
                            crate::RESERVED,
                        ));
                    }
                    output.push_str("/likedassets");
                    output
                }
                fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
                    let req = self.reqwest.request(::reqwest::Method::GET, path);
                    let req = req.query(&[("format", &self.format)]);
                    let req = req.query(&[("orderBy", &self.order_by)]);
                    let req = req.query(&[("pageSize", &self.page_size)]);
                    let req = req.query(&[("pageToken", &self.page_token)]);
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
