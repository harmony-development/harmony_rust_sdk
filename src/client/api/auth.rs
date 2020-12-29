use super::*;

use crate::{api::auth::*, client_api};

pub use crate::api::auth::{auth_step, next_step_request, AuthStep, Session};

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
