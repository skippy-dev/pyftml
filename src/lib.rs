#[macro_use]
extern crate slog;

use ftml::prelude::*;
use ftml::info::VERSION;
use ftml::render::html::HtmlRender;
use ftml::render::text::TextRender;
use ftml::render::Render;

use pyo3::prelude::*;

use std::collections::HashMap;
use std::borrow::Cow::Borrowed;


fn render<R: Render>(
    text: &mut String,
    renderer: &R) -> R::Output
{
    let drain = slog::Discard;
    let log = slog::Logger::root(drain, o!());

    // TODO includer

    crate::preprocess(&log, text);
    let tokens = crate::tokenize(&log, &text);
    let (tree, _warnings) = crate::parse(&log, &tokens).into();
    let output = renderer.render(&log, &page_info_dummy(), &tree);
    output
}


fn page_info_dummy() -> PageInfo<'static>
{
    PageInfo {
            page: Borrowed("some-page"),
            category: None,
            site: Borrowed("sandbox"),
            title: Borrowed("title"),
            alt_title: None,
            rating: 0.0,
            tags: vec![],
            locale: Borrowed("")
        }
}


#[pyfunction]
fn render_html(
    source: &str) -> PyResult<HashMap<String, String>>
{
    let html_output = render(&mut source.to_string(), &HtmlRender);

    let mut output = HashMap::new();
    output.insert(String::from("body"), html_output.html);
    output.insert(String::from("style"), html_output.style);

    Ok(output)
}


#[pyfunction]
fn render_text(
    source: &str) -> PyResult<String> 
{
    Ok(render(&mut source.to_string(), &TextRender))
}


/// A Python module implemented in Rust.
#[pymodule]
fn pyftml(
    _py: Python, 
    m: &PyModule) -> PyResult<()> {
    m.add("ftml_version", VERSION.to_string())?;
    m.add_function(wrap_pyfunction!(render_html, m)?)?;
    m.add_function(wrap_pyfunction!(render_text, m)?)?;

    Ok(())
}