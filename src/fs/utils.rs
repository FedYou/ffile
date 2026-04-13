use std::time::SystemTime;

use time::{format_description::parse, OffsetDateTime, UtcOffset};

pub fn system_time_string(system_time: SystemTime) -> String {
    let format =
        parse("[weekday repr:short], [day] [month repr:short] [year] [hour]:[minute]:[second]")
            .unwrap();

    let utc = OffsetDateTime::from(system_time);

    let local = utc.to_offset(UtcOffset::current_local_offset().unwrap_or(UtcOffset::UTC));

    return local.format(&format).expect(
        &format!(
            "Unable to convert System Time to string. '{:?}' ",
            system_time
        )
        .to_string(),
    );
}

pub fn bytes_to_size_string(bytes: u64) -> String {
    if bytes < 1024 * 1024 {
        return format!("{:.2} kB", bytes as f32 / 1024.0);
    } else if bytes < 1024 * 1024 * 1024 {
        return format!("{:.2} MB", bytes as f32 / (1024.0 * 1024.0));
    } else if bytes < 1024 * 1024 * 1024 * 1024 {
        return format!("{:.2} GB", bytes as f32 / (1024.0 * 1024.0 * 1024.0));
    }

    return format!("{:.2}B", bytes);
}
