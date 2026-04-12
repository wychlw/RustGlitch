# RustGlitch: Yet another Rust Compiler Fuzzer

> 项目级 README（骨架版）

## 1) 项目简介

- 项目定位：TODO
- 主要目标：TODO
- 适用场景：TODO

## 2) 核心能力（Features）

- Pipeline DSL：TODO
- 多策略生成/变异：TODO
- 编译执行与结果分类：TODO
- 去重过滤与落盘：TODO
- CLI + JSON 配置：TODO

## 3) 快速开始（Quick Start）

### 3.1 环境要求

- Rust 工具链：TODO
- 操作系统：TODO
- 可选依赖（模型/脚本）：TODO

### 3.2 最小运行示例

- 示例 1：TODO
- 示例 2：TODO

## 4) 安装与依赖（Installation）

### 4.1 克隆与构建

- 步骤：TODO

### 4.2 可选组件

- Python 相关：TODO
- 模型相关：TODO
- C/C++ 相关：TODO

## 5) 使用方式（Usage）

### 5.1 CLI 总览

- 参数分组说明：TODO

### 5.2 Pipeline DSL 语法

- 基本语法：TODO
- 内置 stage（`filter` / `dump`）：TODO
- 兼容写法：TODO

### 5.3 常见 Pipeline 模板

- 基础模板：TODO
- 含过滤模板：TODO
- 含多 dump 模板：TODO

## 6) 结果与输出（Outputs）

### 6.1 输出目录约定

- 输出文件命名：TODO

### 6.2 结果类型说明

- `ice`：TODO
- `success`：TODO
- `compile_error`：TODO
- `hang`：TODO

### 6.3 文件附加信息

- `Compile Args`：TODO
- `Original Flags`：TODO

## 7) 配置文件（Configuration）

### 7.1 配置入口

- `--config`：TODO

### 7.2 常用字段

- 运行控制：TODO
- Pipeline 相关：TODO
- SynMutate 参数：TODO

### 7.3 CLI 与配置覆盖关系

- 覆盖规则：TODO

## 8) 项目结构（Project Structure）

- `src/main.rs`：TODO
- `src/conf.rs`：TODO
- `src/pipeline/`：TODO
- `src/fuzz/`：TODO
- `src/ice_process/`：TODO
- `src/strategies/`：TODO
- `docs/`：TODO

## 9) 扩展开发（Extensibility）

### 9.1 新增策略（Fuzzer）

- 接口与步骤：TODO

### 9.2 新增过滤器

- 接口与步骤：TODO

### 9.3 新增内置 stage

- DSL 与 runtime 接入点：TODO

---

详细使用说明：

- [docs/USAGE.zh-CN.md](docs/USAGE.zh-CN.md)

架构说明：

- [docs/ARCHITECTURE.zh-CN.md](docs/ARCHITECTURE.zh-CN.md)