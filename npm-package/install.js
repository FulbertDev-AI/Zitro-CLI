const https = require('https');
const fs = require('fs');
const path = require('path');
const { execSync } = require('child_process');
const os = require('os');

const VERSION = '1.0.1';
const GITHUB_REPO = 'FulbertDev-AI/Zitro-CLI';

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

// Fonction pour afficher la bannière ASCII
function displayBanner() {
    const banner = `
███████╗██╗████████╗██████╗  ██████╗      ██████╗██╗     ██╗
╚══███╔╝██║╚══██╔══╝██╔══██╗██╔═══██╗    ██╔════╝██║     ██║
  ███╔╝ ██║   ██║   ██████╔╝██║   ██║    ██║     ██║     ██║
 ███╔╝  ██║   ██║   ██╔══██╗██║   ██║    ██║     ██║     ██║
███████╗██║   ██║   ██║  ██║╚██████╔╝    ╚██████╔███████╗██║
╚══════╝╚═╝   ╚═╝   ╚═╝  ╚═╝ ╚═════╝      ╚═════╝╚══════╝╚═╝`;

    const subtitle = "AUDITEUR     D'EMPREINTE      CARBONE      NUMERIQUE";

    // \x1b[32m = Vert, \x1b[1m = Gras, \x1b[0m = Reset
    console.log('\x1b[32m\x1b[1m' + banner + '\x1b[0m');
    console.log('\x1b[32m\x1b[1m' + subtitle + '\x1b[0m');
    console.log('');
}

// Fonction pour créer le lanceur Windows (.cmd)
function createWindowsCmdLauncher() {
  const npmDir = path.join(os.homedir(), 'AppData', 'Roaming', 'npm');
  const cmdContent = '@echo off\r\nSETLOCAL\r\nSET DIR=%~dp0\r\nnode "%DIR%node_modules\\zitro-cli\\bin\\zitro.js" %*\r\nENDLOCAL\r\n';
  
  const cmdPath = path.join(npmDir, 'zitro.cmd');
  fs.writeFileSync(cmdPath, cmdContent);
}

async function install() {
  const platform = getPlatformInfo();
  const downloadUrl = `https://github.com/${GITHUB_REPO}/releases/download/v${VERSION}/${platform.archive}`;
  const tempDir = os.tmpdir();
  const archivePath = path.join(tempDir, platform.archive);
  const extractDir = path.join(__dirname, 'bin');
  
  console.log('Downloading ZITRO CLI for ' + platform.name + '...');
  
  try {
    await downloadFile(downloadUrl, archivePath);
    
    if (!fs.existsSync(extractDir)) {
      fs.mkdirSync(extractDir, { recursive: true });
    }
    
    if (platform.name === 'windows') {
      execSync('powershell -command "Expand-Archive -Path ' + archivePath + ' -DestinationPath ' + extractDir + ' -Force"');
    } else {
      execSync('unzip ' + archivePath + ' -d ' + extractDir);
      execSync('chmod +x ' + path.join(extractDir, 'zitro'));
    }
    
    fs.unlinkSync(archivePath);
    
    // Créer le lanceur Windows APRÈS l'installation
    if (platform.name === 'windows') {
      createWindowsCmdLauncher();
    }
    
    // Afficher la bannière et les messages de succès
    console.log('');
    displayBanner();
    console.log('ZITRO CLI v' + VERSION + ' installe avec succes.');
    console.log('');
    console.log('Commandes pour debuter :');
    console.log('   zitro --help             Afficher le menu d\'aide complet');
    console.log('   zitro --version          Verifier la version de l\'outil');
    console.log('   zitro scan <URL>         Lancer un audit (ex: zitro scan http://localhost:3000)');
    console.log('   zitro scan <URL> -c TG   Lancer un audit avec le mix energetique du Togo');
    console.log('');
    console.log('Rendez votre code plus vert, un scan a la fois.');
    
  } catch (error) {
    console.error('Installation failed:', error.message);
    process.exit(1);
  }
}

install();