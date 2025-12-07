pub struct Tokenizer<'a> {
    input: &'a [char]
}

impl<'a> Tokenizer<'a> {
    pub fn from_chars(input: &'a [char]) -> Self {
        Self {
            input
        }
    }

    fn trim_leading_whitespace(&mut self) {
        while !self.input.is_empty() && self.input[0].is_whitespace() {
            self.input = &self.input[1..];
        }
    }

    fn chop_while<F>(&mut self, mut input_function: F) -> &'a [char] where F: FnMut(&char) -> bool {
        let mut n = 0;
        while self.input.len() > n && input_function(&self.input[n]) {
            n+=1;
        }
        let output = &self.input[0..n];
        self.input = &self.input[n..];
        output
    }
}

impl<'a> Iterator for Tokenizer<'a> {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        self.trim_leading_whitespace();

        if self.input.is_empty() {
            return None;
        }

        if self.input[0].is_numeric() {
            return Some(self.chop_while(|x| x.is_numeric()).iter().collect());
        }

        if self.input[0].is_alphabetic() {
            return Some(self.chop_while(|x| x.is_alphabetic()).iter().collect());
        }

        let output = &self.input[0..1];
        self.input = &self.input[1..];

        Some(output.iter().collect())
    }
}

#[cfg(test)]
mod tests {
    use crate::tokenizer::Tokenizer;

    #[test]
    fn tokenize_three_plain_words() {
        let input: Vec<char> = "good morning everyone".chars().collect();
        let expected = [
            "good",
            "morning",
            "everyone"
        ];

        let tokenizer = Tokenizer::from_chars(&input);

        for (index, token) in tokenizer.enumerate() {
            assert_eq!(expected.get(index).unwrap().to_owned(), token)
        }
    }


    #[test]
    fn trim_leading() {
        let input = [' ', ' ', 'b', 'c'];

        let mut tokenizer = Tokenizer::from_chars(&input);

        tokenizer.trim_leading_whitespace();

        assert_eq!(tokenizer.input, ['b', 'c']);
    }

    #[test]
    fn tokenizer_returns_none() {

        let input: Vec<char> = "          ".chars().collect();

        let mut tokenizer = Tokenizer::from_chars(&input);

        assert_eq!(None, tokenizer.next())
    }
}
