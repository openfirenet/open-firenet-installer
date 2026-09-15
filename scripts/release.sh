#!/usr/bin/env bash
# ==============================================================================
# Open-Firenet Installer Release Script
#
# Automatise le processus de release avec garde-fous stricts :
# 1. Vérification de la propreté du git tree
# 2. Vérification de la branche (main) et synchronisation remote
# 3. Exécution obligatoire des tests unitaires Rust & build frontend
# 4. Calcul automatique SemVer (patch / minor / major)
# 5. Création et push du tag annoté pour déclencher la CI Release
# ==============================================================================

set -eo pipefail

RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
CYAN='\033[0;36m'
BOLD='\033[1m'
NC='\033[0m' # No Color

info()  { echo -e "${BLUE}ℹ${NC} $*"; }
ok()    { echo -e "${GREEN}✔${NC} $*"; }
warn()  { echo -e "${YELLOW}⚠${NC} $*"; }
err()   { echo -e "${RED}✖${NC} $*" >&2; }
fatal() { err "$*"; exit 1; }

# Trouver la racine du dépôt git
REPO_ROOT="$(git rev-parse --show-toplevel 2>/dev/null || true)"
if [[ -z "$REPO_ROOT" ]]; then
  fatal "Ce script doit être exécuté dans un dépôt Git."
fi
cd "$REPO_ROOT"

echo -e "\n${BOLD}${CYAN}=== Open-Firenet Installer Release Assistant ===${NC}\n"

# 1. Vérification de la propreté de l'arbre de travail
info "Vérification de l'état de l'arbre de travail Git..."
if [[ -n "$(git status --porcelain)" ]]; then
  fatal "L'arbre de travail n'est pas propre. Veuillez commiter ou remiser vos modifications avant de releaser."
fi
ok "Arbre de travail propre."

# 2. Vérification de la branche
CURRENT_BRANCH="$(git branch --show-current)"
info "Branche actuelle : ${BOLD}${CURRENT_BRANCH}${NC}"
if [[ "$CURRENT_BRANCH" != "main" ]]; then
  warn "Vous n'êtes pas sur la branche 'main' (branche actuelle: $CURRENT_BRANCH)."
  read -rp "Voulez-vous vraiment créer une release depuis '$CURRENT_BRANCH' ? [o/N] " confirm_branch
  if [[ ! "$confirm_branch" =~ ^[oOyY]$ ]]; then
    fatal "Release annulée. Basculez sur 'main' avec : git checkout main"
  fi
fi

# 3. Synchronisation avec origin
info "Vérification de la synchronisation avec origin..."
git fetch origin "$CURRENT_BRANCH" --quiet || true
LOCAL_COMMIT="$(git rev-parse HEAD)"
REMOTE_COMMIT="$(git rev-parse "origin/$CURRENT_BRANCH" 2>/dev/null || true)"

if [[ -n "$REMOTE_COMMIT" && "$LOCAL_COMMIT" != "$REMOTE_COMMIT" ]]; then
  BEHIND="$(git rev-list --count HEAD..origin/"$CURRENT_BRANCH" 2>/dev/null || echo 0)"
  if [[ "$BEHIND" -gt 0 ]]; then
    fatal "Votre branche locale a $BEHIND commit(s) de retard par rapport à origin. Faites un 'git pull' d'abord."
  fi
fi
ok "Synchronisation remote vérifiée."

# 4. Exécution obligatoire des tests et build frontend
info "Validation du build frontend..."
npm run build >/dev/null 2>&1 || fatal "Le build frontend (npm run build) a échoué !"
ok "Frontend build validé."

info "Exécution des tests unitaires Rust..."
cargo test >/dev/null 2>&1 || fatal "Les tests unitaires Rust ont échoué !"
ok "Tests unitaires validés."

# 5. Détection du dernier tag et calcul SemVer
LATEST_TAG="$(git describe --tags --abbrev=0 2>/dev/null || true)"

if [[ -z "$LATEST_TAG" ]]; then
  warn "Aucun tag existant trouvé dans ce dépôt."
  DEFAULT_NEXT="v0.1.0"
  LATEST_TAG="aucun"
else
  info "Dernier tag détecté : ${BOLD}${LATEST_TAG}${NC}"
  RAW_VER="${LATEST_TAG#v}"
  IFS='.' read -r MAJOR MINOR PATCH <<< "$RAW_VER"
  
  NEXT_PATCH="v${MAJOR}.${MINOR}.$((PATCH + 1))"
  NEXT_MINOR="v${MAJOR}.$((MINOR + 1)).0"
  NEXT_MAJOR="v$((MAJOR + 1)).0.0"
  DEFAULT_NEXT="$NEXT_PATCH"
fi

TARGET_VERSION=""
ARG_INPUT="${1:-}"

if [[ -n "$ARG_INPUT" ]]; then
  case "$ARG_INPUT" in
    patch) TARGET_VERSION="$NEXT_PATCH" ;;
    minor) TARGET_VERSION="$NEXT_MINOR" ;;
    major) TARGET_VERSION="$NEXT_MAJOR" ;;
    v*.*.*) TARGET_VERSION="$ARG_INPUT" ;;
    *.*.*)  TARGET_VERSION="v$ARG_INPUT" ;;
    *) fatal "Argument invalide '$ARG_INPUT'. Utilisez: patch, minor, major ou vX.Y.Z" ;;
  esac
else
  if [[ "$LATEST_TAG" == "aucun" ]]; then
    read -rp "Entrez la version initiale à releaser [défaut: $DEFAULT_NEXT]: " user_ver
    TARGET_VERSION="${user_ver:-$DEFAULT_NEXT}"
  else
    echo -e "\nOptions de version disponibles :"
    echo "  1) Patch : ${BOLD}${NEXT_PATCH}${NC}"
    echo "  2) Minor : ${BOLD}${NEXT_MINOR}${NC}"
    echo "  3) Major : ${BOLD}${NEXT_MAJOR}${NC}"
    echo "  4) Personnalisée"
    read -rp "Votre choix [1/2/3/4, défaut: 1] : " choice
    case "$choice" in
      2) TARGET_VERSION="$NEXT_MINOR" ;;
      3) TARGET_VERSION="$NEXT_MAJOR" ;;
      4) read -rp "Entrez la version (ex: v1.2.3) : " TARGET_VERSION ;;
      *) TARGET_VERSION="$NEXT_PATCH" ;;
    esac
  fi
fi

[[ "$TARGET_VERSION" =~ ^v[0-9]+\.[0-9]+\.[0-9]+$ ]] || fatal "Format SemVer invalide: '$TARGET_VERSION' (attendu: vX.Y.Z)"

if git rev-parse "$TARGET_VERSION" >/dev/null 2>&1; then
  fatal "Le tag $TARGET_VERSION existe déjà dans le dépôt."
fi

# 6. Récapitulatif
echo -e "\n${BOLD}--- Récapitulatif de la release ---${NC}"
echo -e "Version précédente : ${YELLOW}${LATEST_TAG}${NC}"
echo -e "Version cible      : ${GREEN}${BOLD}${TARGET_VERSION}${NC}"
echo -e "Branche source     : ${BOLD}${CURRENT_BRANCH}${NC}"
echo -e "Dernier commit     : $(git log -1 --pretty=format:'%h - %s (%an)')"

echo -e "\n${YELLOW}⚠ Cette action va créer et pousser le tag '${TARGET_VERSION}' sur GitHub.${NC}"
read -rp "Confirmez-vous la publication de la release ${TARGET_VERSION} ? [o/N] " confirm
if [[ ! "$confirm" =~ ^[oOyY]$ ]]; then
  fatal "Publication annulée par l'utilisateur."
fi

# 7. Création et push du tag
info "Création du tag annoté '${TARGET_VERSION}'..."
git tag -a "$TARGET_VERSION" -m "Release $TARGET_VERSION"
ok "Tag créé localement."

info "Push du tag sur origin..."
git push origin "$TARGET_VERSION"
ok "Tag poussé sur GitHub !"

echo -e "\n${BOLD}${GREEN}✔ Release ${TARGET_VERSION} déclenchée avec succès !${NC}"
echo -e "Suivez la compilation et la signature sur : ${CYAN}https://github.com/openfirenet/open-firenet-installer/actions${NC}\n"
