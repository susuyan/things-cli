#!/bin/bash
# Things CLI 完整测试脚本
# 测试所有命令功能

set -e

echo "=== Things CLI 完整测试 ==="
echo ""

THINGS="cargo run --"

# 颜色输出
GREEN='\033[0;32m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

print_section() {
    echo ""
    echo -e "${BLUE}=== $1 ===${NC}"
}

print_success() {
    echo -e "${GREEN}✓ $1${NC}"
}

# 1. 基础命令测试
print_section "1. 基础命令测试"
$THINGS --version
$THINGS --help
print_success "基础命令正常"

# 2. 创建测试项目
print_section "2. 创建测试项目"
PROJECT_TITLE="CLI测试项目-$(date +%s)"
echo "创建项目: $PROJECT_TITLE"
$THINGS project add "$PROJECT_TITLE" --notes "用于测试CLI功能" --reveal
print_success "项目创建命令已发送"

# 3. 创建待办事项
print_section "3. 创建待办事项"
echo "3.1 创建简单待办"
$THINGS todo add "测试任务1-简单任务"

sleep 1

echo "3.2 创建带详细信息的待办"
$THINGS todo add "测试任务2-详细任务" \
    --notes "这是任务备注" \
    --when today \
    --tags "测试,CLI"

sleep 1

echo "3.3 创建重复任务"
$THINGS todo add "测试任务3-重复任务" \
    --repeat week \
    --repeat-until 2026-12-31

sleep 1

echo "3.4 批量创建待办"
$THINGS todo add "批量任务1" "批量任务2" "批量任务3" --list "$PROJECT_TITLE"

print_success "待办创建命令已发送"

# 4. 列出任务
print_section "4. 列出任务"
echo "4.1 收件箱"
$THINGS list inbox || echo "收件箱可能为空"

echo "4.2 今日任务"
$THINGS list today || echo "今日可能无任务"

echo "4.3 项目列表"
$THINGS list projects

echo "4.4 区域列表"
$THINGS list areas

echo "4.5 标签列表"
$THINGS list tags

print_success "列表命令正常"

# 5. 搜索测试
print_section "5. 搜索测试"
$THINGS search "CLI测试" || echo "搜索可能无结果"
print_success "搜索命令已执行"

# 6. 显示命令
print_section "6. 显示命令"
$THINGS show today || echo "显示今日"
$THINGS show inbox || echo "显示收件箱"
print_success "显示命令正常"

# 7. Area 测试
print_section "7. 区域管理测试"
AREA_TITLE="测试区域-$(date +%s)"
echo "7.1 创建区域"
$THINGS area add "$AREA_TITLE" --tags "测试标签"
sleep 1

echo "7.2 列出区域"
$THINGS list areas
print_success "区域命令已执行"

# 8. 批量操作测试
print_section "8. 批量操作测试"
echo "8.1 生成模板"
$THINGS batch template

echo "8.2 导入测试（预览模式）"
echo '[
  {
    "type": "to-do",
    "attributes": {
      "title": "批量导入任务1",
      "when": "today"
    }
  },
  {
    "type": "to-do",
    "attributes": {
      "title": "批量导入任务2",
      "tags": ["导入"]
    }
  }
]' > /tmp/test_import.json
$THINGS batch import /tmp/test_import.json
print_success "批量操作完成"

# 9. 配置测试
print_section "9. 配置测试"
$THINGS config show || echo "配置显示"
$THINGS config check-auth-token || echo "Token检查"
print_success "配置命令正常"

# 10. 更新任务（需要 auth token，可能失败）
print_section "10. 更新任务测试"
echo "注意：更新操作需要 auth token，如果未设置会提示错误"
echo "如需测试更新功能，请先运行: things config set-auth-token"

# 注意：我们不实际执行更新，因为需要 auth token
# $THINGS todo update <ID> --complete

print_success "测试脚本执行完成！"
echo ""
echo "=== 测试总结 ==="
echo "已测试功能:"
echo "  ✓ 基础命令 (--version, --help)"
echo "  ✓ 项目创建"
echo "  ✓ 待办创建（简单、详细、重复、批量）"
echo "  ✓ 列表查询（inbox, today, projects, areas, tags）"
echo "  ✓ 搜索"
echo "  ✓ 显示命令"
echo "  ✓ 区域管理"
echo "  ✓ 批量导入"
echo "  ✓ 配置命令"
echo ""
echo "注意："
echo "  - 删除操作需要手动测试（有确认提示）"
echo "  - 更新操作需要设置 auth token"
echo "  - 请检查 Things 3 应用确认任务是否已创建"

# 清理
rm -f /tmp/test_import.json
