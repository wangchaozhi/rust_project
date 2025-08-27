# 户籍管理系统

一个使用Rust和egui开发的现代化户籍管理系统，具有完整的增删改查功能和用户友好的界面。

## 🚀 功能特性

- **户籍管理**: 完整的户籍信息增删改查
- **家庭成员管理**: 支持多个家庭成员的详细信息管理
- **搜索过滤**: 实时搜索户籍信息
- **数据验证**: 完整的表单验证和错误提示
- **现代化UI**: 使用egui框架的响应式界面
- **自定义字体**: 支持中文字体显示

## 🛠️ 技术栈

- **Rust**: 系统编程语言
- **egui**: 现代化的即时模式GUI框架
- **serde**: 序列化/反序列化
- **chrono**: 日期时间处理
- **uuid**: 唯一标识符生成

## 📦 安装和运行

### 前置要求

- Rust 1.70+ 
- Cargo

### 运行步骤

1. 克隆仓库
```bash
git clone https://github.com/wangchaozhi/rust_project.git
cd rust_project
```

2. 编译并运行
```bash
cargo run
```

## 🏗️ 项目结构

```
src/
├── main.rs          # 程序入口
├── lib.rs           # 库模块导出
├── app.rs           # 应用核心
├── config.rs        # 配置管理
├── data/            # 数据层
│   ├── mod.rs
│   ├── models.rs    # 数据模型
│   ├── manager.rs   # 数据管理
│   └── validation.rs # 数据验证
├── ui/              # 界面层
│   ├── mod.rs
│   ├── components.rs # UI组件
│   ├── panels.rs    # 面板组件
│   ├── dialogs.rs   # 对话框
│   └── styles.rs    # 样式主题
└── utils/           # 工具层
    ├── mod.rs
    ├── date.rs      # 日期工具
    ├── format.rs    # 格式化工具
    └── export.rs    # 导出工具
```

## 🎯 主要功能

### 户籍管理
- 添加新户籍
- 编辑现有户籍
- 删除户籍
- 查看户籍详情

### 家庭成员管理
- 添加家庭成员
- 编辑成员信息
- 删除成员
- 关系管理

### 数据验证
- 身份证号格式验证
- 必填字段检查
- 日期有效性验证
- 用户友好的错误提示

## 🔧 开发

### 编译
```bash
cargo build
```

### 测试
```bash
cargo test
```

### 发布版本
```bash
cargo build --release
```

## 📝 许可证

MIT License

## 🤝 贡献

欢迎提交Issue和Pull Request！

## 📞 联系方式

如有问题，请通过GitHub Issues联系。
