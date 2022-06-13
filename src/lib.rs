extern crate proc_macro;
use proc_macro::TokenStream;

use quote::quote;
use rustfmt_wrapper::rustfmt;
use synoptic::languages::rust;
use synoptic::Token;
/// This macro turns rust code snippets into syntax highlighted html.
/// If you have the html feature enabled it will directly return a `yew::Html` type using a component called `RawHtml` in order to turn a string into html.
/// This is needed because yew does not yet support turning a string into html
/// In order for this to work, this component needs to be created as follows:
///
/// ```
/// use yew::{function_component, Html, Properties};
/// #[derive(Properties, PartialEq)]
/// pub struct Props {
///    pub html: String,
/// }

/// #[function_component(RawHtml)]
/// pub fn raw_html(props: &Props) -> Html {
///    let div = web_sys::window()
///        .unwrap()
///        .document()
///        .unwrap()
///        .create_element("div")
///        .unwrap();
///    div.set_inner_html(&props.html.clone());
///
///    Html::VRef(div.into())
/// }
/// ```
/// Without this feature, it will just return a string of html.
/// # Panics
///
/// Panics at compile time if code is invalid.
#[proc_macro]
pub fn generate_snippet(item: TokenStream) -> TokenStream {
    let code = rustfmt(item).unwrap();
    let mut result = String::from(
        r#"<div style="background: #181818; overflow:auto;width:auto;padding:.2em .6em;"><pre style="margin: 0; line-height: 125%">"#,
    );
    let rust = rust();
    let highlighting = rust.run(code.trim());
    for (c, row) in highlighting.iter().enumerate() {
        // Print line number (with padding)
        result.push_str(format!("{: >2} ", c).as_str());
        // For each token within each row
        for tok in row {
            // Handle the tokens
            match tok {
                // Handle the start token (start foreground colour)
                Token::Start(kind) => {
                    result.push_str("<span style=\"");
                    match kind.as_str() {
                        "keyword" => result.push_str("color: #ba8baf;\">"),
                        "struct" => result.push_str("color: #f7ca88;\">"),
                        "boolean" | "number" | "global" => result.push_str("color: #dc9656;\">"),
                        "operator" => result.push_str("color: #d8d8d8;\">"),
                        "comment" | "reference" => result.push_str("color: #585858;\">"),
                        "string" | "character" => result.push_str("color: #a1b56c;\">"),
                        "function" | "macro" => result.push_str("color: #7cafc2;\">"),
                        "regex" | "symbol" => result.push_str("color: #86c1b9;\">"),
                        token => println!("unknow token: {}", token),
                    }
                }

                // Handle a text token (print out the contents)
                Token::Text(txt) => result.push_str(txt),
                // Handle an end token (reset foreground colour)
                Token::End(_) => result.push_str("</span>"),
            }
        }
        result.push('\n');
    }
    result.push_str("</pre></div>");
    #[cfg(feature = "html")]
    return quote! {
    html! {
            <RawHtml html={#result} />
        }
    }
    .into();

    quote! {
        #result
    }
    .into()
}
