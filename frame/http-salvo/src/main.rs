use salvo::prelude::*;

#[handler]
async fn hello() -> &'static str {
	"Hello World"
}


#[tokio::main]
async fn main() {
	tracing_subscriber::fmt().init();

	let router = Router::new().get(hello);
	let acceptor = TcpListener::new("127.0.0.1:5800").bind().await;
	Server::new(acceptor).serve(router).await;
}