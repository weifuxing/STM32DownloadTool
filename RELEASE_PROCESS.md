# 发布流程指南

本文档说明如何发布新版本并启用自动更新功能。

## 前置准备

### 1. 生成签名密钥（如已有可跳过）

自动更新需要使用密钥对构建产物进行签名。如果尚未配置或需要重新生成：

```bash
# 生成新的密钥对（Windows）
npm run tauri signer generate
```

该命令会：
1. 在终端输出私钥（Private Key）和公钥（Public Key）。
2. 自动更新 `src-tauri/tauri.conf.json` 中的 `pubkey` 字段。
3. **重要**：你需要将私钥保存到安全的地方，**不要提交到代码仓库**。

### 2. 配置 GitHub Secrets

在 GitHub 仓库的 `Settings` -> `Secrets and variables` -> `Actions` 中添加以下 Secrets：

- `TAURI_SIGNING_PRIVATE_KEY`: 刚才生成的私钥内容。
- `TAURI_SIGNING_KEY_PASSWORD`: 私钥密码（如果生成时设置了密码，否则留空）。

## 发布步骤

### 1. 更新版本号

修改以下两个文件中的版本号（例如从 `0.1.0` 升级到 `0.1.1`）：
- `package.json`
- `src-tauri/tauri.conf.json`

### 2. 提交代码

```bash
git add .
git commit -m "chore: bump version to 0.1.1"
git push
```

### 3. 打标签并推送

创建与版本号对应的 git tag（必须以 `v` 开头）：

```bash
git tag v0.1.1
git push origin v0.1.1
```

### 4. 验证发布

1. 访问 GitHub 仓库的 Actions 页面，查看 `Release` 工作流的运行状态。
2. 构建成功后，会自动在 Releases 页面创建一个新的 Release（默认为 Draft 状态，可在 Workflow 中修改配置）。
3. Release 中应包含：
   - Windows 安装包（`.exe`）
   - `latest.json`（用于自动更新检测）
   - 签名文件

### 5. 客户端更新

客户端应用启动时会自动检查 `src-tauri/tauri.conf.json` 中配置的 `endpoints` URL。
如果发现新版本的 `latest.json` 且版本号大于当前版本，将提示用户更新。
