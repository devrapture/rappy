# 🚀 Rappy

[![Rust CI](https://github.com/devrapture/rappy/actions/workflows/build.yml/badge.svg)](https://github.com/devrapture/rappy/actions/workflows/build.yml)
[![Release](https://github.com/devrapture/rappy/actions/workflows/release.yml/badge.svg)](https://github.com/devrapture/rappy/actions/workflows/release.yml)
[![NPM Publish](https://github.com/devrapture/rappy/actions/workflows/npm_publish.yml/badge.svg)](https://github.com/devrapture/rappy/actions/workflows/npm_publish.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)


A lightning-fast CLI tool for scaffolding Web3 projects with Next.js frontend and your choice of Foundry or Hardhat for smart contract development.

## 🎯 Motivation

Web3 development often involves a complex setup process - configuring the frontend, setting up smart contract development environments, and ensuring everything works together seamlessly. 

Rappy aims to solve this by providing:

- **Quick Start**: Get a production-ready Web3 project running in minutes
- **Flexibility**: Choose between Foundry or Hardhat for your smart contract development
- **Developer Experience**: Includes pre-configured testing, formatting, and development scripts

## 🛠 Features

### Frontend
- Next.js 15+ with App Router and built-in TypeScript support
- Tailwind CSS for modern, utility-first styling


### Smart Contracts
- Choice between Foundry or Hardhat
- Pre-configured development environment

## 🔧 Commands

```bash
# Create a new project
pnpm create rappy-app@latest

# Add dependencies
pnpm add <package> --filter <workspace>

# E.g for frontend packages
pnpm add wagmi --filter frontend

# E.g for contract packages
pnpm add copyfiles --filter contract
```

## Demo
<div align="center">
  <a href="https://youtu.be/PxsUDz1tWB4">
    <img src="https://img.youtube.com/vi/PxsUDz1tWB4/0.jpg" alt="Rappy Demo Video" style="width:100%;">
  </a>
  <br>
  <p><strong>Watch the Rappy Demo Video</strong></p>
</div>
## 🤝 Contributing

We welcome contributions! 

1. Fork the repository
2. Create your feature branch
3. Commit your changes
4. Push to the branch
5. Open a Pull Request

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 💫 Acknowledgments

Special thanks to:
- The Next.js team for the amazing framework
- Create t3 app for inspiration
- lerna for managing multi-package repositories
- Foundry and Hardhat teams for their smart contract development tools
- Our contributors and the Web3 community

## 📞 Support

Need help? Here's how to get support:

- Open an [Issue](https://github.com/devrapture/rappy/issues)
- Check our [Website](https://rappy-website.vercel.app)
