#!/usr/bin/env node

const https = require('https');
const fs = require('fs');
const path = require('path');
const { execSync } = require('child_process');
const os = require('os');

const VERSION = 'v1.0.0';
const GITHUB_REPO = 'FulbertDev-AI/zitro-cli';

function getPlatformInfo() {
  const platform = os.platform();
  const arch = os.arch();
  
  if (platform === 'win32') {
    return { name: 'windows', ext: '.exe', archive: 'zitro-cli-windows.zip' };
  } else if (platform === 'darwin') {
    if (arch === 'arm64') {
      return { name: 'macos-arm', ext: '', archive: 'zitro-cli-macos-arm.zip' };
    }
    return { name: 'macos-intel', ext: '', archive: 'zitro-cli-macos-intel.zip' };
  } else {
    return { name: 'linux', ext: '', archive: 'zitro-cli-linux.zip' };
  }
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

async function install() {
  const platform = getPlatformInfo();
  const downloadUrl = `https://github.com/${GITHUB_REPO}/releases/download/${VERSION}/${platform.archive}`;
  const tempDir = os.tmpdir();
  const archivePath = path.join(tempDir, platform.archive);
  const extractDir = path.join(__dirname, 'bin');
  
  console.log(`Downloading ZITRO CLI for ${platform.name}...`);
  
  try {
    await downloadFile(downloadUrl, archivePath);
    
    if (!fs.existsSync(extractDir)) {
      fs.mkdirSync(extractDir, { recursive: true });
    }
    
    if (platform.name === 'windows') {
      execSync(`powershell -command "Expand-Archive -Path ${archivePath} -DestinationPath ${extractDir} -Force"`);
    } else {
      execSync(`unzip ${archivePath} -d ${extractDir}`);
      execSync(`chmod +x ${path.join(extractDir, 'zitro')}`);
    }
    
    fs.unlinkSync(archivePath);
    console.log('ZITRO CLI installed successfully!');
    console.log('Run "zitro --help" to get started.');
  } catch (error) {
    console.error('Installation failed:', error.message);
    process.exit(1);
  }
}

// Si ce fichier est exécuté directement (postinstall)
if (require.main === module) {
  const binPath = path.join(__dirname, 'bin', `zitro${platform.ext}`);
  if (fs.existsSync(binPath)) {
    // Le binaire existe déjà, on l'exécute
    const args = process.argv.slice(2);
    execSync(`"${binPath}" ${args.join(' ')}`, { stdio: 'inherit' });
  } else {
    // Premier lancement, on installe
    install();
  }
}

module.exports = { install };