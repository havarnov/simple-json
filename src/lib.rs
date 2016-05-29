use std::collections::HashMap;

#[derive(Debug, Eq, PartialEq)]
pub enum Json {
    Object(HashMap<String, Json>),
    Array(Vec<Json>),
    String(String),
    Number(usize),
    Boolean(bool),
    Null
}

#[derive(Debug, Eq, PartialEq)]
pub enum JsonError {
    ParseError(String),
    NotImplemented,
}

impl Json {
    pub fn from_str(input: &str) -> Result<Json, JsonError> {
        let builder = JsonBuilder::new(input.chars());
        builder.build()
    }
}

struct JsonBuilder<T> {
    iter: T,
    token: Option<char>,
    column: usize,
    line: usize
}

impl<T: Iterator<Item = char>> JsonBuilder<T> {
    fn new(iter: T) -> JsonBuilder<T> {
        JsonBuilder {
            iter: iter,
            token: None,
            column: 0,
            line: 0
        }
    }

    fn build(mut self) -> Result<Json, JsonError> {
        self.next();
        self.parse()
    }

    fn next(&mut self) -> Option<char> {
        self.token = self.iter.next();
        self.token
    }

    fn parse(&mut self) -> Result<Json, JsonError> {
        println!("{:?}", self.token);
        match self.token {
            Some('n') => self.parse_ident("ull", Json::Null),
            Some('t') => self.parse_ident("rue", Json::Boolean(true)),
            Some('f') => self.parse_ident("alse", Json::Boolean(false)),
            Some('"') => self.parse_string(),
            Some('[') => self.parse_list(),
            Some('{') => self.parse_object(),
            Some(' ') => {
                self.column += 1;
                self.next();
                self.parse()
            }
            Some(_) => Err(JsonError::ParseError(format!("unexpected character: {:?}", self.token))),
            None => Err(JsonError::NotImplemented)
        }
    }

    fn parse_ident(&mut self, ident: &str, res: Json) -> Result<Json, JsonError> {
        if ident.chars().all(|c| Some(c) == self.next()) {
            Ok(res)
        } else {
            Err(JsonError::ParseError("woot".to_string()))
        }
    }

    fn parse_string(&mut self) -> Result<Json, JsonError> {
        Err(JsonError::NotImplemented)
    }

    fn parse_list(&mut self) -> Result<Json, JsonError> {
        let mut list = Vec::new();
        loop {
            self.next();
            println!("{:?}", self.token);
            match self.token {
                Some(']') => break,
                Some(',') => {
                    self.column += 1;
                },
                Some(_) => list.push(try!(self.parse())),
                _ => return Err(JsonError::NotImplemented)
            };
        }

        Ok(Json::Array(list))
    }

    fn parse_object(&mut self) -> Result<Json, JsonError> {
        let mut map = HashMap::new();

        loop {
            self.next();
            match self.token {
                Some('}') => break,
                Some('"') => {
                    let (key, value) = match self.parse_object_key_value() {
                        Ok((key, value)) => (key, value),
                        Err(err) => return Err(err)
                    };
                    map.insert(key, value);
                },
                Some(',') => {
                    self.column += 1;
                },
                Some(' ') => {
                    self.column += 1;
                },
                _ => return Err(JsonError::NotImplemented)
            };
        }

        Ok(Json::Object(map))
    }

    fn parse_object_key_value(&mut self) -> Result<(String, Json), JsonError> {
        let key = match self.parse_object_key() {
            Ok(key) => key,
            Err(err) => return Err(err)
        };

        self.next();
        let value = match self.parse() {
            Ok(value) => value,
            Err(err) => return Err(err)
        };

        Ok((key, value))
    }

    fn parse_object_key(&mut self) -> Result<String, JsonError> {
        let key = self.iter.by_ref().take_while(|c| *c != '"').collect();

        if self.next() != Some(':') {
            Err(JsonError::ParseError(format!("Expected to find ':', but found: {:?}", self.token)))
        } else {
            Ok(key)
        }
    }
}

#[cfg(test)]
mod test {
    use std::collections::HashMap;

    use super::*;

    #[test]
    fn parse_null() {

        // act
        let res = Json::from_str("null").unwrap();

        // assert
        assert_eq!(res, Json::Null);
    }

    #[test]
    fn parse_bool_is_false() {

        // act
        let res = Json::from_str("false").unwrap();

        // assert
        assert_eq!(res, Json::Boolean(false));
    }

    #[test]
    fn parse_bool_is_true() {

        // act
        let res = Json::from_str("true").unwrap();

        // assert
        assert_eq!(res, Json::Boolean(true));
    }

    #[test]
    fn parse_string() {

        // act
        let res = Json::from_str("\"foo\"").unwrap();

        // assert
        assert_eq!(res, Json::String("foo".to_string()));
    }

    #[test]
    fn parse_string_with_escaped_quote() {

        // act
        let res = Json::from_str("\"fo\\\"o\"").unwrap();

        // assert
        assert_eq!(res, Json::String("fo\"o".to_string()));
    }

    #[test]
    fn parse_unclosed_string_should_return_parseerror() {

        // act
        let res = Json::from_str("\"foo");

        // assert
        assert_eq!(res, Err(JsonError::ParseError("Parsing error: unclosed string.".to_string())));
    }

    #[test]
    fn parse_empty_list() {

        // act
        let res = Json::from_str("[]").unwrap();

        // assert
        assert_eq!(res, Json::Array(Vec::new()));
    }

    #[test]
    fn parse_list_one_item() {

        // act
        let res = Json::from_str("[null]").unwrap();

        // assert
        assert_eq!(res, Json::Array(vec![Json::Null]));
    }

    #[test]
    fn parse_list_two_item() {

        // act
        let res = Json::from_str("[null, false]").unwrap();

        // assert
        assert_eq!(res, Json::Array(vec![Json::Null, Json::Boolean(false)]));
    }

    #[test]
    fn parse_nested_list() {

        // act
        let res = Json::from_str("[[null, false], [null, true]]").unwrap();

        // assert
        assert_eq!(res, Json::Array(vec![
                Json::Array(vec![Json::Null, Json::Boolean(false)]),
                Json::Array(vec![Json::Null, Json::Boolean(true)])
            ]));
    }

    #[test]
    fn parse_empty_object() {

        // act
        let res = Json::from_str("{}").unwrap();

        // assert
        assert_eq!(res, Json::Object(HashMap::new()));
    }

    #[test]
    fn parse_object_one_item() {
        // arrange
        let mut map = HashMap::new();
        map.insert("something".to_string(), Json::Null);

        // act
        let res = Json::from_str("{\"something\":null}").unwrap();

        // assert
        assert_eq!(res, Json::Object(map));
    }

    #[test]
    fn parse_object_two_item() {
        // arrange
        let mut map = HashMap::new();
        map.insert("something".to_string(), Json::Null);
        map.insert("something_else".to_string(), Json::Boolean(true));

        // act
        let res = Json::from_str("{\"something\":null,\"something_else\":true}").unwrap();

        // assert
        assert_eq!(res, Json::Object(map));
    }

    #[test]
    fn parse_nested_objects() {
        // arrange
        let mut map = HashMap::new();
        map.insert("1".to_string(), Json::Object(HashMap::new()));
        map.insert("2".to_string(), Json::Object(HashMap::new()));

        // act
        let res = Json::from_str("{\"1\":{}, \"2\":{}}").unwrap();

        // assert
        assert_eq!(res, Json::Object(map));
    }

}
