pub use country_codes::{country, CountryCode};
pub use types::{one_to_n::OneToN, zero_to_n::ZeroToN};

mod country_codes;
mod types;

use lei::registration_authority::RegistrationAuthority;

#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct IVMS101 {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub originator: Option<Originator>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub beneficiary: Option<Beneficiary>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "originatingVASP")]
    pub originating_vasp: Option<OriginatingVASP>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "beneficiaryVASP")]
    pub beneficiary_vasp: Option<BeneficiaryVASP>,
}

impl Validatable for IVMS101 {
    fn validate(&self) -> Result<(), Error> {
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
    pub originator_persons: OneToN<Person>,
    #[serde(default, skip_serializing_if = "ZeroToN::is_empty")]
    pub account_number: ZeroToN<types::StringMax100>,
}

impl Validatable for Originator {
    fn validate(&self) -> Result<(), Error> {
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
    pub fn new(person: Person) -> Result<Self, Error> {
        Ok(Self {
            originator_persons: person.into(),
            account_number: None.into(),
        })
    }
}

#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct Beneficiary {
    pub beneficiary_persons: OneToN<Person>,
    #[serde(default, skip_serializing_if = "ZeroToN::is_empty")]
    pub account_number: ZeroToN<types::StringMax100>,
}

impl Validatable for Beneficiary {
    fn validate(&self) -> Result<(), Error> {
        for person in self.beneficiary_persons.clone() {
            person.validate()?;
        }
        Ok(())
    }
}

impl Beneficiary {
    pub fn new(person: Person, account_number: Option<&str>) -> Result<Self, Error> {
        Ok(Self {
            beneficiary_persons: person.into(),
            account_number: account_number.map(TryInto::try_into).transpose()?.into(),
        })
    }
}

#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct OriginatingVASP {
    #[serde(rename = "originatingVASP")]
    pub originating_vasp: Person,
}

impl OriginatingVASP {
    pub fn new(name: &str, lei: &lei::LEI) -> Result<Self, Error> {
        Ok(Self {
            originating_vasp: Person::LegalPerson(LegalPerson {
                name: LegalPersonName {
                    name_identifier: LegalPersonNameID {
                        legal_person_name: name.try_into()?,
                        legal_person_name_identifier_type: LegalPersonNameTypeCode::Legal,
                    }
                    .into(),
                    local_name_identifier: None.into(),
                    phonetic_name_identifier: None.into(),
                },
                geographic_address: ZeroToN::None,
                customer_identification: None,
                national_identification: Some(NationalIdentification {
                    national_identifier: lei.to_string().as_str().try_into().unwrap(),
                    national_identifier_type: NationalIdentifierTypeCode::LegalEntityIdentifier,
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
    fn validate(&self) -> Result<(), Error> {
        self.originating_vasp.validate()
    }
}

#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct BeneficiaryVASP {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "beneficiaryVASP")]
    pub beneficiary_vasp: Option<Person>,
}

impl Validatable for BeneficiaryVASP {
    fn validate(&self) -> Result<(), Error> {
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
    pub fn first_name(&self) -> Option<String> {
        match self {
            Self::NaturalPerson(p) => p.first_name(),
            Self::LegalPerson(_p) => None,
        }
    }
    #[must_use]
    pub fn last_name(&self) -> String {
        match self {
            Self::NaturalPerson(p) => p.last_name(),
            Self::LegalPerson(p) => p.name(),
        }
    }

    #[must_use]
    pub fn address(&self) -> Option<&Address> {
        match self {
            Self::NaturalPerson(p) => p.address(),
            Self::LegalPerson(p) => p.address(),
        }
    }

    #[must_use]
    pub fn customer_identification(&self) -> Option<String> {
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
    fn validate(&self) -> Result<(), Error> {
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
    pub name: OneToN<NaturalPersonName>,
    #[serde(default, skip_serializing_if = "ZeroToN::is_empty")]
    pub geographic_address: ZeroToN<Address>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub national_identification: Option<NationalIdentification>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_identification: Option<types::StringMax50>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_and_place_of_birth: Option<DateAndPlaceOfBirth>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country_of_residence: Option<CountryCode>,
}

impl NaturalPerson {
    pub fn new(
        first_name: &str,
        last_name: &str,
        customer_identification: Option<&str>,
        address: Option<Address>,
    ) -> Result<Self, Error> {
        Ok(Self {
            name: NaturalPersonName {
                name_identifier: NaturalPersonNameID {
                    primary_identifier: last_name.try_into()?,
                    secondary_identifier: Some(first_name.try_into()?),
                    name_identifier_type: NaturalPersonNameTypeCode::LegalName,
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
    fn validate(&self) -> Result<(), Error> {
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
pub struct NaturalPersonName {
    pub name_identifier: OneToN<NaturalPersonNameID>,
    #[serde(default, skip_serializing_if = "ZeroToN::is_empty")]
    pub local_name_identifier: ZeroToN<NaturalPersonNameID>,
    #[serde(default, skip_serializing_if = "ZeroToN::is_empty")]
    pub phonetic_name_identifier: ZeroToN<NaturalPersonNameID>,
}

impl Validatable for NaturalPersonName {
    fn validate(&self) -> Result<(), Error> {
        let has_legl = self
            .name_identifier
            .clone()
            .into_iter()
            .any(|ni| ni.name_identifier_type == NaturalPersonNameTypeCode::LegalName);
        if !has_legl {
            return Err("Natural person must have a legal name id (IVMS101 C6)".into());
        }
        Ok(())
    }
}

#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct NaturalPersonNameID {
    pub primary_identifier: types::StringMax100,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secondary_identifier: Option<types::StringMax100>,
    pub name_identifier_type: NaturalPersonNameTypeCode,
}

#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct Address {
    pub address_type: AddressTypeCode,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub department: Option<types::StringMax50>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sub_department: Option<types::StringMax70>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub street_name: Option<types::StringMax70>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub building_number: Option<types::StringMax16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub building_name: Option<types::StringMax35>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub floor: Option<types::StringMax70>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub post_box: Option<types::StringMax16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub room: Option<types::StringMax70>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub post_code: Option<types::StringMax16>,
    pub town_name: types::StringMax35,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub town_location_name: Option<types::StringMax35>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub district_name: Option<types::StringMax35>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country_sub_division: Option<types::StringMax35>,
    #[serde(default, skip_serializing_if = "ZeroToN::is_empty")]
    pub address_line: ZeroToN<types::StringMax70>,
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
    ) -> Result<Self, Error> {
        Ok(Self {
            address_type: AddressTypeCode::Residential,
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
        country(country_code.to_lowercase().as_str())
    )
}

impl Validatable for Address {
    fn validate(&self) -> Result<(), Error> {
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
pub struct DateAndPlaceOfBirth {
    pub date_of_birth: Date,
    pub place_of_birth: types::StringMax70,
}

impl Validatable for DateAndPlaceOfBirth {
    fn validate(&self) -> Result<(), Error> {
        if self.date_of_birth >= chrono::prelude::Utc::now().date_naive() {
            return Err("Date of birth must be in the past (IVMS101 C2)".into());
        }
        Ok(())
    }
}

#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct NationalIdentification {
    pub national_identifier: types::StringMax35,
    pub national_identifier_type: NationalIdentifierTypeCode,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country_of_issue: Option<CountryCode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registration_authority: Option<RegistrationAuthority>,
}

#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct LegalPerson {
    pub name: LegalPersonName,
    #[serde(default, skip_serializing_if = "ZeroToN::is_empty")]
    pub geographic_address: ZeroToN<Address>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_identification: Option<types::StringMax50>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub national_identification: Option<NationalIdentification>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country_of_registration: Option<CountryCode>,
}

impl LegalPerson {
    pub fn new(
        name: &str,
        customer_identification: &str,
        address: Address,
        lei: &lei::LEI,
    ) -> Result<Self, Error> {
        Ok(Self {
            name: LegalPersonName {
                name_identifier: LegalPersonNameID {
                    legal_person_name: name.try_into()?,
                    legal_person_name_identifier_type: LegalPersonNameTypeCode::Legal,
                }
                .into(),
                local_name_identifier: None.into(),
                phonetic_name_identifier: None.into(),
            },
            geographic_address: Some(address).into(),
            customer_identification: Some(customer_identification.try_into()?),
            national_identification: Some(NationalIdentification {
                national_identifier: lei.to_string().as_str().try_into().unwrap(),
                national_identifier_type: NationalIdentifierTypeCode::LegalEntityIdentifier,
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
    fn validate(&self) -> Result<(), Error> {
        let has_geog = self
            .geographic_address
            .clone()
            .into_iter()
            .any(|addr| addr.address_type == AddressTypeCode::Residential);
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
                NationalIdentifierTypeCode::RegistrationAuthorityIdentifier
                    | NationalIdentifierTypeCode::Unspecified
                    | NationalIdentifierTypeCode::LegalEntityIdentifier
                    | NationalIdentifierTypeCode::TaxIdentificationNumber
            ) {
                return Err("Legal person must have a 'RAID', 'MISC', 'LEIX' or 'TXID' identification (IVMS101 C7)".into());
            }
        };
        if let Some(ni) = &self.national_identification {
            if ni.national_identifier_type == NationalIdentifierTypeCode::LegalEntityIdentifier {
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
                if ni.national_identifier_type != NationalIdentifierTypeCode::LegalEntityIdentifier
                    && ni.registration_authority.is_none()
                {
                    return Err("Legal person must specify registration authority for non-'LEIX' identification (IVMS101 C9)".into());
                }
                if ni.national_identifier_type == NationalIdentifierTypeCode::LegalEntityIdentifier
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
pub struct LegalPersonName {
    pub name_identifier: OneToN<LegalPersonNameID>,
    #[serde(default, skip_serializing_if = "ZeroToN::is_empty")]
    pub local_name_identifier: ZeroToN<LegalPersonNameID>,
    #[serde(default, skip_serializing_if = "ZeroToN::is_empty")]
    pub phonetic_name_identifier: ZeroToN<LegalPersonNameID>,
}

impl Validatable for LegalPersonName {
    fn validate(&self) -> Result<(), Error> {
        let has_legl = self
            .name_identifier
            .clone()
            .into_iter()
            .any(|ni| ni.legal_person_name_identifier_type == LegalPersonNameTypeCode::Legal);
        if !has_legl {
            return Err("Legal person must have a legal name id (IVMS101 C5)".into());
        }
        Ok(())
    }
}

#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct LegalPersonNameID {
    pub legal_person_name: types::StringMax100,
    pub legal_person_name_identifier_type: LegalPersonNameTypeCode,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct IntermediaryVASP {
    pub intermediary_vasp: Person,
    pub sequence: u32,
}

// Validating C12 (sequentialIntegrity) requires surrounding context
impl Validatable for IntermediaryVASP {
    fn validate(&self) -> Result<(), Error> {
        self.intermediary_vasp.validate()?;
        Ok(())
    }
}

#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum NaturalPersonNameTypeCode {
    #[serde(rename = "ALIA")]
    Alias,
    #[serde(rename = "BIRT")]
    NameAtBirth,
    #[serde(rename = "MAID")]
    MaidenName,
    #[serde(rename = "LEGL")]
    LegalName,
    #[serde(rename = "MISC")]
    Unspecified,
}

#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum LegalPersonNameTypeCode {
    #[serde(rename = "LEGL")]
    Legal,
    #[serde(rename = "SHRT")]
    Short,
    #[serde(rename = "TRAD")]
    Trading,
}

type Date = chrono::NaiveDate;

#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum AddressTypeCode {
    #[serde(rename = "HOME")]
    Residential,
    #[serde(rename = "BIZZ")]
    Business,
    #[serde(rename = "GEOG")]
    Geographic,
}

#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum NationalIdentifierTypeCode {
    #[serde(rename = "ARNU")]
    AlienRegistrationNumber,
    #[serde(rename = "CCPT")]
    PassportNumber,
    #[serde(rename = "RAID")]
    RegistrationAuthorityIdentifier,
    #[serde(rename = "DRLC")]
    DriverLicenseNumber,
    #[serde(rename = "FIIN")]
    ForeignInvestmentIdentityNumber,
    #[serde(rename = "TXID")]
    TaxIdentificationNumber,
    #[serde(rename = "SOCS")]
    SocialSecurityNumber,
    #[serde(rename = "IDCD")]
    IdentityCardNumber,
    #[serde(rename = "LEIX")]
    LegalEntityIdentifier,
    #[serde(rename = "MISC")]
    Unspecified,
}

pub trait Validatable {
    fn validate(&self) -> Result<(), Error>;
}

#[derive(thiserror::Error, Debug, PartialEq, Eq)]
pub enum Error {
    #[error("Validation error: {0}")]
    ValidationError(String),
    #[error("invalid country code: {0}")]
    InvalidCountryCode(String),
}

impl From<&str> for Error {
    fn from(value: &str) -> Self {
        Self::ValidationError(value.to_owned())
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
                legal_person_name_identifier_type: LegalPersonNameTypeCode::Legal,
            }
        }
    }

    impl NationalIdentification {
        fn mock() -> Self {
            Self {
                national_identifier: "id".try_into().unwrap(),
                national_identifier_type: NationalIdentifierTypeCode::Unspecified,
                country_of_issue: None,
                registration_authority: Some("RA000001".try_into().unwrap()),
            }
        }
    }

    impl Address {
        fn mock() -> Self {
            Self {
                address_type: AddressTypeCode::Residential,
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
                name_identifier_type: NaturalPersonNameTypeCode::LegalName,
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
            &NaturalPersonNameTypeCode::Alias,
            &[Token::UnitVariant {
                name: "NaturalPersonNameTypeCode",
                variant: "ALIA",
            }],
        );
        assert_tokens(
            &LegalPersonNameTypeCode::Legal,
            &[Token::UnitVariant {
                name: "LegalPersonNameTypeCode",
                variant: "LEGL",
            }],
        );
        assert_tokens(
            &AddressTypeCode::Business,
            &[Token::UnitVariant {
                name: "AddressTypeCode",
                variant: "BIZZ",
            }],
        );
        assert_tokens(
            &NationalIdentifierTypeCode::AlienRegistrationNumber,
            &[Token::UnitVariant {
                name: "NationalIdentifierTypeCode",
                variant: "ARNU",
            }],
        );
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
            legal_person_name_identifier_type: LegalPersonNameTypeCode::Short,
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
            name_identifier_type: NaturalPersonNameTypeCode::Alias,
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
            name_identifier_type: NaturalPersonNameTypeCode::LegalName,
        }
        .into();
        name.validate().unwrap();
    }

    #[test]
    fn test_c7_validation_error() {
        let mut person = LegalPerson::mock();
        let mut id = NationalIdentification::mock();

        for code in [
            NationalIdentifierTypeCode::AlienRegistrationNumber,
            NationalIdentifierTypeCode::PassportNumber,
            NationalIdentifierTypeCode::DriverLicenseNumber,
            NationalIdentifierTypeCode::ForeignInvestmentIdentityNumber,
            NationalIdentifierTypeCode::IdentityCardNumber,
            NationalIdentifierTypeCode::SocialSecurityNumber,
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
            NationalIdentifierTypeCode::LegalEntityIdentifier,
            NationalIdentifierTypeCode::Unspecified,
            NationalIdentifierTypeCode::RegistrationAuthorityIdentifier,
            NationalIdentifierTypeCode::TaxIdentificationNumber,
        ] {
            let mut id = NationalIdentification::mock();
            id.national_identifier_type = code.clone();
            if code == NationalIdentifierTypeCode::LegalEntityIdentifier {
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
        ni.country_of_issue = Some("CH".try_into().unwrap());
        let mut person = LegalPerson::mock();
        person.national_identification = Some(ni.clone());
        match_validation_error(&person, 9);

        ni.national_identifier_type = NationalIdentifierTypeCode::LegalEntityIdentifier;
        // Use a valid LEI to make C11 pass
        ni.national_identifier = "2594007XIACKNMUAW223".try_into().unwrap();
        person.national_identification = Some(ni.clone());
        match_validation_error(&person, 9);

        ni.national_identifier_type = NationalIdentifierTypeCode::Unspecified;
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
        ni.national_identifier_type = NationalIdentifierTypeCode::LegalEntityIdentifier;
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
        ni.national_identifier_type = NationalIdentifierTypeCode::LegalEntityIdentifier;
        ni.national_identifier = "invalid-lei".try_into().unwrap();
        person.national_identification = Some(ni);
        match_validation_error(&person, 11);
    }

    #[test]
    fn test_c11_validation_pass() {
        let mut person = LegalPerson::mock();
        let mut ni = NationalIdentification::mock();
        ni.registration_authority = None;
        ni.national_identifier_type = NationalIdentifierTypeCode::LegalEntityIdentifier;
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
