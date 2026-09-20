use super::errors::DockerClientError;
use bollard::Docker;
use bollard::query_parameters::{BuildImageOptions, RemoveImageOptions};
use bytes::Bytes;
use http_body_util::{Either, Full};
use tokio_stream::StreamExt;

#[derive(Clone)]
pub struct DockerImage {
    client: Docker,
}

impl DockerImage {
    pub fn new(client: Docker) -> Self {
        Self { client }
    }

    pub async fn create(
        &self,
        tag: &str,
        options: BuildImageOptions,
        build_context: Bytes,
    ) -> Result<String, DockerClientError> {
        let body: Either<Full<Bytes>, _> = Either::Left(Full::new(build_context));

        let mut stream = self.client.build_image(options, None, Some(body));

        while let Some(event) = stream.next().await {
            let event = event.map_err(|e| {
                DockerClientError::ImageError(format!("Failed to build image 1: {}", e))
            })?;
            if let Some(v) = event.stream {
                print!("{}", v);
            }
        }

        let image = self.client.inspect_image(tag).await.map_err(|e| {
            DockerClientError::ImageError(format!("Failed to inspect image: {}", e))
        })?;
        let image_id = image.id.unwrap_or_else(|| String::from(""));

        Ok(image_id)
    }

    pub async fn remove(&self, tag: &str) -> Result<(), DockerClientError> {
        self.client
            .remove_image(tag, None::<RemoveImageOptions>, None)
            .await
            .map_err(|e| {
                DockerClientError::ImageError(format!(
                    "Failed to remove the '{}' image: {}",
                    tag, e
                ))
            })?;
        Ok(())
    }
}
