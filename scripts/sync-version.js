import fs from 'fs';
import path from 'path';

// 获取新的版本号（npm version 会在运行 postversion 脚本前更新 package.json）
const packageJson = JSON.parse(fs.readFileSync('package.json', 'utf-8'));
const newVersion = packageJson.version;

const tauriConfigPath = path.join('src-tauri', 'tauri.conf.json');
const tauriConfig = JSON.parse(fs.readFileSync(tauriConfigPath, 'utf-8'));

console.log(`Updating tauri.conf.json version from ${tauriConfig.version} to ${newVersion}`);
tauriConfig.version = newVersion;

fs.writeFileSync(tauriConfigPath, JSON.stringify(tauriConfig, null, 2));
