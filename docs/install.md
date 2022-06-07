## 安装rust
```shell
curl https://sh.rustup.rs -sSf | sh
```

## 更新
```shell
rustup update
```

## 卸载
```shell
rustup self uninstall
```


## 查看版本
```shell
rustc --version  x.y.z(aaabbcc yyyy-dd-mm)
```

## 文档
```shell
rustup doc
```

## 编译器
```shell
rustc只适合简单的编译
```

## cargo
```shell
cargo是rust的构建系统和包管理工具
- 构建代码
- 下载依赖的库
- 构建库

cargo --version

创建项目
cargo new hello_cargo

cargo build  构建
 
cargo run  构建并且运行

cargo check 检查代码

为发布构建
cargo build --release
```

编程范式的演进：
面向过程 -》面向对象 -》模板元编程(泛型编程) -》函数式编程