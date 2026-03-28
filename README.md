<div align="center">

# 🛠️ WindowsTools

### Windows 系统工具箱

**一款功能强大的 Windows 系统管理可视化工具**

将复杂的命令行操作、注册表编辑、系统设置等高级功能转化为直观的图形界面操作

[![Version](https://img.shields.io/badge/Version-1.0.0-blue.svg?style=for-the-badge)](https://github.com/badhope/VisualSpider)
[![License](https://img.shields.io/badge/License-MIT-green.svg?style=for-the-badge)](LICENSE)
[![Tauri](https://img.shields.io/badge/Tauri-2.0-FFC131.svg?style=for-the-badge)](https://tauri.app)
[![Vue](https://img.shields.io/badge/Vue-3.4-4FC08D.svg?style=for-the-badge)](https://vuejs.org)
[![Rust](https://img.shields.io/badge/Rust-1.75+-DEA584.svg?style=for-the-badge)](https://www.rust-lang.org)
[![TypeScript](https://img.shields.io/badge/TypeScript-5.4-3178C6.svg?style=for-the-badge)](https://www.typescriptlang.org)

[![Stars](https://img.shields.io/github/stars/badhope/VisualSpider?style=social)](https://github.com/badhope/VisualSpider/stargazers)
[![Forks](https://img.shields.io/github/forks/badhope/VisualSpider?style=social)](https://github.com/badhope/VisualSpider/network/members)
[![Issues](https://img.shields.io/github/issues/badhope/VisualSpider)](https://github.com/badhope/VisualSpider/issues)
[![Downloads](https://img.shields.io/github/downloads/badhope/VisualSpider/total)](https://github.com/badhope/VisualSpider/releases)

[English](#english) | [简体中文](#简体中文)

</div>

---

## 简体中文

### 📖 简介

VisualSpider 是一款面向 Windows 用户的专业系统管理工具，采用现代化的技术栈（Tauri 2.0 + Vue 3 + Rust）构建，提供流畅的用户体验和强大的系统管理能力。

### ✨ 核心功能

<table>
<tr>
<td width="50%">

#### 🖥️ 系统监控
- **仪表盘** - 系统信息概览、快速访问入口
- **进程管理** - 实时监控、CPU/内存排序、优先级调整
- **服务管理** - 查看/启动/停止/重启 Windows 服务

</td>
<td width="50%">

#### 🔧 系统工具
- **PowerShell 命令中心** - 可视化执行、命令模板、历史记录
- **注册表管理器** - 可视化浏览、编辑、搜索、导出注册表
- **网络工具** - 连接查看、端口占用分析、DNS 管理

</td>
</tr>
<tr>
<td width="50%">

#### 💾 磁盘与优化
- **磁盘工具** - 空间分析、磁盘清理、健康检查
- **系统优化** - 启动项管理、计划任务、临时文件清理
- **快捷操作** - 一键打开常用系统工具

</td>
<td width="50%">

#### ⚙️ 高级功能
- **环境变量编辑器** - 用户/系统环境变量管理
- **Hosts 文件编辑器** - 可视化编辑 Hosts 映射
- **系统修复** - SFC/DISM 系统文件修复

</td>
</tr>
</table>

### 🌐 国际化支持

- 🇨🇳 简体中文
- 🇺🇸 English
- 可在设置中一键切换语言

### 📸 应用截图

> 截图将展示各功能模块界面

### 🚀 快速开始

#### 环境要求

| 依赖 | 版本要求 | 说明 |
|:----:|:--------:|:----:|
| ![Node.js](https://img.shields.io/badge/Node.js-≥18.0.0-339933?logo=node.js) | ≥ 18.0.0 | 前端运行环境 |
| ![Rust](https://img.shields.io/badge/Rust-≥1.75.0-DEA584?logo=rust) | ≥ 1.75.0 | Tauri 后端编译 |
| ![Windows](https://img.shields.io/badge/Windows-10/11-0078D6?logo=windows) | 10/11 | 操作系统 |
| Visual Studio Build Tools | 最新版 | Windows 原生编译 |
| WebView2 | 最新版 | 应用界面渲染 |

#### 安装方式

**方式一：下载发布版本（推荐）**

前往 [Releases](https://github.com/badhope/VisualSpider/releases) 页面下载最新版本的安装包。

**方式二：从源码构建**

```bash
# 克隆仓库
git clone https://github.com/badhope/VisualSpider.git
cd VisualSpider

# 安装依赖
npm install

# 开发模式运行
npm run tauri dev

# 构建发布版本
npm run tauri build
```

**方式三：自动环境配置**

```bash
# 检测并自动安装所需依赖
npm run setup
```

### 📦 项目结构

```
VisualSpider/
├── 📂 src/                    # 前端源码
│   ├── 📂 views/             # 页面视图组件
│   ├── 📂 components/        # 公共组件
│   ├── 📂 stores/            # Pinia 状态管理
│   ├── 📂 locales/           # 国际化语言包
│   ├── 📂 types/             # TypeScript 类型定义
│   └── 📂 router/            # Vue Router 路由配置
├── 📂 src-tauri/             # Tauri 后端
│   ├── 📂 src/
│   │   ├── 📄 system.rs      # 系统操作模块
│   │   ├── 📄 registry.rs    # 注册表操作
│   │   ├── 📄 process.rs     # 进程管理
│   │   ├── 📄 service.rs     # 服务管理
│   │   ├── 📄 network.rs     # 网络工具
│   │   └── 📄 disk.rs        # 磁盘工具
│   └── 📄 Cargo.toml         # Rust 依赖配置
├── 📂 scripts/               # 构建和安装脚本
└── 📄 package.json           # Node.js 项目配置
```

### 🛠️ 技术栈

<table>
<tr>
<th>层级</th>
<th>技术</th>
<th>版本</th>
</tr>
<tr>
<td>前端框架</td>
<td><a href="https://vuejs.org">Vue 3</a></td>
<td>3.4+</td>
</tr>
<tr>
<td>UI 组件库</td>
<td><a href="https://element-plus.org">Element Plus</a></td>
<td>2.6+</td>
</tr>
<tr>
<td>状态管理</td>
<td><a href="https://pinia.vuejs.org">Pinia</a></td>
<td>2.1+</td>
</tr>
<tr>
<td>国际化</td>
<td><a href="https://vue-i18n.intlify.dev">Vue I18n</a></td>
<td>9.10+</td>
</tr>
<tr>
<td>桌面框架</td>
<td><a href="https://tauri.app">Tauri</a></td>
<td>2.0</td>
</tr>
<tr>
<td>后端语言</td>
<td><a href="https://www.rust-lang.org">Rust</a></td>
<td>1.75+</td>
</tr>
<tr>
<td>系统调用</td>
<td><a href="https://docs.microsoft.com/en-us/windows/win32/api/">Windows API</a></td>
<td>-</td>
</tr>
</table>

### 📝 开发命令

```bash
# 开发
npm run dev              # 启动前端开发服务器
npm run tauri dev        # 启动 Tauri 开发模式

# 构建
npm run build            # 构建前端
npm run tauri build      # 构建桌面应用安装包

# 代码质量
npm run lint             # ESLint 代码检查
npm run lint:check       # 仅检查不修复
npm run format           # Prettier 格式化代码
npm run format:check     # 检查格式
npm run typecheck        # TypeScript 类型检查

# 测试
npm run test             # 运行测试
npm run test:run         # 单次运行测试

# 环境
npm run check:env        # 检测环境依赖
npm run setup            # 自动安装依赖
```

### 🤝 参与贡献

我们欢迎所有形式的贡献！

#### 贡献方式

1. **报告问题** - 在 [Issues](https://github.com/badhope/VisualSpider/issues) 提交 Bug 报告或功能建议
2. **提交代码** - Fork 项目并提交 Pull Request
3. **完善文档** - 帮助改进文档和翻译
4. **分享项目** - 向他人推荐此项目

#### 开发流程

```bash
# 1. Fork 并克隆
git clone https://github.com/YOUR_USERNAME/VisualSpider.git

# 2. 创建功能分支
git checkout -b feature/your-feature

# 3. 进行开发并测试
npm run tauri dev
npm run lint
npm run typecheck

# 4. 提交更改
git commit -m "feat: add your feature"

# 5. 推送并创建 PR
git push origin feature/your-feature
```

### 📜 更新日志

查看 [CHANGELOG.md](CHANGELOG.md) 了解版本更新历史。

### 📄 许可证

本项目基于 [MIT License](LICENSE) 开源。

### 🙏 致谢

感谢以下开源项目：

- [Tauri](https://tauri.app) - 构建更小、更快、更安全的桌面应用
- [Vue.js](https://vuejs.org) - 渐进式 JavaScript 框架
- [Element Plus](https://element-plus.org) - Vue 3 UI 组件库
- [Rust](https://www.rust-lang.org) - 系统编程语言

---

<div align="center">

**如果这个项目对你有帮助，请给一个 ⭐ Star 支持一下！**

[![Star History Chart](https://api.star-history.com/svg?repos=badhope/VisualSpider&type=Date)](https://star-history.com/#badhope/VisualSpider&Date)

**Made with ❤️ by [badhope](https://github.com/badhope)**

</div>

---

## English

### 📖 Introduction

VisualSpider is a professional system management tool for Windows users, built with modern technology stack (Tauri 2.0 + Vue 3 + Rust), providing a smooth user experience and powerful system management capabilities.

### ✨ Core Features

<table>
<tr>
<td width="50%">

#### 🖥️ System Monitoring
- **Dashboard** - System overview, quick access
- **Process Manager** - Real-time monitoring, CPU/memory sorting, priority adjustment
- **Service Manager** - View/start/stop/restart Windows services

</td>
<td width="50%">

#### 🔧 System Tools
- **PowerShell Command Center** - Visual execution, command templates, history
- **Registry Manager** - Visual browsing, editing, searching, exporting
- **Network Tools** - Connection view, port analysis, DNS management

</td>
</tr>
<tr>
<td width="50%">

#### 💾 Disk & Optimization
- **Disk Tools** - Space analysis, disk cleanup, health check
- **System Optimization** - Startup items, scheduled tasks, temp file cleanup
- **Quick Actions** - One-click access to system tools

</td>
<td width="50%">

#### ⚙️ Advanced Features
- **Environment Variables Editor** - User/system environment management
- **Hosts File Editor** - Visual Hosts mapping editor
- **System Repair** - SFC/DISM system file repair

</td>
</tr>
</table>

### 🌐 Internationalization

- 🇨🇳 Simplified Chinese
- 🇺🇸 English
- One-click language switching in settings

### 🚀 Quick Start

#### Requirements

| Dependency | Version | Description |
|:----------:|:-------:|:-----------:|
| Node.js | ≥ 18.0.0 | Frontend runtime |
| Rust | ≥ 1.75.0 | Tauri backend compilation |
| Windows | 10/11 | Operating System |
| Visual Studio Build Tools | Latest | Windows native compilation |
| WebView2 | Latest | Application rendering |

#### Installation

**Option 1: Download Release (Recommended)**

Visit the [Releases](https://github.com/badhope/WindowsTools/releases) page to download the latest installer.

**Option 2: Build from Source**

```bash
# Clone repository
git clone https://github.com/badhope/VisualSpider.git
cd VisualSpider

# Install dependencies
npm install

# Development mode
npm run tauri dev

# Build release
npm run tauri build
```

### 🤝 Contributing

We welcome all forms of contribution!

1. **Report Issues** - Submit bug reports or feature suggestions
2. **Submit Code** - Fork and create Pull Requests
3. **Improve Documentation** - Help improve docs and translations
4. **Share** - Recommend this project to others

### 📄 License

This project is open-sourced under the [MIT License](LICENSE).

---

<div align="center">

**If this project helps you, please give it a ⭐ Star!**

**Made with ❤️ by [badhope](https://github.com/badhope)**

</div>
