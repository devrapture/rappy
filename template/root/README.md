# Welcome to Your Rappy Project! 🚀

Congratulations on scaffolding your project with [Rappy](https://github.com/devrapture/rappy)! Let's get you started with your new Web3 development environment.

## 🎯 First Steps

1. If you didn't install project dependencies, Install project dependencies:
```bash
pnpm install
```

2. Start the development server:
```bash
pnpm dev
```

## 🛠 Tech Stack

### Frontend
- [Next.js](https://nextjs.org) - React framework
- [TypeScript](https://www.typescriptlang.org/) - Type safety
- [Tailwind CSS](https://tailwindcss.com) - Styling

### Smart Contracts
- [Foundry](https://book.getfoundry.sh/) or [Hardhat](https://hardhat.org/) (based on your selection)

## 📦 Managing Dependencies

Add new dependencies using pnpm with workspace filtering:

```bash
# For frontend packages
pnpm add wagmi --filter frontend

# For contract packages
pnpm add copyfiles --filter contract
```

## 🎮 Development Commands

### Frontend Development

```bash
# Start development server
pnpm dev

# Build for production
pnpm build

# Copy environment variables from .env.example to .env
pnpm copy-env
```

### Smart Contract Development

#### If using Foundry:

```bash
# format contracts
pnpm forge-format

# Run tests
pnpm forge-test

# Build contracts
pnpm forge-build

# test coverage
pnpm forge-coverage
```

#### If using Hardhat:

```bash
# Compile contracts
pnpm compile

# Run tests
pnpm test
```

## 📚 Additional Resources

- [Rappy Documentation](https://github.com/devrapture/rappy/docs)
- [Next.js Documentation](https://nextjs.org/docs)
- [Foundry Book](https://book.getfoundry.sh/)
- [Hardhat Documentation](https://hardhat.org/getting-started)

## 💡 Support

If you encounter any issues or have questions:
- Open an issue on our [GitHub repository](https://github.com/devrapture/rappy/issues)

## 📄 License

Rappy is open-source software licensed under the MIT license.