# legal_24_nums

一个能够统计出指定范围数字内的所有能够通过加减乘除计算得到24点的组合的程序。

## 功能特点

- 自动查找指定范围内所有可以通过加减乘除计算得到 24 点的 4 个数字组合
- 使用 Rayon 库实现并行计算，提高性能
- 支持将结果保存为 JSON 文件
- 灵活的命令行参数配置

## 依赖

- `clap` - 命令行参数解析
- `rayon` - 充分利用多核CPU并行计算
- `serde_json` - JSON 序列化

## 使用方法

### 运行

默认查找 1 到 10 之间的组合：

### 命令行参数
- `-h, --help` - 查看帮助信息
- `-s, --start <START>` - 起始数字（默认：1）
- `-e, --end <END>` - 结束数字（默认：10）
- `-f, --file` - 生成结果文件

## 示例

### 方式一：直接使用
```bash
# 查找1到10之间的组合
$ cargo run -- --start 1 --end 10

# 查找1到20之间的组合并将结果写入文件
$ cargo run -- --start 1 --end 20 --file
```

### 方式二：编译后安装到全局使用
```bash
$ cargo build --release
$ cargo install --path .

# 在任意目录下运行命令

$ legal_24_nums --start 1 --end 10
$ legal_24_nums --start 1 --end 20 --file
```

## 项目结构

- `src/lib.rs` - 核心库函数
- `src/main.rs` - 命令行入口
