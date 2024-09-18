pub trait DateExtension {
    fn monday_of_last_week() -> Self;
    fn happened_last_week(&self) -> bool;
}

impl DateExtension for jiff::civil::Date {
    fn monday_of_last_week() -> Self {
        use jiff::civil::Weekday;
        use jiff::ToSpan;

        let today = jiff::Zoned::now().date();
        let a_day_last_week = today
            .checked_sub(1.week())
            .expect("Error getting a day last week");

        // Go back to the closest Monday.
        if a_day_last_week.weekday() == Weekday::Monday {
            a_day_last_week
        } else {
            a_day_last_week
                .nth_weekday(-1, Weekday::Monday)
                .expect("Error getting Monday of last week")
        }
    }

    fn happened_last_week(&self) -> bool {
        use jiff::civil::Weekday;

        let monday_of_last_week = Self::monday_of_last_week();

        // Go forward to the closest Sunday.
        let sunday_of_last_week = monday_of_last_week
            .nth_weekday(1, Weekday::Sunday)
            .expect("Problem getting Sunday of last week");

        &monday_of_last_week <= self && self <= &sunday_of_last_week
    }
}
