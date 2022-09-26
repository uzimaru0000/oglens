#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Prefix {
    pub name: String,
    pub key: String,
}

impl Prefix {
    pub fn value_parser(s: &str) -> Result<Self, String> {
        let mut values = s.split(':');

        let name = values.next().ok_or(String::from("invalid format"))?;
        let key = values.next().unwrap_or("property");

        Ok(Self {
            name: name.to_string(),
            key: key.to_string(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::Prefix;

    #[test]
    fn parse1() {
        let val = Prefix::value_parser("twitter:name");

        assert_eq!(
            val,
            Ok(Prefix {
                name: String::from("twitter"),
                key: String::from("name"),
            })
        )
    }

    #[test]
    fn parse2() {
        let val = Prefix::value_parser("twitter");

        assert_eq!(
            val,
            Ok(Prefix {
                name: String::from("twitter"),
                key: String::from("property"),
            })
        )
    }
}
