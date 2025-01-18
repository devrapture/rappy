const https = require('https');
const fs = require('fs');
const path = require('path');
const { platform, arch } = process;

const BINARY_NAME = platform === 'win32' ? 'rappy.exe' : 'rappy';

async function downloadRappy() {
  const version = require('../package.json').version;
  const url = getBinaryUrl(version);
  const binaryPath = path.join(__dirname, BINARY_NAME);
  
  await downloadFile(url, binaryPath);
  await makeExecutable(binaryPath);
}

function getBinaryUrl(version) {
  const base = 'https://github.com/devrapture/rappy/releases/download';
  const target = getTarget();
  return `${base}/v${version}/rappy-${target}`;
}

function getTarget() {
  const platform_name = {
    'darwin': 'apple-darwin',
    'linux': 'unknown-linux-gnu',
    'win32': 'pc-windows-msvc'
  }[platform];

  const arch_name = {
    'x64': 'x86_64',
    'arm64': 'aarch64'
  }[arch];

  return `${arch_name}-${platform_name}`;
}

function downloadFile(url, dest) {
  return new Promise((resolve, reject) => {
    const file = fs.createWriteStream(dest);
    https.get(url, (response) => {
      response.pipe(file);
      file.on('finish', () => {
        file.close();
        resolve();
      });
    }).on('error', reject);
  });
}

function makeExecutable(file) {
  if (platform !== 'win32') {
    return new Promise((resolve, reject) => {
      fs.chmod(file, '755', (err) => {
        if (err) reject(err);
        else resolve();
      });
    });
  }
  return Promise.resolve();
}

module.exports = { downloadRappy }; 