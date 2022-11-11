# Rust Crates 私有化部署
适合企业私有化代码部署

## 核心步骤
- 创建存储 crates-index 的私有仓库
- 部署 alexandrie的私有服务，绑定域名
- 建立认证，获取 cargo login 的 token
- 对应 lib 执行 cargo publish
- 使用依赖，配置指定 registry

## 一、创建存储 crates-index 的私有仓库
### 1.1 创建在 Git 上的 crate index 的存储仓库
```bash
https://github.com/chenfengyanyu/crates-index
```
### 1.2 配置 config.toml 文件
如果不会配置，请查看[配置 Cargo 国内镜像源](https://github.com/chenfengyanyu/my-rust-practice/tree/main/crates_images)。
```bash
[registries]
mrust = { index = "https://github.com/chenfengyanyu/crates-index" }
# 或
[registries.mrust]
index = "https://github.com/chenfengyanyu/crates-index"
```
选择 mrust 源
```bash
[source.crates-io]
registry = "https://github.com/rust-lang/crates.io-index"
# 指定镜像
replace-with = 'mrust'
```

## 二、配置 crates 备用注册表的服务 
私有化的 crates 的注册服务，参考文档：[Installation script](https://hirevo.github.io/alexandrie/installation-script.html)。
### 2.1 创建安装配置脚本（alexandrie.sh）
```shell
#!/bin/bash

# function to run when an error is encountered
function setup_error {
    echo "-------- An error occurred during configuration --------"
    exit 1
}

# exit on error
trap 'setup_error' ERR

# directory to clone Alexandrie into:
ALEXANDRIE_DIR="$2";

# URL to the crate index repository.
CRATE_INDEX_GIT_URL="$1";


while ! git ls-remote -h $CRATE_INDEX_GIT_URL; do
    read -p 'CRATE_INDEX_GIT_URL: ' CRATE_INDEX_GIT_URL;
done

if ! cargo -V; then
    echo;
    echo "In order to build an instance of Alexandrie, you need to have Rust installed on your system";
    echo "You can learn how to install Rust on your system on the official Rust website:";
    echo "https://www.rust-lang.org/tools/install";
    echo;
    ! :;    # trigger error trap
fi

if [ -d "$ALEXANDRIE_DIR" ]; then
    echo
    echo "'$ALEXANDRIE_DIR' (ALEXANDRIE_DIR) is an existing directory, pulling latest changes ...";
    cd "$ALEXANDRIE_DIR";
    git pull;
    echo "Changes have been pulled successfully !";
    echo;
else
    echo;
    echo "Cloning Alexandrie in '$ALEXANDRIE_DIR' ...";
    git clone https://github.com/Hirevo/alexandrie.git "$ALEXANDRIE_DIR";
    cd "$ALEXANDRIE_DIR";
    echo "Successfully cloned Alexandrie !";
    echo;
fi

echo "Building Alexandrie (using the default features)...";
echo "(keep in mind that the default features may not fit your use-case, be sure to review them before deplying it to production)";
cargo build -p alexandrie;
echo "Alexandrie has been built successfully !";

# create the directory serving as the storage of crate archives.
mkdir -p crate-storage;

# setup the crate index.
if [ -d crate-index ]; then
    echo;
    echo "'${ALEXANDRIE_DIR}/crate-index' is an existing directory, pulling latest changes ...";
    cd crate-index;
    git pull;
    echo "Changes have been pulled successfully !";
    echo;
else
    echo;
    echo "Cloning crate index in '${ALEXANDRIE_DIR}/crate-index' ...";
    git clone "$CRATE_INDEX_GIT_URL" crate-index;
    cd crate-index;
    echo "Successfully cloned the crate index !";
    echo;
fi

# configure the crate index
if [ ! -f config.json ]; then
    echo "The crate index does not have a 'config.json' file.";
    echo "Creating an initial one (please also review it before deploying the registry in production) ..."
    cat > config.json << EOF;
{
    "dl": "http://$(hostname):3000/api/v1/crates/{crate}/{version}/download",
    "api": "http://$(hostname):3000",
    "allowed-registries": ["https://github.com/rust-lang/crates.io-index"]
}
EOF
    git add config.json;
    git commit -m 'Added `config.json`';
    git push -u origin master;
    echo "Initial 'config.json' file has been created and pushed to the crate index !";
    echo;
fi

echo "Alexandrie should be good to go for an initial run.";
echo "You can start the Alexandrie instance by:";
echo "  - navigating to '${ALEXANDRIE_DIR}'";
echo "  - tweaking the 'alexandrie.toml' file";
echo "  - run `./target/debug/alexandrie`";
echo;

```
这里注意，需要修改两个参数：
- ALEXANDRIE_DIR: 本地的路径，列如：/Users/jartto/Documents/Project/alexandrie
- CRATE_INDEX_GIT_URL: 就是我们在 1.1 创建的 crats index 的 Git 地址，如：git@github.com:chenfengyanyu/crates-index.git
### 2.2 执行脚本（alexandrie.sh）
alexandrie.sh 脚本，它主要做了两件事情：
- Clone CRATE_INDEX_GIT_URL 仓库；
- 创建 config.json 文件；
```
sh alexandrie.sh git@github.com:chenfengyanyu/crates-index.git /Users/jartto/Documents/Project/alexandrie
```
> 这一步需要关注网络是否可用（确保代理开启），否则会出现 443 异常，导致下载失败。

第一步，Clone Alexandrie 项目，控制台输出如下：
```
Alexandrie has been built successfully !

Cloning crate index in '/Users/jartto/Documents/Project/alexandrie/crate-index' ...
Cloning into 'crate-index'...
remote: Enumerating objects: 4, done.
remote: Counting objects: 100% (4/4), done.
remote: Compressing objects: 100% (3/3), done.
remote: Total 4 (delta 0), reused 0 (delta 0), pack-reused 0
Receiving objects: 100% (4/4), done.
Successfully cloned the crate index !
```
可能出现异常：
```
The crate index does not have a 'config.json' file.
Creating an initial one (please also review it before deploying the registry in production) ...
[main a326c06] Added `config.json`
 1 file changed, 5 insertions(+)
 create mode 100644 config.json
error: src refspec master does not match any
error: failed to push some refs to 'github.com:chenfengyanyu/crates-index.git'
-------- An error occurred during configuration --------
```
如果碰到上述问题，请查看[解决方案](https://blog.csdn.net/dietime1943/article/details/85682688)。
第二步，自动创建 config.json 文件，配置如下：
```
{
    "dl": "http://{{host:port}}/api/v1/crates/{crate}/{version}/download",
    "api": "http://{{host:port}}",
    "allowed-registries": ["https://github.com/rust-lang/crates.io-index"]
}
```
注意：需要将这里的 host，port 最终换成线上部署的服务。
```bash
### 最终成功日志
Cloning crate index in '/Users/jartto/Documents/Project/alexandrie/crate-index' ...
Cloning into 'crate-index'...
warning: You appear to have cloned an empty repository.
Successfully cloned the crate index !

The crate index does not have a 'config.json' file.
Creating an initial one (please also review it before deploying the registry in production) ...
[master (root-commit) 69f3990] Added `config.json`
 1 file changed, 5 insertions(+)
 create mode 100644 config.json
Enumerating objects: 3, done.
Counting objects: 100% (3/3), done.
Delta compression using up to 8 threads
Compressing objects: 100% (2/2), done.
Writing objects: 100% (3/3), 356 bytes | 356.00 KiB/s, done.
Total 3 (delta 0), reused 0 (delta 0), pack-reused 0
To github.com:chenfengyanyu/crates-index.git
 * [new branch]      master -> master
Branch 'master' set up to track remote branch 'master' from 'origin'.
Initial 'config.json' file has been created and pushed to the crate index !

Alexandrie should be good to go for an initial run.
You can start the Alexandrie instance by:
  - navigating to '/Users/jartto/Documents/Project/alexandrie'
  - tweaking the 'alexandrie.toml' file
alexandrie.sh: line 95: ./target/debug/alexandrie: No such file or directory
  - run
```
### 2.3 核心文件
通过上述步骤，我们建了三个项目，分别是：
- crates-shell：存放了执行脚本（alexandrie.sh）
- crates-index：在 Git 上的 crate index 的存储仓库
- alexandrie：私有化的 crates 注册服务
注意：alexandrie 会将我们创建的 crates-index 拷贝到 '/Users/jartto/Documents/Project/alexandrie/crate-index' 目录下。
```bash
Cloning crate index in '/Users/jartto/Documents/Project/alexandrie/crate-index' ...
Cloning into 'crate-index'...
```
### 2.4 启动服务
进入 alexandrie 目录，先构建 alexandrie：
```bash
# 进入目录
cd /Users/jartto/Documents/Project/alexandrie
# 构建
cargo build -p alexandrie
```
然后运行脚本
```bash
./target/debug/alexandrie -c  alexandrie.toml
```
看到日志台输出如下内容，则表明成功启动服务：
```bash
Nov 09 08:54:02.074 INFO starting Alexandrie (version: 0.1.0 - 6e0b9019)
Nov 09 08:54:02.075 INFO running database migrations
Nov 09 08:54:02.080 INFO setting up request logger middleware
Nov 09 08:54:02.080 INFO setting up session middleware
Nov 09 08:54:02.080 INFO setting up authentication middleware
Nov 09 08:54:02.080 INFO mounting '/'
Nov 09 08:54:02.080 INFO mounting '/me'
Nov 09 08:54:02.080 INFO mounting '/search'
Nov 09 08:54:02.080 INFO mounting '/most-downloaded'
Nov 09 08:54:02.080 INFO mounting '/last-updated'
Nov 09 08:54:02.080 INFO mounting '/crates/:crate'
Nov 09 08:54:02.081 INFO mounting '/account/login'
Nov 09 08:54:02.081 INFO mounting '/account/logout'
Nov 09 08:54:02.081 INFO mounting '/account/register'
Nov 09 08:54:02.081 INFO mounting '/account/manage'
Nov 09 08:54:02.081 INFO mounting '/account/manage/password'
Nov 09 08:54:02.081 INFO mounting '/account/manage/tokens'
Nov 09 08:54:02.081 INFO mounting '/account/manage/tokens/:token-id/revoke'
Nov 09 08:54:02.081 INFO mounting '/assets/*path'
Nov 09 08:54:02.081 INFO mounting '/api/v1/account/register'
Nov 09 08:54:02.081 INFO mounting '/api/v1/account/login'
Nov 09 08:54:02.081 INFO mounting '/api/v1/account/tokens'
Nov 09 08:54:02.081 INFO mounting '/api/v1/account/tokens/:name'
Nov 09 08:54:02.081 INFO mounting '/api/v1/categories'
Nov 09 08:54:02.081 INFO mounting '/api/v1/crates'
Nov 09 08:54:02.081 INFO mounting '/api/v1/crates/new'
Nov 09 08:54:02.081 INFO mounting '/api/v1/crates/suggest'
Nov 09 08:54:02.081 INFO mounting '/api/v1/crates/:name'
Nov 09 08:54:02.081 INFO mounting '/api/v1/crates/:name/owners'
Nov 09 08:54:02.081 INFO mounting '/api/v1/crates/:name/:version/yank'
Nov 09 08:54:02.081 INFO mounting '/api/v1/crates/:name/:version/unyank'
Nov 09 08:54:02.081 INFO mounting '/api/v1/crates/:name/:version/download'
Nov 09 08:54:02.081 INFO listening on '127.0.0.1:3000'
Nov 09 08:54:02.081 INFO Server listening on http://127.0.0.1:3000
```
访问服务，查看效果
![Welcome to Alexandrie !](https://github.com/chenfengyanyu/my-rust-practice/blob/main/private_registry/img/1.png?raw=true)
我们顺便注册一个账号，并生成 Token 以备下文使用。
![Created Token](https://github.com/chenfengyanyu/my-rust-practice/blob/main/private_registry/img/2.png?raw=true)
Cargo 会自动将 Token 保存至“~/.cargo/credential”。

### 2.5 登录 Cargo 账号
我们通过 Cargo login 来登录账号，并输入 2.4 节中生成的 Token，这样就可以方便的在私有库发布 Crates 了。
```bash
# cargo login --registry=your-registry-name
cargo login --registry=mrust
```
![Login Info](https://github.com/chenfengyanyu/my-rust-practice/blob/main/private_registry/img/3.png?raw=true)
创建一个 Rust lib，如下：
```
cargo new jartto_lib
```
通过 cargo publish 命令进行发布
```
cargo publish --registry=mrust --allow-dirty
```
注意：要发布未提交的更改可以使用“--allow-dirty”参数。

如果有如下异常：
```bash
error: failed to publish to registry at http://JarttodeiMac.lan:3000
```
需要修改文件“alexandrie/crate-index/config.json”
```bash
{
    "dl": "http://{{host:port}}/api/v1/crates/{crate}/{version}/download",
    "api": "http://{{host:port}}/",
    "allowed-registries": ["https://github.com/rust-lang/crates.io-index"]
}
```
重新 Publish 即可。
## 三、使用私有 crates 依赖
使用私有 crates 依赖的时候，只需要确定 registry = "mrust" 即可。
```
...
[dependencies]
serde_derive = "1.0.111"
serde = "1.0.111"
serde_json = "1.0.53"
...

hula_common = { version="0.1.4", registry = "mrust" }
B = { package = "B", version="0.3.2", registry = "mrust" }
A = { package = "A", version="0.5.7", registry = "mrust" }
...
```
## 四、私有库运维
由于 Cargo 仓库被设计成永久保存，发布上去的 crate 不能删除（无法重新发布同版本的 crate）。这意味着一旦传错，只能在服务端删库重来：
```
rm alexandrie.db
```

## 相关文档
- [Rust crates 私有化部署指南](https://baoyachi.github.io/Rust/crates_private_alternative_registry.html)
- [Alexandrie](https://github.com/Hirevo/alexandrie) Modular alternative crate registry for Rust
- [Cargo 私有仓库部署](https://blog.csdn.net/ssdlearnerused/article/details/108596640)

