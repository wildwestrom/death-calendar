pub use gregorian::Date;

/// Compute the estimated day you will die.
#[must_use]
pub const fn death_day(birthday: Date, lifespan_years: i16) -> Date {
    match birthday
        .year_month()
        .add_years(lifespan_years)
        .with_day(birthday.day())
    {
        Ok(date) => date,
        Err(wrongdate) => wrongdate.prev_valid(),
    }
}

/// Compute the estimated lifespan in days, given a lifespan in years.
#[must_use]
pub const fn lifespan_days(birthday: Date, lifespan_years: i16) -> i32 {
    let deathday = death_day(birthday, lifespan_years);
    Date::days_since(birthday, deathday)
}

/// Compute the estimated lifespan in weeks, given a lifespan in years.
#[must_use]
pub const fn lifespan_weeks(birthday: Date, lifespan_years: i16) -> i32 {
    lifespan_days(birthday, lifespan_years) / 7
}

/// Compute the number of days lived since birth.
#[must_use]
pub const fn days_lived(today: Date, birthday: Date) -> i32 {
    Date::days_since(birthday, today)
}

/// Compute the number of weeks lived since birth.
#[must_use]
pub const fn weeks_lived(today: Date, birthday: Date) -> i32 {
    days_lived(today, birthday) / 7
}

/// Compute the number of years lived since birth.
#[must_use]
pub const fn years_lived(today: Date, birthday: Date) -> i32 {
    weeks_lived(today, birthday) / 52
}

/// Compute the estimated number of days of life remaining, given a lifespan in years.
#[must_use]
pub const fn days_left(today: Date, birthday: Date, lifespan_years: i16) -> i32 {
    Date::days_since(today, death_day(birthday, lifespan_years))
}

/// Compute the estimated number of weeks of life remaining, given a lifespan in years.
#[must_use]
pub const fn weeks_left(today: Date, birthday: Date, lifespan_years: i16) -> i32 {
    let days = Date::days_since(today, death_day(birthday, lifespan_years));
    days / 7
}

/// Compute the estimated number of weeks of life remaining, given a lifespan in years.
#[must_use]
pub const fn years_left(today: Date, birthday: Date, lifespan_years: i16) -> i32 {
    let weeks = weeks_left(today, birthday, lifespan_years);
    weeks / 52
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn given_birthday_and_lifespan_return_death_day() {
        assert_eq!(
            death_day(Date::new(2000, 1, 1).unwrap(), 80),
            Date::new(2080, 1, 1).unwrap(),
        );
        assert_eq!(
            death_day(Date::new(1996, 2, 29).unwrap(), 99),
            Date::new(2095, 2, 28).unwrap(),
        );
        assert_eq!(
            death_day(Date::new(1998, 8, 15).unwrap(), 1000),
            Date::new(2998, 8, 15).unwrap(),
        );
    }

    #[test]
    fn given_birthday_and_lifespan_return_lifespan_in_days() {
        assert_eq!(lifespan_days(Date::new(2000, 1, 1).unwrap(), 80), 29220);
        assert_eq!(lifespan_days(Date::new(1996, 2, 29).unwrap(), 99), 36159);
        assert_eq!(lifespan_days(Date::new(1998, 8, 15).unwrap(), 1000), 365243);
    }
}
