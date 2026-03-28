# Contributing to VisualSpider

First off, thank you for considering contributing to VisualSpider! It's people like you that make VisualSpider such a great tool.

## Code of Conduct

This project and everyone participating in it is governed by our Code of Conduct. By participating, you are expected to uphold this code.

## How Can I Contribute?

### Reporting Bugs

Before creating bug reports, please check the issue list as you might find out that you don't need to create one. When you are creating a bug report, please include as many details as possible:

- Use a clear and descriptive title
- Describe the exact steps to reproduce the problem
- Provide specific examples to demonstrate the steps
- Describe the behavior you observed and what you expected
- Include screenshots if helpful
- Include your environment details

### Suggesting Enhancements

Enhancement suggestions are tracked as GitHub issues. When creating an enhancement suggestion, include:

- Use a clear and descriptive title
- Provide a step-by-step description of the suggested enhancement
- Provide specific examples to demonstrate the steps
- Describe the current behavior and explain the behavior you expected
- Explain why this enhancement would be useful

### Pull Requests

- Fill in the required template
- Do not include issue numbers in the PR title
- Follow the code style guidelines
- Include thoughtfully-worded, well-structured tests
- Document new code
- End all files with a newline

## Development Setup

### Prerequisites

- Node.js >= 18.0.0
- Rust >= 1.75.0
- Visual Studio Build Tools
- WebView2 Runtime

### Setup Steps

```bash
# Clone the repository
git clone https://github.com/badhope/VisualSpider.git
cd VisualSpider

# Install dependencies
npm install

# Start development server
npm run tauri dev
```

### Code Style

- We use ESLint for JavaScript/TypeScript linting
- We use Prettier for code formatting
- Run `npm run lint` to check for issues
- Run `npm run format` to format code

### Commit Messages

- Use the present tense ("Add feature" not "Added feature")
- Use the imperative mood ("Move cursor to..." not "Moves cursor to...")
- Limit the first line to 72 characters or less
- Reference issues and pull requests liberally after the first line

## License

By contributing, you agree that your contributions will be licensed under the MIT License.
