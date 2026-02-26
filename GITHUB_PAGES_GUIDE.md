# 使用 GitHub Pages 发布更新

本指南说明如何使用 GitHub Pages 作为应用的更新源。

## 1. 准备工作

您已拥有仓库：[https://github.com/weifuxing/STM32DownloadTool](https://github.com/weifuxing/STM32DownloadTool)

1.  **开启 GitHub Pages**：
    -   进入仓库页面 -> **Settings** -> **Pages**。
    -   在 **Build and deployment** 下：
        -   **Source**: 选择 `Deploy from a branch`。
        -   **Branch**: 选择 `main` 分支，目录选择 `/docs`。
    -   点击 **Save**。
    -   GitHub 会自动部署，稍等片刻后，您的 Pages 网站地址通常是 `https://weifuxing.github.io/STM32DownloadTool/`。

## 2. 修改配置

确保 `src-tauri/tauri.conf.json` 中的 `updater.endpoints` 指向您的 GitHub Pages 地址加上 `/updates/latest.json`。

例如：
```json
"endpoints": [
  "https://weifuxing.github.io/STM32DownloadTool/updates/latest.json"
]
```

## 3. 发布流程

每次发布新版本时，您需要手动构建并将更新文件上传到仓库的 `docs/updates` 目录。

### 步骤 A：构建更新包

在本地运行构建命令（确保已配置好私钥环境变量，或者使用之前生成的无密码密钥）：

```bash
# Windows PowerShell
$env:TAURI_SIGNING_PRIVATE_KEY="您的私钥内容"
$env:TAURI_SIGNING_KEY_PASSWORD=""
npm run tauri build
```

构建完成后，在 `src-tauri/target/release/bundle/nsis/` 目录下会生成：
-   `STM32 Download Tool_x.x.x_x64-setup.exe` (安装包)
-   `STM32 Download Tool_x.x.x_x64-setup.nsis.zip` (更新包)
-   `STM32 Download Tool_x.x.x_x64-setup.nsis.zip.sig` (签名文件)

### 步骤 B：准备 update.json

在仓库的 `docs/updates` 目录下维护一个 `latest.json` 文件。内容格式如下：

```json
{
  "version": "0.1.8",
  "notes": "这里写更新日志，例如：修复了xx bug",
  "pub_date": "2024-03-20T12:00:00Z",
  "platforms": {
    "windows-x86_64": {
      "signature": "这里填 .sig 文件里的内容",
      "url": "https://weifuxing.github.io/STM32DownloadTool/updates/STM32_Download_Tool_0.1.8_x64-setup.nsis.zip"
    }
  }
}
```

*注意：`url` 必须是 zip 更新包的直链，且版本号必须大于当前版本。*

### 步骤 C：上传到 GitHub

1.  将 `latest.json` 和生成的 `.zip` 更新包（建议改名为 `v0.1.8.zip` 并同步修改 json 里的 url）复制到 `docs/updates` 目录。
2.  提交并推送：
    ```bash
    git add docs/updates
    git commit -m "release: update latest version to v0.1.8"
    git push
    ```
3.  GitHub Pages 会自动重新部署（通常需要 1-2 分钟）。

## 4. 验证

在浏览器访问 `https://weifuxing.github.io/STM32DownloadTool/updates/latest.json`，如果能看到刚才提交的 JSON 内容，说明配置成功。客户端启动时就会检测到这个新版本。
