use colored::Colorize;
use serde::Serialize;

use crate::cli::args::ListCommand;
use crate::db::ThingsDb;

/// JSON 输出结构
#[derive(Serialize)]
struct JsonOutput<T: Serialize> {
    success: bool,
    #[serde(rename = "type")]
    data_type: String,
    count: usize,
    data: Vec<T>,
}

impl<T: Serialize> JsonOutput<T> {
    fn new(data_type: &str, data: Vec<T>) -> Self {
        let count = data.len();
        Self {
            success: true,
            data_type: data_type.to_string(),
            count,
            data,
        }
    }
}

pub fn handle(cmd: ListCommand, json: bool) -> anyhow::Result<()> {
    // 检查数据库是否可访问
    if let Err(e) = crate::db::check_database_access() {
        if json {
            let error_output = serde_json::json!({
                "success": false,
                "error": e.to_string()
            });
            println!("{}", serde_json::to_string_pretty(&error_output)?);
        } else {
            eprintln!("{}", e);
        }
        return Ok(());
    }

    let db = ThingsDb::open_default()?;

    match cmd {
        ListCommand::Inbox => {
            let tasks = db.get_inbox_tasks()?;
            if json {
                print_json("tasks", tasks)?;
            } else {
                print_tasks("Inbox", &tasks);
            }
        }
        ListCommand::Today => {
            let tasks = db.get_today_tasks()?;
            if json {
                print_json("tasks", tasks)?;
            } else {
                print_tasks("Today", &tasks);
            }
        }
        ListCommand::Evening => {
            let tasks = db.get_evening_tasks()?;
            if json {
                print_json("tasks", tasks)?;
            } else {
                print_tasks("This Evening", &tasks);
            }
        }
        ListCommand::Upcoming => {
            let tasks = db.get_upcoming_tasks()?;
            if json {
                print_json("tasks", tasks)?;
            } else {
                print_tasks("Upcoming", &tasks);
            }
        }
        ListCommand::Someday => {
            let tasks = db.get_someday_tasks()?;
            if json {
                print_json("tasks", tasks)?;
            } else {
                print_tasks("Someday", &tasks);
            }
        }
        ListCommand::Anytime => {
            let tasks = db.get_anytime_tasks()?;
            if json {
                print_json("tasks", tasks)?;
            } else {
                print_tasks("Anytime", &tasks);
            }
        }
        ListCommand::Completed => {
            let tasks = db.get_completed_tasks()?;
            if json {
                print_json("tasks", tasks)?;
            } else {
                print_tasks("Completed", &tasks);
            }
        }
        ListCommand::CompletedToday => {
            let tasks = db.get_completed_today()?;
            if json {
                print_json("tasks", tasks)?;
            } else {
                print_tasks("Completed Today", &tasks);
            }
        }
        ListCommand::Canceled => {
            let tasks = db.get_canceled_tasks()?;
            if json {
                print_json("tasks", tasks)?;
            } else {
                print_tasks("Canceled", &tasks);
            }
        }
        ListCommand::Deadlines => {
            let tasks = db.get_tasks_with_deadlines()?;
            if json {
                print_json("tasks", tasks)?;
            } else {
                print_tasks_with_deadlines("Tasks with Deadlines", &tasks);
            }
        }
        ListCommand::Projects => {
            let projects = db.get_projects(None)?;
            if json {
                print_json("projects", projects)?;
            } else {
                print_projects(&projects);
            }
        }
        ListCommand::Areas => {
            let areas = db.get_areas()?;
            if json {
                print_json("areas", areas)?;
            } else {
                print_areas(&areas);
            }
        }
        ListCommand::Tags => {
            let tags = db.get_tags()?;
            if json {
                print_json("tags", tags)?;
            } else {
                print_tags(&tags);
            }
        }
    }

    Ok(())
}

fn print_json<T: Serialize>(data_type: &str, data: Vec<T>) -> anyhow::Result<()> {
    let output = JsonOutput::new(data_type, data);
    println!("{}", serde_json::to_string_pretty(&output)?);
    Ok(())
}

fn print_tasks(title: &str, tasks: &[crate::db::Task]) {
    println!("{}", title.bold().underline());
    if tasks.is_empty() {
        println!("  {}", "No tasks".dimmed());
        return;
    }

    for task in tasks {
        let status_icon = match task.status {
            crate::db::TaskStatus::Completed => "✓".green(),
            crate::db::TaskStatus::Canceled => "✗".red(),
            _ => "○".normal(),
        };

        let title = if task.status == crate::db::TaskStatus::Completed
            || task.status == crate::db::TaskStatus::Canceled
        {
            task.title.as_str().dimmed().to_string()
        } else {
            task.title.clone()
        };

        let project_info = task.project.as_ref()
            .map(|p: &String| format!(" [{}]", p.truecolor(128, 128, 128)))
            .unwrap_or_default();

        let tag_info = if !task.tags.is_empty() {
            format!(" {}", format!("#{}" , task.tags.join(" #")).cyan())
        } else {
            String::new()
        };

        println!(
            "  {} {}{}{}",
            status_icon,
            title,
            project_info,
            tag_info
        );

        // 打印 ID（简短形式）
        let short_id = &task.uuid[..8];
        println!("     {}", short_id.dimmed());
    }

    println!();
    println!("  Total: {}", tasks.len().to_string().bold());
}

fn print_tasks_with_deadlines(title: &str, tasks: &[crate::db::Task]) {
    println!("{}", title.bold().underline());
    if tasks.is_empty() {
        println!("  {}", "No deadlines".dimmed());
        return;
    }

    for task in tasks {
        let deadline = task
            .deadline
            .map(|d: chrono::NaiveDate| d.to_string())
            .unwrap_or_else(|| "?".to_string());

        let project_info = task.project.as_ref()
            .map(|p: &String| format!(" [{}]", p.truecolor(128, 128, 128)))
            .unwrap_or_default();

        println!(
            "  {} {}{}{}",
            deadline.yellow(),
            task.title,
            project_info,
            format!(" [{}]", &task.uuid[..8]).truecolor(128, 128, 128)
        );
    }

    println!();
    println!("  Total: {}", tasks.len().to_string().bold());
}

fn print_projects(projects: &[crate::db::Project]) {
    println!("{}", "Projects".bold().underline());
    if projects.is_empty() {
        println!("  {}", "No projects".dimmed());
        return;
    }

    for project in projects {
        let area_info = project.area.as_ref()
            .map(|a: &String| format!(" [{}]", a.truecolor(128, 128, 128)))
            .unwrap_or_default();

        let tag_info = if !project.tags.is_empty() {
            format!(" {}", format!("#{}" , project.tags.join(" #")).cyan())
        } else {
            String::new()
        };

        println!(
            "  • {}{}{}{}",
            project.title.bold(),
            area_info,
            tag_info,
            format!(" [{}]", &project.uuid[..8]).truecolor(128, 128, 128)
        );
    }

    println!();
    println!("  Total: {}", projects.len().to_string().bold());
}

fn print_areas(areas: &[crate::db::Area]) {
    println!("{}", "Areas".bold().underline());
    if areas.is_empty() {
        println!("  {}", "No areas".dimmed());
        return;
    }

    for area in areas {
        println!(
            "  • {}{}",
            area.title.bold(),
            format!(" [{}]", &area.uuid[..8]).truecolor(128, 128, 128)
        );
    }

    println!();
    println!("  Total: {}", areas.len().to_string().bold());
}

fn print_tags(tags: &[crate::db::Tag]) {
    println!("{}", "Tags".bold().underline());
    if tags.is_empty() {
        println!("  {}", "No tags".dimmed());
        return;
    }

    for tag in tags {
        let shortcut_info = tag.shortcut.as_ref()
            .map(|s: &String| format!(" ({})", s.truecolor(128, 128, 128)))
            .unwrap_or_default();

        println!(
            "  #{}{}{}",
            tag.title.cyan(),
            shortcut_info,
            format!(" [{}]", &tag.uuid[..8]).truecolor(128, 128, 128)
        );
    }

    println!();
    println!("  Total: {}", tags.len().to_string().bold());
}
