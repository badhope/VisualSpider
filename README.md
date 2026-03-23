# 🕷️ VisualSpider

<p align="center">
  <img src="https://img.shields.io/badge/Version-1.0.1-blue.svg" alt="Version">
  <img src="https://img.shields.io/badge/License-MIT-green.svg" alt="License">
  <img src="https://img.shields.io/badge/Vue-3.4-brightgreen.svg" alt="Vue">
  <img src="https://img.shields.io/badge/TypeScript-5.4-blue.svg" alt="TypeScript">
  <img src="https://img.shields.io/badge/Node.js-18%2B-brightgreen.svg" alt="Node">
</p>

> 🌍 **[English](README.md)** | **[中文](README_zh-CN.md)**

---

## 📖 Overview

**VisualSpider** is a powerful, user-friendly visual web crawler that enables users to extract data from any website without writing code. Built with Vue 3 and Element Plus, it provides an intuitive graphical interface for configuring crawling tasks, testing selectors, cleaning data, and exporting results in multiple formats.

### ✨ Key Features

- 🎨 **Visual Configuration** - Point-and-click interface for configuring crawl tasks
- 🔍 **Smart Selector Testing** - Test CSS and XPath selectors in real-time
- 🧹 **Data Cleaning** - Built-in tools for text processing and data normalization
- 📊 **Multiple Export Formats** - Export to CSV, JSON, Excel, HTML, PDF, Markdown, and more
- 🌐 **Internationalization** - Full support for English and Chinese languages
- 🔄 **Browser Automation** - Puppeteer-powered backend for JavaScript-rendered pages
- 🖥️ **Interface Adaptation** - Intelligent handling of complex website structures
- 📸 **Screenshot Capture** - Full-page and region screenshots with annotations
- 🛡️ **Error Handling** - Clear error messages and comprehensive logging
- ⚡ **Performance Optimization** - Efficient batch processing and caching
- 🕵️ **Anti-Detection** - Stealth mode with random User-Agent rotation

---

## 🚀 Quick Start

### Prerequisites

| Requirement | Version | Notes |
|------------|---------|-------|
| Node.js | ≥ 18.0.0 | [Download](https://nodejs.org/) |
| npm | ≥ 9.0.0 | Included with Node.js |
| Chromium | Optional | Required for JS-rendered pages |

### Installation

```bash
# 1. Clone the repository
git clone https://github.com/badhope/VisualSpider.git
cd VisualSpider

# 2. Install frontend dependencies
npm install

# 3. Install backend dependencies (optional)
cd server && npm install
cd ..

# 4. Start development server
npm run dev
```

### Environment Setup (Optional)

For Puppeteer browser automation support:

```bash
# Setup browser and environment
npm run setup:env

# Or run server setup manually
cd server && npm run setup-browser
```

### Access the Application

| Service | URL | Description |
|---------|-----|-------------|
| Frontend | http://localhost:1420 | Main application |
| Backend API | http://localhost:3000 | API documentation |
| Health Check | http://localhost:3000/api/health | Server status |

---

## 🎯 Usage Guide

### 1. Create a Crawl Task

1. Navigate to **Task Config** page
2. Enter the target URL (e.g., `https://example.com/products`)
3. Click **Auto Detect** or manually add data selectors
4. Configure pagination if needed
5. Click **Start Crawling**

### 2. Configure Selectors

Supported selector types:

| Type | Example | Use Case |
|------|---------|----------|
| CSS | `.product-card .title` | Class-based elements |
| XPath | `//div[@class='title']` | Complex queries |
| Regex | `\d+\.\d{2}` | Pattern matching |

### 3. Data Cleaning

Upload raw data files to **Data Clean** page:

- **Transform**: Remove duplicates, trim whitespace, find & replace
- **Format Convert**: Convert between CSV, JSON, Excel, XML, HTML
- **Text Analysis**: Statistics, keywords, sentiment analysis
- **Regex Tool**: Test and apply regular expressions

### 4. Export Results

| Format | Extension | Best For |
|--------|-----------|----------|
| CSV | .csv | Spreadsheets, databases |
| JSON | .json | APIs, structured data |
| Excel | .xlsx | Microsoft Excel |
| HTML | .html | Web pages, reports |
| PDF | .pdf | Documentation |
| Markdown | .md | GitHub, wikis |
| TSV | .tsv | Tab-separated data |
| XML | .xml | Structured documents |

---

## 🏗️ Project Structure

```
visual-spider/
├── src/
│   ├── components/          # Reusable Vue components
│   │   ├── LanguageSwitcher.vue
│   │   ├── LogViewer.vue
│   │   ├── NotificationPanel.vue
│   │   ├── ErrorDetail.vue
│   │   └── WelcomeGuide.vue
│   ├── views/              # Page components
│   │   ├── TaskConfig.vue       # Crawl task configuration
│   │   ├── TaskList.vue         # Task management
│   │   ├── DataClean.vue        # Data cleaning tools
│   │   ├── SelectorTester.vue   # Selector testing
│   │   ├── Screenshot.vue        # Screenshot capture
│   │   ├── UrlAnalyzer.vue      # URL analysis
│   │   ├── ServerManager.vue    # Backend management
│   │   └── Settings.vue         # Application settings
│   ├── locales/             # i18n translations
│   │   ├── en.json          # English
│   │   └── zh-CN.json       # Chinese
│   ├── stores/              # Pinia state management
│   ├── services/            # API services
│   ├── utils/               # Utility functions
│   └── types/               # TypeScript definitions
├── server/                  # Backend (Express + Puppeteer)
│   └── src/
│       ├── routes/          # API endpoints
│       │   ├── crawler.ts    # Crawling logic
│       │   ├── browser.ts    # Browser automation
│       │   └── proxy.ts      # Proxy management
│       └── utils/           # Server utilities
├── public/
│   └── test-pages/         # Test pages
├── .github/
│   └── workflows/          # CI/CD pipelines
└── dist/                    # Production build
```

---

## 🌍 Internationalization

VisualSpider supports **English** and **Chinese**. Click the 🌐 icon in the header to switch languages.

### Supported Languages

| Code | Language | Status |
|------|----------|--------|
| en | English | ✅ Complete |
| zh-CN | 简体中文 | ✅ Complete |

### Adding New Languages

1. Create `src/locales/{locale}.json`
2. Add locale to `src/locales/index.ts`
3. Update `LanguageSwitcher.vue`

---

## 🔧 Advanced Features

### Anti-Detection Settings

The crawler includes stealth mode to avoid blocking:

- **Random User-Agent Rotation** - 6 browser profiles
- **Stealth Mode** - Hides webdriver property
- **Navigator Override** - Masks automation signatures
- **Request Interception** - Blocks tracking resources

### Proxy Pool

Configure proxy servers in **Settings**:

```bash
# HTTP proxy
http://proxy.example.com:8080

# HTTPS proxy
https://proxy.example.com:8080

# SOCKS5 proxy
socks5://proxy.example.com:1080

# Authenticated proxy
http://user:pass@proxy.example.com:8080
```

### Browser Automation (Backend)

The backend server provides:

| Feature | Description |
|---------|-------------|
| Launch Browser | Start headless Chrome |
| Navigate | Open URLs |
| Evaluate | Execute JavaScript |
| Screenshot | Capture page images |
| Cookies | Manage authentication |

---

## 📝 API Reference

### Crawler Endpoints

| Method | Endpoint | Description |
|--------|----------|-------------|
| POST | /api/crawler/crawl | Start crawl task |
| GET | /api/crawler/tasks | List all tasks |
| GET | /api/crawler/task/:id | Get task details |
| DELETE | /api/crawler/task/:id | Delete task |

### Browser Endpoints

| Method | Endpoint | Description |
|--------|----------|-------------|
| POST | /api/browser/launch | Launch browser |
| POST | /api/browser/close | Close browser |
| POST | /api/browser/screenshot | Take screenshot |
| POST | /api/browser/evaluate | Run JS code |

### Proxy Endpoints

| Method | Endpoint | Description |
|--------|----------|-------------|
| GET | /api/proxy/list | List proxies |
| POST | /api/proxy/add | Add proxy |
| POST | /api/proxy/test | Test proxy |
| DELETE | /api/proxy/:id | Remove proxy |

---

## 🔒 Upload Instructions

### Preparing Your Data

#### File Format Requirements

| Format | Extension | Max Size | Encoding |
|--------|-----------|----------|----------|
| CSV | .csv | 50MB | UTF-8 |
| JSON | .json | 20MB | UTF-8 |
| Excel | .xlsx | 20MB | - |
| Text | .txt | 10MB | UTF-8 |

#### Naming Conventions

```
# Files should follow these naming rules:
- Use alphanumeric characters only
- No spaces, use underscores or hyphens
- Include date if relevant: data_20240115.csv
- Be descriptive: product_list_q1.csv

# Examples:
✅ valid_file_name.csv
✅ product-data-2024.csv
✅ article_titles.json
❌ my file.csv (spaces not allowed)
❌ file@name.csv (special chars not allowed)
```

### Upload Steps

1. **Prepare your data file**
   - Convert to supported format (CSV, JSON, Excel)
   - Ensure UTF-8 encoding
   - Follow naming conventions

2. **Navigate to Data Clean page**
   - Click "导入数据" or "Quick Import"
   - Select file format

3. **Configure processing options**
   - Select transformation operations
   - Set regex patterns if needed
   - Choose output format

4. **Export processed data**
   - Select target format
   - Click export button
   - Save to local directory

### Error Handling

| Error | Cause | Solution |
|-------|-------|----------|
| File too large | Exceeds size limit | Split into smaller files |
| Invalid encoding | Non-UTF-8 content | Re-save with UTF-8 encoding |
| Parse error | Malformed file | Check file structure |
| Format mismatch | Wrong file type | Verify file extension |

### Verification Methods

1. **Preview**: Check data preview before processing
2. **Validate**: Use "Test" button for regex patterns
3. **Export Sample**: Export first 100 rows for verification
4. **Checksum**: Compare file hash before/after upload

---

## 🤝 Contributing

Contributions are welcome! Please read our [Contributing Guide](CONTRIBUTING.md).

### Development Setup

```bash
# Install dependencies
npm install

# Start development server
npm run dev

# Type checking
npm run typecheck

# Linting
npm run lint

# Testing
npm run test

# Build for production
npm run build
```

### Code Style

- Use **Vue 3 Composition API**
- Follow **TypeScript** best practices
- Use **Element Plus** components
- Follow existing naming conventions

---

## 📄 License

This project is licensed under the MIT License - see [LICENSE](LICENSE) for details.

---

## 🙏 Acknowledgments

- [Vue.js](https://vuejs.org/) - The Progressive JavaScript Framework
- [Element Plus](https://element-plus.org/) - A Vue.js 3 UI Library
- [Puppeteer](https://pptr.dev/) - Headless Chrome Node.js API
- [xlsx](https://sheetjs.com/) - SheetJS Excel/CSV Parser
- [html2canvas](https://html2canvas.hertzen.com/) - HTML-to-Canvas converter

---

## 📧 Contact & Support

| Channel | Link |
|---------|------|
| GitHub Issues | [Report bugs or request features](https://github.com/badhope/VisualSpider/issues) |
| Documentation | [Wiki](https://github.com/badhope/VisualSpider/wiki) |

---

<p align="center">
  <strong>Made with ❤️ by the VisualSpider Team</strong>
  <br>
  <sub>⭐ Star this repo if you find it useful!</sub>
</p>
