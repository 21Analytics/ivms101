/// A ISO 3166-1 Alpha-2 country code.
#[derive(Clone, Debug, PartialEq, Eq, serde::Deserialize)]
#[serde(try_from = "&str")]
pub struct CountryCode {
    inner: String,
}

impl serde::Serialize for CountryCode {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.inner.serialize(serializer)
    }
}

impl TryFrom<&str> for CountryCode {
    type Error = crate::Error;
    fn try_from(from: &str) -> Result<Self, Self::Error> {
        // XX represents an unknown state or entity
        // https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2
        if iso3166_1::alpha2(from).is_none() && from != "XX" {
            return Err(crate::Error::InvalidCountryCode(from.to_string()));
        }
        Ok(Self { inner: from.into() })
    }
}

impl CountryCode {
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.inner
    }
}

#[must_use]
#[allow(clippy::too_many_lines)]
pub fn country(country_code: &str) -> &str {
    COUNTRY_CODES
        .get_or_init(|| {
            [
                ("af", "Afghanistan"),
                ("ax", "Aland Islands"),
                ("al", "Albania"),
                ("dz", "Algeria"),
                ("as", "American Samoa"),
                ("ad", "Andorra"),
                ("ao", "Angola"),
                ("ai", "Anguilla"),
                ("aq", "Antarctica"),
                ("ag", "Antigua And Barbuda"),
                ("ar", "Argentina"),
                ("am", "Armenia"),
                ("aw", "Aruba"),
                ("au", "Australia"),
                ("at", "Austria"),
                ("az", "Azerbaijan"),
                ("bs", "Bahamas"),
                ("bh", "Bahrain"),
                ("bd", "Bangladesh"),
                ("bb", "Barbados"),
                ("by", "Belarus"),
                ("be", "Belgium"),
                ("bz", "Belize"),
                ("bj", "Benin"),
                ("bm", "Bermuda"),
                ("bt", "Bhutan"),
                ("bo", "Bolivia"),
                ("ba", "Bosnia And Herzegovina"),
                ("bw", "Botswana"),
                ("bv", "Bouvet Island"),
                ("br", "Brazil"),
                ("io", "British Indian Ocean Territory"),
                ("bn", "Brunei Darussalam"),
                ("bg", "Bulgaria"),
                ("bf", "Burkina Faso"),
                ("bi", "Burundi"),
                ("kh", "Cambodia"),
                ("cm", "Cameroon"),
                ("ca", "Canada"),
                ("cv", "Cape Verde"),
                ("ky", "Cayman Islands"),
                ("cf", "Central African Republic"),
                ("td", "Chad"),
                ("cl", "Chile"),
                ("cn", "China"),
                ("cx", "Christmas Island"),
                ("cc", "Cocos (Keeling) Islands"),
                ("co", "Colombia"),
                ("km", "Comoros"),
                ("cg", "Congo"),
                ("cd", "Congo, Democratic Republic"),
                ("ck", "Cook Islands"),
                ("cr", "Costa Rica"),
                ("ci", "Cote D'Ivoire"),
                ("hr", "Croatia"),
                ("cu", "Cuba"),
                ("cy", "Cyprus"),
                ("cz", "Czech Republic"),
                ("dk", "Denmark"),
                ("dj", "Djibouti"),
                ("dm", "Dominica"),
                ("do", "Dominican Republic"),
                ("ec", "Ecuador"),
                ("eg", "Egypt"),
                ("sv", "El Salvador"),
                ("gq", "Equatorial Guinea"),
                ("er", "Eritrea"),
                ("ee", "Estonia"),
                ("et", "Ethiopia"),
                ("fk", "Falkland Islands (Malvinas)"),
                ("fo", "Faroe Islands"),
                ("fj", "Fiji"),
                ("fi", "Finland"),
                ("fr", "France"),
                ("gf", "French Guiana"),
                ("pf", "French Polynesia"),
                ("tf", "French Southern Territories"),
                ("ga", "Gabon"),
                ("gm", "Gambia"),
                ("ge", "Georgia"),
                ("de", "Germany"),
                ("gh", "Ghana"),
                ("gi", "Gibraltar"),
                ("gr", "Greece"),
                ("gl", "Greenland"),
                ("gd", "Grenada"),
                ("gp", "Guadeloupe"),
                ("gu", "Guam"),
                ("gt", "Guatemala"),
                ("gg", "Guernsey"),
                ("gn", "Guinea"),
                ("gw", "Guinea-Bissau"),
                ("gy", "Guyana"),
                ("ht", "Haiti"),
                ("hm", "Heard Island & Mcdonald Islands"),
                ("va", "Holy See (Vatican City State)"),
                ("hn", "Honduras"),
                ("hk", "Hong Kong"),
                ("hu", "Hungary"),
                ("is", "Iceland"),
                ("in", "India"),
                ("id", "Indonesia"),
                ("ir", "Iran, Islamic Republic Of"),
                ("iq", "Iraq"),
                ("ie", "Ireland"),
                ("im", "Isle Of Man"),
                ("il", "Israel"),
                ("it", "Italy"),
                ("jm", "Jamaica"),
                ("jp", "Japan"),
                ("je", "Jersey"),
                ("jo", "Jordan"),
                ("kz", "Kazakhstan"),
                ("ke", "Kenya"),
                ("ki", "Kiribati"),
                ("kp", "Democratic People's Republic of Korea"),
                ("kr", "South Korea"),
                ("kw", "Kuwait"),
                ("kg", "Kyrgyzstan"),
                ("la", "Lao People's Democratic Republic"),
                ("lv", "Latvia"),
                ("lb", "Lebanon"),
                ("ls", "Lesotho"),
                ("lr", "Liberia"),
                ("ly", "Libyan Arab Jamahiriya"),
                ("li", "Liechtenstein"),
                ("lt", "Lithuania"),
                ("lu", "Luxembourg"),
                ("mo", "Macao"),
                ("mk", "Macedonia"),
                ("mg", "Madagascar"),
                ("mw", "Malawi"),
                ("my", "Malaysia"),
                ("mv", "Maldives"),
                ("ml", "Mali"),
                ("mt", "Malta"),
                ("mh", "Marshall Islands"),
                ("mq", "Martinique"),
                ("mr", "Mauritania"),
                ("mu", "Mauritius"),
                ("yt", "Mayotte"),
                ("mx", "Mexico"),
                ("fm", "Micronesia, Federated States Of"),
                ("md", "Moldova"),
                ("mc", "Monaco"),
                ("mn", "Mongolia"),
                ("me", "Montenegro"),
                ("ms", "Montserrat"),
                ("ma", "Morocco"),
                ("mz", "Mozambique"),
                ("mm", "Myanmar"),
                ("na", "Namibia"),
                ("nr", "Nauru"),
                ("np", "Nepal"),
                ("nl", "Netherlands"),
                ("an", "Netherlands Antilles"),
                ("nc", "New Caledonia"),
                ("nz", "New Zealand"),
                ("ni", "Nicaragua"),
                ("ne", "Niger"),
                ("ng", "Nigeria"),
                ("nu", "Niue"),
                ("nf", "Norfolk Island"),
                ("mp", "Northern Mariana Islands"),
                ("no", "Norway"),
                ("om", "Oman"),
                ("pk", "Pakistan"),
                ("pw", "Palau"),
                ("ps", "Palestinian Territory, Occupied"),
                ("pa", "Panama"),
                ("pg", "Papua New Guinea"),
                ("py", "Paraguay"),
                ("pe", "Peru"),
                ("ph", "Philippines"),
                ("pn", "Pitcairn"),
                ("pl", "Poland"),
                ("pt", "Portugal"),
                ("pr", "Puerto Rico"),
                ("qa", "Qatar"),
                ("re", "Reunion"),
                ("ro", "Romania"),
                ("ru", "Russian Federation"),
                ("rw", "Rwanda"),
                ("bl", "Saint Barthelemy"),
                ("sh", "Saint Helena"),
                ("kn", "Saint Kitts And Nevis"),
                ("lc", "Saint Lucia"),
                ("mf", "Saint Martin"),
                ("pm", "Saint Pierre And Miquelon"),
                ("vc", "Saint Vincent And Grenadines"),
                ("ws", "Samoa"),
                ("sm", "San Marino"),
                ("st", "Sao Tome And Principe"),
                ("sa", "Saudi Arabia"),
                ("sn", "Senegal"),
                ("rs", "Serbia"),
                ("sc", "Seychelles"),
                ("sl", "Sierra Leone"),
                ("sg", "Singapore"),
                ("sk", "Slovakia"),
                ("si", "Slovenia"),
                ("sb", "Solomon Islands"),
                ("so", "Somalia"),
                ("za", "South Africa"),
                ("gs", "South Georgia And Sandwich Isl."),
                ("es", "Spain"),
                ("lk", "Sri Lanka"),
                ("sd", "Sudan"),
                ("sr", "Suriname"),
                ("sj", "Svalbard And Jan Mayen"),
                ("sz", "Swaziland"),
                ("se", "Sweden"),
                ("ch", "Switzerland"),
                ("sy", "Syrian Arab Republic"),
                ("tw", "Taiwan"),
                ("tj", "Tajikistan"),
                ("tz", "Tanzania"),
                ("th", "Thailand"),
                ("tl", "Timor-Leste"),
                ("tg", "Togo"),
                ("tk", "Tokelau"),
                ("to", "Tonga"),
                ("tt", "Trinidad And Tobago"),
                ("tn", "Tunisia"),
                ("tr", "Turkey"),
                ("tm", "Turkmenistan"),
                ("tc", "Turks And Caicos Islands"),
                ("tv", "Tuvalu"),
                ("ug", "Uganda"),
                ("ua", "Ukraine"),
                ("ae", "United Arab Emirates"),
                ("gb", "United Kingdom"),
                ("us", "United States"),
                ("um", "United States Outlying Islands"),
                ("uy", "Uruguay"),
                ("uz", "Uzbekistan"),
                ("vu", "Vanuatu"),
                ("ve", "Venezuela"),
                ("vn", "Viet Nam"),
                ("vg", "Virgin Islands, British"),
                ("vi", "Virgin Islands, U.S."),
                ("wf", "Wallis And Futuna"),
                ("eh", "Western Sahara"),
                ("ye", "Yemen"),
                ("zm", "Zambia"),
                ("zw", "Zimbabwe"),
            ]
            .into()
        })
        .get(country_code)
        .copied()
        .unwrap_or(country_code)
}

static COUNTRY_CODES: std::sync::OnceLock<std::collections::HashMap<&'static str, &'static str>> =
    std::sync::OnceLock::new();

#[cfg(test)]
mod tests {
    use super::CountryCode;
    use serde_test::{assert_tokens, Token};

    #[test]
    fn test_country_code() {
        let de = CountryCode { inner: "DE".into() };
        assert_tokens(&de, &[Token::BorrowedStr("DE")]);
    }

    #[test]
    fn test_country_code_unknown_placeholder() {
        CountryCode::try_from("XX").unwrap();
    }

    #[test]
    fn test_country_code_invalid_length() {
        serde_test::assert_de_tokens_error::<CountryCode>(
            &[Token::BorrowedStr("C")],
            "invalid country code: C",
        );
        serde_test::assert_de_tokens_error::<CountryCode>(
            &[Token::BorrowedStr("CHE")],
            "invalid country code: CHE",
        );
    }

    #[test]
    fn test_invalid_country_code() {
        assert!(CountryCode::try_from("RR").is_err());
    }
}
