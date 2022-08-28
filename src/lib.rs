pub use gregorian::Date;

const AVERAGE_MONTH_DAYS: f64 = 365.2425 / 12.0;
const DAYS_IN_WEEK: i32 = 7;
const WEEKS_IN_YEAR: i32 = 52;

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
    lifespan_days(birthday, lifespan_years) / DAYS_IN_WEEK
}

/// Compute the estimated lifespan in months, given a lifespan in years.
#[allow(clippy::cast_possible_truncation)]
#[must_use]
pub fn lifespan_months(birthday: Date, lifespan_years: i16) -> i32 {
    (f64::from(lifespan_days(birthday, lifespan_years)) / AVERAGE_MONTH_DAYS) as i32
}

/// Compute the number of days lived since birth.
#[must_use]
pub const fn days_lived(today: Date, birthday: Date) -> i32 {
    Date::days_since(birthday, today)
}

/// Compute the number of weeks lived since birth.
#[must_use]
pub const fn weeks_lived(today: Date, birthday: Date) -> i32 {
    days_lived(today, birthday) / DAYS_IN_WEEK
}

/// Compute the number of months lived since birth.
#[allow(clippy::cast_possible_truncation)]
#[must_use]
pub fn months_lived(today: Date, birthday: Date) -> i32 {
    (f64::from(days_lived(today, birthday)) / AVERAGE_MONTH_DAYS) as i32
}

/// Compute the number of years lived since birth.
#[must_use]
pub const fn years_lived(today: Date, birthday: Date) -> i32 {
    weeks_lived(today, birthday) / WEEKS_IN_YEAR
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
    days / DAYS_IN_WEEK
}

/// Compute the estimated number of months of life remaining, given a lifespan in years.
#[allow(clippy::cast_possible_truncation)]
#[must_use]
pub fn months_left(today: Date, birthday: Date, lifespan_years: i16) -> i32 {
    let days = Date::days_since(today, death_day(birthday, lifespan_years));
    (f64::from(days) / AVERAGE_MONTH_DAYS) as i32
}

/// Compute the estimated number of weeks of life remaining, given a lifespan in years.
#[must_use]
pub const fn years_left(today: Date, birthday: Date, lifespan_years: i16) -> i32 {
    let weeks = weeks_left(today, birthday, lifespan_years);
    weeks / WEEKS_IN_YEAR
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
        assert_eq!(
            lifespan_days(Date::new(1998, 8, 15).unwrap(), 1000),
            365_243
        );
    }
}
