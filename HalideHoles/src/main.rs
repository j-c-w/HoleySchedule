mod parser;
mod options;

fn main() {
    let options = options::options::parse_options();

    let sketch = parser::parser::parse(&options, &options.source);
}
