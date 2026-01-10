// src/sanitize.rs

use ammonia::Builder as AmmoniaBuilder;
use std::collections::HashSet;

pub fn ensure_article(html: String) -> String {
    let low = html.to_lowercase();
    let has_open = low.contains("<article");
    let has_close = low.contains("</article>");
    if has_open && has_close {
        html
    } else {
        format!("<article>{}</article>", html.trim())
    }
}

pub fn sanitize_ai_html(html: &str) -> String {
    // Tags necesarios para el layout del informe y contenido estructurado.
    let tags: HashSet<&str> = [
        "article", "section", "div", "h2", "h3", "h4", "p", "span", "strong", "em", "ul", "ol",
        "li", "br", "hr", "blockquote", "pre", "code", "table", "thead", "tbody", "tr", "th", "td",
        "a",
    ]
    .into_iter()
    .collect();

    let mut builder = AmmoniaBuilder::default();
    builder.tags(tags);

    // Clases permitidas (para tu CSS del reporte).
    // Nota: ammonia aplica whitelist por atributo; aquí permitimos "class" globalmente.
    builder.add_generic_attributes(["class"].into_iter());

    // Atributos para links
    builder.add_tag_attributes("a", ["href", "title", "target"].into_iter());

    // Tablas: soporte de spans.
    builder.add_tag_attributes("th", ["colspan", "rowspan"].into_iter());
    builder.add_tag_attributes("td", ["colspan", "rowspan"].into_iter());

    // Política segura para enlaces (tabnabbing + SEO básico)
    builder.link_rel(Some("noopener noreferrer nofollow"));

    // Schemes permitidos (ammonia 4.x exige HashSet)
    let schemes: std::collections::HashSet<&'static str> =
        ["http", "https", "mailto"].into_iter().collect();
    builder.url_schemes(schemes);

    builder.clean(html).to_string()
}

