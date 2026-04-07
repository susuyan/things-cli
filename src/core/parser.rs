use chrono::{Datelike, Local, NaiveDate, NaiveDateTime, NaiveTime};

use super::models::When;
use super::ThingsError;

/// 解析 when 参数
pub fn parse_when(input: &str) -> Result<When, ThingsError> {
    let normalized = input.trim().to_lowercase();

    match normalized.as_str() {
        "today" | "今" | "今天" => Ok(When::Today),
        "tomorrow" | "tom" | "明" | "明天" => Ok(When::Tomorrow),
        "evening" | "今晚" => Ok(When::Evening),
        "anytime" | "任意时间" => Ok(When::Anytime),
        "someday" | "某天" => Ok(When::Someday),
        _ => {
            // 尝试解析为日期时间（带 @）
            if input.contains('@') {
                return parse_datetime(input);
            }

            // 尝试解析为日期: 2026-03-25
            if let Ok(date) = NaiveDate::parse_from_str(input, "%Y-%m-%d") {
                return Ok(When::Date(date));
            }

            // 尝试其他常见格式
            if let Ok(date) = NaiveDate::parse_from_str(input, "%Y/%m/%d") {
                return Ok(When::Date(date));
            }

            if let Ok(date) = NaiveDate::parse_from_str(input, "%m-%d") {
                // 假设是今年
                let today = Local::now().date_naive();
                let date = date.with_year(today.year()).unwrap_or(date);
                return Ok(When::Date(date));
            }

            // 解析自然语言
            parse_natural_language(input)
        }
    }
}

/// 解析日期时间格式: 2026-03-25@14:00
fn parse_datetime(input: &str) -> Result<When, ThingsError> {
    let parts: Vec<&str> = input.split('@').collect();
    if parts.len() != 2 {
        return Err(ThingsError::InvalidDate(format!(
            "Invalid datetime format: {}",
            input
        )));
    }

    let date = parse_date(parts[0])?;

    // 解析时间
    let time_str = parts[1].trim();
    let time = parse_time(time_str)?;

    let datetime = NaiveDateTime::new(date, time);
    Ok(When::DateTime(datetime))
}

/// 解析日期
fn parse_date(input: &str) -> Result<NaiveDate, ThingsError> {
    let input = input.trim();

    // yyyy-mm-dd
    if let Ok(date) = NaiveDate::parse_from_str(input, "%Y-%m-%d") {
        return Ok(date);
    }

    // yyyy/mm/dd
    if let Ok(date) = NaiveDate::parse_from_str(input, "%Y/%m/%d") {
        return Ok(date);
    }

    // mm-dd (当前年)
    if let Ok(date) = NaiveDate::parse_from_str(input, "%m-%d") {
        let today = Local::now().date_naive();
        if let Some(date) = date.with_year(today.year()) {
            // 如果日期已过，可能是明年
            if date < today {
                return date.with_year(today.year() + 1).ok_or_else(|| {
                    ThingsError::InvalidDate(format!("Invalid date: {}", input))
                });
            }
            return Ok(date);
        }
    }

    Err(ThingsError::InvalidDate(format!(
        "Cannot parse date: {}",
        input
    )))
}

/// 解析时间
fn parse_time(input: &str) -> Result<NaiveTime, ThingsError> {
    let input = input.trim().to_lowercase();

    // HH:MM (24小时制)
    if let Ok(time) = NaiveTime::parse_from_str(&input, "%H:%M") {
        return Ok(time);
    }

    // H:MM AM/PM
    if let Ok(time) = NaiveTime::parse_from_str(&input, "%I:%M%p") {
        return Ok(time);
    }

    if let Ok(time) = NaiveTime::parse_from_str(&input, "%I:%M %p") {
        return Ok(time);
    }

    // 简写如 6pm
    if let Ok(time) = NaiveTime::parse_from_str(&input, "%I%p") {
        return Ok(time);
    }

    Err(ThingsError::InvalidTime(format!(
        "Cannot parse time: {}",
        input
    )))
}

/// 解析自然语言
fn parse_natural_language(input: &str) -> Result<When, ThingsError> {
    let normalized = input.trim().to_lowercase();

    // in X days
    if let Some(captures) = regex::Regex::new(r"in\s+(\d+)\s+days?")
        .ok()
        .and_then(|re| re.captures(&normalized))
    {
        if let Some(days_str) = captures.get(1) {
            if let Ok(days) = days_str.as_str().parse::<i64>() {
                let date = Local::now() + chrono::Duration::days(days);
                return Ok(When::Date(date.date_naive()));
            }
        }
    }

    // next week
    if normalized.contains("next week") {
        let date = Local::now() + chrono::Duration::weeks(1);
        return Ok(When::Date(date.date_naive()));
    }

    // next month
    if normalized.contains("next month") {
        let today = Local::now();
        let next_month = today.date_naive() + chrono::Months::new(1);
        return Ok(When::Date(next_month));
    }

    // weekday names
    let weekdays = [
        ("monday", 1),
        ("tuesday", 2),
        ("wednesday", 3),
        ("thursday", 4),
        ("friday", 5),
        ("saturday", 6),
        ("sunday", 0),
    ];

    for (name, target_wd) in &weekdays {
        if normalized.contains(name) || normalized.contains(&name[..3]) {
            let today = Local::now();
            let current_wd = today.weekday().num_days_from_sunday() as i64;
            let target_wd = *target_wd as i64;

            let days_diff = if normalized.contains("next") {
                // 明确说 "next"
                (7 - current_wd + target_wd) % 7 + 7
            } else {
                // 未说 "next"，找下一个该星期几
                let diff = (target_wd - current_wd + 7) % 7;
                if diff == 0 { 7 } else { diff }
            };

            let date = today + chrono::Duration::days(days_diff);
            return Ok(When::Date(date.date_naive()));
        }
    }

    // 无法解析，作为自然语言字符串返回
    Ok(When::Natural(input.to_string()))
}

/// 解析逗号分隔的标签列表
#[allow(dead_code)]
pub fn parse_tags(input: &str) -> Vec<String> {
    input
        .split(',')
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .collect()
}

/// 解析多行标题
#[allow(dead_code)]
pub fn parse_multiline_titles(input: &str) -> Vec<String> {
    input
        .lines()
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_when_keywords() {
        assert!(matches!(parse_when("today").unwrap(), When::Today));
        assert!(matches!(parse_when("Today").unwrap(), When::Today));
        assert!(matches!(parse_when("tomorrow").unwrap(), When::Tomorrow));
        assert!(matches!(parse_when("evening").unwrap(), When::Evening));
        assert!(matches!(parse_when("anytime").unwrap(), When::Anytime));
        assert!(matches!(parse_when("someday").unwrap(), When::Someday));
    }

    #[test]
    fn test_parse_when_date() {
        let result = parse_when("2026-03-25").unwrap();
        assert!(
            matches!(result, When::Date(d) if d == NaiveDate::from_ymd_opt(2026, 3, 25).unwrap())
        );
    }

    #[test]
    fn test_parse_datetime() {
        let result = parse_when("2026-03-25@14:30").unwrap();
        assert!(matches!(result, When::DateTime(_)));

        let result = parse_when("2026-03-25@2:30PM").unwrap();
        assert!(matches!(result, When::DateTime(_)));
    }

    #[test]
    fn test_parse_tags() {
        let tags = parse_tags("Errand, Shopping, Important");
        assert_eq!(tags, vec!["Errand", "Shopping", "Important"]);
    }
}
