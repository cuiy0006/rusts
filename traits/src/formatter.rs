pub trait Formatter {
    fn format(&self, input: &mut String) -> bool;
}

struct MarkdownFormatter;
impl Formatter for MarkdownFormatter {
    fn format(&self, input: &mut String) -> bool {
        input.push_str("\n formatted with Markdown formatter");
        true
    }
}

struct RustFormatter;
impl Formatter for RustFormatter {
    fn format(&self, input: &mut String) -> bool {
        input.push_str("\n formatted with Rust formatter");
        true
    }
}

struct HtmlFormatter;
impl Formatter for HtmlFormatter {
    fn format(&self, input: &mut String) -> bool {
        input.push_str("\n formatted with Html formatter");
        true
    }
}

pub fn format(input: &mut String, formatters: Vec<&dyn Formatter>) {
    for formatter in formatters {
        formatter.format(input);
    }
}

#[test]
fn formatter_should_work() {
    let formatter1: &dyn Formatter = &RustFormatter;
    let formatter2: &dyn Formatter = &HtmlFormatter;
    let formatters = vec![formatter1, formatter2];
    let mut input = "hello world".to_string();
    format(&mut input, formatters);
    assert_eq!(input, "hello world\n formatted with Rust formatter\n formatted with Html formatter");
}