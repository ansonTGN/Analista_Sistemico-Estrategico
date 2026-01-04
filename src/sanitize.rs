use ammonia::Builder;
use pulldown_cmark::{html, Options, Parser};

/// Sanitiza HTML (p.ej. resultado de Markdown -> HTML) de forma segura.
/// Corrección del panic:
/// - NO permitir el atributo "rel" en allowlists (generic/tag attributes)
/// - Mantener link_rel(Some(...)) para que Ammonia lo inyecte sin colisión
pub fn sanitize_html(input_html: &str) -> String {
    let mut b = Builder::default();

    // Tags razonables para contenido enriquecido (ajusta según tu caso)
    b.add_tags([
        "p", "br", "hr",
        "em", "strong", "s", "u",
        "blockquote",
        "ul", "ol", "li",
        "code", "pre",
        "h1", "h2", "h3", "h4", "h5", "h6",
        "table", "thead", "tbody", "tr", "th", "td",
        "a",
    ]);

    // Atributos permitidos por tag.
    // Importante: NO incluyas "rel" aquí.
    b.add_tag_attributes("a", ["href", "title", "target"].into_iter());
    b.add_tag_attributes("th", ["colspan", "rowspan"].into_iter());
    b.add_tag_attributes("td", ["colspan", "rowspan"].into_iter());

    // Asegura rel seguro para links (y evita tabnabbing)
    // Si permitieras "rel" en allowlist, esto debe ser None.
    b.link_rel(Some("noopener noreferrer nofollow"));

    // Opcional: limitar esquemas permitidos
    b.url_schemes(["http", "https", "mailto"].into_iter());

    b.clean(input_html).to_string()
}

/// Convierte Markdown a HTML y lo sanitiza.
pub fn markdown_to_safe_html(md: &str) -> String {
    let mut options = Options::empty();
    options.insert(Options::ENABLE_STRIKETHROUGH);
    options.insert(Options::ENABLE_TABLES);
    options.insert(Options::ENABLE_FOOTNOTES);
    options.insert(Options::ENABLE_TASKLISTS);

    let parser = Parser::new_ext(md, options);

    let mut raw_html = String::new();
    html::push_html(&mut raw_html, parser);

    sanitize_html(&raw_html)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn does_not_panic_with_rel_policy() {
        let html = r#"<a href="https://example.com" rel="ugc noopener">x</a>"#;
        // Sanitiza sin panicar y aplica política rel segura
        let out = sanitize_html(html);
        assert!(out.contains("href=\"https://example.com\""));
        // Ammonia inyecta rel según link_rel(Some(...)) y no colisiona
        assert!(out.contains("rel="));
    }

    #[test]
    fn markdown_roundtrip_safe() {
        let md = r#"[link](https://example.com "t")"#;
        let out = markdown_to_safe_html(md);
        assert!(out.contains("href=\"https://example.com\""));
        assert!(out.contains("rel="));
    }
}
