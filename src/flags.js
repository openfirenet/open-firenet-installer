// Vector SVG flag icons for language selection across all operating systems.
// Windows' Segoe UI Emoji does not render regional indicator flag emojis.

export const LANG_LABELS = {
  fr: "Français",
  en: "English",
  de: "Deutsch",
  it: "Italiano",
};

export function flag(lang, idPrefix = "") {
  const p = idPrefix ? `${idPrefix}-` : "";
  switch (lang) {
    case "fr":
      return '<svg class="flag-icon" viewBox="0 0 3 2" width="20" height="14" aria-hidden="true"><rect width="1" height="2" fill="#002395"/><rect x="1" width="1" height="2" fill="#ffffff"/><rect x="2" width="1" height="2" fill="#ed2939"/></svg>';
    case "en":
      return `<svg class="flag-icon" viewBox="0 0 60 30" width="20" height="14" aria-hidden="true"><clipPath id="${p}gb-s"><path d="M0,0 v30 h60 v-30 z"/></clipPath><clipPath id="${p}gb-t"><path d="M30,15 h30 v15 z v15 h-30 z h-30 v-15 z v-15 h30 z"/></clipPath><g clip-path="url(#${p}gb-s)"><path d="M0,0 v30 h60 v-30 z" fill="#012169"/><path d="M0,0 L60,30 M60,0 L0,30" stroke="#fff" stroke-width="6"/><path d="M0,0 L60,30 M60,0 L0,30" clip-path="url(#${p}gb-t)" stroke="#c8102e" stroke-width="4"/><path d="M30,0 v30 M0,15 h60" stroke="#fff" stroke-width="10"/><path d="M30,0 v30 M0,15 h60" stroke="#c8102e" stroke-width="6"/></g></svg>`;
    case "de":
      return '<svg class="flag-icon" viewBox="0 0 5 3" width="20" height="14" aria-hidden="true"><rect width="5" height="1" fill="#000000"/><rect y="1" width="5" height="1" fill="#dd0000"/><rect y="2" width="5" height="1" fill="#ffce00"/></svg>';
    case "it":
      return '<svg class="flag-icon" viewBox="0 0 3 2" width="20" height="14" aria-hidden="true"><rect width="1" height="2" fill="#009246"/><rect x="1" width="1" height="2" fill="#ffffff"/><rect x="2" width="1" height="2" fill="#ce2b37"/></svg>';
    default:
      return "";
  }
}
