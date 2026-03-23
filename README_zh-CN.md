# 🕷️ VisualSpider

<p align="center">
  <img src="https://img.shields.io/badge/Version-1.0.1-blue.svg" alt="版本">
  <img src="https://img.shields.io/badge/License-MIT-green.svg" alt="许可证">
  <img src="https://img.shields.io/badge/Vue-3.4-brightgreen.svg" alt="Vue">
  <img src="https://img.shields.io/badge/TypeScript-5.4-blue.svg" alt="TypeScript">
  <img src="https://img.shields.io/badge/Node.js-18%2B-brightgreen.svg" alt="Node">
</p>

> 🌍 **[English](README.md)** | **[中文](README_zh-CN.md)**

---

## 📖 项目概述

**VisualSpider** 是一款功能强大、操作简便的可视化网页爬虫工具。用户无需编写代码，即可从任意网站提取数据。系统基于 Vue 3 和 Element Plus 构建，提供直观的图形化界面，支持配置爬取任务、测试选择器、数据清洗及多格式导出。

### ✨ 核心功能

- 🎨 **可视化配置** - 通过点击界面配置爬取任务
- 🔍 **智能选择器测试** - 实时测试 CSS 和 XPath 选择器
- 🧹 **数据清洗** - 内置文本处理和数据规范化工具
- 📊 **多格式导出** - 支持 CSV、JSON、Excel、HTML、PDF、Markdown 等格式
- 🌐 **国际化支持** - 完整的英文和中文语言支持
- 🔄 **浏览器自动化** - 基于 Puppeteer 的后端，支持 JavaScript 渲染页面
- 🖥️ **界面适配** - 智能处理复杂网站结构
- 📸 **截图捕获** - 支持整页和区域截图及标注
- 🛡️ **错误处理** - 清晰的错误消息和全面的日志记录
- ⚡ **性能优化** - 高效的批处理和缓存机制
- 🕵️ **反检测** - 隐身模式，随机 User-Agent 轮换

---

## 🚀 快速开始

### 环境要求

| 要求 | 版本 | 说明 |
|------|------|------|
| Node.js | ≥ 18.0.0 | [下载](https://nodejs.org/) |
| npm | ≥ 9.0.0 | 随 Node.js 一起安装 |
| Chromium | 可选 | 用于 JS 渲染页面 |

### 安装步骤

```bash
# 1. 克隆仓库
git clone https://github.com/badhope/VisualSpider.git
cd VisualSpider

# 2. 安装前端依赖
npm install

# 3. 安装后端依赖（可选）
cd server && npm install
cd ..

# 4. 启动开发服务器
npm run dev
```

### 环境配置（可选）

如需 Puppeteer 浏览器自动化支持：

```bash
# 设置浏览器和环境
npm run setup:env

# 或手动设置服务器
cd server && npm run setup-browser
```

### 访问应用

| 服务 | 地址 | 说明 |
|------|------|------|
| 前端 | http://localhost:1420 | 主应用 |
| 后端 API | http://localhost:3000 | API 文档 |
| 健康检查 | http://localhost:3000/api/health | 服务器状态 |

---

## 🎯 使用指南

### 1. 创建爬取任务

1. 进入 **任务配置** 页面
2. 输入目标网址（如 `https://example.com/products`）
3. 点击 **自动检测** 或手动添加数据选择器
4. 如需分页，配置分页选项
5. 点击 **开始抓取**

### 2. 配置选择器

支持的選擇器類型：

| 类型 | 示例 | 用途 |
|------|------|------|
| CSS | `.product-card .title` | 基于类的元素 |
| XPath | `//div[@class='title']` | 复杂查询 |
| Regex | `\d+\.\d{2}` | 模式匹配 |

### 3. 数据清洗

进入 **数据清洗** 页面上传原始数据文件：

- **转换**：去重、去除空白、查找替换
- **格式转换**：CSV、JSON、Excel、XML、HTML 互转
- **文本分析**：统计、关键词、情感分析
- **正则工具**：测试和应用正则表达式

### 4. 导出结果

| 格式 | 扩展名 | 适用场景 |
|------|--------|----------|
| CSV | .csv | 表格、数据库 |
| JSON | .json | API、结构化数据 |
| Excel | .xlsx | Microsoft Excel |
| HTML | .html | 网页、报告 |
| PDF | .pdf | 文档 |
| Markdown | .md | GitHub、Wiki |
| TSV | .tsv | 制表符分隔数据 |
| XML | .xml | 结构化文档 |

---

## 🏗️ 项目结构

```
visual-spider/
├── src/
│   ├── components/          # 可复用 Vue 组件
│   │   ├── LanguageSwitcher.vue   # 语言切换
│   │   ├── LogViewer.vue          # 日志查看
│   │   ├── NotificationPanel.vue   # 通知面板
│   │   ├── ErrorDetail.vue         # 错误详情
│   │   └── WelcomeGuide.vue       # 欢迎指南
│   ├── views/              # 页面组件
│   │   ├── TaskConfig.vue       # 任务配置
│   │   ├── TaskList.vue         # 任务列表
│   │   ├── DataClean.vue        # 数据清洗
│   │   ├── SelectorTester.vue   # 选择器测试
│   │   ├── Screenshot.vue        # 截图捕获
│   │   ├── UrlAnalyzer.vue      # URL 分析
│   │   ├── ServerManager.vue    # 服务器管理
│   │   └── Settings.vue         # 应用设置
│   ├── locales/             # 国际化文件
│   │   ├── en.json          # 英文
│   │   └── zh-CN.json       # 中文
│   ├── stores/              # Pinia 状态管理
│   ├── services/            # API 服务
│   ├── utils/               # 工具函数
│   └── types/               # TypeScript 类型定义
├── server/                  # 后端 (Express + Puppeteer)
│   └── src/
│       ├── routes/          # API 端点
│       │   ├── crawler.ts    # 爬取逻辑
│       │   ├── browser.ts    # 浏览器自动化
│       │   └── proxy.ts      # 代理管理
│       └── utils/           # 服务器工具
├── public/
│   └── test-pages/         # 测试页面
├── .github/
│   └── workflows/          # CI/CD 流程
└── dist/                    # 生产构建
```

---

## 🌍 国际化

VisualSpider 支持 **英文** 和 **中文**。点击顶部导航栏的 🌐 图标切换语言。

### 支持的语言

| 代码 | 语言 | 状态 |
|------|------|------|
| en | English | ✅ 完整 |
| zh-CN | 简体中文 | ✅ 完整 |

### 添加新语言

1. 创建 `src/locales/{locale}.json` 文件
2. 在 `src/locales/index.ts` 中添加语言配置
3. 更新 `LanguageSwitcher.vue` 组件

---

## 🔧 高级功能

### 反检测设置

爬虫包含隐身模式以避免被封锁：

- **随机 User-Agent 轮换** - 6 种浏览器配置
- **隐身模式** - 隐藏 webdriver 属性
- **Navigator 覆盖** - 掩码自动化特征
- **请求拦截** - 屏蔽追踪资源

### 代理池

在 **设置** 中配置代理服务器：

```bash
# HTTP 代理
http://proxy.example.com:8080

# HTTPS 代理
https://proxy.example.com:8080

# SOCKS5 代理
socks5://proxy.example.com:1080

# 认证代理
http://user:pass@proxy.example.com:8080
```

### 浏览器自动化（后端）

后端服务器提供以下功能：

| 功能 | 说明 |
|------|------|
| 启动浏览器 | 启动无头 Chrome |
| 页面导航 | 打开网址 |
| 执行脚本 | 运行 JavaScript 代码 |
| 截图 | 捕获页面图片 |
| Cookie 管理 | 管理认证状态 |

---

## 📝 API 参考

### 爬虫端点

| 方法 | 端点 | 说明 |
|------|------|------|
| POST | /api/crawler/crawl | 开始爬取任务 |
| GET | /api/crawler/tasks | 列出所有任务 |
| GET | /api/crawler/task/:id | 获取任务详情 |
| DELETE | /api/crawler/task/:id | 删除任务 |

### 浏览器端点

| 方法 | 端点 | 说明 |
|------|------|------|
| POST | /api/browser/launch | 启动浏览器 |
| POST | /api/browser/close | 关闭浏览器 |
| POST | /api/browser/screenshot | 截图 |
| POST | /api/browser/evaluate | 运行 JS 代码 |

### 代理端点

| 方法 | 端点 | 说明 |
|------|------|------|
| GET | /api/proxy/list | 列出代理 |
| POST | /api/proxy/add | 添加代理 |
| POST | /api/proxy/test | 测试代理 |
| DELETE | /api/proxy/:id | 删除代理 |

---

## 🔒 上传说明

### 数据准备

#### 文件格式要求

| 格式 | 扩展名 | 最大大小 | 编码 |
|------|--------|----------|------|
| CSV | .csv | 50MB | UTF-8 |
| JSON | .json | 20MB | UTF-8 |
| Excel | .xlsx | 20MB | - |
| 文本 | .txt | 10MB | UTF-8 |

#### 命名规范

```
# 文件命名规则：
- 仅使用字母和数字
- 不使用空格，使用下划线或连字符
- 如有关联日期：data_20240115.csv
- 应具有描述性：product_list_q1.csv

# 示例：
✅ valid_file_name.csv
✅ product-data-2024.csv
✅ article_titles.json
❌ my file.csv（包含空格）
❌ file@name.csv（特殊字符）
```

### 上传步骤

1. **准备数据文件**
   - 转换为支持的格式（CSV、JSON、Excel）
   - 确保使用 UTF-8 编码
   - 遵循命名规范

2. **进入数据清洗页面**
   - 点击 "导入数据" 或 "快速导入"
   - 选择文件格式

3. **配置处理选项**
   - 选择转换操作
   - 如需要设置正则表达式
   - 选择输出格式

4. **导出处理后的数据**
   - 选择目标格式
   - 点击导出按钮
   - 保存到本地目录

### 错误处理

| 错误 | 原因 | 解决方案 |
|------|------|----------|
| 文件过大 | 超过大小限制 | 拆分为更小的文件 |
| 编码无效 | 非 UTF-8 内容 | 重新保存为 UTF-8 编码 |
| 解析错误 | 文件格式损坏 | 检查文件结构 |
| 格式不匹配 | 文件类型错误 | 验证文件扩展名 |

### 验证方法

1. **预览**：处理前检查数据预览
2. **验证**：使用"测试"按钮测试正则表达式
3. **导出样本**：导出前 100 行进行验证
4. **校验和**：比较上传前后的文件哈希

---

## 🤝 贡献指南

欢迎贡献代码！请在提交 Pull Request 前阅读 [贡献指南](CONTRIBUTING.md)。

### 开发环境设置

```bash
# 安装依赖
npm install

# 启动开发服务器
npm run dev

# 类型检查
npm run typecheck

# 代码检查
npm run lint

# 运行测试
npm run test

# 生产构建
npm run build
```

### 代码规范

- 使用 **Vue 3 Composition API**
- 遵循 **TypeScript** 最佳实践
- 使用 **Element Plus** 组件
- 遵循现有命名规范

---

## 📄 许可证

本项目采用 MIT 许可证 - 详见 [LICENSE](LICENSE) 文件。

---

## 🙏 致谢

- [Vue.js](https://vuejs.org/) - 渐进式 JavaScript 框架
- [Element Plus](https://element-plus.org/) - Vue 3 UI 组件库
- [Puppeteer](https://pptr.dev/) - 无头 Chrome Node.js API
- [xlsx](https://sheetjs.com/) - SheetJS Excel/CSV 解析器
- [html2canvas](https://html2canvas.hertzen.com/) - HTML 转 Canvas 工具

---

## 📧 联系与支持

| 渠道 | 链接 |
|------|------|
| GitHub Issues | [报告问题或请求功能](https://github.com/badhope/VisualSpider/issues) |
| 文档 | [Wiki](https://github.com/badhope/VisualSpider/wiki) |

---

<p align="center">
  <strong>❤️ 由 VisualSpider 团队用心打造</strong>
  <br>
  <sub>如果觉得有用，请给我们一个 ⭐！</sub>
</p>
