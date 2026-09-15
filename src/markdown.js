/**
 * Lightweight and safe Markdown to HTML renderer for GitHub release notes.
 * Converts headings, bold, italic, inline code, lists, and links into styled HTML.
 */
export function renderMarkdown(text) {
  if (!text) return "";

  // 1. Échapper les entités HTML pour la sécurité
  let raw = text
    .replace(/&/g, "&amp;")
    .replace(/</g, "&lt;")
    .replace(/>/g, "&gt;");

  // 2. Traitement ligne par ligne pour les titres et les listes à puces
  const lines = raw.split(/\r?\n/);
  const output = [];
  let inList = false;

  for (let line of lines) {
    const trimmed = line.trim();

    // Titres Markdown
    if (trimmed.startsWith("### ")) {
      if (inList) { output.push("</ul>"); inList = false; }
      output.push(`<h5 class="release-h5">${formatInline(trimmed.substring(4))}</h5>`);
      continue;
    }
    if (trimmed.startsWith("## ")) {
      if (inList) { output.push("</ul>"); inList = false; }
      output.push(`<h4 class="release-h4">${formatInline(trimmed.substring(3))}</h4>`);
      continue;
    }
    if (trimmed.startsWith("# ")) {
      if (inList) { output.push("</ul>"); inList = false; }
      output.push(`<h3 class="release-h3">${formatInline(trimmed.substring(2))}</h3>`);
      continue;
    }

    // Listes à puces (* item ou - item)
    if (trimmed.startsWith("* ") || trimmed.startsWith("- ")) {
      if (!inList) {
        output.push('<ul class="release-list">');
        inList = true;
      }
      output.push(`<li>${formatInline(trimmed.substring(2))}</li>`);
      continue;
    }

    // Fin de liste si ligne normale
    if (inList) {
      output.push("</ul>");
      inList = false;
    }

    // Lignes de texte normales
    if (trimmed.length > 0) {
      output.push(`<p class="release-p">${formatInline(line)}</p>`);
    }
  }

  if (inList) {
    output.push("</ul>");
  }

  return output.join("\n");
}

function formatInline(str) {
  // Gras & Italique
  let s = str.replace(/\*\*\*(.*?)\*\*\*/g, "<strong><em>$1</em></strong>");
  s = s.replace(/\*\*(.*?)\*\*/g, "<strong>$1</strong>");
  s = s.replace(/\*(.*?)\*/g, "<em>$1</em>");

  // Code en ligne
  s = s.replace(/`([^`]+)`/g, '<code class="release-code">$1</code>');

  // Liens Markdown [libellé](url)
  s = s.replace(/\[([^\]]+)\]\((https?:\/\/[^\s)]+)\)/g, '<a href="$2" class="external-link" target="_blank" rel="noopener">$1 <span class="ext-icon">↗</span></a>');

  // URLs brutes https://... non incluses dans un <a>
  s = s.replace(/(^|[\s(])(https?:\/\/[^\s<)]+)/g, '$1<a href="$2" class="external-link" target="_blank" rel="noopener">$2 <span class="ext-icon">↗</span></a>');

  // Mentions d'utilisateurs @username
  s = s.replace(/@([a-zA-Z0-9_-]+)/g, '<a href="https://github.com/$1" class="external-link mention-link" target="_blank" rel="noopener">@$1</a>');

  return s;
}
