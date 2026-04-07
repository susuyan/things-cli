use clap::{Args, Subcommand};

/// 全局选项
#[derive(Args, Debug)]
pub struct GlobalOpts {
    /// 启用调试模式
    #[arg(short, long, global = true)]
    pub debug: bool,

    /// 使用指定的 auth-token（覆盖配置）
    #[arg(long, global = true, env = "THINGS_AUTH_TOKEN")]
    pub auth_token: Option<String>,
}

/// 待办事项命令
#[derive(Subcommand, Debug)]
pub enum TodoCommand {
    /// 添加待办事项
    Add {
        /// 待办事项标题（可多个）
        #[arg(required = true)]
        titles: Vec<String>,

        /// 备注
        #[arg(short, long)]
        notes: Option<String>,

        /// 安排时间（today, tomorrow, evening, 2026-03-25, in 3 days）
        #[arg(short, long)]
        when: Option<String>,

        /// 截止日期
        #[arg(long)]
        deadline: Option<String>,

        /// 标签（逗号分隔）
        #[arg(short, long, value_delimiter = ',')]
        tags: Vec<String>,

        /// 添加到指定列表（项目或区域标题）
        #[arg(short, long)]
        list: Option<String>,

        /// 添加到指定列表（ID）
        #[arg(long)]
        list_id: Option<String>,

        /// 添加到指定标题
        #[arg(long)]
        heading: Option<String>,

        /// 清单项目（逗号分隔，或多次使用）
        #[arg(long, value_delimiter = ',')]
        checklist: Vec<String>,

        /// 标记为完成
        #[arg(long, conflicts_with = "canceled")]
        completed: bool,

        /// 标记为取消
        #[arg(long)]
        canceled: bool,

        /// 显示快速输入对话框
        #[arg(long)]
        show_quick_entry: bool,

        /// 创建后显示
        #[arg(long)]
        reveal: bool,

        /// 从 stdin 读取标题（每行一个）
        #[arg(long, conflicts_with = "titles")]
        stdin: bool,

        /// 重复模式（day, week, month, year, 2-day, 3-week）
        #[arg(long)]
        repeat: Option<String>,

        /// 重复结束日期
        #[arg(long)]
        repeat_until: Option<String>,
    },

    /// 更新待办事项
    Update {
        /// 待办事项 ID
        id: String,

        /// 新标题
        #[arg(short, long)]
        title: Option<String>,

        /// 新备注（替换）
        #[arg(short, long)]
        notes: Option<String>,

        /// 在前面添加备注
        #[arg(long)]
        prepend_notes: Option<String>,

        /// 在后面添加备注
        #[arg(long)]
        append_notes: Option<String>,

        /// 新时间安排
        #[arg(short, long)]
        when: Option<String>,

        /// 新截止日期（空字符串表示清除）
        #[arg(long)]
        deadline: Option<String>,

        /// 新标签（替换）
        #[arg(long, value_delimiter = ',')]
        tags: Vec<String>,

        /// 添加标签
        #[arg(long, value_delimiter = ',')]
        add_tags: Vec<String>,

        /// 移动到指定列表
        #[arg(short, long)]
        list: Option<String>,

        /// 移动到指定列表（ID）
        #[arg(long)]
        list_id: Option<String>,

        /// 移动到指定标题
        #[arg(long)]
        heading: Option<String>,

        /// 标记为完成
        #[arg(long)]
        complete: bool,

        /// 标记为未完成
        #[arg(long)]
        uncomplete: bool,

        /// 取消
        #[arg(long)]
        cancel: bool,

        /// 复制后再更新
        #[arg(long)]
        duplicate: bool,

        /// 更新后显示
        #[arg(long)]
        reveal: bool,

        /// 重复模式（day, week, month, year, 2-day, 3-week）
        #[arg(long)]
        repeat: Option<String>,

        /// 重复结束日期
        #[arg(long)]
        repeat_until: Option<String>,

        /// 取消重复
        #[arg(long, conflicts_with = "repeat")]
        no_repeat: bool,
    },

    /// 删除待办事项
    Delete {
        /// 待办事项 ID
        id: String,

        /// 强制删除，不提示确认
        #[arg(short, long)]
        force: bool,
    },
}

/// 项目命令
#[derive(Subcommand, Debug)]
pub enum ProjectCommand {
    /// 添加项目
    Add {
        /// 项目标题
        title: String,

        /// 备注
        #[arg(short, long)]
        notes: Option<String>,

        /// 安排时间
        #[arg(short, long)]
        when: Option<String>,

        /// 截止日期
        #[arg(long)]
        deadline: Option<String>,

        /// 标签
        #[arg(short, long, value_delimiter = ',')]
        tags: Vec<String>,

        /// 添加到区域（标题）
        #[arg(short, long)]
        area: Option<String>,

        /// 添加到区域（ID）
        #[arg(long)]
        area_id: Option<String>,

        /// 待办事项（逗号分隔）
        #[arg(long, value_delimiter = ',')]
        todos: Vec<String>,

        /// 标记为完成
        #[arg(long)]
        completed: bool,

        /// 标记为取消
        #[arg(long)]
        canceled: bool,

        /// 创建后显示
        #[arg(long)]
        reveal: bool,

        /// 从 JSON 文件创建
        #[arg(long, conflicts_with = "title")]
        json: Option<String>,
    },

    /// 更新项目
    Update {
        /// 项目 ID
        id: String,

        /// 新标题
        #[arg(short, long)]
        title: Option<String>,

        /// 备注
        #[arg(short, long)]
        notes: Option<String>,

        /// 在前面添加备注
        #[arg(long)]
        prepend_notes: Option<String>,

        /// 在后面添加备注
        #[arg(long)]
        append_notes: Option<String>,

        /// 新时间安排
        #[arg(short, long)]
        when: Option<String>,

        /// 新截止日期
        #[arg(long)]
        deadline: Option<String>,

        /// 新标签（替换）
        #[arg(long, value_delimiter = ',')]
        tags: Vec<String>,

        /// 添加标签
        #[arg(long, value_delimiter = ',')]
        add_tags: Vec<String>,

        /// 移动到区域
        #[arg(short, long)]
        area: Option<String>,

        /// 移动到区域（ID）
        #[arg(long)]
        area_id: Option<String>,

        /// 标记为完成
        #[arg(long)]
        complete: bool,

        /// 标记为未完成
        #[arg(long)]
        uncomplete: bool,

        /// 取消
        #[arg(long)]
        cancel: bool,

        /// 复制后再更新
        #[arg(long)]
        duplicate: bool,

        /// 更新后显示
        #[arg(long)]
        reveal: bool,
    },

    /// 删除项目
    Delete {
        /// 项目 ID
        id: String,

        /// 强制删除，不提示确认
        #[arg(short, long)]
        force: bool,
    },
}

/// 显示命令
#[derive(Args, Debug)]
pub struct ShowCommand {
    /// 要显示的内容（today, inbox, anytime, 或项目/区域/待办的标题）
    pub query: Option<String>,

    /// 通过 ID 查找
    #[arg(short, long)]
    pub id: Option<String>,

    /// 按标签过滤
    #[arg(short, long, value_delimiter = ',')]
    pub filter: Vec<String>,
}

/// 批量命令
#[derive(Subcommand, Debug)]
pub enum BatchCommand {
    /// 从 JSON 文件批量导入
    Import {
        /// JSON 文件路径（- 表示从 stdin 读取）
        file: String,

        /// 创建后显示
        #[arg(long)]
        reveal: bool,
    },

    /// 生成 JSON 模板
    Template,
}

/// 区域命令
#[derive(Subcommand, Debug)]
pub enum AreaCommand {
    /// 添加区域
    Add {
        /// 区域标题
        title: String,

        /// 标签（逗号分隔）
        #[arg(short, long, value_delimiter = ',')]
        tags: Vec<String>,

        /// 创建后显示
        #[arg(long)]
        reveal: bool,
    },

    /// 更新区域
    Update {
        /// 区域 ID
        id: String,

        /// 新标题
        #[arg(short, long)]
        title: Option<String>,

        /// 新标签（替换）
        #[arg(long, value_delimiter = ',')]
        tags: Vec<String>,

        /// 添加标签
        #[arg(long, value_delimiter = ',')]
        add_tags: Vec<String>,

        /// 更新后显示
        #[arg(long)]
        reveal: bool,
    },

    /// 删除区域
    Delete {
        /// 区域 ID
        id: String,

        /// 强制删除，不提示确认
        #[arg(short, long)]
        force: bool,
    },
}

/// 列表查询命令
#[derive(Subcommand, Debug)]
pub enum ListCommand {
    /// 收件箱
    Inbox,

    /// 今日任务
    Today,

    /// 今晚任务
    Evening,

    /// 待办任务
    Upcoming,

    /// 某天任务
    Someday,

    /// 任意时间任务
    Anytime,

    /// 已完成任务
    Completed,

    /// 今日完成的任务
    CompletedToday,

    /// 已取消任务
    Canceled,

    /// 带截止日期的任务
    Deadlines,

    /// 所有项目
    Projects,

    /// 所有区域
    Areas,

    /// 所有标签
    Tags,
}

/// 配置命令
#[derive(Subcommand, Debug)]
pub enum ConfigCommand {
    /// 设置 auth-token（交互式或命令行）
    SetAuthToken {
        /// 直接提供 token（不推荐，会留在 shell history）
        token: Option<String>,
    },

    /// 删除 auth-token
    DeleteAuthToken,

    /// 检查 auth-token 是否已设置
    CheckAuthToken,

    /// 设置默认列表
    SetDefaultList {
        list: String,
    },

    /// 设置默认标签
    SetDefaultTags {
        #[arg(value_delimiter = ',')]
        tags: Vec<String>,
    },

    /// 查看当前配置
    Show,

    /// 编辑配置文件
    Edit,
}
