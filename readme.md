## 社区

欢迎加入我们 https://rmw.zulipchat.com

## 克隆代码

```
git clone --recurse-submodules -j8 git@github.com:rmw-link/rust.git
```

升级所有子模块

```
git submodule update --recursive --remote
```

## 编译备忘

## 编译安卓版本

下载好andriod ndk，解压后放在同级目录

编辑 `~/.cargo/config`

加上

```
[target.aarch64-linux-android]
linker = "aarch64-linux-android30-clang"
ar = "aarch64-linux-android-ar"
```

然后运行 `./android.sh`

## 编译windows版本

安装并运行 [git bash](https://gitforwindows.org/)

编辑 `~/.cargo/config` 加上

```
[target.x86_64-pc-windows-msvc]
rustflags = ["-C", "target-feature=+crt-static"]
```

这样是为了避免windows缺少dll的报错，参见[Rust RFC : 1721-crt-static](https://rust-lang.github.io/rfcs/1721-crt-static.html)

## rust学习资料

* [rust primer 中文版](https://hardocs.com/d/rustprimer/macro/macro.html)
* [设计优雅的 Rust 库 API](https://mp.weixin.qq.com/s/02rb4OA22puTYxm4sG88_A)
* [anyhow和thiserror - 错误处理库](https://rustcc.cn/article?id=6dcbf032-0483-4980-8bfe-c64a7dfb33c7)
* [Rust的Pin与Unpin](https://folyd.com/blog/rust-pin-unpin/)

## 开发技巧

* [调试grpc : bloomrpc](https://github.com/uw-labs/bloomrpc)
* `git merge --squash dev` 可以稀疏合并dev分支，把多个日志合并为一个
