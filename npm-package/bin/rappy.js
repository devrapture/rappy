#!/usr/bin/env node

const { spawn } = require('child_process');
const path = require('path');
const os = require('os');
const fs = require('fs');

// Determine platform and architecture
const platform = os.platform();
const arch = os.arch();

// Map to binary name
let binaryName = 'rappy-linux';
if (platform === 'darwin') {
  binaryName = arch === 'arm64' ? 'rappy-macos-arm64' : 'rappy-macos';
} else if (platform === 'win32') {
  binaryName = 'rappy-win.exe';
}

const binaryPath = path.join(__dirname, binaryName);

// Check if binary exists
if (!fs.existsSync(binaryPath)) {
  console.error(`Binary not found: ${binaryPath}`);
  console.error('Platform:', platform, 'Architecture:', arch);
  process.exit(1);
}

// Execute the binary with 'create' command by default
const args = ['create', ...process.argv.slice(2)];

// Execute the binary
const child = spawn(binaryPath, args, { stdio: 'inherit' });

child.on('error', (err) => {
  console.error('Failed to start binary:', err);
  process.exit(1);
});

child.on('exit', (code) => {
  process.exit(code);
}); 