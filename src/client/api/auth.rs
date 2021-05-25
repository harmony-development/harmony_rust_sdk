pub use crate::api::auth::{
    auth_step, next_step_request, AuthStep, NextStepRequest, Session, StepBackRequest,
    StreamStepsRequest,
};

use super::*;
use crate::{
    api::auth::{next_step_request::form_fields::Field, *},
    client_api,
};

/// A response to an [`AuthStep`].
#[derive(Debug, Clone)]
pub enum AuthStepResponse {
    /// A choice selection.
    Choice(String),
    /// A form.
    Form(Vec<Field>),
    Initial,
}

impl AuthStepResponse {
    /// Create a new [`AuthStepResponse`] with the specified choice.
    #[inline(always)]
    pub fn choice(choice: impl ToString) -> Self {
        Self::Choice(choice.to_string())
    }

    /// Create a new [`AuthStepResponse`] with the specified form fields.
    #[inline(always)]
    pub fn form(fields: Vec<Field>) -> Self {
        Self::Form(fields)
    }

    /// A login choice response.
    #[inline(always)]
    pub fn login_choice() -> Self {
        Self::choice("login")
    }

    /// A register choice response.
    #[inline(always)]
    pub fn register_choice() -> Self {
        Self::choice("register")
    }

    /// Create a login form response from specified email and password.
    pub fn login_form(email: impl ToString, password: impl ToString) -> Self {
        Self::form(vec![
            Field::String(email.to_string()),
            Field::Bytes(password.to_string().into_bytes()),
        ])
    }

    /// Create a register form response from specified email, username and password.
    pub fn register_form(
        email: impl ToString,
        username: impl ToString,
        password: impl ToString,
    ) -> Self {
        Self::form(vec![
            Field::String(email.to_string()),
            Field::String(username.to_string()),
            Field::Bytes(password.to_string().into_bytes()),
        ])
    }
}

impl From<AuthStepResponse> for Option<next_step_request::Step> {
    fn from(other: AuthStepResponse) -> Option<next_step_request::Step> {
        match other {
            AuthStepResponse::Choice(choice) => {
                Some(next_step_request::Step::Choice(next_step_request::Choice {
                    choice,
                }))
            }
            AuthStepResponse::Form(fields) => {
                Some(next_step_request::Step::Form(next_step_request::Form {
                    fields: fields
                        .into_iter()
                        .map(|f| next_step_request::FormFields { field: Some(f) })
                        .collect(),
                }))
            }
            AuthStepResponse::Initial => None,
        }
    }
}

client_api! {
    /// Starts an authentication session.
    response: BeginAuthResponse,
    request: (),
    api_fn: begin_auth,
    service: auth,
}

/// Convenience type to create a valid [`NextStepRequest`].
#[into_request("NextStepRequest")]
#[derive(Debug, Clone, new)]
pub struct AuthResponse {
    auth_id: String,
    step: AuthStepResponse,
}

client_api! {
    /// Requests the next step of an authentication session from the homeserver.
    response: AuthStep,
    request: NextStepRequest,
    api_fn: next_step,
    service: auth,
}

/// Wrapper around an auth ID which can be used as multiple requests.
#[into_request("StepBackRequest", "StreamStepsRequest")]
#[derive(Debug, Clone, From, Into, Display, new)]
pub struct AuthId {
    auth_id: String,
}

client_api! {
    /// Steps back in an authentication session.
    response: AuthStep,
    request: StepBackRequest,
    api_fn: step_back,
    service: auth,
}

client_api! {
    /// Check if logged in to server.
    request: (),
    api_fn: check_logged_in,
    service: auth,
}

/// Stream steps sent from the server.
pub async fn stream_steps(
    client: &Client,
    request: impl Into<StreamStepsRequest>,
) -> ClientResult<hrpc::client::socket::ReadSocket<StreamStepsRequest, AuthStep>> {
    use hrpc::IntoRequest;

    let req = request.into().into_request();
    let response = client.auth_lock().await.stream_steps(req).await;
    tracing::debug!("Received response: {:?}", response);
    response.map_err(Into::into)
}
