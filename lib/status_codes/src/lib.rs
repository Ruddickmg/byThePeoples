pub type StatusCode = u16;

pub const OKAY: StatusCode = 200;
pub const GONE: StatusCode = 410;
pub const CREATED: StatusCode = 201;
pub const ACCEPTED: StatusCode = 202;
pub const BAD_REQUEST: StatusCode = 400;
pub const UNAUTHORIZED: StatusCode = 401;
pub const FORBIDDEN: StatusCode = 403;
pub const NOT_FOUND: StatusCode = 404;
pub const CONFLICT: StatusCode = 409;
pub const UNPROCESSABLE_ENTITY: StatusCode = 422;
pub const INTERNAL_SERVER_ERROR: StatusCode = 500;
