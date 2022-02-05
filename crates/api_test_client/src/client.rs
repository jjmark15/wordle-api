use std::net::SocketAddr;

use reqwest::Response;

use crate::api_response::{ApiResponse, HttpResponseDetails};
use crate::guess_validation::GuessValidation;
use crate::response::{GuessValidationResponse, ResponseBody};

#[derive(derive_new::new)]
pub struct Client {
    http_client: reqwest::Client,
    server_address: SocketAddr,
}

impl Client {
    pub async fn status(&self) -> ApiResponse<()> {
        let url = format!("http://{}/admin/status", self.server_address);
        let response = self.http_client.get(url).send().await.unwrap();
        ApiResponse::new((), Self::http_response_details(&response))
    }

    pub async fn validate(&self, guess: impl AsRef<str>) -> ApiResponse<GuessValidation> {
        let url = format!(
            "http://{}/guess/validate/{}",
            self.server_address,
            guess.as_ref()
        );

        let response = self.http_client.get(url).send().await.unwrap();
        let response_details = Self::http_response_details(&response);

        let response_body = response
            .json::<ResponseBody<GuessValidationResponse>>()
            .await
            .unwrap();

        match response_body {
            ResponseBody::Error(error) => ApiResponse::from_error(error.into(), response_details),
            ResponseBody::Ok(validation_response) => {
                ApiResponse::new(validation_response.into(), response_details)
            }
        }
    }

    fn http_response_details(response: &Response) -> HttpResponseDetails {
        HttpResponseDetails::new(response.status(), response.headers().clone())
    }
}
