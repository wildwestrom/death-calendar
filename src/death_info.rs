use death_calendar::{
	days_left, days_lived, death_day, lifespan_days, lifespan_months, lifespan_weeks, months_left,
	months_lived, weeks_left, weeks_lived, years_left, years_lived,
};
use gregorian::Date;

#[allow(clippy::print_stdout)]
// #[allow(clippy::uninlined_format_args)] // Nightly only for now
pub fn show(bday: Date, years: i16) {
	let today: Date = Date::today_utc();
	println!("Your birthday is {}.", bday);
	println!();
	println!("You will live for approximately:");
	println!("- {} days", lifespan_days(bday, years));
	println!("- {} weeks", lifespan_weeks(years));
	println!("- {} months", lifespan_months(years));
	println!("- {} years", years);
	println!();
	println!("You will probably die around {}.", death_day(bday, years));
	println!("You have lived for:");
	println!("- {} days", days_lived(today, bday));
	println!("- {} weeks", weeks_lived(today, bday));
	println!("- {} months", months_lived(today, bday));
	println!("- {} years", years_lived(today, bday));
	println!();
	println!("You have remaining:");
	println!("- {} days", days_left(today, bday, years).abs());
	println!("- {} weeks", weeks_left(today, bday, years).abs());
	println!("- {} months", months_left(today, bday, years).abs());
	println!("- {} years", years_left(today, bday, years).abs());
}
