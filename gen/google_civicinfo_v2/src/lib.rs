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
    pub struct AdministrationRegion {
        #[doc = "The election administration body for this area."]
        #[serde(rename = "electionAdministrationBody", default)]
        pub election_administration_body: ::std::option::Option<crate::schemas::AdministrativeBody>,
        #[doc = "An ID for this object. IDs may change in future requests and should not be cached. Access to this field requires special access that can be requested from the Request more link on the Quotas page."]
        #[serde(rename = "id", default)]
        pub id: ::std::option::Option<String>,
        #[doc = "The city or county that provides election information for this voter. This object can have the same elements as state."]
        #[serde(rename = "local_jurisdiction", default)]
        pub local_jurisdiction: ::std::option::Option<Box<crate::schemas::AdministrationRegion>>,
        #[doc = "The name of the jurisdiction."]
        #[serde(rename = "name", default)]
        pub name: ::std::option::Option<String>,
        #[doc = "A list of sources for this area. If multiple sources are listed the data has been aggregated from those sources."]
        #[serde(rename = "sources", default)]
        pub sources: ::std::option::Option<Vec<crate::schemas::Source>>,
    }
    impl ::field_selector::FieldSelector for AdministrationRegion {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct AdministrativeBody {
        #[doc = "A URL provided by this administrative body for information on absentee voting."]
        #[serde(rename = "absenteeVotingInfoUrl", default)]
        pub absentee_voting_info_url: ::std::option::Option<String>,
        #[serde(rename = "addressLines", default)]
        pub address_lines: ::std::option::Option<Vec<String>>,
        #[doc = "A URL provided by this administrative body to give contest information to the voter."]
        #[serde(rename = "ballotInfoUrl", default)]
        pub ballot_info_url: ::std::option::Option<String>,
        #[doc = "The mailing address of this administrative body."]
        #[serde(rename = "correspondenceAddress", default)]
        pub correspondence_address: ::std::option::Option<crate::schemas::SimpleAddressType>,
        #[doc = "A URL provided by this administrative body for looking up general election information."]
        #[serde(rename = "electionInfoUrl", default)]
        pub election_info_url: ::std::option::Option<String>,
        #[doc = "The election officials for this election administrative body."]
        #[serde(rename = "electionOfficials", default)]
        pub election_officials: ::std::option::Option<Vec<crate::schemas::ElectionOfficial>>,
        #[doc = "A URL provided by this administrative body for confirming that the voter is registered to vote."]
        #[serde(rename = "electionRegistrationConfirmationUrl", default)]
        pub election_registration_confirmation_url: ::std::option::Option<String>,
        #[doc = "A URL provided by this administrative body for looking up how to register to vote."]
        #[serde(rename = "electionRegistrationUrl", default)]
        pub election_registration_url: ::std::option::Option<String>,
        #[doc = "A URL provided by this administrative body describing election rules to the voter."]
        #[serde(rename = "electionRulesUrl", default)]
        pub election_rules_url: ::std::option::Option<String>,
        #[doc = "A description of the hours of operation for this administrative body."]
        #[serde(rename = "hoursOfOperation", default)]
        pub hours_of_operation: ::std::option::Option<String>,
        #[doc = "The name of this election administrative body."]
        #[serde(rename = "name", default)]
        pub name: ::std::option::Option<String>,
        #[doc = "The physical address of this administrative body."]
        #[serde(rename = "physicalAddress", default)]
        pub physical_address: ::std::option::Option<crate::schemas::SimpleAddressType>,
        #[doc = "A description of the services this administrative body may provide."]
        #[serde(rename = "voter_services", default)]
        pub voter_services: ::std::option::Option<Vec<String>>,
        #[doc = "A URL provided by this administrative body for looking up where to vote."]
        #[serde(rename = "votingLocationFinderUrl", default)]
        pub voting_location_finder_url: ::std::option::Option<String>,
    }
    impl ::field_selector::FieldSelector for AdministrativeBody {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct Candidate {
        #[doc = "The URL for the candidate's campaign web site."]
        #[serde(rename = "candidateUrl", default)]
        pub candidate_url: ::std::option::Option<String>,
        #[doc = "A list of known (social) media channels for this candidate."]
        #[serde(rename = "channels", default)]
        pub channels: ::std::option::Option<Vec<crate::schemas::Channel>>,
        #[doc = "The email address for the candidate's campaign."]
        #[serde(rename = "email", default)]
        pub email: ::std::option::Option<String>,
        #[doc = "The candidate's name. If this is a joint ticket it will indicate the name of the candidate at the top of a ticket followed by a / and that name of candidate at the bottom of the ticket. e.g. \"Mitt Romney / Paul Ryan\""]
        #[serde(rename = "name", default)]
        pub name: ::std::option::Option<String>,
        #[doc = "The order the candidate appears on the ballot for this contest."]
        #[serde(rename = "orderOnBallot", default)]
        #[serde(with = "crate::parsed_string")]
        pub order_on_ballot: ::std::option::Option<i64>,
        #[doc = "The full name of the party the candidate is a member of."]
        #[serde(rename = "party", default)]
        pub party: ::std::option::Option<String>,
        #[doc = "The voice phone number for the candidate's campaign office."]
        #[serde(rename = "phone", default)]
        pub phone: ::std::option::Option<String>,
        #[doc = "A URL for a photo of the candidate."]
        #[serde(rename = "photoUrl", default)]
        pub photo_url: ::std::option::Option<String>,
    }
    impl ::field_selector::FieldSelector for Candidate {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct Channel {
        #[doc = "The unique public identifier for the candidate's channel."]
        #[serde(rename = "id", default)]
        pub id: ::std::option::Option<String>,
        #[doc = "The type of channel. The following is a list of types of channels, but is not exhaustive. More channel types may be added at a later time. One of: GooglePlus, YouTube, Facebook, Twitter"]
        #[serde(rename = "type", default)]
        pub r#type: ::std::option::Option<String>,
    }
    impl ::field_selector::FieldSelector for Channel {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct Contest {
        #[doc = "A number specifying the position of this contest on the voter's ballot."]
        #[serde(rename = "ballotPlacement", default)]
        #[serde(with = "crate::parsed_string")]
        pub ballot_placement: ::std::option::Option<i64>,
        #[doc = "The official title on the ballot for this contest, only where available."]
        #[serde(rename = "ballotTitle", default)]
        pub ballot_title: ::std::option::Option<String>,
        #[doc = "The candidate choices for this contest."]
        #[serde(rename = "candidates", default)]
        pub candidates: ::std::option::Option<Vec<crate::schemas::Candidate>>,
        #[doc = "Information about the electoral district that this contest is in."]
        #[serde(rename = "district", default)]
        pub district: ::std::option::Option<crate::schemas::ElectoralDistrict>,
        #[doc = "A description of any additional eligibility requirements for voting in this contest."]
        #[serde(rename = "electorateSpecifications", default)]
        pub electorate_specifications: ::std::option::Option<String>,
        #[doc = "An ID for this object. IDs may change in future requests and should not be cached. Access to this field requires special access that can be requested from the Request more link on the Quotas page."]
        #[serde(rename = "id", default)]
        pub id: ::std::option::Option<String>,
        #[doc = "The levels of government of the office for this contest. There may be more than one in cases where a jurisdiction effectively acts at two different levels of government; for example, the mayor of the District of Columbia acts at \"locality\" level, but also effectively at both \"administrative-area-2\" and \"administrative-area-1\"."]
        #[serde(rename = "level", default)]
        pub level: ::std::option::Option<Vec<String>>,
        #[doc = "The number of candidates that will be elected to office in this contest."]
        #[serde(rename = "numberElected", default)]
        #[serde(with = "crate::parsed_string")]
        pub number_elected: ::std::option::Option<i64>,
        #[doc = "The number of candidates that a voter may vote for in this contest."]
        #[serde(rename = "numberVotingFor", default)]
        #[serde(with = "crate::parsed_string")]
        pub number_voting_for: ::std::option::Option<i64>,
        #[doc = "The name of the office for this contest."]
        #[serde(rename = "office", default)]
        pub office: ::std::option::Option<String>,
        #[doc = "If this is a partisan election, the name of the party it is for."]
        #[serde(rename = "primaryParty", default)]
        pub primary_party: ::std::option::Option<String>,
        #[doc = "The type of contest. Usually this will be 'General', 'Primary', or 'Run-off' for contests with candidates. For referenda this will be 'Referendum'. For Retention contests this will typically be 'Retention'."]
        #[serde(rename = "type", default)]
        pub r#type: ::std::option::Option<String>,
        #[doc = "The set of ballot responses for the referendum. A ballot response represents a line on the ballot. Common examples might include \"yes\" or \"no\" for referenda. This field is only populated for contests of type 'Referendum'."]
        #[serde(rename = "referendumBallotResponses", default)]
        pub referendum_ballot_responses: ::std::option::Option<Vec<String>>,
        #[doc = "Specifies a short summary of the referendum that is typically on the ballot below the title but above the text. This field is only populated for contests of type 'Referendum'."]
        #[serde(rename = "referendumBrief", default)]
        pub referendum_brief: ::std::option::Option<String>,
        #[doc = "A statement in opposition to the referendum. It does not necessarily appear on the ballot. This field is only populated for contests of type 'Referendum'."]
        #[serde(rename = "referendumConStatement", default)]
        pub referendum_con_statement: ::std::option::Option<String>,
        #[doc = "Specifies what effect abstaining (not voting) on the proposition will have (i.e. whether abstaining is considered a vote against it). This field is only populated for contests of type 'Referendum'."]
        #[serde(rename = "referendumEffectOfAbstain", default)]
        pub referendum_effect_of_abstain: ::std::option::Option<String>,
        #[doc = "The threshold of votes that the referendum needs in order to pass, e.g. \"two-thirds\". This field is only populated for contests of type 'Referendum'."]
        #[serde(rename = "referendumPassageThreshold", default)]
        pub referendum_passage_threshold: ::std::option::Option<String>,
        #[doc = "A statement in favor of the referendum. It does not necessarily appear on the ballot. This field is only populated for contests of type 'Referendum'."]
        #[serde(rename = "referendumProStatement", default)]
        pub referendum_pro_statement: ::std::option::Option<String>,
        #[doc = "A brief description of the referendum. This field is only populated for contests of type 'Referendum'."]
        #[serde(rename = "referendumSubtitle", default)]
        pub referendum_subtitle: ::std::option::Option<String>,
        #[doc = "The full text of the referendum. This field is only populated for contests of type 'Referendum'."]
        #[serde(rename = "referendumText", default)]
        pub referendum_text: ::std::option::Option<String>,
        #[doc = "The title of the referendum (e.g. 'Proposition 42'). This field is only populated for contests of type 'Referendum'."]
        #[serde(rename = "referendumTitle", default)]
        pub referendum_title: ::std::option::Option<String>,
        #[doc = "A link to the referendum. This field is only populated for contests of type 'Referendum'."]
        #[serde(rename = "referendumUrl", default)]
        pub referendum_url: ::std::option::Option<String>,
        #[doc = "The roles which this office fulfills."]
        #[serde(rename = "roles", default)]
        pub roles: ::std::option::Option<Vec<String>>,
        #[doc = "A list of sources for this contest. If multiple sources are listed, the data has been aggregated from those sources."]
        #[serde(rename = "sources", default)]
        pub sources: ::std::option::Option<Vec<crate::schemas::Source>>,
        #[doc = "\"Yes\" or \"No\" depending on whether this a contest being held outside the normal election cycle."]
        #[serde(rename = "special", default)]
        pub special: ::std::option::Option<String>,
    }
    impl ::field_selector::FieldSelector for Contest {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct ContextParams {
        #[serde(rename = "clientProfile", default)]
        pub client_profile: ::std::option::Option<String>,
    }
    impl ::field_selector::FieldSelector for ContextParams {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct DivisionRepresentativeInfoRequest {
        #[serde(rename = "contextParams", default)]
        pub context_params: ::std::option::Option<crate::schemas::ContextParams>,
    }
    impl ::field_selector::FieldSelector for DivisionRepresentativeInfoRequest {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct DivisionSearchRequest {
        #[serde(rename = "contextParams", default)]
        pub context_params: ::std::option::Option<crate::schemas::ContextParams>,
    }
    impl ::field_selector::FieldSelector for DivisionSearchRequest {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct DivisionSearchResponse {
        #[doc = "Identifies what kind of resource this is. Value: the fixed string \"civicinfo#divisionSearchResponse\"."]
        #[serde(rename = "kind", default)]
        pub kind: ::std::option::Option<String>,
        #[serde(rename = "results", default)]
        pub results: ::std::option::Option<Vec<crate::schemas::DivisionSearchResult>>,
    }
    impl ::field_selector::FieldSelector for DivisionSearchResponse {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct DivisionSearchResult {
        #[doc = "Other Open Civic Data identifiers that refer to the same division -- for example, those that refer to other political divisions whose boundaries are defined to be coterminous with this one. For example, ocd-division/country:us/state:wy will include an alias of ocd-division/country:us/state:wy/cd:1, since Wyoming has only one Congressional district."]
        #[serde(rename = "aliases", default)]
        pub aliases: ::std::option::Option<Vec<String>>,
        #[doc = "The name of the division."]
        #[serde(rename = "name", default)]
        pub name: ::std::option::Option<String>,
        #[doc = "The unique Open Civic Data identifier for this division."]
        #[serde(rename = "ocdId", default)]
        pub ocd_id: ::std::option::Option<String>,
    }
    impl ::field_selector::FieldSelector for DivisionSearchResult {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct Election {
        #[doc = "Day of the election in YYYY-MM-DD format."]
        #[serde(rename = "electionDay", default)]
        pub election_day: ::std::option::Option<String>,
        #[doc = "The unique ID of this election."]
        #[serde(rename = "id", default)]
        #[serde(with = "crate::parsed_string")]
        pub id: ::std::option::Option<i64>,
        #[doc = "A displayable name for the election."]
        #[serde(rename = "name", default)]
        pub name: ::std::option::Option<String>,
        #[doc = "The political division of the election. Represented as an OCD Division ID. Voters within these political jurisdictions are covered by this election. This is typically a state such as ocd-division/country:us/state:ca or for the midterms or general election the entire US (i.e. ocd-division/country:us)."]
        #[serde(rename = "ocdDivisionId", default)]
        pub ocd_division_id: ::std::option::Option<String>,
    }
    impl ::field_selector::FieldSelector for Election {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct ElectionOfficial {
        #[doc = "The email address of the election official."]
        #[serde(rename = "emailAddress", default)]
        pub email_address: ::std::option::Option<String>,
        #[doc = "The fax number of the election official."]
        #[serde(rename = "faxNumber", default)]
        pub fax_number: ::std::option::Option<String>,
        #[doc = "The full name of the election official."]
        #[serde(rename = "name", default)]
        pub name: ::std::option::Option<String>,
        #[doc = "The office phone number of the election official."]
        #[serde(rename = "officePhoneNumber", default)]
        pub office_phone_number: ::std::option::Option<String>,
        #[doc = "The title of the election official."]
        #[serde(rename = "title", default)]
        pub title: ::std::option::Option<String>,
    }
    impl ::field_selector::FieldSelector for ElectionOfficial {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct ElectionsQueryRequest {
        #[serde(rename = "contextParams", default)]
        pub context_params: ::std::option::Option<crate::schemas::ContextParams>,
    }
    impl ::field_selector::FieldSelector for ElectionsQueryRequest {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct ElectionsQueryResponse {
        #[doc = "A list of available elections"]
        #[serde(rename = "elections", default)]
        pub elections: ::std::option::Option<Vec<crate::schemas::Election>>,
        #[doc = "Identifies what kind of resource this is. Value: the fixed string \"civicinfo#electionsQueryResponse\"."]
        #[serde(rename = "kind", default)]
        pub kind: ::std::option::Option<String>,
    }
    impl ::field_selector::FieldSelector for ElectionsQueryResponse {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct ElectoralDistrict {
        #[doc = "An identifier for this district, relative to its scope. For example, the 34th State Senate district would have id \"34\" and a scope of stateUpper."]
        #[serde(rename = "id", default)]
        pub id: ::std::option::Option<String>,
        #[serde(rename = "kgForeignKey", default)]
        pub kg_foreign_key: ::std::option::Option<String>,
        #[doc = "The name of the district."]
        #[serde(rename = "name", default)]
        pub name: ::std::option::Option<String>,
        #[doc = "The geographic scope of this district. If unspecified the district's geography is not known. One of: national, statewide, congressional, stateUpper, stateLower, countywide, judicial, schoolBoard, cityWide, township, countyCouncil, cityCouncil, ward, special"]
        #[serde(rename = "scope", default)]
        pub scope: ::std::option::Option<String>,
    }
    impl ::field_selector::FieldSelector for ElectoralDistrict {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct FieldMetadataProto {
        #[serde(rename = "internal", default)]
        pub internal: ::std::option::Option<crate::schemas::InternalFieldMetadataProto>,
    }
    impl ::field_selector::FieldSelector for FieldMetadataProto {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct GeographicDivision {
        #[doc = "Any other valid OCD IDs that refer to the same division.\n\nBecause OCD IDs are meant to be human-readable and at least somewhat predictable, there are occasionally several identifiers for a single division. These identifiers are defined to be equivalent to one another, and one is always indicated as the primary identifier. The primary identifier will be returned in ocd_id above, and any other equivalent valid identifiers will be returned in this list.\n\nFor example, if this division's OCD ID is ocd-division/country:us/district:dc, this will contain ocd-division/country:us/state:dc."]
        #[serde(rename = "alsoKnownAs", default)]
        pub also_known_as: ::std::option::Option<Vec<String>>,
        #[doc = "The name of the division."]
        #[serde(rename = "name", default)]
        pub name: ::std::option::Option<String>,
        #[doc = "List of indices in the offices array, one for each office elected from this division. Will only be present if includeOffices was true (or absent) in the request."]
        #[serde(rename = "officeIndices", default)]
        pub office_indices: ::std::option::Option<Vec<u32>>,
    }
    impl ::field_selector::FieldSelector for GeographicDivision {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct InternalFieldMetadataProto {
        #[serde(rename = "isAuto", default)]
        pub is_auto: ::std::option::Option<bool>,
        #[serde(rename = "sourceSummary", default)]
        pub source_summary: ::std::option::Option<crate::schemas::InternalSourceSummaryProto>,
    }
    impl ::field_selector::FieldSelector for InternalFieldMetadataProto {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct InternalSourceSummaryProto {
        #[serde(rename = "dataset", default)]
        pub dataset: ::std::option::Option<String>,
        #[serde(rename = "provider", default)]
        pub provider: ::std::option::Option<String>,
    }
    impl ::field_selector::FieldSelector for InternalSourceSummaryProto {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct LivegraphBacktraceRecordInfo {
        #[serde(rename = "dataSourcePublishMsec", default)]
        #[serde(with = "crate::parsed_string")]
        pub data_source_publish_msec: ::std::option::Option<i64>,
        #[serde(rename = "expId", default)]
        pub exp_id: ::std::option::Option<String>,
        #[serde(rename = "expInfo", default)]
        pub exp_info: ::std::option::Option<crate::schemas::LivegraphBacktraceRecordInfoExpInfo>,
        #[serde(rename = "isRecon", default)]
        pub is_recon: ::std::option::Option<bool>,
        #[serde(rename = "isWlmThrottled", default)]
        pub is_wlm_throttled: ::std::option::Option<bool>,
        #[serde(rename = "numberOfTriples", default)]
        #[serde(with = "crate::parsed_string")]
        pub number_of_triples: ::std::option::Option<i64>,
        #[serde(rename = "priority", default)]
        pub priority: ::std::option::Option<String>,
        #[serde(rename = "process", default)]
        pub process: ::std::option::Option<String>,
        #[serde(rename = "proxyReceiveMsec", default)]
        #[serde(with = "crate::parsed_string")]
        pub proxy_receive_msec: ::std::option::Option<i64>,
        #[serde(rename = "proxySentMsec", default)]
        #[serde(with = "crate::parsed_string")]
        pub proxy_sent_msec: ::std::option::Option<i64>,
        #[serde(rename = "recordId", default)]
        pub record_id: ::std::option::Option<String>,
        #[serde(rename = "shouldMonitorLatency", default)]
        pub should_monitor_latency: ::std::option::Option<bool>,
        #[serde(rename = "subscriberReceiveMsec", default)]
        #[serde(with = "crate::parsed_string")]
        pub subscriber_receive_msec: ::std::option::Option<i64>,
        #[serde(rename = "topicBuildFinishMsec", default)]
        #[serde(with = "crate::parsed_string")]
        pub topic_build_finish_msec: ::std::option::Option<i64>,
        #[serde(rename = "topicBuildStartMsec", default)]
        #[serde(with = "crate::parsed_string")]
        pub topic_build_start_msec: ::std::option::Option<i64>,
        #[serde(rename = "version", default)]
        pub version: ::std::option::Option<String>,
    }
    impl ::field_selector::FieldSelector for LivegraphBacktraceRecordInfo {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct LivegraphBacktraceRecordInfoExpInfo {
        #[serde(rename = "deletedIns", default)]
        pub deleted_ins: ::std::option::Option<Vec<String>>,
    }
    impl ::field_selector::FieldSelector for LivegraphBacktraceRecordInfoExpInfo {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct MessageSet {
        #[serde(rename = "recordMessageSetExt", default)]
        pub record_message_set_ext:
            ::std::option::Option<crate::schemas::LivegraphBacktraceRecordInfo>,
    }
    impl ::field_selector::FieldSelector for MessageSet {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct Office {
        #[doc = "The OCD ID of the division with which this office is associated."]
        #[serde(rename = "divisionId", default)]
        pub division_id: ::std::option::Option<String>,
        #[doc = "The levels of government of which this office is part. There may be more than one in cases where a jurisdiction effectively acts at two different levels of government; for example, the mayor of the District of Columbia acts at \"locality\" level, but also effectively at both \"administrative-area-2\" and \"administrative-area-1\"."]
        #[serde(rename = "levels", default)]
        pub levels: ::std::option::Option<Vec<String>>,
        #[doc = "The human-readable name of the office."]
        #[serde(rename = "name", default)]
        pub name: ::std::option::Option<String>,
        #[doc = "List of indices in the officials array of people who presently hold this office."]
        #[serde(rename = "officialIndices", default)]
        pub official_indices: ::std::option::Option<Vec<u32>>,
        #[doc = "The roles which this office fulfills. Roles are not meant to be exhaustive, or to exactly specify the entire set of responsibilities of a given office, but are meant to be rough categories that are useful for general selection from or sorting of a list of offices."]
        #[serde(rename = "roles", default)]
        pub roles: ::std::option::Option<Vec<String>>,
        #[doc = "A list of sources for this office. If multiple sources are listed, the data has been aggregated from those sources."]
        #[serde(rename = "sources", default)]
        pub sources: ::std::option::Option<Vec<crate::schemas::Source>>,
    }
    impl ::field_selector::FieldSelector for Office {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct Official {
        #[doc = "Addresses at which to contact the official."]
        #[serde(rename = "address", default)]
        pub address: ::std::option::Option<Vec<crate::schemas::SimpleAddressType>>,
        #[doc = "A list of known (social) media channels for this official."]
        #[serde(rename = "channels", default)]
        pub channels: ::std::option::Option<Vec<crate::schemas::Channel>>,
        #[doc = "The direct email addresses for the official."]
        #[serde(rename = "emails", default)]
        pub emails: ::std::option::Option<Vec<String>>,
        #[doc = "The official's name."]
        #[serde(rename = "name", default)]
        pub name: ::std::option::Option<String>,
        #[doc = "The full name of the party the official belongs to."]
        #[serde(rename = "party", default)]
        pub party: ::std::option::Option<String>,
        #[doc = "The official's public contact phone numbers."]
        #[serde(rename = "phones", default)]
        pub phones: ::std::option::Option<Vec<String>>,
        #[doc = "A URL for a photo of the official."]
        #[serde(rename = "photoUrl", default)]
        pub photo_url: ::std::option::Option<String>,
        #[doc = "The official's public website URLs."]
        #[serde(rename = "urls", default)]
        pub urls: ::std::option::Option<Vec<String>>,
    }
    impl ::field_selector::FieldSelector for Official {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct PointProto {
        #[serde(rename = "latE7", default)]
        pub lat_e7: ::std::option::Option<u32>,
        #[serde(rename = "lngE7", default)]
        pub lng_e7: ::std::option::Option<u32>,
        #[serde(rename = "metadata", default)]
        pub metadata: ::std::option::Option<crate::schemas::FieldMetadataProto>,
        #[serde(rename = "temporaryData", default)]
        pub temporary_data: ::std::option::Option<crate::schemas::MessageSet>,
    }
    impl ::field_selector::FieldSelector for PointProto {
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
    pub struct PollingLocation {
        #[doc = "The address of the location."]
        #[serde(rename = "address", default)]
        pub address: ::std::option::Option<crate::schemas::SimpleAddressType>,
        #[doc = "The last date that this early vote site or drop off location may be used. This field is not populated for polling locations."]
        #[serde(rename = "endDate", default)]
        pub end_date: ::std::option::Option<String>,
        #[doc = "An ID for this object. IDs may change in future requests and should not be cached. Access to this field requires special access that can be requested from the Request more link on the Quotas page."]
        #[serde(rename = "id", default)]
        pub id: ::std::option::Option<String>,
        #[doc = "Latitude of the location, in degrees north of the equator. Only some locations -- generally, ballot drop boxes for vote-by-mail elections -- will have this set; for others, use a geocoding service like the Google Maps API to resolve the address to a geographic point."]
        #[serde(rename = "latitude", default)]
        pub latitude: ::std::option::Option<f64>,
        #[doc = "Longitude of the location, in degrees east of the Prime Meridian. Only some locations -- generally, ballot drop boxes for vote-by-mail elections -- will have this set; for others, use a geocoding service like the Google Maps API to resolve the address to a geographic point."]
        #[serde(rename = "longitude", default)]
        pub longitude: ::std::option::Option<f64>,
        #[doc = "The name of the early vote site or drop off location. This field is not populated for polling locations."]
        #[serde(rename = "name", default)]
        pub name: ::std::option::Option<String>,
        #[doc = "Notes about this location (e.g. accessibility ramp or entrance to use)."]
        #[serde(rename = "notes", default)]
        pub notes: ::std::option::Option<String>,
        #[doc = "A description of when this location is open."]
        #[serde(rename = "pollingHours", default)]
        pub polling_hours: ::std::option::Option<String>,
        #[doc = "A list of sources for this location. If multiple sources are listed the data has been aggregated from those sources."]
        #[serde(rename = "sources", default)]
        pub sources: ::std::option::Option<Vec<crate::schemas::Source>>,
        #[doc = "The first date that this early vote site or drop off location may be used. This field is not populated for polling locations."]
        #[serde(rename = "startDate", default)]
        pub start_date: ::std::option::Option<String>,
        #[doc = "The services provided by this early vote site or drop off location. This field is not populated for polling locations."]
        #[serde(rename = "voterServices", default)]
        pub voter_services: ::std::option::Option<String>,
    }
    impl ::field_selector::FieldSelector for PollingLocation {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct PostalAddress {
        #[serde(rename = "addressLines", default)]
        pub address_lines: ::std::option::Option<Vec<String>>,
        #[serde(rename = "administrativeAreaName", default)]
        pub administrative_area_name: ::std::option::Option<String>,
        #[serde(rename = "countryName", default)]
        pub country_name: ::std::option::Option<String>,
        #[serde(rename = "countryNameCode", default)]
        pub country_name_code: ::std::option::Option<String>,
        #[serde(rename = "dependentLocalityName", default)]
        pub dependent_locality_name: ::std::option::Option<String>,
        #[serde(rename = "dependentThoroughfareName", default)]
        pub dependent_thoroughfare_name: ::std::option::Option<String>,
        #[serde(rename = "firmName", default)]
        pub firm_name: ::std::option::Option<String>,
        #[serde(rename = "isDisputed", default)]
        pub is_disputed: ::std::option::Option<bool>,
        #[serde(rename = "languageCode", default)]
        pub language_code: ::std::option::Option<String>,
        #[serde(rename = "localityName", default)]
        pub locality_name: ::std::option::Option<String>,
        #[serde(rename = "postBoxNumber", default)]
        pub post_box_number: ::std::option::Option<String>,
        #[serde(rename = "postalCodeNumber", default)]
        pub postal_code_number: ::std::option::Option<String>,
        #[serde(rename = "postalCodeNumberExtension", default)]
        pub postal_code_number_extension: ::std::option::Option<String>,
        #[serde(rename = "premiseName", default)]
        pub premise_name: ::std::option::Option<String>,
        #[serde(rename = "recipientName", default)]
        pub recipient_name: ::std::option::Option<String>,
        #[serde(rename = "sortingCode", default)]
        pub sorting_code: ::std::option::Option<String>,
        #[serde(rename = "subAdministrativeAreaName", default)]
        pub sub_administrative_area_name: ::std::option::Option<String>,
        #[serde(rename = "subPremiseName", default)]
        pub sub_premise_name: ::std::option::Option<String>,
        #[serde(rename = "thoroughfareName", default)]
        pub thoroughfare_name: ::std::option::Option<String>,
        #[serde(rename = "thoroughfareNumber", default)]
        pub thoroughfare_number: ::std::option::Option<String>,
    }
    impl ::field_selector::FieldSelector for PostalAddress {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct Provenance {
        #[serde(rename = "collidedSegmentSource", default)]
        pub collided_segment_source: ::std::option::Option<crate::schemas::StreetSegmentList>,
        #[serde(rename = "ctclContestUuid", default)]
        pub ctcl_contest_uuid: ::std::option::Option<String>,
        #[serde(rename = "ctclOfficeUuid", default)]
        pub ctcl_office_uuid: ::std::option::Option<String>,
        #[serde(rename = "datasetId", default)]
        #[serde(with = "crate::parsed_string")]
        pub dataset_id: ::std::option::Option<i64>,
        #[serde(rename = "precinctId", default)]
        #[serde(with = "crate::parsed_string")]
        pub precinct_id: ::std::option::Option<i64>,
        #[serde(rename = "precinctSplitId", default)]
        #[serde(with = "crate::parsed_string")]
        pub precinct_split_id: ::std::option::Option<i64>,
        #[serde(rename = "tsStreetSegmentId", default)]
        pub ts_street_segment_id: ::std::option::Option<String>,
        #[serde(rename = "vip5PrecinctId", default)]
        pub vip_5_precinct_id: ::std::option::Option<String>,
        #[serde(rename = "vip5StreetSegmentId", default)]
        pub vip_5_street_segment_id: ::std::option::Option<String>,
        #[serde(rename = "vipStreetSegmentId", default)]
        #[serde(with = "crate::parsed_string")]
        pub vip_street_segment_id: ::std::option::Option<i64>,
    }
    impl ::field_selector::FieldSelector for Provenance {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct RepresentativeInfoData {
        #[doc = "Political geographic divisions that contain the requested address."]
        #[serde(rename = "divisions", default)]
        pub divisions: ::std::option::Option<
            ::std::collections::BTreeMap<String, crate::schemas::GeographicDivision>,
        >,
        #[doc = "Elected offices referenced by the divisions listed above. Will only be present if includeOffices was true in the request."]
        #[serde(rename = "offices", default)]
        pub offices: ::std::option::Option<Vec<crate::schemas::Office>>,
        #[doc = "Officials holding the offices listed above. Will only be present if includeOffices was true in the request."]
        #[serde(rename = "officials", default)]
        pub officials: ::std::option::Option<Vec<crate::schemas::Official>>,
    }
    impl ::field_selector::FieldSelector for RepresentativeInfoData {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct RepresentativeInfoRequest {
        #[serde(rename = "contextParams", default)]
        pub context_params: ::std::option::Option<crate::schemas::ContextParams>,
    }
    impl ::field_selector::FieldSelector for RepresentativeInfoRequest {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct RepresentativeInfoResponse {
        #[doc = "Political geographic divisions that contain the requested address."]
        #[serde(rename = "divisions", default)]
        pub divisions: ::std::option::Option<
            ::std::collections::BTreeMap<String, crate::schemas::GeographicDivision>,
        >,
        #[doc = "Identifies what kind of resource this is. Value: the fixed string \"civicinfo#representativeInfoResponse\"."]
        #[serde(rename = "kind", default)]
        pub kind: ::std::option::Option<String>,
        #[doc = "The normalized version of the requested address"]
        #[serde(rename = "normalizedInput", default)]
        pub normalized_input: ::std::option::Option<crate::schemas::SimpleAddressType>,
        #[doc = "Elected offices referenced by the divisions listed above. Will only be present if includeOffices was true in the request."]
        #[serde(rename = "offices", default)]
        pub offices: ::std::option::Option<Vec<crate::schemas::Office>>,
        #[doc = "Officials holding the offices listed above. Will only be present if includeOffices was true in the request."]
        #[serde(rename = "officials", default)]
        pub officials: ::std::option::Option<Vec<crate::schemas::Official>>,
    }
    impl ::field_selector::FieldSelector for RepresentativeInfoResponse {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct SimpleAddressType {
        #[doc = "The city or town for the address."]
        #[serde(rename = "city", default)]
        pub city: ::std::option::Option<String>,
        #[doc = "The street name and number of this address."]
        #[serde(rename = "line1", default)]
        pub line_1: ::std::option::Option<String>,
        #[doc = "The second line the address, if needed."]
        #[serde(rename = "line2", default)]
        pub line_2: ::std::option::Option<String>,
        #[doc = "The third line of the address, if needed."]
        #[serde(rename = "line3", default)]
        pub line_3: ::std::option::Option<String>,
        #[doc = "The name of the location."]
        #[serde(rename = "locationName", default)]
        pub location_name: ::std::option::Option<String>,
        #[doc = "The US two letter state abbreviation of the address."]
        #[serde(rename = "state", default)]
        pub state: ::std::option::Option<String>,
        #[doc = "The US Postal Zip Code of the address."]
        #[serde(rename = "zip", default)]
        pub zip: ::std::option::Option<String>,
    }
    impl ::field_selector::FieldSelector for SimpleAddressType {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct Source {
        #[doc = "The name of the data source."]
        #[serde(rename = "name", default)]
        pub name: ::std::option::Option<String>,
        #[doc = "Whether this data comes from an official government source."]
        #[serde(rename = "official", default)]
        pub official: ::std::option::Option<bool>,
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
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct StreetSegment {
        #[serde(rename = "administrationRegionIds", default)]
        pub administration_region_ids: ::std::option::Option<Vec<String>>,
        #[serde(rename = "beforeGeocodeId", default)]
        pub before_geocode_id: ::std::option::Option<String>,
        #[serde(rename = "catalistUniquePrecinctCode", default)]
        pub catalist_unique_precinct_code: ::std::option::Option<String>,
        #[serde(rename = "city", default)]
        pub city: ::std::option::Option<String>,
        #[serde(rename = "cityCouncilDistrict", default)]
        pub city_council_district: ::std::option::Option<String>,
        #[serde(rename = "congressionalDistrict", default)]
        pub congressional_district: ::std::option::Option<String>,
        #[serde(rename = "contestIds", default)]
        pub contest_ids: ::std::option::Option<Vec<String>>,
        #[serde(rename = "countyCouncilDistrict", default)]
        pub county_council_district: ::std::option::Option<String>,
        #[serde(rename = "countyFips", default)]
        pub county_fips: ::std::option::Option<String>,
        #[serde(rename = "datasetId", default)]
        #[serde(with = "crate::parsed_string")]
        pub dataset_id: ::std::option::Option<i64>,
        #[serde(rename = "earlyVoteSiteByIds", default)]
        pub early_vote_site_by_ids: ::std::option::Option<Vec<String>>,
        #[serde(rename = "endHouseNumber", default)]
        #[serde(with = "crate::parsed_string")]
        pub end_house_number: ::std::option::Option<i64>,
        #[serde(rename = "geocodedPoint", default)]
        pub geocoded_point: ::std::option::Option<crate::schemas::PointProto>,
        #[serde(rename = "geographicDivisionOcdIds", default)]
        pub geographic_division_ocd_ids: ::std::option::Option<Vec<String>>,
        #[serde(rename = "id", default)]
        pub id: ::std::option::Option<String>,
        #[serde(rename = "judicialDistrict", default)]
        pub judicial_district: ::std::option::Option<String>,
        #[serde(rename = "mailOnly", default)]
        pub mail_only: ::std::option::Option<bool>,
        #[serde(rename = "municipalDistrict", default)]
        pub municipal_district: ::std::option::Option<String>,
        #[serde(rename = "ncoaAddress", default)]
        pub ncoa_address: ::std::option::Option<String>,
        #[serde(rename = "oddOrEvens", default)]
        pub odd_or_evens: ::std::option::Option<Vec<String>>,
        #[serde(rename = "originalId", default)]
        pub original_id: ::std::option::Option<String>,
        #[serde(rename = "pollinglocationByIds", default)]
        pub pollinglocation_by_ids: ::std::option::Option<Vec<String>>,
        #[serde(rename = "precinctName", default)]
        pub precinct_name: ::std::option::Option<String>,
        #[serde(rename = "precinctOcdId", default)]
        pub precinct_ocd_id: ::std::option::Option<String>,
        #[serde(rename = "provenances", default)]
        pub provenances: ::std::option::Option<Vec<crate::schemas::Provenance>>,
        #[serde(rename = "published", default)]
        pub published: ::std::option::Option<bool>,
        #[serde(rename = "schoolDistrict", default)]
        pub school_district: ::std::option::Option<String>,
        #[serde(rename = "startHouseNumber", default)]
        #[serde(with = "crate::parsed_string")]
        pub start_house_number: ::std::option::Option<i64>,
        #[serde(rename = "startLatE7", default)]
        #[serde(with = "crate::parsed_string")]
        pub start_lat_e7: ::std::option::Option<i64>,
        #[serde(rename = "startLngE7", default)]
        #[serde(with = "crate::parsed_string")]
        pub start_lng_e7: ::std::option::Option<i64>,
        #[serde(rename = "state", default)]
        pub state: ::std::option::Option<String>,
        #[serde(rename = "stateHouseDistrict", default)]
        pub state_house_district: ::std::option::Option<String>,
        #[serde(rename = "stateSenateDistrict", default)]
        pub state_senate_district: ::std::option::Option<String>,
        #[serde(rename = "streetName", default)]
        pub street_name: ::std::option::Option<String>,
        #[serde(rename = "subAdministrativeAreaName", default)]
        pub sub_administrative_area_name: ::std::option::Option<String>,
        #[serde(rename = "surrogateId", default)]
        #[serde(with = "crate::parsed_string")]
        pub surrogate_id: ::std::option::Option<i64>,
        #[serde(rename = "targetsmartUniquePrecinctCode", default)]
        pub targetsmart_unique_precinct_code: ::std::option::Option<String>,
        #[serde(rename = "townshipDistrict", default)]
        pub township_district: ::std::option::Option<String>,
        #[serde(rename = "unitNumber", default)]
        pub unit_number: ::std::option::Option<String>,
        #[serde(rename = "unitType", default)]
        pub unit_type: ::std::option::Option<String>,
        #[serde(rename = "vanPrecinctCode", default)]
        pub van_precinct_code: ::std::option::Option<String>,
        #[serde(rename = "voterGeographicDivisionOcdIds", default)]
        pub voter_geographic_division_ocd_ids: ::std::option::Option<Vec<String>>,
        #[serde(rename = "wardDistrict", default)]
        pub ward_district: ::std::option::Option<String>,
        #[serde(rename = "wildcard", default)]
        pub wildcard: ::std::option::Option<bool>,
        #[serde(rename = "zip", default)]
        pub zip: ::std::option::Option<String>,
    }
    impl ::field_selector::FieldSelector for StreetSegment {
        fn field_selector_with_ident(ident: &str, selector: &mut String) {
            match selector.chars().rev().nth(0) {
                Some(',') | None => {}
                _ => selector.push_str(","),
            }
            selector.push_str(ident);
        }
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        Hash,
        PartialOrd,
        Ord,
        Eq,
        Default,
        :: serde :: Deserialize,
        :: serde :: Serialize,
    )]
    pub struct StreetSegmentList {
        #[serde(rename = "segments", default)]
        pub segments: ::std::option::Option<Vec<crate::schemas::StreetSegment>>,
    }
    impl ::field_selector::FieldSelector for StreetSegmentList {
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
    pub struct VoterInfoRequest {
        #[serde(rename = "contextParams", default)]
        pub context_params: ::std::option::Option<crate::schemas::ContextParams>,
        #[serde(rename = "voterInfoSegmentResult", default)]
        pub voter_info_segment_result:
            ::std::option::Option<Box<crate::schemas::VoterInfoSegmentResult>>,
    }
    impl ::field_selector::FieldSelector for VoterInfoRequest {
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
    pub struct VoterInfoResponse {
        #[doc = "Contests that will appear on the voter's ballot."]
        #[serde(rename = "contests", default)]
        pub contests: ::std::option::Option<Vec<crate::schemas::Contest>>,
        #[doc = "Locations where a voter is eligible to drop off a completed ballot. The voter must have received and completed a ballot prior to arriving at the location. The location may not have ballots available on the premises. These locations could be open on or before election day as indicated in the pollingHours field."]
        #[serde(rename = "dropOffLocations", default)]
        pub drop_off_locations: ::std::option::Option<Vec<crate::schemas::PollingLocation>>,
        #[doc = "Locations where the voter is eligible to vote early, prior to election day."]
        #[serde(rename = "earlyVoteSites", default)]
        pub early_vote_sites: ::std::option::Option<Vec<crate::schemas::PollingLocation>>,
        #[doc = "The election that was queried."]
        #[serde(rename = "election", default)]
        pub election: ::std::option::Option<crate::schemas::Election>,
        #[doc = "Identifies what kind of resource this is. Value: the fixed string \"civicinfo#voterInfoResponse\"."]
        #[serde(rename = "kind", default)]
        pub kind: ::std::option::Option<String>,
        #[doc = "Specifies whether voters in the precinct vote only by mailing their ballots (with the possible option of dropping off their ballots as well)."]
        #[serde(rename = "mailOnly", default)]
        pub mail_only: ::std::option::Option<bool>,
        #[doc = "The normalized version of the requested address"]
        #[serde(rename = "normalizedInput", default)]
        pub normalized_input: ::std::option::Option<crate::schemas::SimpleAddressType>,
        #[doc = "When there are multiple elections for a voter address, the otherElections field is populated in the API response and there are two possibilities: 1. If the earliest election is not the intended election, specify the election ID of the desired election in a second API request using the electionId field. 2. If these elections occur on the same day, the API doesn?t return any polling location, contest, or election official information to ensure that an additional query is made. For user-facing applications, we recommend displaying these elections to the user to disambiguate. A second API request using the electionId field should be made for the election that is relevant to the user."]
        #[serde(rename = "otherElections", default)]
        pub other_elections: ::std::option::Option<Vec<crate::schemas::Election>>,
        #[doc = "Locations where the voter is eligible to vote on election day."]
        #[serde(rename = "pollingLocations", default)]
        pub polling_locations: ::std::option::Option<Vec<crate::schemas::PollingLocation>>,
        #[serde(rename = "precinctId", default)]
        pub precinct_id: ::std::option::Option<String>,
        #[serde(rename = "segments", default)]
        pub segments: ::std::option::Option<Vec<crate::schemas::StreetSegment>>,
        #[doc = "Local Election Information for the state that the voter votes in. For the US, there will only be one element in this array."]
        #[serde(rename = "state", default)]
        pub state: ::std::option::Option<Vec<crate::schemas::AdministrationRegion>>,
    }
    impl ::field_selector::FieldSelector for VoterInfoResponse {
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
    pub struct VoterInfoSegmentResult {
        #[serde(rename = "generatedMillis", default)]
        #[serde(with = "crate::parsed_string")]
        pub generated_millis: ::std::option::Option<i64>,
        #[serde(rename = "postalAddress", default)]
        pub postal_address: ::std::option::Option<crate::schemas::PostalAddress>,
        #[serde(rename = "request", default)]
        pub request: ::std::option::Option<Box<crate::schemas::VoterInfoRequest>>,
        #[serde(rename = "response", default)]
        pub response: ::std::option::Option<crate::schemas::VoterInfoResponse>,
    }
    impl ::field_selector::FieldSelector for VoterInfoSegmentResult {
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
    #[doc = "Actions that can be performed on the divisions resource"]
    pub fn divisions(&self) -> crate::resources::divisions::DivisionsActions<A> {
        crate::resources::divisions::DivisionsActions {
            reqwest: &self.reqwest,
            auth: &self.auth,
        }
    }
    #[doc = "Actions that can be performed on the elections resource"]
    pub fn elections(&self) -> crate::resources::elections::ElectionsActions<A> {
        crate::resources::elections::ElectionsActions {
            reqwest: &self.reqwest,
            auth: &self.auth,
        }
    }
    #[doc = "Actions that can be performed on the representatives resource"]
    pub fn representatives(&self) -> crate::resources::representatives::RepresentativesActions<A> {
        crate::resources::representatives::RepresentativesActions {
            reqwest: &self.reqwest,
            auth: &self.auth,
        }
    }
}
pub mod resources {
    pub mod divisions {
        pub mod params {}
        pub struct DivisionsActions<'a, A> {
            pub(crate) reqwest: &'a reqwest::Client,
            pub(crate) auth: &'a std::sync::Mutex<A>,
        }
        impl<'a, A: yup_oauth2::GetToken> DivisionsActions<'a, A> {
            #[doc = "Searches for political divisions by their natural name or OCD ID."]
            pub fn search(
                &self,
                request: crate::schemas::DivisionSearchRequest,
            ) -> SearchRequestBuilder<A> {
                SearchRequestBuilder {
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
                    query: None,
                }
            }
        }
        #[derive(Debug, Clone)]
        pub struct SearchRequestBuilder<'a, A> {
            pub(crate) reqwest: &'a ::reqwest::Client,
            pub(crate) auth: &'a ::std::sync::Mutex<A>,
            request: crate::schemas::DivisionSearchRequest,
            query: Option<String>,
            alt: Option<crate::params::Alt>,
            fields: Option<String>,
            key: Option<String>,
            oauth_token: Option<String>,
            pretty_print: Option<bool>,
            quota_user: Option<String>,
            user_ip: Option<String>,
        }
        impl<'a, A: yup_oauth2::GetToken> SearchRequestBuilder<'a, A> {
            #[doc = "The search query. Queries can cover any parts of a OCD ID or a human readable division name. All words given in the query are treated as required patterns. In addition to that, most query operators of the Apache Lucene library are supported. See http://lucene.apache.org/core/2_9_4/queryparsersyntax.html"]
            pub fn query(mut self, value: impl Into<String>) -> Self {
                self.query = Some(value.into());
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
            ) -> Result<crate::schemas::DivisionSearchResponse, Box<dyn ::std::error::Error>>
            {
                self.execute_fields::<_, &str>(None)
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub fn execute_debug(
                self,
            ) -> Result<crate::schemas::DivisionSearchResponse, Box<dyn ::std::error::Error>>
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
                let mut output = "https://www.googleapis.com/civicinfo/v2/".to_owned();
                output.push_str("divisions");
                output
            }
            fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
                let req = self.reqwest.request(::reqwest::Method::GET, path);
                let req = req.query(&[("query", &self.query)]);
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
    pub mod elections {
        pub mod params {}
        pub struct ElectionsActions<'a, A> {
            pub(crate) reqwest: &'a reqwest::Client,
            pub(crate) auth: &'a std::sync::Mutex<A>,
        }
        impl<'a, A: yup_oauth2::GetToken> ElectionsActions<'a, A> {
            #[doc = "List of available elections to query."]
            pub fn election_query(
                &self,
                request: crate::schemas::ElectionsQueryRequest,
            ) -> ElectionQueryRequestBuilder<A> {
                ElectionQueryRequestBuilder {
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
            #[doc = "Looks up information relevant to a voter based on the voter's registered address."]
            pub fn voter_info_query(
                &self,
                request: crate::schemas::VoterInfoRequest,
                address: impl Into<String>,
            ) -> VoterInfoQueryRequestBuilder<A> {
                VoterInfoQueryRequestBuilder {
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
                    address: address.into(),
                    election_id: None,
                    official_only: None,
                    return_all_available_data: None,
                }
            }
        }
        #[derive(Debug, Clone)]
        pub struct ElectionQueryRequestBuilder<'a, A> {
            pub(crate) reqwest: &'a ::reqwest::Client,
            pub(crate) auth: &'a ::std::sync::Mutex<A>,
            request: crate::schemas::ElectionsQueryRequest,
            alt: Option<crate::params::Alt>,
            fields: Option<String>,
            key: Option<String>,
            oauth_token: Option<String>,
            pretty_print: Option<bool>,
            quota_user: Option<String>,
            user_ip: Option<String>,
        }
        impl<'a, A: yup_oauth2::GetToken> ElectionQueryRequestBuilder<'a, A> {
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
            ) -> Result<crate::schemas::ElectionsQueryResponse, Box<dyn ::std::error::Error>>
            {
                self.execute_fields::<_, &str>(None)
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub fn execute_debug(
                self,
            ) -> Result<crate::schemas::ElectionsQueryResponse, Box<dyn ::std::error::Error>>
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
                let mut output = "https://www.googleapis.com/civicinfo/v2/".to_owned();
                output.push_str("elections");
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
        pub struct VoterInfoQueryRequestBuilder<'a, A> {
            pub(crate) reqwest: &'a ::reqwest::Client,
            pub(crate) auth: &'a ::std::sync::Mutex<A>,
            request: crate::schemas::VoterInfoRequest,
            address: String,
            election_id: Option<i64>,
            official_only: Option<bool>,
            return_all_available_data: Option<bool>,
            alt: Option<crate::params::Alt>,
            fields: Option<String>,
            key: Option<String>,
            oauth_token: Option<String>,
            pretty_print: Option<bool>,
            quota_user: Option<String>,
            user_ip: Option<String>,
        }
        impl<'a, A: yup_oauth2::GetToken> VoterInfoQueryRequestBuilder<'a, A> {
            #[doc = "The unique ID of the election to look up. A list of election IDs can be obtained at https://www.googleapis.com/civicinfo/{version}/electionsIf no election ID is specified in the query and there is more than one election with data for the given voter, the additional elections are provided in the otherElections response field."]
            pub fn election_id(mut self, value: i64) -> Self {
                self.election_id = Some(value);
                self
            }
            #[doc = "If set to true, only data from official state sources will be returned."]
            pub fn official_only(mut self, value: bool) -> Self {
                self.official_only = Some(value);
                self
            }
            #[doc = "If set to true, the query will return the success codeand include any partial information when it is unable to determine a matching address or unable to determine the election for electionId=0 queries."]
            pub fn return_all_available_data(mut self, value: bool) -> Self {
                self.return_all_available_data = Some(value);
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
            ) -> Result<crate::schemas::VoterInfoResponse, Box<dyn ::std::error::Error>>
            {
                self.execute_fields::<_, &str>(None)
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub fn execute_debug(
                self,
            ) -> Result<crate::schemas::VoterInfoResponse, Box<dyn ::std::error::Error>>
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
                let mut output = "https://www.googleapis.com/civicinfo/v2/".to_owned();
                output.push_str("voterinfo");
                output
            }
            fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
                let req = self.reqwest.request(::reqwest::Method::GET, path);
                let req = req.query(&[("address", &self.address)]);
                let req = req.query(&[("electionId", &self.election_id)]);
                let req = req.query(&[("officialOnly", &self.official_only)]);
                let req = req.query(&[("returnAllAvailableData", &self.return_all_available_data)]);
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
    pub mod representatives {
        pub mod params {
            #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
            pub enum RepresentativeInfoByAddressLevelsItems {
                AdministrativeArea1,
                AdministrativeArea2,
                Country,
                International,
                Locality,
                Regional,
                Special,
                SubLocality1,
                SubLocality2,
            }
            impl RepresentativeInfoByAddressLevelsItems {
                pub fn as_str(self) -> &'static str {
                    match self {
                        RepresentativeInfoByAddressLevelsItems::AdministrativeArea1 => {
                            "administrativeArea1"
                        }
                        RepresentativeInfoByAddressLevelsItems::AdministrativeArea2 => {
                            "administrativeArea2"
                        }
                        RepresentativeInfoByAddressLevelsItems::Country => "country",
                        RepresentativeInfoByAddressLevelsItems::International => "international",
                        RepresentativeInfoByAddressLevelsItems::Locality => "locality",
                        RepresentativeInfoByAddressLevelsItems::Regional => "regional",
                        RepresentativeInfoByAddressLevelsItems::Special => "special",
                        RepresentativeInfoByAddressLevelsItems::SubLocality1 => "subLocality1",
                        RepresentativeInfoByAddressLevelsItems::SubLocality2 => "subLocality2",
                    }
                }
            }
            impl ::std::fmt::Display for RepresentativeInfoByAddressLevelsItems {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                    f.write_str(self.as_str())
                }
            }
            impl ::serde::Serialize for RepresentativeInfoByAddressLevelsItems {
                fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
                where
                    S: ::serde::ser::Serializer,
                {
                    serializer.serialize_str(self.as_str())
                }
            }
            impl<'de> ::serde::Deserialize<'de> for RepresentativeInfoByAddressLevelsItems {
                fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
                where
                    D: ::serde::de::Deserializer<'de>,
                {
                    let value: &'de str = <&str>::deserialize(deserializer)?;
                    Ok(match value {
                        "administrativeArea1" => {
                            RepresentativeInfoByAddressLevelsItems::AdministrativeArea1
                        }
                        "administrativeArea2" => {
                            RepresentativeInfoByAddressLevelsItems::AdministrativeArea2
                        }
                        "country" => RepresentativeInfoByAddressLevelsItems::Country,
                        "international" => RepresentativeInfoByAddressLevelsItems::International,
                        "locality" => RepresentativeInfoByAddressLevelsItems::Locality,
                        "regional" => RepresentativeInfoByAddressLevelsItems::Regional,
                        "special" => RepresentativeInfoByAddressLevelsItems::Special,
                        "subLocality1" => RepresentativeInfoByAddressLevelsItems::SubLocality1,
                        "subLocality2" => RepresentativeInfoByAddressLevelsItems::SubLocality2,
                        _ => {
                            return Err(::serde::de::Error::custom(format!(
                                "invalid enum for #name: {}",
                                value
                            )))
                        }
                    })
                }
            }
            impl ::field_selector::FieldSelector for RepresentativeInfoByAddressLevelsItems {
                fn field_selector_with_ident(ident: &str, selector: &mut String) {
                    match selector.chars().rev().nth(0) {
                        Some(',') | None => {}
                        _ => selector.push_str(","),
                    }
                    selector.push_str(ident);
                }
            }
            #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
            pub enum RepresentativeInfoByAddressRolesItems {
                DeputyHeadOfGovernment,
                ExecutiveCouncil,
                GovernmentOfficer,
                HeadOfGovernment,
                HeadOfState,
                HighestCourtJudge,
                Judge,
                LegislatorLowerBody,
                LegislatorUpperBody,
                SchoolBoard,
                SpecialPurposeOfficer,
            }
            impl RepresentativeInfoByAddressRolesItems {
                pub fn as_str(self) -> &'static str {
                    match self {
                        RepresentativeInfoByAddressRolesItems::DeputyHeadOfGovernment => {
                            "deputyHeadOfGovernment"
                        }
                        RepresentativeInfoByAddressRolesItems::ExecutiveCouncil => {
                            "executiveCouncil"
                        }
                        RepresentativeInfoByAddressRolesItems::GovernmentOfficer => {
                            "governmentOfficer"
                        }
                        RepresentativeInfoByAddressRolesItems::HeadOfGovernment => {
                            "headOfGovernment"
                        }
                        RepresentativeInfoByAddressRolesItems::HeadOfState => "headOfState",
                        RepresentativeInfoByAddressRolesItems::HighestCourtJudge => {
                            "highestCourtJudge"
                        }
                        RepresentativeInfoByAddressRolesItems::Judge => "judge",
                        RepresentativeInfoByAddressRolesItems::LegislatorLowerBody => {
                            "legislatorLowerBody"
                        }
                        RepresentativeInfoByAddressRolesItems::LegislatorUpperBody => {
                            "legislatorUpperBody"
                        }
                        RepresentativeInfoByAddressRolesItems::SchoolBoard => "schoolBoard",
                        RepresentativeInfoByAddressRolesItems::SpecialPurposeOfficer => {
                            "specialPurposeOfficer"
                        }
                    }
                }
            }
            impl ::std::fmt::Display for RepresentativeInfoByAddressRolesItems {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                    f.write_str(self.as_str())
                }
            }
            impl ::serde::Serialize for RepresentativeInfoByAddressRolesItems {
                fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
                where
                    S: ::serde::ser::Serializer,
                {
                    serializer.serialize_str(self.as_str())
                }
            }
            impl<'de> ::serde::Deserialize<'de> for RepresentativeInfoByAddressRolesItems {
                fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
                where
                    D: ::serde::de::Deserializer<'de>,
                {
                    let value: &'de str = <&str>::deserialize(deserializer)?;
                    Ok(match value {
                        "deputyHeadOfGovernment" => {
                            RepresentativeInfoByAddressRolesItems::DeputyHeadOfGovernment
                        }
                        "executiveCouncil" => {
                            RepresentativeInfoByAddressRolesItems::ExecutiveCouncil
                        }
                        "governmentOfficer" => {
                            RepresentativeInfoByAddressRolesItems::GovernmentOfficer
                        }
                        "headOfGovernment" => {
                            RepresentativeInfoByAddressRolesItems::HeadOfGovernment
                        }
                        "headOfState" => RepresentativeInfoByAddressRolesItems::HeadOfState,
                        "highestCourtJudge" => {
                            RepresentativeInfoByAddressRolesItems::HighestCourtJudge
                        }
                        "judge" => RepresentativeInfoByAddressRolesItems::Judge,
                        "legislatorLowerBody" => {
                            RepresentativeInfoByAddressRolesItems::LegislatorLowerBody
                        }
                        "legislatorUpperBody" => {
                            RepresentativeInfoByAddressRolesItems::LegislatorUpperBody
                        }
                        "schoolBoard" => RepresentativeInfoByAddressRolesItems::SchoolBoard,
                        "specialPurposeOfficer" => {
                            RepresentativeInfoByAddressRolesItems::SpecialPurposeOfficer
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
            impl ::field_selector::FieldSelector for RepresentativeInfoByAddressRolesItems {
                fn field_selector_with_ident(ident: &str, selector: &mut String) {
                    match selector.chars().rev().nth(0) {
                        Some(',') | None => {}
                        _ => selector.push_str(","),
                    }
                    selector.push_str(ident);
                }
            }
            #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
            pub enum RepresentativeInfoByDivisionLevelsItems {
                AdministrativeArea1,
                AdministrativeArea2,
                Country,
                International,
                Locality,
                Regional,
                Special,
                SubLocality1,
                SubLocality2,
            }
            impl RepresentativeInfoByDivisionLevelsItems {
                pub fn as_str(self) -> &'static str {
                    match self {
                        RepresentativeInfoByDivisionLevelsItems::AdministrativeArea1 => {
                            "administrativeArea1"
                        }
                        RepresentativeInfoByDivisionLevelsItems::AdministrativeArea2 => {
                            "administrativeArea2"
                        }
                        RepresentativeInfoByDivisionLevelsItems::Country => "country",
                        RepresentativeInfoByDivisionLevelsItems::International => "international",
                        RepresentativeInfoByDivisionLevelsItems::Locality => "locality",
                        RepresentativeInfoByDivisionLevelsItems::Regional => "regional",
                        RepresentativeInfoByDivisionLevelsItems::Special => "special",
                        RepresentativeInfoByDivisionLevelsItems::SubLocality1 => "subLocality1",
                        RepresentativeInfoByDivisionLevelsItems::SubLocality2 => "subLocality2",
                    }
                }
            }
            impl ::std::fmt::Display for RepresentativeInfoByDivisionLevelsItems {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                    f.write_str(self.as_str())
                }
            }
            impl ::serde::Serialize for RepresentativeInfoByDivisionLevelsItems {
                fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
                where
                    S: ::serde::ser::Serializer,
                {
                    serializer.serialize_str(self.as_str())
                }
            }
            impl<'de> ::serde::Deserialize<'de> for RepresentativeInfoByDivisionLevelsItems {
                fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
                where
                    D: ::serde::de::Deserializer<'de>,
                {
                    let value: &'de str = <&str>::deserialize(deserializer)?;
                    Ok(match value {
                        "administrativeArea1" => {
                            RepresentativeInfoByDivisionLevelsItems::AdministrativeArea1
                        }
                        "administrativeArea2" => {
                            RepresentativeInfoByDivisionLevelsItems::AdministrativeArea2
                        }
                        "country" => RepresentativeInfoByDivisionLevelsItems::Country,
                        "international" => RepresentativeInfoByDivisionLevelsItems::International,
                        "locality" => RepresentativeInfoByDivisionLevelsItems::Locality,
                        "regional" => RepresentativeInfoByDivisionLevelsItems::Regional,
                        "special" => RepresentativeInfoByDivisionLevelsItems::Special,
                        "subLocality1" => RepresentativeInfoByDivisionLevelsItems::SubLocality1,
                        "subLocality2" => RepresentativeInfoByDivisionLevelsItems::SubLocality2,
                        _ => {
                            return Err(::serde::de::Error::custom(format!(
                                "invalid enum for #name: {}",
                                value
                            )))
                        }
                    })
                }
            }
            impl ::field_selector::FieldSelector for RepresentativeInfoByDivisionLevelsItems {
                fn field_selector_with_ident(ident: &str, selector: &mut String) {
                    match selector.chars().rev().nth(0) {
                        Some(',') | None => {}
                        _ => selector.push_str(","),
                    }
                    selector.push_str(ident);
                }
            }
            #[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Ord, Eq, Copy)]
            pub enum RepresentativeInfoByDivisionRolesItems {
                DeputyHeadOfGovernment,
                ExecutiveCouncil,
                GovernmentOfficer,
                HeadOfGovernment,
                HeadOfState,
                HighestCourtJudge,
                Judge,
                LegislatorLowerBody,
                LegislatorUpperBody,
                SchoolBoard,
                SpecialPurposeOfficer,
            }
            impl RepresentativeInfoByDivisionRolesItems {
                pub fn as_str(self) -> &'static str {
                    match self {
                        RepresentativeInfoByDivisionRolesItems::DeputyHeadOfGovernment => {
                            "deputyHeadOfGovernment"
                        }
                        RepresentativeInfoByDivisionRolesItems::ExecutiveCouncil => {
                            "executiveCouncil"
                        }
                        RepresentativeInfoByDivisionRolesItems::GovernmentOfficer => {
                            "governmentOfficer"
                        }
                        RepresentativeInfoByDivisionRolesItems::HeadOfGovernment => {
                            "headOfGovernment"
                        }
                        RepresentativeInfoByDivisionRolesItems::HeadOfState => "headOfState",
                        RepresentativeInfoByDivisionRolesItems::HighestCourtJudge => {
                            "highestCourtJudge"
                        }
                        RepresentativeInfoByDivisionRolesItems::Judge => "judge",
                        RepresentativeInfoByDivisionRolesItems::LegislatorLowerBody => {
                            "legislatorLowerBody"
                        }
                        RepresentativeInfoByDivisionRolesItems::LegislatorUpperBody => {
                            "legislatorUpperBody"
                        }
                        RepresentativeInfoByDivisionRolesItems::SchoolBoard => "schoolBoard",
                        RepresentativeInfoByDivisionRolesItems::SpecialPurposeOfficer => {
                            "specialPurposeOfficer"
                        }
                    }
                }
            }
            impl ::std::fmt::Display for RepresentativeInfoByDivisionRolesItems {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                    f.write_str(self.as_str())
                }
            }
            impl ::serde::Serialize for RepresentativeInfoByDivisionRolesItems {
                fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
                where
                    S: ::serde::ser::Serializer,
                {
                    serializer.serialize_str(self.as_str())
                }
            }
            impl<'de> ::serde::Deserialize<'de> for RepresentativeInfoByDivisionRolesItems {
                fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
                where
                    D: ::serde::de::Deserializer<'de>,
                {
                    let value: &'de str = <&str>::deserialize(deserializer)?;
                    Ok(match value {
                        "deputyHeadOfGovernment" => {
                            RepresentativeInfoByDivisionRolesItems::DeputyHeadOfGovernment
                        }
                        "executiveCouncil" => {
                            RepresentativeInfoByDivisionRolesItems::ExecutiveCouncil
                        }
                        "governmentOfficer" => {
                            RepresentativeInfoByDivisionRolesItems::GovernmentOfficer
                        }
                        "headOfGovernment" => {
                            RepresentativeInfoByDivisionRolesItems::HeadOfGovernment
                        }
                        "headOfState" => RepresentativeInfoByDivisionRolesItems::HeadOfState,
                        "highestCourtJudge" => {
                            RepresentativeInfoByDivisionRolesItems::HighestCourtJudge
                        }
                        "judge" => RepresentativeInfoByDivisionRolesItems::Judge,
                        "legislatorLowerBody" => {
                            RepresentativeInfoByDivisionRolesItems::LegislatorLowerBody
                        }
                        "legislatorUpperBody" => {
                            RepresentativeInfoByDivisionRolesItems::LegislatorUpperBody
                        }
                        "schoolBoard" => RepresentativeInfoByDivisionRolesItems::SchoolBoard,
                        "specialPurposeOfficer" => {
                            RepresentativeInfoByDivisionRolesItems::SpecialPurposeOfficer
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
            impl ::field_selector::FieldSelector for RepresentativeInfoByDivisionRolesItems {
                fn field_selector_with_ident(ident: &str, selector: &mut String) {
                    match selector.chars().rev().nth(0) {
                        Some(',') | None => {}
                        _ => selector.push_str(","),
                    }
                    selector.push_str(ident);
                }
            }
        }
        pub struct RepresentativesActions<'a, A> {
            pub(crate) reqwest: &'a reqwest::Client,
            pub(crate) auth: &'a std::sync::Mutex<A>,
        }
        impl<'a, A: yup_oauth2::GetToken> RepresentativesActions<'a, A> {
            #[doc = "Looks up political geography and representative information for a single address."]
            pub fn representative_info_by_address(
                &self,
                request: crate::schemas::RepresentativeInfoRequest,
            ) -> RepresentativeInfoByAddressRequestBuilder<A> {
                RepresentativeInfoByAddressRequestBuilder {
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
                    address: None,
                    include_offices: None,
                    levels: None,
                    roles: None,
                }
            }
            #[doc = "Looks up representative information for a single geographic division."]
            pub fn representative_info_by_division(
                &self,
                request: crate::schemas::DivisionRepresentativeInfoRequest,
                ocd_id: impl Into<String>,
            ) -> RepresentativeInfoByDivisionRequestBuilder<A> {
                RepresentativeInfoByDivisionRequestBuilder {
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
                    ocd_id: ocd_id.into(),
                    levels: None,
                    recursive: None,
                    roles: None,
                }
            }
        }
        #[derive(Debug, Clone)]
        pub struct RepresentativeInfoByAddressRequestBuilder < 'a , A > { pub ( crate ) reqwest : & 'a :: reqwest :: Client , pub ( crate ) auth : & 'a :: std :: sync :: Mutex < A > , request : crate :: schemas :: RepresentativeInfoRequest , address : Option < String > , include_offices : Option < bool > , levels : Option < Vec < crate :: resources :: representatives :: params :: RepresentativeInfoByAddressLevelsItems > > , roles : Option < Vec < crate :: resources :: representatives :: params :: RepresentativeInfoByAddressRolesItems > > , alt : Option < crate :: params :: Alt > , fields : Option < String > , key : Option < String > , oauth_token : Option < String > , pretty_print : Option < bool > , quota_user : Option < String > , user_ip : Option < String > , }
        impl<'a, A: yup_oauth2::GetToken> RepresentativeInfoByAddressRequestBuilder<'a, A> {
            #[doc = "The address to look up. May only be specified if the field ocdId is not given in the URL."]
            pub fn address(mut self, value: impl Into<String>) -> Self {
                self.address = Some(value.into());
                self
            }
            #[doc = "Whether to return information about offices and officials. If false, only the top-level district information will be returned."]
            pub fn include_offices(mut self, value: bool) -> Self {
                self.include_offices = Some(value);
                self
            }
            #[doc = "A list of office levels to filter by. Only offices that serve at least one of these levels will be returned. Divisions that don't contain a matching office will not be returned."]
            pub fn levels(
                mut self,
                value : impl Into < Vec < crate :: resources :: representatives :: params :: RepresentativeInfoByAddressLevelsItems > >,
            ) -> Self {
                self.levels = Some(value.into());
                self
            }
            #[doc = "A list of office roles to filter by. Only offices fulfilling one of these roles will be returned. Divisions that don't contain a matching office will not be returned."]
            pub fn roles(
                mut self,
                value : impl Into < Vec < crate :: resources :: representatives :: params :: RepresentativeInfoByAddressRolesItems > >,
            ) -> Self {
                self.roles = Some(value.into());
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
            ) -> Result<crate::schemas::RepresentativeInfoResponse, Box<dyn ::std::error::Error>>
            {
                self.execute_fields::<_, &str>(None)
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub fn execute_debug(
                self,
            ) -> Result<crate::schemas::RepresentativeInfoResponse, Box<dyn ::std::error::Error>>
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
                let mut output = "https://www.googleapis.com/civicinfo/v2/".to_owned();
                output.push_str("representatives");
                output
            }
            fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
                let req = self.reqwest.request(::reqwest::Method::GET, path);
                let req = req.query(&[("address", &self.address)]);
                let req = req.query(&[("includeOffices", &self.include_offices)]);
                let req = req.query(&[("levels", &self.levels)]);
                let req = req.query(&[("roles", &self.roles)]);
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
        pub struct RepresentativeInfoByDivisionRequestBuilder < 'a , A > { pub ( crate ) reqwest : & 'a :: reqwest :: Client , pub ( crate ) auth : & 'a :: std :: sync :: Mutex < A > , request : crate :: schemas :: DivisionRepresentativeInfoRequest , ocd_id : String , levels : Option < Vec < crate :: resources :: representatives :: params :: RepresentativeInfoByDivisionLevelsItems > > , recursive : Option < bool > , roles : Option < Vec < crate :: resources :: representatives :: params :: RepresentativeInfoByDivisionRolesItems > > , alt : Option < crate :: params :: Alt > , fields : Option < String > , key : Option < String > , oauth_token : Option < String > , pretty_print : Option < bool > , quota_user : Option < String > , user_ip : Option < String > , }
        impl<'a, A: yup_oauth2::GetToken> RepresentativeInfoByDivisionRequestBuilder<'a, A> {
            #[doc = "A list of office levels to filter by. Only offices that serve at least one of these levels will be returned. Divisions that don't contain a matching office will not be returned."]
            pub fn levels(
                mut self,
                value : impl Into < Vec < crate :: resources :: representatives :: params :: RepresentativeInfoByDivisionLevelsItems > >,
            ) -> Self {
                self.levels = Some(value.into());
                self
            }
            #[doc = "If true, information about all divisions contained in the division requested will be included as well. For example, if querying ocd-division/country:us/district:dc, this would also return all DC's wards and ANCs."]
            pub fn recursive(mut self, value: bool) -> Self {
                self.recursive = Some(value);
                self
            }
            #[doc = "A list of office roles to filter by. Only offices fulfilling one of these roles will be returned. Divisions that don't contain a matching office will not be returned."]
            pub fn roles(
                mut self,
                value : impl Into < Vec < crate :: resources :: representatives :: params :: RepresentativeInfoByDivisionRolesItems > >,
            ) -> Self {
                self.roles = Some(value.into());
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
            ) -> Result<crate::schemas::RepresentativeInfoData, Box<dyn ::std::error::Error>>
            {
                self.execute_fields::<_, &str>(None)
            }
            #[doc = r" Execute the given operation. This will provide a `fields`"]
            #[doc = r" selector of `*`. This will include every attribute of the"]
            #[doc = r" response resource and should be limited to use during"]
            #[doc = r" development or debugging."]
            pub fn execute_debug(
                self,
            ) -> Result<crate::schemas::RepresentativeInfoData, Box<dyn ::std::error::Error>>
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
                let mut output = "https://www.googleapis.com/civicinfo/v2/".to_owned();
                output.push_str("representatives/");
                {
                    let var_as_str = &self.ocd_id;
                    output.extend(::percent_encoding::utf8_percent_encode(
                        &var_as_str,
                        crate::SIMPLE,
                    ));
                }
                output
            }
            fn _request(&self, path: &str) -> ::reqwest::RequestBuilder {
                let req = self.reqwest.request(::reqwest::Method::GET, path);
                let req = req.query(&[("levels", &self.levels)]);
                let req = req.query(&[("recursive", &self.recursive)]);
                let req = req.query(&[("roles", &self.roles)]);
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
