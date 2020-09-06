/// Every `struct` that is sent through the Mailjet's SendAPI must
/// implement `Payload`
///
/// This `trait` ensures that the `struct` is capable of being serialized
/// into a JSON object.
pub trait Payload {
    fn to_json(&self) -> String;
}
