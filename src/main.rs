use cdrs_tokio::authenticators::StaticPasswordAuthenticatorProvider;
use cdrs_tokio::cluster::session::{SessionBuilder, TcpSessionBuilder};
use cdrs_tokio::cluster::NodeTcpConfigBuilder;
use cdrs_tokio::frame::Version;
use cdrs_tokio::load_balancing::RoundRobinLoadBalancingStrategy;
use std::sync::Arc;
use tracing_subscriber;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let user = "cassandra";
    let password = "cassandra";
    let auth = StaticPasswordAuthenticatorProvider::new(&user, &password);

    let config = NodeTcpConfigBuilder::new()
        .with_contact_point("127.0.0.1:9043".into())
        .with_authenticator_provider(Arc::new(auth))
        .with_version(Version::V5)
        .build()
        .await
        .unwrap();

    let session = TcpSessionBuilder::new(RoundRobinLoadBalancingStrategy::new(), config)
        .build()
        .unwrap();

    let create_ks = "SELECT * FROM system.peers_v2";

    let r = session.query(create_ks).await.unwrap();

    println!("result : {:?}", r);
}
