// 这段代码实现了一个**“每秒钟自动打一次招呼”**的复读机：
// 打印日志：每当有一个浏览器连接进来，它先在后台打印出是谁连接了（User-Agent）。
// 创建流（Stream）：stream::repeat_with 创建了一个无限循环的源，内容是字符串 "hi!"。
// 限流（Throttle）：.throttle(Duration::from_secs(1)) 规定每秒钟只发一次。如果不加这行，你的服务器会用 100% 的 CPU 疯狂向用户发“hi!”，瞬间撑爆网络。
// 保活（Keep Alive）：keep_alive 每秒发一个特殊的注释文本（keep-alive-text）。这就像服务器在拉着用户的手说：“我还在呢，别把连接断开。”

use std::{convert::Infallible, time::Duration};

use axum::response::{Sse, sse::Event};
use axum_extra::{TypedHeader, headers};
use futures::{Stream, stream};
use tokio_stream::StreamExt;

pub(crate) async fn sse_handler(
    TypedHeader(user_agent): TypedHeader<headers::UserAgent>,
) -> Sse<impl Stream<Item = Result<Event, Infallible>>> {
    println!("`{}` connected", user_agent.as_str());

    // A `Stream` that repeats an event every second
    //
    // You can also create streams from tokio channels using the wrappers in
    // https://docs.rs/tokio-stream
    let stream = stream::repeat_with(|| Event::default().data("hi!"))
        .map(Ok)
        .throttle(Duration::from_secs(1));

    Sse::new(stream).keep_alive(
        axum::response::sse::KeepAlive::new()
            .interval(Duration::from_secs(1))
            .text("keep-alive-text"),
    )
}
