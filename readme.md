# Hardware ID Generator / 硬件 ID 生成器

[English](#english) | [中文](#中文)

## English

A cross-platform tool for generating unique hardware identifiers for deployment authorization and verification.

### Features

- Generates unique hardware ID using:
  - Primary identifiers:
    - Motherboard serial number
    - Motherboard UUID
    - Network interface MAC addresses
  - Secondary identifiers:
    - CPU physical ID
    - Disk model information
    - Product model name
- Output format: `XXXX-XXXX-XXXX-XXXX` (16-character unique code using MD5)
- Platform support:
  - Windows (x64)
  - Linux (x64)
  - macOS (Intel x64)

### Prerequisites

- Rust toolchain (1.70.0 or later)
- Docker

### Building

```bash
./build-all.sh
```

The script will automatically:
1. Check and install required tools (MinGW-w64, musl-cross)
2. Add necessary Rust targets
3. Build for all platforms

The compiled binaries will be in `target/release/output/`:
- `hardware_id.exe` (Windows)
- `hardware_id_linux` (Linux)
- `hardware_id_mac` (macOS)

### Usage

Run the executable to display hardware information and generate a unique code:

```bash
./hardware_id

# Example output:
Hardware Information:
- Motherboard: XXX-XXX-XXX
- CPU: Intel(R) Core(TM) ...
- Network: eth0 (XX:XX:XX:XX:XX:XX)

Unique Code: ABCD-EFGH-IJKL-MNOP
```

### Security & Privacy

- Only collects essential hardware information
- Sensitive information is hashed before output
- No personal data or user information is collected
- Network information limited to physical interfaces

### Known Limitations

- macOS: Intel x64 platform only (Apple Silicon not supported)
- Virtual Machines
  - Hardware information might change after VM restart or migration

### License

MIT License

---

## 中文

用于部署授权和验证的跨平台硬件标识符生成工具。

### 功能特点

- 基于以下信息生成唯一硬件ID：
  - 主要标识符：
    - 主板序列号
    - 主板UUID
    - 网卡MAC地址
  - 次要标识符：
    - CPU物理ID
    - 硬盘型号信息
    - 产品型号名称
- 输出格式：`XXXX-XXXX-XXXX-XXXX`（16位唯一码，使用MD5算法）
- 平台支持：
  - Windows (x64)
  - Linux (x64)
  - macOS (Intel x64)

### 环境要求

- Rust工具链（1.70.0或更高版本）
- Docker

### 构建方法

```bash
./build-all.sh
```

脚本会自动：
1. 检查并安装所需工具（MinGW-w64, musl-cross）
2. 添加必要的Rust目标平台
3. 构建所有平台版本

编译后的文件将位于 `target/release/output/` 目录：
- `hardware_id.exe` (Windows版)
- `hardware_id_linux` (Linux版)
- `hardware_id_mac` (macOS版)

### 使用方法

运行可执行文件以显示硬件信息并生成唯一码：

```bash
./hardware_id

# 输出示例：
硬件信息：
- 主板：XXX-XXX-XXX
- CPU：Intel(R) Core(TM) ...
- 网卡：eth0 (XX:XX:XX:XX:XX:XX)

唯一码：ABCD-EFGH-IJKL-MNOP
```

### 安全性与隐私

- 仅收集必要的硬件信息
- 敏感信息经过哈希处理
- 不收集个人数据或用户信息
- 网络信息仅限物理接口

### 已知限制

- macOS：仅支持Intel x64平台（不支持Apple Silicon）
- 虚拟机
  - 重启或迁移后硬件信息可能发生变化

### 许可证

MIT许可证