# RustGlitch: Yet another Rust Compiler Fuzzer

## 3) 快速开始（Quick Start）

### 3.1 环境要求

- Rust 工具链：rust 1.90+，推荐使用最新的 stable 或 nightly。
- 理论上cargo会帮你安装好所有依赖。

### 3.2 最小运行示例

- 示例：`cargo run --release -- -i samples -o out --datas datas -l 1 -j 1 node-mutate:gen 'rustc:fuzz(nightly-x86_64-unknown-linux-gnu)' 'gate:filter(query-stack&panic-func)' 'gate:filter(ice|success)' dump:pretty`

以上这个示例为：使用一个线程，生成一个输出文件，生成该文件的过程如下：
- 使用 `node-mutate` 生成器基于 `samples` 目录下的样本进行变异生成。
- 使用 `rustc:fuzz` 对生成的样本进行编译，得到编译结果，采用的编译器为 `rustup toolchain list` 中的 `nightly-x86_64-unknown-linux-gnu`。（注：如果使用自己编译的编译器，可以将其加入rustup自定义工具链中来调用）
- 使用 `gate:filter` 进行过滤，如果是ice，则使用 `query-stack` 和 `panic-func` 过滤器进行去重
- 使用 `gate:filter` 进行过滤，只保留 ice 和 success 类型的结果
- 使用 `dump:pretty` 输出变异的程序，输出采用尽量格式化的方式，输出目录为 `out`。

对于一次经典的测试用例生成+fuzz+调参流程，有如下示例：
`cargo run --release -- -i samples -o out --datas datas -l 1000 -j 16 --use-unstable --timeout-sec 3600 node-mutate:gen rustc:fuzz(stage2-toolchain) gate:filter(query-stack&panic-func) gate:filter(ice|success) dump:pretty rustc:fuzz(starge2-fuzz-target-with-coverage) --mutate-p 0.5 --choose-adj-rate 0.9 --dup-ice-adj-rate 0.8 --min-choose 0.05`

## 5) 使用方式（Usage）

### 5.1 CLI 参数

下列参数，可通过 `cargo run -- --help` 查看参数具体默认值与可选值。

| 参数 | 含义 | 备注 |
| --- | --- | --- |
| `-i, --input <DIR>` | 输入语料目录 | 可重复传入多个目录 |
| `-o, --output <DIR>` | 输出目录 | 默认 `out` |
| `-d, --datas <DIR>` | 过滤器持久化数据目录 | 默认 `.`，会存储panic-func和query-stack，用于过滤重复ice |
| `-j <THREAD>` | 工作线程数 |  |
| `--skip-hang` | 是否跳过编译超时样本 | 默认 `true` |
| `--use-unstable` | 启用不稳定 rustc flags 路径 | 默认 `false`，在fuzz unstable编译器时建议为 `true` |
| `--rustc-arg <ARG>` | 追加原始 rustc 参数 |  |
| `--log <LEVEL>` | 日志级别 | `error/warn/info/debug` |
| `-l, --loopcnt <COUNT>` | 目标输出数量（达到即停） | 默认 `1` |
| `--max-iter <COUNT>` | 最大迭代次数保护阈值 | 可选，防止一直无法达成 `-l` 中的目标 |
| `--timeout-sec <SECONDS>` | 运行超时秒数 | 可选，防止一直无法达成 `-l` 中的目标 |
| `[PIPELINE]...` | 见5.3 | 程序具体行为 |

### 5.2 node-mutate 参数

`node-mutate` 参数用于控制变异过程中的行为和权重调整，主要包括：

| 参数 | 作用 |
| --- | --- |
| `--mutate-p` | 控制节点发生变异的概率 |
| `--max-nested` | 限制单轮深层递归变异次数，避免爆栈|
| `--max-analyze-depth` | 限制 AST 分析深度，避免爆栈 |
| `--new-ice-adj-rate` | 发现新 ICE 时，相关节点权重提升倍率 |
| `--dup-ice-adj-rate` | 命中重复 ICE 时，相关节点权重调整倍率 |
| `--choose-adj-rate` | 节点被选择后权重衰减倍率 |
| `--min-choose` | 节点最小保留权重阈值 |

### 5.3 Pipeline

对于一次测试用例的生成，我们可以将其抽象为对一个源文件的一系列流水线操作。如，我们可能想：
- 通过某种方式生成一个样本（无论是变异、构造还是大模型等等）
- 对样本进行某种处理，如节选或继续变异等等
- 此时我们可能就想输出一次这个最初样本
- 将这个样本放到目标编译器中，得到编译结果
- 根据编译结果进行过滤
- 可能我们已经想输出这个样本了
- 根据结果采用不同的过滤选项过滤
- 再次输出这个样本

以上这一串流水线步骤，用接下来介绍的语法，就可以表示为：
- `node-mutate:gen` 生成一个变异样本
- `model:infill` 再让模型来个填空式变异
- `dump:raw` 输出一次这个样本
- `rustc:fuzz(custom-compiler)` 送到编译器里编译，得到结果；特别的，使用 `custom-compiler` 这个 toolchain
- `gate:filter(ice|success)` 根据结果过滤，只保留 ice 和 success 类型的结果
- `dump:pretty` 美观的输出一次这个样本
- `gate:filter(query-stack&panic-func)` 根据结果过滤，只保留未被 `query-stack` 和 `panic-func` 过滤掉的样本
- `dump:pretty` 再次美观的输出一次这个样本

当然，它也可以帮你批量的在目标编译器上运行生成好的一系列样本，来进行覆盖率等实验，只需要：
- `load:gen` 从输入目录加载样本（不变异）
- `rustc:fuzz(custom-fuzzable-target)` 送到编译器里编译，得到结果
（没错我们甚至不需要输出结果，在运行编译器过后自然输出gcda）

也就是说完全可以组合运行，如 `node-mutate:gen rustc:fuzz(custom-compiler) gate:filter(query-stack&panic-func) gate:filter(ice|success) rustc:fuzz(custom-fuzzable-target)` 就可以自动化的完成：生成样本 -> 使用一个经过优化，不会生成覆盖率的编译器编译过滤得到新样本 -> 在目标编译器上编译得到覆盖率数据 的流程。 

接下来介绍pipeline的语法：

#### 基本语法：
- `tasker:task`
- `tasker:task:arg`
- `tasker:task(arg)`（在 zsh/bash 中通常需要引号）

#### 基本部件

样本生成：
- `node-mutate:gen`：基于输入样本进行 AST 变异
- `load:gen`：从输入目录加载样本（不变异）

程序运行：
- `rustc:fuzz(toolchain)`：使用指定 rustc toolchain 编译样本，得到编译结果。若不指定 toolchain，则使用当前环境默认 rustc。

结果过滤：
- `gate:filter(xxx)`：根据编译结果进行过滤，参数 xxx 决定过滤选项：
    - 去重过滤器：`query-stack`、`panic-func`
    - 结果类型：`ice`、`success`、`compile-error`、`hang`
    - 支持使用 `&` `|` 组合过滤选项，例如：`gate:filter(query-stack&panic-func)` 表示同时使用 `query-stack` 和 `panic-func` 进行过滤；`gate:filter(ice|success)` 表示保留 ice 和 success 类型的结果。

结果输出：
- `dump:raw`：原样输出当前样本到文件
- `dump:pretty`：尽量格式化后输出当前样本到文件

## 6) 结果与输出（Outputs）

### 6.1 输出目录约定

- 输出文件命名：
	- 默认输出到 `-o/--output` 指定目录
	- 常见命名：`ice_<idx>.rs`、`success_<idx>.rs`、`compile_error_<idx>.rs`、`hang_<idx>.rs`
	- 当使用多个 dump stage 时，后续输出会追加模式后缀避免覆盖（例如 `compile_error_pretty_0.rs`）

### 6.2 结果类型说明

- `ice`：编译器内部错误（Internal Compiler Error）
- `success`：编译成功
- `compile_error`：常规编译错误（非 ICE）
- `hang`：编译超时（若开启 `--skip-hang`，默认会跳过）

### 6.3 文件附加信息

以下附加信息仅在调用 `rustc:fuzz` 后才会具有该类属性：
- `Compile Args`：该样本对应的 rustc 实际编译参数，用于复现
- `Original Flags`：仅在 `--use-unstable` 选项下存在，若程序导致ice，则会二分查找出导致ice的最小flags集合。

---

详细使用说明：

- [docs/USAGE.zh-CN.md](docs/USAGE.zh-CN.md)

架构说明：

- [docs/ARCHITECTURE.zh-CN.md](docs/ARCHITECTURE.zh-CN.md)