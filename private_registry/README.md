# Rust Crates 私有化部署
适合企业私有化代码部署

## 核心步骤
- 创建存储 crates-index 的私有仓库
- 部署 alexandrie的私有服务，绑定域名
- 建立认证，获取 cargo login 的 token
- 对应 lib 执行 cargo publish
- 使用依赖，配置指定 registry

## 相关文档
- [Rust crates 私有化部署指南](https://baoyachi.github.io/Rust/crates_private_alternative_registry.html)
