#!/usr/bin/env node

const { execSync } = require('child_process');
const path = require('path');
const os = require('os');

function getBinaryPath() {
  const platform = os.platform();
  const ext = platform === 'win32' ? '.exe' : '';
  return path.join(__dirname, `zitro${ext}`);
}

const binaryPath = getBinaryPath();
const args = process.argv.slice(2);

try {
  execSync(`"${binaryPath}" ${args.join(' ')}`, { stdio: 'inherit' });
} catch (error) {
  process.exit(error.status || 1);
}