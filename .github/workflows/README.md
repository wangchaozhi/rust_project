# GitHub Actions 工作流说明

本项目包含两个主要的GitHub Actions工作流：

## 1. build.yml - 完整构建工作流

这个工作流提供最全面的构建选项：

### 触发条件
- 推送到 `main` 分支
- Pull Request 到 `main` 分支  
- 创建 Release

### 构建产物
- **Linux**: 
  - 原生二进制文件
  - AppImage 格式（便携式应用）
- **Windows**:
  - 原生可执行文件 (.exe)
  - NSIS 安装程序
- **macOS**:
  - x86_64 和 ARM64 二进制文件
  - 通用二进制 DMG 安装包

## 2. release.yml - 发布构建工作流（推荐）

这个工作流专注于创建发布版本：

### 触发条件
- 创建以 `v` 开头的标签 (如 `v1.0.0`)
- 手动触发

### 构建产物
- Linux AppImage
- macOS 通用 DMG
- Windows 可执行文件压缩包
- 各平台原生二进制文件压缩包

## 如何创建发布版本

1. 创建并推送标签：
   ```bash
   git tag v1.0.0
   git push origin v1.0.0
   ```

2. GitHub Actions 会自动：
   - 为所有平台构建应用
   - 创建 GitHub Release
   - 上传所有构建产物

## 构建产物说明

### Linux
- `household_management-linux-x86_64.AppImage`: 便携式应用，无需安装
- `household_management-linux-x86_64.tar.gz`: 原生二进制文件

### macOS  
- `household_management-macos-universal.dmg`: 包含 Intel 和 Apple Silicon 支持的安装包
- 支持 macOS 10.12 及以上版本

### Windows
- `household_management-windows-x86_64.exe.zip`: 可执行文件压缩包
- `household_management-windows-x86_64-installer.exe`: 完整安装程序（仅在 build.yml 中）

## 依赖说明

### Linux 依赖
工作流会自动安装以下依赖：
- libgtk-3-dev
- libxcb-render0-dev
- libxcb-shape0-dev  
- libxcb-xfixes0-dev
- libxkbcommon-dev
- libssl-dev

### 缓存优化
两个工作流都包含 Cargo 缓存优化，可显著减少构建时间。

## 故障排除

### 构建失败
1. 检查是否有新的依赖需要添加到工作流中
2. 确认 Rust 代码在所有目标平台上都能编译
3. 查看 GitHub Actions 日志获取详细错误信息

### 发布失败
1. 确认有正确的 `GITHUB_TOKEN` 权限
2. 检查标签格式是否正确 (必须以 `v` 开头)
3. 确认仓库设置允许创建 Release

## 自定义

### 添加新平台
在 `matrix.platform` 中添加新的平台配置：

```yaml
- os: ubuntu-latest
  target: aarch64-unknown-linux-gnu
  name: household_management-linux-aarch64
```

### 修改应用信息
在 DMG 和安装程序构建步骤中修改：
- 应用名称
- Bundle ID
- 版本信息
- 图标等
