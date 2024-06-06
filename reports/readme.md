# intro

这个文件夹下记录我是如何完成实习任务的。

首先修改rust-toolchain.toml的channel改为nightly，不然我这里没有对应的rustc版本/

## 任务1 参照arceos/apps构建同等功能系统

我直接将arceos中的`api`,`apps`,`crates`,`modules`,`platforms`,`ulib`复制了过来

然后将根目录下的Cargo.toml的members注释，profile.release注释，剩下的所有patch都注释

然后向其中添加想要运行的apps项目，比如下面这样
```toml
members = [
    # "axconfig/rt_axconfig",
    # ...
    # "macrokernel/rt_macrokernel",
    "apps/rt_helloworld",
    "apps/rt_memtest"
]
```

接着修改Repo.toml，向其中添加
```toml
# 我看了lktool工具的要求，root组件必须rt_或test_开头，然后必须要被一个文件夹包裹
rt_memtest = "apps" 
rt_helloworld = "apps"
```

之后直接运行lk run即可