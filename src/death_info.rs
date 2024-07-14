use death_calendar::{
	days_left, days_lived, death_day, lifespan_days, lifespan_months, lifespan_weeks, months_left,
	months_lived, weeks_left, weeks_lived, years_left, years_lived,
};
use gregorian::Date;

#[allow(clippy::cast_possible_wrap)]
const fn truncate_u16(num: u16) -> i16 {
	if num > 32767 {
		i16::MAX
	} else {
		num as i16
	}
}

#[allow(clippy::uninlined_format_args)]
pub fn info(bday: Date, lifespan_years: u16) -> String {
	let years = truncate_u16(lifespan_years);
	let today = Date::today_utc();
	format!(
		r#"You will live for approximately:
• {lifespan_days} days
• {lifespan_weeks} weeks
• {lifespan_months} months
• {years} years

You will probably die around {death_day}.

You have lived for:
• {days_lived} days
• {weeks_lived} weeks
• {months_lived} months
• {years_lived} years

You have remaining:
• {days_left} days
• {weeks_left} weeks
• {months_left} months
• {years_left} years"#,
		lifespan_days = lifespan_days(bday, years),
		lifespan_weeks = lifespan_weeks(years),
		lifespan_months = lifespan_months(years),
		death_day = death_day(bday, years),
		days_lived = days_lived(today, bday),
		weeks_lived = weeks_lived(today, bday),
		months_lived = months_lived(today, bday),
		years_lived = years_lived(today, bday),
		days_left = days_left(today, bday, years).abs(),
		weeks_left = weeks_left(today, bday, years).abs(),
		months_left = months_left(today, bday, years).abs(),
		years_left = years_left(today, bday, years).abs(),
	)
}
