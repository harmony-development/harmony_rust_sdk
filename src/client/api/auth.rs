use super::*;

use crate::{
    api::auth::{next_step_request::form_fields::Field, *},
    client_api,
};

pub use crate::api::auth::{auth_step, next_step_request, AuthStep, Session};

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

impl Into<Option<next_step_request::Step>> for AuthStepResponse {
    fn into(self) -> Option<next_step_request::Step> {
        match self {
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
    api_func: begin_auth,
    service: auth,
}

client_api! {
    /// Requests the next step of an authentication session from the homeserver.
    args: {
        auth_id: String,
        step: Option<next_step_request::Step>,
    },
    response: AuthStep,
    request_type: NextStepRequest,
    api_func: next_step,
    service: auth,
}

client_api! {
    /// Steps back in an authentication session.
    args: { auth_id: String, },
    response: AuthStep,
    request_type: StepBackRequest,
    api_func: step_back,
    service: auth,
}

client_api! {
    /// Stream steps sent from the server.
    args: { auth_id: String, },
    response: tonic::Streaming<AuthStep>,
    request_type: StreamStepsRequest,
    api_func: stream_steps,
    service: auth,
}
