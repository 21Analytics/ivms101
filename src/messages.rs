use crate::types;

#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct IVMS101 {
    #[serde(skip_serializing_if = "Option::is_none")]
    originator: Option<Originator>,
    #[serde(skip_serializing_if = "Option::is_none")]
    beneficiary: Option<Beneficiary>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "originatingVASP")]
    originating_vasp: Option<OriginatingVASP>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "beneficiaryVASP")]
    beneficiary_vasp: Option<BeneficiaryVASP>,
}

impl Validatable for IVMS101 {
    fn validate(&self) -> Result<(), ValidationError> {
        if let Some(o) = &self.originator {
            o.validate()?;
        }
        if let Some(b) = &self.beneficiary {
            b.validate()?;
        }
        if let Some(ov) = &self.originating_vasp {
            ov.validate()?;
        }
        if let Some(bv) = &self.beneficiary_vasp {
            bv.validate()?;
        }
        Ok(())
    }
}

#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct Originator {
    originator_persons: types::OneToN<Person>,
    #[serde(default, skip_serializing_if = "types::ZeroToN::is_empty")]
    account_number: types::ZeroToN<types::StringMax100>,
}

impl Validatable for Originator {
    fn validate(&self) -> Result<(), ValidationError> {
        for person in self.originator_persons.clone() {
            if let Person::NaturalPerson(np) = &person {
                if np.geographic_address.is_empty()
                    && np.customer_identification.is_none()
                    && np.national_identification.is_none()
                    && np.date_and_place_of_birth.is_none()
                {
                    return Err(
                        "Natural person: one of 1) geographic address 2) customer id 3) national id 4) date and place of birth is required (IVMS101 C1)".into());
                }
            };
            person.validate()?;
        }
        Ok(())
    }
}

impl Originator {
    pub fn new(person: Person) -> Result<Self, ValidationError> {
        Ok(Self {
            originator_persons: person.into(),
            account_number: None.into(),
        })
    }

    #[must_use]
    pub fn first_name(&self) -> Option<String> {
        self.originator_persons.first().first_name()
    }

    #[must_use]
    pub fn last_name(&self) -> String {
        self.originator_persons.first().last_name()
    }

    #[must_use]
    pub fn address(&self) -> Option<&Address> {
        self.originator_persons.first().address()
    }

    #[must_use]
    pub fn customer_identification(&self) -> Option<String> {
        self.originator_persons.first().customer_identification()
    }

    #[must_use]
    pub fn originator_person(&self) -> &Person {
        self.originator_persons.first()
    }
}

#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct Beneficiary {
    beneficiary_persons: types::OneToN<Person>,
    #[serde(default, skip_serializing_if = "types::ZeroToN::is_empty")]
    account_number: types::ZeroToN<types::StringMax100>,
}

impl Validatable for Beneficiary {
    fn validate(&self) -> Result<(), ValidationError> {
        for person in self.beneficiary_persons.clone() {
            person.validate()?;
        }
        Ok(())
    }
}

impl Beneficiary {
    pub fn new(person: Person, account_number: Option<&str>) -> Result<Self, ValidationError> {
        Ok(Self {
            beneficiary_persons: person.into(),
            account_number: account_number.map(TryInto::try_into).transpose()?.into(),
        })
    }

    #[must_use]
    pub fn first_name(&self) -> Option<String> {
        self.beneficiary_persons.first().first_name()
    }

    #[must_use]
    pub fn last_name(&self) -> String {
        self.beneficiary_persons.first().last_name()
    }
}

#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct OriginatingVASP {
    #[serde(rename = "originatingVASP")]
    originating_vasp: Person,
}

impl OriginatingVASP {
    pub fn new(name: &str, lei: &lei::LEI) -> Result<Self, ValidationError> {
        Ok(Self {
            originating_vasp: Person::LegalPerson(LegalPerson {
                name: LegalPersonName {
                    name_identifier: LegalPersonNameID {
                        legal_person_name: name.try_into()?,
                        legal_person_name_identifier_type: LegalPersonNameTypeCode::Legl,
                    }
                    .into(),
                    local_name_identifier: None.into(),
                    phonetic_name_identifier: None.into(),
                },
                geographic_address: types::ZeroToN::None,
                customer_identification: None,
                national_identification: Some(NationalIdentification {
                    national_identifier: lei.to_string().as_str().try_into()?,
                    national_identifier_type: NationalIdentifierTypeCode::Leix,
                    country_of_issue: None,
                    registration_authority: None,
                }),
                country_of_registration: None,
            }),
        })
    }

    pub fn lei(&self) -> Result<Option<lei::LEI>, lei::Error> {
        self.originating_vasp.lei()
    }
}

impl Validatable for OriginatingVASP {
    fn validate(&self) -> Result<(), ValidationError> {
        self.originating_vasp.validate()
    }
}

#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
struct BeneficiaryVASP {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "beneficiaryVASP")]
    beneficiary_vasp: Option<Person>,
}

impl Validatable for BeneficiaryVASP {
    fn validate(&self) -> Result<(), ValidationError> {
        match &self.beneficiary_vasp {
            None => Ok(()),
            Some(p) => p.validate(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub enum Person {
    NaturalPerson(NaturalPerson),
    LegalPerson(LegalPerson),
}

impl Person {
    #[must_use]
    fn first_name(&self) -> Option<String> {
        match self {
            Self::NaturalPerson(p) => p.first_name(),
            Self::LegalPerson(_p) => None,
        }
    }
    #[must_use]
    fn last_name(&self) -> String {
        match self {
            Self::NaturalPerson(p) => p.last_name(),
            Self::LegalPerson(p) => p.name(),
        }
    }

    #[must_use]
    fn address(&self) -> Option<&Address> {
        match self {
            Self::NaturalPerson(p) => p.address(),
            Self::LegalPerson(p) => p.address(),
        }
    }

    #[must_use]
    fn customer_identification(&self) -> Option<String> {
        match self {
            Self::NaturalPerson(p) => p.customer_identification.clone().map(|s| s.to_string()),
            Self::LegalPerson(p) => p.customer_identification.clone().map(|s| s.to_string()),
        }
    }

    pub fn lei(&self) -> Result<Option<lei::LEI>, lei::Error> {
        match self {
            Self::NaturalPerson(_) => Ok(None),
            Self::LegalPerson(l) => l.lei(),
        }
    }
}

impl Validatable for Person {
    fn validate(&self) -> Result<(), ValidationError> {
        match self {
            Person::NaturalPerson(p) => p.validate(),
            Person::LegalPerson(p) => p.validate(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct NaturalPerson {
    name: types::OneToN<NaturalPersonName>,
    #[serde(default, skip_serializing_if = "types::ZeroToN::is_empty")]
    geographic_address: types::ZeroToN<Address>,
    #[serde(skip_serializing_if = "Option::is_none")]
    national_identification: Option<NationalIdentification>,
    #[serde(skip_serializing_if = "Option::is_none")]
    customer_identification: Option<types::StringMax50>,
    #[serde(skip_serializing_if = "Option::is_none")]
    date_and_place_of_birth: Option<DateAndPlaceOfBirth>,
    #[serde(skip_serializing_if = "Option::is_none")]
    country_of_residence: Option<CountryCode>,
}

impl NaturalPerson {
    pub fn new(
        first_name: &str,
        last_name: &str,
        customer_identification: Option<&str>,
        address: Option<Address>,
    ) -> Result<Self, ValidationError> {
        Ok(Self {
            name: NaturalPersonName {
                name_identifier: NaturalPersonNameID {
                    primary_identifier: last_name.try_into()?,
                    secondary_identifier: Some(first_name.try_into()?),
                    name_identifier_type: NaturalPersonNameTypeCode::Legl,
                }
                .into(),
                local_name_identifier: None.into(),
                phonetic_name_identifier: None.into(),
            }
            .into(),
            geographic_address: address.into(),
            national_identification: None,
            customer_identification: customer_identification.map(TryInto::try_into).transpose()?,
            date_and_place_of_birth: None,
            country_of_residence: None,
        })
    }

    #[must_use]
    fn first_name(&self) -> Option<String> {
        Some(
            self.name
                .first()
                .name_identifier
                .first()
                .clone()
                .secondary_identifier?
                .into(),
        )
    }

    #[must_use]
    fn last_name(&self) -> String {
        self.name
            .first()
            .name_identifier
            .first()
            .primary_identifier
            .to_string()
    }

    #[must_use]
    fn address(&self) -> Option<&Address> {
        self.geographic_address.first()
    }
}

impl Validatable for NaturalPerson {
    fn validate(&self) -> Result<(), ValidationError> {
        self.name
            .clone()
            .into_iter()
            .try_for_each(|name| name.validate())?;
        self.geographic_address
            .clone()
            .into_iter()
            .try_for_each(|addr| addr.validate())?;

        Ok(())
    }
}

#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
struct NaturalPersonName {
    name_identifier: types::OneToN<NaturalPersonNameID>,
    #[serde(default, skip_serializing_if = "types::ZeroToN::is_empty")]
    local_name_identifier: types::ZeroToN<LocalNaturalPersonNameID>,
    #[serde(default, skip_serializing_if = "types::ZeroToN::is_empty")]
    phonetic_name_identifier: types::ZeroToN<LocalNaturalPersonNameID>,
}

impl Validatable for NaturalPersonName {
    fn validate(&self) -> Result<(), ValidationError> {
        let has_legl = self
            .name_identifier
            .clone()
            .into_iter()
            .any(|ni| ni.name_identifier_type == NaturalPersonNameTypeCode::Legl);
        if !has_legl {
            return Err("Natural person must have a legal name id (IVMS101 C6)".into());
        }
        Ok(())
    }
}

#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
struct NaturalPersonNameID {
    primary_identifier: types::StringMax100,
    #[serde(skip_serializing_if = "Option::is_none")]
    secondary_identifier: Option<types::StringMax100>,
    name_identifier_type: NaturalPersonNameTypeCode,
}

#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
struct LocalNaturalPersonNameID {
    primary_identifier: types::StringMax100,
    #[serde(skip_serializing_if = "Option::is_none")]
    secondary_identifier: Option<types::StringMax100>,
    name_identifier_type: NaturalPersonNameTypeCode,
}

#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct Address {
    address_type: AddressTypeCode,
    #[serde(skip_serializing_if = "Option::is_none")]
    department: Option<types::StringMax50>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sub_department: Option<types::StringMax70>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub street_name: Option<types::StringMax70>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub building_number: Option<types::StringMax16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    building_name: Option<types::StringMax35>,
    #[serde(skip_serializing_if = "Option::is_none")]
    floor: Option<types::StringMax70>,
    #[serde(skip_serializing_if = "Option::is_none")]
    post_box: Option<types::StringMax16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    room: Option<types::StringMax70>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub post_code: Option<types::StringMax16>,
    pub town_name: types::StringMax35,
    #[serde(skip_serializing_if = "Option::is_none")]
    town_location_name: Option<types::StringMax35>,
    #[serde(skip_serializing_if = "Option::is_none")]
    district_name: Option<types::StringMax35>,
    #[serde(skip_serializing_if = "Option::is_none")]
    country_sub_division: Option<types::StringMax35>,
    #[serde(default, skip_serializing_if = "types::ZeroToN::is_empty")]
    address_line: types::ZeroToN<types::StringMax70>,
    pub country: CountryCode,
}

impl Address {
    pub fn new(
        street: Option<&str>,
        number: Option<&str>,
        address_line: Option<&str>,
        postal_code: &str,
        town: &str,
        country: &str,
    ) -> Result<Self, ValidationError> {
        Ok(Self {
            address_type: AddressTypeCode::Home,
            department: None,
            sub_department: None,
            street_name: street.map(TryInto::try_into).transpose()?,
            building_number: number.map(TryInto::try_into).transpose()?,
            building_name: None,
            floor: None,
            post_box: None,
            room: None,
            post_code: Some(postal_code.try_into()?),
            town_name: town.try_into()?,
            town_location_name: None,
            district_name: None,
            country_sub_division: None,
            address_line: address_line.map(TryInto::try_into).transpose()?.into(),
            country: country.try_into()?,
        })
    }

    #[must_use]
    pub fn address_lines(&self) -> Option<String> {
        if self.address_line.is_empty() {
            None
        } else {
            Some(
                self.address_line
                    .clone()
                    .into_iter()
                    .map(Into::into)
                    .collect::<Vec<String>>()
                    .join(", "),
            )
        }
    }
}

impl std::fmt::Display for Address {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        format_address(
            f,
            self.street_name.as_ref().map(types::StringMax70::as_str),
            self.building_number
                .as_ref()
                .map(types::StringMax16::as_str),
            self.address_lines().as_deref(),
            self.post_code.as_ref().map(types::StringMax16::as_str),
            self.town_name.as_str(),
            self.country.as_str(),
        )
    }
}

pub fn format_address(
    f: &mut std::fmt::Formatter,
    street: Option<&str>,
    number: Option<&str>,
    address_line: Option<&str>,
    postcode: Option<&str>,
    town: &str,
    country_code: &str,
) -> std::fmt::Result {
    if let Some(s) = street {
        write!(f, "{s}")?;
        if let Some(n) = number {
            write!(f, " {n}")?;
        }
        write!(f, ", ")?;
    }
    if let Some(al) = address_line {
        write!(f, "{al}, ")?;
    }
    if let Some(pc) = postcode {
        write!(f, "{pc} ")?;
    }
    write!(
        f,
        "{town}, {}",
        crate::country_codes::country(country_code.to_lowercase().as_str())
    )
}

impl Validatable for Address {
    fn validate(&self) -> Result<(), ValidationError> {
        if self.address_line.is_empty()
            && (self.street_name.is_none()
                || (self.building_name.is_none() && self.building_number.is_none()))
        {
            return Err("Either 1) address line or 2) street name and either building name or building number are required (IVMS101 C8)".into());
        }
        Ok(())
    }
}

#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
struct DateAndPlaceOfBirth {
    date_of_birth: Date,
    place_of_birth: types::StringMax70,
}

impl Validatable for DateAndPlaceOfBirth {
    fn validate(&self) -> Result<(), ValidationError> {
        if self.date_of_birth >= chrono::prelude::Utc::now().date_naive() {
            return Err("Date of birth must be in the past (IVMS101 C2)".into());
        }
        Ok(())
    }
}

#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
struct NationalIdentification {
    national_identifier: types::StringMax35,
    national_identifier_type: NationalIdentifierTypeCode,
    #[serde(skip_serializing_if = "Option::is_none")]
    country_of_issue: Option<CountryCode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    registration_authority: Option<RegistrationAuthority>,
}

impl Validatable for NationalIdentification {
    fn validate(&self) -> Result<(), ValidationError> {
        if let Some(ra) = &self.registration_authority {
            ra.validate()?;
        };
        Ok(())
    }
}

#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct LegalPerson {
    name: LegalPersonName,
    #[serde(default, skip_serializing_if = "types::ZeroToN::is_empty")]
    geographic_address: types::ZeroToN<Address>,
    #[serde(skip_serializing_if = "Option::is_none")]
    customer_identification: Option<types::StringMax50>,
    #[serde(skip_serializing_if = "Option::is_none")]
    national_identification: Option<NationalIdentification>,
    #[serde(skip_serializing_if = "Option::is_none")]
    country_of_registration: Option<CountryCode>,
}

impl LegalPerson {
    pub fn new(
        name: &str,
        customer_identification: &str,
        address: Address,
        lei: &lei::LEI,
    ) -> Result<Self, ValidationError> {
        Ok(Self {
            name: LegalPersonName {
                name_identifier: LegalPersonNameID {
                    legal_person_name: name.try_into()?,
                    legal_person_name_identifier_type: LegalPersonNameTypeCode::Legl,
                }
                .into(),
                local_name_identifier: None.into(),
                phonetic_name_identifier: None.into(),
            },
            geographic_address: Some(address).into(),
            customer_identification: Some(customer_identification.try_into()?),
            national_identification: Some(NationalIdentification {
                national_identifier: lei.to_string().as_str().try_into()?,
                national_identifier_type: NationalIdentifierTypeCode::Leix,
                country_of_issue: None,
                registration_authority: None,
            }),
            country_of_registration: None,
        })
    }

    fn lei(&self) -> Result<Option<lei::LEI>, lei::Error> {
        self.national_identification
            .as_ref()
            .map(|ni| lei::LEI::try_from(ni.national_identifier.to_string().as_str()))
            .transpose()
    }
}

impl LegalPerson {
    #[must_use]
    fn name(&self) -> String {
        self.name
            .name_identifier
            .first()
            .legal_person_name
            .to_string()
    }

    #[must_use]
    fn address(&self) -> Option<&Address> {
        self.geographic_address.first()
    }
}

impl Validatable for LegalPerson {
    fn validate(&self) -> Result<(), ValidationError> {
        let has_geog = self
            .geographic_address
            .clone()
            .into_iter()
            .any(|addr| addr.address_type == AddressTypeCode::Geog);
        if !has_geog
            && self.national_identification.is_none()
            && self.customer_identification.is_none()
        {
            return Err(
                "Legal person needs either geographic address, customer number or national identification (IVMS101 C4)"
                    .into(),
            );
        }
        if let Some(ni) = &self.national_identification {
            if !matches!(
                ni.national_identifier_type,
                NationalIdentifierTypeCode::Raid
                    | NationalIdentifierTypeCode::Misc
                    | NationalIdentifierTypeCode::Leix
                    | NationalIdentifierTypeCode::Txid
            ) {
                return Err("Legal person must have a 'RAID', 'MISC', 'LEIX' or 'TXID' identification (IVMS101 C7)".into());
            }
        };
        if let Some(ni) = &self.national_identification {
            if ni.national_identifier_type == NationalIdentifierTypeCode::Leix {
                if let Err(e) = lei::LEI::try_from(ni.national_identifier.as_str()) {
                    return Err(format!("Invalid LEI: {e} (IVMS101 C11)").as_str().into());
                }
            }
        };
        self.name.validate()?;
        self.geographic_address
            .clone()
            .into_iter()
            .try_for_each(|addr| addr.validate())?;
        match &self.national_identification {
            Some(ni) => {
                if ni.country_of_issue.is_some() {
                    return Err("Legal person must not have a country of issue (IVMS101 C9)".into());
                }
                if ni.national_identifier_type != NationalIdentifierTypeCode::Leix
                    && ni.registration_authority.is_none()
                {
                    return Err("Legal person must specify registration authority for non-'LEIX' identification (IVMS101 C9)".into());
                }
                if ni.national_identifier_type == NationalIdentifierTypeCode::Leix
                    && ni.registration_authority.is_some()
                {
                    return Err("Legal person must not specify registration authority for 'LEIX' identification (IVMS101 C9)".into());
                }
            }
            None => (),
        }
        Ok(())
    }
}

#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
struct LegalPersonName {
    name_identifier: types::OneToN<LegalPersonNameID>,
    #[serde(default, skip_serializing_if = "types::ZeroToN::is_empty")]
    local_name_identifier: types::ZeroToN<LocalLegalPersonNameID>,
    #[serde(default, skip_serializing_if = "types::ZeroToN::is_empty")]
    phonetic_name_identifier: types::ZeroToN<LocalLegalPersonNameID>,
}

impl Validatable for LegalPersonName {
    fn validate(&self) -> Result<(), ValidationError> {
        let has_legl = self
            .name_identifier
            .clone()
            .into_iter()
            .any(|ni| ni.legal_person_name_identifier_type == LegalPersonNameTypeCode::Legl);
        if !has_legl {
            return Err("Legal person must have a legal name id (IVMS101 C5)".into());
        }
        Ok(())
    }
}

#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
struct LegalPersonNameID {
    legal_person_name: types::StringMax100,
    legal_person_name_identifier_type: LegalPersonNameTypeCode,
}

#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
struct LocalLegalPersonNameID {
    legal_person_name: types::StringMax100,
    legal_person_name_identifier_type: LegalPersonNameTypeCode,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
struct IntermediaryVASP {
    intermediary_vasp: Person,
    sequence: u32,
}

// Validating C12 (sequentialIntegrity) requires surrounding context
impl Validatable for IntermediaryVASP {
    fn validate(&self) -> Result<(), ValidationError> {
        self.intermediary_vasp.validate()?;
        Ok(())
    }
}

#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "UPPERCASE")]
enum NaturalPersonNameTypeCode {
    Alia, // Alias name
    Birt, // Name at birth
    Maid, // Maiden name
    Legl, // Legal name
    Misc, // Unspecified
}

#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "UPPERCASE")]
enum LegalPersonNameTypeCode {
    Legl, // Legal name
    Shrt, // Short name
    Trad, // Trading name
}

type Date = chrono::NaiveDate;

#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "UPPERCASE")]
enum AddressTypeCode {
    Home, // Residential
    Bizz, // Business
    Geog, // Geographic
}

#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "UPPERCASE")]
enum NationalIdentifierTypeCode {
    Arnu, // Alien registration number
    Ccpt, // Passport number
    Raid, // Registration authority identifier
    Drlc, // Driver license number
    Fiin, // Foreign investment identity number
    Txid, // Tax identification number
    Socs, // Social security number
    Idcd, // Identity card number
    Leix, // Legal Entity Identifier
    Misc, // Unspecified
}

pub trait Validatable {
    fn validate(&self) -> Result<(), ValidationError>;
}

crate::constrained_string!(CountryCode, |l| l == 2);

impl Validatable for CountryCode {
    fn validate(&self) -> Result<(), ValidationError> {
        // represents an "unknown State or entity"
        if self.to_string() == "XX" {
            return Ok(());
        }
        match iso3166_1::alpha2(&self.to_string()) {
            Some(_) => Ok(()),
            None => Err("Invalid country code (IVMS101 C3)".into()),
        }
    }
}

crate::constrained_string!(RegistrationAuthority, |l| l == 8);

impl Validatable for RegistrationAuthority {
    fn validate(&self) -> Result<(), ValidationError> {
        if !lei::registration_authority::is_valid_ra(&self.inner) {
            return Err(
                "Provided registration authority is not on the GLEIF list (IVMS101 C10)".into(),
            );
        }
        Ok(())
    }
}
#[derive(thiserror::Error, Debug, PartialEq, Eq)]
#[error("Validation error: {0}")]
pub struct ValidationError(String);

impl From<&str> for ValidationError {
    fn from(value: &str) -> Self {
        Self(value.to_owned())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_test::{assert_tokens, Token};

    impl NaturalPerson {
        fn mock() -> Self {
            Self {
                name: NaturalPersonName::mock().into(),
                geographic_address: None.into(),
                national_identification: None,
                customer_identification: None,
                date_and_place_of_birth: None,
                country_of_residence: None,
            }
        }
    }

    impl LegalPerson {
        fn mock() -> Self {
            Self {
                name: LegalPersonName::mock(),
                geographic_address: None.into(),
                customer_identification: None,
                national_identification: None,
                country_of_registration: None,
            }
        }
    }

    impl LegalPersonName {
        fn mock() -> Self {
            Self {
                name_identifier: LegalPersonNameID::mock().into(),
                local_name_identifier: None.into(),
                phonetic_name_identifier: None.into(),
            }
        }
    }

    impl LegalPersonNameID {
        fn mock() -> Self {
            Self {
                legal_person_name: "Company A".try_into().unwrap(),
                legal_person_name_identifier_type: LegalPersonNameTypeCode::Legl,
            }
        }
    }

    impl NationalIdentification {
        fn mock() -> Self {
            Self {
                national_identifier: "id".try_into().unwrap(),
                national_identifier_type: NationalIdentifierTypeCode::Misc,
                country_of_issue: None,
                registration_authority: Some("RA000001".try_into().unwrap()),
            }
        }
    }

    impl Address {
        fn mock() -> Self {
            Self {
                address_type: AddressTypeCode::Geog,
                department: None,
                sub_department: None,
                street_name: None,
                building_number: None,
                building_name: None,
                floor: None,
                post_box: None,
                room: None,
                post_code: None,
                town_name: "Zurich".try_into().unwrap(),
                town_location_name: None,
                district_name: None,
                country_sub_division: None,
                address_line: Some("Main street".try_into().unwrap()).into(),
                country: "CH".try_into().unwrap(),
            }
        }
    }

    impl NaturalPersonNameID {
        fn mock() -> Self {
            Self {
                primary_identifier: "Engels".try_into().unwrap(),
                secondary_identifier: Some("Friedrich".try_into().unwrap()),
                name_identifier_type: NaturalPersonNameTypeCode::Legl,
            }
        }
    }

    impl NaturalPersonName {
        fn mock() -> Self {
            Self {
                name_identifier: NaturalPersonNameID::mock().into(),
                local_name_identifier: None.into(),
                phonetic_name_identifier: None.into(),
            }
        }
    }

    impl DateAndPlaceOfBirth {
        fn mock() -> Self {
            Self {
                date_of_birth: chrono::NaiveDate::from_ymd_opt(1946, 11, 5).unwrap(),
                place_of_birth: "London".try_into().unwrap(),
            }
        }
    }

    #[test]
    fn test_date() {
        assert_tokens(
            &Date::from_ymd_opt(2018, 11, 5).unwrap(),
            &[Token::String("2018-11-05")],
        );
    }

    #[test]
    fn test_type_codes() {
        assert_tokens(
            &NaturalPersonNameTypeCode::Alia,
            &[Token::UnitVariant {
                name: "NaturalPersonNameTypeCode",
                variant: "ALIA",
            }],
        );
        assert_tokens(
            &LegalPersonNameTypeCode::Legl,
            &[Token::UnitVariant {
                name: "LegalPersonNameTypeCode",
                variant: "LEGL",
            }],
        );
        assert_tokens(
            &AddressTypeCode::Bizz,
            &[Token::UnitVariant {
                name: "AddressTypeCode",
                variant: "BIZZ",
            }],
        );
        assert_tokens(
            &NationalIdentifierTypeCode::Arnu,
            &[Token::UnitVariant {
                name: "NationalIdentifierTypeCode",
                variant: "ARNU",
            }],
        );
    }

    #[test]
    fn test_country_code() {
        let de = CountryCode { inner: "DE".into() };
        assert_tokens(&de, &[Token::BorrowedStr("DE")]);
        de.validate().unwrap();
    }

    #[test]
    fn test_country_code_unknown_placeholder() {
        let cc = CountryCode { inner: "XX".into() };
        cc.validate().unwrap();
    }

    #[test]
    fn test_country_code_invalid_length() {
        serde_test::assert_de_tokens_error::<CountryCode>(
            &[Token::BorrowedStr("C")],
            r#"Validation error: Cannot parse String of length 1 into a "ivms101::messages::CountryCode""#,
        );
        serde_test::assert_de_tokens_error::<CountryCode>(
            &[Token::BorrowedStr("CHE")],
            r#"Validation error: Cannot parse String of length 3 into a "ivms101::messages::CountryCode""#,
        );
    }

    #[test]
    fn test_invalid_country_code() {
        let invalid = CountryCode { inner: "RR".into() };
        match_validation_error(&invalid, 3);
    }

    #[test]
    fn test_registration_authority_invalid_length() {
        serde_test::assert_de_tokens_error::<RegistrationAuthority>(
            &[Token::BorrowedStr("RA00009")],
            r#"Validation error: Cannot parse String of length 7 into a "ivms101::messages::RegistrationAuthority""#,
        );
        serde_test::assert_de_tokens_error::<RegistrationAuthority>(
            &[Token::BorrowedStr("RA0000945")],
            r#"Validation error: Cannot parse String of length 9 into a "ivms101::messages::RegistrationAuthority""#,
        );
    }

    #[test]
    fn test_registration_authority_invalid_value() {
        match_validation_error(
            &<RegistrationAuthority as TryFrom<_>>::try_from("RA100094").unwrap(),
            10,
        );
    }

    #[test]
    fn test_registration_authority() {
        let ra: RegistrationAuthority = "RA000094".try_into().unwrap();
        assert_tokens(&ra, &[Token::BorrowedStr("RA000094")]);
        ra.validate().unwrap();
    }

    fn match_validation_error(val: &impl Validatable, code: u8) {
        let res = val.validate();
        assert!(res
            .unwrap_err()
            .to_string()
            .ends_with(format!("(IVMS101 C{code})").as_str()));
    }

    #[test]
    fn test_person_serialization() {
        let person = Person::NaturalPerson(NaturalPerson::mock());
        let serialized = serde_json::to_string(&person).unwrap();
        assert_eq!(
            serialized,
            r#"{"naturalPerson":{"name":{"nameIdentifier":{"primaryIdentifier":"Engels","secondaryIdentifier":"Friedrich","nameIdentifierType":"LEGL"}}}}"#
        );
        let deserialized: Person = serde_json::from_str(&serialized).unwrap();
        assert_eq!(person, deserialized);

        let person = Person::LegalPerson(LegalPerson::mock());
        let serialized = serde_json::to_string(&person).unwrap();
        assert_eq!(
            serialized,
            r#"{"legalPerson":{"name":{"nameIdentifier":{"legalPersonName":"Company A","legalPersonNameIdentifierType":"LEGL"}}}}"#
        );
        let deserialized: Person = serde_json::from_str(&serialized).unwrap();
        assert_eq!(person, deserialized);
    }

    #[test]
    fn test_c1_validation_error() {
        let originator = Originator {
            originator_persons: Person::NaturalPerson(NaturalPerson::mock()).into(),
            account_number: None.into(),
        };
        match_validation_error(&originator, 1);
    }

    #[test]
    fn test_c1_validation_pass() {
        let mut person = NaturalPerson::mock();
        person.geographic_address = Some(Address::mock()).into();
        let originator = Originator {
            originator_persons: Person::NaturalPerson(person.clone()).into(),
            account_number: None.into(),
        };
        originator.validate().unwrap();

        person.geographic_address = None.into();
        person.customer_identification = Some("customer-id".try_into().unwrap());
        let originator = Originator {
            originator_persons: Person::NaturalPerson(person.clone()).into(),
            account_number: None.into(),
        };
        originator.validate().unwrap();

        person.customer_identification = None;
        person.national_identification = Some(NationalIdentification::mock());
        let originator = Originator {
            originator_persons: Person::NaturalPerson(person.clone()).into(),
            account_number: None.into(),
        };
        originator.validate().unwrap();

        person.national_identification = None;
        person.date_and_place_of_birth = Some(DateAndPlaceOfBirth::mock());
        let originator = Originator {
            originator_persons: Person::NaturalPerson(person).into(),
            account_number: None.into(),
        };
        originator.validate().unwrap();

        let beneficiary = Beneficiary {
            beneficiary_persons: Person::NaturalPerson(NaturalPerson::mock()).into(),
            account_number: None.into(),
        };
        beneficiary.validate().unwrap();
    }

    #[test]
    fn test_c2_validation_error() {
        let date = DateAndPlaceOfBirth {
            date_of_birth: chrono::NaiveDate::MAX,
            place_of_birth: "Bern".try_into().unwrap(),
        };
        match_validation_error(&date, 2);
    }

    #[test]
    fn test_c2_validation_pass() {
        let date = DateAndPlaceOfBirth {
            date_of_birth: chrono::NaiveDate::MIN,
            place_of_birth: "Bern".try_into().unwrap(),
        };

        date.validate().unwrap();
    }

    // C3 is tested in test_invalid_country_code

    #[test]
    fn test_c4_validation_error() {
        let legal = LegalPerson::mock();
        match_validation_error(&legal, 4);
    }

    #[test]
    fn test_c4_validation_pass() {
        let mut legal = LegalPerson::mock();

        legal.geographic_address = Some(Address::mock()).into();
        legal.validate().unwrap();
        legal.geographic_address = None.into();

        legal.customer_identification = Some("id".try_into().unwrap());
        legal.validate().unwrap();
        legal.customer_identification = None;

        legal.national_identification = Some(NationalIdentification::mock());
        legal.validate().unwrap();
    }

    #[test]
    fn test_c5_validation_error() {
        let mut legal = LegalPersonName::mock();
        legal.name_identifier = LegalPersonNameID {
            legal_person_name: "Company A".try_into().unwrap(),
            legal_person_name_identifier_type: LegalPersonNameTypeCode::Shrt,
        }
        .into();
        match_validation_error(&legal, 5);
    }

    #[test]
    fn test_c5_validation_pass() {
        let legal = LegalPersonName::mock();
        legal.validate().unwrap();
    }

    #[test]
    fn test_c6_validation_error() {
        let mut name = NaturalPersonName::mock();
        name.name_identifier = NaturalPersonNameID {
            primary_identifier: "Karl".try_into().unwrap(),
            name_identifier_type: NaturalPersonNameTypeCode::Alia,
            secondary_identifier: None,
        }
        .into();
        match_validation_error(&name, 6);
    }

    #[test]
    fn test_c6_validation_pass() {
        let mut name = NaturalPersonName::mock();
        name.name_identifier = NaturalPersonNameID {
            primary_identifier: "Emil Steinberger".try_into().unwrap(),
            secondary_identifier: None,
            name_identifier_type: NaturalPersonNameTypeCode::Legl,
        }
        .into();
        name.validate().unwrap();
    }

    #[test]
    fn test_c7_validation_error() {
        let mut person = LegalPerson::mock();
        let mut id = NationalIdentification::mock();

        for code in [
            NationalIdentifierTypeCode::Arnu,
            NationalIdentifierTypeCode::Ccpt,
            NationalIdentifierTypeCode::Drlc,
            NationalIdentifierTypeCode::Fiin,
            NationalIdentifierTypeCode::Idcd,
            NationalIdentifierTypeCode::Socs,
        ] {
            id.national_identifier_type = code;
            person.national_identification = Some(id.clone());
            match_validation_error(&person, 7);
        }
    }

    #[test]
    fn test_c7_validation_pass() {
        let mut person = LegalPerson::mock();

        for code in [
            NationalIdentifierTypeCode::Leix,
            NationalIdentifierTypeCode::Misc,
            NationalIdentifierTypeCode::Raid,
            NationalIdentifierTypeCode::Txid,
        ] {
            let mut id = NationalIdentification::mock();
            id.national_identifier_type = code.clone();
            if code == NationalIdentifierTypeCode::Leix {
                // Use a valid LEI to make C11 pass
                id.national_identifier = "2594007XIACKNMUAW223".try_into().unwrap();
                // Make C9 pass
                id.registration_authority = None;
            }
            person.national_identification = Some(id.clone());
            person.validate().unwrap();
        }
    }

    #[test]
    fn test_c8_validation_error() {
        let mut addr = Address::mock();
        addr.address_line = None.into();
        match_validation_error(&addr, 8);

        addr.street_name = Some("main street".try_into().unwrap());
        match_validation_error(&addr, 8);
    }

    #[test]
    fn test_c8_validation_pass() {
        let mut addr = Address::mock();
        addr.validate().unwrap();

        addr.address_line = None.into();
        addr.street_name = Some("main street".try_into().unwrap());
        addr.building_name = Some("main building".try_into().unwrap());
        addr.validate().unwrap();

        addr.building_name = None;
        addr.building_number = Some("12".try_into().unwrap());
        addr.validate().unwrap();
    }

    #[test]
    fn test_c9_validation_error() {
        let mut ni = NationalIdentification::mock();
        ni.country_of_issue = Some(CountryCode { inner: "CH".into() });
        let mut person = LegalPerson::mock();
        person.national_identification = Some(ni.clone());
        match_validation_error(&person, 9);

        ni.national_identifier_type = NationalIdentifierTypeCode::Leix;
        // Use a valid LEI to make C11 pass
        ni.national_identifier = "2594007XIACKNMUAW223".try_into().unwrap();
        person.national_identification = Some(ni.clone());
        match_validation_error(&person, 9);

        ni.national_identifier_type = NationalIdentifierTypeCode::Misc;
        ni.registration_authority = None;
        person.national_identification = Some(ni);
        match_validation_error(&person, 9);
    }

    #[test]
    fn test_c9_validation_pass() {
        let mut person = LegalPerson::mock();
        person.customer_identification = Some("id".try_into().unwrap());
        person.validate().unwrap();

        let mut ni = NationalIdentification::mock();
        person.national_identification = Some(ni.clone());
        person.validate().unwrap();

        ni.registration_authority = None;
        ni.national_identifier_type = NationalIdentifierTypeCode::Leix;
        // Use a valid LEI to make C11 pass
        ni.national_identifier = "2594007XIACKNMUAW223".try_into().unwrap();
        person.national_identification = Some(ni);
        person.validate().unwrap();
    }

    // C10 is tested in test_registration_authority_invalid_value

    #[test]
    fn test_c11_validation_error() {
        let mut person = LegalPerson::mock();
        let mut ni = NationalIdentification::mock();
        ni.registration_authority = None;
        ni.national_identifier_type = NationalIdentifierTypeCode::Leix;
        ni.national_identifier = "invalid-lei".try_into().unwrap();
        person.national_identification = Some(ni);
        match_validation_error(&person, 11);
    }

    #[test]
    fn test_c11_validation_pass() {
        let mut person = LegalPerson::mock();
        let mut ni = NationalIdentification::mock();
        ni.registration_authority = None;
        ni.national_identifier_type = NationalIdentifierTypeCode::Leix;
        ni.national_identifier = "2594007XIACKNMUAW223".try_into().unwrap();
        person.national_identification = Some(ni);
        person.validate().unwrap();
    }

    #[test]
    fn test_natural_person_name() {
        let mut person = NaturalPerson::mock();
        assert_eq!(person.first_name(), Some("Friedrich".into()));
        assert_eq!(person.last_name(), "Engels");
        let mut name = NaturalPersonNameID::mock();
        name.secondary_identifier = None;
        person.name = NaturalPersonName {
            name_identifier: name.into(),
            local_name_identifier: None.into(),
            phonetic_name_identifier: None.into(),
        }
        .into();
        assert_eq!(person.first_name(), None);
        assert_eq!(person.last_name(), "Engels".to_string());
    }

    #[test]
    fn test_legal_person_name() {
        assert_eq!(LegalPerson::mock().name(), "Company A");
    }

    #[test]
    fn test_address_display() {
        let person = NaturalPerson::mock();
        assert_eq!(person.address(), None);
        let mut address = Address::mock();
        assert_eq!(
            address.to_string(),
            "Main street, Zurich, Switzerland".to_string()
        );
        address.post_code = Some("8000".try_into().unwrap());
        assert_eq!(
            address.to_string(),
            "Main street, 8000 Zurich, Switzerland".to_string()
        );
        address.address_line =
            vec!["line 1".try_into().unwrap(), "line 2".try_into().unwrap()].into();
        assert_eq!(
            address.to_string(),
            "line 1, line 2, 8000 Zurich, Switzerland".to_string()
        );
        address.address_line = None.into();
        assert_eq!(address.to_string(), "8000 Zurich, Switzerland".to_string());
        address.street_name = Some("Main street".try_into().unwrap());
        address.building_number = Some("12".try_into().unwrap());
        assert_eq!(
            address.to_string(),
            "Main street 12, 8000 Zurich, Switzerland".to_string()
        );
    }
}
