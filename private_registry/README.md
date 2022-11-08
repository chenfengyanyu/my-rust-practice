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
```
sh alexandrie.sh git@github.com:chenfengyanyu/crates-index.git /Users/jartto/Documents/Project/alexandrie
```
> 这一步需要关注网络是否可用（确保代理开启），否则会出现 443 异常，导致下载失败。

脚本成功执行后，会看到如下输出：
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

接下来，我们来解释下 alexandrie.sh 脚本，它主要做了两件事情：
- Clone CRATE_INDEX_GIT_URL 仓库；
- 创建 config.json 文件；

config.json 配置如下：
```
{
    "dl": "http://{{host:port}}/api/v1/crates/{crate}/{version}/download",
    "api": "http://{{host:port}}",
    "allowed-registries": ["https://github.com/rust-lang/crates.io-index"]
}
```
注意：需要将这里的 host，port 最终换成线上部署的服务。
### 2.3 启动服务


### 2.4 验证服务


## 三、使用私有 crates 依赖


## 相关文档
- [Rust crates 私有化部署指南](https://baoyachi.github.io/Rust/crates_private_alternative_registry.html)
- [Alexandrie](https://github.com/Hirevo/alexandrie) Modular alternative crate registry for Rust
