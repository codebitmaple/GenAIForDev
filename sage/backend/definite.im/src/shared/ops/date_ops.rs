use chrono::{DateTime, Local, NaiveDate, NaiveDateTime, NaiveTime, TimeZone, Utc};

pub fn local_date_time() -> NaiveDateTime {
    Local::now().naive_local()
}

pub fn add_days(
    date: NaiveDate,
    days: i64,
) -> NaiveDate {
    date + chrono::Duration::days(days)
}

pub fn local_date() -> NaiveDate {
    Local::now().naive_local().date()
}

pub fn to_display_date(date: NaiveDateTime) -> String {
    Local.from_local_datetime(&date).unwrap().naive_local().format("%v").to_string()
}

pub fn to_input_date_from(date: NaiveDateTime) -> String {
    Local.from_local_datetime(&date).unwrap().naive_local().format("%Y-%m-%d").to_string()
}

pub fn to_input_date() -> String {
    Local::now().naive_local().format("%Y-%m-%d").to_string()
}

pub fn to_timestamp_from(date: NaiveDateTime) -> String {
    Local.from_local_datetime(&date).unwrap().timestamp().to_string()
}

pub fn from(timestamp: i64) -> NaiveDateTime {
    DateTime::from_timestamp(timestamp, 0).unwrap().naive_local()
}

pub fn to_timestamp() -> i64 {
    Local::now().timestamp()
}

pub fn to_year_only() -> String {
    let date = from(to_timestamp());
    let target_format = "%Y";
    date.format(target_format).to_string()
}

pub fn format_date(
    date_str: &str,
    source_format: Option<&str>,
    target_format: Option<&str>,
) -> String {
    // Default formats
    let source_format = source_format.unwrap_or("%Y-%m-%d");
    let target_format = target_format.unwrap_or("%A, %B %e, %Y");

    // Parse the date using the source format
    let naive_date = NaiveDate::parse_from_str(date_str, source_format).expect("Failed to parse date");

    // Format the date using the target format
    naive_date.format(target_format).to_string()
}

pub fn format_time(time_str: &str) -> String {
    // Parse the time string into a NaiveTime
    let naive_time = NaiveTime::parse_from_str(time_str, "%H:%M").expect("Failed to parse time");

    // Format the time as "9 AM" or "5:45 PM"
    naive_time.format("%-I:%M %p").to_string()
}

pub fn format_time_from(time: &NaiveTime) -> String {
    // Format the time as "9 AM" or "5:45 PM"
    time.format("%-I:%M %p").to_string()
}

pub fn days_ago(timestamp: u64) -> String {
    // Convert the timestamp to a NaiveDateTime
    let naive_datetime = DateTime::<Utc>::from_timestamp(timestamp as i64, 0).map(|dt| dt.naive_utc());
    if let Some(naive_datetime) = naive_datetime {
        // Get the current UTC time
        let now = Utc::now().naive_utc();
        // Calculate the duration between now and the provided timestamp
        let duration = now.signed_duration_since(naive_datetime);
        // Calculate the number of days
        let days = duration.num_days();

        if days == 0 {
            return "Today".to_string();
        } else if days == 1 {
            return "Yesterday".to_string();
        }
        // Return the formatted string
        format!("{} days ago", days)
    } else {
        "Invalid timestamp".to_string()
    }
}
