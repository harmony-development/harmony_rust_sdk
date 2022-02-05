/// v1 of auth service.
pub mod v1 {
    #![allow(clippy::unit_arg, missing_docs)]
    hrpc::include_proto!("protocol.auth.v1");
}
pub use v1::*;

use self::next_step_request::form_fields::Field;

/// A response to an [`AuthStep`].
#[derive(Debug, Clone)]
pub enum AuthStepResponse {
    /// A choice selection.
    Choice(String),
    /// A form.
    Form(Vec<Field>),
    /// Initial request.
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
