use chrono::prelude::*;
pub fn get_date() -> String {
    let datetime: DateTime<Local> = Local::now().into();
    datetime.date_naive().to_string()
}
