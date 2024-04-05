#[allow(warnings)]
mod bindings;

//use bindings::Guest;
use bindings::exports::wilsonzlin::minify_html::minify_html::{Guest, Config};

struct Component;

impl Guest for Component {
    /// Minify HTML string.
    fn minify_html(src: String, cfg: Option<Config>) -> String {
        let cfg = cfg.map(|cfg| cfg.to_cfg()).unwrap_or_default();
        let minified = minify_html::minify(src.as_bytes(), &cfg);
        unsafe { String::from_utf8_unchecked(minified) }
    }

    /// Minify HTML UTF-8 encoded string as bytes.
    fn minify_html_bytes(src: Vec<u8>, cfg: Option<Config>) -> Vec<u8> {
        let cfg = cfg.map(|cfg| cfg.to_cfg()).unwrap_or_default();
        minify_html::minify(&src, &cfg)
    }
}

impl Config {
    fn to_cfg(self) -> minify_html::Cfg {
        minify_html::Cfg{
            allow_noncompliant_unquoted_attribute_values: self.allow_noncompliant_unquoted_attribute_values,
            allow_optimal_entities: self.allow_optimal_entities,
            allow_removing_spaces_between_attributes: self.allow_removing_spaces_between_attributes,
            keep_closing_tags: self.keep_closing_tags,
            keep_comments: self.keep_comments,
            keep_html_and_head_opening_tags: self.keep_html_and_head_opening_tags,
            keep_input_type_text_attr: self.keep_input_type_text_attr,
            keep_ssi_comments: self.keep_ssi_comments,
            minify_css: self.minify_css,
            minify_doctype: self.minify_doctype,
            minify_js: self.minify_js,
            preserve_brace_template_syntax: self.preserve_brace_template_syntax,
            preserve_chevron_percent_template_syntax: self.preserve_chevron_percent_template_syntax,
            remove_bangs: self.remove_bangs,
            remove_processing_instructions: self.remove_processing_instructions,
        }
    }
}

bindings::export!(Component with_types_in bindings);
