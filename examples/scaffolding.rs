use std::{error::Error, fmt::Debug, io, io::{BufRead, Lines}, str::FromStr};

fn solution() {
    // Enter your code here
}

fn main() {
    let stdin = io::stdin();
    let mut reader = InputParser::new(stdin.lock());
    // Enter your parsing code here
    solution();
}

pub type InputParserResult<T> = Result<T, Box<dyn Error>>;

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

    pub fn next_vector<T>(&mut self) -> InputParserResult<Vec<T>>
    where
        T: FromStr,
        T::Err: Debug,
    {
        let num_values = self.next_value::<usize>()?;
        self.next_values(num_values)
    }
}
