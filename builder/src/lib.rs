#[derive(Debug, Clone)]
pub struct PhoneNumberE164(pub String);

#[derive(Debug, Clone)]
pub struct Details {
    pub given_name: String,
    pub preferred_name: Option<String>,
    pub middle_name: Option<String>,
    pub family_name: String,
    pub mobile_phone: Option<PhoneNumberE164>,
    pub date_of_birth: time::Date,
    pub last_seen: Option<time::OffsetDateTime>,
}

pub struct DetailsBuilder(Details);

impl DetailsBuilder {
    /// Start building a new [`Details`] object.
    pub fn new(given_name: &str, family_name: &str, date_of_birth: time::Date) -> Self {
        DetailsBuilder(Details {
            given_name: given_name.to_owned(),
            preferred_name: None,
            middle_name: None,
            family_name: family_name.to_owned(),
            mobile_phone: None,
            date_of_birth,
            last_seen: None,
        })
    }

    /// Set the preferred name.
    pub fn preferred_name(&mut self, preferred_name: &str) -> &mut Self {
        self.0.preferred_name = Some(preferred_name.to_owned());
        self
    }

    /// Set the middle name.
    pub fn middle_name(&mut self, middle_name: &str) -> &mut Self {
        self.0.middle_name = Some(middle_name.to_owned());
        self
    }

    /// Update the `last_seen` field to the current date/time.
    pub fn just_seen(&mut self) -> &mut Self {
        self.0.last_seen = Some(time::OffsetDateTime::now_utc());
        self
    }

    /// Consume the builder object and return a fully built [`Details`]
    /// object.
    pub fn build(&self) -> Details {
        self.0.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builds_an_object() {
        let details = DetailsBuilder::new(
            "Robert",
            "Builder",
            time::Date::from_calendar_date(1998, time::Month::November, 28).unwrap(),
        )
        .middle_name("the")
        .preferred_name("Bob")
        .just_seen()
        .build();

        assert_eq!("Robert", details.given_name);
        assert_eq!(Some("Bob".to_string()), details.preferred_name);
    }

    #[test]
    fn build_is_separate_from_setters() {
        let mut builder = DetailsBuilder::new(
            "Robert",
            "Builder",
            time::Date::from_calendar_date(1998, time::Month::November, 28).unwrap(),
        );
        builder.preferred_name("Bob");

        let details = builder.build();

        assert_eq!("Robert", details.given_name);
        assert_eq!(Some("Bob".to_string()), details.preferred_name);
    }
}
