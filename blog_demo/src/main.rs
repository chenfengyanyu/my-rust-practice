/// 使用状态模式实现一个发布博客的工作流
/// 1.在新建博客文章时生成一份空白的草稿文档
/// 2.在草稿撰写完毕后，请求对这篇草稿状态的文章进行审批
/// 3.在文章通过审批后正式对外发布
/// 4.仅返回并打印成功发布后的文章，而不能意外地发布没有通过审批的文章

use blog::Post;
fn main() {
    // 创建一篇新的文章草稿
    let mut post = Post::new();
    // 草稿状态时将文字添加到文章中
    post.add_text("I ate a saled for lunch today");
    assert_eq!("", post.content()); // 草稿状态不能写入

    // 发出审批文章的请求
    post.request_review();
    assert_eq!("", post.content()); // 等待状态依然是空内容

    // 当文章获得审批并能够正式对外发布
    post.approve();
    assert_eq!("I ate a salad for lunch today", post.content()); // 审核后调用 content 方法则应当返回完整的文章内容
}
