# lkmodel
LK model components.

### 下载lktool和编译该工具

选择一个工作目录下载lktool工具并编译。

```sh
git clone git@github.com:shilei-massclouds/lktool.git
cd lktool
git checkout refine_init
cargo build
```

把lktool加入到环境变量，采取临时方式

```sh
export PATH=$PATH:/home/cloud/gitWork/lktool/target/debug
```

> 注意：需要把/home/cloud/gitWork/lktool替换为实际路径

### 进入lkmodel目录，并切换分支

当前最新分支：regression_testing

```sh
git checkout regression_testing
```

### 查看可以作为根的组件

```sh
lktool list -c root
```

选择一个root组件rt_macrokernel为示例，基于它可以构建宏内核。

### 选择一个根组件用于构建内核系统

```sh
lktool chroot rt_macrokernel
```

### 配置目标内核

目前仅能选择体系结构

```sh
lktool config [riscv64|x86_64|]
```

另外，下步需要能够支持lktool menuconfig，以精细的控制配置选项，以替代features控制的方式。

### 为构建和运行内核作准备

以当前要构建的宏内核为例，需要为它创建根文件系统磁盘，格式化和安装部分linux应用以进行测试验证。

```sh
lktool prepare
```

当前目录下建立了disk.img磁盘文件，其中包含必要的linux应用程序和应用库。

可以选择在文件系统中加入ltp测试用例，目前只有mmapXX集合，详见最后。

### 构建并运行目标内核

```rust
lktool run
```

正常会切换用户态之后，启动第一个用户态应用init，该应用目前只是打印Hello，确认内核的构建和模块的测试成功。

注：可以随时按照“配置目标内核”的方式切换当前体系结构，重新构建或运行目标内核。

### 查看仓库中的普通组件

```sh
lktool list
```

显示：

```sh
cloud@server:/tmp/test_earlycon$ lktool list
boot
config
earlycon
```

### 依赖关系

在工程目录下执行命令，产生依赖关系图

```sh
lktool dep-graph
```

这个工具需要改进，能够适应体系结构的配置，目前展示不全。

### 引入LTP测试用例

1. 下载并编译ltp

```sh
git clone git@github.com:shilei-massclouds/ltp.git
git checkout lkmodel
cd lkmodel
make autotools
./mk_riscv64.sh
```

注: 对x86_64平台，执行./mk_x86_64.sh

2. 在lkmodel目录下配置lk.toml

把下面的**path**改为第1步的路径。

```
[ltp]
path = "/home/cloud/gitWork/ltp"
```
