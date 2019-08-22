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
    pub struct AdoptionInteraction {
        #[doc = "The timestamp, in microseconds since Epoch, of the latest treatment interaction that contributed to this adoption event."]
        #[serde(rename = "latestTreatmentInstanceTimestamp", default)]
        #[serde(with = "crate::parsed_string")]
        pub latest_treatment_instance_timestamp: ::std::option::Option<u64>,
    }
    impl ::field_selector::FieldSelector for AdoptionInteraction {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct AdvertiserExperienceData {
        #[doc = "A list of adopted treatment instance ids. Will use for adoption event."]
        #[serde(rename = "treatmentInstanceId", default)]
        pub treatment_instance_id: ::std::option::Option<Vec<String>>,
    }
    impl ::field_selector::FieldSelector for AdvertiserExperienceData {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct AssociatedEmail {
        #[doc = "The email address."]
        #[serde(rename = "emailAddress", default)]
        pub email_address: ::std::option::Option<String>,
        #[doc = "Represents associated email structure data."]
        #[serde(rename = "kind", default)]
        pub kind: ::std::option::Option<String>,
        #[doc = "The program or person who initiated the operation."]
        #[serde(rename = "operator", default)]
        pub operator: ::std::option::Option<String>,
    }
    impl ::field_selector::FieldSelector for AssociatedEmail {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct CasesData {
        #[doc = "The agent who initiated this interaction."]
        #[serde(rename = "agent", default)]
        pub agent: ::std::option::Option<crate::schemas::CasesInteractionSource>,
        #[doc = "The agent (Gaia ID) this case was assigned to at the point of this action."]
        #[serde(rename = "assignedGaia", default)]
        #[serde(with = "crate::parsed_string")]
        pub assigned_gaia: ::std::option::Option<i64>,
        #[doc = "The type of the email. Only available for the user interaction of email type. Possible values: SUPPORT_EMAIL, MARKETING_EMAIL, SYSTEM_EMAIL, GRM_EMAIL, BOB_EMAIL."]
        #[serde(rename = "emailType", default)]
        pub email_type: ::std::option::Option<String>,
        #[doc = "This ID will be globally unique even between different cases."]
        #[serde(rename = "messageId", default)]
        pub message_id: ::std::option::Option<String>,
        #[doc = "The state of the case at the point when the interaction occurs. Possible values: NEW, ASSIGNED, UNASSIGNED, IN_CONSULT, NEED_INFO, SOLUTION_OFFERED, FINISHED, MERGED, DELETED, BLOCKED_BY."]
        #[serde(rename = "state", default)]
        pub state: ::std::option::Option<String>,
        #[doc = "The user id which is involved in the interaction."]
        #[serde(rename = "userId", default)]
        pub user_id: ::std::option::Option<crate::schemas::UserId>,
    }
    impl ::field_selector::FieldSelector for CasesData {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct CasesInteractionSource {
        #[doc = "Only routing rule can have multiple values. For systems like UDS this field is empty. Note: The id should be the gaia id if type == USER. If so, it is stored as contact_gaia_id in the enclosing UserInteraction proto."]
        #[serde(rename = "id", default)]
        pub id: ::std::option::Option<Vec<String>>,
        #[doc = "Type of the source. Possible values: ROUTING_RULE, USER, API, EMAIL_PROC, CHAT_SUPPORT_BOT, OUTBOX_SENDER, UDS, MEDLEY, FINISHER, OUT_OF_OFFICE, ATLAS, CUF. For more info see //customer_support/cih/proto/cases_data.proto."]
        #[serde(rename = "type", default)]
        pub r#type: ::std::option::Option<String>,
    }
    impl ::field_selector::FieldSelector for CasesInteractionSource {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
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
    pub struct ChatInteraction;
    impl ::field_selector::FieldSelector for ChatInteraction {
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
        Copy,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct EmailInteraction;
    impl ::field_selector::FieldSelector for EmailInteraction {
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
    pub struct Entity {
        #[doc = "Child entities of the Entity."]
        #[serde(rename = "childEntity", default)]
        pub child_entity: ::std::option::Option<Vec<crate::schemas::Entity>>,
        #[doc = "Indicates the entity opt out of the chief company bigtable"]
        #[serde(rename = "deprecated", default)]
        pub deprecated: ::std::option::Option<bool>,
        #[doc = "The identifier associated with thie Entity. For ADWORDS_CID it is the customer id. For PARENT_COMPANY, CHILD_COMPANY, DIVISION or COMPANY it is the company id. For EMAIL it is the email address. For ADDRESS_DIGEST it is the SHA-1 digest of the email address."]
        #[serde(rename = "entityId", default)]
        pub entity_id: ::std::option::Option<String>,
        #[doc = "The type of the entity. Possible values: PARENT_COMPANY, CHILD_COMPANY, DIVISION, ADWORDS_CID, EMAIL, COMPANY, ADDRESS_DIGEST. For more info see: //customer_support/cih/proto/entity.proto."]
        #[serde(rename = "entityType", default)]
        pub entity_type: ::std::option::Option<String>,
        #[doc = "Represents entity structure data."]
        #[serde(rename = "kind", default)]
        pub kind: ::std::option::Option<String>,
        #[doc = "This flag is only available for any parent AdWords account entity of an email entity or any child email entity of an AdWords account entity. If the flag is true, the linkage of the AdWords account and the email is from the AdsDb. If the flag is false, it is from other sources such as gCases. Note, for either an AdWords account or an email, there is at most one linkage from AdsDb."]
        #[serde(rename = "linkedByAdsdb", default)]
        pub linked_by_adsdb: ::std::option::Option<bool>,
        #[doc = "Parent entities of the Entity."]
        #[serde(rename = "parentEntity", default)]
        pub parent_entity: ::std::option::Option<Vec<crate::schemas::Entity>>,
    }
    impl ::field_selector::FieldSelector for Entity {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct GammaData {
        #[doc = "Campaign id is the primary identifier for an email blast."]
        #[serde(rename = "campaignId", default)]
        pub campaign_id: ::std::option::Option<i32>,
        #[doc = "A dispatch is a particular mailing of a campaign."]
        #[serde(rename = "dispatchId", default)]
        #[serde(with = "crate::parsed_string")]
        pub dispatch_id: ::std::option::Option<i64>,
        #[doc = "Experiment id is a controlled variation on a particular dispatch of a campaign. It is being deprecated in favor of node id."]
        #[serde(rename = "experimentId", default)]
        pub experiment_id: ::std::option::Option<i32>,
        #[doc = "GAMMA Product ID. For more info please see java/com/google/ads/crm/gamma/common/domain/Product.java"]
        #[serde(rename = "gammaProductId", default)]
        pub gamma_product_id: ::std::option::Option<i32>,
        #[doc = "A node is a controlled variation on a particular dispatch of a campaign."]
        #[serde(rename = "nodeId", default)]
        pub node_id: ::std::option::Option<i32>,
        #[doc = "UAID of the recipient of the email, if available."]
        #[serde(rename = "uaid", default)]
        pub uaid: ::std::option::Option<crate::schemas::Uaid>,
    }
    impl ::field_selector::FieldSelector for GammaData {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct GenieData {
        #[doc = "If type = REQUEST_RECOMMENDATION, store the user signals."]
        #[serde(rename = "enteredText", default)]
        pub entered_text: ::std::option::Option<String>,
        #[doc = "Type of the Genie interaction. Possible values: REQUEST_RECOMMENDATION, SELECT_RECOMMENDATION, ENTER_TEXT, DISMISS_GENIE. For more info see: //customer_support/cih/proto/genie_data.proto."]
        #[serde(rename = "type", default)]
        pub r#type: ::std::option::Option<String>,
        #[doc = "A join key for interactions, used to follow the user through multiple Genie interactions."]
        #[serde(rename = "sessionId", default)]
        pub session_id: ::std::option::Option<String>,
        #[doc = "User agent."]
        #[serde(rename = "userAgent", default)]
        pub user_agent: ::std::option::Option<String>,
        #[doc = "The IP address of the user."]
        #[serde(rename = "userIp", default)]
        pub user_ip: ::std::option::Option<String>,
    }
    impl ::field_selector::FieldSelector for GenieData {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct GoalInteraction {
        #[doc = "The status of goal interaction. Possible values: OPEN, WON, LOST, ABANDONED."]
        #[serde(rename = "status", default)]
        pub status: ::std::option::Option<String>,
    }
    impl ::field_selector::FieldSelector for GoalInteraction {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct GreenTeaData {
        #[doc = "Additional information about the the meeting method (e.g. phone number, email address, hangout link). This is a copy of meeting_method_detail field in MeetingPB proto. The source of this field is a free-form text."]
        #[serde(rename = "meetingMethodDetail", default)]
        pub meeting_method_detail: ::std::option::Option<String>,
        #[doc = "PKs of Opportunity table."]
        #[serde(rename = "opportunityId", default)]
        pub opportunity_id: ::std::option::Option<Vec<i64>>,
    }
    impl ::field_selector::FieldSelector for GreenTeaData {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct GrmData {
        #[doc = "Some company ids represent real-world agencies to GRM. If the interaction applies to an agency this value will be set."]
        #[serde(rename = "agencyId", default)]
        #[serde(with = "crate::parsed_string")]
        pub agency_id: ::std::option::Option<i64>,
        #[doc = "The LDAP of the internal Google owner of this interaction."]
        #[serde(rename = "contactLdap", default)]
        pub contact_ldap: ::std::option::Option<String>,
        #[doc = "This field indicates the Interaction Intent. The intents are stored externally."]
        #[serde(rename = "intentId", default)]
        pub intent_id: ::std::option::Option<Vec<i64>>,
    }
    impl ::field_selector::FieldSelector for GrmData {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
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
    pub struct HangoutInteraction;
    impl ::field_selector::FieldSelector for HangoutInteraction {
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
    pub struct HelpcenterData {
        #[doc = "Internal name is unique and maps to gKMS."]
        #[serde(rename = "internalHelpCenterName", default)]
        pub internal_help_center_name: ::std::option::Option<String>,
        #[doc = "Was purely API or from UI?"]
        #[serde(rename = "isApiClient", default)]
        pub is_api_client: ::std::option::Option<bool>,
        #[doc = "Referrer (http) for request."]
        #[serde(rename = "referer", default)]
        pub referer: ::std::option::Option<String>,
    }
    impl ::field_selector::FieldSelector for HelpcenterData {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct IncentiveInteractionMonetaryReward {
        #[doc = "Reward value represented in micros of the reward currency."]
        #[serde(rename = "amountMicros", default)]
        #[serde(with = "crate::parsed_string")]
        pub amount_micros: ::std::option::Option<i64>,
        #[doc = "Currency code as defined by Unicode's \"CLDR\", itself based on ISO 4217 (e.g., \"USD\")."]
        #[serde(rename = "currencyCode", default)]
        pub currency_code: ::std::option::Option<String>,
    }
    impl ::field_selector::FieldSelector for IncentiveInteractionMonetaryReward {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct IncentiveInteraction {
        #[doc = "The data of monetary reward of coupon code"]
        #[serde(rename = "monetaryReward", default)]
        pub monetary_reward:
            ::std::option::Option<crate::schemas::IncentiveInteractionMonetaryReward>,
        #[doc = "The type of the Incentive Interaction. Possible values: COUPON_CODE_REDEEMED, REWARD_GRANTED, COUPON_CODE_REDEEMED_REWARD_GRANTED."]
        #[serde(rename = "type", default)]
        pub r#type: ::std::option::Option<String>,
    }
    impl ::field_selector::FieldSelector for IncentiveInteraction {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct IncentivesData {
        #[doc = "Foreign keys of related objects within go/Incentives system. (Coupon code is sent as source_system_primary_key field)."]
        #[serde(rename = "campaignId", default)]
        #[serde(with = "crate::parsed_string")]
        pub campaign_id: ::std::option::Option<i64>,
        #[doc = "The Id of the coupon."]
        #[serde(rename = "couponId", default)]
        #[serde(with = "crate::parsed_string")]
        pub coupon_id: ::std::option::Option<i64>,
        #[doc = "The batch Id of the incentive."]
        #[serde(rename = "incentiveBatchId", default)]
        #[serde(with = "crate::parsed_string")]
        pub incentive_batch_id: ::std::option::Option<i64>,
        #[doc = "the Id of the incentive."]
        #[serde(rename = "incentiveId", default)]
        #[serde(with = "crate::parsed_string")]
        pub incentive_id: ::std::option::Option<i64>,
    }
    impl ::field_selector::FieldSelector for IncentivesData {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct MarketingEmailInteraction {
        #[doc = "If false, the mail was delivered. If true, mail delivery may have been re-attempted but it is possible the mail never made it to the intended recipient."]
        #[serde(rename = "bounced", default)]
        pub bounced: ::std::option::Option<bool>,
        #[doc = "If true, one or more non-opt-out links in the email have been clicked and the opt-out link click has not been clicked."]
        #[serde(rename = "clicked", default)]
        pub clicked: ::std::option::Option<bool>,
        #[doc = "SHA-1 Digest of the normalized destination (To:) email address. We don't store the raw address."]
        #[serde(rename = "emailAddressDigest", default)]
        pub email_address_digest: ::std::option::Option<String>,
        #[doc = "The from address of the email."]
        #[serde(rename = "fromAddress", default)]
        pub from_address: ::std::option::Option<String>,
        #[doc = "If true, the email has been opened and the opt-out link click has not been clicked."]
        #[serde(rename = "opened", default)]
        pub opened: ::std::option::Option<bool>,
        #[doc = "If true, the user clicked on the opt-out link in the email some time after the email was sent."]
        #[serde(rename = "optoutClicked", default)]
        pub optout_clicked: ::std::option::Option<bool>,
        #[doc = "Possible values: SENT, BOUNCED, OPENED, CLICKED, OPTOUT_CLICKED. For types other than SENT, the optional boolean field will be set to true, and the interaction timestamp will indicate the time of the post-send event. For SENT, the optional boolean fields reflect a summary of all post-send events and more than one may be true. Note: we might not have OPEN event even if the interaction is already opened and and clicked."]
        #[serde(rename = "type", default)]
        pub r#type: ::std::option::Option<String>,
    }
    impl ::field_selector::FieldSelector for MarketingEmailInteraction {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct MeetingInteraction {
        #[doc = "Meeting status. Possible values: CREATED, MODIFIED, DELETED."]
        #[serde(rename = "status", default)]
        pub status: ::std::option::Option<String>,
    }
    impl ::field_selector::FieldSelector for MeetingInteraction {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct NoteInteraction {
        #[doc = "Note status. Possible values: CREATED, MODIFIED, DELETED."]
        #[serde(rename = "status", default)]
        pub status: ::std::option::Option<String>,
    }
    impl ::field_selector::FieldSelector for NoteInteraction {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct Participant {
        #[doc = "GreenTea contact id if the participant is a GreenTea contact."]
        #[serde(rename = "contactId", default)]
        pub contact_id: ::std::option::Option<String>,
        #[doc = "Participant's email. If the interaction included email address, e.g email message, this field contains the sanitized address used in this interaction. If the interaction did not have email address, this field will contain the email address for the first Profile returned by Focus Backend Service"]
        #[serde(rename = "email", default)]
        pub email: ::std::option::Option<String>,
        #[serde(rename = "familyName", default)]
        pub family_name: ::std::option::Option<String>,
        #[doc = "GAIA id of the participant."]
        #[serde(rename = "gaiaId", default)]
        #[serde(with = "crate::parsed_string")]
        pub gaia_id: ::std::option::Option<i64>,
        #[serde(rename = "givenName", default)]
        pub given_name: ::std::option::Option<String>,
        #[doc = "Name of the participant. Depending on the source of interaction, the name may be either in unstructured or structured form."]
        #[serde(rename = "name", default)]
        pub name: ::std::option::Option<String>,
    }
    impl ::field_selector::FieldSelector for Participant {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
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
    pub struct PartnerSearchData;
    impl ::field_selector::FieldSelector for PartnerSearchData {
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
        Copy,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct PhoneInteraction;
    impl ::field_selector::FieldSelector for PhoneInteraction {
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
    pub struct TaskInteraction {
        #[doc = "Task status. Possible values: OPEN, CLOSED."]
        #[serde(rename = "status", default)]
        pub status: ::std::option::Option<String>,
    }
    impl ::field_selector::FieldSelector for TaskInteraction {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct TraxAgent {
        #[doc = "The agent's email."]
        #[serde(rename = "email", default)]
        pub email: ::std::option::Option<String>,
        #[doc = "The agent's gaia Id. It is an unobfuscated GaiaID."]
        #[serde(rename = "gaiaId", default)]
        pub gaia_id: ::std::option::Option<String>,
        #[doc = "The agent's name."]
        #[serde(rename = "name", default)]
        pub name: ::std::option::Option<String>,
    }
    impl ::field_selector::FieldSelector for TraxAgent {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct TraxData {
        #[doc = "The type of action that occurred. Possible values: NONE, INJECT, CLOSE, ASSIGN, ROUTE, EXPIRE, DELETE, OUTGOING_CONSULT, INCOMING_CONSULT, REPLY, REOPEN, BULK, UNBULK, BOUNCE, STIFLED_ACKNOWLEDGE, STIFLED_AUTO_RESPOND, CANCEL, MOVE, POSTPONE, SURVEY_SCHEDULED, SURVEY_SENT, CONSULT_INJECTED, MSG_SENT, LBL_APPLY, LBL_REMOVE, MOVE_IN. For more information see //customer_support/cih/proto/trax_data.proto."]
        #[serde(rename = "actionType", default)]
        pub action_type: ::std::option::Option<String>,
        #[doc = "Data of the agent that performed the action."]
        #[serde(rename = "agent", default)]
        pub agent: ::std::option::Option<crate::schemas::TraxAgent>,
        #[doc = "For a reassign action, this is the agent that the ticket was assigned to."]
        #[serde(rename = "assignedAgent", default)]
        pub assigned_agent: ::std::option::Option<crate::schemas::TraxAgent>,
        #[doc = "The message Id."]
        #[serde(rename = "messageId", default)]
        pub message_id: ::std::option::Option<String>,
        #[doc = "Trax state. Possible values: OPEN, CLOSED, PENDING, REOPENED, POSTPONED, BULKED, DELETED, DELAYED, WAITING, BROKEN, NEW, ASSIGNED, UNASSIGNED, IN_CONSULT, NEED_INFO, SOLUTION_OFFERED, FINISHED, MERGED. For more info see //customer_support/cih/proto/trax_data.proto."]
        #[serde(rename = "state", default)]
        pub state: ::std::option::Option<String>,
        #[doc = "The user Id of the ticket. It should contain email address for the TraxData."]
        #[serde(rename = "userId", default)]
        pub user_id: ::std::option::Option<crate::schemas::UserId>,
    }
    impl ::field_selector::FieldSelector for TraxData {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct TraxInteraction {
        #[doc = "The type of the Trax interaction. Possible values: EMAIL, PHONE, NOTE, CHAT, STUB."]
        #[serde(rename = "type", default)]
        pub r#type: ::std::option::Option<String>,
    }
    impl ::field_selector::FieldSelector for TraxInteraction {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct Uaid {
        #[doc = "Account id."]
        #[serde(rename = "account_id", default)]
        pub account_id: ::std::option::Option<String>,
        #[doc = "Product ID component of the UAID."]
        #[serde(rename = "product", default)]
        pub product: ::std::option::Option<String>,
    }
    impl ::field_selector::FieldSelector for Uaid {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
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
    pub struct UserCommData;
    impl ::field_selector::FieldSelector for UserCommData {
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
    pub struct UserId {
        #[doc = "customer id."]
        #[serde(rename = "customerId", default)]
        pub customer_id: ::std::option::Option<String>,
        #[doc = "email address."]
        #[serde(rename = "email", default)]
        pub email: ::std::option::Option<String>,
        #[doc = "The unobfuscated Gaia id."]
        #[serde(rename = "gaiaId", default)]
        pub gaia_id: ::std::option::Option<String>,
        #[doc = "The name."]
        #[serde(rename = "name", default)]
        pub name: ::std::option::Option<String>,
    }
    impl ::field_selector::FieldSelector for UserId {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct UserInteraction {
        #[doc = "available if the interaction is an ADOPTION type interaction."]
        #[serde(rename = "adoptionInteraction", default)]
        pub adoption_interaction: ::std::option::Option<crate::schemas::AdoptionInteraction>,
        #[doc = "Contains Advertiser Experience data for interactions from ADVERTISER_EXPERIENCE origin."]
        #[serde(rename = "advertiserExperienceData", default)]
        pub advertiser_experience_data:
            ::std::option::Option<crate::schemas::AdvertiserExperienceData>,
        #[doc = "Contains Cases data for interactions from CASES origin."]
        #[serde(rename = "casesData", default)]
        pub cases_data: ::std::option::Option<crate::schemas::CasesData>,
        #[doc = "Available if the interaction is a CHAT type interaction."]
        #[serde(rename = "chatInteraction", default)]
        pub chat_interaction: ::std::option::Option<crate::schemas::ChatInteraction>,
        #[doc = "The Gaia ID of the primary Google Contact/CSR involved in this interaction. Only certain types of interactions and origin systems provide this information. It is a string of digits in base 10."]
        #[serde(rename = "contactGaiaId", default)]
        pub contact_gaia_id: ::std::option::Option<String>,
        #[doc = "Google user or customer participant."]
        #[serde(rename = "customer", default)]
        pub customer: ::std::option::Option<Vec<crate::schemas::Participant>>,
        #[doc = "For detailed_description, only the first 512 characters are stored. If truncation was necessary, detailed_description_truncated will be true. Use the source_system_primarykey if necessary to refer back to the original object for more information rather than duplicating it."]
        #[serde(rename = "detailedDescription", default)]
        pub detailed_description: ::std::option::Option<String>,
        #[doc = "Indicates if the detailedDescription is truncated."]
        #[serde(rename = "detailedDescriptionTruncated", default)]
        pub detailed_description_truncated: ::std::option::Option<bool>,
        #[doc = "Available if the interaction is an EMAIL type interaction."]
        #[serde(rename = "emailInteraction", default)]
        pub email_interaction: ::std::option::Option<crate::schemas::EmailInteraction>,
        #[doc = "Only entity_id and entity_type of the following field is set. parent_entity and child_entities are not used here."]
        #[serde(rename = "entity", default)]
        pub entity: ::std::option::Option<crate::schemas::Entity>,
        #[doc = "Contains Gamma data for interactions from GAMMA origin."]
        #[serde(rename = "gammaData", default)]
        pub gamma_data: ::std::option::Option<crate::schemas::GammaData>,
        #[doc = "Contains Genie data for interactions from GENIE origin."]
        #[serde(rename = "genieData", default)]
        pub genie_data: ::std::option::Option<crate::schemas::GenieData>,
        #[doc = "Available if the interaction is a GOAL type interaction."]
        #[serde(rename = "goalInteraction", default)]
        pub goal_interaction: ::std::option::Option<crate::schemas::GoalInteraction>,
        #[doc = "Google participants. Can be full-time Googlers, interns, TVCs."]
        #[serde(rename = "googler", default)]
        pub googler: ::std::option::Option<Vec<crate::schemas::Participant>>,
        #[doc = "Contains Green Tea data for interactions from GREENTEA origin."]
        #[serde(rename = "greenTeaData", default)]
        pub green_tea_data: ::std::option::Option<crate::schemas::GreenTeaData>,
        #[doc = "Contains GRM data for interactions from GRM origin."]
        #[serde(rename = "grmData", default)]
        pub grm_data: ::std::option::Option<crate::schemas::GrmData>,
        #[doc = "available if the interaction is an HANGOUT type interaction."]
        #[serde(rename = "hangoutInteraction", default)]
        pub hangout_interaction: ::std::option::Option<crate::schemas::HangoutInteraction>,
        #[doc = "Contains Helpcenter data for interactions from HELPCENTER origin."]
        #[serde(rename = "helpcenterData", default)]
        pub helpcenter_data: ::std::option::Option<crate::schemas::HelpcenterData>,
        #[doc = "available if the interaction is an INCENTIVE type interaction."]
        #[serde(rename = "incentiveInteraction", default)]
        pub incentive_interaction: ::std::option::Option<crate::schemas::IncentiveInteraction>,
        #[doc = "Contains Incentive data for interactions from INCENTIVES origin."]
        #[serde(rename = "incentivesData", default)]
        pub incentives_data: ::std::option::Option<crate::schemas::IncentivesData>,
        #[doc = "Initiator of the interaction."]
        #[serde(rename = "initiator", default)]
        pub initiator: ::std::option::Option<crate::schemas::Participant>,
        #[doc = "The origin of the interaction. This field with source_system_primary_key uniquely determines the foreign key. Possible values: GENIE_SOURCE, GRM_SOURCE, CASES_SOURCE, GAMMA_SOURCE, TRAX_SOURCE, HELPCENTER_SOURCE."]
        #[serde(rename = "interactionOrigin", default)]
        pub interaction_origin: ::std::option::Option<String>,
        #[doc = "The type of the interaction. Possible values: STUB, PHONE, CHAT, EMAIL GOAL, TASK, NOTE, MEETING, MARKETING_EMAIL, TRAX, SUMMARY, HELPCENTER."]
        #[serde(rename = "interactionType", default)]
        pub interaction_type: ::std::option::Option<String>,
        #[doc = "Kind of this resource."]
        #[serde(rename = "kind", default)]
        pub kind: ::std::option::Option<String>,
        #[doc = "This stores a canonical language code as specified in go/iii. Use one of the converter classes (e.g. LanguageCode.forString()) to convert this to an enum."]
        #[serde(rename = "language", default)]
        pub language: ::std::option::Option<String>,
        #[doc = "Available if the interaction is a MARKETING_EMAIL type interaction."]
        #[serde(rename = "marketingEmailInteraction", default)]
        pub marketing_email_interaction:
            ::std::option::Option<crate::schemas::MarketingEmailInteraction>,
        #[doc = "Available if the interaction is a MEETING type interaction."]
        #[serde(rename = "meetingInteraction", default)]
        pub meeting_interaction: ::std::option::Option<crate::schemas::MeetingInteraction>,
        #[doc = "Interaction classification for CRM use cases. See http://go/cih-gt-api for details."]
        #[serde(rename = "metaType", default)]
        pub meta_type: ::std::option::Option<String>,
        #[doc = "Available if the interaction is a NOTE type interaction."]
        #[serde(rename = "noteInteraction", default)]
        pub note_interaction: ::std::option::Option<crate::schemas::NoteInteraction>,
        #[doc = "Other participant, such as maillist, GAIA group. May contain participants for which CIH was not able to determine if they are either Googler or Customer."]
        #[serde(rename = "otherParticipant", default)]
        pub other_participant: ::std::option::Option<Vec<crate::schemas::Participant>>,
        #[doc = "Contains Partner Search data for interactions from PARTNERSEARCH origin."]
        #[serde(rename = "partnerSearchData", default)]
        pub partner_search_data: ::std::option::Option<crate::schemas::PartnerSearchData>,
        #[doc = "Available if the interaction is a PHONE type interaction."]
        #[serde(rename = "phoneInteraction", default)]
        pub phone_interaction: ::std::option::Option<crate::schemas::PhoneInteraction>,
        #[doc = "Primary key of the interaction."]
        #[serde(rename = "sourceSystemPrimaryKey", default)]
        pub source_system_primary_key: ::std::option::Option<String>,
        #[doc = "If the interaction has one description, use summary. If it has a subject or summary and a longer description, use detailed_descripton for the body. For summary, only the first 128 characters are stored. If truncation was necessary, summary_truncated will be true."]
        #[serde(rename = "summary", default)]
        pub summary: ::std::option::Option<String>,
        #[doc = "Indicates if the summary is truncated."]
        #[serde(rename = "summaryTruncated", default)]
        pub summary_truncated: ::std::option::Option<bool>,
        #[doc = "Available if the interaction is a TASK type interaction."]
        #[serde(rename = "taskInteraction", default)]
        pub task_interaction: ::std::option::Option<crate::schemas::TaskInteraction>,
        #[doc = "Timestamp of the interaction as the number of microseconds since the Epoch."]
        #[serde(rename = "timestamp", default)]
        #[serde(with = "crate::parsed_string")]
        pub timestamp: ::std::option::Option<u64>,
        #[doc = "Contains Trax data for interactions from TRAX origin."]
        #[serde(rename = "traxData", default)]
        pub trax_data: ::std::option::Option<crate::schemas::TraxData>,
        #[doc = "Available if the interaction is a TRAX type interaction."]
        #[serde(rename = "traxInteraction", default)]
        pub trax_interaction: ::std::option::Option<crate::schemas::TraxInteraction>,
        #[doc = "Contains UserComm data for interactions from USERCOMM origin."]
        #[serde(rename = "userCommData", default)]
        pub user_comm_data: ::std::option::Option<crate::schemas::UserCommData>,
    }
    impl ::field_selector::FieldSelector for UserInteraction {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct UserInteractionsListResponse {
        #[doc = "Entity structures to be returned."]
        #[serde(rename = "entity", default)]
        pub entity: ::std::option::Option<Vec<crate::schemas::Entity>>,
        #[doc = "UserInteractions."]
        #[serde(rename = "items", default)]
        pub items: ::std::option::Option<Vec<crate::schemas::UserInteraction>>,
        #[doc = "Kind of this resource."]
        #[serde(rename = "kind", default)]
        pub kind: ::std::option::Option<String>,
        #[doc = "Token to pass in for pagination for the next page. Token is present only if more interactions are available."]
        #[serde(rename = "nextPageToken", default)]
        pub next_page_token: ::std::option::Option<String>,
    }
    impl ::field_selector::FieldSelector for UserInteractionsListResponse {
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
    #[doc = "Actions that can be performed on the associated_emails resource"]
    pub fn associated_emails(
        &self,
    ) -> crate::resources::associated_emails::AssociatedEmailsActions<A> {
        crate::resources::associated_emails::AssociatedEmailsActions {
            reqwest: &self.reqwest,
            auth: &self.auth,
        }
    }
    #[doc = "Actions that can be performed on the user_interactions resource"]
    pub fn user_interactions(
        &self,
    ) -> crate::resources::user_interactions::UserInteractionsActions<A> {
        crate::resources::user_interactions::UserInteractionsActions {
            reqwest: &self.reqwest,
            auth: &self.auth,
        }
    }
}
pub mod resources {
    pub mod associated_emails {
        pub mod params {}
        pub struct AssociatedEmailsActions<'a, A> {
            pub(crate) reqwest: &'a reqwest::Client,
            pub(crate) auth: &'a std::sync::Mutex<A>,
        }
        impl<'a, A: yup_oauth2::GetToken> AssociatedEmailsActions<'a, A> {
            #[doc = "Associates an AdWords customer id to an email address."]
            pub fn create(
                &self,
                request: crate::schemas::AssociatedEmail,
                customer_id: impl Into<String>,
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
                    customer_id: customer_id.into(),
                }
            }
            #[doc = "Dissociates an AdWords customer id from an email address."]
            pub fn delete(
                &self,
                customer_id: impl Into<String>,
                email_address: impl Into<String>,
                operator: impl Into<String>,
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
                    customer_id: customer_id.into(),
                    email_address: email_address.into(),
                    operator: operator.into(),
                }
            }
        }
        #[derive(Debug, Clone)]
        pub struct CreateRequestBuilder<'a, A> {
            pub(crate) reqwest: &'a ::reqwest::Client,
            pub(crate) auth: &'a ::std::sync::Mutex<A>,
            request: crate::schemas::AssociatedEmail,
            customer_id: String,
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
            ) -> Result<crate::schemas::AssociatedEmail, Box<dyn ::std::error::Error>> {
                self.execute_fields::<_, &str>(None)
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub fn execute_debug(
                self,
            ) -> Result<crate::schemas::AssociatedEmail, Box<dyn ::std::error::Error>> {
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
                let mut output = "https://www.googleapis.com/cih/v3/".to_owned();
                output.push_str("associatedEmails/insert/");
                {
                    let var_as_str = &self.customer_id;
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
        pub struct DeleteRequestBuilder<'a, A> {
            pub(crate) reqwest: &'a ::reqwest::Client,
            pub(crate) auth: &'a ::std::sync::Mutex<A>,
            customer_id: String,
            email_address: String,
            operator: String,
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
                let mut output = "https://www.googleapis.com/cih/v3/".to_owned();
                output.push_str("associatedEmails/delete/");
                {
                    let var_as_str = &self.customer_id;
                    output.extend(::percent_encoding::utf8_percent_encode(
                        &var_as_str,
                        crate::SIMPLE,
                    ));
                }
                output
            }
            fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
                let req = self.reqwest.request(::reqwest::Method::DELETE, path);
                let req = req.query(&[("emailAddress", &self.email_address)]);
                let req = req.query(&[("operator", &self.operator)]);
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
    pub mod user_interactions {
        pub mod params {}
        pub struct UserInteractionsActions<'a, A> {
            pub(crate) reqwest: &'a reqwest::Client,
            pub(crate) auth: &'a std::sync::Mutex<A>,
        }
        impl<'a, A: yup_oauth2::GetToken> UserInteractionsActions<'a, A> {
            #[doc = "Gets an interaction."]
            pub fn get(
                &self,
                entity_type: impl Into<String>,
                entity_id: impl Into<String>,
                timestamp: u64,
                interaction_type: impl Into<String>,
            ) -> GetRequestBuilder<A> {
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
                    entity_type: entity_type.into(),
                    entity_id: entity_id.into(),
                    timestamp,
                    interaction_type: interaction_type.into(),
                }
            }
            #[doc = "Inserts a new interaction to CIH."]
            pub fn insert(
                &self,
                request: crate::schemas::UserInteraction,
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
                }
            }
            #[doc = "Get a list of interactions for the given entity and its relatives if requested. The returned list is sorted by timestamp in descending order."]
            pub fn list(&self, entity: impl Into<Vec<String>>) -> ListRequestBuilder<A> {
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
                    entity: entity.into(),
                    entity_filter: None,
                    exclude_passed_interaction_origin: None,
                    exclude_passed_interaction_type: None,
                    include_related_interactions: None,
                    interaction_origin: None,
                    interaction_type: None,
                    lookup_participant_info: None,
                    max_interactions_per_page: None,
                    max_results: None,
                    meta_type_filter: None,
                    min_main_entity_interactions: None,
                    page_token: None,
                    phone_matcher: None,
                    timestamp_end: None,
                    timestamp_start: None,
                }
            }
        }
        #[derive(Debug, Clone)]
        pub struct GetRequestBuilder<'a, A> {
            pub(crate) reqwest: &'a ::reqwest::Client,
            pub(crate) auth: &'a ::std::sync::Mutex<A>,
            entity_type: String,
            entity_id: String,
            timestamp: u64,
            interaction_type: String,
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
            ) -> Result<crate::schemas::UserInteraction, Box<dyn ::std::error::Error>> {
                self.execute_fields::<_, &str>(None)
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub fn execute_debug(
                self,
            ) -> Result<crate::schemas::UserInteraction, Box<dyn ::std::error::Error>> {
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
                let mut output = "https://www.googleapis.com/cih/v3/".to_owned();
                output.push_str("userInteractions/");
                {
                    let var_as_str = &self.entity_type;
                    output.extend(::percent_encoding::utf8_percent_encode(
                        &var_as_str,
                        crate::SIMPLE,
                    ));
                }
                output.push_str("/");
                {
                    let var_as_str = &self.entity_id;
                    output.extend(::percent_encoding::utf8_percent_encode(
                        &var_as_str,
                        crate::SIMPLE,
                    ));
                }
                output.push_str("/");
                {
                    let var_as_string = self.timestamp.to_string();
                    let var_as_str = &var_as_string;
                    output.extend(::percent_encoding::utf8_percent_encode(
                        &var_as_str,
                        crate::SIMPLE,
                    ));
                }
                output.push_str("/");
                {
                    let var_as_str = &self.interaction_type;
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
                req
            }
        }
        #[derive(Debug, Clone)]
        pub struct InsertRequestBuilder<'a, A> {
            pub(crate) reqwest: &'a ::reqwest::Client,
            pub(crate) auth: &'a ::std::sync::Mutex<A>,
            request: crate::schemas::UserInteraction,
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
            ) -> Result<crate::schemas::UserInteraction, Box<dyn ::std::error::Error>> {
                self.execute_fields::<_, &str>(None)
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub fn execute_debug(
                self,
            ) -> Result<crate::schemas::UserInteraction, Box<dyn ::std::error::Error>> {
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
                let mut output = "https://www.googleapis.com/cih/v3/".to_owned();
                output.push_str("userInteractions/insert");
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
            entity: Vec<String>,
            entity_filter: Option<Vec<String>>,
            exclude_passed_interaction_origin: Option<bool>,
            exclude_passed_interaction_type: Option<bool>,
            include_related_interactions: Option<bool>,
            interaction_origin: Option<Vec<String>>,
            interaction_type: Option<Vec<String>>,
            lookup_participant_info: Option<bool>,
            max_interactions_per_page: Option<i32>,
            max_results: Option<u32>,
            meta_type_filter: Option<Vec<String>>,
            min_main_entity_interactions: Option<i32>,
            page_token: Option<String>,
            phone_matcher: Option<Vec<String>>,
            timestamp_end: Option<u64>,
            timestamp_start: Option<u64>,
            alt: Option<crate::params::Alt>,
            fields: Option<String>,
            key: Option<String>,
            oauth_token: Option<String>,
            pretty_print: Option<bool>,
            quota_user: Option<String>,
            user_ip: Option<String>,
        }
        impl<'a, A: yup_oauth2::GetToken> ListRequestBuilder<'a, A> {
            #[doc = "Primary or secondary entities, if available only interactions whose primary or secondary entities are given are returned. For info about the format of this field see the comments of UserInteractionsApiaryFilter proto."]
            pub fn entity_filter(mut self, value: impl Into<Vec<String>>) -> Self {
                self.entity_filter = Some(value.into());
                self
            }
            #[doc = "Indicates the inclusive or exclusive behavior of interactionOrigin field. See the description of interactionOrigin."]
            pub fn exclude_passed_interaction_origin(mut self, value: bool) -> Self {
                self.exclude_passed_interaction_origin = Some(value);
                self
            }
            #[doc = "Indicates the inclusive or exclusive behavior of interactionType field. See the description of interactionType."]
            pub fn exclude_passed_interaction_type(mut self, value: bool) -> Self {
                self.exclude_passed_interaction_type = Some(value);
                self
            }
            #[doc = "By default, all interactions which apply to any member of the entity structure which contains the provided entity are returned. If include_related_interactions is false, then only the interactions which are associated directly with this entity are returned, and neither parent_entity nor child_entities in the Entity object returned by this operation are populated."]
            pub fn include_related_interactions(mut self, value: bool) -> Self {
                self.include_related_interactions = Some(value);
                self
            }
            #[doc = "This limit is ignored if absent and all interactions regardless of their origin will be returned. Otherwise the meaning of this field depends on the include_interaction_origin field. 1. exclude_passed_interaction_origin is true: Only interactions whose Origin is contained in interaction_origin will be returned. 2. exclude_passed_interaction_origin is false: Only interactions whose Origin isn't contained in interaction_origin will be returned."]
            pub fn interaction_origin(mut self, value: impl Into<Vec<String>>) -> Self {
                self.interaction_origin = Some(value.into());
                self
            }
            #[doc = "This limit is ignored if absent and all interactions regardless of their type will be returned. Otherwise the meaning of this field depends on the exclude_passed_interaction_type field. 1. exclude_passed_interaction_type is true: Only interactions whose Type is contained in interaction_type will be returned. 2. exclude_passed_interaction_type is false: Only interactions whose Type isn't contained in interaction_type will be returned."]
            pub fn interaction_type(mut self, value: impl Into<Vec<String>>) -> Self {
                self.interaction_type = Some(value.into());
                self
            }
            #[doc = "Request to get additional information about interaction participants, such as names, email addresses. May increase latency of this call."]
            pub fn lookup_participant_info(mut self, value: bool) -> Self {
                self.lookup_participant_info = Some(value);
                self
            }
            #[doc = "The limit on the number of returned interactions. This is the maximum number of interactions which will be returned, starting with the most recent. Thie default value is 100. If it is equal to zero then only the entity structure is returned."]
            pub fn max_interactions_per_page(mut self, value: i32) -> Self {
                self.max_interactions_per_page = Some(value);
                self
            }
            #[doc = "The maximum number of results per page."]
            pub fn max_results(mut self, value: u32) -> Self {
                self.max_results = Some(value);
                self
            }
            #[doc = "Represents the interaction's classification. Possible values: SALES, SUPPORT, MARKETING. For more info see http://go/cih-gt-api"]
            pub fn meta_type_filter(mut self, value: impl Into<Vec<String>>) -> Self {
                self.meta_type_filter = Some(value.into());
                self
            }
            #[doc = "Attempt to read at least this many main entity interactions. The default value is set to 0. Pagination is disabled if a positive value is set."]
            pub fn min_main_entity_interactions(mut self, value: i32) -> Self {
                self.min_main_entity_interactions = Some(value);
                self
            }
            #[doc = "The pagination token."]
            pub fn page_token(mut self, value: impl Into<String>) -> Self {
                self.page_token = Some(value.into());
                self
            }
            #[doc = "If available only interactions whose phone number is given are returned. For info about the format of this field see the comments of UserInteractionsApiaryFilter proto."]
            pub fn phone_matcher(mut self, value: impl Into<Vec<String>>) -> Self {
                self.phone_matcher = Some(value.into());
                self
            }
            #[doc = "Upper limit on the timestamp for the returned interactions. It is measured as the number of microseconds since the Epoch."]
            pub fn timestamp_end(mut self, value: u64) -> Self {
                self.timestamp_end = Some(value);
                self
            }
            #[doc = "Lower limit on the timestamp for the returned interactions. It is measured as the number of microseconds since the Epoch."]
            pub fn timestamp_start(mut self, value: u64) -> Self {
                self.timestamp_start = Some(value);
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
            pub fn iter_entity<T>(mut self) -> crate::iter::PageItemIter<Self, T>
            where
                T: ::serde::de::DeserializeOwned + ::field_selector::FieldSelector,
            {
                let mut fields = concat!("nextPageToken,", "entity").to_owned();
                let items_fields = T::field_selector();
                if !items_fields.is_empty() {
                    fields.push_str("(");
                    fields.push_str(&items_fields);
                    fields.push_str(")");
                }
                self.fields = Some(fields);
                crate::iter::PageItemIter::new(self, "entity")
            }
            #[doc = r" Return an iterator that iterates over all `#prop_ident`. The"]
            #[doc = r" items yielded by the iterator are `#items_type`. The populated"]
            #[doc = r" fields in `#items_type` will be the default fields populated by"]
            #[doc = r" the server."]
            pub fn iter_entity_standard(
                mut self,
            ) -> crate::iter::PageItemIter<Self, crate::schemas::Entity> {
                self.fields = Some(concat!("nextPageToken,", "entity").to_owned());
                crate::iter::PageItemIter::new(self, "entity")
            }
            #[doc = r" Return an iterator that iterates over all `#prop_ident`. The"]
            #[doc = r" items yielded by the iterator are `#items_type`. The populated"]
            #[doc = r" fields in `#items_type` will be all fields available. This should"]
            #[doc = r" primarily be used during developement and debugging as fetching"]
            #[doc = r" all fields can be expensive both in bandwidth and server"]
            #[doc = r" resources."]
            pub fn iter_entity_debug(
                mut self,
            ) -> crate::iter::PageItemIter<Self, crate::schemas::Entity> {
                self.fields = Some(concat!("nextPageToken,", "entity", "(*)").to_owned());
                crate::iter::PageItemIter::new(self, "entity")
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
            ) -> crate::iter::PageItemIter<Self, crate::schemas::UserInteraction> {
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
            ) -> crate::iter::PageItemIter<Self, crate::schemas::UserInteraction> {
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
            ) -> crate::iter::PageIter<Self, crate::schemas::UserInteractionsListResponse>
            {
                crate::iter::PageIter::new(self)
            }
            pub fn iter_debug(
                mut self,
            ) -> crate::iter::PageIter<Self, crate::schemas::UserInteractionsListResponse>
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
            ) -> Result<crate::schemas::UserInteractionsListResponse, Box<dyn ::std::error::Error>>
            {
                self.execute_fields::<_, &str>(None)
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub fn execute_debug(
                self,
            ) -> Result<crate::schemas::UserInteractionsListResponse, Box<dyn ::std::error::Error>>
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
                let mut output = "https://www.googleapis.com/cih/v3/".to_owned();
                output.push_str("userInteractions");
                output
            }
            fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
                let req = self.reqwest.request(::reqwest::Method::GET, path);
                let req = req.query(&[("entity", &self.entity)]);
                let req = req.query(&[("entityFilter", &self.entity_filter)]);
                let req = req.query(&[(
                    "excludePassedInteractionOrigin",
                    &self.exclude_passed_interaction_origin,
                )]);
                let req = req.query(&[(
                    "excludePassedInteractionType",
                    &self.exclude_passed_interaction_type,
                )]);
                let req = req.query(&[(
                    "includeRelatedInteractions",
                    &self.include_related_interactions,
                )]);
                let req = req.query(&[("interactionOrigin", &self.interaction_origin)]);
                let req = req.query(&[("interactionType", &self.interaction_type)]);
                let req = req.query(&[("lookup_participant_info", &self.lookup_participant_info)]);
                let req = req.query(&[("maxInteractionsPerPage", &self.max_interactions_per_page)]);
                let req = req.query(&[("maxResults", &self.max_results)]);
                let req = req.query(&[("metaTypeFilter", &self.meta_type_filter)]);
                let req = req.query(&[(
                    "minMainEntityInteractions",
                    &self.min_main_entity_interactions,
                )]);
                let req = req.query(&[("pageToken", &self.page_token)]);
                let req = req.query(&[("phoneMatcher", &self.phone_matcher)]);
                let req = req.query(&[("timestampEnd", &self.timestamp_end)]);
                let req = req.query(&[("timestampStart", &self.timestamp_start)]);
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
