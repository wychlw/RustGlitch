# nfuzz2 架构与运行逻辑（nightly + synmutate）

本文档描述当前程序的核心结构、数据流、线程模型与停止条件。

## 1. 总体模块

- 入口与调度： [src/main.rs](src/main.rs)
- 参数与配置： [src/conf.rs](src/conf.rs)
- Fuzzer 抽象与编译执行： [src/fuzz/fuzzbase.rs](src/fuzz/fuzzbase.rs)
- SynMutate 策略： [src/strategies/synmutate/mod.rs](src/strategies/synmutate/mod.rs)
- AST 变异核心： [src/strategies/synmutate/astmutator.rs](src/strategies/synmutate/astmutator.rs)
- ICE 去重过滤器：
  - [src/ice_process/querystack.rs](src/ice_process/querystack.rs)
  - [src/ice_process/panicfunc.rs](src/ice_process/panicfunc.rs)

## 2. 运行主流程

1) 解析参数（CLI + 可选 JSON config）

- `Args::parse()` 后调用 `apply_config_if_needed()`。
- 可从 config 覆盖 `synmutate` 参数、jobs、filters、keep_results、停止条件等。

2) 初始化过滤器

- 根据 `--filters` 创建过滤器对象。
- 调用 `import()` 读取已有去重状态（如 `panic_filters.json`、`func_filters.json`）。

3) 解析并构建 Job 流水线（DSL）

- 语法：`tasker:task(args)`
- 解析位置： [src/conf.rs](src/conf.rs)
- 运行位置： [src/main.rs](src/main.rs)

- 常用链：`node-mutate:gen` -> `dummy:fuzz` -> `dump:raw`。
- `node-mutate:gen` 负责产生变异程序。
- `dummy:fuzz` 负责调用 rustc 编译、ICE 去重判定，并生成“待输出计划”。
- `dump:raw` / `dump:pretty` 负责最终落盘输出（受 keep 过滤控制）。

额外配置步骤：

- `*:filter(query-stack&panic-func)`：运行时覆盖使用的 ICE 去重过滤器。
- `*:filter(ice|success|compile-error|hang)`：运行时覆盖 `keep` 结果类型。

4) 多线程执行循环

- 每个线程调用 `run()`，共享全局索引。
- 每轮执行 `JobHolder::do_once()`，按 job 顺序流水处理。

5) 分类与输出（流水线融合）

- 编译结果分为：`ice / success / compile-error / hang`。
- 在流水线模式下，`--keep` 在 `dump` 阶段生效，决定是否落盘输出。
- 文件前缀：`ice_*.rs`、`success_*.rs`、`compile_error_*.rs`、`hang_*.rs`。
- `dump:raw` / `dump:pretty` 可选择原样输出或尝试格式化输出。

兼容逻辑：

- 若未配置 `dump` job，程序保持旧行为：`fuzz` 阶段直接输出文件。

6) 停止与收尾

- 满足任意停止条件后退出（见第 5 节）。
- 导出过滤器状态 `export()`，供下次运行复用。

## 3. SynMutate 逻辑

`ASTMutator` 包含三种模式：

- `Add`：遍历样本 AST，收集节点到 `nodeset`。
- `Modify`：按权重从 `nodeset` 选节点替换当前节点。
- `Adjust(dup)`：根据是否重复 ICE 调整权重。

关键参数（来自 `Args`）：

- `mutate_p`
- `max_nested`
- `max_analyze_depth`
- `new_ice_adj_rate`
- `dup_ice_adj_rate`
- `choose_adj_rate`
- `min_choose`

权重自适应策略：

- 发现新 ICE：倾向提升相关节点权重。
- 命中重复 ICE：降低相关节点权重。
- 节点被选择后：按衰减系数调整，避免单一路径过拟合。

## 4. ICE 去重机制

默认过滤器：`query-stack` + `panic-func`。

- `query-stack`：提取 rustc panic query stack 作为签名。
- `panic-func`：提取 panic 位点（文件:行列）作为签名。

判重逻辑：

- 只有“未被过滤器判重”的 ICE 会输出为新 ICE 文件。
- 过滤器状态会持久化到 `datas` 目录。

## 5. 停止条件（执行控制）

循环停止条件由以下共同控制：

- `-l/--loopcnt`：最大轮数
- `--until-ice`
- `--until-success`
- `--until-compile-error`
- `--timeout-sec`
- Ctrl+C / SIGTERM

语义：

- 若设置多个 `--until-*`，需全部达到。
- 任意时刻超时则停止。
- 输出计数基于“实际落盘输出”，因此目标类型需在 `--keep` 中启用。

## 6. 线程与共享状态

- 全局索引通过 `AtomicUsize` 分配，避免重复索引。
- 过滤器通过 `Arc<Mutex<...>>` 在线程间共享，保证去重一致性。
- 结果计数（ice/success/compile-error）通过原子计数器统计。

## 7. 推荐实验模式

- 输入：组合 `demo` + `tmp/issue` + 历史样本目录。
- jobs：`node-mutate:gen dummy:fuzz dump:raw`
- keep：`ice,success,compile-error`
- 停止：`-l` 大上限 + `--until-*` + `--timeout-sec`

示例：

- `cargo run --release -- -i demo -i tmp/issue -o out_verify --datas . -l 100000 --use-unstable --keep ice,success,compile-error --until-ice 3 --until-compile-error 200 --timeout-sec 3600 node-mutate:gen dummy:fuzz dump:raw`
