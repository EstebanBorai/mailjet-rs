/// Every `struct` that is sent through the Mailjet's SendAPI must
/// implement `Payload`
///
/// This `trait` ensures that the `struct` is capable of being serialized
/// into a JSON object which is supported by the Mailjet API
pub trait Payload {
    /// Creates the JSON representation of `self` consumed by Mailjet's API
    fn to_json(&self) -> String;
}
