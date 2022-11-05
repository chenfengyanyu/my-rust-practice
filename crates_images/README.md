# 配置 Cargo 国内镜像源
Rust 官方服务器部署在北美洲，中国大陆用户下载速度较慢，经常遇到 Block 情况。因此为了使用更流畅一些，建议配置国内镜像源。当然，如果你愿意等待较长时间，可以采用默认的官方源。

## 如何配置
本文提供两种形式：全局配置和局部配置。
### 方式一：全局配置（推荐）
查找配置文件
```
where cargo
```
终端会提示
```
/Users/jartto/.cargo/bin/cargo
```
直接访问
```
cd /Users/jartto/.cargo
```
如果文件目录下没有 config 文件，那么需要新建一个（touch config 新建文件 config 文件），并写入如下配置
```bash
[source.crates-io]
registry = "https://github.com/rust-lang/crates.io-index"
# 指定镜像
replace-with = 'rsproxy' # 如：tuna、sjtu、ustc、rsproxy，或者 rustcc

# 注：以下源配置一个即可，无需全部

# 中国科学技术大学
[source.ustc]
registry = "git://mirrors.ustc.edu.cn/crates.io-index"

# 上海交通大学
[source.sjtu]
registry = "https://mirrors.sjtug.sjtu.edu.cn/git/crates.io-index"

# 清华大学
[source.tuna]
registry = "https://mirrors.tuna.tsinghua.edu.cn/git/crates.io-index.git"

# rustcc社区
[source.rustcc]
registry = "https://code.aliyun.com/rustcc/crates.io-index.git"

# 头条
[source.rsproxy]
registry = "https://rsproxy.cn/crates.io-index"
[source.rsproxy-sparse]
registry = "sparse+https://rsproxy.cn/index/"

[registries.rsproxy]
index = "https://rsproxy.cn/crates.io-index"

[net]
git-fetch-with-cli = true
```
配置完成，保存即可。

### 方式二：局部配置
在项目工程结构中，与 Cargo.toml 同级目录的 .cargo 文件夹下创建 config 文件，config 文件配置方法和内容与第一种相同。

## 选择镜像源
提供了 Rust 工具链镜像源的站点，一般也会提供 Cargo 国内镜像源服务。目前，国内 Cargo 镜像源有：清华大学源、中国科学技术大学源、上海交通大学源、头条RSProxy，以及 rustcc 社区源。比如，我们就选择了清华大学的镜像源 tuna。
```
[source.crates-io]
registry = "https://github.com/rust-lang/crates.io-index"
# 指定镜像
replace-with = 'tuna' # 如：tuna、sjtu、ustc，或者 rustcc
```

## 测试镜像源
问题来了，镜像源是不少，但是如何知道镜像源的速度呢？
### TOP1
```
# 清华大学
PING bfdmirrors.s.tuna.tsinghua.edu.cn (101.6.15.130): 56 data bytes
64 bytes from 101.6.15.130: icmp_seq=0 ttl=50 time=8.986 ms
64 bytes from 101.6.15.130: icmp_seq=1 ttl=50 time=14.003 ms
64 bytes from 101.6.15.130: icmp_seq=2 ttl=50 time=15.006 ms
64 bytes from 101.6.15.130: icmp_seq=3 ttl=50 time=15.463 ms
64 bytes from 101.6.15.130: icmp_seq=4 ttl=50 time=9.362 ms
64 bytes from 101.6.15.130: icmp_seq=5 ttl=50 time=10.075 ms
64 bytes from 101.6.15.130: icmp_seq=6 ttl=50 time=9.298 ms
64 bytes from 101.6.15.130: icmp_seq=7 ttl=50 time=16.872 ms
```
### TOP2
```
# 头条
PING direct.toutiao-other-v3.hl.bytelb.net (110.249.198.6): 56 data bytes
64 bytes from 110.249.198.6: icmp_seq=0 ttl=53 time=19.447 ms
64 bytes from 110.249.198.6: icmp_seq=1 ttl=53 time=18.868 ms
64 bytes from 110.249.198.6: icmp_seq=2 ttl=53 time=27.473 ms
64 bytes from 110.249.198.6: icmp_seq=3 ttl=53 time=26.829 ms
64 bytes from 110.249.198.6: icmp_seq=4 ttl=53 time=18.848 ms
64 bytes from 110.249.198.6: icmp_seq=5 ttl=53 time=25.957 ms
64 bytes from 110.249.198.6: icmp_seq=6 ttl=53 time=22.957 ms
```
### TOP3
```
# 中国科学技术大学
PING mirrors.ustc.edu.cn (218.104.71.170): 56 data bytes
64 bytes from 218.104.71.170: icmp_seq=0 ttl=56 time=22.320 ms
64 bytes from 218.104.71.170: icmp_seq=1 ttl=56 time=28.426 ms
64 bytes from 218.104.71.170: icmp_seq=2 ttl=56 time=28.489 ms
64 bytes from 218.104.71.170: icmp_seq=3 ttl=56 time=21.730 ms
64 bytes from 218.104.71.170: icmp_seq=4 ttl=56 time=28.143 ms
```
### TOP4
```
# 上海交通大学
PING mirrors.sjtug.sjtu.edu.cn (202.120.58.155): 56 data bytes
64 bytes from 202.120.58.155: icmp_seq=0 ttl=46 time=32.485 ms
64 bytes from 202.120.58.155: icmp_seq=1 ttl=46 time=37.849 ms
64 bytes from 202.120.58.155: icmp_seq=2 ttl=46 time=38.564 ms
64 bytes from 202.120.58.155: icmp_seq=3 ttl=46 time=37.983 ms
64 bytes from 202.120.58.155: icmp_seq=4 ttl=46 time=31.548 ms
64 bytes from 202.120.58.155: icmp_seq=5 ttl=46 time=37.042 ms
64 bytes from 202.120.58.155: icmp_seq=6 ttl=46 time=34.583 ms
```
### TOP5
```
# rustcc社区
PING code.aliyun.com (47.98.49.44): 56 data bytes
64 bytes from 47.98.49.44: icmp_seq=0 ttl=90 time=36.091 ms
64 bytes from 47.98.49.44: icmp_seq=1 ttl=90 time=35.661 ms
64 bytes from 47.98.49.44: icmp_seq=2 ttl=90 time=39.757 ms
64 bytes from 47.98.49.44: icmp_seq=3 ttl=90 time=36.315 ms
64 bytes from 47.98.49.44: icmp_seq=4 ttl=90 time=34.802 ms
64 bytes from 47.98.49.44: icmp_seq=5 ttl=90 time=38.668 ms
64 bytes from 47.98.49.44: icmp_seq=6 ttl=90 time=40.714 ms
```
## 参考
[配置 Cargo 国内镜像源](https://mirrors.gitcode.host/zzy/rust-crate-guide/4-cargo/4.1-source-replacement.html)
[开源软件的国内镜像站点](https://lework.github.io/lemonitor/#/usage)
[清华大学开源软件镜像站](https://mirrors.tuna.tsinghua.edu.cn/)
[RsProxy.cn](https://rsproxy.cn/)