@echo off
chcp 65001 >nul
title Windows 工具箱 - 安装向导

echo.
echo ========================================
echo   Windows 工具箱 - 安装向导
echo ========================================
echo.
echo 此向导将帮助您安装运行此应用所需的所有依赖
echo.

powershell -ExecutionPolicy Bypass -File "%~dp0check-environment.ps1" -Install

echo.
echo ========================================
echo   安装完成!
echo ========================================
echo.
echo 您现在可以运行 start.bat 启动应用
echo.

pause
