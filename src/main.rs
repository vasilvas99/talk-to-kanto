#[cfg(unix)]
use tokio::net::UnixStream;
use tonic::transport::{Endpoint, Uri};
use tower::service_fn;

pub mod containers {
    //This is a hack because tonic has an issue with deeply nested protobufs
    tonic::include_proto!("mod");
}
use containers::github::com::eclipse_kanto::container_management::containerm::api::services::containers as kanto;

#[cfg(unix)]
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let socket_path = "/run/container-management/container-management.sock";

    let channel = Endpoint::try_from("lttp://[::]:50051")?
        .connect_with_connector(service_fn(move |_: Uri| {
            // Connect to a Uds socket
            UnixStream::connect(socket_path)
        }))
        .await?;

    let mut client = kanto::containers_client::ContainersClient::new(channel);
    let request = tonic::Request::new(kanto::ListContainersRequest {});
    let response = client.list(request).await?;

    println!("RESPONSE={:?}", response);
    Ok(())
}
