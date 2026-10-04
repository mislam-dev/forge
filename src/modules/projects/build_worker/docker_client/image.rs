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

    pub async fn create_with_log_handler<F, Fut>(
        &self,
        tag: &str,
        options: BuildImageOptions,
        build_context: Bytes,
        mut on_log: F,
    ) -> Result<String, DockerClientError>
    where
        F: FnMut(String) -> Fut,
        Fut: std::future::Future<Output = ()>,
    {
        let body: Either<Full<Bytes>, _> = Either::Left(Full::new(build_context));

        let mut stream = self.client.build_image(options, None, Some(body));

        while let Some(event) = stream.next().await {
            let event = event.map_err(|e| {
                DockerClientError::ImageError(format!("Failed to build image: {}", e))
            })?;
            if let Some(ref err_detail) = event.error_detail {
                let msg = err_detail
                    .message
                    .clone()
                    .unwrap_or_else(|| "Unknown docker build error".to_string());
                on_log(format!("ERROR: {}", msg)).await;
                return Err(DockerClientError::ImageError(msg));
            }
            if let Some(v) = event.stream {
                on_log(v).await;
            }
        }

        let image = self.client.inspect_image(tag).await.map_err(|e| {
            DockerClientError::ImageError(format!("Failed to inspect image: {}", e))
        })?;
        let image_id = image.id.unwrap_or_default();

        Ok(image_id)
    }

    pub async fn create(
        &self,
        tag: &str,
        options: BuildImageOptions,
        build_context: Bytes,
    ) -> Result<String, DockerClientError> {
        self.create_with_log_handler(tag, options, build_context, |v| async move {
            print!("{}", v);
        })
        .await
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
