pub use crate::api::auth::{
    auth_step, next_step_request, AuthStep, NextStepRequest, Session, StepBackRequest,
    StreamStepsRequest,
};

use super::*;
use crate::api::auth::next_step_request::form_fields::Field;

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

/// Convenience type to create a valid [`NextStepRequest`].
#[into_request("NextStepRequest")]
#[derive(Debug, Clone, new)]
pub struct AuthResponse {
    auth_id: String,
    step: AuthStepResponse,
}

/// Wrapper around an auth ID which can be used as multiple requests.
#[into_request("StepBackRequest", "StreamStepsRequest")]
#[derive(Debug, Clone, From, Into, Display, new)]
pub struct AuthId {
    auth_id: String,
}