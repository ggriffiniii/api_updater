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
    pub struct MpdGetResponseCaptionsItems {
        #[serde(rename = "format", default)]
        #[serde(with = "crate::parsed_string")]
        pub format: ::std::option::Option<i64>,
        #[serde(rename = "lang", default)]
        pub lang: ::std::option::Option<String>,
    }
    impl ::field_selector::FieldSelector for MpdGetResponseCaptionsItems {
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
    pub struct MpdGetResponseHlsMasterPlaylistsItems {
        #[serde(rename = "hls_url", default)]
        pub hls_url: ::std::option::Option<String>,
        #[serde(rename = "max_height", default)]
        pub max_height: ::std::option::Option<i32>,
        #[serde(rename = "max_width", default)]
        pub max_width: ::std::option::Option<i32>,
    }
    impl ::field_selector::FieldSelector for MpdGetResponseHlsMasterPlaylistsItems {
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
    pub struct MpdGetResponsePostrollCardsItemsCaptionsItems {
        #[serde(rename = "format", default)]
        #[serde(with = "crate::parsed_string")]
        pub format: ::std::option::Option<i64>,
        #[serde(rename = "lang", default)]
        pub lang: ::std::option::Option<String>,
    }
    impl ::field_selector::FieldSelector for MpdGetResponsePostrollCardsItemsCaptionsItems {
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
    pub struct MpdGetResponsePostrollCardsItemsRepresentationsItemsAudioInfo {
        #[serde(rename = "language", default)]
        pub language: ::std::option::Option<String>,
        #[serde(rename = "language_type", default)]
        pub language_type: ::std::option::Option<String>,
    }
    impl ::field_selector::FieldSelector
        for MpdGetResponsePostrollCardsItemsRepresentationsItemsAudioInfo
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
    pub struct MpdGetResponsePostrollCardsItemsRepresentationsItemsIndex {
        #[serde(rename = "first", default)]
        pub first: ::std::option::Option<i32>,
        #[serde(rename = "last", default)]
        pub last: ::std::option::Option<i32>,
    }
    impl ::field_selector::FieldSelector for MpdGetResponsePostrollCardsItemsRepresentationsItemsIndex {
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
    pub struct MpdGetResponsePostrollCardsItemsRepresentationsItemsInit {
        #[serde(rename = "first", default)]
        pub first: ::std::option::Option<i32>,
        #[serde(rename = "last", default)]
        pub last: ::std::option::Option<i32>,
    }
    impl ::field_selector::FieldSelector for MpdGetResponsePostrollCardsItemsRepresentationsItemsInit {
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
    pub struct MpdGetResponsePostrollCardsItemsRepresentationsItems {
        #[serde(rename = "audio_info", default)]
        pub audio_info: ::std::option::Option<
            crate::schemas::MpdGetResponsePostrollCardsItemsRepresentationsItemsAudioInfo,
        >,
        #[serde(rename = "bitrate", default)]
        pub bitrate: ::std::option::Option<i32>,
        #[serde(rename = "codec", default)]
        pub codec: ::std::option::Option<String>,
        #[serde(rename = "duration", default)]
        pub duration: ::std::option::Option<f64>,
        #[serde(rename = "file_size", default)]
        #[serde(with = "crate::parsed_string")]
        pub file_size: ::std::option::Option<i64>,
        #[serde(rename = "format", default)]
        pub format: ::std::option::Option<i32>,
        #[serde(rename = "index", default)]
        pub index: ::std::option::Option<
            crate::schemas::MpdGetResponsePostrollCardsItemsRepresentationsItemsIndex,
        >,
        #[serde(rename = "init", default)]
        pub init: ::std::option::Option<
            crate::schemas::MpdGetResponsePostrollCardsItemsRepresentationsItemsInit,
        >,
        #[serde(rename = "playback_url", default)]
        pub playback_url: ::std::option::Option<String>,
        #[serde(rename = "timestamp_millis", default)]
        #[serde(with = "crate::parsed_string")]
        pub timestamp_millis: ::std::option::Option<i64>,
    }
    impl ::field_selector::FieldSelector for MpdGetResponsePostrollCardsItemsRepresentationsItems {
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
    pub struct MpdGetResponsePostrollCardsItemsResource {
        #[serde(rename = "id", default)]
        pub id: ::std::option::Option<String>,
        #[serde(rename = "mid", default)]
        pub mid: ::std::option::Option<String>,
    }
    impl ::field_selector::FieldSelector for MpdGetResponsePostrollCardsItemsResource {
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
    pub struct MpdGetResponsePostrollCardsItems {
        #[doc = "The captions"]
        #[serde(rename = "captions", default)]
        pub captions: ::std::option::Option<
            Vec<crate::schemas::MpdGetResponsePostrollCardsItemsCaptionsItems>,
        >,
        #[doc = "The Etag"]
        #[serde(rename = "etag", default)]
        pub etag: ::std::option::Option<String>,
        #[doc = "The HLS URL"]
        #[serde(rename = "hls_url", default)]
        pub hls_url: ::std::option::Option<String>,
        #[doc = "The kind of object returned - Mpd"]
        #[serde(rename = "kind", default)]
        pub kind: ::std::option::Option<String>,
        #[doc = "The representations"]
        #[serde(rename = "representations", default)]
        pub representations: ::std::option::Option<
            Vec<crate::schemas::MpdGetResponsePostrollCardsItemsRepresentationsItems>,
        >,
        #[serde(rename = "resource", default)]
        pub resource:
            ::std::option::Option<crate::schemas::MpdGetResponsePostrollCardsItemsResource>,
        #[doc = "The manifest"]
        #[serde(rename = "xml", default)]
        pub xml: ::std::option::Option<String>,
    }
    impl ::field_selector::FieldSelector for MpdGetResponsePostrollCardsItems {
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
    pub struct MpdGetResponseRepresentationsItemsAudioInfo {
        #[serde(rename = "language", default)]
        pub language: ::std::option::Option<String>,
        #[serde(rename = "language_type", default)]
        pub language_type: ::std::option::Option<String>,
    }
    impl ::field_selector::FieldSelector for MpdGetResponseRepresentationsItemsAudioInfo {
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
    pub struct MpdGetResponseRepresentationsItemsIndex {
        #[serde(rename = "first", default)]
        pub first: ::std::option::Option<i32>,
        #[serde(rename = "last", default)]
        pub last: ::std::option::Option<i32>,
    }
    impl ::field_selector::FieldSelector for MpdGetResponseRepresentationsItemsIndex {
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
    pub struct MpdGetResponseRepresentationsItemsInit {
        #[serde(rename = "first", default)]
        pub first: ::std::option::Option<i32>,
        #[serde(rename = "last", default)]
        pub last: ::std::option::Option<i32>,
    }
    impl ::field_selector::FieldSelector for MpdGetResponseRepresentationsItemsInit {
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
    pub struct MpdGetResponseRepresentationsItems {
        #[serde(rename = "audio_info", default)]
        pub audio_info:
            ::std::option::Option<crate::schemas::MpdGetResponseRepresentationsItemsAudioInfo>,
        #[serde(rename = "bitrate", default)]
        pub bitrate: ::std::option::Option<i32>,
        #[serde(rename = "codec", default)]
        pub codec: ::std::option::Option<String>,
        #[serde(rename = "duration", default)]
        pub duration: ::std::option::Option<f64>,
        #[serde(rename = "file_size", default)]
        #[serde(with = "crate::parsed_string")]
        pub file_size: ::std::option::Option<i64>,
        #[serde(rename = "format", default)]
        pub format: ::std::option::Option<i32>,
        #[serde(rename = "height", default)]
        pub height: ::std::option::Option<i32>,
        #[serde(rename = "immersive", default)]
        pub immersive: ::std::option::Option<bool>,
        #[serde(rename = "index", default)]
        pub index: ::std::option::Option<crate::schemas::MpdGetResponseRepresentationsItemsIndex>,
        #[serde(rename = "init", default)]
        pub init: ::std::option::Option<crate::schemas::MpdGetResponseRepresentationsItemsInit>,
        #[serde(rename = "playback_url", default)]
        pub playback_url: ::std::option::Option<String>,
        #[serde(rename = "stereo_layout", default)]
        pub stereo_layout: ::std::option::Option<i32>,
        #[serde(rename = "timestamp_millis", default)]
        #[serde(with = "crate::parsed_string")]
        pub timestamp_millis: ::std::option::Option<i64>,
        #[serde(rename = "video_fps", default)]
        pub video_fps: ::std::option::Option<f64>,
        #[serde(rename = "width", default)]
        pub width: ::std::option::Option<i32>,
    }
    impl ::field_selector::FieldSelector for MpdGetResponseRepresentationsItems {
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
    pub struct MpdGetResponseResource {
        #[serde(rename = "id", default)]
        pub id: ::std::option::Option<String>,
        #[serde(rename = "mid", default)]
        pub mid: ::std::option::Option<String>,
    }
    impl ::field_selector::FieldSelector for MpdGetResponseResource {
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
    pub struct MpdGetResponse {
        #[doc = "The captions"]
        #[serde(rename = "captions", default)]
        pub captions: ::std::option::Option<Vec<crate::schemas::MpdGetResponseCaptionsItems>>,
        #[doc = "The Etag"]
        #[serde(rename = "etag", default)]
        pub etag: ::std::option::Option<String>,
        #[doc = "The HLS master playlists"]
        #[serde(rename = "hls_master_playlists", default)]
        pub hls_master_playlists:
            ::std::option::Option<Vec<crate::schemas::MpdGetResponseHlsMasterPlaylistsItems>>,
        #[doc = "The (deprecated) HLS URL"]
        #[serde(rename = "hls_url", default)]
        pub hls_url: ::std::option::Option<String>,
        #[doc = "The kind of object returned - Mpd"]
        #[serde(rename = "kind", default)]
        pub kind: ::std::option::Option<String>,
        #[doc = "The post-roll card MPDs (ie. dub cards)"]
        #[serde(rename = "postroll_cards", default)]
        pub postroll_cards:
            ::std::option::Option<Vec<crate::schemas::MpdGetResponsePostrollCardsItems>>,
        #[doc = "The representations"]
        #[serde(rename = "representations", default)]
        pub representations:
            ::std::option::Option<Vec<crate::schemas::MpdGetResponseRepresentationsItems>>,
        #[serde(rename = "resource", default)]
        pub resource: ::std::option::Option<crate::schemas::MpdGetResponseResource>,
        #[doc = "The manifest"]
        #[serde(rename = "xml", default)]
        pub xml: ::std::option::Option<String>,
    }
    impl ::field_selector::FieldSelector for MpdGetResponse {
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
    pub struct MpdUrlGetResponse {
        #[doc = "The Etag"]
        #[serde(rename = "etag", default)]
        pub etag: ::std::option::Option<String>,
        #[doc = "The kind of object returned - MpdUrl"]
        #[serde(rename = "kind", default)]
        pub kind: ::std::option::Option<String>,
        #[doc = "The url"]
        #[serde(rename = "url", default)]
        pub url: ::std::option::Option<String>,
    }
    impl ::field_selector::FieldSelector for MpdUrlGetResponse {
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
        #[doc = "Responses with Content-Type of application/x-protobuf"]
        Proto,
    }
    impl Alt {
        pub fn as_str(self) -> &'static str {
            match self {
                Alt::Json => "json",
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
    #[doc = "Actions that can be performed on the accountlink resource"]
    pub fn accountlink(&self) -> crate::resources::accountlink::AccountlinkActions<A> {
        crate::resources::accountlink::AccountlinkActions {
            reqwest: &self.reqwest,
            auth: &self.auth,
        }
    }
    #[doc = "Actions that can be performed on the asset resource"]
    pub fn asset(&self) -> crate::resources::asset::AssetActions<A> {
        crate::resources::asset::AssetActions {
            reqwest: &self.reqwest,
            auth: &self.auth,
        }
    }
    #[doc = "Actions that can be performed on the assetreview resource"]
    pub fn assetreview(&self) -> crate::resources::assetreview::AssetreviewActions<A> {
        crate::resources::assetreview::AssetreviewActions {
            reqwest: &self.reqwest,
            auth: &self.auth,
        }
    }
    #[doc = "Actions that can be performed on the category resource"]
    pub fn category(&self) -> crate::resources::category::CategoryActions<A> {
        crate::resources::category::CategoryActions {
            reqwest: &self.reqwest,
            auth: &self.auth,
        }
    }
    #[doc = "Actions that can be performed on the collection resource"]
    pub fn collection(&self) -> crate::resources::collection::CollectionActions<A> {
        crate::resources::collection::CollectionActions {
            reqwest: &self.reqwest,
            auth: &self.auth,
        }
    }
    #[doc = "Actions that can be performed on the device resource"]
    pub fn device(&self) -> crate::resources::device::DeviceActions<A> {
        crate::resources::device::DeviceActions {
            reqwest: &self.reqwest,
            auth: &self.auth,
        }
    }
    #[doc = "Actions that can be performed on the gunsproxy resource"]
    pub fn gunsproxy(&self) -> crate::resources::gunsproxy::GunsproxyActions<A> {
        crate::resources::gunsproxy::GunsproxyActions {
            reqwest: &self.reqwest,
            auth: &self.auth,
        }
    }
    #[doc = "Actions that can be performed on the mpd resource"]
    pub fn mpd(&self) -> crate::resources::mpd::MpdActions<A> {
        crate::resources::mpd::MpdActions {
            reqwest: &self.reqwest,
            auth: &self.auth,
        }
    }
    #[doc = "Actions that can be performed on the mpdurl resource"]
    pub fn mpdurl(&self) -> crate::resources::mpdurl::MpdurlActions<A> {
        crate::resources::mpdurl::MpdurlActions {
            reqwest: &self.reqwest,
            auth: &self.auth,
        }
    }
    #[doc = "Actions that can be performed on the promotion resource"]
    pub fn promotion(&self) -> crate::resources::promotion::PromotionActions<A> {
        crate::resources::promotion::PromotionActions {
            reqwest: &self.reqwest,
            auth: &self.auth,
        }
    }
    #[doc = "Actions that can be performed on the purchase resource"]
    pub fn purchase(&self) -> crate::resources::purchase::PurchaseActions<A> {
        crate::resources::purchase::PurchaseActions {
            reqwest: &self.reqwest,
            auth: &self.auth,
        }
    }
    #[doc = "Actions that can be performed on the recommendation resource"]
    pub fn recommendation(&self) -> crate::resources::recommendation::RecommendationActions<A> {
        crate::resources::recommendation::RecommendationActions {
            reqwest: &self.reqwest,
            auth: &self.auth,
        }
    }
    #[doc = "Actions that can be performed on the search resource"]
    pub fn search(&self) -> crate::resources::search::SearchActions<A> {
        crate::resources::search::SearchActions {
            reqwest: &self.reqwest,
            auth: &self.auth,
        }
    }
    #[doc = "Actions that can be performed on the userconfig resource"]
    pub fn userconfig(&self) -> crate::resources::userconfig::UserconfigActions<A> {
        crate::resources::userconfig::UserconfigActions {
            reqwest: &self.reqwest,
            auth: &self.auth,
        }
    }
    #[doc = "Actions that can be performed on the userlibrary resource"]
    pub fn userlibrary(&self) -> crate::resources::userlibrary::UserlibraryActions<A> {
        crate::resources::userlibrary::UserlibraryActions {
            reqwest: &self.reqwest,
            auth: &self.auth,
        }
    }
    #[doc = "Actions that can be performed on the usersetting resource"]
    pub fn usersetting(&self) -> crate::resources::usersetting::UsersettingActions<A> {
        crate::resources::usersetting::UsersettingActions {
            reqwest: &self.reqwest,
            auth: &self.auth,
        }
    }
    #[doc = "Actions that can be performed on the video resource"]
    pub fn video(&self) -> crate::resources::video::VideoActions<A> {
        crate::resources::video::VideoActions {
            reqwest: &self.reqwest,
            auth: &self.auth,
        }
    }
    #[doc = "Actions that can be performed on the wishlistitem resource"]
    pub fn wishlistitem(&self) -> crate::resources::wishlistitem::WishlistitemActions<A> {
        crate::resources::wishlistitem::WishlistitemActions {
            reqwest: &self.reqwest,
            auth: &self.auth,
        }
    }
}
pub mod resources {
    pub mod accountlink {
        pub mod params {}
        pub struct AccountlinkActions<'a, A> {
            pub(crate) reqwest: &'a reqwest::Client,
            pub(crate) auth: &'a std::sync::Mutex<A>,
        }
        impl<'a, A: yup_oauth2::GetToken> AccountlinkActions<'a, A> {
            #[doc = ""]
            pub fn delete(&self) -> DeleteRequestBuilder<A> {
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
                    pid: None,
                }
            }
        }
        #[derive(Debug, Clone)]
        pub struct DeleteRequestBuilder<'a, A> {
            pub(crate) reqwest: &'a ::reqwest::Client,
            pub(crate) auth: &'a ::std::sync::Mutex<A>,
            pid: Option<String>,
            alt: Option<crate::params::Alt>,
            fields: Option<String>,
            key: Option<String>,
            oauth_token: Option<String>,
            pretty_print: Option<bool>,
            quota_user: Option<String>,
            user_ip: Option<String>,
        }
        impl<'a, A: yup_oauth2::GetToken> DeleteRequestBuilder<'a, A> {
            #[doc = ""]
            pub fn pid(mut self, value: impl Into<String>) -> Self {
                self.pid = Some(value.into());
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
                let mut output = "https://www.googleapis.com/android_video/v1/".to_owned();
                output.push_str("accountlink");
                output
            }
            fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
                let req = self.reqwest.request(::reqwest::Method::DELETE, path);
                let req = req.query(&[("pid", &self.pid)]);
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
    pub mod asset {
        pub mod params {}
        pub struct AssetActions<'a, A> {
            pub(crate) reqwest: &'a reqwest::Client,
            pub(crate) auth: &'a std::sync::Mutex<A>,
        }
        impl<'a, A: yup_oauth2::GetToken> AssetActions<'a, A> {
            #[doc = ""]
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
                    apptype: None,
                    b: None,
                    cr: None,
                    device: None,
                    devid: None,
                    devtype: None,
                    exp: None,
                    i: None,
                    id: None,
                    lr: None,
                    m: None,
                    make: None,
                    model: None,
                    o: None,
                    product: None,
                    r: None,
                    r#if: None,
                    sn: None,
                }
            }
        }
        #[derive(Debug, Clone)]
        pub struct ListRequestBuilder<'a, A> {
            pub(crate) reqwest: &'a ::reqwest::Client,
            pub(crate) auth: &'a ::std::sync::Mutex<A>,
            apptype: Option<String>,
            b: Option<bool>,
            cr: Option<String>,
            device: Option<String>,
            devid: Option<String>,
            devtype: Option<String>,
            exp: Option<String>,
            i: Option<bool>,
            id: Option<String>,
            lr: Option<String>,
            m: Option<bool>,
            make: Option<String>,
            model: Option<String>,
            o: Option<bool>,
            product: Option<String>,
            r: Option<bool>,
            r#if: Option<String>,
            sn: Option<String>,
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
            pub fn apptype(mut self, value: impl Into<String>) -> Self {
                self.apptype = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn b(mut self, value: bool) -> Self {
                self.b = Some(value);
                self
            }
            #[doc = ""]
            pub fn cr(mut self, value: impl Into<String>) -> Self {
                self.cr = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn device(mut self, value: impl Into<String>) -> Self {
                self.device = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn devid(mut self, value: impl Into<String>) -> Self {
                self.devid = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn devtype(mut self, value: impl Into<String>) -> Self {
                self.devtype = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn exp(mut self, value: impl Into<String>) -> Self {
                self.exp = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn i(mut self, value: bool) -> Self {
                self.i = Some(value);
                self
            }
            #[doc = ""]
            pub fn id(mut self, value: impl Into<String>) -> Self {
                self.id = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn lr(mut self, value: impl Into<String>) -> Self {
                self.lr = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn m(mut self, value: bool) -> Self {
                self.m = Some(value);
                self
            }
            #[doc = ""]
            pub fn make(mut self, value: impl Into<String>) -> Self {
                self.make = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn model(mut self, value: impl Into<String>) -> Self {
                self.model = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn o(mut self, value: bool) -> Self {
                self.o = Some(value);
                self
            }
            #[doc = ""]
            pub fn product(mut self, value: impl Into<String>) -> Self {
                self.product = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn r(mut self, value: bool) -> Self {
                self.r = Some(value);
                self
            }
            #[doc = ""]
            pub fn r#if(mut self, value: impl Into<String>) -> Self {
                self.r#if = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn sn(mut self, value: impl Into<String>) -> Self {
                self.sn = Some(value.into());
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
                let mut output = "https://www.googleapis.com/android_video/v1/".to_owned();
                output.push_str("asset/list");
                output
            }
            fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
                let req = self.reqwest.request(::reqwest::Method::GET, path);
                let req = req.query(&[("apptype", &self.apptype)]);
                let req = req.query(&[("b", &self.b)]);
                let req = req.query(&[("cr", &self.cr)]);
                let req = req.query(&[("device", &self.device)]);
                let req = req.query(&[("devid", &self.devid)]);
                let req = req.query(&[("devtype", &self.devtype)]);
                let req = req.query(&[("exp", &self.exp)]);
                let req = req.query(&[("i", &self.i)]);
                let req = req.query(&[("id", &self.id)]);
                let req = req.query(&[("lr", &self.lr)]);
                let req = req.query(&[("m", &self.m)]);
                let req = req.query(&[("make", &self.make)]);
                let req = req.query(&[("model", &self.model)]);
                let req = req.query(&[("o", &self.o)]);
                let req = req.query(&[("product", &self.product)]);
                let req = req.query(&[("r", &self.r)]);
                let req = req.query(&[("if", &self.r#if)]);
                let req = req.query(&[("sn", &self.sn)]);
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
    pub mod assetreview {
        pub mod params {}
        pub struct AssetreviewActions<'a, A> {
            pub(crate) reqwest: &'a reqwest::Client,
            pub(crate) auth: &'a std::sync::Mutex<A>,
        }
        impl<'a, A: yup_oauth2::GetToken> AssetreviewActions<'a, A> {
            #[doc = ""]
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
                    aid: None,
                    apptype: None,
                    at: None,
                    device: None,
                    devid: None,
                    devtype: None,
                    exp: None,
                    make: None,
                    max_results: None,
                    model: None,
                    page_token: None,
                    product: None,
                    sn: None,
                }
            }
        }
        #[derive(Debug, Clone)]
        pub struct ListRequestBuilder<'a, A> {
            pub(crate) reqwest: &'a ::reqwest::Client,
            pub(crate) auth: &'a ::std::sync::Mutex<A>,
            aid: Option<String>,
            apptype: Option<String>,
            at: Option<String>,
            device: Option<String>,
            devid: Option<String>,
            devtype: Option<String>,
            exp: Option<String>,
            make: Option<String>,
            max_results: Option<u32>,
            model: Option<String>,
            page_token: Option<String>,
            product: Option<String>,
            sn: Option<String>,
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
            pub fn aid(mut self, value: impl Into<String>) -> Self {
                self.aid = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn apptype(mut self, value: impl Into<String>) -> Self {
                self.apptype = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn at(mut self, value: impl Into<String>) -> Self {
                self.at = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn device(mut self, value: impl Into<String>) -> Self {
                self.device = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn devid(mut self, value: impl Into<String>) -> Self {
                self.devid = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn devtype(mut self, value: impl Into<String>) -> Self {
                self.devtype = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn exp(mut self, value: impl Into<String>) -> Self {
                self.exp = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn make(mut self, value: impl Into<String>) -> Self {
                self.make = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn max_results(mut self, value: u32) -> Self {
                self.max_results = Some(value);
                self
            }
            #[doc = ""]
            pub fn model(mut self, value: impl Into<String>) -> Self {
                self.model = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn page_token(mut self, value: impl Into<String>) -> Self {
                self.page_token = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn product(mut self, value: impl Into<String>) -> Self {
                self.product = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn sn(mut self, value: impl Into<String>) -> Self {
                self.sn = Some(value.into());
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
                let mut output = "https://www.googleapis.com/android_video/v1/".to_owned();
                output.push_str("review/list");
                output
            }
            fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
                let req = self.reqwest.request(::reqwest::Method::GET, path);
                let req = req.query(&[("aid", &self.aid)]);
                let req = req.query(&[("apptype", &self.apptype)]);
                let req = req.query(&[("at", &self.at)]);
                let req = req.query(&[("device", &self.device)]);
                let req = req.query(&[("devid", &self.devid)]);
                let req = req.query(&[("devtype", &self.devtype)]);
                let req = req.query(&[("exp", &self.exp)]);
                let req = req.query(&[("make", &self.make)]);
                let req = req.query(&[("maxResults", &self.max_results)]);
                let req = req.query(&[("model", &self.model)]);
                let req = req.query(&[("pageToken", &self.page_token)]);
                let req = req.query(&[("product", &self.product)]);
                let req = req.query(&[("sn", &self.sn)]);
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
    pub mod category {
        pub mod params {}
        pub struct CategoryActions<'a, A> {
            pub(crate) reqwest: &'a reqwest::Client,
            pub(crate) auth: &'a std::sync::Mutex<A>,
        }
        impl<'a, A: yup_oauth2::GetToken> CategoryActions<'a, A> {
            #[doc = ""]
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
                    apptype: None,
                    cr: None,
                    device: None,
                    devid: None,
                    devtype: None,
                    exp: None,
                    lr: None,
                    make: None,
                    model: None,
                    product: None,
                    r#type: None,
                    sn: None,
                }
            }
        }
        #[derive(Debug, Clone)]
        pub struct ListRequestBuilder<'a, A> {
            pub(crate) reqwest: &'a ::reqwest::Client,
            pub(crate) auth: &'a ::std::sync::Mutex<A>,
            apptype: Option<String>,
            cr: Option<String>,
            device: Option<String>,
            devid: Option<String>,
            devtype: Option<String>,
            exp: Option<String>,
            lr: Option<String>,
            make: Option<String>,
            model: Option<String>,
            product: Option<String>,
            r#type: Option<String>,
            sn: Option<String>,
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
            pub fn apptype(mut self, value: impl Into<String>) -> Self {
                self.apptype = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn cr(mut self, value: impl Into<String>) -> Self {
                self.cr = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn device(mut self, value: impl Into<String>) -> Self {
                self.device = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn devid(mut self, value: impl Into<String>) -> Self {
                self.devid = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn devtype(mut self, value: impl Into<String>) -> Self {
                self.devtype = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn exp(mut self, value: impl Into<String>) -> Self {
                self.exp = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn lr(mut self, value: impl Into<String>) -> Self {
                self.lr = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn make(mut self, value: impl Into<String>) -> Self {
                self.make = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn model(mut self, value: impl Into<String>) -> Self {
                self.model = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn product(mut self, value: impl Into<String>) -> Self {
                self.product = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn r#type(mut self, value: impl Into<String>) -> Self {
                self.r#type = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn sn(mut self, value: impl Into<String>) -> Self {
                self.sn = Some(value.into());
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
                let mut output = "https://www.googleapis.com/android_video/v1/".to_owned();
                output.push_str("category");
                output
            }
            fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
                let req = self.reqwest.request(::reqwest::Method::GET, path);
                let req = req.query(&[("apptype", &self.apptype)]);
                let req = req.query(&[("cr", &self.cr)]);
                let req = req.query(&[("device", &self.device)]);
                let req = req.query(&[("devid", &self.devid)]);
                let req = req.query(&[("devtype", &self.devtype)]);
                let req = req.query(&[("exp", &self.exp)]);
                let req = req.query(&[("lr", &self.lr)]);
                let req = req.query(&[("make", &self.make)]);
                let req = req.query(&[("model", &self.model)]);
                let req = req.query(&[("product", &self.product)]);
                let req = req.query(&[("type", &self.r#type)]);
                let req = req.query(&[("sn", &self.sn)]);
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
    pub mod collection {
        pub mod params {}
        pub struct CollectionActions<'a, A> {
            pub(crate) reqwest: &'a reqwest::Client,
            pub(crate) auth: &'a std::sync::Mutex<A>,
        }
        impl<'a, A: yup_oauth2::GetToken> CollectionActions<'a, A> {
            #[doc = ""]
            pub fn get(&self) -> GetRequestBuilder<A> {
                GetRequestBuilder {
                    reqwest: &self.reqwest,
                    auth: &self.auth,
                    alt: None,
                    fields: None,
                    key: None,
                    oauth_token: None,
                    pretty_print: None,
                    quota_user: None,
                    user_ip: None,
                    apptype: None,
                    cat: None,
                    cid: None,
                    cr: None,
                    device: None,
                    devid: None,
                    devtype: None,
                    exp: None,
                    fs: None,
                    lr: None,
                    make: None,
                    max_results: None,
                    mc: None,
                    model: None,
                    mrf: None,
                    page_token: None,
                    product: None,
                    sn: None,
                    stags: None,
                    stale: None,
                    tagdbid: None,
                    tagtype: None,
                    tvrf: None,
                    utoken: None,
                }
            }
            #[doc = ""]
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
                    apptype: None,
                    cat: None,
                    cr: None,
                    device: None,
                    devid: None,
                    devtype: None,
                    exp: None,
                    fs: None,
                    lr: None,
                    make: None,
                    max_results: None,
                    mc: None,
                    model: None,
                    mrf: None,
                    page_token: None,
                    product: None,
                    sn: None,
                    stale: None,
                    tvrf: None,
                    utoken: None,
                }
            }
            #[doc = ""]
            pub fn paginatebytoken(&self) -> PaginatebytokenRequestBuilder<A> {
                PaginatebytokenRequestBuilder {
                    reqwest: &self.reqwest,
                    auth: &self.auth,
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
        pub struct GetRequestBuilder<'a, A> {
            pub(crate) reqwest: &'a ::reqwest::Client,
            pub(crate) auth: &'a ::std::sync::Mutex<A>,
            apptype: Option<String>,
            cat: Option<String>,
            cid: Option<String>,
            cr: Option<String>,
            device: Option<String>,
            devid: Option<String>,
            devtype: Option<String>,
            exp: Option<String>,
            fs: Option<String>,
            lr: Option<String>,
            make: Option<String>,
            max_results: Option<u32>,
            mc: Option<u32>,
            model: Option<String>,
            mrf: Option<String>,
            page_token: Option<String>,
            product: Option<String>,
            sn: Option<String>,
            stags: Option<Vec<String>>,
            stale: Option<i32>,
            tagdbid: Option<String>,
            tagtype: Option<String>,
            tvrf: Option<String>,
            utoken: Option<String>,
            alt: Option<crate::params::Alt>,
            fields: Option<String>,
            key: Option<String>,
            oauth_token: Option<String>,
            pretty_print: Option<bool>,
            quota_user: Option<String>,
            user_ip: Option<String>,
        }
        impl<'a, A: yup_oauth2::GetToken> GetRequestBuilder<'a, A> {
            #[doc = ""]
            pub fn apptype(mut self, value: impl Into<String>) -> Self {
                self.apptype = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn cat(mut self, value: impl Into<String>) -> Self {
                self.cat = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn cid(mut self, value: impl Into<String>) -> Self {
                self.cid = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn cr(mut self, value: impl Into<String>) -> Self {
                self.cr = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn device(mut self, value: impl Into<String>) -> Self {
                self.device = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn devid(mut self, value: impl Into<String>) -> Self {
                self.devid = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn devtype(mut self, value: impl Into<String>) -> Self {
                self.devtype = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn exp(mut self, value: impl Into<String>) -> Self {
                self.exp = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn fs(mut self, value: impl Into<String>) -> Self {
                self.fs = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn lr(mut self, value: impl Into<String>) -> Self {
                self.lr = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn make(mut self, value: impl Into<String>) -> Self {
                self.make = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn max_results(mut self, value: u32) -> Self {
                self.max_results = Some(value);
                self
            }
            #[doc = ""]
            pub fn mc(mut self, value: u32) -> Self {
                self.mc = Some(value);
                self
            }
            #[doc = ""]
            pub fn model(mut self, value: impl Into<String>) -> Self {
                self.model = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn mrf(mut self, value: impl Into<String>) -> Self {
                self.mrf = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn page_token(mut self, value: impl Into<String>) -> Self {
                self.page_token = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn product(mut self, value: impl Into<String>) -> Self {
                self.product = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn sn(mut self, value: impl Into<String>) -> Self {
                self.sn = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn stags(mut self, value: impl Into<Vec<String>>) -> Self {
                self.stags = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn stale(mut self, value: i32) -> Self {
                self.stale = Some(value);
                self
            }
            #[doc = ""]
            pub fn tagdbid(mut self, value: impl Into<String>) -> Self {
                self.tagdbid = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn tagtype(mut self, value: impl Into<String>) -> Self {
                self.tagtype = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn tvrf(mut self, value: impl Into<String>) -> Self {
                self.tvrf = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn utoken(mut self, value: impl Into<String>) -> Self {
                self.utoken = Some(value.into());
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
                let mut output = "https://www.googleapis.com/android_video/v1/".to_owned();
                output.push_str("collection");
                output
            }
            fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
                let req = self.reqwest.request(::reqwest::Method::GET, path);
                let req = req.query(&[("apptype", &self.apptype)]);
                let req = req.query(&[("cat", &self.cat)]);
                let req = req.query(&[("cid", &self.cid)]);
                let req = req.query(&[("cr", &self.cr)]);
                let req = req.query(&[("device", &self.device)]);
                let req = req.query(&[("devid", &self.devid)]);
                let req = req.query(&[("devtype", &self.devtype)]);
                let req = req.query(&[("exp", &self.exp)]);
                let req = req.query(&[("fs", &self.fs)]);
                let req = req.query(&[("lr", &self.lr)]);
                let req = req.query(&[("make", &self.make)]);
                let req = req.query(&[("maxResults", &self.max_results)]);
                let req = req.query(&[("mc", &self.mc)]);
                let req = req.query(&[("model", &self.model)]);
                let req = req.query(&[("mrf", &self.mrf)]);
                let req = req.query(&[("pageToken", &self.page_token)]);
                let req = req.query(&[("product", &self.product)]);
                let req = req.query(&[("sn", &self.sn)]);
                let req = req.query(&[("stags", &self.stags)]);
                let req = req.query(&[("stale", &self.stale)]);
                let req = req.query(&[("tagdbid", &self.tagdbid)]);
                let req = req.query(&[("tagtype", &self.tagtype)]);
                let req = req.query(&[("tvrf", &self.tvrf)]);
                let req = req.query(&[("utoken", &self.utoken)]);
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
            apptype: Option<String>,
            cat: Option<String>,
            cr: Option<String>,
            device: Option<String>,
            devid: Option<String>,
            devtype: Option<String>,
            exp: Option<String>,
            fs: Option<String>,
            lr: Option<String>,
            make: Option<String>,
            max_results: Option<u32>,
            mc: Option<u32>,
            model: Option<String>,
            mrf: Option<String>,
            page_token: Option<String>,
            product: Option<String>,
            sn: Option<String>,
            stale: Option<i32>,
            tvrf: Option<String>,
            utoken: Option<String>,
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
            pub fn apptype(mut self, value: impl Into<String>) -> Self {
                self.apptype = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn cat(mut self, value: impl Into<String>) -> Self {
                self.cat = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn cr(mut self, value: impl Into<String>) -> Self {
                self.cr = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn device(mut self, value: impl Into<String>) -> Self {
                self.device = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn devid(mut self, value: impl Into<String>) -> Self {
                self.devid = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn devtype(mut self, value: impl Into<String>) -> Self {
                self.devtype = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn exp(mut self, value: impl Into<String>) -> Self {
                self.exp = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn fs(mut self, value: impl Into<String>) -> Self {
                self.fs = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn lr(mut self, value: impl Into<String>) -> Self {
                self.lr = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn make(mut self, value: impl Into<String>) -> Self {
                self.make = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn max_results(mut self, value: u32) -> Self {
                self.max_results = Some(value);
                self
            }
            #[doc = ""]
            pub fn mc(mut self, value: u32) -> Self {
                self.mc = Some(value);
                self
            }
            #[doc = ""]
            pub fn model(mut self, value: impl Into<String>) -> Self {
                self.model = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn mrf(mut self, value: impl Into<String>) -> Self {
                self.mrf = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn page_token(mut self, value: impl Into<String>) -> Self {
                self.page_token = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn product(mut self, value: impl Into<String>) -> Self {
                self.product = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn sn(mut self, value: impl Into<String>) -> Self {
                self.sn = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn stale(mut self, value: i32) -> Self {
                self.stale = Some(value);
                self
            }
            #[doc = ""]
            pub fn tvrf(mut self, value: impl Into<String>) -> Self {
                self.tvrf = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn utoken(mut self, value: impl Into<String>) -> Self {
                self.utoken = Some(value.into());
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
                let mut output = "https://www.googleapis.com/android_video/v1/".to_owned();
                output.push_str("collection/list");
                output
            }
            fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
                let req = self.reqwest.request(::reqwest::Method::GET, path);
                let req = req.query(&[("apptype", &self.apptype)]);
                let req = req.query(&[("cat", &self.cat)]);
                let req = req.query(&[("cr", &self.cr)]);
                let req = req.query(&[("device", &self.device)]);
                let req = req.query(&[("devid", &self.devid)]);
                let req = req.query(&[("devtype", &self.devtype)]);
                let req = req.query(&[("exp", &self.exp)]);
                let req = req.query(&[("fs", &self.fs)]);
                let req = req.query(&[("lr", &self.lr)]);
                let req = req.query(&[("make", &self.make)]);
                let req = req.query(&[("maxResults", &self.max_results)]);
                let req = req.query(&[("mc", &self.mc)]);
                let req = req.query(&[("model", &self.model)]);
                let req = req.query(&[("mrf", &self.mrf)]);
                let req = req.query(&[("pageToken", &self.page_token)]);
                let req = req.query(&[("product", &self.product)]);
                let req = req.query(&[("sn", &self.sn)]);
                let req = req.query(&[("stale", &self.stale)]);
                let req = req.query(&[("tvrf", &self.tvrf)]);
                let req = req.query(&[("utoken", &self.utoken)]);
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
        pub struct PaginatebytokenRequestBuilder<'a, A> {
            pub(crate) reqwest: &'a ::reqwest::Client,
            pub(crate) auth: &'a ::std::sync::Mutex<A>,
            alt: Option<crate::params::Alt>,
            fields: Option<String>,
            key: Option<String>,
            oauth_token: Option<String>,
            pretty_print: Option<bool>,
            quota_user: Option<String>,
            user_ip: Option<String>,
        }
        impl<'a, A: yup_oauth2::GetToken> PaginatebytokenRequestBuilder<'a, A> {
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
                let mut output = "https://www.googleapis.com/android_video/v1/".to_owned();
                output.push_str("collection/paginatebytoken");
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
    pub mod device {
        pub mod params {}
        pub struct DeviceActions<'a, A> {
            pub(crate) reqwest: &'a reqwest::Client,
            pub(crate) auth: &'a std::sync::Mutex<A>,
        }
        impl<'a, A: yup_oauth2::GetToken> DeviceActions<'a, A> {
            #[doc = ""]
            pub fn deauthorize(&self) -> DeauthorizeRequestBuilder<A> {
                DeauthorizeRequestBuilder {
                    reqwest: &self.reqwest,
                    auth: &self.auth,
                    alt: None,
                    fields: None,
                    key: None,
                    oauth_token: None,
                    pretty_print: None,
                    quota_user: None,
                    user_ip: None,
                    deviceid: None,
                    exp: None,
                    make: None,
                    model: None,
                }
            }
            #[doc = ""]
            pub fn gcmregister(
                &self,
                android_id: impl Into<String>,
            ) -> GcmregisterRequestBuilder<A> {
                GcmregisterRequestBuilder {
                    reqwest: &self.reqwest,
                    auth: &self.auth,
                    alt: None,
                    fields: None,
                    key: None,
                    oauth_token: None,
                    pretty_print: None,
                    quota_user: None,
                    user_ip: None,
                    android_id: android_id.into(),
                    exp: None,
                }
            }
            #[doc = ""]
            pub fn gcmunregister(
                &self,
                android_id: impl Into<String>,
            ) -> GcmunregisterRequestBuilder<A> {
                GcmunregisterRequestBuilder {
                    reqwest: &self.reqwest,
                    auth: &self.auth,
                    alt: None,
                    fields: None,
                    key: None,
                    oauth_token: None,
                    pretty_print: None,
                    quota_user: None,
                    user_ip: None,
                    android_id: android_id.into(),
                    exp: None,
                }
            }
            #[doc = ""]
            pub fn getpinstate(&self) -> GetpinstateRequestBuilder<A> {
                GetpinstateRequestBuilder {
                    reqwest: &self.reqwest,
                    auth: &self.auth,
                    alt: None,
                    fields: None,
                    key: None,
                    oauth_token: None,
                    pretty_print: None,
                    quota_user: None,
                    user_ip: None,
                    devid: None,
                    exdevid: None,
                    exp: None,
                    vid: None,
                }
            }
        }
        #[derive(Debug, Clone)]
        pub struct DeauthorizeRequestBuilder<'a, A> {
            pub(crate) reqwest: &'a ::reqwest::Client,
            pub(crate) auth: &'a ::std::sync::Mutex<A>,
            deviceid: Option<String>,
            exp: Option<String>,
            make: Option<String>,
            model: Option<String>,
            alt: Option<crate::params::Alt>,
            fields: Option<String>,
            key: Option<String>,
            oauth_token: Option<String>,
            pretty_print: Option<bool>,
            quota_user: Option<String>,
            user_ip: Option<String>,
        }
        impl<'a, A: yup_oauth2::GetToken> DeauthorizeRequestBuilder<'a, A> {
            #[doc = ""]
            pub fn deviceid(mut self, value: impl Into<String>) -> Self {
                self.deviceid = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn exp(mut self, value: impl Into<String>) -> Self {
                self.exp = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn make(mut self, value: impl Into<String>) -> Self {
                self.make = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn model(mut self, value: impl Into<String>) -> Self {
                self.model = Some(value.into());
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
                let mut output = "https://www.googleapis.com/android_video/v1/".to_owned();
                output.push_str("device/deauthorize");
                output
            }
            fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
                let req = self.reqwest.request(::reqwest::Method::DELETE, path);
                let req = req.query(&[("deviceid", &self.deviceid)]);
                let req = req.query(&[("exp", &self.exp)]);
                let req = req.query(&[("make", &self.make)]);
                let req = req.query(&[("model", &self.model)]);
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
        pub struct GcmregisterRequestBuilder<'a, A> {
            pub(crate) reqwest: &'a ::reqwest::Client,
            pub(crate) auth: &'a ::std::sync::Mutex<A>,
            android_id: String,
            exp: Option<String>,
            alt: Option<crate::params::Alt>,
            fields: Option<String>,
            key: Option<String>,
            oauth_token: Option<String>,
            pretty_print: Option<bool>,
            quota_user: Option<String>,
            user_ip: Option<String>,
        }
        impl<'a, A: yup_oauth2::GetToken> GcmregisterRequestBuilder<'a, A> {
            #[doc = ""]
            pub fn exp(mut self, value: impl Into<String>) -> Self {
                self.exp = Some(value.into());
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
                let mut output = "https://www.googleapis.com/android_video/v1/".to_owned();
                output.push_str("device/");
                {
                    let var_as_str = &self.android_id;
                    output.extend(::percent_encoding::utf8_percent_encode(
                        &var_as_str,
                        crate::SIMPLE,
                    ));
                }
                output.push_str("/gcm");
                output
            }
            fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
                let req = self.reqwest.request(::reqwest::Method::PUT, path);
                let req = req.query(&[("exp", &self.exp)]);
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
        pub struct GcmunregisterRequestBuilder<'a, A> {
            pub(crate) reqwest: &'a ::reqwest::Client,
            pub(crate) auth: &'a ::std::sync::Mutex<A>,
            android_id: String,
            exp: Option<String>,
            alt: Option<crate::params::Alt>,
            fields: Option<String>,
            key: Option<String>,
            oauth_token: Option<String>,
            pretty_print: Option<bool>,
            quota_user: Option<String>,
            user_ip: Option<String>,
        }
        impl<'a, A: yup_oauth2::GetToken> GcmunregisterRequestBuilder<'a, A> {
            #[doc = ""]
            pub fn exp(mut self, value: impl Into<String>) -> Self {
                self.exp = Some(value.into());
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
                let mut output = "https://www.googleapis.com/android_video/v1/".to_owned();
                output.push_str("device/");
                {
                    let var_as_str = &self.android_id;
                    output.extend(::percent_encoding::utf8_percent_encode(
                        &var_as_str,
                        crate::SIMPLE,
                    ));
                }
                output.push_str("/gcm");
                output
            }
            fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
                let req = self.reqwest.request(::reqwest::Method::DELETE, path);
                let req = req.query(&[("exp", &self.exp)]);
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
        pub struct GetpinstateRequestBuilder<'a, A> {
            pub(crate) reqwest: &'a ::reqwest::Client,
            pub(crate) auth: &'a ::std::sync::Mutex<A>,
            devid: Option<String>,
            exdevid: Option<String>,
            exp: Option<String>,
            vid: Option<String>,
            alt: Option<crate::params::Alt>,
            fields: Option<String>,
            key: Option<String>,
            oauth_token: Option<String>,
            pretty_print: Option<bool>,
            quota_user: Option<String>,
            user_ip: Option<String>,
        }
        impl<'a, A: yup_oauth2::GetToken> GetpinstateRequestBuilder<'a, A> {
            #[doc = ""]
            pub fn devid(mut self, value: impl Into<String>) -> Self {
                self.devid = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn exdevid(mut self, value: impl Into<String>) -> Self {
                self.exdevid = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn exp(mut self, value: impl Into<String>) -> Self {
                self.exp = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn vid(mut self, value: impl Into<String>) -> Self {
                self.vid = Some(value.into());
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
                let mut output = "https://www.googleapis.com/android_video/v1/".to_owned();
                output.push_str("device/getpinstate");
                output
            }
            fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
                let req = self.reqwest.request(::reqwest::Method::GET, path);
                let req = req.query(&[("devid", &self.devid)]);
                let req = req.query(&[("exdevid", &self.exdevid)]);
                let req = req.query(&[("exp", &self.exp)]);
                let req = req.query(&[("vid", &self.vid)]);
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
    pub mod gunsproxy {
        pub mod params {}
        pub struct GunsproxyActions<'a, A> {
            pub(crate) reqwest: &'a reqwest::Client,
            pub(crate) auth: &'a std::sync::Mutex<A>,
        }
        impl<'a, A: yup_oauth2::GetToken> GunsproxyActions<'a, A> {
            #[doc = ""]
            pub fn broadcast(&self) -> BroadcastRequestBuilder<A> {
                BroadcastRequestBuilder {
                    reqwest: &self.reqwest,
                    auth: &self.auth,
                    alt: None,
                    fields: None,
                    key: None,
                    oauth_token: None,
                    pretty_print: None,
                    quota_user: None,
                    user_ip: None,
                }
            }
            #[doc = ""]
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
                    appid: None,
                    apptype: None,
                    device: None,
                    devid: None,
                    devtype: None,
                    make: None,
                    model: None,
                    msgid: None,
                    nt: None,
                    product: None,
                    sn: None,
                    view: None,
                }
            }
            #[doc = ""]
            pub fn register(&self) -> RegisterRequestBuilder<A> {
                RegisterRequestBuilder {
                    reqwest: &self.reqwest,
                    auth: &self.auth,
                    alt: None,
                    fields: None,
                    key: None,
                    oauth_token: None,
                    pretty_print: None,
                    quota_user: None,
                    user_ip: None,
                    appid: None,
                    apptype: None,
                    device: None,
                    devid: None,
                    devtype: None,
                    make: None,
                    model: None,
                    product: None,
                    sn: None,
                    view: None,
                }
            }
            #[doc = ""]
            pub fn unregister(&self) -> UnregisterRequestBuilder<A> {
                UnregisterRequestBuilder {
                    reqwest: &self.reqwest,
                    auth: &self.auth,
                    alt: None,
                    fields: None,
                    key: None,
                    oauth_token: None,
                    pretty_print: None,
                    quota_user: None,
                    user_ip: None,
                    appid: None,
                    apptype: None,
                    device: None,
                    devid: None,
                    devtype: None,
                    make: None,
                    model: None,
                    product: None,
                    sn: None,
                    view: None,
                }
            }
        }
        #[derive(Debug, Clone)]
        pub struct BroadcastRequestBuilder<'a, A> {
            pub(crate) reqwest: &'a ::reqwest::Client,
            pub(crate) auth: &'a ::std::sync::Mutex<A>,
            alt: Option<crate::params::Alt>,
            fields: Option<String>,
            key: Option<String>,
            oauth_token: Option<String>,
            pretty_print: Option<bool>,
            quota_user: Option<String>,
            user_ip: Option<String>,
        }
        impl<'a, A: yup_oauth2::GetToken> BroadcastRequestBuilder<'a, A> {
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
                let mut output = "https://www.googleapis.com/android_video/v1/".to_owned();
                output.push_str("gunsproxy/broadcast");
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
            appid: Option<String>,
            apptype: Option<String>,
            device: Option<String>,
            devid: Option<String>,
            devtype: Option<String>,
            make: Option<String>,
            model: Option<String>,
            msgid: Option<String>,
            nt: Option<String>,
            product: Option<String>,
            sn: Option<String>,
            view: Option<String>,
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
            pub fn appid(mut self, value: impl Into<String>) -> Self {
                self.appid = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn apptype(mut self, value: impl Into<String>) -> Self {
                self.apptype = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn device(mut self, value: impl Into<String>) -> Self {
                self.device = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn devid(mut self, value: impl Into<String>) -> Self {
                self.devid = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn devtype(mut self, value: impl Into<String>) -> Self {
                self.devtype = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn make(mut self, value: impl Into<String>) -> Self {
                self.make = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn model(mut self, value: impl Into<String>) -> Self {
                self.model = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn msgid(mut self, value: impl Into<String>) -> Self {
                self.msgid = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn nt(mut self, value: impl Into<String>) -> Self {
                self.nt = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn product(mut self, value: impl Into<String>) -> Self {
                self.product = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn sn(mut self, value: impl Into<String>) -> Self {
                self.sn = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn view(mut self, value: impl Into<String>) -> Self {
                self.view = Some(value.into());
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
                let mut output = "https://www.googleapis.com/android_video/v1/".to_owned();
                output.push_str("gunsproxy/list");
                output
            }
            fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
                let req = self.reqwest.request(::reqwest::Method::GET, path);
                let req = req.query(&[("appid", &self.appid)]);
                let req = req.query(&[("apptype", &self.apptype)]);
                let req = req.query(&[("device", &self.device)]);
                let req = req.query(&[("devid", &self.devid)]);
                let req = req.query(&[("devtype", &self.devtype)]);
                let req = req.query(&[("make", &self.make)]);
                let req = req.query(&[("model", &self.model)]);
                let req = req.query(&[("msgid", &self.msgid)]);
                let req = req.query(&[("nt", &self.nt)]);
                let req = req.query(&[("product", &self.product)]);
                let req = req.query(&[("sn", &self.sn)]);
                let req = req.query(&[("view", &self.view)]);
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
        pub struct RegisterRequestBuilder<'a, A> {
            pub(crate) reqwest: &'a ::reqwest::Client,
            pub(crate) auth: &'a ::std::sync::Mutex<A>,
            appid: Option<String>,
            apptype: Option<String>,
            device: Option<String>,
            devid: Option<String>,
            devtype: Option<String>,
            make: Option<String>,
            model: Option<String>,
            product: Option<String>,
            sn: Option<String>,
            view: Option<String>,
            alt: Option<crate::params::Alt>,
            fields: Option<String>,
            key: Option<String>,
            oauth_token: Option<String>,
            pretty_print: Option<bool>,
            quota_user: Option<String>,
            user_ip: Option<String>,
        }
        impl<'a, A: yup_oauth2::GetToken> RegisterRequestBuilder<'a, A> {
            #[doc = ""]
            pub fn appid(mut self, value: impl Into<String>) -> Self {
                self.appid = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn apptype(mut self, value: impl Into<String>) -> Self {
                self.apptype = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn device(mut self, value: impl Into<String>) -> Self {
                self.device = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn devid(mut self, value: impl Into<String>) -> Self {
                self.devid = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn devtype(mut self, value: impl Into<String>) -> Self {
                self.devtype = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn make(mut self, value: impl Into<String>) -> Self {
                self.make = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn model(mut self, value: impl Into<String>) -> Self {
                self.model = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn product(mut self, value: impl Into<String>) -> Self {
                self.product = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn sn(mut self, value: impl Into<String>) -> Self {
                self.sn = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn view(mut self, value: impl Into<String>) -> Self {
                self.view = Some(value.into());
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
                let mut output = "https://www.googleapis.com/android_video/v1/".to_owned();
                output.push_str("gunsproxy/register");
                output
            }
            fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
                let req = self.reqwest.request(::reqwest::Method::POST, path);
                let req = req.query(&[("appid", &self.appid)]);
                let req = req.query(&[("apptype", &self.apptype)]);
                let req = req.query(&[("device", &self.device)]);
                let req = req.query(&[("devid", &self.devid)]);
                let req = req.query(&[("devtype", &self.devtype)]);
                let req = req.query(&[("make", &self.make)]);
                let req = req.query(&[("model", &self.model)]);
                let req = req.query(&[("product", &self.product)]);
                let req = req.query(&[("sn", &self.sn)]);
                let req = req.query(&[("view", &self.view)]);
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
        pub struct UnregisterRequestBuilder<'a, A> {
            pub(crate) reqwest: &'a ::reqwest::Client,
            pub(crate) auth: &'a ::std::sync::Mutex<A>,
            appid: Option<String>,
            apptype: Option<String>,
            device: Option<String>,
            devid: Option<String>,
            devtype: Option<String>,
            make: Option<String>,
            model: Option<String>,
            product: Option<String>,
            sn: Option<String>,
            view: Option<String>,
            alt: Option<crate::params::Alt>,
            fields: Option<String>,
            key: Option<String>,
            oauth_token: Option<String>,
            pretty_print: Option<bool>,
            quota_user: Option<String>,
            user_ip: Option<String>,
        }
        impl<'a, A: yup_oauth2::GetToken> UnregisterRequestBuilder<'a, A> {
            #[doc = ""]
            pub fn appid(mut self, value: impl Into<String>) -> Self {
                self.appid = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn apptype(mut self, value: impl Into<String>) -> Self {
                self.apptype = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn device(mut self, value: impl Into<String>) -> Self {
                self.device = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn devid(mut self, value: impl Into<String>) -> Self {
                self.devid = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn devtype(mut self, value: impl Into<String>) -> Self {
                self.devtype = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn make(mut self, value: impl Into<String>) -> Self {
                self.make = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn model(mut self, value: impl Into<String>) -> Self {
                self.model = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn product(mut self, value: impl Into<String>) -> Self {
                self.product = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn sn(mut self, value: impl Into<String>) -> Self {
                self.sn = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn view(mut self, value: impl Into<String>) -> Self {
                self.view = Some(value.into());
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
                let mut output = "https://www.googleapis.com/android_video/v1/".to_owned();
                output.push_str("gunsproxy/unregister");
                output
            }
            fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
                let req = self.reqwest.request(::reqwest::Method::POST, path);
                let req = req.query(&[("appid", &self.appid)]);
                let req = req.query(&[("apptype", &self.apptype)]);
                let req = req.query(&[("device", &self.device)]);
                let req = req.query(&[("devid", &self.devid)]);
                let req = req.query(&[("devtype", &self.devtype)]);
                let req = req.query(&[("make", &self.make)]);
                let req = req.query(&[("model", &self.model)]);
                let req = req.query(&[("product", &self.product)]);
                let req = req.query(&[("sn", &self.sn)]);
                let req = req.query(&[("view", &self.view)]);
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
    pub mod mpd {
        pub mod params {}
        pub struct MpdActions<'a, A> {
            pub(crate) reqwest: &'a reqwest::Client,
            pub(crate) auth: &'a std::sync::Mutex<A>,
        }
        impl<'a, A: yup_oauth2::GetToken> MpdActions<'a, A> {
            #[doc = ""]
            pub fn get(&self) -> GetRequestBuilder<A> {
                GetRequestBuilder {
                    reqwest: &self.reqwest,
                    auth: &self.auth,
                    alt: None,
                    fields: None,
                    key: None,
                    oauth_token: None,
                    pretty_print: None,
                    quota_user: None,
                    user_ip: None,
                    ac_3: None,
                    all: None,
                    all_51: None,
                    apptype: None,
                    cl: None,
                    cn: None,
                    ddd: None,
                    device: None,
                    devid: None,
                    devtype: None,
                    exp: None,
                    fc: None,
                    hdcp: None,
                    hdcp_level: None,
                    hdsb: None,
                    hls: None,
                    hvc: None,
                    id: None,
                    lr: None,
                    ma: None,
                    make: None,
                    model: None,
                    msu: None,
                    narra: None,
                    nd: None,
                    opus: None,
                    product: None,
                    pues: None,
                    secure: None,
                    sn: None,
                    ssrc: None,
                    url: None,
                    vor: None,
                    webm: None,
                }
            }
        }
        #[derive(Debug, Clone)]
        pub struct GetRequestBuilder<'a, A> {
            pub(crate) reqwest: &'a ::reqwest::Client,
            pub(crate) auth: &'a ::std::sync::Mutex<A>,
            ac_3: Option<bool>,
            all: Option<bool>,
            all_51: Option<bool>,
            apptype: Option<String>,
            cl: Option<String>,
            cn: Option<String>,
            ddd: Option<bool>,
            device: Option<String>,
            devid: Option<String>,
            devtype: Option<String>,
            exp: Option<String>,
            fc: Option<bool>,
            hdcp: Option<bool>,
            hdcp_level: Option<String>,
            hdsb: Option<bool>,
            hls: Option<bool>,
            hvc: Option<bool>,
            id: Option<String>,
            lr: Option<String>,
            ma: Option<bool>,
            make: Option<String>,
            model: Option<String>,
            msu: Option<bool>,
            narra: Option<bool>,
            nd: Option<bool>,
            opus: Option<String>,
            product: Option<String>,
            pues: Option<u64>,
            secure: Option<bool>,
            sn: Option<String>,
            ssrc: Option<String>,
            url: Option<String>,
            vor: Option<String>,
            webm: Option<bool>,
            alt: Option<crate::params::Alt>,
            fields: Option<String>,
            key: Option<String>,
            oauth_token: Option<String>,
            pretty_print: Option<bool>,
            quota_user: Option<String>,
            user_ip: Option<String>,
        }
        impl<'a, A: yup_oauth2::GetToken> GetRequestBuilder<'a, A> {
            #[doc = ""]
            pub fn ac_3(mut self, value: bool) -> Self {
                self.ac_3 = Some(value);
                self
            }
            #[doc = ""]
            pub fn all(mut self, value: bool) -> Self {
                self.all = Some(value);
                self
            }
            #[doc = ""]
            pub fn all_51(mut self, value: bool) -> Self {
                self.all_51 = Some(value);
                self
            }
            #[doc = ""]
            pub fn apptype(mut self, value: impl Into<String>) -> Self {
                self.apptype = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn cl(mut self, value: impl Into<String>) -> Self {
                self.cl = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn cn(mut self, value: impl Into<String>) -> Self {
                self.cn = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn ddd(mut self, value: bool) -> Self {
                self.ddd = Some(value);
                self
            }
            #[doc = ""]
            pub fn device(mut self, value: impl Into<String>) -> Self {
                self.device = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn devid(mut self, value: impl Into<String>) -> Self {
                self.devid = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn devtype(mut self, value: impl Into<String>) -> Self {
                self.devtype = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn exp(mut self, value: impl Into<String>) -> Self {
                self.exp = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn fc(mut self, value: bool) -> Self {
                self.fc = Some(value);
                self
            }
            #[doc = ""]
            pub fn hdcp(mut self, value: bool) -> Self {
                self.hdcp = Some(value);
                self
            }
            #[doc = ""]
            pub fn hdcp_level(mut self, value: impl Into<String>) -> Self {
                self.hdcp_level = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn hdsb(mut self, value: bool) -> Self {
                self.hdsb = Some(value);
                self
            }
            #[doc = ""]
            pub fn hls(mut self, value: bool) -> Self {
                self.hls = Some(value);
                self
            }
            #[doc = ""]
            pub fn hvc(mut self, value: bool) -> Self {
                self.hvc = Some(value);
                self
            }
            #[doc = ""]
            pub fn id(mut self, value: impl Into<String>) -> Self {
                self.id = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn lr(mut self, value: impl Into<String>) -> Self {
                self.lr = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn ma(mut self, value: bool) -> Self {
                self.ma = Some(value);
                self
            }
            #[doc = ""]
            pub fn make(mut self, value: impl Into<String>) -> Self {
                self.make = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn model(mut self, value: impl Into<String>) -> Self {
                self.model = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn msu(mut self, value: bool) -> Self {
                self.msu = Some(value);
                self
            }
            #[doc = ""]
            pub fn narra(mut self, value: bool) -> Self {
                self.narra = Some(value);
                self
            }
            #[doc = ""]
            pub fn nd(mut self, value: bool) -> Self {
                self.nd = Some(value);
                self
            }
            #[doc = ""]
            pub fn opus(mut self, value: impl Into<String>) -> Self {
                self.opus = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn product(mut self, value: impl Into<String>) -> Self {
                self.product = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn pues(mut self, value: u64) -> Self {
                self.pues = Some(value);
                self
            }
            #[doc = ""]
            pub fn secure(mut self, value: bool) -> Self {
                self.secure = Some(value);
                self
            }
            #[doc = ""]
            pub fn sn(mut self, value: impl Into<String>) -> Self {
                self.sn = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn ssrc(mut self, value: impl Into<String>) -> Self {
                self.ssrc = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn url(mut self, value: impl Into<String>) -> Self {
                self.url = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn vor(mut self, value: impl Into<String>) -> Self {
                self.vor = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn webm(mut self, value: bool) -> Self {
                self.webm = Some(value);
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
            ) -> Result<crate::schemas::MpdGetResponse, Box<dyn ::std::error::Error>> {
                self.execute_fields::<_, &str>(None)
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub fn execute_debug(
                self,
            ) -> Result<crate::schemas::MpdGetResponse, Box<dyn ::std::error::Error>> {
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
                let mut output = "https://www.googleapis.com/android_video/v1/".to_owned();
                output.push_str("mpd");
                output
            }
            fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
                let req = self.reqwest.request(::reqwest::Method::GET, path);
                let req = req.query(&[("ac3", &self.ac_3)]);
                let req = req.query(&[("all", &self.all)]);
                let req = req.query(&[("all51", &self.all_51)]);
                let req = req.query(&[("apptype", &self.apptype)]);
                let req = req.query(&[("cl", &self.cl)]);
                let req = req.query(&[("cn", &self.cn)]);
                let req = req.query(&[("ddd", &self.ddd)]);
                let req = req.query(&[("device", &self.device)]);
                let req = req.query(&[("devid", &self.devid)]);
                let req = req.query(&[("devtype", &self.devtype)]);
                let req = req.query(&[("exp", &self.exp)]);
                let req = req.query(&[("fc", &self.fc)]);
                let req = req.query(&[("hdcp", &self.hdcp)]);
                let req = req.query(&[("hdcp_level", &self.hdcp_level)]);
                let req = req.query(&[("hdsb", &self.hdsb)]);
                let req = req.query(&[("hls", &self.hls)]);
                let req = req.query(&[("hvc", &self.hvc)]);
                let req = req.query(&[("id", &self.id)]);
                let req = req.query(&[("lr", &self.lr)]);
                let req = req.query(&[("ma", &self.ma)]);
                let req = req.query(&[("make", &self.make)]);
                let req = req.query(&[("model", &self.model)]);
                let req = req.query(&[("msu", &self.msu)]);
                let req = req.query(&[("narra", &self.narra)]);
                let req = req.query(&[("nd", &self.nd)]);
                let req = req.query(&[("opus", &self.opus)]);
                let req = req.query(&[("product", &self.product)]);
                let req = req.query(&[("pues", &self.pues)]);
                let req = req.query(&[("secure", &self.secure)]);
                let req = req.query(&[("sn", &self.sn)]);
                let req = req.query(&[("ssrc", &self.ssrc)]);
                let req = req.query(&[("url", &self.url)]);
                let req = req.query(&[("vor", &self.vor)]);
                let req = req.query(&[("webm", &self.webm)]);
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
    pub mod mpdurl {
        pub mod params {}
        pub struct MpdurlActions<'a, A> {
            pub(crate) reqwest: &'a reqwest::Client,
            pub(crate) auth: &'a std::sync::Mutex<A>,
        }
        impl<'a, A: yup_oauth2::GetToken> MpdurlActions<'a, A> {
            #[doc = ""]
            pub fn get(&self, video_id: impl Into<String>) -> GetRequestBuilder<A> {
                GetRequestBuilder {
                    reqwest: &self.reqwest,
                    auth: &self.auth,
                    alt: None,
                    fields: None,
                    key: None,
                    oauth_token: None,
                    pretty_print: None,
                    quota_user: None,
                    user_ip: None,
                    video_id: video_id.into(),
                    apptype: None,
                    device: None,
                    devid: None,
                    devtype: None,
                    exp: None,
                    lr: None,
                    ma: None,
                    make: None,
                    model: None,
                    opus: None,
                    product: None,
                    sn: None,
                    vor: None,
                    webm: None,
                    youtube_com: None,
                }
            }
        }
        #[derive(Debug, Clone)]
        pub struct GetRequestBuilder<'a, A> {
            pub(crate) reqwest: &'a ::reqwest::Client,
            pub(crate) auth: &'a ::std::sync::Mutex<A>,
            video_id: String,
            apptype: Option<String>,
            device: Option<String>,
            devid: Option<String>,
            devtype: Option<String>,
            exp: Option<String>,
            lr: Option<String>,
            ma: Option<bool>,
            make: Option<String>,
            model: Option<String>,
            opus: Option<String>,
            product: Option<String>,
            sn: Option<String>,
            vor: Option<String>,
            webm: Option<bool>,
            youtube_com: Option<bool>,
            alt: Option<crate::params::Alt>,
            fields: Option<String>,
            key: Option<String>,
            oauth_token: Option<String>,
            pretty_print: Option<bool>,
            quota_user: Option<String>,
            user_ip: Option<String>,
        }
        impl<'a, A: yup_oauth2::GetToken> GetRequestBuilder<'a, A> {
            #[doc = ""]
            pub fn apptype(mut self, value: impl Into<String>) -> Self {
                self.apptype = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn device(mut self, value: impl Into<String>) -> Self {
                self.device = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn devid(mut self, value: impl Into<String>) -> Self {
                self.devid = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn devtype(mut self, value: impl Into<String>) -> Self {
                self.devtype = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn exp(mut self, value: impl Into<String>) -> Self {
                self.exp = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn lr(mut self, value: impl Into<String>) -> Self {
                self.lr = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn ma(mut self, value: bool) -> Self {
                self.ma = Some(value);
                self
            }
            #[doc = ""]
            pub fn make(mut self, value: impl Into<String>) -> Self {
                self.make = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn model(mut self, value: impl Into<String>) -> Self {
                self.model = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn opus(mut self, value: impl Into<String>) -> Self {
                self.opus = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn product(mut self, value: impl Into<String>) -> Self {
                self.product = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn sn(mut self, value: impl Into<String>) -> Self {
                self.sn = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn vor(mut self, value: impl Into<String>) -> Self {
                self.vor = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn webm(mut self, value: bool) -> Self {
                self.webm = Some(value);
                self
            }
            #[doc = ""]
            pub fn youtube_com(mut self, value: bool) -> Self {
                self.youtube_com = Some(value);
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
            ) -> Result<crate::schemas::MpdUrlGetResponse, Box<dyn ::std::error::Error>>
            {
                self.execute_fields::<_, &str>(None)
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub fn execute_debug(
                self,
            ) -> Result<crate::schemas::MpdUrlGetResponse, Box<dyn ::std::error::Error>>
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
                let mut output = "https://www.googleapis.com/android_video/v1/".to_owned();
                output.push_str("mpdurl/");
                {
                    let var_as_str = &self.video_id;
                    output.extend(::percent_encoding::utf8_percent_encode(
                        &var_as_str,
                        crate::SIMPLE,
                    ));
                }
                output
            }
            fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
                let req = self.reqwest.request(::reqwest::Method::GET, path);
                let req = req.query(&[("apptype", &self.apptype)]);
                let req = req.query(&[("device", &self.device)]);
                let req = req.query(&[("devid", &self.devid)]);
                let req = req.query(&[("devtype", &self.devtype)]);
                let req = req.query(&[("exp", &self.exp)]);
                let req = req.query(&[("lr", &self.lr)]);
                let req = req.query(&[("ma", &self.ma)]);
                let req = req.query(&[("make", &self.make)]);
                let req = req.query(&[("model", &self.model)]);
                let req = req.query(&[("opus", &self.opus)]);
                let req = req.query(&[("product", &self.product)]);
                let req = req.query(&[("sn", &self.sn)]);
                let req = req.query(&[("vor", &self.vor)]);
                let req = req.query(&[("webm", &self.webm)]);
                let req = req.query(&[("youtube_com", &self.youtube_com)]);
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
    pub mod promotion {
        pub mod params {}
        pub struct PromotionActions<'a, A> {
            pub(crate) reqwest: &'a reqwest::Client,
            pub(crate) auth: &'a std::sync::Mutex<A>,
        }
        impl<'a, A: yup_oauth2::GetToken> PromotionActions<'a, A> {
            #[doc = ""]
            pub fn campaigndetails(&self) -> CampaigndetailsRequestBuilder<A> {
                CampaigndetailsRequestBuilder {
                    reqwest: &self.reqwest,
                    auth: &self.auth,
                    alt: None,
                    fields: None,
                    key: None,
                    oauth_token: None,
                    pretty_print: None,
                    quota_user: None,
                    user_ip: None,
                    apptype: None,
                    campaign: None,
                    cr: None,
                    device: None,
                    devid: None,
                    devtype: None,
                    exp: None,
                    lr: None,
                    make: None,
                    model: None,
                    product: None,
                    sn: None,
                    ttype: None,
                }
            }
            #[doc = ""]
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
                    apptype: None,
                    cr: None,
                    device: None,
                    devid: None,
                    devtype: None,
                    exp: None,
                    lr: None,
                    make: None,
                    model: None,
                    mrf: None,
                    product: None,
                    sn: None,
                    ttype: None,
                    tvrf: None,
                }
            }
            #[doc = ""]
            pub fn redeem(&self) -> RedeemRequestBuilder<A> {
                RedeemRequestBuilder {
                    reqwest: &self.reqwest,
                    auth: &self.auth,
                    alt: None,
                    fields: None,
                    key: None,
                    oauth_token: None,
                    pretty_print: None,
                    quota_user: None,
                    user_ip: None,
                    aid: None,
                    apptype: None,
                    cr: None,
                    ct: None,
                    device: None,
                    devid: None,
                    devtype: None,
                    exp: None,
                    make: None,
                    model: None,
                    product: None,
                    promo: None,
                    sn: None,
                    ta: None,
                    ttype: None,
                }
            }
        }
        #[derive(Debug, Clone)]
        pub struct CampaigndetailsRequestBuilder<'a, A> {
            pub(crate) reqwest: &'a ::reqwest::Client,
            pub(crate) auth: &'a ::std::sync::Mutex<A>,
            apptype: Option<String>,
            campaign: Option<String>,
            cr: Option<String>,
            device: Option<String>,
            devid: Option<String>,
            devtype: Option<String>,
            exp: Option<String>,
            lr: Option<String>,
            make: Option<String>,
            model: Option<String>,
            product: Option<String>,
            sn: Option<String>,
            ttype: Option<String>,
            alt: Option<crate::params::Alt>,
            fields: Option<String>,
            key: Option<String>,
            oauth_token: Option<String>,
            pretty_print: Option<bool>,
            quota_user: Option<String>,
            user_ip: Option<String>,
        }
        impl<'a, A: yup_oauth2::GetToken> CampaigndetailsRequestBuilder<'a, A> {
            #[doc = ""]
            pub fn apptype(mut self, value: impl Into<String>) -> Self {
                self.apptype = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn campaign(mut self, value: impl Into<String>) -> Self {
                self.campaign = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn cr(mut self, value: impl Into<String>) -> Self {
                self.cr = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn device(mut self, value: impl Into<String>) -> Self {
                self.device = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn devid(mut self, value: impl Into<String>) -> Self {
                self.devid = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn devtype(mut self, value: impl Into<String>) -> Self {
                self.devtype = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn exp(mut self, value: impl Into<String>) -> Self {
                self.exp = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn lr(mut self, value: impl Into<String>) -> Self {
                self.lr = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn make(mut self, value: impl Into<String>) -> Self {
                self.make = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn model(mut self, value: impl Into<String>) -> Self {
                self.model = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn product(mut self, value: impl Into<String>) -> Self {
                self.product = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn sn(mut self, value: impl Into<String>) -> Self {
                self.sn = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn ttype(mut self, value: impl Into<String>) -> Self {
                self.ttype = Some(value.into());
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
                let mut output = "https://www.googleapis.com/android_video/v1/".to_owned();
                output.push_str("promotion/campaigndetails");
                output
            }
            fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
                let req = self.reqwest.request(::reqwest::Method::POST, path);
                let req = req.query(&[("apptype", &self.apptype)]);
                let req = req.query(&[("campaign", &self.campaign)]);
                let req = req.query(&[("cr", &self.cr)]);
                let req = req.query(&[("device", &self.device)]);
                let req = req.query(&[("devid", &self.devid)]);
                let req = req.query(&[("devtype", &self.devtype)]);
                let req = req.query(&[("exp", &self.exp)]);
                let req = req.query(&[("lr", &self.lr)]);
                let req = req.query(&[("make", &self.make)]);
                let req = req.query(&[("model", &self.model)]);
                let req = req.query(&[("product", &self.product)]);
                let req = req.query(&[("sn", &self.sn)]);
                let req = req.query(&[("ttype", &self.ttype)]);
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
            apptype: Option<String>,
            cr: Option<String>,
            device: Option<String>,
            devid: Option<String>,
            devtype: Option<String>,
            exp: Option<String>,
            lr: Option<String>,
            make: Option<String>,
            model: Option<String>,
            mrf: Option<String>,
            product: Option<String>,
            sn: Option<String>,
            ttype: Option<String>,
            tvrf: Option<String>,
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
            pub fn apptype(mut self, value: impl Into<String>) -> Self {
                self.apptype = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn cr(mut self, value: impl Into<String>) -> Self {
                self.cr = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn device(mut self, value: impl Into<String>) -> Self {
                self.device = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn devid(mut self, value: impl Into<String>) -> Self {
                self.devid = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn devtype(mut self, value: impl Into<String>) -> Self {
                self.devtype = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn exp(mut self, value: impl Into<String>) -> Self {
                self.exp = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn lr(mut self, value: impl Into<String>) -> Self {
                self.lr = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn make(mut self, value: impl Into<String>) -> Self {
                self.make = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn model(mut self, value: impl Into<String>) -> Self {
                self.model = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn mrf(mut self, value: impl Into<String>) -> Self {
                self.mrf = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn product(mut self, value: impl Into<String>) -> Self {
                self.product = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn sn(mut self, value: impl Into<String>) -> Self {
                self.sn = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn ttype(mut self, value: impl Into<String>) -> Self {
                self.ttype = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn tvrf(mut self, value: impl Into<String>) -> Self {
                self.tvrf = Some(value.into());
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
                let mut output = "https://www.googleapis.com/android_video/v1/".to_owned();
                output.push_str("promotion/list");
                output
            }
            fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
                let req = self.reqwest.request(::reqwest::Method::GET, path);
                let req = req.query(&[("apptype", &self.apptype)]);
                let req = req.query(&[("cr", &self.cr)]);
                let req = req.query(&[("device", &self.device)]);
                let req = req.query(&[("devid", &self.devid)]);
                let req = req.query(&[("devtype", &self.devtype)]);
                let req = req.query(&[("exp", &self.exp)]);
                let req = req.query(&[("lr", &self.lr)]);
                let req = req.query(&[("make", &self.make)]);
                let req = req.query(&[("model", &self.model)]);
                let req = req.query(&[("mrf", &self.mrf)]);
                let req = req.query(&[("product", &self.product)]);
                let req = req.query(&[("sn", &self.sn)]);
                let req = req.query(&[("ttype", &self.ttype)]);
                let req = req.query(&[("tvrf", &self.tvrf)]);
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
        pub struct RedeemRequestBuilder<'a, A> {
            pub(crate) reqwest: &'a ::reqwest::Client,
            pub(crate) auth: &'a ::std::sync::Mutex<A>,
            aid: Option<String>,
            apptype: Option<String>,
            cr: Option<String>,
            ct: Option<String>,
            device: Option<String>,
            devid: Option<String>,
            devtype: Option<String>,
            exp: Option<String>,
            make: Option<String>,
            model: Option<String>,
            product: Option<String>,
            promo: Option<String>,
            sn: Option<String>,
            ta: Option<bool>,
            ttype: Option<String>,
            alt: Option<crate::params::Alt>,
            fields: Option<String>,
            key: Option<String>,
            oauth_token: Option<String>,
            pretty_print: Option<bool>,
            quota_user: Option<String>,
            user_ip: Option<String>,
        }
        impl<'a, A: yup_oauth2::GetToken> RedeemRequestBuilder<'a, A> {
            #[doc = ""]
            pub fn aid(mut self, value: impl Into<String>) -> Self {
                self.aid = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn apptype(mut self, value: impl Into<String>) -> Self {
                self.apptype = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn cr(mut self, value: impl Into<String>) -> Self {
                self.cr = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn ct(mut self, value: impl Into<String>) -> Self {
                self.ct = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn device(mut self, value: impl Into<String>) -> Self {
                self.device = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn devid(mut self, value: impl Into<String>) -> Self {
                self.devid = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn devtype(mut self, value: impl Into<String>) -> Self {
                self.devtype = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn exp(mut self, value: impl Into<String>) -> Self {
                self.exp = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn make(mut self, value: impl Into<String>) -> Self {
                self.make = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn model(mut self, value: impl Into<String>) -> Self {
                self.model = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn product(mut self, value: impl Into<String>) -> Self {
                self.product = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn promo(mut self, value: impl Into<String>) -> Self {
                self.promo = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn sn(mut self, value: impl Into<String>) -> Self {
                self.sn = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn ta(mut self, value: bool) -> Self {
                self.ta = Some(value);
                self
            }
            #[doc = ""]
            pub fn ttype(mut self, value: impl Into<String>) -> Self {
                self.ttype = Some(value.into());
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
                let mut output = "https://www.googleapis.com/android_video/v1/".to_owned();
                output.push_str("promotion/redeem");
                output
            }
            fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
                let req = self.reqwest.request(::reqwest::Method::POST, path);
                let req = req.query(&[("aid", &self.aid)]);
                let req = req.query(&[("apptype", &self.apptype)]);
                let req = req.query(&[("cr", &self.cr)]);
                let req = req.query(&[("ct", &self.ct)]);
                let req = req.query(&[("device", &self.device)]);
                let req = req.query(&[("devid", &self.devid)]);
                let req = req.query(&[("devtype", &self.devtype)]);
                let req = req.query(&[("exp", &self.exp)]);
                let req = req.query(&[("make", &self.make)]);
                let req = req.query(&[("model", &self.model)]);
                let req = req.query(&[("product", &self.product)]);
                let req = req.query(&[("promo", &self.promo)]);
                let req = req.query(&[("sn", &self.sn)]);
                let req = req.query(&[("ta", &self.ta)]);
                let req = req.query(&[("ttype", &self.ttype)]);
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
    pub mod purchase {
        pub mod params {}
        pub struct PurchaseActions<'a, A> {
            pub(crate) reqwest: &'a reqwest::Client,
            pub(crate) auth: &'a std::sync::Mutex<A>,
        }
        impl<'a, A: yup_oauth2::GetToken> PurchaseActions<'a, A> {
            #[doc = ""]
            pub fn cancel(&self) -> CancelRequestBuilder<A> {
                CancelRequestBuilder {
                    reqwest: &self.reqwest,
                    auth: &self.auth,
                    alt: None,
                    fields: None,
                    key: None,
                    oauth_token: None,
                    pretty_print: None,
                    quota_user: None,
                    user_ip: None,
                    aid: None,
                    apptype: None,
                    device: None,
                    devid: None,
                    devtype: None,
                    exp: None,
                    ft: None,
                    make: None,
                    model: None,
                    ot: None,
                    product: None,
                    sn: None,
                }
            }
            #[doc = ""]
            pub fn complete(&self) -> CompleteRequestBuilder<A> {
                CompleteRequestBuilder {
                    reqwest: &self.reqwest,
                    auth: &self.auth,
                    alt: None,
                    fields: None,
                    key: None,
                    oauth_token: None,
                    pretty_print: None,
                    quota_user: None,
                    user_ip: None,
                    aid: None,
                    apptype: None,
                    ati: None,
                    cart: None,
                    cldu: None,
                    cr: None,
                    device: None,
                    devid: None,
                    devtype: None,
                    exp: None,
                    ft: None,
                    lr: None,
                    make: None,
                    model: None,
                    ofid: None,
                    ot: None,
                    product: None,
                    seii: None,
                    sn: None,
                    vhid: None,
                }
            }
            #[doc = ""]
            pub fn findvouchers(&self) -> FindvouchersRequestBuilder<A> {
                FindvouchersRequestBuilder {
                    reqwest: &self.reqwest,
                    auth: &self.auth,
                    alt: None,
                    fields: None,
                    key: None,
                    oauth_token: None,
                    pretty_print: None,
                    quota_user: None,
                    user_ip: None,
                    apptype: None,
                    device: None,
                    devid: None,
                    devtype: None,
                    exp: None,
                    ft: None,
                    id: None,
                    make: None,
                    model: None,
                    ot: None,
                    product: None,
                    sn: None,
                    vhid: None,
                }
            }
            #[doc = ""]
            pub fn grant(&self) -> GrantRequestBuilder<A> {
                GrantRequestBuilder {
                    reqwest: &self.reqwest,
                    auth: &self.auth,
                    alt: None,
                    fields: None,
                    key: None,
                    oauth_token: None,
                    pretty_print: None,
                    quota_user: None,
                    user_ip: None,
                    aid: None,
                    apptype: None,
                    cr: None,
                    device: None,
                    devid: None,
                    devtype: None,
                    exp: None,
                    ft: None,
                    lr: None,
                    make: None,
                    model: None,
                    ofid: None,
                    ot: None,
                    product: None,
                    sn: None,
                }
            }
            #[doc = ""]
            pub fn prepare(&self) -> PrepareRequestBuilder<A> {
                PrepareRequestBuilder {
                    reqwest: &self.reqwest,
                    auth: &self.auth,
                    alt: None,
                    fields: None,
                    key: None,
                    oauth_token: None,
                    pretty_print: None,
                    quota_user: None,
                    user_ip: None,
                    aid: None,
                    apptype: None,
                    auvh: None,
                    cr: None,
                    device: None,
                    devid: None,
                    devtype: None,
                    exp: None,
                    ft: None,
                    lr: None,
                    make: None,
                    model: None,
                    ofid: None,
                    ot: None,
                    product: None,
                    sn: None,
                    vhid: None,
                }
            }
        }
        #[derive(Debug, Clone)]
        pub struct CancelRequestBuilder<'a, A> {
            pub(crate) reqwest: &'a ::reqwest::Client,
            pub(crate) auth: &'a ::std::sync::Mutex<A>,
            aid: Option<String>,
            apptype: Option<String>,
            device: Option<String>,
            devid: Option<String>,
            devtype: Option<String>,
            exp: Option<String>,
            ft: Option<String>,
            make: Option<String>,
            model: Option<String>,
            ot: Option<String>,
            product: Option<String>,
            sn: Option<String>,
            alt: Option<crate::params::Alt>,
            fields: Option<String>,
            key: Option<String>,
            oauth_token: Option<String>,
            pretty_print: Option<bool>,
            quota_user: Option<String>,
            user_ip: Option<String>,
        }
        impl<'a, A: yup_oauth2::GetToken> CancelRequestBuilder<'a, A> {
            #[doc = ""]
            pub fn aid(mut self, value: impl Into<String>) -> Self {
                self.aid = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn apptype(mut self, value: impl Into<String>) -> Self {
                self.apptype = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn device(mut self, value: impl Into<String>) -> Self {
                self.device = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn devid(mut self, value: impl Into<String>) -> Self {
                self.devid = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn devtype(mut self, value: impl Into<String>) -> Self {
                self.devtype = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn exp(mut self, value: impl Into<String>) -> Self {
                self.exp = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn ft(mut self, value: impl Into<String>) -> Self {
                self.ft = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn make(mut self, value: impl Into<String>) -> Self {
                self.make = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn model(mut self, value: impl Into<String>) -> Self {
                self.model = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn ot(mut self, value: impl Into<String>) -> Self {
                self.ot = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn product(mut self, value: impl Into<String>) -> Self {
                self.product = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn sn(mut self, value: impl Into<String>) -> Self {
                self.sn = Some(value.into());
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
                let mut output = "https://www.googleapis.com/android_video/v1/".to_owned();
                output.push_str("purchase/cancel");
                output
            }
            fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
                let req = self.reqwest.request(::reqwest::Method::POST, path);
                let req = req.query(&[("aid", &self.aid)]);
                let req = req.query(&[("apptype", &self.apptype)]);
                let req = req.query(&[("device", &self.device)]);
                let req = req.query(&[("devid", &self.devid)]);
                let req = req.query(&[("devtype", &self.devtype)]);
                let req = req.query(&[("exp", &self.exp)]);
                let req = req.query(&[("ft", &self.ft)]);
                let req = req.query(&[("make", &self.make)]);
                let req = req.query(&[("model", &self.model)]);
                let req = req.query(&[("ot", &self.ot)]);
                let req = req.query(&[("product", &self.product)]);
                let req = req.query(&[("sn", &self.sn)]);
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
        pub struct CompleteRequestBuilder<'a, A> {
            pub(crate) reqwest: &'a ::reqwest::Client,
            pub(crate) auth: &'a ::std::sync::Mutex<A>,
            aid: Option<String>,
            apptype: Option<String>,
            ati: Option<String>,
            cart: Option<String>,
            cldu: Option<Vec<String>>,
            cr: Option<String>,
            device: Option<String>,
            devid: Option<String>,
            devtype: Option<String>,
            exp: Option<String>,
            ft: Option<String>,
            lr: Option<String>,
            make: Option<String>,
            model: Option<String>,
            ofid: Option<String>,
            ot: Option<String>,
            product: Option<String>,
            seii: Option<String>,
            sn: Option<String>,
            vhid: Option<String>,
            alt: Option<crate::params::Alt>,
            fields: Option<String>,
            key: Option<String>,
            oauth_token: Option<String>,
            pretty_print: Option<bool>,
            quota_user: Option<String>,
            user_ip: Option<String>,
        }
        impl<'a, A: yup_oauth2::GetToken> CompleteRequestBuilder<'a, A> {
            #[doc = ""]
            pub fn aid(mut self, value: impl Into<String>) -> Self {
                self.aid = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn apptype(mut self, value: impl Into<String>) -> Self {
                self.apptype = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn ati(mut self, value: impl Into<String>) -> Self {
                self.ati = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn cart(mut self, value: impl Into<String>) -> Self {
                self.cart = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn cldu(mut self, value: impl Into<Vec<String>>) -> Self {
                self.cldu = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn cr(mut self, value: impl Into<String>) -> Self {
                self.cr = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn device(mut self, value: impl Into<String>) -> Self {
                self.device = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn devid(mut self, value: impl Into<String>) -> Self {
                self.devid = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn devtype(mut self, value: impl Into<String>) -> Self {
                self.devtype = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn exp(mut self, value: impl Into<String>) -> Self {
                self.exp = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn ft(mut self, value: impl Into<String>) -> Self {
                self.ft = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn lr(mut self, value: impl Into<String>) -> Self {
                self.lr = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn make(mut self, value: impl Into<String>) -> Self {
                self.make = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn model(mut self, value: impl Into<String>) -> Self {
                self.model = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn ofid(mut self, value: impl Into<String>) -> Self {
                self.ofid = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn ot(mut self, value: impl Into<String>) -> Self {
                self.ot = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn product(mut self, value: impl Into<String>) -> Self {
                self.product = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn seii(mut self, value: impl Into<String>) -> Self {
                self.seii = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn sn(mut self, value: impl Into<String>) -> Self {
                self.sn = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn vhid(mut self, value: impl Into<String>) -> Self {
                self.vhid = Some(value.into());
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
                let mut output = "https://www.googleapis.com/android_video/v1/".to_owned();
                output.push_str("purchase/complete");
                output
            }
            fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
                let req = self.reqwest.request(::reqwest::Method::POST, path);
                let req = req.query(&[("aid", &self.aid)]);
                let req = req.query(&[("apptype", &self.apptype)]);
                let req = req.query(&[("ati", &self.ati)]);
                let req = req.query(&[("cart", &self.cart)]);
                let req = req.query(&[("cldu", &self.cldu)]);
                let req = req.query(&[("cr", &self.cr)]);
                let req = req.query(&[("device", &self.device)]);
                let req = req.query(&[("devid", &self.devid)]);
                let req = req.query(&[("devtype", &self.devtype)]);
                let req = req.query(&[("exp", &self.exp)]);
                let req = req.query(&[("ft", &self.ft)]);
                let req = req.query(&[("lr", &self.lr)]);
                let req = req.query(&[("make", &self.make)]);
                let req = req.query(&[("model", &self.model)]);
                let req = req.query(&[("ofid", &self.ofid)]);
                let req = req.query(&[("ot", &self.ot)]);
                let req = req.query(&[("product", &self.product)]);
                let req = req.query(&[("seii", &self.seii)]);
                let req = req.query(&[("sn", &self.sn)]);
                let req = req.query(&[("vhid", &self.vhid)]);
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
        pub struct FindvouchersRequestBuilder<'a, A> {
            pub(crate) reqwest: &'a ::reqwest::Client,
            pub(crate) auth: &'a ::std::sync::Mutex<A>,
            apptype: Option<String>,
            device: Option<String>,
            devid: Option<String>,
            devtype: Option<String>,
            exp: Option<String>,
            ft: Option<Vec<String>>,
            id: Option<String>,
            make: Option<String>,
            model: Option<String>,
            ot: Option<Vec<String>>,
            product: Option<String>,
            sn: Option<String>,
            vhid: Option<Vec<String>>,
            alt: Option<crate::params::Alt>,
            fields: Option<String>,
            key: Option<String>,
            oauth_token: Option<String>,
            pretty_print: Option<bool>,
            quota_user: Option<String>,
            user_ip: Option<String>,
        }
        impl<'a, A: yup_oauth2::GetToken> FindvouchersRequestBuilder<'a, A> {
            #[doc = ""]
            pub fn apptype(mut self, value: impl Into<String>) -> Self {
                self.apptype = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn device(mut self, value: impl Into<String>) -> Self {
                self.device = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn devid(mut self, value: impl Into<String>) -> Self {
                self.devid = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn devtype(mut self, value: impl Into<String>) -> Self {
                self.devtype = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn exp(mut self, value: impl Into<String>) -> Self {
                self.exp = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn ft(mut self, value: impl Into<Vec<String>>) -> Self {
                self.ft = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn id(mut self, value: impl Into<String>) -> Self {
                self.id = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn make(mut self, value: impl Into<String>) -> Self {
                self.make = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn model(mut self, value: impl Into<String>) -> Self {
                self.model = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn ot(mut self, value: impl Into<Vec<String>>) -> Self {
                self.ot = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn product(mut self, value: impl Into<String>) -> Self {
                self.product = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn sn(mut self, value: impl Into<String>) -> Self {
                self.sn = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn vhid(mut self, value: impl Into<Vec<String>>) -> Self {
                self.vhid = Some(value.into());
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
                let mut output = "https://www.googleapis.com/android_video/v1/".to_owned();
                output.push_str("purchase/findvouchers");
                output
            }
            fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
                let req = self.reqwest.request(::reqwest::Method::GET, path);
                let req = req.query(&[("apptype", &self.apptype)]);
                let req = req.query(&[("device", &self.device)]);
                let req = req.query(&[("devid", &self.devid)]);
                let req = req.query(&[("devtype", &self.devtype)]);
                let req = req.query(&[("exp", &self.exp)]);
                let req = req.query(&[("ft", &self.ft)]);
                let req = req.query(&[("id", &self.id)]);
                let req = req.query(&[("make", &self.make)]);
                let req = req.query(&[("model", &self.model)]);
                let req = req.query(&[("ot", &self.ot)]);
                let req = req.query(&[("product", &self.product)]);
                let req = req.query(&[("sn", &self.sn)]);
                let req = req.query(&[("vhid", &self.vhid)]);
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
        pub struct GrantRequestBuilder<'a, A> {
            pub(crate) reqwest: &'a ::reqwest::Client,
            pub(crate) auth: &'a ::std::sync::Mutex<A>,
            aid: Option<String>,
            apptype: Option<String>,
            cr: Option<String>,
            device: Option<String>,
            devid: Option<String>,
            devtype: Option<String>,
            exp: Option<String>,
            ft: Option<String>,
            lr: Option<String>,
            make: Option<String>,
            model: Option<String>,
            ofid: Option<String>,
            ot: Option<String>,
            product: Option<String>,
            sn: Option<String>,
            alt: Option<crate::params::Alt>,
            fields: Option<String>,
            key: Option<String>,
            oauth_token: Option<String>,
            pretty_print: Option<bool>,
            quota_user: Option<String>,
            user_ip: Option<String>,
        }
        impl<'a, A: yup_oauth2::GetToken> GrantRequestBuilder<'a, A> {
            #[doc = ""]
            pub fn aid(mut self, value: impl Into<String>) -> Self {
                self.aid = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn apptype(mut self, value: impl Into<String>) -> Self {
                self.apptype = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn cr(mut self, value: impl Into<String>) -> Self {
                self.cr = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn device(mut self, value: impl Into<String>) -> Self {
                self.device = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn devid(mut self, value: impl Into<String>) -> Self {
                self.devid = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn devtype(mut self, value: impl Into<String>) -> Self {
                self.devtype = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn exp(mut self, value: impl Into<String>) -> Self {
                self.exp = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn ft(mut self, value: impl Into<String>) -> Self {
                self.ft = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn lr(mut self, value: impl Into<String>) -> Self {
                self.lr = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn make(mut self, value: impl Into<String>) -> Self {
                self.make = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn model(mut self, value: impl Into<String>) -> Self {
                self.model = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn ofid(mut self, value: impl Into<String>) -> Self {
                self.ofid = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn ot(mut self, value: impl Into<String>) -> Self {
                self.ot = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn product(mut self, value: impl Into<String>) -> Self {
                self.product = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn sn(mut self, value: impl Into<String>) -> Self {
                self.sn = Some(value.into());
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
                let mut output = "https://www.googleapis.com/android_video/v1/".to_owned();
                output.push_str("purchase/grant");
                output
            }
            fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
                let req = self.reqwest.request(::reqwest::Method::POST, path);
                let req = req.query(&[("aid", &self.aid)]);
                let req = req.query(&[("apptype", &self.apptype)]);
                let req = req.query(&[("cr", &self.cr)]);
                let req = req.query(&[("device", &self.device)]);
                let req = req.query(&[("devid", &self.devid)]);
                let req = req.query(&[("devtype", &self.devtype)]);
                let req = req.query(&[("exp", &self.exp)]);
                let req = req.query(&[("ft", &self.ft)]);
                let req = req.query(&[("lr", &self.lr)]);
                let req = req.query(&[("make", &self.make)]);
                let req = req.query(&[("model", &self.model)]);
                let req = req.query(&[("ofid", &self.ofid)]);
                let req = req.query(&[("ot", &self.ot)]);
                let req = req.query(&[("product", &self.product)]);
                let req = req.query(&[("sn", &self.sn)]);
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
        pub struct PrepareRequestBuilder<'a, A> {
            pub(crate) reqwest: &'a ::reqwest::Client,
            pub(crate) auth: &'a ::std::sync::Mutex<A>,
            aid: Option<String>,
            apptype: Option<String>,
            auvh: Option<bool>,
            cr: Option<String>,
            device: Option<String>,
            devid: Option<String>,
            devtype: Option<String>,
            exp: Option<String>,
            ft: Option<String>,
            lr: Option<String>,
            make: Option<String>,
            model: Option<String>,
            ofid: Option<String>,
            ot: Option<String>,
            product: Option<String>,
            sn: Option<String>,
            vhid: Option<String>,
            alt: Option<crate::params::Alt>,
            fields: Option<String>,
            key: Option<String>,
            oauth_token: Option<String>,
            pretty_print: Option<bool>,
            quota_user: Option<String>,
            user_ip: Option<String>,
        }
        impl<'a, A: yup_oauth2::GetToken> PrepareRequestBuilder<'a, A> {
            #[doc = ""]
            pub fn aid(mut self, value: impl Into<String>) -> Self {
                self.aid = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn apptype(mut self, value: impl Into<String>) -> Self {
                self.apptype = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn auvh(mut self, value: bool) -> Self {
                self.auvh = Some(value);
                self
            }
            #[doc = ""]
            pub fn cr(mut self, value: impl Into<String>) -> Self {
                self.cr = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn device(mut self, value: impl Into<String>) -> Self {
                self.device = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn devid(mut self, value: impl Into<String>) -> Self {
                self.devid = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn devtype(mut self, value: impl Into<String>) -> Self {
                self.devtype = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn exp(mut self, value: impl Into<String>) -> Self {
                self.exp = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn ft(mut self, value: impl Into<String>) -> Self {
                self.ft = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn lr(mut self, value: impl Into<String>) -> Self {
                self.lr = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn make(mut self, value: impl Into<String>) -> Self {
                self.make = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn model(mut self, value: impl Into<String>) -> Self {
                self.model = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn ofid(mut self, value: impl Into<String>) -> Self {
                self.ofid = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn ot(mut self, value: impl Into<String>) -> Self {
                self.ot = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn product(mut self, value: impl Into<String>) -> Self {
                self.product = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn sn(mut self, value: impl Into<String>) -> Self {
                self.sn = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn vhid(mut self, value: impl Into<String>) -> Self {
                self.vhid = Some(value.into());
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
                let mut output = "https://www.googleapis.com/android_video/v1/".to_owned();
                output.push_str("purchase/prepare");
                output
            }
            fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
                let req = self.reqwest.request(::reqwest::Method::POST, path);
                let req = req.query(&[("aid", &self.aid)]);
                let req = req.query(&[("apptype", &self.apptype)]);
                let req = req.query(&[("auvh", &self.auvh)]);
                let req = req.query(&[("cr", &self.cr)]);
                let req = req.query(&[("device", &self.device)]);
                let req = req.query(&[("devid", &self.devid)]);
                let req = req.query(&[("devtype", &self.devtype)]);
                let req = req.query(&[("exp", &self.exp)]);
                let req = req.query(&[("ft", &self.ft)]);
                let req = req.query(&[("lr", &self.lr)]);
                let req = req.query(&[("make", &self.make)]);
                let req = req.query(&[("model", &self.model)]);
                let req = req.query(&[("ofid", &self.ofid)]);
                let req = req.query(&[("ot", &self.ot)]);
                let req = req.query(&[("product", &self.product)]);
                let req = req.query(&[("sn", &self.sn)]);
                let req = req.query(&[("vhid", &self.vhid)]);
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
    pub mod recommendation {
        pub mod params {}
        pub struct RecommendationActions<'a, A> {
            pub(crate) reqwest: &'a reqwest::Client,
            pub(crate) auth: &'a std::sync::Mutex<A>,
        }
        impl<'a, A: yup_oauth2::GetToken> RecommendationActions<'a, A> {
            #[doc = ""]
            pub fn findrelated(&self) -> FindrelatedRequestBuilder<A> {
                FindrelatedRequestBuilder {
                    reqwest: &self.reqwest,
                    auth: &self.auth,
                    alt: None,
                    fields: None,
                    key: None,
                    oauth_token: None,
                    pretty_print: None,
                    quota_user: None,
                    user_ip: None,
                }
            }
            #[doc = ""]
            pub fn getfeed(&self) -> GetfeedRequestBuilder<A> {
                GetfeedRequestBuilder {
                    reqwest: &self.reqwest,
                    auth: &self.auth,
                    alt: None,
                    fields: None,
                    key: None,
                    oauth_token: None,
                    pretty_print: None,
                    quota_user: None,
                    user_ip: None,
                    apptype: None,
                    cat: None,
                    cr: None,
                    device: None,
                    devid: None,
                    devtype: None,
                    exp: None,
                    feed: None,
                    fkey: None,
                    genres: None,
                    lr: None,
                    make: None,
                    max_results: None,
                    mc: None,
                    mcc_mnc: None,
                    model: None,
                    mrf: None,
                    page_token: None,
                    product: None,
                    r#if: None,
                    sn: None,
                    stags: None,
                    structure: None,
                    tagdbid: None,
                    tagtype: None,
                    ttype: None,
                    tvrf: None,
                    types: None,
                    utoken: None,
                }
            }
            #[doc = ""]
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
                    apptype: None,
                    cat: None,
                    cr: None,
                    device: None,
                    devid: None,
                    devtype: None,
                    exp: None,
                    lr: None,
                    make: None,
                    max: None,
                    mcc_mnc: None,
                    model: None,
                    mrf: None,
                    product: None,
                    q: None,
                    r#if: None,
                    safe: None,
                    sn: None,
                    tvrf: None,
                    utoken: None,
                    utype: None,
                }
            }
            #[doc = ""]
            pub fn readrecommendationfeedback(
                &self,
            ) -> ReadrecommendationfeedbackRequestBuilder<A> {
                ReadrecommendationfeedbackRequestBuilder {
                    reqwest: &self.reqwest,
                    auth: &self.auth,
                    alt: None,
                    fields: None,
                    key: None,
                    oauth_token: None,
                    pretty_print: None,
                    quota_user: None,
                    user_ip: None,
                    apptype: None,
                    cr: None,
                    device: None,
                    devid: None,
                    devtype: None,
                    exp: None,
                    lr: None,
                    make: None,
                    mcc_mnc: None,
                    model: None,
                    product: None,
                    r#type: None,
                    sn: None,
                    token: None,
                    utoken: None,
                }
            }
            #[doc = ""]
            pub fn updaterecommendationfeedback(
                &self,
            ) -> UpdaterecommendationfeedbackRequestBuilder<A> {
                UpdaterecommendationfeedbackRequestBuilder {
                    reqwest: &self.reqwest,
                    auth: &self.auth,
                    alt: None,
                    fields: None,
                    key: None,
                    oauth_token: None,
                    pretty_print: None,
                    quota_user: None,
                    user_ip: None,
                    apptype: None,
                    cr: None,
                    device: None,
                    devid: None,
                    devtype: None,
                    exp: None,
                    lr: None,
                    make: None,
                    mcc_mnc: None,
                    model: None,
                    product: None,
                    sn: None,
                    utoken: None,
                }
            }
        }
        #[derive(Debug, Clone)]
        pub struct FindrelatedRequestBuilder<'a, A> {
            pub(crate) reqwest: &'a ::reqwest::Client,
            pub(crate) auth: &'a ::std::sync::Mutex<A>,
            alt: Option<crate::params::Alt>,
            fields: Option<String>,
            key: Option<String>,
            oauth_token: Option<String>,
            pretty_print: Option<bool>,
            quota_user: Option<String>,
            user_ip: Option<String>,
        }
        impl<'a, A: yup_oauth2::GetToken> FindrelatedRequestBuilder<'a, A> {
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
                let mut output = "https://www.googleapis.com/android_video/v1/".to_owned();
                output.push_str("recommendation/findrelated");
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
        pub struct GetfeedRequestBuilder<'a, A> {
            pub(crate) reqwest: &'a ::reqwest::Client,
            pub(crate) auth: &'a ::std::sync::Mutex<A>,
            apptype: Option<String>,
            cat: Option<String>,
            cr: Option<String>,
            device: Option<String>,
            devid: Option<String>,
            devtype: Option<String>,
            exp: Option<String>,
            feed: Option<u32>,
            fkey: Option<Vec<String>>,
            genres: Option<Vec<String>>,
            lr: Option<String>,
            make: Option<String>,
            max_results: Option<u32>,
            mc: Option<u32>,
            mcc_mnc: Option<String>,
            model: Option<String>,
            mrf: Option<String>,
            page_token: Option<String>,
            product: Option<String>,
            r#if: Option<String>,
            sn: Option<String>,
            stags: Option<Vec<String>>,
            structure: Option<u32>,
            tagdbid: Option<String>,
            tagtype: Option<String>,
            ttype: Option<String>,
            tvrf: Option<String>,
            types: Option<Vec<String>>,
            utoken: Option<String>,
            alt: Option<crate::params::Alt>,
            fields: Option<String>,
            key: Option<String>,
            oauth_token: Option<String>,
            pretty_print: Option<bool>,
            quota_user: Option<String>,
            user_ip: Option<String>,
        }
        impl<'a, A: yup_oauth2::GetToken> GetfeedRequestBuilder<'a, A> {
            #[doc = ""]
            pub fn apptype(mut self, value: impl Into<String>) -> Self {
                self.apptype = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn cat(mut self, value: impl Into<String>) -> Self {
                self.cat = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn cr(mut self, value: impl Into<String>) -> Self {
                self.cr = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn device(mut self, value: impl Into<String>) -> Self {
                self.device = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn devid(mut self, value: impl Into<String>) -> Self {
                self.devid = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn devtype(mut self, value: impl Into<String>) -> Self {
                self.devtype = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn exp(mut self, value: impl Into<String>) -> Self {
                self.exp = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn feed(mut self, value: u32) -> Self {
                self.feed = Some(value);
                self
            }
            #[doc = ""]
            pub fn fkey(mut self, value: impl Into<Vec<String>>) -> Self {
                self.fkey = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn genres(mut self, value: impl Into<Vec<String>>) -> Self {
                self.genres = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn lr(mut self, value: impl Into<String>) -> Self {
                self.lr = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn make(mut self, value: impl Into<String>) -> Self {
                self.make = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn max_results(mut self, value: u32) -> Self {
                self.max_results = Some(value);
                self
            }
            #[doc = ""]
            pub fn mc(mut self, value: u32) -> Self {
                self.mc = Some(value);
                self
            }
            #[doc = ""]
            pub fn mcc_mnc(mut self, value: impl Into<String>) -> Self {
                self.mcc_mnc = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn model(mut self, value: impl Into<String>) -> Self {
                self.model = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn mrf(mut self, value: impl Into<String>) -> Self {
                self.mrf = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn page_token(mut self, value: impl Into<String>) -> Self {
                self.page_token = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn product(mut self, value: impl Into<String>) -> Self {
                self.product = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn r#if(mut self, value: impl Into<String>) -> Self {
                self.r#if = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn sn(mut self, value: impl Into<String>) -> Self {
                self.sn = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn stags(mut self, value: impl Into<Vec<String>>) -> Self {
                self.stags = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn structure(mut self, value: u32) -> Self {
                self.structure = Some(value);
                self
            }
            #[doc = ""]
            pub fn tagdbid(mut self, value: impl Into<String>) -> Self {
                self.tagdbid = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn tagtype(mut self, value: impl Into<String>) -> Self {
                self.tagtype = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn ttype(mut self, value: impl Into<String>) -> Self {
                self.ttype = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn tvrf(mut self, value: impl Into<String>) -> Self {
                self.tvrf = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn types(mut self, value: impl Into<Vec<String>>) -> Self {
                self.types = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn utoken(mut self, value: impl Into<String>) -> Self {
                self.utoken = Some(value.into());
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
                let mut output = "https://www.googleapis.com/android_video/v1/".to_owned();
                output.push_str("recommendation/feed");
                output
            }
            fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
                let req = self.reqwest.request(::reqwest::Method::GET, path);
                let req = req.query(&[("apptype", &self.apptype)]);
                let req = req.query(&[("cat", &self.cat)]);
                let req = req.query(&[("cr", &self.cr)]);
                let req = req.query(&[("device", &self.device)]);
                let req = req.query(&[("devid", &self.devid)]);
                let req = req.query(&[("devtype", &self.devtype)]);
                let req = req.query(&[("exp", &self.exp)]);
                let req = req.query(&[("feed", &self.feed)]);
                let req = req.query(&[("fkey", &self.fkey)]);
                let req = req.query(&[("genres", &self.genres)]);
                let req = req.query(&[("lr", &self.lr)]);
                let req = req.query(&[("make", &self.make)]);
                let req = req.query(&[("maxResults", &self.max_results)]);
                let req = req.query(&[("mc", &self.mc)]);
                let req = req.query(&[("mcc_mnc", &self.mcc_mnc)]);
                let req = req.query(&[("model", &self.model)]);
                let req = req.query(&[("mrf", &self.mrf)]);
                let req = req.query(&[("pageToken", &self.page_token)]);
                let req = req.query(&[("product", &self.product)]);
                let req = req.query(&[("if", &self.r#if)]);
                let req = req.query(&[("sn", &self.sn)]);
                let req = req.query(&[("stags", &self.stags)]);
                let req = req.query(&[("structure", &self.structure)]);
                let req = req.query(&[("tagdbid", &self.tagdbid)]);
                let req = req.query(&[("tagtype", &self.tagtype)]);
                let req = req.query(&[("ttype", &self.ttype)]);
                let req = req.query(&[("tvrf", &self.tvrf)]);
                let req = req.query(&[("types", &self.types)]);
                let req = req.query(&[("utoken", &self.utoken)]);
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
            apptype: Option<String>,
            cat: Option<String>,
            cr: Option<String>,
            device: Option<String>,
            devid: Option<String>,
            devtype: Option<String>,
            exp: Option<String>,
            lr: Option<String>,
            make: Option<String>,
            max: Option<i32>,
            mcc_mnc: Option<String>,
            model: Option<String>,
            mrf: Option<String>,
            product: Option<String>,
            q: Option<String>,
            r#if: Option<String>,
            safe: Option<String>,
            sn: Option<String>,
            tvrf: Option<String>,
            utoken: Option<String>,
            utype: Option<u32>,
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
            pub fn apptype(mut self, value: impl Into<String>) -> Self {
                self.apptype = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn cat(mut self, value: impl Into<String>) -> Self {
                self.cat = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn cr(mut self, value: impl Into<String>) -> Self {
                self.cr = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn device(mut self, value: impl Into<String>) -> Self {
                self.device = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn devid(mut self, value: impl Into<String>) -> Self {
                self.devid = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn devtype(mut self, value: impl Into<String>) -> Self {
                self.devtype = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn exp(mut self, value: impl Into<String>) -> Self {
                self.exp = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn lr(mut self, value: impl Into<String>) -> Self {
                self.lr = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn make(mut self, value: impl Into<String>) -> Self {
                self.make = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn max(mut self, value: i32) -> Self {
                self.max = Some(value);
                self
            }
            #[doc = ""]
            pub fn mcc_mnc(mut self, value: impl Into<String>) -> Self {
                self.mcc_mnc = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn model(mut self, value: impl Into<String>) -> Self {
                self.model = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn mrf(mut self, value: impl Into<String>) -> Self {
                self.mrf = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn product(mut self, value: impl Into<String>) -> Self {
                self.product = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn q(mut self, value: impl Into<String>) -> Self {
                self.q = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn r#if(mut self, value: impl Into<String>) -> Self {
                self.r#if = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn safe(mut self, value: impl Into<String>) -> Self {
                self.safe = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn sn(mut self, value: impl Into<String>) -> Self {
                self.sn = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn tvrf(mut self, value: impl Into<String>) -> Self {
                self.tvrf = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn utoken(mut self, value: impl Into<String>) -> Self {
                self.utoken = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn utype(mut self, value: u32) -> Self {
                self.utype = Some(value);
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
                let mut output = "https://www.googleapis.com/android_video/v1/".to_owned();
                output.push_str("recommendation");
                output
            }
            fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
                let req = self.reqwest.request(::reqwest::Method::GET, path);
                let req = req.query(&[("apptype", &self.apptype)]);
                let req = req.query(&[("cat", &self.cat)]);
                let req = req.query(&[("cr", &self.cr)]);
                let req = req.query(&[("device", &self.device)]);
                let req = req.query(&[("devid", &self.devid)]);
                let req = req.query(&[("devtype", &self.devtype)]);
                let req = req.query(&[("exp", &self.exp)]);
                let req = req.query(&[("lr", &self.lr)]);
                let req = req.query(&[("make", &self.make)]);
                let req = req.query(&[("max", &self.max)]);
                let req = req.query(&[("mcc_mnc", &self.mcc_mnc)]);
                let req = req.query(&[("model", &self.model)]);
                let req = req.query(&[("mrf", &self.mrf)]);
                let req = req.query(&[("product", &self.product)]);
                let req = req.query(&[("q", &self.q)]);
                let req = req.query(&[("if", &self.r#if)]);
                let req = req.query(&[("safe", &self.safe)]);
                let req = req.query(&[("sn", &self.sn)]);
                let req = req.query(&[("tvrf", &self.tvrf)]);
                let req = req.query(&[("utoken", &self.utoken)]);
                let req = req.query(&[("utype", &self.utype)]);
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
        pub struct ReadrecommendationfeedbackRequestBuilder<'a, A> {
            pub(crate) reqwest: &'a ::reqwest::Client,
            pub(crate) auth: &'a ::std::sync::Mutex<A>,
            apptype: Option<String>,
            cr: Option<String>,
            device: Option<String>,
            devid: Option<String>,
            devtype: Option<String>,
            exp: Option<String>,
            lr: Option<String>,
            make: Option<String>,
            mcc_mnc: Option<String>,
            model: Option<String>,
            product: Option<String>,
            r#type: Option<u32>,
            sn: Option<String>,
            token: Option<String>,
            utoken: Option<String>,
            alt: Option<crate::params::Alt>,
            fields: Option<String>,
            key: Option<String>,
            oauth_token: Option<String>,
            pretty_print: Option<bool>,
            quota_user: Option<String>,
            user_ip: Option<String>,
        }
        impl<'a, A: yup_oauth2::GetToken> ReadrecommendationfeedbackRequestBuilder<'a, A> {
            #[doc = ""]
            pub fn apptype(mut self, value: impl Into<String>) -> Self {
                self.apptype = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn cr(mut self, value: impl Into<String>) -> Self {
                self.cr = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn device(mut self, value: impl Into<String>) -> Self {
                self.device = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn devid(mut self, value: impl Into<String>) -> Self {
                self.devid = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn devtype(mut self, value: impl Into<String>) -> Self {
                self.devtype = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn exp(mut self, value: impl Into<String>) -> Self {
                self.exp = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn lr(mut self, value: impl Into<String>) -> Self {
                self.lr = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn make(mut self, value: impl Into<String>) -> Self {
                self.make = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn mcc_mnc(mut self, value: impl Into<String>) -> Self {
                self.mcc_mnc = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn model(mut self, value: impl Into<String>) -> Self {
                self.model = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn product(mut self, value: impl Into<String>) -> Self {
                self.product = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn r#type(mut self, value: u32) -> Self {
                self.r#type = Some(value);
                self
            }
            #[doc = ""]
            pub fn sn(mut self, value: impl Into<String>) -> Self {
                self.sn = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn token(mut self, value: impl Into<String>) -> Self {
                self.token = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn utoken(mut self, value: impl Into<String>) -> Self {
                self.utoken = Some(value.into());
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
                let mut output = "https://www.googleapis.com/android_video/v1/".to_owned();
                output.push_str("recommendation/feedback");
                output
            }
            fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
                let req = self.reqwest.request(::reqwest::Method::GET, path);
                let req = req.query(&[("apptype", &self.apptype)]);
                let req = req.query(&[("cr", &self.cr)]);
                let req = req.query(&[("device", &self.device)]);
                let req = req.query(&[("devid", &self.devid)]);
                let req = req.query(&[("devtype", &self.devtype)]);
                let req = req.query(&[("exp", &self.exp)]);
                let req = req.query(&[("lr", &self.lr)]);
                let req = req.query(&[("make", &self.make)]);
                let req = req.query(&[("mcc_mnc", &self.mcc_mnc)]);
                let req = req.query(&[("model", &self.model)]);
                let req = req.query(&[("product", &self.product)]);
                let req = req.query(&[("type", &self.r#type)]);
                let req = req.query(&[("sn", &self.sn)]);
                let req = req.query(&[("token", &self.token)]);
                let req = req.query(&[("utoken", &self.utoken)]);
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
        pub struct UpdaterecommendationfeedbackRequestBuilder<'a, A> {
            pub(crate) reqwest: &'a ::reqwest::Client,
            pub(crate) auth: &'a ::std::sync::Mutex<A>,
            apptype: Option<String>,
            cr: Option<String>,
            device: Option<String>,
            devid: Option<String>,
            devtype: Option<String>,
            exp: Option<String>,
            lr: Option<String>,
            make: Option<String>,
            mcc_mnc: Option<String>,
            model: Option<String>,
            product: Option<String>,
            sn: Option<String>,
            utoken: Option<String>,
            alt: Option<crate::params::Alt>,
            fields: Option<String>,
            key: Option<String>,
            oauth_token: Option<String>,
            pretty_print: Option<bool>,
            quota_user: Option<String>,
            user_ip: Option<String>,
        }
        impl<'a, A: yup_oauth2::GetToken> UpdaterecommendationfeedbackRequestBuilder<'a, A> {
            #[doc = ""]
            pub fn apptype(mut self, value: impl Into<String>) -> Self {
                self.apptype = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn cr(mut self, value: impl Into<String>) -> Self {
                self.cr = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn device(mut self, value: impl Into<String>) -> Self {
                self.device = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn devid(mut self, value: impl Into<String>) -> Self {
                self.devid = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn devtype(mut self, value: impl Into<String>) -> Self {
                self.devtype = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn exp(mut self, value: impl Into<String>) -> Self {
                self.exp = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn lr(mut self, value: impl Into<String>) -> Self {
                self.lr = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn make(mut self, value: impl Into<String>) -> Self {
                self.make = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn mcc_mnc(mut self, value: impl Into<String>) -> Self {
                self.mcc_mnc = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn model(mut self, value: impl Into<String>) -> Self {
                self.model = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn product(mut self, value: impl Into<String>) -> Self {
                self.product = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn sn(mut self, value: impl Into<String>) -> Self {
                self.sn = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn utoken(mut self, value: impl Into<String>) -> Self {
                self.utoken = Some(value.into());
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
                let mut output = "https://www.googleapis.com/android_video/v1/".to_owned();
                output.push_str("recommendation/updatefeedback");
                output
            }
            fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
                let req = self.reqwest.request(::reqwest::Method::POST, path);
                let req = req.query(&[("apptype", &self.apptype)]);
                let req = req.query(&[("cr", &self.cr)]);
                let req = req.query(&[("device", &self.device)]);
                let req = req.query(&[("devid", &self.devid)]);
                let req = req.query(&[("devtype", &self.devtype)]);
                let req = req.query(&[("exp", &self.exp)]);
                let req = req.query(&[("lr", &self.lr)]);
                let req = req.query(&[("make", &self.make)]);
                let req = req.query(&[("mcc_mnc", &self.mcc_mnc)]);
                let req = req.query(&[("model", &self.model)]);
                let req = req.query(&[("product", &self.product)]);
                let req = req.query(&[("sn", &self.sn)]);
                let req = req.query(&[("utoken", &self.utoken)]);
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
    pub mod search {
        pub mod params {}
        pub struct SearchActions<'a, A> {
            pub(crate) reqwest: &'a reqwest::Client,
            pub(crate) auth: &'a std::sync::Mutex<A>,
        }
        impl<'a, A: yup_oauth2::GetToken> SearchActions<'a, A> {
            #[doc = ""]
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
                    apptype: None,
                    cat: None,
                    cf: None,
                    cr: None,
                    device: None,
                    devid: None,
                    devtype: None,
                    exp: None,
                    fs: None,
                    lr: None,
                    make: None,
                    max_results: None,
                    mc: None,
                    model: None,
                    mrf: None,
                    page_token: None,
                    product: None,
                    q: None,
                    sn: None,
                    stags: None,
                    tagdbid: None,
                    tagtype: None,
                    tvrf: None,
                }
            }
        }
        #[derive(Debug, Clone)]
        pub struct ListRequestBuilder<'a, A> {
            pub(crate) reqwest: &'a ::reqwest::Client,
            pub(crate) auth: &'a ::std::sync::Mutex<A>,
            apptype: Option<String>,
            cat: Option<String>,
            cf: Option<Vec<String>>,
            cr: Option<String>,
            device: Option<String>,
            devid: Option<String>,
            devtype: Option<String>,
            exp: Option<String>,
            fs: Option<String>,
            lr: Option<String>,
            make: Option<String>,
            max_results: Option<u32>,
            mc: Option<u32>,
            model: Option<String>,
            mrf: Option<String>,
            page_token: Option<String>,
            product: Option<String>,
            q: Option<String>,
            sn: Option<String>,
            stags: Option<Vec<String>>,
            tagdbid: Option<String>,
            tagtype: Option<String>,
            tvrf: Option<String>,
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
            pub fn apptype(mut self, value: impl Into<String>) -> Self {
                self.apptype = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn cat(mut self, value: impl Into<String>) -> Self {
                self.cat = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn cf(mut self, value: impl Into<Vec<String>>) -> Self {
                self.cf = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn cr(mut self, value: impl Into<String>) -> Self {
                self.cr = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn device(mut self, value: impl Into<String>) -> Self {
                self.device = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn devid(mut self, value: impl Into<String>) -> Self {
                self.devid = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn devtype(mut self, value: impl Into<String>) -> Self {
                self.devtype = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn exp(mut self, value: impl Into<String>) -> Self {
                self.exp = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn fs(mut self, value: impl Into<String>) -> Self {
                self.fs = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn lr(mut self, value: impl Into<String>) -> Self {
                self.lr = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn make(mut self, value: impl Into<String>) -> Self {
                self.make = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn max_results(mut self, value: u32) -> Self {
                self.max_results = Some(value);
                self
            }
            #[doc = ""]
            pub fn mc(mut self, value: u32) -> Self {
                self.mc = Some(value);
                self
            }
            #[doc = ""]
            pub fn model(mut self, value: impl Into<String>) -> Self {
                self.model = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn mrf(mut self, value: impl Into<String>) -> Self {
                self.mrf = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn page_token(mut self, value: impl Into<String>) -> Self {
                self.page_token = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn product(mut self, value: impl Into<String>) -> Self {
                self.product = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn q(mut self, value: impl Into<String>) -> Self {
                self.q = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn sn(mut self, value: impl Into<String>) -> Self {
                self.sn = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn stags(mut self, value: impl Into<Vec<String>>) -> Self {
                self.stags = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn tagdbid(mut self, value: impl Into<String>) -> Self {
                self.tagdbid = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn tagtype(mut self, value: impl Into<String>) -> Self {
                self.tagtype = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn tvrf(mut self, value: impl Into<String>) -> Self {
                self.tvrf = Some(value.into());
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
                let mut output = "https://www.googleapis.com/android_video/v1/".to_owned();
                output.push_str("search");
                output
            }
            fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
                let req = self.reqwest.request(::reqwest::Method::GET, path);
                let req = req.query(&[("apptype", &self.apptype)]);
                let req = req.query(&[("cat", &self.cat)]);
                let req = req.query(&[("cf", &self.cf)]);
                let req = req.query(&[("cr", &self.cr)]);
                let req = req.query(&[("device", &self.device)]);
                let req = req.query(&[("devid", &self.devid)]);
                let req = req.query(&[("devtype", &self.devtype)]);
                let req = req.query(&[("exp", &self.exp)]);
                let req = req.query(&[("fs", &self.fs)]);
                let req = req.query(&[("lr", &self.lr)]);
                let req = req.query(&[("make", &self.make)]);
                let req = req.query(&[("maxResults", &self.max_results)]);
                let req = req.query(&[("mc", &self.mc)]);
                let req = req.query(&[("model", &self.model)]);
                let req = req.query(&[("mrf", &self.mrf)]);
                let req = req.query(&[("pageToken", &self.page_token)]);
                let req = req.query(&[("product", &self.product)]);
                let req = req.query(&[("q", &self.q)]);
                let req = req.query(&[("sn", &self.sn)]);
                let req = req.query(&[("stags", &self.stags)]);
                let req = req.query(&[("tagdbid", &self.tagdbid)]);
                let req = req.query(&[("tagtype", &self.tagtype)]);
                let req = req.query(&[("tvrf", &self.tvrf)]);
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
    pub mod userconfig {
        pub mod params {}
        pub struct UserconfigActions<'a, A> {
            pub(crate) reqwest: &'a reqwest::Client,
            pub(crate) auth: &'a std::sync::Mutex<A>,
        }
        impl<'a, A: yup_oauth2::GetToken> UserconfigActions<'a, A> {
            #[doc = ""]
            pub fn get(&self) -> GetRequestBuilder<A> {
                GetRequestBuilder {
                    reqwest: &self.reqwest,
                    auth: &self.auth,
                    alt: None,
                    fields: None,
                    key: None,
                    oauth_token: None,
                    pretty_print: None,
                    quota_user: None,
                    user_ip: None,
                    an: None,
                    apptype: None,
                    av: None,
                    cr: None,
                    device: None,
                    devid: None,
                    devtype: None,
                    exp: None,
                    fs: None,
                    make: None,
                    model: None,
                    platform: None,
                    product: None,
                    sn: None,
                }
            }
        }
        #[derive(Debug, Clone)]
        pub struct GetRequestBuilder<'a, A> {
            pub(crate) reqwest: &'a ::reqwest::Client,
            pub(crate) auth: &'a ::std::sync::Mutex<A>,
            an: Option<String>,
            apptype: Option<String>,
            av: Option<String>,
            cr: Option<String>,
            device: Option<String>,
            devid: Option<String>,
            devtype: Option<String>,
            exp: Option<String>,
            fs: Option<String>,
            make: Option<String>,
            model: Option<String>,
            platform: Option<String>,
            product: Option<String>,
            sn: Option<String>,
            alt: Option<crate::params::Alt>,
            fields: Option<String>,
            key: Option<String>,
            oauth_token: Option<String>,
            pretty_print: Option<bool>,
            quota_user: Option<String>,
            user_ip: Option<String>,
        }
        impl<'a, A: yup_oauth2::GetToken> GetRequestBuilder<'a, A> {
            #[doc = ""]
            pub fn an(mut self, value: impl Into<String>) -> Self {
                self.an = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn apptype(mut self, value: impl Into<String>) -> Self {
                self.apptype = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn av(mut self, value: impl Into<String>) -> Self {
                self.av = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn cr(mut self, value: impl Into<String>) -> Self {
                self.cr = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn device(mut self, value: impl Into<String>) -> Self {
                self.device = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn devid(mut self, value: impl Into<String>) -> Self {
                self.devid = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn devtype(mut self, value: impl Into<String>) -> Self {
                self.devtype = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn exp(mut self, value: impl Into<String>) -> Self {
                self.exp = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn fs(mut self, value: impl Into<String>) -> Self {
                self.fs = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn make(mut self, value: impl Into<String>) -> Self {
                self.make = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn model(mut self, value: impl Into<String>) -> Self {
                self.model = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn platform(mut self, value: impl Into<String>) -> Self {
                self.platform = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn product(mut self, value: impl Into<String>) -> Self {
                self.product = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn sn(mut self, value: impl Into<String>) -> Self {
                self.sn = Some(value.into());
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
                let mut output = "https://www.googleapis.com/android_video/v1/".to_owned();
                output.push_str("config");
                output
            }
            fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
                let req = self.reqwest.request(::reqwest::Method::GET, path);
                let req = req.query(&[("an", &self.an)]);
                let req = req.query(&[("apptype", &self.apptype)]);
                let req = req.query(&[("av", &self.av)]);
                let req = req.query(&[("cr", &self.cr)]);
                let req = req.query(&[("device", &self.device)]);
                let req = req.query(&[("devid", &self.devid)]);
                let req = req.query(&[("devtype", &self.devtype)]);
                let req = req.query(&[("exp", &self.exp)]);
                let req = req.query(&[("fs", &self.fs)]);
                let req = req.query(&[("make", &self.make)]);
                let req = req.query(&[("model", &self.model)]);
                let req = req.query(&[("platform", &self.platform)]);
                let req = req.query(&[("product", &self.product)]);
                let req = req.query(&[("sn", &self.sn)]);
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
    pub mod userlibrary {
        pub mod params {}
        pub struct UserlibraryActions<'a, A> {
            pub(crate) reqwest: &'a reqwest::Client,
            pub(crate) auth: &'a std::sync::Mutex<A>,
        }
        impl<'a, A: yup_oauth2::GetToken> UserlibraryActions<'a, A> {
            #[doc = ""]
            pub fn hide(&self) -> HideRequestBuilder<A> {
                HideRequestBuilder {
                    reqwest: &self.reqwest,
                    auth: &self.auth,
                    alt: None,
                    fields: None,
                    key: None,
                    oauth_token: None,
                    pretty_print: None,
                    quota_user: None,
                    user_ip: None,
                    aid: None,
                    apptype: None,
                    device: None,
                    devid: None,
                    devtype: None,
                    exp: None,
                    make: None,
                    model: None,
                    product: None,
                    sn: None,
                }
            }
            #[doc = ""]
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
                    ac: None,
                    aid: None,
                    apptype: None,
                    cr: None,
                    device: None,
                    devid: None,
                    devtype: None,
                    exp: None,
                    fs: None,
                    lr: None,
                    make: None,
                    max_results: None,
                    model: None,
                    or: None,
                    page_token: None,
                    product: None,
                    sn: None,
                    snapshot_token: None,
                    tr: None,
                }
            }
            #[doc = ""]
            pub fn listplaybacks(&self) -> ListplaybacksRequestBuilder<A> {
                ListplaybacksRequestBuilder {
                    reqwest: &self.reqwest,
                    auth: &self.auth,
                    alt: None,
                    fields: None,
                    key: None,
                    oauth_token: None,
                    pretty_print: None,
                    quota_user: None,
                    user_ip: None,
                    apptype: None,
                    cr: None,
                    device: None,
                    devid: None,
                    devtype: None,
                    exp: None,
                    fs: None,
                    lr: None,
                    make: None,
                    max_results: None,
                    model: None,
                    product: None,
                    sn: None,
                    tr: None,
                    wts: None,
                }
            }
            #[doc = ""]
            pub fn playnext(&self) -> PlaynextRequestBuilder<A> {
                PlaynextRequestBuilder {
                    reqwest: &self.reqwest,
                    auth: &self.auth,
                    alt: None,
                    fields: None,
                    key: None,
                    oauth_token: None,
                    pretty_print: None,
                    quota_user: None,
                    user_ip: None,
                    aid: None,
                    apptype: None,
                    device: None,
                    devid: None,
                    devtype: None,
                    exp: None,
                    make: None,
                    max: None,
                    model: None,
                    playlist: None,
                    product: None,
                    rev: None,
                    sn: None,
                }
            }
            #[doc = ""]
            pub fn share(&self) -> ShareRequestBuilder<A> {
                ShareRequestBuilder {
                    reqwest: &self.reqwest,
                    auth: &self.auth,
                    alt: None,
                    fields: None,
                    key: None,
                    oauth_token: None,
                    pretty_print: None,
                    quota_user: None,
                    user_ip: None,
                    aid: None,
                    apptype: None,
                    device: None,
                    devid: None,
                    devtype: None,
                    exp: None,
                    make: None,
                    model: None,
                    product: None,
                    r#type: None,
                    sn: None,
                }
            }
            #[doc = ""]
            pub fn unhide(&self) -> UnhideRequestBuilder<A> {
                UnhideRequestBuilder {
                    reqwest: &self.reqwest,
                    auth: &self.auth,
                    alt: None,
                    fields: None,
                    key: None,
                    oauth_token: None,
                    pretty_print: None,
                    quota_user: None,
                    user_ip: None,
                    aid: None,
                    apptype: None,
                    device: None,
                    devid: None,
                    devtype: None,
                    exp: None,
                    make: None,
                    model: None,
                    product: None,
                    sn: None,
                }
            }
            #[doc = ""]
            pub fn unshare(&self) -> UnshareRequestBuilder<A> {
                UnshareRequestBuilder {
                    reqwest: &self.reqwest,
                    auth: &self.auth,
                    alt: None,
                    fields: None,
                    key: None,
                    oauth_token: None,
                    pretty_print: None,
                    quota_user: None,
                    user_ip: None,
                    aid: None,
                    apptype: None,
                    device: None,
                    devid: None,
                    devtype: None,
                    exp: None,
                    make: None,
                    model: None,
                    product: None,
                    r#type: None,
                    sn: None,
                }
            }
            #[doc = ""]
            pub fn watchnext(&self) -> WatchnextRequestBuilder<A> {
                WatchnextRequestBuilder {
                    reqwest: &self.reqwest,
                    auth: &self.auth,
                    alt: None,
                    fields: None,
                    key: None,
                    oauth_token: None,
                    pretty_print: None,
                    quota_user: None,
                    user_ip: None,
                    apptype: None,
                    cr: None,
                    device: None,
                    devid: None,
                    devtype: None,
                    exp: None,
                    fs: None,
                    lr: None,
                    make: None,
                    max_per_type: None,
                    max_results: None,
                    model: None,
                    mrf: None,
                    page_token: None,
                    product: None,
                    sn: None,
                    snapshot_token: None,
                    tr: None,
                    tvrf: None,
                    ver: None,
                }
            }
        }
        #[derive(Debug, Clone)]
        pub struct HideRequestBuilder<'a, A> {
            pub(crate) reqwest: &'a ::reqwest::Client,
            pub(crate) auth: &'a ::std::sync::Mutex<A>,
            aid: Option<Vec<String>>,
            apptype: Option<String>,
            device: Option<String>,
            devid: Option<String>,
            devtype: Option<String>,
            exp: Option<String>,
            make: Option<String>,
            model: Option<String>,
            product: Option<String>,
            sn: Option<String>,
            alt: Option<crate::params::Alt>,
            fields: Option<String>,
            key: Option<String>,
            oauth_token: Option<String>,
            pretty_print: Option<bool>,
            quota_user: Option<String>,
            user_ip: Option<String>,
        }
        impl<'a, A: yup_oauth2::GetToken> HideRequestBuilder<'a, A> {
            #[doc = ""]
            pub fn aid(mut self, value: impl Into<Vec<String>>) -> Self {
                self.aid = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn apptype(mut self, value: impl Into<String>) -> Self {
                self.apptype = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn device(mut self, value: impl Into<String>) -> Self {
                self.device = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn devid(mut self, value: impl Into<String>) -> Self {
                self.devid = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn devtype(mut self, value: impl Into<String>) -> Self {
                self.devtype = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn exp(mut self, value: impl Into<String>) -> Self {
                self.exp = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn make(mut self, value: impl Into<String>) -> Self {
                self.make = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn model(mut self, value: impl Into<String>) -> Self {
                self.model = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn product(mut self, value: impl Into<String>) -> Self {
                self.product = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn sn(mut self, value: impl Into<String>) -> Self {
                self.sn = Some(value.into());
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
                let mut output = "https://www.googleapis.com/android_video/v1/".to_owned();
                output.push_str("userlibrary/hide");
                output
            }
            fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
                let req = self.reqwest.request(::reqwest::Method::POST, path);
                let req = req.query(&[("aid", &self.aid)]);
                let req = req.query(&[("apptype", &self.apptype)]);
                let req = req.query(&[("device", &self.device)]);
                let req = req.query(&[("devid", &self.devid)]);
                let req = req.query(&[("devtype", &self.devtype)]);
                let req = req.query(&[("exp", &self.exp)]);
                let req = req.query(&[("make", &self.make)]);
                let req = req.query(&[("model", &self.model)]);
                let req = req.query(&[("product", &self.product)]);
                let req = req.query(&[("sn", &self.sn)]);
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
            ac: Option<bool>,
            aid: Option<Vec<String>>,
            apptype: Option<String>,
            cr: Option<String>,
            device: Option<String>,
            devid: Option<String>,
            devtype: Option<String>,
            exp: Option<String>,
            fs: Option<String>,
            lr: Option<String>,
            make: Option<String>,
            max_results: Option<u32>,
            model: Option<String>,
            or: Option<String>,
            page_token: Option<String>,
            product: Option<String>,
            sn: Option<String>,
            snapshot_token: Option<String>,
            tr: Option<Vec<String>>,
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
            pub fn ac(mut self, value: bool) -> Self {
                self.ac = Some(value);
                self
            }
            #[doc = ""]
            pub fn aid(mut self, value: impl Into<Vec<String>>) -> Self {
                self.aid = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn apptype(mut self, value: impl Into<String>) -> Self {
                self.apptype = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn cr(mut self, value: impl Into<String>) -> Self {
                self.cr = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn device(mut self, value: impl Into<String>) -> Self {
                self.device = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn devid(mut self, value: impl Into<String>) -> Self {
                self.devid = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn devtype(mut self, value: impl Into<String>) -> Self {
                self.devtype = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn exp(mut self, value: impl Into<String>) -> Self {
                self.exp = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn fs(mut self, value: impl Into<String>) -> Self {
                self.fs = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn lr(mut self, value: impl Into<String>) -> Self {
                self.lr = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn make(mut self, value: impl Into<String>) -> Self {
                self.make = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn max_results(mut self, value: u32) -> Self {
                self.max_results = Some(value);
                self
            }
            #[doc = ""]
            pub fn model(mut self, value: impl Into<String>) -> Self {
                self.model = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn or(mut self, value: impl Into<String>) -> Self {
                self.or = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn page_token(mut self, value: impl Into<String>) -> Self {
                self.page_token = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn product(mut self, value: impl Into<String>) -> Self {
                self.product = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn sn(mut self, value: impl Into<String>) -> Self {
                self.sn = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn snapshot_token(mut self, value: impl Into<String>) -> Self {
                self.snapshot_token = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn tr(mut self, value: impl Into<Vec<String>>) -> Self {
                self.tr = Some(value.into());
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
                let mut output = "https://www.googleapis.com/android_video/v1/".to_owned();
                output.push_str("userlibrary");
                output
            }
            fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
                let req = self.reqwest.request(::reqwest::Method::GET, path);
                let req = req.query(&[("ac", &self.ac)]);
                let req = req.query(&[("aid", &self.aid)]);
                let req = req.query(&[("apptype", &self.apptype)]);
                let req = req.query(&[("cr", &self.cr)]);
                let req = req.query(&[("device", &self.device)]);
                let req = req.query(&[("devid", &self.devid)]);
                let req = req.query(&[("devtype", &self.devtype)]);
                let req = req.query(&[("exp", &self.exp)]);
                let req = req.query(&[("fs", &self.fs)]);
                let req = req.query(&[("lr", &self.lr)]);
                let req = req.query(&[("make", &self.make)]);
                let req = req.query(&[("maxResults", &self.max_results)]);
                let req = req.query(&[("model", &self.model)]);
                let req = req.query(&[("or", &self.or)]);
                let req = req.query(&[("pageToken", &self.page_token)]);
                let req = req.query(&[("product", &self.product)]);
                let req = req.query(&[("sn", &self.sn)]);
                let req = req.query(&[("snapshotToken", &self.snapshot_token)]);
                let req = req.query(&[("tr", &self.tr)]);
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
        pub struct ListplaybacksRequestBuilder<'a, A> {
            pub(crate) reqwest: &'a ::reqwest::Client,
            pub(crate) auth: &'a ::std::sync::Mutex<A>,
            apptype: Option<String>,
            cr: Option<String>,
            device: Option<String>,
            devid: Option<String>,
            devtype: Option<String>,
            exp: Option<String>,
            fs: Option<String>,
            lr: Option<String>,
            make: Option<String>,
            max_results: Option<u64>,
            model: Option<String>,
            product: Option<String>,
            sn: Option<String>,
            tr: Option<Vec<String>>,
            wts: Option<u64>,
            alt: Option<crate::params::Alt>,
            fields: Option<String>,
            key: Option<String>,
            oauth_token: Option<String>,
            pretty_print: Option<bool>,
            quota_user: Option<String>,
            user_ip: Option<String>,
        }
        impl<'a, A: yup_oauth2::GetToken> ListplaybacksRequestBuilder<'a, A> {
            #[doc = ""]
            pub fn apptype(mut self, value: impl Into<String>) -> Self {
                self.apptype = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn cr(mut self, value: impl Into<String>) -> Self {
                self.cr = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn device(mut self, value: impl Into<String>) -> Self {
                self.device = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn devid(mut self, value: impl Into<String>) -> Self {
                self.devid = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn devtype(mut self, value: impl Into<String>) -> Self {
                self.devtype = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn exp(mut self, value: impl Into<String>) -> Self {
                self.exp = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn fs(mut self, value: impl Into<String>) -> Self {
                self.fs = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn lr(mut self, value: impl Into<String>) -> Self {
                self.lr = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn make(mut self, value: impl Into<String>) -> Self {
                self.make = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn max_results(mut self, value: u64) -> Self {
                self.max_results = Some(value);
                self
            }
            #[doc = ""]
            pub fn model(mut self, value: impl Into<String>) -> Self {
                self.model = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn product(mut self, value: impl Into<String>) -> Self {
                self.product = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn sn(mut self, value: impl Into<String>) -> Self {
                self.sn = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn tr(mut self, value: impl Into<Vec<String>>) -> Self {
                self.tr = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn wts(mut self, value: u64) -> Self {
                self.wts = Some(value);
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
                let mut output = "https://www.googleapis.com/android_video/v1/".to_owned();
                output.push_str("userlibrary/listplaybacks");
                output
            }
            fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
                let req = self.reqwest.request(::reqwest::Method::GET, path);
                let req = req.query(&[("apptype", &self.apptype)]);
                let req = req.query(&[("cr", &self.cr)]);
                let req = req.query(&[("device", &self.device)]);
                let req = req.query(&[("devid", &self.devid)]);
                let req = req.query(&[("devtype", &self.devtype)]);
                let req = req.query(&[("exp", &self.exp)]);
                let req = req.query(&[("fs", &self.fs)]);
                let req = req.query(&[("lr", &self.lr)]);
                let req = req.query(&[("make", &self.make)]);
                let req = req.query(&[("maxResults", &self.max_results)]);
                let req = req.query(&[("model", &self.model)]);
                let req = req.query(&[("product", &self.product)]);
                let req = req.query(&[("sn", &self.sn)]);
                let req = req.query(&[("tr", &self.tr)]);
                let req = req.query(&[("wts", &self.wts)]);
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
        pub struct PlaynextRequestBuilder<'a, A> {
            pub(crate) reqwest: &'a ::reqwest::Client,
            pub(crate) auth: &'a ::std::sync::Mutex<A>,
            aid: Option<String>,
            apptype: Option<String>,
            device: Option<String>,
            devid: Option<String>,
            devtype: Option<String>,
            exp: Option<String>,
            make: Option<String>,
            max: Option<i32>,
            model: Option<String>,
            playlist: Option<String>,
            product: Option<String>,
            rev: Option<bool>,
            sn: Option<String>,
            alt: Option<crate::params::Alt>,
            fields: Option<String>,
            key: Option<String>,
            oauth_token: Option<String>,
            pretty_print: Option<bool>,
            quota_user: Option<String>,
            user_ip: Option<String>,
        }
        impl<'a, A: yup_oauth2::GetToken> PlaynextRequestBuilder<'a, A> {
            #[doc = ""]
            pub fn aid(mut self, value: impl Into<String>) -> Self {
                self.aid = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn apptype(mut self, value: impl Into<String>) -> Self {
                self.apptype = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn device(mut self, value: impl Into<String>) -> Self {
                self.device = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn devid(mut self, value: impl Into<String>) -> Self {
                self.devid = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn devtype(mut self, value: impl Into<String>) -> Self {
                self.devtype = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn exp(mut self, value: impl Into<String>) -> Self {
                self.exp = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn make(mut self, value: impl Into<String>) -> Self {
                self.make = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn max(mut self, value: i32) -> Self {
                self.max = Some(value);
                self
            }
            #[doc = ""]
            pub fn model(mut self, value: impl Into<String>) -> Self {
                self.model = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn playlist(mut self, value: impl Into<String>) -> Self {
                self.playlist = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn product(mut self, value: impl Into<String>) -> Self {
                self.product = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn rev(mut self, value: bool) -> Self {
                self.rev = Some(value);
                self
            }
            #[doc = ""]
            pub fn sn(mut self, value: impl Into<String>) -> Self {
                self.sn = Some(value.into());
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
                let mut output = "https://www.googleapis.com/android_video/v1/".to_owned();
                output.push_str("userlibrary/playnext");
                output
            }
            fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
                let req = self.reqwest.request(::reqwest::Method::GET, path);
                let req = req.query(&[("aid", &self.aid)]);
                let req = req.query(&[("apptype", &self.apptype)]);
                let req = req.query(&[("device", &self.device)]);
                let req = req.query(&[("devid", &self.devid)]);
                let req = req.query(&[("devtype", &self.devtype)]);
                let req = req.query(&[("exp", &self.exp)]);
                let req = req.query(&[("make", &self.make)]);
                let req = req.query(&[("max", &self.max)]);
                let req = req.query(&[("model", &self.model)]);
                let req = req.query(&[("playlist", &self.playlist)]);
                let req = req.query(&[("product", &self.product)]);
                let req = req.query(&[("rev", &self.rev)]);
                let req = req.query(&[("sn", &self.sn)]);
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
        pub struct ShareRequestBuilder<'a, A> {
            pub(crate) reqwest: &'a ::reqwest::Client,
            pub(crate) auth: &'a ::std::sync::Mutex<A>,
            aid: Option<Vec<String>>,
            apptype: Option<String>,
            device: Option<String>,
            devid: Option<String>,
            devtype: Option<String>,
            exp: Option<String>,
            make: Option<String>,
            model: Option<String>,
            product: Option<String>,
            r#type: Option<String>,
            sn: Option<String>,
            alt: Option<crate::params::Alt>,
            fields: Option<String>,
            key: Option<String>,
            oauth_token: Option<String>,
            pretty_print: Option<bool>,
            quota_user: Option<String>,
            user_ip: Option<String>,
        }
        impl<'a, A: yup_oauth2::GetToken> ShareRequestBuilder<'a, A> {
            #[doc = ""]
            pub fn aid(mut self, value: impl Into<Vec<String>>) -> Self {
                self.aid = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn apptype(mut self, value: impl Into<String>) -> Self {
                self.apptype = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn device(mut self, value: impl Into<String>) -> Self {
                self.device = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn devid(mut self, value: impl Into<String>) -> Self {
                self.devid = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn devtype(mut self, value: impl Into<String>) -> Self {
                self.devtype = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn exp(mut self, value: impl Into<String>) -> Self {
                self.exp = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn make(mut self, value: impl Into<String>) -> Self {
                self.make = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn model(mut self, value: impl Into<String>) -> Self {
                self.model = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn product(mut self, value: impl Into<String>) -> Self {
                self.product = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn r#type(mut self, value: impl Into<String>) -> Self {
                self.r#type = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn sn(mut self, value: impl Into<String>) -> Self {
                self.sn = Some(value.into());
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
                let mut output = "https://www.googleapis.com/android_video/v1/".to_owned();
                output.push_str("userlibrary/share");
                output
            }
            fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
                let req = self.reqwest.request(::reqwest::Method::POST, path);
                let req = req.query(&[("aid", &self.aid)]);
                let req = req.query(&[("apptype", &self.apptype)]);
                let req = req.query(&[("device", &self.device)]);
                let req = req.query(&[("devid", &self.devid)]);
                let req = req.query(&[("devtype", &self.devtype)]);
                let req = req.query(&[("exp", &self.exp)]);
                let req = req.query(&[("make", &self.make)]);
                let req = req.query(&[("model", &self.model)]);
                let req = req.query(&[("product", &self.product)]);
                let req = req.query(&[("type", &self.r#type)]);
                let req = req.query(&[("sn", &self.sn)]);
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
        pub struct UnhideRequestBuilder<'a, A> {
            pub(crate) reqwest: &'a ::reqwest::Client,
            pub(crate) auth: &'a ::std::sync::Mutex<A>,
            aid: Option<Vec<String>>,
            apptype: Option<String>,
            device: Option<String>,
            devid: Option<String>,
            devtype: Option<String>,
            exp: Option<String>,
            make: Option<String>,
            model: Option<String>,
            product: Option<String>,
            sn: Option<String>,
            alt: Option<crate::params::Alt>,
            fields: Option<String>,
            key: Option<String>,
            oauth_token: Option<String>,
            pretty_print: Option<bool>,
            quota_user: Option<String>,
            user_ip: Option<String>,
        }
        impl<'a, A: yup_oauth2::GetToken> UnhideRequestBuilder<'a, A> {
            #[doc = ""]
            pub fn aid(mut self, value: impl Into<Vec<String>>) -> Self {
                self.aid = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn apptype(mut self, value: impl Into<String>) -> Self {
                self.apptype = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn device(mut self, value: impl Into<String>) -> Self {
                self.device = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn devid(mut self, value: impl Into<String>) -> Self {
                self.devid = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn devtype(mut self, value: impl Into<String>) -> Self {
                self.devtype = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn exp(mut self, value: impl Into<String>) -> Self {
                self.exp = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn make(mut self, value: impl Into<String>) -> Self {
                self.make = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn model(mut self, value: impl Into<String>) -> Self {
                self.model = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn product(mut self, value: impl Into<String>) -> Self {
                self.product = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn sn(mut self, value: impl Into<String>) -> Self {
                self.sn = Some(value.into());
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
                let mut output = "https://www.googleapis.com/android_video/v1/".to_owned();
                output.push_str("userlibrary/unhide");
                output
            }
            fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
                let req = self.reqwest.request(::reqwest::Method::POST, path);
                let req = req.query(&[("aid", &self.aid)]);
                let req = req.query(&[("apptype", &self.apptype)]);
                let req = req.query(&[("device", &self.device)]);
                let req = req.query(&[("devid", &self.devid)]);
                let req = req.query(&[("devtype", &self.devtype)]);
                let req = req.query(&[("exp", &self.exp)]);
                let req = req.query(&[("make", &self.make)]);
                let req = req.query(&[("model", &self.model)]);
                let req = req.query(&[("product", &self.product)]);
                let req = req.query(&[("sn", &self.sn)]);
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
        pub struct UnshareRequestBuilder<'a, A> {
            pub(crate) reqwest: &'a ::reqwest::Client,
            pub(crate) auth: &'a ::std::sync::Mutex<A>,
            aid: Option<Vec<String>>,
            apptype: Option<String>,
            device: Option<String>,
            devid: Option<String>,
            devtype: Option<String>,
            exp: Option<String>,
            make: Option<String>,
            model: Option<String>,
            product: Option<String>,
            r#type: Option<String>,
            sn: Option<String>,
            alt: Option<crate::params::Alt>,
            fields: Option<String>,
            key: Option<String>,
            oauth_token: Option<String>,
            pretty_print: Option<bool>,
            quota_user: Option<String>,
            user_ip: Option<String>,
        }
        impl<'a, A: yup_oauth2::GetToken> UnshareRequestBuilder<'a, A> {
            #[doc = ""]
            pub fn aid(mut self, value: impl Into<Vec<String>>) -> Self {
                self.aid = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn apptype(mut self, value: impl Into<String>) -> Self {
                self.apptype = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn device(mut self, value: impl Into<String>) -> Self {
                self.device = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn devid(mut self, value: impl Into<String>) -> Self {
                self.devid = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn devtype(mut self, value: impl Into<String>) -> Self {
                self.devtype = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn exp(mut self, value: impl Into<String>) -> Self {
                self.exp = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn make(mut self, value: impl Into<String>) -> Self {
                self.make = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn model(mut self, value: impl Into<String>) -> Self {
                self.model = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn product(mut self, value: impl Into<String>) -> Self {
                self.product = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn r#type(mut self, value: impl Into<String>) -> Self {
                self.r#type = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn sn(mut self, value: impl Into<String>) -> Self {
                self.sn = Some(value.into());
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
                let mut output = "https://www.googleapis.com/android_video/v1/".to_owned();
                output.push_str("userlibrary/unshare");
                output
            }
            fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
                let req = self.reqwest.request(::reqwest::Method::POST, path);
                let req = req.query(&[("aid", &self.aid)]);
                let req = req.query(&[("apptype", &self.apptype)]);
                let req = req.query(&[("device", &self.device)]);
                let req = req.query(&[("devid", &self.devid)]);
                let req = req.query(&[("devtype", &self.devtype)]);
                let req = req.query(&[("exp", &self.exp)]);
                let req = req.query(&[("make", &self.make)]);
                let req = req.query(&[("model", &self.model)]);
                let req = req.query(&[("product", &self.product)]);
                let req = req.query(&[("type", &self.r#type)]);
                let req = req.query(&[("sn", &self.sn)]);
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
        pub struct WatchnextRequestBuilder<'a, A> {
            pub(crate) reqwest: &'a ::reqwest::Client,
            pub(crate) auth: &'a ::std::sync::Mutex<A>,
            apptype: Option<String>,
            cr: Option<String>,
            device: Option<String>,
            devid: Option<String>,
            devtype: Option<String>,
            exp: Option<String>,
            fs: Option<String>,
            lr: Option<String>,
            make: Option<String>,
            max_per_type: Option<String>,
            max_results: Option<u32>,
            model: Option<String>,
            mrf: Option<String>,
            page_token: Option<String>,
            product: Option<String>,
            sn: Option<String>,
            snapshot_token: Option<String>,
            tr: Option<Vec<String>>,
            tvrf: Option<String>,
            ver: Option<String>,
            alt: Option<crate::params::Alt>,
            fields: Option<String>,
            key: Option<String>,
            oauth_token: Option<String>,
            pretty_print: Option<bool>,
            quota_user: Option<String>,
            user_ip: Option<String>,
        }
        impl<'a, A: yup_oauth2::GetToken> WatchnextRequestBuilder<'a, A> {
            #[doc = ""]
            pub fn apptype(mut self, value: impl Into<String>) -> Self {
                self.apptype = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn cr(mut self, value: impl Into<String>) -> Self {
                self.cr = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn device(mut self, value: impl Into<String>) -> Self {
                self.device = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn devid(mut self, value: impl Into<String>) -> Self {
                self.devid = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn devtype(mut self, value: impl Into<String>) -> Self {
                self.devtype = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn exp(mut self, value: impl Into<String>) -> Self {
                self.exp = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn fs(mut self, value: impl Into<String>) -> Self {
                self.fs = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn lr(mut self, value: impl Into<String>) -> Self {
                self.lr = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn make(mut self, value: impl Into<String>) -> Self {
                self.make = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn max_per_type(mut self, value: impl Into<String>) -> Self {
                self.max_per_type = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn max_results(mut self, value: u32) -> Self {
                self.max_results = Some(value);
                self
            }
            #[doc = ""]
            pub fn model(mut self, value: impl Into<String>) -> Self {
                self.model = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn mrf(mut self, value: impl Into<String>) -> Self {
                self.mrf = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn page_token(mut self, value: impl Into<String>) -> Self {
                self.page_token = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn product(mut self, value: impl Into<String>) -> Self {
                self.product = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn sn(mut self, value: impl Into<String>) -> Self {
                self.sn = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn snapshot_token(mut self, value: impl Into<String>) -> Self {
                self.snapshot_token = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn tr(mut self, value: impl Into<Vec<String>>) -> Self {
                self.tr = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn tvrf(mut self, value: impl Into<String>) -> Self {
                self.tvrf = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn ver(mut self, value: impl Into<String>) -> Self {
                self.ver = Some(value.into());
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
                let mut output = "https://www.googleapis.com/android_video/v1/".to_owned();
                output.push_str("userlibrary/watchnext");
                output
            }
            fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
                let req = self.reqwest.request(::reqwest::Method::GET, path);
                let req = req.query(&[("apptype", &self.apptype)]);
                let req = req.query(&[("cr", &self.cr)]);
                let req = req.query(&[("device", &self.device)]);
                let req = req.query(&[("devid", &self.devid)]);
                let req = req.query(&[("devtype", &self.devtype)]);
                let req = req.query(&[("exp", &self.exp)]);
                let req = req.query(&[("fs", &self.fs)]);
                let req = req.query(&[("lr", &self.lr)]);
                let req = req.query(&[("make", &self.make)]);
                let req = req.query(&[("maxPerType", &self.max_per_type)]);
                let req = req.query(&[("maxResults", &self.max_results)]);
                let req = req.query(&[("model", &self.model)]);
                let req = req.query(&[("mrf", &self.mrf)]);
                let req = req.query(&[("pageToken", &self.page_token)]);
                let req = req.query(&[("product", &self.product)]);
                let req = req.query(&[("sn", &self.sn)]);
                let req = req.query(&[("snapshotToken", &self.snapshot_token)]);
                let req = req.query(&[("tr", &self.tr)]);
                let req = req.query(&[("tvrf", &self.tvrf)]);
                let req = req.query(&[("ver", &self.ver)]);
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
    pub mod usersetting {
        pub mod params {}
        pub struct UsersettingActions<'a, A> {
            pub(crate) reqwest: &'a reqwest::Client,
            pub(crate) auth: &'a std::sync::Mutex<A>,
        }
        impl<'a, A: yup_oauth2::GetToken> UsersettingActions<'a, A> {
            #[doc = ""]
            pub fn batchupdate(&self) -> BatchupdateRequestBuilder<A> {
                BatchupdateRequestBuilder {
                    reqwest: &self.reqwest,
                    auth: &self.auth,
                    alt: None,
                    fields: None,
                    key: None,
                    oauth_token: None,
                    pretty_print: None,
                    quota_user: None,
                    user_ip: None,
                }
            }
            #[doc = ""]
            pub fn get(&self) -> GetRequestBuilder<A> {
                GetRequestBuilder {
                    reqwest: &self.reqwest,
                    auth: &self.auth,
                    alt: None,
                    fields: None,
                    key: None,
                    oauth_token: None,
                    pretty_print: None,
                    quota_user: None,
                    user_ip: None,
                    apptype: None,
                    cr: None,
                    device: None,
                    devid: None,
                    devtype: None,
                    exp: None,
                    fs: None,
                    lr: None,
                    make: None,
                    model: None,
                    product: None,
                    resid: None,
                    sn: None,
                    utoken: None,
                }
            }
            #[doc = ""]
            pub fn update(&self) -> UpdateRequestBuilder<A> {
                UpdateRequestBuilder {
                    reqwest: &self.reqwest,
                    auth: &self.auth,
                    alt: None,
                    fields: None,
                    key: None,
                    oauth_token: None,
                    pretty_print: None,
                    quota_user: None,
                    user_ip: None,
                    apptype: None,
                    cr: None,
                    device: None,
                    devid: None,
                    devtype: None,
                    exp: None,
                    fs: None,
                    lr: None,
                    make: None,
                    model: None,
                    product: None,
                    sn: None,
                    utoken: None,
                }
            }
        }
        #[derive(Debug, Clone)]
        pub struct BatchupdateRequestBuilder<'a, A> {
            pub(crate) reqwest: &'a ::reqwest::Client,
            pub(crate) auth: &'a ::std::sync::Mutex<A>,
            alt: Option<crate::params::Alt>,
            fields: Option<String>,
            key: Option<String>,
            oauth_token: Option<String>,
            pretty_print: Option<bool>,
            quota_user: Option<String>,
            user_ip: Option<String>,
        }
        impl<'a, A: yup_oauth2::GetToken> BatchupdateRequestBuilder<'a, A> {
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
                let mut output = "https://www.googleapis.com/android_video/v1/".to_owned();
                output.push_str("usersetting/batchupdate");
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
        pub struct GetRequestBuilder<'a, A> {
            pub(crate) reqwest: &'a ::reqwest::Client,
            pub(crate) auth: &'a ::std::sync::Mutex<A>,
            apptype: Option<String>,
            cr: Option<String>,
            device: Option<String>,
            devid: Option<String>,
            devtype: Option<String>,
            exp: Option<String>,
            fs: Option<String>,
            lr: Option<String>,
            make: Option<String>,
            model: Option<String>,
            product: Option<String>,
            resid: Option<String>,
            sn: Option<String>,
            utoken: Option<String>,
            alt: Option<crate::params::Alt>,
            fields: Option<String>,
            key: Option<String>,
            oauth_token: Option<String>,
            pretty_print: Option<bool>,
            quota_user: Option<String>,
            user_ip: Option<String>,
        }
        impl<'a, A: yup_oauth2::GetToken> GetRequestBuilder<'a, A> {
            #[doc = ""]
            pub fn apptype(mut self, value: impl Into<String>) -> Self {
                self.apptype = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn cr(mut self, value: impl Into<String>) -> Self {
                self.cr = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn device(mut self, value: impl Into<String>) -> Self {
                self.device = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn devid(mut self, value: impl Into<String>) -> Self {
                self.devid = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn devtype(mut self, value: impl Into<String>) -> Self {
                self.devtype = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn exp(mut self, value: impl Into<String>) -> Self {
                self.exp = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn fs(mut self, value: impl Into<String>) -> Self {
                self.fs = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn lr(mut self, value: impl Into<String>) -> Self {
                self.lr = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn make(mut self, value: impl Into<String>) -> Self {
                self.make = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn model(mut self, value: impl Into<String>) -> Self {
                self.model = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn product(mut self, value: impl Into<String>) -> Self {
                self.product = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn resid(mut self, value: impl Into<String>) -> Self {
                self.resid = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn sn(mut self, value: impl Into<String>) -> Self {
                self.sn = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn utoken(mut self, value: impl Into<String>) -> Self {
                self.utoken = Some(value.into());
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
                let mut output = "https://www.googleapis.com/android_video/v1/".to_owned();
                output.push_str("usersetting");
                output
            }
            fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
                let req = self.reqwest.request(::reqwest::Method::GET, path);
                let req = req.query(&[("apptype", &self.apptype)]);
                let req = req.query(&[("cr", &self.cr)]);
                let req = req.query(&[("device", &self.device)]);
                let req = req.query(&[("devid", &self.devid)]);
                let req = req.query(&[("devtype", &self.devtype)]);
                let req = req.query(&[("exp", &self.exp)]);
                let req = req.query(&[("fs", &self.fs)]);
                let req = req.query(&[("lr", &self.lr)]);
                let req = req.query(&[("make", &self.make)]);
                let req = req.query(&[("model", &self.model)]);
                let req = req.query(&[("product", &self.product)]);
                let req = req.query(&[("resid", &self.resid)]);
                let req = req.query(&[("sn", &self.sn)]);
                let req = req.query(&[("utoken", &self.utoken)]);
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
            apptype: Option<String>,
            cr: Option<String>,
            device: Option<String>,
            devid: Option<String>,
            devtype: Option<String>,
            exp: Option<String>,
            fs: Option<String>,
            lr: Option<String>,
            make: Option<String>,
            model: Option<String>,
            product: Option<String>,
            sn: Option<String>,
            utoken: Option<String>,
            alt: Option<crate::params::Alt>,
            fields: Option<String>,
            key: Option<String>,
            oauth_token: Option<String>,
            pretty_print: Option<bool>,
            quota_user: Option<String>,
            user_ip: Option<String>,
        }
        impl<'a, A: yup_oauth2::GetToken> UpdateRequestBuilder<'a, A> {
            #[doc = ""]
            pub fn apptype(mut self, value: impl Into<String>) -> Self {
                self.apptype = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn cr(mut self, value: impl Into<String>) -> Self {
                self.cr = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn device(mut self, value: impl Into<String>) -> Self {
                self.device = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn devid(mut self, value: impl Into<String>) -> Self {
                self.devid = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn devtype(mut self, value: impl Into<String>) -> Self {
                self.devtype = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn exp(mut self, value: impl Into<String>) -> Self {
                self.exp = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn fs(mut self, value: impl Into<String>) -> Self {
                self.fs = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn lr(mut self, value: impl Into<String>) -> Self {
                self.lr = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn make(mut self, value: impl Into<String>) -> Self {
                self.make = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn model(mut self, value: impl Into<String>) -> Self {
                self.model = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn product(mut self, value: impl Into<String>) -> Self {
                self.product = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn sn(mut self, value: impl Into<String>) -> Self {
                self.sn = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn utoken(mut self, value: impl Into<String>) -> Self {
                self.utoken = Some(value.into());
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
                let mut output = "https://www.googleapis.com/android_video/v1/".to_owned();
                output.push_str("usersetting");
                output
            }
            fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
                let req = self.reqwest.request(::reqwest::Method::PUT, path);
                let req = req.query(&[("apptype", &self.apptype)]);
                let req = req.query(&[("cr", &self.cr)]);
                let req = req.query(&[("device", &self.device)]);
                let req = req.query(&[("devid", &self.devid)]);
                let req = req.query(&[("devtype", &self.devtype)]);
                let req = req.query(&[("exp", &self.exp)]);
                let req = req.query(&[("fs", &self.fs)]);
                let req = req.query(&[("lr", &self.lr)]);
                let req = req.query(&[("make", &self.make)]);
                let req = req.query(&[("model", &self.model)]);
                let req = req.query(&[("product", &self.product)]);
                let req = req.query(&[("sn", &self.sn)]);
                let req = req.query(&[("utoken", &self.utoken)]);
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
    pub mod video {
        pub mod params {}
        pub struct VideoActions<'a, A> {
            pub(crate) reqwest: &'a reqwest::Client,
            pub(crate) auth: &'a std::sync::Mutex<A>,
        }
        impl<'a, A: yup_oauth2::GetToken> VideoActions<'a, A> {
            #[doc = ""]
            pub fn get(&self, video_id: impl Into<String>) -> GetRequestBuilder<A> {
                GetRequestBuilder {
                    reqwest: &self.reqwest,
                    auth: &self.auth,
                    alt: None,
                    fields: None,
                    key: None,
                    oauth_token: None,
                    pretty_print: None,
                    quota_user: None,
                    user_ip: None,
                    video_id: video_id.into(),
                    apptype: None,
                    device: None,
                    devid: None,
                    devtype: None,
                    exp: None,
                    make: None,
                    model: None,
                    product: None,
                    sn: None,
                    snapshot_token: None,
                }
            }
            #[doc = ""]
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
                    apptype: None,
                    device: None,
                    devid: None,
                    devtype: None,
                    exp: None,
                    id: None,
                    make: None,
                    model: None,
                    product: None,
                    sn: None,
                    snapshot_token: None,
                }
            }
            #[doc = ""]
            pub fn update(&self, video_id: impl Into<String>) -> UpdateRequestBuilder<A> {
                UpdateRequestBuilder {
                    reqwest: &self.reqwest,
                    auth: &self.auth,
                    alt: None,
                    fields: None,
                    key: None,
                    oauth_token: None,
                    pretty_print: None,
                    quota_user: None,
                    user_ip: None,
                    video_id: video_id.into(),
                    apptype: None,
                    cr: None,
                    device: None,
                    devid: None,
                    devtype: None,
                    exp: None,
                    gp: None,
                    lr: None,
                    make: None,
                    model: None,
                    product: None,
                    sn: None,
                    snapshot_token: None,
                }
            }
        }
        #[derive(Debug, Clone)]
        pub struct GetRequestBuilder<'a, A> {
            pub(crate) reqwest: &'a ::reqwest::Client,
            pub(crate) auth: &'a ::std::sync::Mutex<A>,
            video_id: String,
            apptype: Option<String>,
            device: Option<String>,
            devid: Option<String>,
            devtype: Option<String>,
            exp: Option<String>,
            make: Option<String>,
            model: Option<String>,
            product: Option<String>,
            sn: Option<String>,
            snapshot_token: Option<String>,
            alt: Option<crate::params::Alt>,
            fields: Option<String>,
            key: Option<String>,
            oauth_token: Option<String>,
            pretty_print: Option<bool>,
            quota_user: Option<String>,
            user_ip: Option<String>,
        }
        impl<'a, A: yup_oauth2::GetToken> GetRequestBuilder<'a, A> {
            #[doc = ""]
            pub fn apptype(mut self, value: impl Into<String>) -> Self {
                self.apptype = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn device(mut self, value: impl Into<String>) -> Self {
                self.device = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn devid(mut self, value: impl Into<String>) -> Self {
                self.devid = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn devtype(mut self, value: impl Into<String>) -> Self {
                self.devtype = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn exp(mut self, value: impl Into<String>) -> Self {
                self.exp = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn make(mut self, value: impl Into<String>) -> Self {
                self.make = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn model(mut self, value: impl Into<String>) -> Self {
                self.model = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn product(mut self, value: impl Into<String>) -> Self {
                self.product = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn sn(mut self, value: impl Into<String>) -> Self {
                self.sn = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn snapshot_token(mut self, value: impl Into<String>) -> Self {
                self.snapshot_token = Some(value.into());
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
                let mut output = "https://www.googleapis.com/android_video/v1/".to_owned();
                output.push_str("video/");
                {
                    let var_as_str = &self.video_id;
                    output.extend(::percent_encoding::utf8_percent_encode(
                        &var_as_str,
                        crate::SIMPLE,
                    ));
                }
                output
            }
            fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
                let req = self.reqwest.request(::reqwest::Method::GET, path);
                let req = req.query(&[("apptype", &self.apptype)]);
                let req = req.query(&[("device", &self.device)]);
                let req = req.query(&[("devid", &self.devid)]);
                let req = req.query(&[("devtype", &self.devtype)]);
                let req = req.query(&[("exp", &self.exp)]);
                let req = req.query(&[("make", &self.make)]);
                let req = req.query(&[("model", &self.model)]);
                let req = req.query(&[("product", &self.product)]);
                let req = req.query(&[("sn", &self.sn)]);
                let req = req.query(&[("snapshotToken", &self.snapshot_token)]);
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
            apptype: Option<String>,
            device: Option<String>,
            devid: Option<String>,
            devtype: Option<String>,
            exp: Option<String>,
            id: Option<Vec<String>>,
            make: Option<String>,
            model: Option<String>,
            product: Option<String>,
            sn: Option<String>,
            snapshot_token: Option<String>,
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
            pub fn apptype(mut self, value: impl Into<String>) -> Self {
                self.apptype = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn device(mut self, value: impl Into<String>) -> Self {
                self.device = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn devid(mut self, value: impl Into<String>) -> Self {
                self.devid = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn devtype(mut self, value: impl Into<String>) -> Self {
                self.devtype = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn exp(mut self, value: impl Into<String>) -> Self {
                self.exp = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn id(mut self, value: impl Into<Vec<String>>) -> Self {
                self.id = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn make(mut self, value: impl Into<String>) -> Self {
                self.make = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn model(mut self, value: impl Into<String>) -> Self {
                self.model = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn product(mut self, value: impl Into<String>) -> Self {
                self.product = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn sn(mut self, value: impl Into<String>) -> Self {
                self.sn = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn snapshot_token(mut self, value: impl Into<String>) -> Self {
                self.snapshot_token = Some(value.into());
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
                let mut output = "https://www.googleapis.com/android_video/v1/".to_owned();
                output.push_str("video");
                output
            }
            fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
                let req = self.reqwest.request(::reqwest::Method::GET, path);
                let req = req.query(&[("apptype", &self.apptype)]);
                let req = req.query(&[("device", &self.device)]);
                let req = req.query(&[("devid", &self.devid)]);
                let req = req.query(&[("devtype", &self.devtype)]);
                let req = req.query(&[("exp", &self.exp)]);
                let req = req.query(&[("id", &self.id)]);
                let req = req.query(&[("make", &self.make)]);
                let req = req.query(&[("model", &self.model)]);
                let req = req.query(&[("product", &self.product)]);
                let req = req.query(&[("sn", &self.sn)]);
                let req = req.query(&[("snapshotToken", &self.snapshot_token)]);
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
            video_id: String,
            apptype: Option<String>,
            cr: Option<String>,
            device: Option<String>,
            devid: Option<String>,
            devtype: Option<String>,
            exp: Option<String>,
            gp: Option<bool>,
            lr: Option<String>,
            make: Option<String>,
            model: Option<String>,
            product: Option<String>,
            sn: Option<String>,
            snapshot_token: Option<String>,
            alt: Option<crate::params::Alt>,
            fields: Option<String>,
            key: Option<String>,
            oauth_token: Option<String>,
            pretty_print: Option<bool>,
            quota_user: Option<String>,
            user_ip: Option<String>,
        }
        impl<'a, A: yup_oauth2::GetToken> UpdateRequestBuilder<'a, A> {
            #[doc = ""]
            pub fn apptype(mut self, value: impl Into<String>) -> Self {
                self.apptype = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn cr(mut self, value: impl Into<String>) -> Self {
                self.cr = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn device(mut self, value: impl Into<String>) -> Self {
                self.device = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn devid(mut self, value: impl Into<String>) -> Self {
                self.devid = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn devtype(mut self, value: impl Into<String>) -> Self {
                self.devtype = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn exp(mut self, value: impl Into<String>) -> Self {
                self.exp = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn gp(mut self, value: bool) -> Self {
                self.gp = Some(value);
                self
            }
            #[doc = ""]
            pub fn lr(mut self, value: impl Into<String>) -> Self {
                self.lr = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn make(mut self, value: impl Into<String>) -> Self {
                self.make = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn model(mut self, value: impl Into<String>) -> Self {
                self.model = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn product(mut self, value: impl Into<String>) -> Self {
                self.product = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn sn(mut self, value: impl Into<String>) -> Self {
                self.sn = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn snapshot_token(mut self, value: impl Into<String>) -> Self {
                self.snapshot_token = Some(value.into());
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
                let mut output = "https://www.googleapis.com/android_video/v1/".to_owned();
                output.push_str("video/");
                {
                    let var_as_str = &self.video_id;
                    output.extend(::percent_encoding::utf8_percent_encode(
                        &var_as_str,
                        crate::SIMPLE,
                    ));
                }
                output
            }
            fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
                let req = self.reqwest.request(::reqwest::Method::PUT, path);
                let req = req.query(&[("apptype", &self.apptype)]);
                let req = req.query(&[("cr", &self.cr)]);
                let req = req.query(&[("device", &self.device)]);
                let req = req.query(&[("devid", &self.devid)]);
                let req = req.query(&[("devtype", &self.devtype)]);
                let req = req.query(&[("exp", &self.exp)]);
                let req = req.query(&[("gp", &self.gp)]);
                let req = req.query(&[("lr", &self.lr)]);
                let req = req.query(&[("make", &self.make)]);
                let req = req.query(&[("model", &self.model)]);
                let req = req.query(&[("product", &self.product)]);
                let req = req.query(&[("sn", &self.sn)]);
                let req = req.query(&[("snapshotToken", &self.snapshot_token)]);
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
    pub mod wishlistitem {
        pub mod params {}
        pub struct WishlistitemActions<'a, A> {
            pub(crate) reqwest: &'a reqwest::Client,
            pub(crate) auth: &'a std::sync::Mutex<A>,
        }
        impl<'a, A: yup_oauth2::GetToken> WishlistitemActions<'a, A> {
            #[doc = ""]
            pub fn delete(&self) -> DeleteRequestBuilder<A> {
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
                    apptype: None,
                    device: None,
                    devid: None,
                    devtype: None,
                    docid: None,
                    exp: None,
                    listtype: None,
                    make: None,
                    model: None,
                    product: None,
                    sn: None,
                }
            }
            #[doc = ""]
            pub fn insert(&self) -> InsertRequestBuilder<A> {
                InsertRequestBuilder {
                    reqwest: &self.reqwest,
                    auth: &self.auth,
                    alt: None,
                    fields: None,
                    key: None,
                    oauth_token: None,
                    pretty_print: None,
                    quota_user: None,
                    user_ip: None,
                    apptype: None,
                    device: None,
                    devid: None,
                    devtype: None,
                    docid: None,
                    exp: None,
                    listtype: None,
                    make: None,
                    model: None,
                    product: None,
                    sn: None,
                }
            }
            #[doc = ""]
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
                    apptype: None,
                    device: None,
                    devid: None,
                    devtype: None,
                    exp: None,
                    fs: None,
                    listtype: None,
                    make: None,
                    model: None,
                    product: None,
                    sn: None,
                    snapshot_token: None,
                }
            }
        }
        #[derive(Debug, Clone)]
        pub struct DeleteRequestBuilder<'a, A> {
            pub(crate) reqwest: &'a ::reqwest::Client,
            pub(crate) auth: &'a ::std::sync::Mutex<A>,
            apptype: Option<String>,
            device: Option<String>,
            devid: Option<String>,
            devtype: Option<String>,
            docid: Option<String>,
            exp: Option<String>,
            listtype: Option<String>,
            make: Option<String>,
            model: Option<String>,
            product: Option<String>,
            sn: Option<String>,
            alt: Option<crate::params::Alt>,
            fields: Option<String>,
            key: Option<String>,
            oauth_token: Option<String>,
            pretty_print: Option<bool>,
            quota_user: Option<String>,
            user_ip: Option<String>,
        }
        impl<'a, A: yup_oauth2::GetToken> DeleteRequestBuilder<'a, A> {
            #[doc = ""]
            pub fn apptype(mut self, value: impl Into<String>) -> Self {
                self.apptype = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn device(mut self, value: impl Into<String>) -> Self {
                self.device = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn devid(mut self, value: impl Into<String>) -> Self {
                self.devid = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn devtype(mut self, value: impl Into<String>) -> Self {
                self.devtype = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn docid(mut self, value: impl Into<String>) -> Self {
                self.docid = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn exp(mut self, value: impl Into<String>) -> Self {
                self.exp = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn listtype(mut self, value: impl Into<String>) -> Self {
                self.listtype = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn make(mut self, value: impl Into<String>) -> Self {
                self.make = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn model(mut self, value: impl Into<String>) -> Self {
                self.model = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn product(mut self, value: impl Into<String>) -> Self {
                self.product = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn sn(mut self, value: impl Into<String>) -> Self {
                self.sn = Some(value.into());
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
                let mut output = "https://www.googleapis.com/android_video/v1/".to_owned();
                output.push_str("wishlistitem");
                output
            }
            fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
                let req = self.reqwest.request(::reqwest::Method::DELETE, path);
                let req = req.query(&[("apptype", &self.apptype)]);
                let req = req.query(&[("device", &self.device)]);
                let req = req.query(&[("devid", &self.devid)]);
                let req = req.query(&[("devtype", &self.devtype)]);
                let req = req.query(&[("docid", &self.docid)]);
                let req = req.query(&[("exp", &self.exp)]);
                let req = req.query(&[("listtype", &self.listtype)]);
                let req = req.query(&[("make", &self.make)]);
                let req = req.query(&[("model", &self.model)]);
                let req = req.query(&[("product", &self.product)]);
                let req = req.query(&[("sn", &self.sn)]);
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
        pub struct InsertRequestBuilder<'a, A> {
            pub(crate) reqwest: &'a ::reqwest::Client,
            pub(crate) auth: &'a ::std::sync::Mutex<A>,
            apptype: Option<String>,
            device: Option<String>,
            devid: Option<String>,
            devtype: Option<String>,
            docid: Option<String>,
            exp: Option<String>,
            listtype: Option<String>,
            make: Option<String>,
            model: Option<String>,
            product: Option<String>,
            sn: Option<String>,
            alt: Option<crate::params::Alt>,
            fields: Option<String>,
            key: Option<String>,
            oauth_token: Option<String>,
            pretty_print: Option<bool>,
            quota_user: Option<String>,
            user_ip: Option<String>,
        }
        impl<'a, A: yup_oauth2::GetToken> InsertRequestBuilder<'a, A> {
            #[doc = ""]
            pub fn apptype(mut self, value: impl Into<String>) -> Self {
                self.apptype = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn device(mut self, value: impl Into<String>) -> Self {
                self.device = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn devid(mut self, value: impl Into<String>) -> Self {
                self.devid = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn devtype(mut self, value: impl Into<String>) -> Self {
                self.devtype = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn docid(mut self, value: impl Into<String>) -> Self {
                self.docid = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn exp(mut self, value: impl Into<String>) -> Self {
                self.exp = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn listtype(mut self, value: impl Into<String>) -> Self {
                self.listtype = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn make(mut self, value: impl Into<String>) -> Self {
                self.make = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn model(mut self, value: impl Into<String>) -> Self {
                self.model = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn product(mut self, value: impl Into<String>) -> Self {
                self.product = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn sn(mut self, value: impl Into<String>) -> Self {
                self.sn = Some(value.into());
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
                let mut output = "https://www.googleapis.com/android_video/v1/".to_owned();
                output.push_str("wishlistitem");
                output
            }
            fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
                let req = self.reqwest.request(::reqwest::Method::POST, path);
                let req = req.query(&[("apptype", &self.apptype)]);
                let req = req.query(&[("device", &self.device)]);
                let req = req.query(&[("devid", &self.devid)]);
                let req = req.query(&[("devtype", &self.devtype)]);
                let req = req.query(&[("docid", &self.docid)]);
                let req = req.query(&[("exp", &self.exp)]);
                let req = req.query(&[("listtype", &self.listtype)]);
                let req = req.query(&[("make", &self.make)]);
                let req = req.query(&[("model", &self.model)]);
                let req = req.query(&[("product", &self.product)]);
                let req = req.query(&[("sn", &self.sn)]);
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
            apptype: Option<String>,
            device: Option<String>,
            devid: Option<String>,
            devtype: Option<String>,
            exp: Option<String>,
            fs: Option<String>,
            listtype: Option<String>,
            make: Option<String>,
            model: Option<String>,
            product: Option<String>,
            sn: Option<String>,
            snapshot_token: Option<String>,
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
            pub fn apptype(mut self, value: impl Into<String>) -> Self {
                self.apptype = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn device(mut self, value: impl Into<String>) -> Self {
                self.device = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn devid(mut self, value: impl Into<String>) -> Self {
                self.devid = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn devtype(mut self, value: impl Into<String>) -> Self {
                self.devtype = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn exp(mut self, value: impl Into<String>) -> Self {
                self.exp = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn fs(mut self, value: impl Into<String>) -> Self {
                self.fs = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn listtype(mut self, value: impl Into<String>) -> Self {
                self.listtype = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn make(mut self, value: impl Into<String>) -> Self {
                self.make = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn model(mut self, value: impl Into<String>) -> Self {
                self.model = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn product(mut self, value: impl Into<String>) -> Self {
                self.product = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn sn(mut self, value: impl Into<String>) -> Self {
                self.sn = Some(value.into());
                self
            }
            #[doc = ""]
            pub fn snapshot_token(mut self, value: impl Into<String>) -> Self {
                self.snapshot_token = Some(value.into());
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
                let mut output = "https://www.googleapis.com/android_video/v1/".to_owned();
                output.push_str("wishlistitem");
                output
            }
            fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
                let req = self.reqwest.request(::reqwest::Method::GET, path);
                let req = req.query(&[("apptype", &self.apptype)]);
                let req = req.query(&[("device", &self.device)]);
                let req = req.query(&[("devid", &self.devid)]);
                let req = req.query(&[("devtype", &self.devtype)]);
                let req = req.query(&[("exp", &self.exp)]);
                let req = req.query(&[("fs", &self.fs)]);
                let req = req.query(&[("listtype", &self.listtype)]);
                let req = req.query(&[("make", &self.make)]);
                let req = req.query(&[("model", &self.model)]);
                let req = req.query(&[("product", &self.product)]);
                let req = req.query(&[("sn", &self.sn)]);
                let req = req.query(&[("snapshotToken", &self.snapshot_token)]);
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
