// Copyright (c) 2025 Michael A Wright
// Licensed under the MIT License

use std::time::SystemTime;

#[rustfmt::skip]
pub fn format_time_ago(now: SystemTime, then: SystemTime) -> String {
    let secs = now.duration_since(then).map(|d| d.as_secs()).unwrap_or(0);
    let p = |n: u64| if n == 1 { "" } else { "s" };
    if secs < 60 { return format!("{} seconds ago", secs); }
    let mins = secs / 60;
    if mins < 60 { return format!("{} minute{} ago", mins, p(mins)); }
    let hours = mins / 60;
    if hours < 24 { return format!("{} hour{} ago", hours, p(hours)); }
    let days = hours / 24;
    if days < 7 { return format!("{} day{} ago", days, p(days)); }
    if days < 30 { return format!("{} week{} ago", days / 7, p(days / 7)); }
    if days < 365 { return format!("{} month{} ago", days / 30, p(days / 30)); }
    format!("{} year{} ago", days / 365, p(days / 365))
}
