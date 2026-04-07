use chrono::{NaiveDate, NaiveDateTime};
use serde::{Deserialize, Serialize};

/// 待办事项
#[derive(Debug, Clone, Default)]
#[allow(dead_code)]
pub struct Todo {
    pub title: String,
    pub notes: Option<String>,
    pub when: Option<When>,
    pub deadline: Option<NaiveDate>,
    pub tags: Vec<String>,
    pub list: Option<ListRef>,
    pub heading: Option<String>,
    pub completed: bool,
    pub canceled: bool,
    pub checklist_items: Vec<String>,
    pub reveal: Option<bool>,
    pub creation_date: Option<NaiveDateTime>,
    pub completion_date: Option<NaiveDateTime>,
}

/// 项目
#[derive(Debug, Clone, Default)]
#[allow(dead_code)]
pub struct Project {
    pub title: String,
    pub notes: Option<String>,
    pub when: Option<When>,
    pub deadline: Option<NaiveDate>,
    pub tags: Vec<String>,
    pub area: Option<AreaRef>,
    pub to_dos: Vec<String>,
    pub completed: bool,
    pub canceled: bool,
    pub reveal: Option<bool>,
    pub creation_date: Option<NaiveDateTime>,
    pub completion_date: Option<NaiveDateTime>,
}

/// 时间安排
#[derive(Debug, Clone)]
pub enum When {
    Today,
    Tomorrow,
    Evening,
    Anytime,
    Someday,
    Date(NaiveDate),
    DateTime(NaiveDateTime),
    Natural(String), // "in 3 days", "next tuesday"
}

impl When {
    /// 转换为 URL 参数字符串
    pub fn to_url_param(&self) -> String {
        match self {
            When::Today => "today".to_string(),
            When::Tomorrow => "tomorrow".to_string(),
            When::Evening => "evening".to_string(),
            When::Anytime => "anytime".to_string(),
            When::Someday => "someday".to_string(),
            When::Date(d) => d.format("%Y-%m-%d").to_string(),
            When::DateTime(dt) => format!(
                "{}@{}",
                dt.format("%Y-%m-%d"),
                dt.format("%H:%M")
            ),
            When::Natural(s) => s.clone(),
        }
    }
}

/// 列表引用（项目或区域）
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub enum ListRef {
    Inbox,
    Id(String),
    Title(String),
}

/// 区域引用
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub enum AreaRef {
    Id(String),
    Title(String),
}

/// 待办事项更新操作
#[derive(Debug, Clone, Default)]
#[allow(dead_code)]
pub struct TodoUpdate {
    pub id: String,
    pub title: Option<String>,
    pub notes: Option<String>,
    pub prepend_notes: Option<String>,
    pub append_notes: Option<String>,
    pub when: Option<When>,
    pub deadline: Option<Option<NaiveDate>>, // None = no change, Some(None) = clear
    pub tags: Option<Vec<String>>,
    pub add_tags: Option<Vec<String>>,
    pub checklist_items: Option<Vec<String>>,
    pub prepend_checklist_items: Option<Vec<String>>,
    pub append_checklist_items: Option<Vec<String>>,
    pub list: Option<ListRef>,
    pub heading: Option<String>,
    pub completed: Option<bool>,
    pub canceled: Option<bool>,
    pub reveal: Option<bool>,
    pub duplicate: Option<bool>,
    pub creation_date: Option<NaiveDateTime>,
    pub completion_date: Option<NaiveDateTime>,
}

/// 项目更新操作
#[derive(Debug, Clone, Default)]
#[allow(dead_code)]
pub struct ProjectUpdate {
    pub id: String,
    pub title: Option<String>,
    pub notes: Option<String>,
    pub prepend_notes: Option<String>,
    pub append_notes: Option<String>,
    pub when: Option<When>,
    pub deadline: Option<Option<NaiveDate>>,
    pub tags: Option<Vec<String>>,
    pub add_tags: Option<Vec<String>>,
    pub area: Option<AreaRef>,
    pub completed: Option<bool>,
    pub canceled: Option<bool>,
    pub reveal: Option<bool>,
    pub duplicate: Option<bool>,
    pub creation_date: Option<NaiveDateTime>,
    pub completion_date: Option<NaiveDateTime>,
}

/// 显示查询
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub enum ShowQuery {
    /// 内置列表
    Builtin(BuiltinList),
    /// 通过 ID 查找
    Id(String),
    /// 通过标题查找
    Title(String),
}

/// 内置列表
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum BuiltinList {
    Inbox,
    Today,
    Tomorrow,
    Anytime,
    Upcoming,
    Someday,
    Logbook,
    Deadlines,
    Repeating,
    AllProjects,
    LoggedProjects,
}

impl BuiltinList {
    /// 转换为 URL id 参数
    pub fn as_str(&self) -> &'static str {
        match self {
            BuiltinList::Inbox => "inbox",
            BuiltinList::Today => "today",
            BuiltinList::Tomorrow => "tomorrow",
            BuiltinList::Anytime => "anytime",
            BuiltinList::Upcoming => "upcoming",
            BuiltinList::Someday => "someday",
            BuiltinList::Logbook => "logbook",
            BuiltinList::Deadlines => "deadlines",
            BuiltinList::Repeating => "repeating",
            BuiltinList::AllProjects => "all-projects",
            BuiltinList::LoggedProjects => "logged-projects",
        }
    }
}

impl std::fmt::Display for BuiltinList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl std::str::FromStr for BuiltinList {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "inbox" => Ok(BuiltinList::Inbox),
            "today" => Ok(BuiltinList::Today),
            "tomorrow" => Ok(BuiltinList::Tomorrow),
            "anytime" => Ok(BuiltinList::Anytime),
            "upcoming" => Ok(BuiltinList::Upcoming),
            "someday" => Ok(BuiltinList::Someday),
            "logbook" => Ok(BuiltinList::Logbook),
            "deadlines" => Ok(BuiltinList::Deadlines),
            "repeating" => Ok(BuiltinList::Repeating),
            "all-projects" => Ok(BuiltinList::AllProjects),
            "logged-projects" => Ok(BuiltinList::LoggedProjects),
            _ => Err(format!("Unknown builtin list: {}", s)),
        }
    }
}

/// 重复模式
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub enum RepeatPattern {
    /// 每天
    Daily,
    /// 每周
    Weekly,
    /// 每月
    Monthly,
    /// 每年
    Yearly,
    /// 每 N 天
    EveryNDays(u32),
    /// 每 N 周
    EveryNWeeks(u32),
    /// 每 N 月
    EveryNMonths(u32),
    /// 每 N 年
    EveryNYears(u32),
}

impl RepeatPattern {
    /// 解析重复模式字符串
    pub fn parse(input: &str) -> Option<Self> {
        let normalized = input.trim().to_lowercase();

        match normalized.as_str() {
            "day" | "daily" | "d" => Some(RepeatPattern::Daily),
            "week" | "weekly" | "w" => Some(RepeatPattern::Weekly),
            "month" | "monthly" | "m" => Some(RepeatPattern::Monthly),
            "year" | "yearly" | "y" => Some(RepeatPattern::Yearly),
            _ => {
                // 尝试解析 "every N days/weeks/months/years"
                if let Some(caps) = regex::Regex::new(r"every\s+(\d+)\s*(day|week|month|year)s?")
                    .ok()
                    .and_then(|re| re.captures(&normalized))
                {
                    let n: u32 = caps.get(1)?.as_str().parse().ok()?;
                    let unit = caps.get(2)?.as_str();
                    match unit {
                        "day" => Some(RepeatPattern::EveryNDays(n)),
                        "week" => Some(RepeatPattern::EveryNWeeks(n)),
                        "month" => Some(RepeatPattern::EveryNMonths(n)),
                        "year" => Some(RepeatPattern::EveryNYears(n)),
                        _ => None,
                    }
                } else {
                    None
                }
            }
        }
    }

    /// 转换为 URL 参数字符串
    pub fn to_url_param(&self) -> String {
        match self {
            RepeatPattern::Daily => "day".to_string(),
            RepeatPattern::Weekly => "week".to_string(),
            RepeatPattern::Monthly => "month".to_string(),
            RepeatPattern::Yearly => "year".to_string(),
            RepeatPattern::EveryNDays(n) => format!("{}-day", n),
            RepeatPattern::EveryNWeeks(n) => format!("{}-week", n),
            RepeatPattern::EveryNMonths(n) => format!("{}-month", n),
            RepeatPattern::EveryNYears(n) => format!("{}-year", n),
        }
    }
}
