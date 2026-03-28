@echo off
chcp 65001 >nul
title Windows 工具箱 - 启动程序

echo.
echo ========================================
echo   Windows 工具箱 - 启动程序
echo ========================================
echo.

powershell -ExecutionPolicy Bypass -File "%~dp0check-environment.ps1" -CheckOnly

if %errorlevel% neq 0 (
    echo.
    echo 检测到环境问题，正在尝试自动修复...
    powershell -ExecutionPolicy Bypass -File "%~dp0check-environment.ps1" -Install -Silent
)

echo.
echo 正在启动应用...
echo.

cd /d "%~dp0.."
call npm run tauri dev

pause
