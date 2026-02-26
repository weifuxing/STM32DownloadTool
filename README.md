# STM32 Download Tool

基于 Tauri + Vue 3 + Rust 开发的 STM32 串口下载工具 (ISP)。

## 功能特性

- **串口通信**：自动扫描串口，支持波特率配置。
- **固件支持**：支持加载 .hex 和 .bin 格式固件。
- **Flash 操作**：
  - 芯片信息读取
  - 全片/扇区擦除
  - 固件写入与校验
  - 读保护解除与使能
- **跨平台**：支持 Windows (x64)、macOS (Intel/Apple Silicon) 和 Ubuntu。
- **自动更新**：支持基于 GitHub Releases 的自动更新。

## 快速开始

### 环境要求

- [Node.js](https://nodejs.org/) (v18+)
- [Rust](https://www.rust-lang.org/) (Stable)
- [Tauri CLI](https://tauri.app/)

### 安装与运行

1. 安装依赖：
   ```bash
   npm install
   ```

2. 开发模式运行：
   ```bash
   npm run tauri dev
   ```

3. 构建生产版本：
   ```bash
   npm run tauri build
   ```
   构建产物位于 `src-tauri/target/release/bundle/nsis/`。

## 发布与更新

本项目使用 GitHub Actions 自动化构建和发布流程。详细的发布指南请参考 [RELEASE_PROCESS.md](./RELEASE_PROCESS.md)。

简要流程：
1. 更新 `package.json` 和 `src-tauri/tauri.conf.json` 中的版本号。
2. 提交代码并打上 `v*` 格式的 tag（如 `v0.1.0`）。
3. 推送 tag 到 GitHub，自动触发构建和发布。
