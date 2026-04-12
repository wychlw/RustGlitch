# nfuzz2 使用指南（nightly Rust + SynMutate 参数化实验）

本文档面向你们当前实验场景：

- 仅使用 `synmutate`
- 运行环境使用 nightly
- 被 fuzz 的编译器也是 nightly
- 重点调参：`MUTATE_P`、`MIN_CHOOSE` 等
- 需要筛选 `ice / success / compile-error / hang`

## 1. 环境准备

## 1.1 Rust

建议：

- Rust nightly（最新）
- Cargo（随 Rust 安装）

检查：

- `rustup toolchain list`
- `rustc --version`
- `cargo --version`

如果你想用**自己编译的 rustc**（比如你本地用 `x.py` 构建出来的 stage1/stage2），推荐做法是把它以“自定义 toolchain 名称”的形式挂到 rustup 里，然后在 pipeline 里选择它（见下文 `rustc:fuzz:<toolchain>`）。

本次实验不涉及 Python 模块，**可不安装 Python 依赖**。

---

## 2. 编译与基本运行

在项目根目录执行：

- `cargo build --release`

最小示例（仅演示）：

- `cargo run --release -- -i datas/issues -o out --use-unstable -l 1 --max-iter 3 --timeout-sec 30 node-mutate:gen rustc:fuzz gate:filter:ice+success+compile-error+hang dump:pretty`

说明：

- `-i`：输入样本目录（可重复指定）
- `-o`：输出目录
- `node-mutate:gen`：生成变异样本
- `rustc:fuzz`：对上一步样本执行编译判定（ICE / compile-error / success / hang）
- `gate:filter:...`：可同时写“ICE 去重过滤器集合”和“输出结果类型集合”（按 token 自动识别并分别生效）
- `dump:pretty`：根据 keep 结果落盘（pretty 会尽量格式化）
- 推荐以 `gen -> fuzz -> filter -> dump` 的流水线运行，不要只给 `node-mutate:fuzz`

提示：`node-mutate` 生成器需要输入语料（`-i`），否则可能因为空集合采样而 panic。

## 2.1 新流水线 DSL（推荐避免 shell 冲突）

现在 jobs 支持更灵活表达：

- 基础：`tasker:task`
- 带参数（推荐）：`tasker:task:arg`
- 带参数（兼容）：`tasker:task(args)`（在 zsh/bash 下通常需要加引号）

示例（空格分隔的 pipeline stages）：

- `load:gen llm:mask llm:infill rustc:fuzz gate:filter:query-stack+panic-func gate:filter:ice+success dump:pretty`

### 2.2 选择 rustc toolchain（支持你自己编译的 stage1/stage2）

`rustc:fuzz` 支持携带一个“rustup toolchain 名称”参数，用来选择具体调用哪个编译器：

- `rustc:fuzz:nightly-x86_64-unknown-linux-gnu`
- `rustc:fuzz(stable-x86_64-unknown-linux-gnu)`（兼容写法，shell 可能需要引号）
- `rustc:fuzz:stage1-x86_64-unknown-linux-gnu`（你自己 `rustup toolchain link` 的名字）

内部会变成类似：`rustc +<toolchain> ...`。

#### 如何把自编译 rustc 加到 rustup toolchain list 里

大体流程（示意，具体路径以你本机 rust 源码目录为准）：

1) 获取 rust 源码并构建（示例以 stage1 为例）：

- `./x.py build --stage 1 compiler/rustc`

2) 用 `rustup toolchain link` 把构建产物注册为一个本地 toolchain：

- `rustup toolchain link stage1-x86_64-unknown-linux-gnu <RUST_SRC>/build/x86_64-unknown-linux-gnu/stage1`

3) 验证：

- `rustup toolchain list`（应能看到 `stage1-x86_64-unknown-linux-gnu`）
- `rustc +stage1-x86_64-unknown-linux-gnu --version`

然后在 nfuzz 的 pipeline 里就可以直接用：`rustc:fuzz:stage1-x86_64-unknown-linux-gnu`。

说明：

- `gate:filter:query-stack+panic-func`：选择 ICE 去重过滤器集合（去重键）。
- `gate:filter:ice+success+compile-error+hang`：选择输出结果类型集合（keep）。
- `dump:raw`：原样输出。
- `dump:pretty`：尽量格式化后输出。

`dump` 可以重复写，例如：`dump:raw dump:pretty`。第一份按默认前缀输出，后续 dump 会自动追加模式后缀（如 `_pretty`）。

为什么推荐 `tasker:task:arg` + `+`？

- `()`、`|`、`&` 在 shell 里是特殊字符：`gate:filter(ice|success)` 需要引号，且容易被误当成管道/后台。
- `gate:filter:ice+success` 不含这些字符，zsh/bash 下通常无需引号。

---

## 3. SynMutate 参数（可调）

现在这些参数可通过 CLI 或 JSON 配置设置：

- `--mutate-p`：变异概率（原 `MUTATE_P`）
- `--min-choose`：最小权重阈值（原 `MIN_CHOOSE`）
- `--new-ice-adj-rate`：新 ICE 样本的权重调整倍率
- `--dup-ice-adj-rate`：重复 ICE 样本的权重调整倍率
- `--choose-adj-rate`：节点被选中后的衰减倍率
- `--max-nested`：单次遍历最大嵌套变异次数
- `--max-analyze-depth`：AST 分析深度上限（防止栈爆）

示例：

- `cargo run --release -- -i datas/issues -o out --use-unstable -l 20 --max-iter 2000 node-mutate:gen rustc:fuzz gate:filter:ice+success+compile-error dump:pretty`

---

## 4. 结果筛选与停止条件

本版本不再提供全局 `--keep/--until-*` 一类参数；筛选与停止通过 pipeline + 通用控制参数完成。

### 4.1 输出结果类型（keep）

用 `gate:filter:<kinds>` 选择要落盘的结果类型：

- `gate:filter:ice`
- `gate:filter:compile-error`
- `gate:filter:ice+success+compile-error+hang`

输出文件前缀：

输出文件前缀：

- `ice_*.rs`
- `success_*.rs`
- `compile_error_*.rs`
- `hang_*.rs`

说明：

- 在 `gen -> fuzz -> filter -> dump` 流水线中，`fuzz` 只产出判定结果；`filter` 决定哪些结果允许输出；`dump` 负责落盘。

### 4.2 运行停止

- `-l/--loopcnt <COUNT>`：**目标输出数量**（dumped outputs）到达后停止。
- `--max-iter <COUNT>`：最大迭代次数兜底（可选）。
- `--timeout-sec <SECONDS>`：超时兜底（可选）。

注意：`-l` 计数的是“最终落盘的数量”。如果你把 keep 设得很窄（例如只保留 `ice`），达到 `-l` 会更慢。

示例：

- `cargo run --release -- -i datas/issues -o out_verify --datas datas -l 100 --max-iter 20000 --timeout-sec 3600 --use-unstable node-mutate:gen rustc:fuzz gate:filter:ice+success+compile-error dump:pretty`

---

## 5. 配置文件方式（JSON）

通过 `--config <path>` 加载 JSON 配置。

示例文件：`nfuzz_config.json`

```json
{
  "input": ["datas/issues"],
  "output": "out",
  "datas": "datas",
  "nthread": 4,
  "loopcnt": 2000,
  "skip_hang": true,
  "use_unstable": true,
  "rustc_args": ["-Cdebuginfo=0"],
  "max_iter": 20000,
  "timeout_sec": 3600,
  "jobs": [
    "node-mutate:gen",
    "rustc:fuzz",
    "gate:filter:query-stack+panic-func",
    "gate:filter:ice+success+compile-error",
    "dump:pretty"
  ],
  "synmutate": {
    "mutate_p": 0.15,
    "max_nested": 40,
    "max_analyze_depth": 220,
    "new_ice_adj_rate": 1.08,
    "dup_ice_adj_rate": 0.93,
    "choose_adj_rate": 0.97,
    "min_choose": 0.45
  }
}
```

运行：

- `cargo run --release -- --config nfuzz_config.json`

说明：

- 当前实现中，`--config` 读取后会应用配置值。
- 建议同一次运行主要通过一种方式管理参数（CLI 或 config），避免混淆。

---

## 6. 关于 stable 与 nightly

你们当前实验应固定为 nightly 流程：

- 构建使用 nightly
- 运行时加 `--use-unstable`
- 被 fuzz 的也是 nightly `rustc`

如果你还要启用旧 `syn` 策略（非 `synmutate`），需额外开启：

- `--features nightly-syn`

并可附加：

- `--rustc-arg <ARG>`（可重复）

示例：

- `cargo run --release --features nightly-syn -- -i datas/issues -o out --use-unstable --rustc-arg=-Zvalidate-mir syn:fuzz`

## 6.1 直接覆盖输出目录（不删除目录）

你可以直接使用同一个 `-o out_verify` 重跑。

- 新生成的同名文件（如 `compile_error_0.rs`）会直接覆盖。
- 若本轮循环数变小，旧的高编号文件可能保留（例如上次有 `compile_error_11.rs`，这次只跑到 `compile_error_5.rs`）。

建议：

- 固定 `-l/loopcnt` 与 keep 选择，便于结果可比。
- 或在实验记录里注明本轮有效索引范围。

不需要每次手动 `mkdir`：

- 程序启动时会自动确保输出目录存在。

## 6.2 常用参数速查（含 `-h` / `--log` / `-l`）

- `-h, --help`：查看完整参数帮助。
- `--log <LEVEL>`：日志级别（`error`/`warn`/`info`/`debug`）。
- `-l, --loopcnt <N>`：目标输出（落盘）数量，达到后停止（默认 1）。
- `-j <N>`：并发线程数。
- `-i, --input <DIR>`：输入种子目录，可重复指定。
- `-o, --output <DIR>`：输出目录。
- `-d, --datas <DIR>`：过滤器数据目录（`panic_filters.json`、`func_filters.json`）。
- `--use-unstable`：启用 nightly 编译参数路径。
- `--rustc-arg <ARG>`：追加 rustc 参数，可重复。

常用 pipeline stage：

- `gate:filter:query-stack+panic-func`：启用 ICE 去重过滤器集合。
- `gate:filter:ice+success+compile-error+hang`：选择输出结果类型集合。
- `dump:raw` / `dump:pretty`：落盘格式。

---

## 7. 参数扫描建议（覆盖率实验）

你们要比较不同参数对 rustc 覆盖效果时，建议：

1. 固定输入语料、线程数、循环次数
2. 每组参数重复运行多次
3. 分开输出目录
4. 统计各目录中 `ice_*.rs`、`success_*.rs` 数量和唯一性

推荐先扫描：

- `mutate_p`：0.05, 0.1, 0.15, 0.2
- `min_choose`：0.3, 0.4, 0.5
- `choose_adj_rate`：0.95, 0.97, 0.99

---

## 8. 常见问题

1) 为什么几乎没有 ICE？

- 即使在 nightly，ICE 也通常远少于 compile-error；可尝试：
  - 增加 `loopcnt`
  - 增大输入语料
  - 调高 `mutate_p`
  - 适当降低 `min_choose`，增加探索强度

2) 我只想收集“能编译通过”的程序

- 使用：`gate:filter:success`

3) 我只想收集“导致 ICE”的程序

- 使用：`gate:filter:ice`

---

## 9. 变更摘要

本次改造点：

- `synmutate` 参数由硬编码常量改为可配置（CLI / JSON）。
- 新增结果筛选：按 `ice/success/compile-error/hang` 输出。
- 可通过 `--use-unstable` 启用 nightly 相关编译参数流程。
- 新增配置文件加载能力：`--config`。

## 10. 实测记录（已编译+已运行）

本地实测环境：

- `rustc 1.94.0-nightly (31cd367b9 2026-01-08)`
- `cargo 1.94.0-nightly (b54051b15 2025-12-30)`

实测命令（直接覆盖同一输出目录，不使用删除目录命令）：

- `cargo run --release -- -i datas/issues -o out_verify --datas datas -l 12 -j 1 --use-unstable node-mutate:gen rustc:fuzz gate:filter:ice+success+compile-error dump:pretty`

实测结果（一次运行）：

- 加载种子：`Mutator loaded 186 files`
- 输出统计：
  - `compile_error`: 12
  - `ice`: 0
  - `success`: 0

说明：当前 seed 集与参数下，样本主要落在 `compile-error`，这是正常现象，可继续通过参数扫描提升多样性。

如果你希望我再补一个“批量参数网格实验脚本”（自动多组参数运行并汇总结果），我可以继续直接给你生成。
