use serde::{Deserialize, Serialize};

/// Email recipient composed by an email address and
/// the name of the owner
#[derive(Debug, Serialize, Deserialize)]
pub struct Recipient {
    #[serde(rename = "Email")]
    pub email: String,
    #[serde(rename = "Name")]
    pub name: String,
}

impl Recipient {
    /// Creates a new `Recipient` instance with no name
    pub fn new(email: &str) -> Self {
        Self {
            email: String::from(email),
            name: String::default(),
        }
    }

    /// Creates a new `Recipient` instance with an `email` and
    /// a `name`
    pub fn with_name(email: &str, name: &str) -> Self {
        Self {
            email: String::from(email),
            name: String::from(name),
        }
    }

    pub fn from_comma_separated(recipients: &str) -> Vec<Self> {
        let as_string_vec = recipients.split(",");

        as_string_vec
            .into_iter()
            .map(|r| Recipient::new(r))
            .collect::<Vec<Recipient>>()
    }

    pub fn as_comma_separated(&self) -> String {
        // "John Doe" &lt;john@example.com&lt;
        let mut string = String::default();

        if self.name != String::default() {
            string += &format!("\"{}\"", self.name);
            string += " ";
        }

        string += &format!("&lt;{}&gt;", self.email);

        string
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn creates_recipient_from_comma_separated() {
        let have = "foo@bar.com,rust@rust-lang.org,hyper_rs.alpha@gmail.com";
        let want = vec![
            Recipient::new("foo@bar.com"),
            Recipient::new("rust@rust-lang.org"),
            Recipient::new("hyper_rs.alpha@gmail.com"),
        ];

        for (index, recipient) in have.split(',').enumerate().into_iter() {
            assert_eq!(recipient.to_string(), want.get(index).unwrap().email);
        }
    }

    #[test]
    fn creates_comma_separated_from_recipient() {
        let have = vec![
            Recipient::with_name("rust@rust-lang.org", "The Rust Programming Language"),
            Recipient::new("foo@bar.com"),
        ];
        let want = vec![
            String::from("\"The Rust Programming Language\" &lt;rust@rust-lang.org&gt;"),
            String::from("&lt;foo@bar.com&gt;"),
        ];

        have.into_iter().enumerate().for_each(|(index, value)| {
            assert_eq!(
                value.as_comma_separated().as_str(),
                want.get(index).unwrap()
            );
        })
    }
}
