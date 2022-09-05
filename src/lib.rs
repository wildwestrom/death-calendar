use gregorian::Date;
use gregorian::DateResultExt;

/// Compute the estimated day you will die.
#[must_use]
#[inline]
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
#[inline]
pub const fn lifespan_days(birthday: Date, lifespan_years: i16) -> i32 {
    let deathday = death_day(birthday, lifespan_years);
    Date::days_since(birthday, deathday)
}

/// Compute the estimated lifespan in weeks, given a lifespan in years.
#[must_use]
#[inline]
pub fn lifespan_weeks(lifespan_years: i16) -> i32 {
    (lifespan_years * 52).into()
}

/// Compute the estimated lifespan in months, given a lifespan in years.
#[must_use]
#[inline]
pub fn lifespan_months(lifespan_years: i16) -> i32 {
    (lifespan_years * 12).into()
}

/// Compute the number of days lived since birth.
#[must_use]
#[inline]
pub const fn days_lived(today: Date, birthday: Date) -> i32 {
    Date::days_since(birthday, today)
}

/// Compute the number of weeks lived since birth.
#[must_use]
#[inline]
pub const fn weeks_lived(today: Date, birthday: Date) -> i32 {
    days_lived(today, birthday) / 7_i32
}

/// Compute the number of months lived since birth.
#[must_use]
#[inline]
pub fn months_lived(today: Date, birthday: Date) -> i32 {
    let mut inc = 0_i32;
    while birthday.add_months(inc).or_prev_valid() < today {
        inc += 1_i32;
    }
    inc
}

/// Compute the number of years lived since birth.
#[must_use]
#[inline]
pub fn years_lived(today: Date, birthday: Date) -> i32 {
    let mut year_inc = 0_i16;
    let mut new_date;
    while birthday.add_years(year_inc).or_prev_valid() < today {
        year_inc += 1_i16;
    }
    new_date = birthday.add_years(year_inc).or_prev_valid();
    let mut day_inc = 0_i32;
    while today < new_date {
        new_date = new_date.sub_days(day_inc);
        day_inc += 1_i32;
    }
    if day_inc > 0_i32 {
        year_inc -= 1_i16;
    };
    year_inc.into()
}

/// Compute the estimated number of days of life remaining, given a lifespan in years.
#[must_use]
#[inline]
pub const fn days_left(today: Date, birthday: Date, lifespan_years: i16) -> i32 {
    Date::days_since(today, death_day(birthday, lifespan_years))
}

/// Compute the estimated number of weeks of life remaining, given a lifespan in years.
#[must_use]
#[inline]
pub fn weeks_left(today: Date, birthday: Date, lifespan_years: i16) -> i32 {
    let mut inc = 0_i16;
    while birthday.add_years(inc).or_next_valid() < today {
        inc += 1_i16;
    }
    i32::from(lifespan_years - inc) * 52_i32
}

/// Compute the estimated number of months of life remaining, given a lifespan in years.
#[must_use]
#[inline]
pub fn months_left(today: Date, birthday: Date, lifespan_years: i16) -> i32 {
    let mut inc = 0_i32;
    while birthday.add_months(inc).or_next_valid() < today {
        inc += 1_i32;
    }
    i32::from(lifespan_years * 12_i16) - inc
}

/// Compute the estimated number of weeks of life remaining, given a lifespan in years.
#[must_use]
#[inline]
pub fn years_left(today: Date, birthday: Date, lifespan_years: i16) -> i32 {
    let mut inc = 0;
    while birthday.add_years(inc).or_next_valid() < today {
        inc += 1;
    }
    (lifespan_years - inc).into()
}

#[cfg(test)]
mod tests {
    use super::*;

    const AVERAGE_DAYS_IN_YEAR: f64 = 365.2425;

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
        assert_eq!(
            death_day(Date::new(1, 1, 1).unwrap(), 100),
            Date::new(101, 1, 1).unwrap(),
        );
    }

    #[test]
    fn given_birthday_and_lifespan_return_lifespan() {
        // Months
        assert_eq!(lifespan_months(100), (12 * 100));
        assert_eq!(lifespan_months(75), (12 * 75));
        // Weeks
        assert_eq!(lifespan_weeks(100), (52 * 100));
        assert_eq!(lifespan_weeks(75), (52 * 75));
        // Days
        assert_eq!(lifespan_days(Date::new(2000, 1, 1).unwrap(), 80), 29220);
        assert_eq!(lifespan_days(Date::new(1996, 2, 29).unwrap(), 99), 36159);
        assert_eq!(
            lifespan_days(Date::new(1998, 8, 15).unwrap(), 1000),
            365_243
        );
    }

    #[test]
    fn given_birthday_lifespan_and_date_after_birth_return_life_lived() {
        // Years
        assert_eq!(
            years_lived(Date::new(2000, 1, 1).unwrap(), Date::new(0, 1, 1).unwrap()),
            2000
        );
        assert_eq!(
            years_lived(
                Date::new(1999, 12, 31).unwrap(),
                Date::new(0, 1, 1).unwrap()
            ),
            1999
        );
        assert_eq!(
            years_lived(
                Date::new(2100, 1, 1).unwrap(),
                Date::new(2000, 1, 1).unwrap()
            ),
            100
        );
        assert_eq!(
            years_lived(
                Date::new(2099, 12, 31).unwrap(),
                Date::new(2000, 1, 1).unwrap()
            ),
            99
        );
        assert_eq!(
            years_lived(
                Date::new(2100, 3, 3).unwrap(),
                Date::new(2000, 3, 3).unwrap()
            ),
            100
        );
        assert_eq!(
            years_lived(
                Date::new(2100, 3, 2).unwrap(),
                Date::new(2000, 3, 3).unwrap()
            ),
            99
        );
        // Months
        assert_eq!(
            months_lived(
                Date::new(2100, 1, 1).unwrap(),
                Date::new(2000, 1, 1).unwrap()
            ),
            (100 * 12)
        );
        // Weeks
        assert_eq!(
            weeks_lived(
                Date::new(2000, 1, 7).unwrap(),
                Date::new(2000, 1, 1).unwrap()
            ),
            0
        );
        assert_eq!(
            weeks_lived(
                Date::new(2000, 1, 8).unwrap(),
                Date::new(2000, 1, 1).unwrap()
            ),
            1
        );
        assert_eq!(
            weeks_lived(
                Date::new(2000, 12, 31).unwrap(),
                Date::new(2000, 1, 1).unwrap()
            ),
            52
        );
        assert_eq!(
            weeks_lived(
                Date::new(2001, 1, 1).unwrap(),
                Date::new(2000, 1, 1).unwrap()
            ),
            52
        );
        // Days
        assert_eq!(
            days_lived(
                Date::new(2000, 1, 2).unwrap(),
                Date::new(2000, 1, 1).unwrap()
            ),
            1
        );
        assert_eq!(
            days_lived(
                Date::new(2000, 2, 1).unwrap(),
                Date::new(2000, 1, 1).unwrap()
            ),
            31
        );
        assert_eq!(
            days_lived(
                Date::new(2010, 1, 1).unwrap(),
                Date::new(2000, 1, 1).unwrap()
            ),
            (10.0 * AVERAGE_DAYS_IN_YEAR) as i32 + 1
        );
        assert_eq!(
            days_lived(
                Date::new(2100, 1, 1).unwrap(),
                Date::new(2000, 1, 1).unwrap()
            ),
            (100.0 * AVERAGE_DAYS_IN_YEAR) as i32 + 1
        );
        assert_eq!(
            days_lived(
                Date::new(3000, 1, 1).unwrap(),
                Date::new(2000, 1, 1).unwrap()
            ),
            (1000.0 * AVERAGE_DAYS_IN_YEAR) as i32 + 1
        );
    }

    #[test]
    fn given_birthday_lifespan_and_date_after_birth_return_life_remaining() {
        // Years
        assert_eq!(
            years_left(
                Date::new(2050, 1, 1).unwrap(),
                Date::new(2000, 1, 1).unwrap(),
                100
            ),
            50,
        );
        assert_eq!(
            years_left(
                Date::new(2020, 1, 1).unwrap(),
                Date::new(2000, 1, 1).unwrap(),
                100
            ),
            80,
        );
        assert_eq!(
            years_left(
                Date::new(2000, 1, 1).unwrap(),
                Date::new(0, 1, 1).unwrap(),
                100
            ),
            -1900,
        );
        // Months
        assert_eq!(
            months_left(
                Date::new(2050, 1, 1).unwrap(),
                Date::new(2000, 1, 1).unwrap(),
                100
            ),
            50 * 12,
        );
        assert_eq!(
            months_left(
                Date::new(2000, 1, 1).unwrap(),
                Date::new(0, 1, 1).unwrap(),
                100
            ),
            -1900 * 12,
        );
        // Weeks
        assert_eq!(
            weeks_left(
                Date::new(2050, 1, 1).unwrap(),
                Date::new(2000, 1, 1).unwrap(),
                100
            ),
            50 * 52
        );
        assert_eq!(
            weeks_left(
                Date::new(2099, 1, 1).unwrap(),
                Date::new(2000, 1, 1).unwrap(),
                100
            ),
            52
        );
        assert_eq!(
            weeks_left(
                Date::new(2000, 1, 1).unwrap(),
                Date::new(0, 1, 1).unwrap(),
                100
            ),
            -1900 * 52
        );
        // Days
        assert_eq!(
            days_left(
                Date::new(2050, 1, 1).unwrap(),
                Date::new(2000, 1, 1).unwrap(),
                100
            ),
            (50.0 * AVERAGE_DAYS_IN_YEAR) as i32,
        );
        assert_eq!(
            days_left(
                Date::new(2000, 1, 1).unwrap(),
                Date::new(0, 1, 1).unwrap(),
                100
            ),
            (-1900.0 * AVERAGE_DAYS_IN_YEAR) as i32,
        );
    }
}
