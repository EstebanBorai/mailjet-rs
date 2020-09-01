/// HTTP Status Codes defined by the Mailjet API.
///
/// Statuses documented in the official documentation are enumerated in the `StatusCode` `enum`.
/// The documentation provided for each `StatusCode` is taken from the documentation as is:
///
/// # Reference
///
/// https://dev.mailjet.com/email/reference/overview/errors/
///
pub enum StatusCode {
    /// All went well. Congrats!
    Ok,
    /// The `POST` request was successfully executed.
    Created,
    /// No content found or expected to return. Returned when a `DELETE` request was successful.
    NoContent,
    /// The `PUT` request didn't affect any record.
    NotModified,
    /// One or more parameters are missing or maybe misspelled (unknown resource or action).
    BadRequest,
    /// You have specified an incorrect API Key / API Secret Key pair.
    /// You may be unauthorized to access the API or your API key may be inactive.
    /// Visit API keys Management section to check your keys.
    Unauthorized,
    /// You are not authorized to access this resource.
    Forbidden,
    /// The resource with the specified ID you are trying to reach does not exist.
    NotFound,
    /// The method requested on the resource does not exist.
    MethodNotAllowed,
    /// Oops! You have reached the maximum number of calls allowed per minute by our API.
    /// Please review your integration to reduce the number of calls issued by your system.
    TooManyRequests,
    /// Ouch! Something went wrong on our side and we apologize! When such error occurs, it
    /// will contain an error identifier in its description (e.g. "ErrorIdentifier" : "D4DF574C-0C5F-45C7-BA52-7AA8E533C3DE"),
    /// which is crucial for us to track the problem and identify the root cause. Please contact our support team, providing the
    /// error identifier and we will do our best to help.
    InternalServerError,
}
