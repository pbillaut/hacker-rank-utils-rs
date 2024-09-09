use std::{
    error::Error,
    fmt::Debug,
    io::{BufRead, Lines},
    str::FromStr,
};

pub type InputParserResult<T> = Result<T, Box<dyn Error>>;

/// Parser with convenience functions for parsing input in common HackerRank formats.
pub struct InputParser<R>
where
    R: BufRead,
{
    lines: Lines<R>,
}

impl<R> InputParser<R>
where
    R: BufRead,
{
    pub fn new(reader: R) -> Self {
        Self { lines: reader.lines() }
    }

    pub fn lines(&mut self) -> &mut Lines<R> {
        &mut self.lines
    }

    /// Parses a line into a value of type `T`.
    pub fn next_value<T>(&mut self) -> InputParserResult<T>
    where
        T: FromStr,
        T::Err: Debug,
    {
        self.lines
            .next()
            .ok_or("Unable to parse next value: iterator is empty")?
            .map(|line| line.trim().parse())?
            .map_err(|err| format!("Unable to parse next value: {:?}", err).into())
    }

    /// Parses `num_values` lines of input into a `Vec<T>`.
    ///
    /// # Example Input
    /// ```ignore
    /// 2
    /// a
    /// b
    /// ```
    pub fn next_values<T>(&mut self, num_values: usize) -> InputParserResult<Vec<T>>
    where
        T: FromStr,
        T::Err: Debug,
    {
        let mut list: Vec<T> = Vec::with_capacity(num_values);
        for _ in 0..num_values {
            let value = self.next_value()?;
            list.push(value);
        }
        Ok(list)
    }

    /// Parses multiple lines of input into a `Vec<T>`. Expects the first line to denote the number
    /// of values to parse.
    ///
    /// # Example Input
    /// ```ignore
    /// 2
    /// a
    /// b
    /// ```
    pub fn next_vector<T>(&mut self) -> InputParserResult<Vec<T>>
    where
        T: FromStr,
        T::Err: Debug,
    {
        let num_values = self.next_value::<usize>()?;
        self.next_values(num_values)
    }
}

#[cfg(test)]
mod tests {
    use super::InputParser;
    use std::io;

    fn raw_input(lines: &[&str]) -> io::Cursor<String> {
        io::Cursor::new(lines.join("\n"))
    }

    #[test]
    fn parse_next_value() {
        let lines = &["1"];
        let expected = 1;

        let input = raw_input(lines);
        let mut reader = InputParser::new(input);

        let maybe_value = reader.next_value::<usize>();
        assert!(maybe_value.is_ok());
        assert_eq!(maybe_value.unwrap(), expected);
    }

    #[test]
    fn parse_next_values() {
        let lines = &["1", "2", "3", "4"];
        let expected = vec![1, 2, 3, 4];

        let input = raw_input(lines);
        let mut reader = InputParser::new(input);

        let maybe_values = reader.next_values::<usize>(lines.len());
        assert!(maybe_values.is_ok());
        assert_eq!(maybe_values.unwrap(), expected);
    }

    #[test]
    fn parse_next_vector() {
        let lines = &["4", "1", "2", "3", "4"];
        let expected = vec![1, 2, 3, 4];

        let input = raw_input(lines);
        let mut reader = InputParser::new(input);

        let maybe_values = reader.next_vector::<usize>();
        assert!(maybe_values.is_ok());
        assert_eq!(maybe_values.unwrap(), expected);
    }
}
