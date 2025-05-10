# CP-Assist: Competitive Programming Workflow Assistant

![CP-Assist Banner](https://raw.githubusercontent.com/tsych0/cp-assist/main/src-tauri/icons desktop application designed to streamline your competitive programming workflow. It integrates with popular tools like competitive-companion and cph-submit to automate test case management, code execution, and solution submission, allowing you to focus on solving problems rather than managing infrastructure.

## ✨ Features

### Problem Management
- **Automatic Problem Parsing**: Integrates with competitive-companion to receive problem details directly from online judges
- **Organized Storage**: Keeps track of problem names, groups, URLs, and constraints
- **Interactive Problem Support**: Handles special interactive problem types

### Test Case Handling
- **Automated Test Collection**: Automatically stores test cases from problems
- **Input/Output Management**: Organizes test data for verification
- **Multiple Test Support**: Handles multiple test cases per problem

### Code Execution & Judging
- **Local Testing Environment**: Run and test your solutions locally before submission
- **Verdict Generation**: Get immediate feedback with verdicts (AC, WA, TLE, MLE, etc.)
- **Performance Monitoring**: Track runtime and memory usage of your solutions

### Language Support
- **Multi-language Compatibility**: Support for various programming languages
- **Custom Configuration**: Customize language settings through Languages.toml
- **Language-specific Compilation**: Handles different compilation and execution requirements

### Submission System
- **Online Judge Integration**: Submit directly to online judges via cph-submit
- **Submission Tracking**: Keep track of your submissions
- **Cross-platform Support**: Works on both Windows and Linux

## 🚀 Getting Started

### Installation

#### Prerequisites
- [Competitive Companion](https://github.com/jmerle/competitive-companion) browser extension

#### Windows
1. Download the latest release from the [Releases page](https://github.com/tsych0/cp-assist/releases)
2. Run the installer and follow the on-screen instructions

#### Linux
1. Download the AppImage or .deb package from the [Releases page](https://github.com/tsych0/cp-assist/releases)
2. For AppImage:
   ```bash
   chmod +x CP-Assist.AppImage
   ./CP-Assist.AppImage
   ```
3. For .deb package:
   ```bash
   sudo dpkg -i cp-assist_0.1.0_amd64.deb
   ```

### Basic Usage

1. **Setting Up**:
   - Launch CP-Assist
   - Configure your preferred programming language and settings

2. **Getting Problems**:
   - Navigate to a problem on a supported online judge
   - Click the Competitive Companion extension button to send the problem to CP-Assist

3. **Solving & Testing**:
   - Write your solution in the editor
   - Run tests against the provided test cases
   - Debug and optimize your solution

4. **Submitting**:
   - Once your solution passes all test cases, use the submission feature to send it to the online judge

## 📝 Configuration

CP-Assist can be configured through the Settings menu:

- **Languages**: Customize compilation and execution commands
- **Editor Settings**: Adjust editor preferences
- **Appearance**: Modify the application's look and feel

## 🔧 Development

### Building from Source

1. Clone the repository:
   ```bash
   git clone https://github.com/tsych0/cp-assist.git
   cd cp-assist
   ```

2. Install dependencies:
   ```bash
   npm install
   ```

3. Run in development mode:
   ```bash
   npm run tauri dev
   ```

4. Build for production:
   ```bash
   npm run tauri build
   ```

## 🤝 Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add some amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## 📜 License

This project is licensed under the MIT License - see the LICENSE file for details.

## 🙏 Acknowledgements

- [Competitive Companion](https://github.com/jmerle/competitive-companion) for problem parsing
- [cph-submit](https://github.com/cph-submit) for submission integration
- [Tauri](https://tauri.app/) for the application framework

---

Made with ❤️ by [tsych0](https://github.com/tsych0)

---
Answer from Perplexity: pplx.ai/share
