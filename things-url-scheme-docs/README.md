# Things URL Scheme 文档

> 原文：https://culturedcode.com/things/support/articles/2803573/
> 抓取时间：2026-04-06

---

## 简介

URL Scheme 允许高级用户和其他应用开发者向 Things 发送命令。支持的命令示例：

- 创建名为 "Buy milk" 的新待办事项
- 显示 Today 列表
- 显示所有带有 "Errand" 标签的待办事项
- 搜索所有待办事项中的 "shipping address"

还有一个强大的基于 JSON 的命令，可以创建整个项目，包括所有笔记、标题和待办事项。

---

## 目录

- [概述](#概述)
- [命令](#命令)
  - [add](#add)
  - [add-project](#add-project)
  - [update](#update)
  - [update-project](#update-project)
  - [show](#show)
  - [search](#search)
  - [version](#version)
- [开发者参考](#开发者参考)
  - [json](#json)

---

## 概述

### URL 格式

命令通过构造特殊 URL 链接发送给 Things：

```
things:///commandName?
    parameter1=value1&
    parameter2=value2&
    ...
```

打开这些链接将启动应用并执行命令。

**示例：创建一个待办事项**
```
things:///add?
    title=Buy%20milk&
    notes=High%20fat
```

### x-callback-url 支持

所有命令都支持 x-callback-url 约定，会在适当时调用 `x-success`、`x-error` 或 `x-cancel` 回调。许多命令会向 `x-success` 回调返回参数。

### 获取 ID

某些命令需要提供待办事项或列表的 ID。获取方法：

**获取待办事项 ID：**
- Mac：Control-点击待办事项 → 分享 → 复制链接
- iOS：点击待办事项打开 → 底部工具栏点击分享 → 复制链接

**获取列表 ID：**
- Mac：Control-点击侧边栏中的列表 → 分享 → 复制链接
- iOS：进入列表 → 右上角点击分享 → 复制链接

### 授权 Token

出于安全原因，修改现有 Things 数据的命令需要授权令牌（`auth-token`）。获取位置：
- Mac：Things → 设置 → 通用 → 启用 Things URLs → 管理
- iOS：设置 → 通用 → Things URLs

### 数据类型

| 类型 | 说明 |
|------|------|
| `string` | 百分比编码。最大未编码长度：4,000 字符（除非另有说明） |
| `date string` | `today`、`tomorrow` 或 `yyyy-mm-dd` 格式（如 `2017-09-29`）。也支持自然语言如 `in 3 days`、`next tuesday`（必须使用英文） |
| `time string` | 本地时区时间，如 `9:30PM` 或 `21:30` |
| `date time string` | 日期字符串 + `@` + 时间字符串，如 `2026-02-25@14:00` |
| `ISO8601 date time string` | ISO8601 格式的日期时间，如 `2026-03-10T14:30:00Z` |
| `boolean` | `true` 或 `false` |
| `JSON string` | JSON 格式字符串 |

### 启用 URL Scheme

首次执行命令时，Things 会询问是否启用此功能。后续可在设置中更改：
- Mac：Things → 设置 → 通用
- iOS：设置 → 通用 → Things URLs

### 版本

当前 URL Scheme 版本：**2**

---

## 命令

### add

添加一个待办事项。

#### 示例

**创建收件箱待办事项：**
```
things:///add?title=Book%20flights
```

**创建带标签和备注的待办事项，设置今晚开始：**
```
things:///add?
    title=Buy%20milk&
    notes=Low%20fat.&
    when=evening&
    tags=Errand
```

**批量创建待办事项并添加到 Shopping 项目：**
```
things:///add?
    titles=Milk%0aBeer%0aCheese&
    list=Shopping
```

**创建待办事项并安排到下周一，放入指定区域：**
```
things:///add?
    title=Call%20doctor&
    when=next%20monday&
    list-id=TryhwrjdiHEXfjgNtw81yt
```

**创建今晚列表的待办事项，设置下午 6 点提醒：**
```
things:///add?
    title=Collect%20dry%20cleaning&
    when=evening@6pm
```

> **注意**：10 秒内最多可添加 250 个项目。

#### 参数

| 参数 | 类型 | 说明 |
|------|------|------|
| `title` | string | 待办事项标题（如果同时指定 `titles`，则忽略此项） |
| `titles` | string | 用换行符分隔的多个标题（编码为 `%0a`）。优先于 `title` 和 `show-quick-entry`，其他参数将应用于所有创建的待办事项 |
| `notes` | string | 备注内容。最大未编码长度：10,000 字符 |
| `when` | string | `today`、`tomorrow`、`evening`、`anytime`、`someday`、日期字符串或日期时间字符串。使用日期时间字符串会添加该时间的提醒 |
| `deadline` | date string | 截止日期 |
| `tags` | string | 逗号分隔的标签标题。不存在的标签将被忽略 |
| `checklist-items` | string | 用换行符分隔的清单项目（最多 100 个） |
| `use-clipboard` | string | `replace-title`（换行溢出到备注）、`replace-notes` 或 `replace-checklist-items`。优先于 `title`、`notes` 或 `checklist-items` |
| `list-id` | string | 项目或区域的 ID。优先于 `list` |
| `list` | string | 项目或区域的标题。如果指定了 `list-id`，则忽略此项 |
| `heading-id` | string | 项目内的标题 ID。优先于 `heading`。如果未指定项目或标题不存在，则忽略 |
| `heading` | string | 项目内的标题。如果指定了 `heading-id`，则忽略 |
| `completed` | boolean | 是否标记为完成。默认：`false`。如果同时设置 `canceled=true`，则忽略此项 |
| `canceled` | boolean | 是否标记为取消。默认：`false`。优先于 `completed` |
| `show-quick-entry` | boolean | 是否显示快速输入对话框（填充提供的数据）而不是直接添加。如果指定了 `titles`，则忽略。默认：`false` |
| `reveal` | boolean | 是否导航并显示新创建的待办事项。如果创建多个，显示第一个。如果同时设置 `show-quick-entry=true`，则忽略。默认：`false` |
| `creation-date` | ISO8601 string | 数据库中的创建日期。如果日期在未来，则忽略 |
| `completion-date` | ISO8601 string | 数据库中的完成日期。如果待办事项未完成/未取消，或日期在未来，则忽略 |

#### x-success 返回参数

| 参数 | 说明 |
|------|------|
| `x-things-id` | 逗号分隔的创建的待办事项 ID |

---

### add-project

添加一个项目。

#### 示例

**创建今天开始的树屋项目：**
```
things:///add-project?
    title=Build%20treehouse&
    when=today
```

**在 Family 区域内创建项目：**
```
things:///add-project?
    title=Plan%20Birthday%20Party&
    area=Family
```

**在指定区域内创建带截止日期的项目：**
```
things:///add-project?
    title=Submit%20Tax&
    deadline=December%2031&
    area-id=Lg8UqVPXo2SbJNiBpDBBQ
```

#### 参数

| 参数 | 类型 | 说明 |
|------|------|------|
| `title` | string | 项目标题 |
| `notes` | string | 备注内容。最大未编码长度：10,000 字符 |
| `when` | string | `today`、`tomorrow`、`evening`、`anytime`、`someday`、日期字符串或日期时间字符串 |
| `deadline` | date string | 截止日期 |
| `tags` | string | 逗号分隔的标签标题 |
| `area-id` | string | 区域的 ID。优先于 `area` |
| `area` | string | 区域的标题。如果指定了 `area-id`，则忽略 |
| `to-dos` | string | 用换行符分隔的待办事项标题，将在项目内创建 |
| `completed` | boolean | 是否标记为完成。默认：`false`。会同时将所有子待办事项标记为完成 |
| `canceled` | boolean | 是否标记为取消。默认：`false`。会同时将所有子待办事项标记为取消 |
| `reveal` | boolean | 是否导航进入新创建的项目。默认：`false` |
| `creation-date` | ISO8601 string | 数据库中的创建日期。如果同时指定 `to-dos`，也应用于它们 |
| `completion-date` | ISO8601 string | 数据库中的完成日期。如果同时指定 `to-dos`，也应用于它们 |

#### x-success 返回参数

| 参数 | 说明 |
|------|------|
| `x-things-id` | 创建的项目 ID |

---

### update

更新现有待办事项。

#### 示例

**设置待办事项今天开始：**
```
things:///update?
    id=SyJEz273ceSkabUbciM73A&
    when=today
```

**更改待办事项标题：**
```
things:///update?
    id=SyJEz273ceSkabUbciM73A&
    title=Buy%20bread
```

**追加备注：**
```
things:///update?
    id=SyJEz273ceSkabUbciM73A&
    append-notes=Wholemeal%20bread
```

**添加清单项目：**
```
things:///update?
    id=SyJEz273ceSkabUbciM73A&
    append-checklist-items=Cheese%0aBread%0aEggplant
```

**移除截止日期：**
```
things:///update?
    id=SyJEz273ceSkabUbciM73A&
    deadline=
```

#### 参数

| 参数 | 类型 | 说明 |
|------|------|------|
| `auth-token` | string | **必需**。Things URL Scheme 授权令牌 |
| `id` | string | **必需**。待办事项的 ID |
| `title` | string | 待办事项标题。替换现有标题 |
| `notes` | string | 备注内容。替换现有备注。最大未编码长度：10,000 字符 |
| `prepend-notes` | string | 在现有备注前添加的文本。最大未编码长度：10,000 字符 |
| `append-notes` | string | 在现有备注后添加的文本。最大未编码长度：10,000 字符 |
| `when` | string | `today`、`tomorrow`、`evening`、`someday`、日期字符串或日期时间字符串。不能用于重复待办事项 |
| `deadline` | date string | 截止日期。不能用于重复待办事项 |
| `tags` | string | 逗号分隔的标签标题。替换所有当前标签 |
| `add-tags` | string | 逗号分隔的标签标题。添加到待办事项 |
| `checklist-items` | string | 用 `%0a` 分隔的字符串。设置清单项目（最多 100 个）。替换所有现有清单项目 |
| `prepend-checklist-items` | string | 在清单项目列表前添加项目（最多 100 个） |
| `append-checklist-items` | string | 在清单项目列表后添加项目（最多 100 个） |
| `list-id` | string | 要移动到的项目或区域的 ID。优先于 `list` |
| `list` | string | 要移动到的项目或区域的标题。如果指定了 `list-id`，则忽略 |
| `heading-id` | string | 项目内标题的 ID。优先于 `heading`。可与 `list` 或 `list-id` 一起使用 |
| `heading` | string | 项目内标题的标题。如果指定了 `heading-id`，则忽略。可与 `list` 或 `list-id` 一起使用 |
| `completed` | boolean | 完成或取消完成待办事项。不能用于重复待办事项 |
| `canceled` | boolean | 取消或取消取消待办事项。优先于 `completed` |
| `reveal` | boolean | 是否导航并显示更新的待办事项。默认：`false` |
| `duplicate` | boolean | 是否在更新前复制待办事项（保留原始不变）。重复待办事项不能被复制。默认：`false` |
| `creation-date` | ISO8601 string | 数据库中的创建日期。如果日期在未来，则忽略 |
| `completion-date` | ISO8601 string | 数据库中的完成日期。如果待办事项未完成/未取消，或日期在未来，则忽略。不能用于重复待办事项 |

#### x-success 返回参数

| 参数 | 说明 |
|------|------|
| `x-things-id` | 更新的待办事项 ID |

---

### update-project

更新现有项目。

#### 示例

**设置项目明天开始：**
```
things:///update-project?
    id=Jvj7EW1fLoScPhaw2JomCT&
    when=tomorrow
```

**添加标签：**
```
things:///update-project?
    id=Jvj7EW1fLoScPhaw2JomCT&
    add-tags=Important
```

**在备注前添加内容：**
```
things:///update-project?
    id=Jvj7EW1fLoScPhaw2JomCT&
    prepend-notes=SFO%20to%20JFK.
```

**清除截止日期：**
```
things:///update-project?
    id=Jvj7EW1fLoScPhaw2JomCT&
    deadline=
```

#### 参数

| 参数 | 类型 | 说明 |
|------|------|------|
| `auth-token` | string | **必需**。Things URL Scheme 授权令牌 |
| `id` | string | **必需**。项目的 ID |
| `title` | string | 项目标题。替换现有标题 |
| `notes` | string | 备注内容。替换现有备注。最大未编码长度：10,000 字符 |
| `prepend-notes` | string | 在现有备注前添加的文本。最大未编码长度：10,000 字符 |
| `append-notes` | string | 在现有备注后添加的文本。最大未编码长度：10,000 字符 |
| `when` | string | `today`、`tomorrow`、`evening`、`someday`、日期字符串或日期时间字符串。不能用于重复项目 |
| `deadline` | date string | 截止日期。不能用于重复项目 |
| `tags` | string | 逗号分隔的标签标题。替换所有当前标签 |
| `add-tags` | string | 逗号分隔的标签标题。添加到项目 |
| `area-id` | string | 要移动到的区域的 ID。优先于 `area` |
| `area` | string | 要移动到的区域的标题。如果指定了 `area-id`，则忽略 |
| `completed` | boolean | 完成或取消完成项目。除非所有子待办事项都已完成或取消，且所有子标题都已归档，否则设置为 true 将被忽略。不能用于重复项目 |
| `canceled` | boolean | 取消或取消取消项目。优先于 `completed` |
| `reveal` | boolean | 是否导航并显示更新的项目。默认：`false` |
| `duplicate` | boolean | 是否在更新前复制项目（保留原始不变）。重复项目不能被复制。默认：`false` |
| `creation-date` | ISO8601 string | 数据库中的创建日期。如果日期在未来，则忽略 |
| `completion-date` | ISO8601 string | 数据库中的完成日期。如果项目未完成/未取消，或日期在未来，则忽略。不能用于重复项目 |

#### x-success 返回参数

| 参数 | 说明 |
|------|------|
| `x-things-id` | 更新的项目 ID |

---

### show

导航并显示区域、项目、标签或待办事项，或内置列表，可选择按一个或多个标签过滤。

#### 示例

**导航到 Today 列表：**
```
things:///show?id=today
```

**显示指定 ID 的待办事项：**
```
things:///show?id=GJJVZHE7SNu7xcVuH2xDDh
```

**进入指定 ID 的项目：**
```
things:///show?id=Qi9pM1heCNAZxKREgQrwnJ
```

**显示标题为 "Vacation" 的项目：**
```
things:///show?query=vacation
```

**显示标题为 "Vacation" 的项目，按 "Errand" 标签过滤：**
```
things:///show?
    query=vacation&
    filter=errand
```

#### 参数

| 参数 | 类型 | 说明 |
|------|------|------|
| `id` | string | 要显示的区域、项目、标签或待办事项的 ID；或内置列表 ID：`inbox`、`today`、`anytime`、`upcoming`、`someday`、`logbook`、`tomorrow`、`deadlines`、`repeating`、`all-projects`、`logged-projects`。优先于 `query` |
| `query` | string | 要显示的区域、项目、标签或内置列表的名称。如果同时设置 `id`，则忽略。注意：不能使用 `query` 参数显示任务；请使用 `id` 参数或 `search` 命令 |
| `filter` | string | 逗号分隔的标签标题，用于过滤列表 |

#### x-success 返回参数

无

---

### search

调用并显示搜索屏幕。

#### 示例

**搜索 "vacation"：**
```
things:///search?query=vacation
```

**显示空搜索屏幕：**
```
things:///search
```

#### 参数

| 参数 | 类型 | 说明 |
|------|------|------|
| `query` | string | 搜索查询（可选） |

#### x-success 返回参数

无

---

### version

获取 Things 应用和 URL Scheme 的版本。

```
things:///version
```

#### 参数

无

#### x-success 返回参数

| 参数 | 说明 |
|------|------|
| `x-things-scheme-version` | Things URL Scheme 版本 |
| `x-things-client-version` | 应用的构建号 |

---

## 开发者参考

### json

Things 还有一个高级的基于 JSON 的添加命令，允许对导入的项目和待办事项进行更多控制。

> **Swift 辅助类**：Things 提供了 Swift 辅助类来更轻松地生成所需的 JSON。获取地址：[Things JSON Coder GitHub](https://github.com/culturedcode/ThingsJSONCoder)

#### 示例

```
things:///json?data=
  [
    {
      "type": "project",
      "attributes": {
        "title": "Go Shopping",
        "items": [
          {
            "type": "to-do",
            "attributes": {
              "title": "Bread"
            }
          },
          {
            "type": "to-do",
            "attributes": {
              "title": "Milk"
            }
          }
        ]
      }
    }
  ]
```

#### 参数

| 参数 | 类型 | 说明 |
|------|------|------|
| `auth-token` | string | 当提供的 JSON 数据包含更新操作时必需 |
| `data` | JSON string | JSON 应该是包含待办事项和项目对象的数组 |
| `reveal` | boolean | 是否导航并显示新创建的待办事项或项目。如果创建多个，显示第一个。默认：`false` |

#### x-success 返回参数

| 参数 | 说明 |
|------|------|
| `x-things-ids` | JSON 字符串。JSON 数组中指定的待办事项和项目创建的 ID 数组。项目内创建的待办事项的 ID 不返回 |

---

## JSON 对象结构

### 通用结构

```json
{
    "type": "to-do",
    "operation": "update",
    "id": "Di9deEJeUkVZaDEdbnzQZw",
    "attributes": {
        "deadline": "today"
    }
}
```

| 字段 | 说明 |
|------|------|
| `type` | 对象类型。必需 |
| `operation` | 操作类型：`create` 或 `update`。如果未指定，默认为 `create`。目前只有 `to-do` 和 `project` 可以更新 |
| `id` | 更新操作必需。要更新的对象的 ID |
| `attributes` | 属性字典。必须包含，但所有属性都是可选的 |

---

### To-do

```json
{
  "type": "to-do",
  "attributes": {
    "title": "Milk"
  }
}
```

#### 属性

| 属性 | 类型 | 说明 |
|------|------|------|
| `title` | string | 待办事项标题 |
| `notes` | string | 备注内容。最大长度：10,000 字符 |
| `when` | string | `today`、`tomorrow`、`evening`、`anytime`、`someday`、日期字符串或日期时间字符串 |
| `deadline` | date string | 截止日期 |
| `tags` | array of strings | 标签标题数组 |
| `checklist-items` | array of objects | 清单项目对象数组（最多 100 个） |
| `list-id` | string | 要添加到的项目或区域的 ID。优先于 `list`。如果在项目的 `items` 数组中指定了待办事项，则忽略 |
| `list` | string | 要添加到的项目或区域的标题。如果在项目的 `items` 数组中指定了待办事项，则忽略 |
| `heading-id` | string | 优先于 `heading`。项目内标题的 ID |
| `heading` | string | 项目内标题的标题。如果指定了 `heading-id`，则忽略 |
| `completed` | boolean | 是否标记为完成。默认：`false`。如果同时设置 `canceled=true`，则忽略 |
| `canceled` | boolean | 是否标记为取消。默认：`false`。优先于 `completed` |
| `creation-date` | ISO8601 string | 数据库中的创建日期。如果日期在未来，则忽略 |
| `completion-date` | ISO8601 string | 数据库中的完成日期。如果待办事项未完成/未取消，或日期在未来，则忽略 |

#### 更新专用属性

| 属性 | 类型 | 说明 |
|------|------|------|
| `prepend-notes` | string | 在现有备注前添加的文本。最大未编码长度：10,000 字符 |
| `append-notes` | string | 在现有备注后添加的文本。最大未编码长度：10,000 字符 |
| `add-tags` | string | 逗号分隔的标签标题。添加到待办事项 |
| `prepend-checklist-items` | string | 在清单项目列表前添加项目 |
| `append-checklist-items` | string | 在清单项目列表后添加项目 |

---

### Project

```json
{
  "type": "project",
  "attributes": {
    "title": "Go Shopping",
    "items": [
      {
        "type": "to-do",
        "attributes": {
          "title": "Bread"
        }
      }
    ]
  }
}
```

#### 属性

| 属性 | 类型 | 说明 |
|------|------|------|
| `title` | string | 项目标题 |
| `notes` | string | 备注内容。最大长度：10,000 字符 |
| `when` | string | `today`、`tomorrow`、`evening`、`anytime`、`someday`、日期字符串或日期时间字符串 |
| `deadline` | date string | 截止日期 |
| `tags` | array of strings | 标签标题数组 |
| `area-id` | string | 要添加到的区域的 ID。优先于 `area` |
| `area` | string | 要添加到的区域的标题。如果指定了 `area-id`，则忽略 |
| `completed` | boolean | 是否标记为完成。默认：`false`。除非所有子待办事项都已完成或取消，否则设置为 true 将被忽略 |
| `canceled` | boolean | 是否标记为取消。默认：`false`。优先于 `completed` |
| `creation-date` | ISO8601 string | 数据库中的创建日期。如果日期在未来，则忽略 |
| `completion-date` | ISO8601 string | 数据库中的完成日期。如果项目未完成/未取消，或日期在未来，则忽略 |

#### 创建专用属性

| 属性 | 类型 | 说明 |
|------|------|------|
| `items` | array | 待办事项或标题对象的数组。要添加到现有项目，请创建单独的待办事项对象 |

#### 更新专用属性

| 属性 | 类型 | 说明 |
|------|------|------|
| `prepend-notes` | string | 在现有备注前添加的文本。最大未编码长度：10,000 字符 |
| `append-notes` | string | 在现有备注后添加的文本。最大未编码长度：10,000 字符 |
| `add-tags` | string | 逗号分隔的标签标题。添加到项目 |

---

### Heading

```json
{
  "type": "heading",
  "attributes": {
    "title": "Sights"
  }
}
```

#### 属性

| 属性 | 类型 | 说明 |
|------|------|------|
| `title` | string | 标题的标题 |
| `archived` | boolean | 标题是否已归档。默认：`false`。除非标题下的所有待办事项都已完成或取消，否则将被忽略 |

---

### Checklist Item

```json
{
  "type": "checklist-item",
  "attributes": {
    "title": "Hotels",
    "completed": true
  }
}
```

#### 属性

| 属性 | 类型 | 说明 |
|------|------|------|
| `title` | string | 清单项目标题 |
| `completed` | boolean | 是否标记为完成。默认：`false`。如果同时设置 `canceled=true`，则忽略 |
| `canceled` | boolean | 是否标记为取消。默认：`false`。优先于 `completed` |

---

## 完整 JSON 示例

```json
[
  {
    "type": "project",
    "attributes": {
      "title": "Go Shopping",
      "items": [
        {
          "type": "to-do",
          "attributes": {
            "title": "Bread"
          }
        },
        {
          "type": "to-do",
          "attributes": {
            "title": "Milk"
          }
        }
      ]
    }
  },
  {
    "type": "project",
    "attributes": {
      "title": "Vacation in Rome",
      "notes": "Some time in August.",
      "area": "Family",
      "items": [
        {
          "type": "to-do",
          "attributes": {
            "title": "Ask Sarah for travel guide"
          }
        },
        {
          "type": "to-do",
          "attributes": {
            "title": "Add dates to calendar"
          }
        },
        {
          "type": "heading",
          "attributes": {
            "title": "Sights"
          }
        },
        {
          "type": "to-do",
          "attributes": {
            "title": "Vatican City"
          }
        },
        {
          "type": "to-do",
          "attributes": {
            "title": "The Colosseum",
            "notes": "12€"
          }
        },
        {
          "type": "heading",
          "attributes": {
            "title": "Planning"
          }
        },
        {
          "type": "to-do",
          "attributes": {
            "title": "Call Paolo",
            "completed": true
          }
        },
        {
          "type": "to-do",
          "attributes": {
            "title": "Book flights",
            "when": "today"
          }
        },
        {
          "type": "to-do",
          "attributes": {
            "title": "Research",
            "checklist-items": [
              {
                "type": "checklist-item",
                "attributes": {
                  "title": "Hotels",
                  "completed": true
                }
              },
              {
                "type": "checklist-item",
                "attributes": {
                  "title": "Transport from airport"
                }
              }
            ]
          }
        }
      ]
    }
  },
  {
    "type": "to-do",
    "attributes": {
      "title": "Pick up dry cleaning",
      "when": "evening",
      "tags": ["Errand"]
    }
  },
  {
    "type": "to-do",
    "attributes": {
      "title": "Submit report",
      "deadline": "2026-02-01",
      "list": "Work"
    }
  }
]
```

---

## URL 编码

所有 JSON 示例在使用前必须删除空白字符并进行 URL 编码。

**原始：**
```
things:///json?data=
  [
    {
      "type": "to-do",
      "attributes": {
        "title": "Buy milk"
      }
    }
  ]
```

**删除空白字符后：**
```
things:///json?data=[{"type":"to-do","attributes":{"title":"Buy milk"}}]
```

**URL 编码后：**
```
things:///json?data=%5B%7B%22type%22:%22to-do%22,%22attributes%22:%7B%22title%22:%22Buy%20milk%22%7D%7D%5D
```
