use dioxus::prelude::*;

/// Generic document viewer card — covers docx, pptx, sheet, pdf, katex, mermaid, monaco, etc.
#[component]
pub fn DocumentViewer(
    title: String,
    format: String,
    #[props(default)] description: String,
    #[props(default)] icon: String,
    #[props(default)] page_count: String,
    #[props(default)] file_size: String,
    children: Element,
) -> Element {
    let format_icon = if icon.is_empty() {
        match format.to_lowercase().as_str() {
            "docx" | "word" => "📄",
            "pptx" | "powerpoint" => "📊",
            "xlsx" | "sheet" | "csv" => "📋",
            "pdf" => "📕",
            "katex" | "latex" | "math" => "🔢",
            "mermaid" | "diagram" => "📐",
            "monaco" | "editor" | "code" => "💻",
            "markdown" | "md" => "📝",
            _ => "📎",
        }
    } else {
        // Leak to get 'static lifetime for use in rsx
        Box::leak(icon.clone().into_boxed_str()) as &str
    };

    rsx! {
        div { class: "qs-doc-card",
            div { class: "qs-doc-icon", "{format_icon}" }
            div { style: "flex: 1; min-width: 0;",
                div { style: "display: flex; align-items: center; gap: 8px; flex-wrap: wrap;",
                    h4 { style: "font-size: 14px; font-weight: 600; color: var(--qs-fg); margin: 0;", "{title}" }
                    span { class: "qs-badge qs-badge-muted", "{format}" }
                }
                if !description.is_empty() {
                    p { style: "font-size: 12px; color: var(--qs-fg-muted); margin: 4px 0 0;", "{description}" }
                }
                if !page_count.is_empty() || !file_size.is_empty() {
                    div { style: "display: flex; gap: 12px; margin-top: 6px;",
                        if !page_count.is_empty() {
                            span { style: "font-size: 10px; color: var(--qs-fg-subtle); font-family: var(--font-mono);", "{page_count} pages" }
                        }
                        if !file_size.is_empty() {
                            span { style: "font-size: 10px; color: var(--qs-fg-subtle); font-family: var(--font-mono);", "{file_size}" }
                        }
                    }
                }
            }
            {children}
        }
    }
}

/// Document viewer grid — multiple doc cards in a responsive grid.
#[component]
pub fn DocumentGrid(children: Element) -> Element {
    rsx! {
        div { style: "display: grid; grid-template-columns: repeat(auto-fill, minmax(320px, 1fr)); gap: 12px;",
            {children}
        }
    }
}

/// Document format collection — groups all supported format types for display.
#[component]
pub fn DocumentFormatShowcase() -> Element {
    let formats: &[(&str, &str, &str, &str)] = &[
        ("Word Document", "docx", "📄", "Microsoft Word .docx reader/writer"),
        ("PowerPoint", "pptx", "📊", "Presentation deck viewer"),
        ("Spreadsheet", "xlsx", "📋", "Excel/CSV data table viewer"),
        ("PDF Viewer", "pdf", "📕", "PDF.js document renderer"),
        ("PDF Generator", "pdfmake", "📄", "Declarative PDF creation engine"),
        ("KaTeX Math", "katex", "🔢", "LaTeX mathematical notation renderer"),
        ("Mermaid Diagrams", "mermaid", "📐", "Flowchart & sequence diagram engine"),
        ("Monaco Editor", "monaco", "💻", "VSCode-based code editor"),
        ("DocxTemplater", "docxtemplater", "📝", "Template-based document generation"),
        ("Markdown", "markdown", "📝", "GFM markdown with Shiki syntax highlighting"),
    ];

    rsx! {
        DocumentGrid {
            for (title, format, icon, desc) in formats.iter() {
                DocumentViewer {
                    title: title.to_string(),
                    format: format.to_string(),
                    icon: icon.to_string(),
                    description: desc.to_string(),
                    span {}
                }
            }
        }
    }
}
