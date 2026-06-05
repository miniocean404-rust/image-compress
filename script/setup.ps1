# Rust Skills 设置脚本

# 确定插件目录路径
$custom_path = "D:\soft-dev\code\rust\rust-skills"
if (Test-Path $custom_path) {
    $plugin_dir = $custom_path
} else {
    $plugin_dir = $PSScriptRoot
}

Write-Host "正在为 Claude Code 设置 Rust Skills..."

# 如果权限文件不存在则创建
$settings_file = ".claude\settings.local.json"
if (-not (Test-Path $settings_file)) {
    New-Item -ItemType Directory -Force -Path ".claude" | Out-Null

    $settings = @{
        permissions = @{
            allow = @(
                "Bash(agent-browser *)"
            )
        }
    } | ConvertTo-Json -Depth 10

    Set-Content -Path $settings_file -Value $settings
    Write-Host "已创建 .claude/settings.local.json 并添加 agent-browser 权限"
} else {
    Write-Host ".claude/settings.local.json 已存在，请手动添加权限："
    Write-Host '  "Bash(agent-browser *)"'
}

Write-Host "设置完成！"
Write-Host ""
Write-Host "使用方法："
Write-Host "claude --plugin-dir $plugin_dir"
