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
    //The uri is ignored and a UDS connection is established instead.
    let channel = Endpoint::try_from("http://[::]:50051")?
        .connect_with_connector(service_fn(move |_: Uri| UnixStream::connect(socket_path)))
        .await?;

    // List all containers
    let mut client = kanto::containers_client::ContainersClient::new(channel);
    let request = tonic::Request::new(kanto::ListContainersRequest {});
    let response = client.list(request).await?;
    println!("RESPONSE={:?}", response);

    // Search for specific container, serving as an example how to use the serde json ser-deserialization
    let container_lookup_name = String::from("test");
    let request = tonic::Request::new(kanto::GetContainerRequest {
        id: container_lookup_name,
    });
    // Consume the response object and return the message
    let response = client.get(request).await?.into_inner();
    // print out the json
    println!("Last response as json: {}", serde_json::to_string(&response)?);
    
    Ok(())
}
