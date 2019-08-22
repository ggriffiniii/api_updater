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
    pub struct Acl {
        #[doc = "Description of the access granted, suitable for display."]
        #[serde(rename = "description", default)]
        pub description: ::std::option::Option<String>,
        #[doc = "Whether access is restricted to the domain."]
        #[serde(rename = "domainRestricted", default)]
        pub domain_restricted: ::std::option::Option<bool>,
        #[doc = "The list of access entries."]
        #[serde(rename = "items", default)]
        pub items: ::std::option::Option<Vec<crate::schemas::PlusDomainsAclentryResource>>,
        #[doc = "Identifies this resource as a collection of access controls. Value: \"plus#acl\"."]
        #[serde(rename = "kind", default)]
        pub kind: ::std::option::Option<String>,
    }
    impl ::field_selector::FieldSelector for Acl {
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
    pub struct ActivityActorClientSpecificActorInfoYoutubeActorInfo {
        #[doc = "ID of the YouTube channel owned by the Actor."]
        #[serde(rename = "channelId", default)]
        pub channel_id: ::std::option::Option<String>,
    }
    impl ::field_selector::FieldSelector for ActivityActorClientSpecificActorInfoYoutubeActorInfo {
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
    pub struct ActivityActorClientSpecificActorInfo {
        #[doc = "Actor info specific to YouTube clients."]
        #[serde(rename = "youtubeActorInfo", default)]
        pub youtube_actor_info: ::std::option::Option<
            crate::schemas::ActivityActorClientSpecificActorInfoYoutubeActorInfo,
        >,
    }
    impl ::field_selector::FieldSelector for ActivityActorClientSpecificActorInfo {
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
    pub struct ActivityActorImage {
        #[doc = "The URL of the actor's profile photo. To resize the image and crop it to a square, append the query string ?sz=x, where x is the dimension in pixels of each side."]
        #[serde(rename = "url", default)]
        pub url: ::std::option::Option<String>,
    }
    impl ::field_selector::FieldSelector for ActivityActorImage {
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
    pub struct ActivityActorName {
        #[doc = "The family name (\"last name\") of the actor."]
        #[serde(rename = "familyName", default)]
        pub family_name: ::std::option::Option<String>,
        #[doc = "The given name (\"first name\") of the actor."]
        #[serde(rename = "givenName", default)]
        pub given_name: ::std::option::Option<String>,
    }
    impl ::field_selector::FieldSelector for ActivityActorName {
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
    pub struct ActivityActorVerification {
        #[doc = "Verification for one-time or manual processes."]
        #[serde(rename = "adHocVerified", default)]
        pub ad_hoc_verified: ::std::option::Option<String>,
    }
    impl ::field_selector::FieldSelector for ActivityActorVerification {
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
    pub struct ActivityActor {
        #[doc = "Actor info specific to particular clients."]
        #[serde(rename = "clientSpecificActorInfo", default)]
        pub client_specific_actor_info:
            ::std::option::Option<crate::schemas::ActivityActorClientSpecificActorInfo>,
        #[doc = "The name of the actor, suitable for display."]
        #[serde(rename = "displayName", default)]
        pub display_name: ::std::option::Option<String>,
        #[doc = "The ID of the actor's Person resource."]
        #[serde(rename = "id", default)]
        pub id: ::std::option::Option<String>,
        #[doc = "The image representation of the actor."]
        #[serde(rename = "image", default)]
        pub image: ::std::option::Option<crate::schemas::ActivityActorImage>,
        #[doc = "An object representation of the individual components of name."]
        #[serde(rename = "name", default)]
        pub name: ::std::option::Option<crate::schemas::ActivityActorName>,
        #[doc = "The link to the actor's Google profile."]
        #[serde(rename = "url", default)]
        pub url: ::std::option::Option<String>,
        #[doc = "Verification status of actor."]
        #[serde(rename = "verification", default)]
        pub verification: ::std::option::Option<crate::schemas::ActivityActorVerification>,
    }
    impl ::field_selector::FieldSelector for ActivityActor {
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
    pub struct ActivityObjectActorClientSpecificActorInfoYoutubeActorInfo {
        #[doc = "ID of the YouTube channel owned by the Actor."]
        #[serde(rename = "channelId", default)]
        pub channel_id: ::std::option::Option<String>,
    }
    impl ::field_selector::FieldSelector
        for ActivityObjectActorClientSpecificActorInfoYoutubeActorInfo
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
    pub struct ActivityObjectActorClientSpecificActorInfo {
        #[doc = "Actor info specific to YouTube clients."]
        #[serde(rename = "youtubeActorInfo", default)]
        pub youtube_actor_info: ::std::option::Option<
            crate::schemas::ActivityObjectActorClientSpecificActorInfoYoutubeActorInfo,
        >,
    }
    impl ::field_selector::FieldSelector for ActivityObjectActorClientSpecificActorInfo {
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
    pub struct ActivityObjectActorImage {
        #[doc = "A URL that points to a thumbnail photo of the original actor."]
        #[serde(rename = "url", default)]
        pub url: ::std::option::Option<String>,
    }
    impl ::field_selector::FieldSelector for ActivityObjectActorImage {
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
    pub struct ActivityObjectActorVerification {
        #[doc = "Verification for one-time or manual processes."]
        #[serde(rename = "adHocVerified", default)]
        pub ad_hoc_verified: ::std::option::Option<String>,
    }
    impl ::field_selector::FieldSelector for ActivityObjectActorVerification {
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
    pub struct ActivityObjectActor {
        #[doc = "Actor info specific to particular clients."]
        #[serde(rename = "clientSpecificActorInfo", default)]
        pub client_specific_actor_info:
            ::std::option::Option<crate::schemas::ActivityObjectActorClientSpecificActorInfo>,
        #[doc = "The original actor's name, which is suitable for display."]
        #[serde(rename = "displayName", default)]
        pub display_name: ::std::option::Option<String>,
        #[doc = "ID of the original actor."]
        #[serde(rename = "id", default)]
        pub id: ::std::option::Option<String>,
        #[doc = "The image representation of the original actor."]
        #[serde(rename = "image", default)]
        pub image: ::std::option::Option<crate::schemas::ActivityObjectActorImage>,
        #[doc = "A link to the original actor's Google profile."]
        #[serde(rename = "url", default)]
        pub url: ::std::option::Option<String>,
        #[doc = "Verification status of actor."]
        #[serde(rename = "verification", default)]
        pub verification: ::std::option::Option<crate::schemas::ActivityObjectActorVerification>,
    }
    impl ::field_selector::FieldSelector for ActivityObjectActor {
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
    pub struct ActivityObjectAttachmentsItemsEmbed {
        #[doc = "Media type of the link."]
        #[serde(rename = "type", default)]
        pub r#type: ::std::option::Option<String>,
        #[doc = "URL of the link."]
        #[serde(rename = "url", default)]
        pub url: ::std::option::Option<String>,
    }
    impl ::field_selector::FieldSelector for ActivityObjectAttachmentsItemsEmbed {
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
    pub struct ActivityObjectAttachmentsItemsFullImage {
        #[doc = "The height, in pixels, of the linked resource."]
        #[serde(rename = "height", default)]
        pub height: ::std::option::Option<u32>,
        #[doc = "Media type of the link."]
        #[serde(rename = "type", default)]
        pub r#type: ::std::option::Option<String>,
        #[doc = "URL of the image."]
        #[serde(rename = "url", default)]
        pub url: ::std::option::Option<String>,
        #[doc = "The width, in pixels, of the linked resource."]
        #[serde(rename = "width", default)]
        pub width: ::std::option::Option<u32>,
    }
    impl ::field_selector::FieldSelector for ActivityObjectAttachmentsItemsFullImage {
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
    pub struct ActivityObjectAttachmentsItemsImage {
        #[doc = "The height, in pixels, of the linked resource."]
        #[serde(rename = "height", default)]
        pub height: ::std::option::Option<u32>,
        #[doc = "Media type of the link."]
        #[serde(rename = "type", default)]
        pub r#type: ::std::option::Option<String>,
        #[doc = "Image URL."]
        #[serde(rename = "url", default)]
        pub url: ::std::option::Option<String>,
        #[doc = "The width, in pixels, of the linked resource."]
        #[serde(rename = "width", default)]
        pub width: ::std::option::Option<u32>,
    }
    impl ::field_selector::FieldSelector for ActivityObjectAttachmentsItemsImage {
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
    pub struct ActivityObjectAttachmentsItemsPreviewThumbnailsItems {
        #[doc = "URL of the thumbnail image."]
        #[serde(rename = "url", default)]
        pub url: ::std::option::Option<String>,
    }
    impl ::field_selector::FieldSelector for ActivityObjectAttachmentsItemsPreviewThumbnailsItems {
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
    pub struct ActivityObjectAttachmentsItemsThumbnailsItemsImage {
        #[doc = "The height, in pixels, of the linked resource."]
        #[serde(rename = "height", default)]
        pub height: ::std::option::Option<u32>,
        #[doc = "Media type of the link."]
        #[serde(rename = "type", default)]
        pub r#type: ::std::option::Option<String>,
        #[doc = "Image url."]
        #[serde(rename = "url", default)]
        pub url: ::std::option::Option<String>,
        #[doc = "The width, in pixels, of the linked resource."]
        #[serde(rename = "width", default)]
        pub width: ::std::option::Option<u32>,
    }
    impl ::field_selector::FieldSelector for ActivityObjectAttachmentsItemsThumbnailsItemsImage {
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
    pub struct ActivityObjectAttachmentsItemsThumbnailsItems {
        #[doc = "Potential name of the thumbnail."]
        #[serde(rename = "description", default)]
        pub description: ::std::option::Option<String>,
        #[doc = "Image resource."]
        #[serde(rename = "image", default)]
        pub image: ::std::option::Option<
            crate::schemas::ActivityObjectAttachmentsItemsThumbnailsItemsImage,
        >,
        #[doc = "URL of the webpage containing the image."]
        #[serde(rename = "url", default)]
        pub url: ::std::option::Option<String>,
    }
    impl ::field_selector::FieldSelector for ActivityObjectAttachmentsItemsThumbnailsItems {
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
    pub struct ActivityObjectAttachmentsItems {
        #[doc = "If the attachment is an article, this property contains a snippet of text from the article. It can also include descriptions for other types."]
        #[serde(rename = "content", default)]
        pub content: ::std::option::Option<String>,
        #[doc = "The title of the attachment, such as a photo caption or an article title."]
        #[serde(rename = "displayName", default)]
        pub display_name: ::std::option::Option<String>,
        #[doc = "If the attachment is a video, the embeddable link."]
        #[serde(rename = "embed", default)]
        pub embed: ::std::option::Option<crate::schemas::ActivityObjectAttachmentsItemsEmbed>,
        #[doc = "The full image URL for photo attachments."]
        #[serde(rename = "fullImage", default)]
        pub full_image:
            ::std::option::Option<crate::schemas::ActivityObjectAttachmentsItemsFullImage>,
        #[doc = "The ID of the attachment."]
        #[serde(rename = "id", default)]
        pub id: ::std::option::Option<String>,
        #[doc = "The preview image for photos or videos."]
        #[serde(rename = "image", default)]
        pub image: ::std::option::Option<crate::schemas::ActivityObjectAttachmentsItemsImage>,
        #[doc = "The type of media object. Possible values include, but are not limited to, the following values:\n\n* \"photo\" - A photo. \n* \"album\" - A photo album. \n* \"video\" - A video. \n* \"article\" - An article, specified by a link."]
        #[serde(rename = "objectType", default)]
        pub object_type: ::std::option::Option<String>,
        #[doc = "When previewing, these are the optional thumbnails for the post. When posting an article, choose one by setting the attachment.image.url property. If you don't choose one, one will be chosen for you."]
        #[serde(rename = "previewThumbnails", default)]
        pub preview_thumbnails: ::std::option::Option<
            Vec<crate::schemas::ActivityObjectAttachmentsItemsPreviewThumbnailsItems>,
        >,
        #[doc = "If the attachment is an album, this property is a list of potential additional thumbnails from the album."]
        #[serde(rename = "thumbnails", default)]
        pub thumbnails: ::std::option::Option<
            Vec<crate::schemas::ActivityObjectAttachmentsItemsThumbnailsItems>,
        >,
        #[doc = "The link to the attachment, which should be of type text/html."]
        #[serde(rename = "url", default)]
        pub url: ::std::option::Option<String>,
    }
    impl ::field_selector::FieldSelector for ActivityObjectAttachmentsItems {
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
    pub struct ActivityObjectPlusoners {
        #[doc = "The URL for the collection of people who +1'd this activity."]
        #[serde(rename = "selfLink", default)]
        pub self_link: ::std::option::Option<String>,
        #[doc = "Total number of people who +1'd this activity."]
        #[serde(rename = "totalItems", default)]
        pub total_items: ::std::option::Option<u32>,
    }
    impl ::field_selector::FieldSelector for ActivityObjectPlusoners {
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
    pub struct ActivityObjectReplies {
        #[doc = "The URL for the collection of comments in reply to this activity."]
        #[serde(rename = "selfLink", default)]
        pub self_link: ::std::option::Option<String>,
        #[doc = "Total number of comments on this activity."]
        #[serde(rename = "totalItems", default)]
        pub total_items: ::std::option::Option<u32>,
    }
    impl ::field_selector::FieldSelector for ActivityObjectReplies {
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
    pub struct ActivityObjectResharers {
        #[doc = "The URL for the collection of resharers."]
        #[serde(rename = "selfLink", default)]
        pub self_link: ::std::option::Option<String>,
        #[doc = "Total number of people who reshared this activity."]
        #[serde(rename = "totalItems", default)]
        pub total_items: ::std::option::Option<u32>,
    }
    impl ::field_selector::FieldSelector for ActivityObjectResharers {
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
    pub struct ActivityObjectStatusForViewer {
        #[doc = "Whether the viewer can comment on the activity."]
        #[serde(rename = "canComment", default)]
        pub can_comment: ::std::option::Option<bool>,
        #[doc = "Whether the viewer can +1 the activity."]
        #[serde(rename = "canPlusone", default)]
        pub can_plusone: ::std::option::Option<bool>,
        #[doc = "Whether the viewer can edit or delete the activity."]
        #[serde(rename = "canUpdate", default)]
        pub can_update: ::std::option::Option<bool>,
        #[doc = "Whether the viewer has +1'd the activity."]
        #[serde(rename = "isPlusOned", default)]
        pub is_plus_oned: ::std::option::Option<bool>,
        #[doc = "Whether reshares are disabled for the activity."]
        #[serde(rename = "resharingDisabled", default)]
        pub resharing_disabled: ::std::option::Option<bool>,
    }
    impl ::field_selector::FieldSelector for ActivityObjectStatusForViewer {
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
    pub struct ActivityObject {
        #[doc = "If this activity's object is itself another activity, such as when a person reshares an activity, this property specifies the original activity's actor."]
        #[serde(rename = "actor", default)]
        pub actor: ::std::option::Option<crate::schemas::ActivityObjectActor>,
        #[doc = "The media objects attached to this activity."]
        #[serde(rename = "attachments", default)]
        pub attachments: ::std::option::Option<Vec<crate::schemas::ActivityObjectAttachmentsItems>>,
        #[doc = "The HTML-formatted content, which is suitable for display."]
        #[serde(rename = "content", default)]
        pub content: ::std::option::Option<String>,
        #[doc = "The ID of the object. When resharing an activity, this is the ID of the activity that is being reshared."]
        #[serde(rename = "id", default)]
        pub id: ::std::option::Option<String>,
        #[doc = "The type of the object. Possible values include, but are not limited to, the following values:\n\n* \"note\" - Textual content. \n* \"activity\" - A Google+ activity."]
        #[serde(rename = "objectType", default)]
        pub object_type: ::std::option::Option<String>,
        #[doc = "The content (text) as provided by the author, which is stored without any HTML formatting. When creating or updating an activity, this value must be supplied as plain text in the request."]
        #[serde(rename = "originalContent", default)]
        pub original_content: ::std::option::Option<String>,
        #[doc = "People who +1'd this activity."]
        #[serde(rename = "plusoners", default)]
        pub plusoners: ::std::option::Option<crate::schemas::ActivityObjectPlusoners>,
        #[doc = "Comments in reply to this activity."]
        #[serde(rename = "replies", default)]
        pub replies: ::std::option::Option<crate::schemas::ActivityObjectReplies>,
        #[doc = "People who reshared this activity."]
        #[serde(rename = "resharers", default)]
        pub resharers: ::std::option::Option<crate::schemas::ActivityObjectResharers>,
        #[doc = "Status of the activity as seen by the viewer."]
        #[serde(rename = "statusForViewer", default)]
        pub status_for_viewer: ::std::option::Option<crate::schemas::ActivityObjectStatusForViewer>,
        #[doc = "The URL that points to the linked resource."]
        #[serde(rename = "url", default)]
        pub url: ::std::option::Option<String>,
    }
    impl ::field_selector::FieldSelector for ActivityObject {
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
    pub struct ActivityProvider {
        #[doc = "Name of the service provider."]
        #[serde(rename = "title", default)]
        pub title: ::std::option::Option<String>,
    }
    impl ::field_selector::FieldSelector for ActivityProvider {
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
    pub struct Activity {
        #[doc = "Identifies who has access to see this activity."]
        #[serde(rename = "access", default)]
        pub access: ::std::option::Option<crate::schemas::Acl>,
        #[doc = "The person who performed this activity."]
        #[serde(rename = "actor", default)]
        pub actor: ::std::option::Option<crate::schemas::ActivityActor>,
        #[doc = "Street address where this activity occurred."]
        #[serde(rename = "address", default)]
        pub address: ::std::option::Option<String>,
        #[doc = "Additional content added by the person who shared this activity, applicable only when resharing an activity."]
        #[serde(rename = "annotation", default)]
        pub annotation: ::std::option::Option<String>,
        #[doc = "If this activity is a crosspost from another system, this property specifies the ID of the original activity."]
        #[serde(rename = "crosspostSource", default)]
        pub crosspost_source: ::std::option::Option<String>,
        #[doc = "ETag of this response for caching purposes."]
        #[serde(rename = "etag", default)]
        pub etag: ::std::option::Option<String>,
        #[doc = "Latitude and longitude where this activity occurred. Format is latitude followed by longitude, space separated."]
        #[serde(rename = "geocode", default)]
        pub geocode: ::std::option::Option<String>,
        #[doc = "The ID of this activity."]
        #[serde(rename = "id", default)]
        pub id: ::std::option::Option<String>,
        #[doc = "Identifies this resource as an activity. Value: \"plus#activity\"."]
        #[serde(rename = "kind", default)]
        pub kind: ::std::option::Option<String>,
        #[doc = "The location where this activity occurred."]
        #[serde(rename = "location", default)]
        pub location: ::std::option::Option<crate::schemas::Place>,
        #[doc = "The object of this activity."]
        #[serde(rename = "object", default)]
        pub object: ::std::option::Option<crate::schemas::ActivityObject>,
        #[doc = "ID of the place where this activity occurred."]
        #[serde(rename = "placeId", default)]
        pub place_id: ::std::option::Option<String>,
        #[doc = "Name of the place where this activity occurred."]
        #[serde(rename = "placeName", default)]
        pub place_name: ::std::option::Option<String>,
        #[doc = "The service provider that initially published this activity."]
        #[serde(rename = "provider", default)]
        pub provider: ::std::option::Option<crate::schemas::ActivityProvider>,
        #[doc = "The time at which this activity was initially published. Formatted as an RFC 3339 timestamp."]
        #[serde(rename = "published", default)]
        pub published: ::std::option::Option<::chrono::DateTime<chrono::offset::Utc>>,
        #[doc = "Radius, in meters, of the region where this activity occurred, centered at the latitude and longitude identified in geocode."]
        #[serde(rename = "radius", default)]
        pub radius: ::std::option::Option<String>,
        #[doc = "Title of this activity."]
        #[serde(rename = "title", default)]
        pub title: ::std::option::Option<String>,
        #[doc = "The time at which this activity was last updated. Formatted as an RFC 3339 timestamp."]
        #[serde(rename = "updated", default)]
        pub updated: ::std::option::Option<::chrono::DateTime<chrono::offset::Utc>>,
        #[doc = "The link to this activity."]
        #[serde(rename = "url", default)]
        pub url: ::std::option::Option<String>,
        #[doc = "This activity's verb, which indicates the action that was performed. Possible values include, but are not limited to, the following values:\n\n* \"post\" - Publish content to the stream. \n* \"share\" - Reshare an activity."]
        #[serde(rename = "verb", default)]
        pub verb: ::std::option::Option<String>,
    }
    impl ::field_selector::FieldSelector for Activity {
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
    pub struct ActivityFeed {
        #[doc = "ETag of this response for caching purposes."]
        #[serde(rename = "etag", default)]
        pub etag: ::std::option::Option<String>,
        #[doc = "The ID of this collection of activities. Deprecated."]
        #[serde(rename = "id", default)]
        pub id: ::std::option::Option<String>,
        #[doc = "The activities in this page of results."]
        #[serde(rename = "items", default)]
        pub items: ::std::option::Option<Vec<crate::schemas::Activity>>,
        #[doc = "Identifies this resource as a collection of activities. Value: \"plus#activityFeed\"."]
        #[serde(rename = "kind", default)]
        pub kind: ::std::option::Option<String>,
        #[doc = "Link to the next page of activities."]
        #[serde(rename = "nextLink", default)]
        pub next_link: ::std::option::Option<String>,
        #[doc = "The continuation token, which is used to page through large result sets. Provide this value in a subsequent request to return the next page of results."]
        #[serde(rename = "nextPageToken", default)]
        pub next_page_token: ::std::option::Option<String>,
        #[doc = "Link to this activity resource."]
        #[serde(rename = "selfLink", default)]
        pub self_link: ::std::option::Option<String>,
        #[doc = "The title of this collection of activities, which is a truncated portion of the content."]
        #[serde(rename = "title", default)]
        pub title: ::std::option::Option<String>,
        #[doc = "The time at which this collection of activities was last updated. Formatted as an RFC 3339 timestamp."]
        #[serde(rename = "updated", default)]
        pub updated: ::std::option::Option<::chrono::DateTime<chrono::offset::Utc>>,
    }
    impl ::field_selector::FieldSelector for ActivityFeed {
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
    pub struct Audience {
        #[doc = "ETag of this response for caching purposes."]
        #[serde(rename = "etag", default)]
        pub etag: ::std::option::Option<String>,
        #[doc = "The access control list entry."]
        #[serde(rename = "item", default)]
        pub item: ::std::option::Option<crate::schemas::PlusDomainsAclentryResource>,
        #[doc = "Identifies this resource as an audience. Value: \"plus#audience\"."]
        #[serde(rename = "kind", default)]
        pub kind: ::std::option::Option<String>,
        #[doc = "The number of people in this circle. This only applies if entity_type is CIRCLE."]
        #[serde(rename = "memberCount", default)]
        pub member_count: ::std::option::Option<u32>,
        #[doc = "The circle members' visibility as chosen by the owner of the circle. This only applies for items with \"item.type\" equals \"circle\". Possible values are:\n\n* \"public\" - Members are visible to the public. \n* \"limited\" - Members are visible to a limited audience. \n* \"private\" - Members are visible to the owner only."]
        #[serde(rename = "visibility", default)]
        pub visibility: ::std::option::Option<String>,
    }
    impl ::field_selector::FieldSelector for Audience {
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
    pub struct AudiencesFeed {
        #[doc = "ETag of this response for caching purposes."]
        #[serde(rename = "etag", default)]
        pub etag: ::std::option::Option<String>,
        #[doc = "The audiences in this result."]
        #[serde(rename = "items", default)]
        pub items: ::std::option::Option<Vec<crate::schemas::Audience>>,
        #[doc = "Identifies this resource as a collection of audiences. Value: \"plus#audienceFeed\"."]
        #[serde(rename = "kind", default)]
        pub kind: ::std::option::Option<String>,
        #[doc = "The continuation token, which is used to page through large result sets. Provide this value in a subsequent request to return the next page of results."]
        #[serde(rename = "nextPageToken", default)]
        pub next_page_token: ::std::option::Option<String>,
        #[doc = "The total number of ACL entries. The number of entries in this response may be smaller due to paging."]
        #[serde(rename = "totalItems", default)]
        pub total_items: ::std::option::Option<i32>,
    }
    impl ::field_selector::FieldSelector for AudiencesFeed {
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
    pub struct CirclePeople {
        #[doc = "The total number of people in this circle."]
        #[serde(rename = "totalItems", default)]
        pub total_items: ::std::option::Option<u32>,
    }
    impl ::field_selector::FieldSelector for CirclePeople {
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
    pub struct Circle {
        #[doc = "The description of this circle."]
        #[serde(rename = "description", default)]
        pub description: ::std::option::Option<String>,
        #[doc = "The circle name."]
        #[serde(rename = "displayName", default)]
        pub display_name: ::std::option::Option<String>,
        #[doc = "ETag of this response for caching purposes."]
        #[serde(rename = "etag", default)]
        pub etag: ::std::option::Option<String>,
        #[doc = "The ID of the circle."]
        #[serde(rename = "id", default)]
        pub id: ::std::option::Option<String>,
        #[doc = "Identifies this resource as a circle. Value: \"plus#circle\"."]
        #[serde(rename = "kind", default)]
        pub kind: ::std::option::Option<String>,
        #[doc = "The people in this circle."]
        #[serde(rename = "people", default)]
        pub people: ::std::option::Option<crate::schemas::CirclePeople>,
        #[doc = "Link to this circle resource"]
        #[serde(rename = "selfLink", default)]
        pub self_link: ::std::option::Option<String>,
    }
    impl ::field_selector::FieldSelector for Circle {
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
    pub struct CircleFeed {
        #[doc = "ETag of this response for caching purposes."]
        #[serde(rename = "etag", default)]
        pub etag: ::std::option::Option<String>,
        #[doc = "The circles in this page of results."]
        #[serde(rename = "items", default)]
        pub items: ::std::option::Option<Vec<crate::schemas::Circle>>,
        #[doc = "Identifies this resource as a collection of circles. Value: \"plus#circleFeed\"."]
        #[serde(rename = "kind", default)]
        pub kind: ::std::option::Option<String>,
        #[doc = "Link to the next page of circles."]
        #[serde(rename = "nextLink", default)]
        pub next_link: ::std::option::Option<String>,
        #[doc = "The continuation token, which is used to page through large result sets. Provide this value in a subsequent request to return the next page of results."]
        #[serde(rename = "nextPageToken", default)]
        pub next_page_token: ::std::option::Option<String>,
        #[doc = "Link to this page of circles."]
        #[serde(rename = "selfLink", default)]
        pub self_link: ::std::option::Option<String>,
        #[doc = "The title of this list of resources."]
        #[serde(rename = "title", default)]
        pub title: ::std::option::Option<String>,
        #[doc = "The total number of circles. The number of circles in this response may be smaller due to paging."]
        #[serde(rename = "totalItems", default)]
        pub total_items: ::std::option::Option<i32>,
    }
    impl ::field_selector::FieldSelector for CircleFeed {
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
    pub struct CommentActorClientSpecificActorInfoYoutubeActorInfo {
        #[doc = "ID of the YouTube channel owned by the Actor."]
        #[serde(rename = "channelId", default)]
        pub channel_id: ::std::option::Option<String>,
    }
    impl ::field_selector::FieldSelector for CommentActorClientSpecificActorInfoYoutubeActorInfo {
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
    pub struct CommentActorClientSpecificActorInfo {
        #[doc = "Actor info specific to YouTube clients."]
        #[serde(rename = "youtubeActorInfo", default)]
        pub youtube_actor_info: ::std::option::Option<
            crate::schemas::CommentActorClientSpecificActorInfoYoutubeActorInfo,
        >,
    }
    impl ::field_selector::FieldSelector for CommentActorClientSpecificActorInfo {
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
    pub struct CommentActorImage {
        #[doc = "The URL of the actor's profile photo. To resize the image and crop it to a square, append the query string ?sz=x, where x is the dimension in pixels of each side."]
        #[serde(rename = "url", default)]
        pub url: ::std::option::Option<String>,
    }
    impl ::field_selector::FieldSelector for CommentActorImage {
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
    pub struct CommentActorVerification {
        #[doc = "Verification for one-time or manual processes."]
        #[serde(rename = "adHocVerified", default)]
        pub ad_hoc_verified: ::std::option::Option<String>,
    }
    impl ::field_selector::FieldSelector for CommentActorVerification {
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
    pub struct CommentActor {
        #[doc = "Actor info specific to particular clients."]
        #[serde(rename = "clientSpecificActorInfo", default)]
        pub client_specific_actor_info:
            ::std::option::Option<crate::schemas::CommentActorClientSpecificActorInfo>,
        #[doc = "The name of this actor, suitable for display."]
        #[serde(rename = "displayName", default)]
        pub display_name: ::std::option::Option<String>,
        #[doc = "The ID of the actor."]
        #[serde(rename = "id", default)]
        pub id: ::std::option::Option<String>,
        #[doc = "The image representation of this actor."]
        #[serde(rename = "image", default)]
        pub image: ::std::option::Option<crate::schemas::CommentActorImage>,
        #[doc = "A link to the Person resource for this actor."]
        #[serde(rename = "url", default)]
        pub url: ::std::option::Option<String>,
        #[doc = "Verification status of actor."]
        #[serde(rename = "verification", default)]
        pub verification: ::std::option::Option<crate::schemas::CommentActorVerification>,
    }
    impl ::field_selector::FieldSelector for CommentActor {
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
    pub struct CommentInReplyToItems {
        #[doc = "The ID of the activity."]
        #[serde(rename = "id", default)]
        pub id: ::std::option::Option<String>,
        #[doc = "The URL of the activity."]
        #[serde(rename = "url", default)]
        pub url: ::std::option::Option<String>,
    }
    impl ::field_selector::FieldSelector for CommentInReplyToItems {
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
    pub struct CommentObject {
        #[doc = "The HTML-formatted content, suitable for display."]
        #[serde(rename = "content", default)]
        pub content: ::std::option::Option<String>,
        #[doc = "The object type of this comment. Possible values are:\n\n* \"comment\" - A comment in reply to an activity."]
        #[serde(rename = "objectType", default)]
        pub object_type: ::std::option::Option<String>,
        #[doc = "The content (text) as provided by the author, stored without any HTML formatting. When creating or updating a comment, this value must be supplied as plain text in the request."]
        #[serde(rename = "originalContent", default)]
        pub original_content: ::std::option::Option<String>,
    }
    impl ::field_selector::FieldSelector for CommentObject {
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
    pub struct CommentPlusoners {
        #[doc = "Total number of people who +1'd this comment."]
        #[serde(rename = "totalItems", default)]
        pub total_items: ::std::option::Option<u32>,
    }
    impl ::field_selector::FieldSelector for CommentPlusoners {
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
    pub struct Comment {
        #[doc = "The person who posted this comment."]
        #[serde(rename = "actor", default)]
        pub actor: ::std::option::Option<crate::schemas::CommentActor>,
        #[doc = "ETag of this response for caching purposes."]
        #[serde(rename = "etag", default)]
        pub etag: ::std::option::Option<String>,
        #[doc = "The ID of this comment."]
        #[serde(rename = "id", default)]
        pub id: ::std::option::Option<String>,
        #[doc = "The activity this comment replied to."]
        #[serde(rename = "inReplyTo", default)]
        pub in_reply_to: ::std::option::Option<Vec<crate::schemas::CommentInReplyToItems>>,
        #[doc = "Identifies this resource as a comment. Value: \"plus#comment\"."]
        #[serde(rename = "kind", default)]
        pub kind: ::std::option::Option<String>,
        #[doc = "The object of this comment."]
        #[serde(rename = "object", default)]
        pub object: ::std::option::Option<crate::schemas::CommentObject>,
        #[doc = "People who +1'd this comment."]
        #[serde(rename = "plusoners", default)]
        pub plusoners: ::std::option::Option<crate::schemas::CommentPlusoners>,
        #[doc = "The time at which this comment was initially published. Formatted as an RFC 3339 timestamp."]
        #[serde(rename = "published", default)]
        pub published: ::std::option::Option<::chrono::DateTime<chrono::offset::Utc>>,
        #[doc = "Link to this comment resource."]
        #[serde(rename = "selfLink", default)]
        pub self_link: ::std::option::Option<String>,
        #[doc = "The time at which this comment was last updated. Formatted as an RFC 3339 timestamp."]
        #[serde(rename = "updated", default)]
        pub updated: ::std::option::Option<::chrono::DateTime<chrono::offset::Utc>>,
        #[doc = "This comment's verb, indicating what action was performed. Possible values are:\n\n* \"post\" - Publish content to the stream."]
        #[serde(rename = "verb", default)]
        pub verb: ::std::option::Option<String>,
    }
    impl ::field_selector::FieldSelector for Comment {
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
    pub struct CommentFeed {
        #[doc = "ETag of this response for caching purposes."]
        #[serde(rename = "etag", default)]
        pub etag: ::std::option::Option<String>,
        #[doc = "The ID of this collection of comments."]
        #[serde(rename = "id", default)]
        pub id: ::std::option::Option<String>,
        #[doc = "The comments in this page of results."]
        #[serde(rename = "items", default)]
        pub items: ::std::option::Option<Vec<crate::schemas::Comment>>,
        #[doc = "Identifies this resource as a collection of comments. Value: \"plus#commentFeed\"."]
        #[serde(rename = "kind", default)]
        pub kind: ::std::option::Option<String>,
        #[doc = "Link to the next page of activities."]
        #[serde(rename = "nextLink", default)]
        pub next_link: ::std::option::Option<String>,
        #[doc = "The continuation token, which is used to page through large result sets. Provide this value in a subsequent request to return the next page of results."]
        #[serde(rename = "nextPageToken", default)]
        pub next_page_token: ::std::option::Option<String>,
        #[doc = "The title of this collection of comments."]
        #[serde(rename = "title", default)]
        pub title: ::std::option::Option<String>,
        #[doc = "The time at which this collection of comments was last updated. Formatted as an RFC 3339 timestamp."]
        #[serde(rename = "updated", default)]
        pub updated: ::std::option::Option<::chrono::DateTime<chrono::offset::Utc>>,
    }
    impl ::field_selector::FieldSelector for CommentFeed {
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
    pub struct MediaAuthorImage {
        #[doc = "The URL of the author's profile photo. To resize the image and crop it to a square, append the query string ?sz=x, where x is the dimension in pixels of each side."]
        #[serde(rename = "url", default)]
        pub url: ::std::option::Option<String>,
    }
    impl ::field_selector::FieldSelector for MediaAuthorImage {
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
    pub struct MediaAuthor {
        #[doc = "The author's name."]
        #[serde(rename = "displayName", default)]
        pub display_name: ::std::option::Option<String>,
        #[doc = "ID of the author."]
        #[serde(rename = "id", default)]
        pub id: ::std::option::Option<String>,
        #[doc = "The author's Google profile image."]
        #[serde(rename = "image", default)]
        pub image: ::std::option::Option<crate::schemas::MediaAuthorImage>,
        #[doc = "A link to the author's Google profile."]
        #[serde(rename = "url", default)]
        pub url: ::std::option::Option<String>,
    }
    impl ::field_selector::FieldSelector for MediaAuthor {
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
    pub struct MediaExif {
        #[doc = "The time the media was captured. Formatted as an RFC 3339 timestamp."]
        #[serde(rename = "time", default)]
        pub time: ::std::option::Option<::chrono::DateTime<chrono::offset::Utc>>,
    }
    impl ::field_selector::FieldSelector for MediaExif {
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
    pub struct Media {
        #[doc = "The person who uploaded this media."]
        #[serde(rename = "author", default)]
        pub author: ::std::option::Option<crate::schemas::MediaAuthor>,
        #[doc = "The display name for this media."]
        #[serde(rename = "displayName", default)]
        pub display_name: ::std::option::Option<String>,
        #[doc = "ETag of this response for caching purposes."]
        #[serde(rename = "etag", default)]
        pub etag: ::std::option::Option<String>,
        #[doc = "Exif information of the media item."]
        #[serde(rename = "exif", default)]
        pub exif: ::std::option::Option<crate::schemas::MediaExif>,
        #[doc = "The height in pixels of the original image."]
        #[serde(rename = "height", default)]
        pub height: ::std::option::Option<u32>,
        #[doc = "ID of this media, which is generated by the API."]
        #[serde(rename = "id", default)]
        pub id: ::std::option::Option<String>,
        #[doc = "The type of resource."]
        #[serde(rename = "kind", default)]
        pub kind: ::std::option::Option<String>,
        #[doc = "The time at which this media was originally created in UTC. Formatted as an RFC 3339 timestamp that matches this example: 2010-11-25T14:30:27.655Z"]
        #[serde(rename = "mediaCreatedTime", default)]
        pub media_created_time: ::std::option::Option<::chrono::DateTime<chrono::offset::Utc>>,
        #[doc = "The URL of this photo or video's still image."]
        #[serde(rename = "mediaUrl", default)]
        pub media_url: ::std::option::Option<String>,
        #[doc = "The time at which this media was uploaded. Formatted as an RFC 3339 timestamp."]
        #[serde(rename = "published", default)]
        pub published: ::std::option::Option<::chrono::DateTime<chrono::offset::Utc>>,
        #[doc = "The size in bytes of this video."]
        #[serde(rename = "sizeBytes", default)]
        #[serde(with = "crate::parsed_string")]
        pub size_bytes: ::std::option::Option<i64>,
        #[doc = "The list of video streams for this video. There might be several different streams available for a single video, either Flash or MPEG, of various sizes"]
        #[serde(rename = "streams", default)]
        pub streams: ::std::option::Option<Vec<crate::schemas::Videostream>>,
        #[doc = "A description, or caption, for this media."]
        #[serde(rename = "summary", default)]
        pub summary: ::std::option::Option<String>,
        #[doc = "The time at which this media was last updated. This includes changes to media metadata. Formatted as an RFC 3339 timestamp."]
        #[serde(rename = "updated", default)]
        pub updated: ::std::option::Option<::chrono::DateTime<chrono::offset::Utc>>,
        #[doc = "The URL for the page that hosts this media."]
        #[serde(rename = "url", default)]
        pub url: ::std::option::Option<String>,
        #[doc = "The duration in milliseconds of this video."]
        #[serde(rename = "videoDuration", default)]
        #[serde(with = "crate::parsed_string")]
        pub video_duration: ::std::option::Option<i64>,
        #[doc = "The encoding status of this video. Possible values are:\n\n* \"UPLOADING\" - Not all the video bytes have been received. \n* \"PENDING\" - Video not yet processed. \n* \"FAILED\" - Video processing failed. \n* \"READY\" - A single video stream is playable. \n* \"FINAL\" - All video streams are playable."]
        #[serde(rename = "videoStatus", default)]
        pub video_status: ::std::option::Option<String>,
        #[doc = "The width in pixels of the original image."]
        #[serde(rename = "width", default)]
        pub width: ::std::option::Option<u32>,
    }
    impl ::field_selector::FieldSelector for Media {
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
    pub struct PeopleFeed {
        #[doc = "ETag of this response for caching purposes."]
        #[serde(rename = "etag", default)]
        pub etag: ::std::option::Option<String>,
        #[doc = "The people in this page of results. Each item includes the id, displayName, image, and url for the person. To retrieve additional profile data, see the people.get method."]
        #[serde(rename = "items", default)]
        pub items: ::std::option::Option<Vec<crate::schemas::Person>>,
        #[doc = "Identifies this resource as a collection of people. Value: \"plus#peopleFeed\"."]
        #[serde(rename = "kind", default)]
        pub kind: ::std::option::Option<String>,
        #[doc = "The continuation token, which is used to page through large result sets. Provide this value in a subsequent request to return the next page of results."]
        #[serde(rename = "nextPageToken", default)]
        pub next_page_token: ::std::option::Option<String>,
        #[doc = "Link to this resource."]
        #[serde(rename = "selfLink", default)]
        pub self_link: ::std::option::Option<String>,
        #[doc = "The title of this collection of people."]
        #[serde(rename = "title", default)]
        pub title: ::std::option::Option<String>,
        #[doc = "The total number of people available in this list. The number of people in a response might be smaller due to paging. This might not be set for all collections."]
        #[serde(rename = "totalItems", default)]
        pub total_items: ::std::option::Option<i32>,
    }
    impl ::field_selector::FieldSelector for PeopleFeed {
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
    pub struct PersonCoverCoverInfo {
        #[doc = "The difference between the left position of the cover image and the actual displayed cover image. Only valid for banner layout."]
        #[serde(rename = "leftImageOffset", default)]
        pub left_image_offset: ::std::option::Option<i32>,
        #[doc = "The difference between the top position of the cover image and the actual displayed cover image. Only valid for banner layout."]
        #[serde(rename = "topImageOffset", default)]
        pub top_image_offset: ::std::option::Option<i32>,
    }
    impl ::field_selector::FieldSelector for PersonCoverCoverInfo {
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
    pub struct PersonCoverCoverPhoto {
        #[doc = "The height of the image."]
        #[serde(rename = "height", default)]
        pub height: ::std::option::Option<i32>,
        #[doc = "The URL of the image."]
        #[serde(rename = "url", default)]
        pub url: ::std::option::Option<String>,
        #[doc = "The width of the image."]
        #[serde(rename = "width", default)]
        pub width: ::std::option::Option<i32>,
    }
    impl ::field_selector::FieldSelector for PersonCoverCoverPhoto {
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
    pub struct PersonCover {
        #[doc = "Extra information about the cover photo."]
        #[serde(rename = "coverInfo", default)]
        pub cover_info: ::std::option::Option<crate::schemas::PersonCoverCoverInfo>,
        #[doc = "The person's primary cover image."]
        #[serde(rename = "coverPhoto", default)]
        pub cover_photo: ::std::option::Option<crate::schemas::PersonCoverCoverPhoto>,
        #[doc = "The layout of the cover art. Possible values include, but are not limited to, the following values:\n\n* \"banner\" - One large image banner."]
        #[serde(rename = "layout", default)]
        pub layout: ::std::option::Option<String>,
    }
    impl ::field_selector::FieldSelector for PersonCover {
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
    pub struct PersonEmailsItems {
        #[doc = "The type of address. Possible values include, but are not limited to, the following values:\n\n* \"account\" - Google account email address. \n* \"home\" - Home email address. \n* \"work\" - Work email address. \n* \"other\" - Other."]
        #[serde(rename = "type", default)]
        pub r#type: ::std::option::Option<String>,
        #[doc = "The email address."]
        #[serde(rename = "value", default)]
        pub value: ::std::option::Option<String>,
    }
    impl ::field_selector::FieldSelector for PersonEmailsItems {
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
    pub struct PersonImage {
        #[doc = "Whether the person's profile photo is the default one"]
        #[serde(rename = "isDefault", default)]
        pub is_default: ::std::option::Option<bool>,
        #[doc = "The URL of the person's profile photo. To resize the image and crop it to a square, append the query string ?sz=x, where x is the dimension in pixels of each side."]
        #[serde(rename = "url", default)]
        pub url: ::std::option::Option<String>,
    }
    impl ::field_selector::FieldSelector for PersonImage {
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
    pub struct PersonName {
        #[doc = "The family name (last name) of this person."]
        #[serde(rename = "familyName", default)]
        pub family_name: ::std::option::Option<String>,
        #[doc = "The full name of this person, including middle names, suffixes, etc."]
        #[serde(rename = "formatted", default)]
        pub formatted: ::std::option::Option<String>,
        #[doc = "The given name (first name) of this person."]
        #[serde(rename = "givenName", default)]
        pub given_name: ::std::option::Option<String>,
        #[doc = "The honorific prefixes (such as \"Dr.\" or \"Mrs.\") for this person."]
        #[serde(rename = "honorificPrefix", default)]
        pub honorific_prefix: ::std::option::Option<String>,
        #[doc = "The honorific suffixes (such as \"Jr.\") for this person."]
        #[serde(rename = "honorificSuffix", default)]
        pub honorific_suffix: ::std::option::Option<String>,
        #[doc = "The middle name of this person."]
        #[serde(rename = "middleName", default)]
        pub middle_name: ::std::option::Option<String>,
    }
    impl ::field_selector::FieldSelector for PersonName {
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
    pub struct PersonOrganizationsItems {
        #[doc = "The department within the organization. Deprecated."]
        #[serde(rename = "department", default)]
        pub department: ::std::option::Option<String>,
        #[doc = "A short description of the person's role in this organization. Deprecated."]
        #[serde(rename = "description", default)]
        pub description: ::std::option::Option<String>,
        #[doc = "The date that the person left this organization."]
        #[serde(rename = "endDate", default)]
        pub end_date: ::std::option::Option<String>,
        #[doc = "The location of this organization. Deprecated."]
        #[serde(rename = "location", default)]
        pub location: ::std::option::Option<String>,
        #[doc = "The name of the organization."]
        #[serde(rename = "name", default)]
        pub name: ::std::option::Option<String>,
        #[doc = "If \"true\", indicates this organization is the person's primary one, which is typically interpreted as the current one."]
        #[serde(rename = "primary", default)]
        pub primary: ::std::option::Option<bool>,
        #[doc = "The type of organization. Possible values include, but are not limited to, the following values:\n\n* \"work\" - Work. \n* \"school\" - School."]
        #[serde(rename = "type", default)]
        pub r#type: ::std::option::Option<String>,
        #[doc = "The date that the person joined this organization."]
        #[serde(rename = "startDate", default)]
        pub start_date: ::std::option::Option<String>,
        #[doc = "The person's job title or role within the organization."]
        #[serde(rename = "title", default)]
        pub title: ::std::option::Option<String>,
    }
    impl ::field_selector::FieldSelector for PersonOrganizationsItems {
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
    pub struct PersonPlacesLivedItems {
        #[doc = "If \"true\", this place of residence is this person's primary residence."]
        #[serde(rename = "primary", default)]
        pub primary: ::std::option::Option<bool>,
        #[doc = "A place where this person has lived. For example: \"Seattle, WA\", \"Near Toronto\"."]
        #[serde(rename = "value", default)]
        pub value: ::std::option::Option<String>,
    }
    impl ::field_selector::FieldSelector for PersonPlacesLivedItems {
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
    pub struct PersonUrlsItems {
        #[doc = "The label of the URL."]
        #[serde(rename = "label", default)]
        pub label: ::std::option::Option<String>,
        #[doc = "The type of URL. Possible values include, but are not limited to, the following values:\n\n* \"otherProfile\" - URL for another profile. \n* \"contributor\" - URL to a site for which this person is a contributor. \n* \"website\" - URL for this Google+ Page's primary website. \n* \"other\" - Other URL."]
        #[serde(rename = "type", default)]
        pub r#type: ::std::option::Option<String>,
        #[doc = "The URL value."]
        #[serde(rename = "value", default)]
        pub value: ::std::option::Option<String>,
    }
    impl ::field_selector::FieldSelector for PersonUrlsItems {
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
    pub struct Person {
        #[doc = "A short biography for this person."]
        #[serde(rename = "aboutMe", default)]
        pub about_me: ::std::option::Option<String>,
        #[doc = "The person's date of birth, represented as YYYY-MM-DD."]
        #[serde(rename = "birthday", default)]
        pub birthday: ::std::option::Option<String>,
        #[doc = "The \"bragging rights\" line of this person."]
        #[serde(rename = "braggingRights", default)]
        pub bragging_rights: ::std::option::Option<String>,
        #[doc = "For followers who are visible, the number of people who have added this person or page to a circle."]
        #[serde(rename = "circledByCount", default)]
        pub circled_by_count: ::std::option::Option<i32>,
        #[doc = "The cover photo content."]
        #[serde(rename = "cover", default)]
        pub cover: ::std::option::Option<crate::schemas::PersonCover>,
        #[doc = "(this field is not currently used)"]
        #[serde(rename = "currentLocation", default)]
        pub current_location: ::std::option::Option<String>,
        #[doc = "The name of this person, which is suitable for display."]
        #[serde(rename = "displayName", default)]
        pub display_name: ::std::option::Option<String>,
        #[doc = "The hosted domain name for the user's Google Apps account. For instance, example.com. The plus.profile.emails.read or email scope is needed to get this domain name."]
        #[serde(rename = "domain", default)]
        pub domain: ::std::option::Option<String>,
        #[doc = "A list of email addresses that this person has, including their Google account email address, and the public verified email addresses on their Google+ profile. The plus.profile.emails.read scope is needed to retrieve these email addresses, or the email scope can be used to retrieve just the Google account email address."]
        #[serde(rename = "emails", default)]
        pub emails: ::std::option::Option<Vec<crate::schemas::PersonEmailsItems>>,
        #[doc = "ETag of this response for caching purposes."]
        #[serde(rename = "etag", default)]
        pub etag: ::std::option::Option<String>,
        #[doc = "The person's gender. Possible values include, but are not limited to, the following values:\n\n* \"male\" - Male gender. \n* \"female\" - Female gender. \n* \"other\" - Other."]
        #[serde(rename = "gender", default)]
        pub gender: ::std::option::Option<String>,
        #[doc = "The ID of this person."]
        #[serde(rename = "id", default)]
        pub id: ::std::option::Option<String>,
        #[doc = "The representation of the person's profile photo."]
        #[serde(rename = "image", default)]
        pub image: ::std::option::Option<crate::schemas::PersonImage>,
        #[doc = "Whether this user has signed up for Google+."]
        #[serde(rename = "isPlusUser", default)]
        pub is_plus_user: ::std::option::Option<bool>,
        #[doc = "Identifies this resource as a person. Value: \"plus#person\"."]
        #[serde(rename = "kind", default)]
        pub kind: ::std::option::Option<String>,
        #[doc = "An object representation of the individual components of a person's name."]
        #[serde(rename = "name", default)]
        pub name: ::std::option::Option<crate::schemas::PersonName>,
        #[doc = "The nickname of this person."]
        #[serde(rename = "nickname", default)]
        pub nickname: ::std::option::Option<String>,
        #[doc = "Type of person within Google+. Possible values include, but are not limited to, the following values:\n\n* \"person\" - represents an actual person. \n* \"page\" - represents a page."]
        #[serde(rename = "objectType", default)]
        pub object_type: ::std::option::Option<String>,
        #[doc = "The occupation of this person."]
        #[serde(rename = "occupation", default)]
        pub occupation: ::std::option::Option<String>,
        #[doc = "A list of current or past organizations with which this person is associated."]
        #[serde(rename = "organizations", default)]
        pub organizations: ::std::option::Option<Vec<crate::schemas::PersonOrganizationsItems>>,
        #[doc = "A list of places where this person has lived."]
        #[serde(rename = "placesLived", default)]
        pub places_lived: ::std::option::Option<Vec<crate::schemas::PersonPlacesLivedItems>>,
        #[doc = "If a Google+ Page, the number of people who have +1'd this page."]
        #[serde(rename = "plusOneCount", default)]
        pub plus_one_count: ::std::option::Option<i32>,
        #[doc = "The person's relationship status. Possible values include, but are not limited to, the following values:\n\n* \"single\" - Person is single. \n* \"in_a_relationship\" - Person is in a relationship. \n* \"engaged\" - Person is engaged. \n* \"married\" - Person is married. \n* \"its_complicated\" - The relationship is complicated. \n* \"open_relationship\" - Person is in an open relationship. \n* \"widowed\" - Person is widowed. \n* \"in_domestic_partnership\" - Person is in a domestic partnership. \n* \"in_civil_union\" - Person is in a civil union."]
        #[serde(rename = "relationshipStatus", default)]
        pub relationship_status: ::std::option::Option<String>,
        #[doc = "The person's skills."]
        #[serde(rename = "skills", default)]
        pub skills: ::std::option::Option<String>,
        #[doc = "The brief description (tagline) of this person."]
        #[serde(rename = "tagline", default)]
        pub tagline: ::std::option::Option<String>,
        #[doc = "The URL of this person's profile."]
        #[serde(rename = "url", default)]
        pub url: ::std::option::Option<String>,
        #[doc = "A list of URLs for this person."]
        #[serde(rename = "urls", default)]
        pub urls: ::std::option::Option<Vec<crate::schemas::PersonUrlsItems>>,
        #[doc = "Whether the person or Google+ Page has been verified."]
        #[serde(rename = "verified", default)]
        pub verified: ::std::option::Option<bool>,
    }
    impl ::field_selector::FieldSelector for Person {
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
    pub struct PlaceAddress {
        #[doc = "The formatted address for display."]
        #[serde(rename = "formatted", default)]
        pub formatted: ::std::option::Option<String>,
    }
    impl ::field_selector::FieldSelector for PlaceAddress {
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
    pub struct PlacePosition {
        #[doc = "The latitude of this position."]
        #[serde(rename = "latitude", default)]
        pub latitude: ::std::option::Option<f64>,
        #[doc = "The longitude of this position."]
        #[serde(rename = "longitude", default)]
        pub longitude: ::std::option::Option<f64>,
    }
    impl ::field_selector::FieldSelector for PlacePosition {
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
    pub struct Place {
        #[doc = "The physical address of the place."]
        #[serde(rename = "address", default)]
        pub address: ::std::option::Option<crate::schemas::PlaceAddress>,
        #[doc = "The display name of the place."]
        #[serde(rename = "displayName", default)]
        pub display_name: ::std::option::Option<String>,
        #[doc = "The id of the place."]
        #[serde(rename = "id", default)]
        pub id: ::std::option::Option<String>,
        #[doc = "Identifies this resource as a place. Value: \"plus#place\"."]
        #[serde(rename = "kind", default)]
        pub kind: ::std::option::Option<String>,
        #[doc = "The position of the place."]
        #[serde(rename = "position", default)]
        pub position: ::std::option::Option<crate::schemas::PlacePosition>,
    }
    impl ::field_selector::FieldSelector for Place {
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
    pub struct PlusDomainsAclentryResource {
        #[doc = "A descriptive name for this entry. Suitable for display."]
        #[serde(rename = "displayName", default)]
        pub display_name: ::std::option::Option<String>,
        #[doc = "The ID of the entry. For entries of type \"person\" or \"circle\", this is the ID of the resource. For other types, this property is not set."]
        #[serde(rename = "id", default)]
        pub id: ::std::option::Option<String>,
        #[doc = "The type of entry describing to whom access is granted. Possible values are:\n\n* \"person\" - Access to an individual. \n* \"circle\" - Access to members of a circle. \n* \"myCircles\" - Access to members of all the person's circles. \n* \"extendedCircles\" - Access to members of all the person's circles, plus all of the people in their circles. \n* \"domain\" - Access to members of the person's Google Apps domain. \n* \"public\" - Access to anyone on the web."]
        #[serde(rename = "type", default)]
        pub r#type: ::std::option::Option<String>,
    }
    impl ::field_selector::FieldSelector for PlusDomainsAclentryResource {
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
    pub struct Videostream {
        #[doc = "The height, in pixels, of the video resource."]
        #[serde(rename = "height", default)]
        pub height: ::std::option::Option<i32>,
        #[doc = "MIME type of the video stream."]
        #[serde(rename = "type", default)]
        pub r#type: ::std::option::Option<String>,
        #[doc = "URL of the video stream."]
        #[serde(rename = "url", default)]
        pub url: ::std::option::Option<String>,
        #[doc = "The width, in pixels, of the video resource."]
        #[serde(rename = "width", default)]
        pub width: ::std::option::Option<i32>,
    }
    impl ::field_selector::FieldSelector for Videostream {
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
        #[doc = "Upload/Download media content"]
        Media,
    }
    impl Alt {
        pub fn as_str(self) -> &'static str {
            match self {
                Alt::Json => "json",
                Alt::Media => "media",
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
    #[doc = "Actions that can be performed on the activities resource"]
    pub fn activities(&self) -> crate::resources::activities::ActivitiesActions<A> {
        crate::resources::activities::ActivitiesActions {
            reqwest: &self.reqwest,
            auth: &self.auth,
        }
    }
    #[doc = "Actions that can be performed on the audiences resource"]
    pub fn audiences(&self) -> crate::resources::audiences::AudiencesActions<A> {
        crate::resources::audiences::AudiencesActions {
            reqwest: &self.reqwest,
            auth: &self.auth,
        }
    }
    #[doc = "Actions that can be performed on the circles resource"]
    pub fn circles(&self) -> crate::resources::circles::CirclesActions<A> {
        crate::resources::circles::CirclesActions {
            reqwest: &self.reqwest,
            auth: &self.auth,
        }
    }
    #[doc = "Actions that can be performed on the comments resource"]
    pub fn comments(&self) -> crate::resources::comments::CommentsActions<A> {
        crate::resources::comments::CommentsActions {
            reqwest: &self.reqwest,
            auth: &self.auth,
        }
    }
    #[doc = "Actions that can be performed on the media resource"]
    pub fn media(&self) -> crate::resources::media::MediaActions<A> {
        crate::resources::media::MediaActions {
            reqwest: &self.reqwest,
            auth: &self.auth,
        }
    }
    #[doc = "Actions that can be performed on the people resource"]
    pub fn people(&self) -> crate::resources::people::PeopleActions<A> {
        crate::resources::people::PeopleActions {
            reqwest: &self.reqwest,
            auth: &self.auth,
        }
    }
}
pub mod resources {
    pub mod activities {
        pub mod params {
            #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
            pub enum ListCollection {
                #[doc = "All activities created by the specified user that the authenticated user is authorized to view."]
                User,
            }
            impl ListCollection {
                pub fn as_str(self) -> &'static str {
                    match self {
                        ListCollection::User => "user",
                    }
                }
            }
            impl ::std::fmt::Display for ListCollection {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                    f.write_str(self.as_str())
                }
            }
            impl ::serde::Serialize for ListCollection {
                fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
                where
                    S: ::serde::ser::Serializer,
                {
                    serializer.serialize_str(self.as_str())
                }
            }
            impl<'de> ::serde::Deserialize<'de> for ListCollection {
                fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
                where
                    D: ::serde::de::Deserializer<'de>,
                {
                    let value: &'de str = <&str>::deserialize(deserializer)?;
                    Ok(match value {
                        "user" => ListCollection::User,
                        _ => {
                            return Err(::serde::de::Error::custom(format!(
                                "invalid enum for #name: {}",
                                value
                            )))
                        }
                    })
                }
            }
            impl ::field_selector::FieldSelector for ListCollection {
                fn field_selector_with_ident(ident: &str, selector: &mut String) {
                    match selector.chars().rev().nth(0) {
                        Some(',') | None => {}
                        _ => selector.push_str(","),
                    }
                    selector.push_str(ident);
                }
            }
        }
        pub struct ActivitiesActions<'a, A> {
            pub(crate) reqwest: &'a reqwest::Client,
            pub(crate) auth: &'a std::sync::Mutex<A>,
        }
        impl<'a, A: yup_oauth2::GetToken> ActivitiesActions<'a, A> {
            #[doc = "Shut down. See https://developers.google.com/+/api-shutdown for more details."]
            pub fn get(&self, activity_id: impl Into<String>) -> GetRequestBuilder<A> {
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
                    activity_id: activity_id.into(),
                }
            }
            #[doc = "Shut down. See https://developers.google.com/+/api-shutdown for more details."]
            pub fn list(
                &self,
                user_id: impl Into<String>,
                collection: crate::resources::activities::params::ListCollection,
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
                    collection,
                    max_results: None,
                    page_token: None,
                }
            }
        }
        #[derive(Debug, Clone)]
        pub struct GetRequestBuilder<'a, A> {
            pub(crate) reqwest: &'a ::reqwest::Client,
            pub(crate) auth: &'a ::std::sync::Mutex<A>,
            activity_id: String,
            alt: Option<crate::params::Alt>,
            fields: Option<String>,
            key: Option<String>,
            oauth_token: Option<String>,
            pretty_print: Option<bool>,
            quota_user: Option<String>,
            user_ip: Option<String>,
        }
        impl<'a, A: yup_oauth2::GetToken> GetRequestBuilder<'a, A> {
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
            ) -> Result<crate::schemas::Activity, Box<dyn ::std::error::Error>> {
                self.execute_fields::<_, &str>(None)
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub fn execute_debug(
                self,
            ) -> Result<crate::schemas::Activity, Box<dyn ::std::error::Error>> {
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
                let mut output = "https://www.googleapis.com/plusDomains/v1/".to_owned();
                output.push_str("activities/");
                {
                    let var_as_str = &self.activity_id;
                    output.extend(::percent_encoding::utf8_percent_encode(
                        &var_as_str,
                        crate::SIMPLE,
                    ));
                }
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
                let mut auth = self.auth.lock().unwrap();
                let fut = auth.token(vec!["https://www.googleapis.com/auth/plus.login"]);
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
            user_id: String,
            collection: crate::resources::activities::params::ListCollection,
            max_results: Option<u32>,
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
            #[doc = "The maximum number of activities to include in the response, which is used for paging. For any response, the actual number returned might be less than the specified maxResults."]
            pub fn max_results(mut self, value: u32) -> Self {
                self.max_results = Some(value);
                self
            }
            #[doc = "The continuation token, which is used to page through large result sets. To get the next page of results, set this parameter to the value of \"nextPageToken\" from the previous response."]
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
            pub fn iter_items<T>(mut self) -> crate::iter::PageItemIter<Self, T>
            where
                T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
            {
                let mut fields = concat!("nextPageToken,", "items").to_owned();
                let items_fields = T::field_selector();
                if !items_fields.is_empty() {
                    fields.push_str("(");
                    fields.push_str(&items_fields);
                    fields.push_str(")");
                }
                self.fields = Some(fields);
                crate::iter::PageItemIter::new(self, "items")
            }
            #[doc = r" Return an iterator that iterates over all `#prop_ident`. The"]
            #[doc = r" items yielded by the iterator are `#items_type`. The populated"]
            #[doc = r" fields in `#items_type` will be the default fields populated by"]
            #[doc = r" the server."]
            pub fn iter_items_standard(
                mut self,
            ) -> crate::iter::PageItemIter<Self, crate::schemas::Activity> {
                self.fields = Some(concat!("nextPageToken,", "items").to_owned());
                crate::iter::PageItemIter::new(self, "items")
            }
            #[doc = r" Return an iterator that iterates over all `#prop_ident`. The"]
            #[doc = r" items yielded by the iterator are `#items_type`. The populated"]
            #[doc = r" fields in `#items_type` will be all fields available. This should"]
            #[doc = r" primarily be used during developement and debugging as fetching"]
            #[doc = r" all fields can be expensive both in bandwidth and server"]
            #[doc = r" resources."]
            pub fn iter_items_debug(
                mut self,
            ) -> crate::iter::PageItemIter<Self, crate::schemas::Activity> {
                self.fields = Some(concat!("nextPageToken,", "items", "(*)").to_owned());
                crate::iter::PageItemIter::new(self, "items")
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
            ) -> crate::iter::PageIter<Self, crate::schemas::ActivityFeed> {
                crate::iter::PageIter::new(self)
            }
            pub fn iter_debug(
                mut self,
            ) -> crate::iter::PageIter<Self, crate::schemas::ActivityFeed> {
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
            ) -> Result<crate::schemas::ActivityFeed, Box<dyn ::std::error::Error>> {
                self.execute_fields::<_, &str>(None)
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub fn execute_debug(
                self,
            ) -> Result<crate::schemas::ActivityFeed, Box<dyn ::std::error::Error>> {
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
                let mut output = "https://www.googleapis.com/plusDomains/v1/".to_owned();
                output.push_str("people/");
                {
                    let var_as_str = &self.user_id;
                    output.extend(::percent_encoding::utf8_percent_encode(
                        &var_as_str,
                        crate::SIMPLE,
                    ));
                }
                output.push_str("/activities/");
                {
                    let var_as_string = self.collection.to_string();
                    let var_as_str = &var_as_string;
                    output.extend(::percent_encoding::utf8_percent_encode(
                        &var_as_str,
                        crate::SIMPLE,
                    ));
                }
                output
            }
            fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
                let req = self.reqwest.request(::reqwest::Method::GET, path);
                let req = req.query(&[("maxResults", &self.max_results)]);
                let req = req.query(&[("pageToken", &self.page_token)]);
                let req = req.query(&[("alt", &self.alt)]);
                let req = req.query(&[("fields", &self.fields)]);
                let req = req.query(&[("key", &self.key)]);
                let req = req.query(&[("oauth_token", &self.oauth_token)]);
                let req = req.query(&[("prettyPrint", &self.pretty_print)]);
                let req = req.query(&[("quotaUser", &self.quota_user)]);
                let req = req.query(&[("userIp", &self.user_ip)]);
                let mut auth = self.auth.lock().unwrap();
                let fut = auth.token(vec!["https://www.googleapis.com/auth/plus.login"]);
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
    pub mod audiences {
        pub mod params {}
        pub struct AudiencesActions<'a, A> {
            pub(crate) reqwest: &'a reqwest::Client,
            pub(crate) auth: &'a std::sync::Mutex<A>,
        }
        impl<'a, A: yup_oauth2::GetToken> AudiencesActions<'a, A> {
            #[doc = "Shut down. See https://developers.google.com/+/api-shutdown for more details."]
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
                    max_results: None,
                    page_token: None,
                }
            }
        }
        #[derive(Debug, Clone)]
        pub struct ListRequestBuilder<'a, A> {
            pub(crate) reqwest: &'a ::reqwest::Client,
            pub(crate) auth: &'a ::std::sync::Mutex<A>,
            user_id: String,
            max_results: Option<u32>,
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
            #[doc = "The maximum number of circles to include in the response, which is used for paging. For any response, the actual number returned might be less than the specified maxResults."]
            pub fn max_results(mut self, value: u32) -> Self {
                self.max_results = Some(value);
                self
            }
            #[doc = "The continuation token, which is used to page through large result sets. To get the next page of results, set this parameter to the value of \"nextPageToken\" from the previous response."]
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
            pub fn iter_items<T>(mut self) -> crate::iter::PageItemIter<Self, T>
            where
                T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
            {
                let mut fields = concat!("nextPageToken,", "items").to_owned();
                let items_fields = T::field_selector();
                if !items_fields.is_empty() {
                    fields.push_str("(");
                    fields.push_str(&items_fields);
                    fields.push_str(")");
                }
                self.fields = Some(fields);
                crate::iter::PageItemIter::new(self, "items")
            }
            #[doc = r" Return an iterator that iterates over all `#prop_ident`. The"]
            #[doc = r" items yielded by the iterator are `#items_type`. The populated"]
            #[doc = r" fields in `#items_type` will be the default fields populated by"]
            #[doc = r" the server."]
            pub fn iter_items_standard(
                mut self,
            ) -> crate::iter::PageItemIter<Self, crate::schemas::Audience> {
                self.fields = Some(concat!("nextPageToken,", "items").to_owned());
                crate::iter::PageItemIter::new(self, "items")
            }
            #[doc = r" Return an iterator that iterates over all `#prop_ident`. The"]
            #[doc = r" items yielded by the iterator are `#items_type`. The populated"]
            #[doc = r" fields in `#items_type` will be all fields available. This should"]
            #[doc = r" primarily be used during developement and debugging as fetching"]
            #[doc = r" all fields can be expensive both in bandwidth and server"]
            #[doc = r" resources."]
            pub fn iter_items_debug(
                mut self,
            ) -> crate::iter::PageItemIter<Self, crate::schemas::Audience> {
                self.fields = Some(concat!("nextPageToken,", "items", "(*)").to_owned());
                crate::iter::PageItemIter::new(self, "items")
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
            ) -> crate::iter::PageIter<Self, crate::schemas::AudiencesFeed> {
                crate::iter::PageIter::new(self)
            }
            pub fn iter_debug(
                mut self,
            ) -> crate::iter::PageIter<Self, crate::schemas::AudiencesFeed> {
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
            ) -> Result<crate::schemas::AudiencesFeed, Box<dyn ::std::error::Error>> {
                self.execute_fields::<_, &str>(None)
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub fn execute_debug(
                self,
            ) -> Result<crate::schemas::AudiencesFeed, Box<dyn ::std::error::Error>> {
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
                let mut output = "https://www.googleapis.com/plusDomains/v1/".to_owned();
                output.push_str("people/");
                {
                    let var_as_str = &self.user_id;
                    output.extend(::percent_encoding::utf8_percent_encode(
                        &var_as_str,
                        crate::SIMPLE,
                    ));
                }
                output.push_str("/audiences");
                output
            }
            fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
                let req = self.reqwest.request(::reqwest::Method::GET, path);
                let req = req.query(&[("maxResults", &self.max_results)]);
                let req = req.query(&[("pageToken", &self.page_token)]);
                let req = req.query(&[("alt", &self.alt)]);
                let req = req.query(&[("fields", &self.fields)]);
                let req = req.query(&[("key", &self.key)]);
                let req = req.query(&[("oauth_token", &self.oauth_token)]);
                let req = req.query(&[("prettyPrint", &self.pretty_print)]);
                let req = req.query(&[("quotaUser", &self.quota_user)]);
                let req = req.query(&[("userIp", &self.user_ip)]);
                let mut auth = self.auth.lock().unwrap();
                let fut = auth.token(vec!["https://www.googleapis.com/auth/plus.circles.read"]);
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
    pub mod circles {
        pub mod params {}
        pub struct CirclesActions<'a, A> {
            pub(crate) reqwest: &'a reqwest::Client,
            pub(crate) auth: &'a std::sync::Mutex<A>,
        }
        impl<'a, A: yup_oauth2::GetToken> CirclesActions<'a, A> {
            #[doc = "Shut down. See https://developers.google.com/+/api-shutdown for more details."]
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
                    max_results: None,
                    page_token: None,
                }
            }
        }
        #[derive(Debug, Clone)]
        pub struct ListRequestBuilder<'a, A> {
            pub(crate) reqwest: &'a ::reqwest::Client,
            pub(crate) auth: &'a ::std::sync::Mutex<A>,
            user_id: String,
            max_results: Option<u32>,
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
            #[doc = "The maximum number of circles to include in the response, which is used for paging. For any response, the actual number returned might be less than the specified maxResults."]
            pub fn max_results(mut self, value: u32) -> Self {
                self.max_results = Some(value);
                self
            }
            #[doc = "The continuation token, which is used to page through large result sets. To get the next page of results, set this parameter to the value of \"nextPageToken\" from the previous response."]
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
            pub fn iter_items<T>(mut self) -> crate::iter::PageItemIter<Self, T>
            where
                T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
            {
                let mut fields = concat!("nextPageToken,", "items").to_owned();
                let items_fields = T::field_selector();
                if !items_fields.is_empty() {
                    fields.push_str("(");
                    fields.push_str(&items_fields);
                    fields.push_str(")");
                }
                self.fields = Some(fields);
                crate::iter::PageItemIter::new(self, "items")
            }
            #[doc = r" Return an iterator that iterates over all `#prop_ident`. The"]
            #[doc = r" items yielded by the iterator are `#items_type`. The populated"]
            #[doc = r" fields in `#items_type` will be the default fields populated by"]
            #[doc = r" the server."]
            pub fn iter_items_standard(
                mut self,
            ) -> crate::iter::PageItemIter<Self, crate::schemas::Circle> {
                self.fields = Some(concat!("nextPageToken,", "items").to_owned());
                crate::iter::PageItemIter::new(self, "items")
            }
            #[doc = r" Return an iterator that iterates over all `#prop_ident`. The"]
            #[doc = r" items yielded by the iterator are `#items_type`. The populated"]
            #[doc = r" fields in `#items_type` will be all fields available. This should"]
            #[doc = r" primarily be used during developement and debugging as fetching"]
            #[doc = r" all fields can be expensive both in bandwidth and server"]
            #[doc = r" resources."]
            pub fn iter_items_debug(
                mut self,
            ) -> crate::iter::PageItemIter<Self, crate::schemas::Circle> {
                self.fields = Some(concat!("nextPageToken,", "items", "(*)").to_owned());
                crate::iter::PageItemIter::new(self, "items")
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
            pub fn iter_standard(self) -> crate::iter::PageIter<Self, crate::schemas::CircleFeed> {
                crate::iter::PageIter::new(self)
            }
            pub fn iter_debug(mut self) -> crate::iter::PageIter<Self, crate::schemas::CircleFeed> {
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
            ) -> Result<crate::schemas::CircleFeed, Box<dyn ::std::error::Error>> {
                self.execute_fields::<_, &str>(None)
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub fn execute_debug(
                self,
            ) -> Result<crate::schemas::CircleFeed, Box<dyn ::std::error::Error>> {
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
                let mut output = "https://www.googleapis.com/plusDomains/v1/".to_owned();
                output.push_str("people/");
                {
                    let var_as_str = &self.user_id;
                    output.extend(::percent_encoding::utf8_percent_encode(
                        &var_as_str,
                        crate::SIMPLE,
                    ));
                }
                output.push_str("/circles");
                output
            }
            fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
                let req = self.reqwest.request(::reqwest::Method::GET, path);
                let req = req.query(&[("maxResults", &self.max_results)]);
                let req = req.query(&[("pageToken", &self.page_token)]);
                let req = req.query(&[("alt", &self.alt)]);
                let req = req.query(&[("fields", &self.fields)]);
                let req = req.query(&[("key", &self.key)]);
                let req = req.query(&[("oauth_token", &self.oauth_token)]);
                let req = req.query(&[("prettyPrint", &self.pretty_print)]);
                let req = req.query(&[("quotaUser", &self.quota_user)]);
                let req = req.query(&[("userIp", &self.user_ip)]);
                let mut auth = self.auth.lock().unwrap();
                let fut = auth.token(vec!["https://www.googleapis.com/auth/plus.circles.read"]);
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
    pub mod comments {
        pub mod params {
            #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
            pub enum ListSortOrder {
                #[doc = "Sort oldest comments first."]
                Ascending,
                #[doc = "Sort newest comments first."]
                Descending,
            }
            impl ListSortOrder {
                pub fn as_str(self) -> &'static str {
                    match self {
                        ListSortOrder::Ascending => "ascending",
                        ListSortOrder::Descending => "descending",
                    }
                }
            }
            impl ::std::fmt::Display for ListSortOrder {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                    f.write_str(self.as_str())
                }
            }
            impl ::serde::Serialize for ListSortOrder {
                fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
                where
                    S: ::serde::ser::Serializer,
                {
                    serializer.serialize_str(self.as_str())
                }
            }
            impl<'de> ::serde::Deserialize<'de> for ListSortOrder {
                fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
                where
                    D: ::serde::de::Deserializer<'de>,
                {
                    let value: &'de str = <&str>::deserialize(deserializer)?;
                    Ok(match value {
                        "ascending" => ListSortOrder::Ascending,
                        "descending" => ListSortOrder::Descending,
                        _ => {
                            return Err(::serde::de::Error::custom(format!(
                                "invalid enum for #name: {}",
                                value
                            )))
                        }
                    })
                }
            }
            impl ::field_selector::FieldSelector for ListSortOrder {
                fn field_selector_with_ident(ident: &str, selector: &mut String) {
                    match selector.chars().rev().nth(0) {
                        Some(',') | None => {}
                        _ => selector.push_str(","),
                    }
                    selector.push_str(ident);
                }
            }
        }
        pub struct CommentsActions<'a, A> {
            pub(crate) reqwest: &'a reqwest::Client,
            pub(crate) auth: &'a std::sync::Mutex<A>,
        }
        impl<'a, A: yup_oauth2::GetToken> CommentsActions<'a, A> {
            #[doc = "Shut down. See https://developers.google.com/+/api-shutdown for more details."]
            pub fn get(&self, comment_id: impl Into<String>) -> GetRequestBuilder<A> {
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
                    comment_id: comment_id.into(),
                }
            }
            #[doc = "Shut down. See https://developers.google.com/+/api-shutdown for more details."]
            pub fn list(&self, activity_id: impl Into<String>) -> ListRequestBuilder<A> {
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
                    activity_id: activity_id.into(),
                    max_results: None,
                    page_token: None,
                    sort_order: None,
                }
            }
        }
        #[derive(Debug, Clone)]
        pub struct GetRequestBuilder<'a, A> {
            pub(crate) reqwest: &'a ::reqwest::Client,
            pub(crate) auth: &'a ::std::sync::Mutex<A>,
            comment_id: String,
            alt: Option<crate::params::Alt>,
            fields: Option<String>,
            key: Option<String>,
            oauth_token: Option<String>,
            pretty_print: Option<bool>,
            quota_user: Option<String>,
            user_ip: Option<String>,
        }
        impl<'a, A: yup_oauth2::GetToken> GetRequestBuilder<'a, A> {
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
            ) -> Result<crate::schemas::Comment, Box<dyn ::std::error::Error>> {
                self.execute_fields::<_, &str>(None)
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub fn execute_debug(
                self,
            ) -> Result<crate::schemas::Comment, Box<dyn ::std::error::Error>> {
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
                let mut output = "https://www.googleapis.com/plusDomains/v1/".to_owned();
                output.push_str("comments/");
                {
                    let var_as_str = &self.comment_id;
                    output.extend(::percent_encoding::utf8_percent_encode(
                        &var_as_str,
                        crate::SIMPLE,
                    ));
                }
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
                let mut auth = self.auth.lock().unwrap();
                let fut = auth.token(vec!["https://www.googleapis.com/auth/plus.login"]);
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
            activity_id: String,
            max_results: Option<u32>,
            page_token: Option<String>,
            sort_order: Option<crate::resources::comments::params::ListSortOrder>,
            alt: Option<crate::params::Alt>,
            fields: Option<String>,
            key: Option<String>,
            oauth_token: Option<String>,
            pretty_print: Option<bool>,
            quota_user: Option<String>,
            user_ip: Option<String>,
        }
        impl<'a, A: yup_oauth2::GetToken> ListRequestBuilder<'a, A> {
            #[doc = "The maximum number of comments to include in the response, which is used for paging. For any response, the actual number returned might be less than the specified maxResults."]
            pub fn max_results(mut self, value: u32) -> Self {
                self.max_results = Some(value);
                self
            }
            #[doc = "The continuation token, which is used to page through large result sets. To get the next page of results, set this parameter to the value of \"nextPageToken\" from the previous response."]
            pub fn page_token(mut self, value: impl Into<String>) -> Self {
                self.page_token = Some(value.into());
                self
            }
            #[doc = "The order in which to sort the list of comments."]
            pub fn sort_order(
                mut self,
                value: crate::resources::comments::params::ListSortOrder,
            ) -> Self {
                self.sort_order = Some(value);
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
            pub fn iter_items<T>(mut self) -> crate::iter::PageItemIter<Self, T>
            where
                T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
            {
                let mut fields = concat!("nextPageToken,", "items").to_owned();
                let items_fields = T::field_selector();
                if !items_fields.is_empty() {
                    fields.push_str("(");
                    fields.push_str(&items_fields);
                    fields.push_str(")");
                }
                self.fields = Some(fields);
                crate::iter::PageItemIter::new(self, "items")
            }
            #[doc = r" Return an iterator that iterates over all `#prop_ident`. The"]
            #[doc = r" items yielded by the iterator are `#items_type`. The populated"]
            #[doc = r" fields in `#items_type` will be the default fields populated by"]
            #[doc = r" the server."]
            pub fn iter_items_standard(
                mut self,
            ) -> crate::iter::PageItemIter<Self, crate::schemas::Comment> {
                self.fields = Some(concat!("nextPageToken,", "items").to_owned());
                crate::iter::PageItemIter::new(self, "items")
            }
            #[doc = r" Return an iterator that iterates over all `#prop_ident`. The"]
            #[doc = r" items yielded by the iterator are `#items_type`. The populated"]
            #[doc = r" fields in `#items_type` will be all fields available. This should"]
            #[doc = r" primarily be used during developement and debugging as fetching"]
            #[doc = r" all fields can be expensive both in bandwidth and server"]
            #[doc = r" resources."]
            pub fn iter_items_debug(
                mut self,
            ) -> crate::iter::PageItemIter<Self, crate::schemas::Comment> {
                self.fields = Some(concat!("nextPageToken,", "items", "(*)").to_owned());
                crate::iter::PageItemIter::new(self, "items")
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
            pub fn iter_standard(self) -> crate::iter::PageIter<Self, crate::schemas::CommentFeed> {
                crate::iter::PageIter::new(self)
            }
            pub fn iter_debug(
                mut self,
            ) -> crate::iter::PageIter<Self, crate::schemas::CommentFeed> {
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
            ) -> Result<crate::schemas::CommentFeed, Box<dyn ::std::error::Error>> {
                self.execute_fields::<_, &str>(None)
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub fn execute_debug(
                self,
            ) -> Result<crate::schemas::CommentFeed, Box<dyn ::std::error::Error>> {
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
                let mut output = "https://www.googleapis.com/plusDomains/v1/".to_owned();
                output.push_str("activities/");
                {
                    let var_as_str = &self.activity_id;
                    output.extend(::percent_encoding::utf8_percent_encode(
                        &var_as_str,
                        crate::SIMPLE,
                    ));
                }
                output.push_str("/comments");
                output
            }
            fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
                let req = self.reqwest.request(::reqwest::Method::GET, path);
                let req = req.query(&[("maxResults", &self.max_results)]);
                let req = req.query(&[("pageToken", &self.page_token)]);
                let req = req.query(&[("sortOrder", &self.sort_order)]);
                let req = req.query(&[("alt", &self.alt)]);
                let req = req.query(&[("fields", &self.fields)]);
                let req = req.query(&[("key", &self.key)]);
                let req = req.query(&[("oauth_token", &self.oauth_token)]);
                let req = req.query(&[("prettyPrint", &self.pretty_print)]);
                let req = req.query(&[("quotaUser", &self.quota_user)]);
                let req = req.query(&[("userIp", &self.user_ip)]);
                let mut auth = self.auth.lock().unwrap();
                let fut = auth.token(vec!["https://www.googleapis.com/auth/plus.login"]);
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
    pub mod media {
        pub mod params {
            #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
            pub enum InsertCollection {
                #[doc = "Upload the media to share on Google+."]
                Cloud,
            }
            impl InsertCollection {
                pub fn as_str(self) -> &'static str {
                    match self {
                        InsertCollection::Cloud => "cloud",
                    }
                }
            }
            impl ::std::fmt::Display for InsertCollection {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                    f.write_str(self.as_str())
                }
            }
            impl ::serde::Serialize for InsertCollection {
                fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
                where
                    S: ::serde::ser::Serializer,
                {
                    serializer.serialize_str(self.as_str())
                }
            }
            impl<'de> ::serde::Deserialize<'de> for InsertCollection {
                fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
                where
                    D: ::serde::de::Deserializer<'de>,
                {
                    let value: &'de str = <&str>::deserialize(deserializer)?;
                    Ok(match value {
                        "cloud" => InsertCollection::Cloud,
                        _ => {
                            return Err(::serde::de::Error::custom(format!(
                                "invalid enum for #name: {}",
                                value
                            )))
                        }
                    })
                }
            }
            impl ::field_selector::FieldSelector for InsertCollection {
                fn field_selector_with_ident(ident: &str, selector: &mut String) {
                    match selector.chars().rev().nth(0) {
                        Some(',') | None => {}
                        _ => selector.push_str(","),
                    }
                    selector.push_str(ident);
                }
            }
        }
        pub struct MediaActions<'a, A> {
            pub(crate) reqwest: &'a reqwest::Client,
            pub(crate) auth: &'a std::sync::Mutex<A>,
        }
        impl<'a, A: yup_oauth2::GetToken> MediaActions<'a, A> {
            #[doc = "Shut down. See https://developers.google.com/+/api-shutdown for more details."]
            pub fn insert(
                &self,
                request: crate::schemas::Media,
                user_id: impl Into<String>,
                collection: crate::resources::media::params::InsertCollection,
            ) -> InsertRequestBuilder<A> {
                InsertRequestBuilder {
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
                    collection,
                }
            }
        }
        #[derive(Debug, Clone)]
        pub struct InsertRequestBuilder<'a, A> {
            pub(crate) reqwest: &'a ::reqwest::Client,
            pub(crate) auth: &'a ::std::sync::Mutex<A>,
            request: crate::schemas::Media,
            user_id: String,
            collection: crate::resources::media::params::InsertCollection,
            alt: Option<crate::params::Alt>,
            fields: Option<String>,
            key: Option<String>,
            oauth_token: Option<String>,
            pretty_print: Option<bool>,
            quota_user: Option<String>,
            user_ip: Option<String>,
        }
        impl<'a, A: yup_oauth2::GetToken> InsertRequestBuilder<'a, A> {
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
            fn _simple_upload_path(&self) -> String {
                let mut output = "https://www.googleapis.com/".to_owned();
                output.push_str("upload/plusDomains/v1/people/");
                {
                    let var_as_str = &self.user_id;
                    output.extend(::percent_encoding::utf8_percent_encode(
                        &var_as_str,
                        crate::SIMPLE,
                    ));
                }
                output.push_str("/media/");
                {
                    let var_as_string = self.collection.to_string();
                    let var_as_str = &var_as_string;
                    output.extend(::percent_encoding::utf8_percent_encode(
                        &var_as_str,
                        crate::SIMPLE,
                    ));
                }
                output
            }
            pub fn upload<T, R>(
                mut self,
                content: R,
                mime_type: ::mime::Mime,
            ) -> Result<T, Box<dyn ::std::error::Error>>
            where
                T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
                R: ::std::io::Read + ::std::io::Seek + Send + 'static,
            {
                if self.fields.is_none() {
                    self.fields = Some(T::field_selector());
                }
                let req = self._request(&self._simple_upload_path());
                let req = req.query(&[("uploadType", "multipart")]);
                use crate::multipart::{Part, RelatedMultiPart};
                let mut multipart = RelatedMultiPart::new();
                let request_json = ::serde_json::to_vec(&self.request)?;
                multipart.new_part(Part::new(
                    ::mime::APPLICATION_JSON,
                    Box::new(::std::io::Cursor::new(request_json)),
                ));
                multipart.new_part(Part::new(mime_type, Box::new(content)));
                let req = req.header(
                    ::reqwest::header::CONTENT_TYPE,
                    format!("multipart/related; boundary={}", multipart.boundary()),
                );
                let req = req.body(reqwest::Body::new(multipart.into_reader()));
                Ok(req.send()?.error_for_status()?.json()?)
            }
            fn _resumable_upload_path(&self) -> String {
                let mut output = "https://www.googleapis.com/".to_owned();
                output.push_str("resumable/upload/plusDomains/v1/people/");
                {
                    let var_as_str = &self.user_id;
                    output.extend(::percent_encoding::utf8_percent_encode(
                        &var_as_str,
                        crate::SIMPLE,
                    ));
                }
                output.push_str("/media/");
                {
                    let var_as_string = self.collection.to_string();
                    let var_as_str = &var_as_string;
                    output.extend(::percent_encoding::utf8_percent_encode(
                        &var_as_str,
                        crate::SIMPLE,
                    ));
                }
                output
            }
            pub fn start_resumable_upload(
                self,
                mime_type: ::mime::Mime,
            ) -> Result<crate::ResumableUpload, Box<dyn ::std::error::Error>> {
                let req = self._request(&self._resumable_upload_path());
                let req = req.query(&[("uploadType", "resumable")]);
                let req = req.header(
                    ::reqwest::header::HeaderName::from_static("x-upload-content-type"),
                    mime_type.to_string(),
                );
                let req = req.json(&self.request);
                let resp = req.send()?.error_for_status()?;
                let location_header =
                    resp.headers()
                        .get(::reqwest::header::LOCATION)
                        .ok_or_else(|| {
                            format!("No LOCATION header returned when initiating resumable upload")
                        })?;
                let upload_url = ::std::str::from_utf8(location_header.as_bytes())?.to_owned();
                Ok(crate::ResumableUpload::new(
                    self.reqwest.clone(),
                    upload_url,
                ))
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
            ) -> Result<crate::schemas::Media, Box<dyn ::std::error::Error>> {
                self.execute_fields::<_, &str>(None)
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub fn execute_debug(
                self,
            ) -> Result<crate::schemas::Media, Box<dyn ::std::error::Error>> {
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
                let mut output = "https://www.googleapis.com/plusDomains/v1/".to_owned();
                output.push_str("people/");
                {
                    let var_as_str = &self.user_id;
                    output.extend(::percent_encoding::utf8_percent_encode(
                        &var_as_str,
                        crate::SIMPLE,
                    ));
                }
                output.push_str("/media/");
                {
                    let var_as_string = self.collection.to_string();
                    let var_as_str = &var_as_string;
                    output.extend(::percent_encoding::utf8_percent_encode(
                        &var_as_str,
                        crate::SIMPLE,
                    ));
                }
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
                let mut auth = self.auth.lock().unwrap();
                let fut = auth.token(vec!["https://www.googleapis.com/auth/plus.login"]);
                let mut runtime = ::tokio::runtime::Runtime::new().unwrap();
                let token = runtime.block_on(fut).unwrap().access_token;
                let req = req.bearer_auth(&token);
                req
            }
        }
    }
    pub mod people {
        pub mod params {
            #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
            pub enum ListCollection {
                #[doc = "The list of people who this user has added to one or more circles."]
                Circled,
            }
            impl ListCollection {
                pub fn as_str(self) -> &'static str {
                    match self {
                        ListCollection::Circled => "circled",
                    }
                }
            }
            impl ::std::fmt::Display for ListCollection {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                    f.write_str(self.as_str())
                }
            }
            impl ::serde::Serialize for ListCollection {
                fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
                where
                    S: ::serde::ser::Serializer,
                {
                    serializer.serialize_str(self.as_str())
                }
            }
            impl<'de> ::serde::Deserialize<'de> for ListCollection {
                fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
                where
                    D: ::serde::de::Deserializer<'de>,
                {
                    let value: &'de str = <&str>::deserialize(deserializer)?;
                    Ok(match value {
                        "circled" => ListCollection::Circled,
                        _ => {
                            return Err(::serde::de::Error::custom(format!(
                                "invalid enum for #name: {}",
                                value
                            )))
                        }
                    })
                }
            }
            impl ::field_selector::FieldSelector for ListCollection {
                fn field_selector_with_ident(ident: &str, selector: &mut String) {
                    match selector.chars().rev().nth(0) {
                        Some(',') | None => {}
                        _ => selector.push_str(","),
                    }
                    selector.push_str(ident);
                }
            }
            #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
            pub enum ListOrderBy {
                #[doc = "Order the people by their display name."]
                Alphabetical,
                #[doc = "Order people based on the relevence to the viewer."]
                Best,
            }
            impl ListOrderBy {
                pub fn as_str(self) -> &'static str {
                    match self {
                        ListOrderBy::Alphabetical => "alphabetical",
                        ListOrderBy::Best => "best",
                    }
                }
            }
            impl ::std::fmt::Display for ListOrderBy {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                    f.write_str(self.as_str())
                }
            }
            impl ::serde::Serialize for ListOrderBy {
                fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
                where
                    S: ::serde::ser::Serializer,
                {
                    serializer.serialize_str(self.as_str())
                }
            }
            impl<'de> ::serde::Deserialize<'de> for ListOrderBy {
                fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
                where
                    D: ::serde::de::Deserializer<'de>,
                {
                    let value: &'de str = <&str>::deserialize(deserializer)?;
                    Ok(match value {
                        "alphabetical" => ListOrderBy::Alphabetical,
                        "best" => ListOrderBy::Best,
                        _ => {
                            return Err(::serde::de::Error::custom(format!(
                                "invalid enum for #name: {}",
                                value
                            )))
                        }
                    })
                }
            }
            impl ::field_selector::FieldSelector for ListOrderBy {
                fn field_selector_with_ident(ident: &str, selector: &mut String) {
                    match selector.chars().rev().nth(0) {
                        Some(',') | None => {}
                        _ => selector.push_str(","),
                    }
                    selector.push_str(ident);
                }
            }
            #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
            pub enum ListByActivityCollection {
                #[doc = "List all people who have +1'd this activity."]
                Plusoners,
                #[doc = "List all people who have reshared this activity."]
                Resharers,
                #[doc = "List all people who this activity was shared to."]
                Sharedto,
            }
            impl ListByActivityCollection {
                pub fn as_str(self) -> &'static str {
                    match self {
                        ListByActivityCollection::Plusoners => "plusoners",
                        ListByActivityCollection::Resharers => "resharers",
                        ListByActivityCollection::Sharedto => "sharedto",
                    }
                }
            }
            impl ::std::fmt::Display for ListByActivityCollection {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                    f.write_str(self.as_str())
                }
            }
            impl ::serde::Serialize for ListByActivityCollection {
                fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
                where
                    S: ::serde::ser::Serializer,
                {
                    serializer.serialize_str(self.as_str())
                }
            }
            impl<'de> ::serde::Deserialize<'de> for ListByActivityCollection {
                fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
                where
                    D: ::serde::de::Deserializer<'de>,
                {
                    let value: &'de str = <&str>::deserialize(deserializer)?;
                    Ok(match value {
                        "plusoners" => ListByActivityCollection::Plusoners,
                        "resharers" => ListByActivityCollection::Resharers,
                        "sharedto" => ListByActivityCollection::Sharedto,
                        _ => {
                            return Err(::serde::de::Error::custom(format!(
                                "invalid enum for #name: {}",
                                value
                            )))
                        }
                    })
                }
            }
            impl ::field_selector::FieldSelector for ListByActivityCollection {
                fn field_selector_with_ident(ident: &str, selector: &mut String) {
                    match selector.chars().rev().nth(0) {
                        Some(',') | None => {}
                        _ => selector.push_str(","),
                    }
                    selector.push_str(ident);
                }
            }
        }
        pub struct PeopleActions<'a, A> {
            pub(crate) reqwest: &'a reqwest::Client,
            pub(crate) auth: &'a std::sync::Mutex<A>,
        }
        impl<'a, A: yup_oauth2::GetToken> PeopleActions<'a, A> {
            #[doc = "Get a person's profile."]
            pub fn get(&self, user_id: impl Into<String>) -> GetRequestBuilder<A> {
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
                    user_id: user_id.into(),
                }
            }
            #[doc = "List all of the people in the specified collection."]
            pub fn list(
                &self,
                user_id: impl Into<String>,
                collection: crate::resources::people::params::ListCollection,
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
                    collection,
                    max_results: None,
                    order_by: None,
                    page_token: None,
                }
            }
            #[doc = "Shut down. See https://developers.google.com/+/api-shutdown for more details."]
            pub fn list_by_activity(
                &self,
                activity_id: impl Into<String>,
                collection: crate::resources::people::params::ListByActivityCollection,
            ) -> ListByActivityRequestBuilder<A> {
                ListByActivityRequestBuilder {
                    reqwest: &self.reqwest,
                    auth: &self.auth,
                    alt: None,
                    fields: None,
                    key: None,
                    oauth_token: None,
                    pretty_print: None,
                    quota_user: None,
                    user_ip: None,
                    activity_id: activity_id.into(),
                    collection,
                    max_results: None,
                    page_token: None,
                }
            }
        }
        #[derive(Debug, Clone)]
        pub struct GetRequestBuilder<'a, A> {
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
        impl<'a, A: yup_oauth2::GetToken> GetRequestBuilder<'a, A> {
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
            ) -> Result<crate::schemas::Person, Box<dyn ::std::error::Error>> {
                self.execute_fields::<_, &str>(None)
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub fn execute_debug(
                self,
            ) -> Result<crate::schemas::Person, Box<dyn ::std::error::Error>> {
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
                let mut output = "https://www.googleapis.com/plusDomains/v1/".to_owned();
                output.push_str("people/");
                {
                    let var_as_str = &self.user_id;
                    output.extend(::percent_encoding::utf8_percent_encode(
                        &var_as_str,
                        crate::SIMPLE,
                    ));
                }
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
                let mut auth = self.auth.lock().unwrap();
                let fut = auth.token(vec!["https://www.googleapis.com/auth/plus.login"]);
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
            user_id: String,
            collection: crate::resources::people::params::ListCollection,
            max_results: Option<u32>,
            order_by: Option<crate::resources::people::params::ListOrderBy>,
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
            #[doc = "The maximum number of people to include in the response, which is used for paging. For any response, the actual number returned might be less than the specified maxResults."]
            pub fn max_results(mut self, value: u32) -> Self {
                self.max_results = Some(value);
                self
            }
            #[doc = "The order to return people in."]
            pub fn order_by(
                mut self,
                value: crate::resources::people::params::ListOrderBy,
            ) -> Self {
                self.order_by = Some(value);
                self
            }
            #[doc = "The continuation token, which is used to page through large result sets. To get the next page of results, set this parameter to the value of \"nextPageToken\" from the previous response."]
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
            pub fn iter_items<T>(mut self) -> crate::iter::PageItemIter<Self, T>
            where
                T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
            {
                let mut fields = concat!("nextPageToken,", "items").to_owned();
                let items_fields = T::field_selector();
                if !items_fields.is_empty() {
                    fields.push_str("(");
                    fields.push_str(&items_fields);
                    fields.push_str(")");
                }
                self.fields = Some(fields);
                crate::iter::PageItemIter::new(self, "items")
            }
            #[doc = r" Return an iterator that iterates over all `#prop_ident`. The"]
            #[doc = r" items yielded by the iterator are `#items_type`. The populated"]
            #[doc = r" fields in `#items_type` will be the default fields populated by"]
            #[doc = r" the server."]
            pub fn iter_items_standard(
                mut self,
            ) -> crate::iter::PageItemIter<Self, crate::schemas::Person> {
                self.fields = Some(concat!("nextPageToken,", "items").to_owned());
                crate::iter::PageItemIter::new(self, "items")
            }
            #[doc = r" Return an iterator that iterates over all `#prop_ident`. The"]
            #[doc = r" items yielded by the iterator are `#items_type`. The populated"]
            #[doc = r" fields in `#items_type` will be all fields available. This should"]
            #[doc = r" primarily be used during developement and debugging as fetching"]
            #[doc = r" all fields can be expensive both in bandwidth and server"]
            #[doc = r" resources."]
            pub fn iter_items_debug(
                mut self,
            ) -> crate::iter::PageItemIter<Self, crate::schemas::Person> {
                self.fields = Some(concat!("nextPageToken,", "items", "(*)").to_owned());
                crate::iter::PageItemIter::new(self, "items")
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
            pub fn iter_standard(self) -> crate::iter::PageIter<Self, crate::schemas::PeopleFeed> {
                crate::iter::PageIter::new(self)
            }
            pub fn iter_debug(mut self) -> crate::iter::PageIter<Self, crate::schemas::PeopleFeed> {
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
            ) -> Result<crate::schemas::PeopleFeed, Box<dyn ::std::error::Error>> {
                self.execute_fields::<_, &str>(None)
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub fn execute_debug(
                self,
            ) -> Result<crate::schemas::PeopleFeed, Box<dyn ::std::error::Error>> {
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
                let mut output = "https://www.googleapis.com/plusDomains/v1/".to_owned();
                output.push_str("people/");
                {
                    let var_as_str = &self.user_id;
                    output.extend(::percent_encoding::utf8_percent_encode(
                        &var_as_str,
                        crate::SIMPLE,
                    ));
                }
                output.push_str("/people/");
                {
                    let var_as_string = self.collection.to_string();
                    let var_as_str = &var_as_string;
                    output.extend(::percent_encoding::utf8_percent_encode(
                        &var_as_str,
                        crate::SIMPLE,
                    ));
                }
                output
            }
            fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
                let req = self.reqwest.request(::reqwest::Method::GET, path);
                let req = req.query(&[("maxResults", &self.max_results)]);
                let req = req.query(&[("orderBy", &self.order_by)]);
                let req = req.query(&[("pageToken", &self.page_token)]);
                let req = req.query(&[("alt", &self.alt)]);
                let req = req.query(&[("fields", &self.fields)]);
                let req = req.query(&[("key", &self.key)]);
                let req = req.query(&[("oauth_token", &self.oauth_token)]);
                let req = req.query(&[("prettyPrint", &self.pretty_print)]);
                let req = req.query(&[("quotaUser", &self.quota_user)]);
                let req = req.query(&[("userIp", &self.user_ip)]);
                let mut auth = self.auth.lock().unwrap();
                let fut = auth.token(vec!["https://www.googleapis.com/auth/plus.circles.read"]);
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
        pub struct ListByActivityRequestBuilder<'a, A> {
            pub(crate) reqwest: &'a ::reqwest::Client,
            pub(crate) auth: &'a ::std::sync::Mutex<A>,
            activity_id: String,
            collection: crate::resources::people::params::ListByActivityCollection,
            max_results: Option<u32>,
            page_token: Option<String>,
            alt: Option<crate::params::Alt>,
            fields: Option<String>,
            key: Option<String>,
            oauth_token: Option<String>,
            pretty_print: Option<bool>,
            quota_user: Option<String>,
            user_ip: Option<String>,
        }
        impl<'a, A: yup_oauth2::GetToken> ListByActivityRequestBuilder<'a, A> {
            #[doc = "The maximum number of people to include in the response, which is used for paging. For any response, the actual number returned might be less than the specified maxResults."]
            pub fn max_results(mut self, value: u32) -> Self {
                self.max_results = Some(value);
                self
            }
            #[doc = "The continuation token, which is used to page through large result sets. To get the next page of results, set this parameter to the value of \"nextPageToken\" from the previous response."]
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
            pub fn iter_items<T>(mut self) -> crate::iter::PageItemIter<Self, T>
            where
                T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
            {
                let mut fields = concat!("nextPageToken,", "items").to_owned();
                let items_fields = T::field_selector();
                if !items_fields.is_empty() {
                    fields.push_str("(");
                    fields.push_str(&items_fields);
                    fields.push_str(")");
                }
                self.fields = Some(fields);
                crate::iter::PageItemIter::new(self, "items")
            }
            #[doc = r" Return an iterator that iterates over all `#prop_ident`. The"]
            #[doc = r" items yielded by the iterator are `#items_type`. The populated"]
            #[doc = r" fields in `#items_type` will be the default fields populated by"]
            #[doc = r" the server."]
            pub fn iter_items_standard(
                mut self,
            ) -> crate::iter::PageItemIter<Self, crate::schemas::Person> {
                self.fields = Some(concat!("nextPageToken,", "items").to_owned());
                crate::iter::PageItemIter::new(self, "items")
            }
            #[doc = r" Return an iterator that iterates over all `#prop_ident`. The"]
            #[doc = r" items yielded by the iterator are `#items_type`. The populated"]
            #[doc = r" fields in `#items_type` will be all fields available. This should"]
            #[doc = r" primarily be used during developement and debugging as fetching"]
            #[doc = r" all fields can be expensive both in bandwidth and server"]
            #[doc = r" resources."]
            pub fn iter_items_debug(
                mut self,
            ) -> crate::iter::PageItemIter<Self, crate::schemas::Person> {
                self.fields = Some(concat!("nextPageToken,", "items", "(*)").to_owned());
                crate::iter::PageItemIter::new(self, "items")
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
            pub fn iter_standard(self) -> crate::iter::PageIter<Self, crate::schemas::PeopleFeed> {
                crate::iter::PageIter::new(self)
            }
            pub fn iter_debug(mut self) -> crate::iter::PageIter<Self, crate::schemas::PeopleFeed> {
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
            ) -> Result<crate::schemas::PeopleFeed, Box<dyn ::std::error::Error>> {
                self.execute_fields::<_, &str>(None)
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub fn execute_debug(
                self,
            ) -> Result<crate::schemas::PeopleFeed, Box<dyn ::std::error::Error>> {
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
                let mut output = "https://www.googleapis.com/plusDomains/v1/".to_owned();
                output.push_str("activities/");
                {
                    let var_as_str = &self.activity_id;
                    output.extend(::percent_encoding::utf8_percent_encode(
                        &var_as_str,
                        crate::SIMPLE,
                    ));
                }
                output.push_str("/people/");
                {
                    let var_as_string = self.collection.to_string();
                    let var_as_str = &var_as_string;
                    output.extend(::percent_encoding::utf8_percent_encode(
                        &var_as_str,
                        crate::SIMPLE,
                    ));
                }
                output
            }
            fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
                let req = self.reqwest.request(::reqwest::Method::GET, path);
                let req = req.query(&[("maxResults", &self.max_results)]);
                let req = req.query(&[("pageToken", &self.page_token)]);
                let req = req.query(&[("alt", &self.alt)]);
                let req = req.query(&[("fields", &self.fields)]);
                let req = req.query(&[("key", &self.key)]);
                let req = req.query(&[("oauth_token", &self.oauth_token)]);
                let req = req.query(&[("prettyPrint", &self.pretty_print)]);
                let req = req.query(&[("quotaUser", &self.quota_user)]);
                let req = req.query(&[("userIp", &self.user_ip)]);
                let mut auth = self.auth.lock().unwrap();
                let fut = auth.token(vec!["https://www.googleapis.com/auth/plus.login"]);
                let mut runtime = ::tokio::runtime::Runtime::new().unwrap();
                let token = runtime.block_on(fut).unwrap().access_token;
                let req = req.bearer_auth(&token);
                req
            }
        }
        impl<'a, A: yup_oauth2::GetToken> crate::iter::IterableMethod
            for ListByActivityRequestBuilder<'a, A>
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
