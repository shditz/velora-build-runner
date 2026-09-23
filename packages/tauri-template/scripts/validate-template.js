import fs from 'node:fs';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);
const templateRoot = path.resolve(__dirname, '..');

console.log('Validating @velora/tauri-template integrity...\n');

let errors = [];

function checkFile(relPath, desc) {
  const fullPath = path.join(templateRoot, relPath);
  if (!fs.existsSync(fullPath)) {
    errors.push(`Missing ${desc}: ${relPath}`);
    return null;
  }
  return fs.readFileSync(fullPath, 'utf8');
}

checkFile('src-tauri/Cargo.toml', 'Cargo manifest');
checkFile('src-tauri/build.rs', 'Tauri build script');
checkFile('src-tauri/src/main.rs', 'Application entry point');
checkFile('src-tauri/src/lib.rs', 'Tauri runtime library');

const capsRaw = checkFile('src-tauri/capabilities/default.json', 'Tauri capabilities file');
if (capsRaw) {
  try {
    const caps = JSON.parse(capsRaw);
    if (!caps.identifier || !Array.isArray(caps.permissions)) {
      errors.push('Invalid capabilities format in src-tauri/capabilities/default.json');
    }
  } catch (err) {
    errors.push(`JSON parse error in capabilities: ${err.message}`);
  }
}

const veloraConfigRaw = checkFile('src-tauri/velora-config.json', 'Velora config placeholder');
if (veloraConfigRaw) {
  try {
    const config = JSON.parse(veloraConfigRaw);
    if (!config.websiteUrl || !config.appName || !config.window) {
      errors.push('Missing required fields in src-tauri/velora-config.json');
    }
  } catch (err) {
    errors.push(`JSON parse error in velora-config.json: ${err.message}`);
  }
}

const tauriConfRaw = checkFile('src-tauri/tauri.conf.json', 'Default tauri.conf.json');
let iconList = [];
if (tauriConfRaw) {
  try {
    const tauriConf = JSON.parse(tauriConfRaw);
    if (!tauriConf.productName || !tauriConf.bundle || !tauriConf.bundle.icon) {
      errors.push('Invalid tauri.conf.json schema');
    } else {
      iconList = tauriConf.bundle.icon;
    }
  } catch (err) {
    errors.push(`JSON parse error in tauri.conf.json: ${err.message}`);
  }
}

const templateRaw = checkFile('src-tauri/tauri.conf.json.template', 'Tauri config template');
if (templateRaw) {
  const expectedPlaceholders = [
    '{{APP_NAME}}',
    '{{VERSION}}',
    '{{BUNDLE_ID}}',
  ];
  for (const ph of expectedPlaceholders) {
    if (!templateRaw.includes(ph)) {
      errors.push(`Template missing placeholder: ${ph}`);
    }
  }
}

for (const iconRel of iconList) {
  const iconPath = path.join(templateRoot, 'src-tauri', iconRel);
  if (!fs.existsSync(iconPath)) {
    errors.push(`Required icon asset missing: src-tauri/${iconRel}`);
  }
}

checkFile('src/index.html', 'Webview loader HTML');
const offlineHtml = checkFile('src/offline.html', 'Offline fallback page');
if (offlineHtml && (!offlineHtml.includes('No Internet Connection') || !offlineHtml.includes('Try Again'))) {
  errors.push('offline.html is missing expected offline UI elements');
}

if (errors.length > 0) {
  console.error('Validation FAILED with errors:');
  for (const err of errors) {
    console.error(`  - ${err}`);
  }
  process.exit(1);
} else {
  console.log('All @velora/tauri-template files verified successfully! ✨');
}
