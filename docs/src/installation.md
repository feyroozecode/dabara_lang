# Installation de Dabara

Ce guide vous accompagne dans l'installation de Dabara sur votre système.

## Prérequis système

### Systèmes d'exploitation supportés
- **Linux** (Ubuntu 18.04+, Debian 10+, Fedora 30+, Arch Linux)
- **Windows** (Windows 10/11)
- **macOS** (10.15 Catalina ou plus récent)

### Configuration minimale
- **RAM** : 512 MB
- **Espace disque** : 50 MB
- **Processeur** : x86_64 ou ARM64

## Option 1 : Installation via binaires pré-compilés (Recommandé)

### Étape 1 : Téléchargement

1. Rendez-vous sur la [page des releases](https://github.com/votre-username/dabara/releases)
2. Téléchargez l'archive correspondant à votre système :

| Système | Architecture | Fichier |
|---------|-------------|---------|
| Linux (glibc) | x86_64 | `dabara-x86_64-unknown-linux-gnu.tar.gz` |
| Linux (musl) | x86_64 | `dabara-x86_64-unknown-linux-musl.tar.gz` |
| Windows | x86_64 | `dabara-x86_64-pc-windows-msvc.zip` |
| macOS (Intel) | x86_64 | `dabara-x86_64-apple-darwin.tar.gz` |
| macOS (Apple Silicon) | ARM64 | `dabara-aarch64-apple-darwin.tar.gz` |

### Étape 2 : Extraction

#### Sur Linux/macOS
```bash
# Naviguez vers le dossier de téléchargement
cd ~/Downloads

# Extrayez l'archive
tar -xzf dabara-*.tar.gz

# Le contenu sera extrait dans un dossier dabara-*
```

#### Sur Windows
```powershell
# Utiliser PowerShell ou l'explorateur Windows
Expand-Archive -Path "dabara-*.zip" -DestinationPath "."
```

### Étape 3 : Installation dans le PATH

#### Sur Linux/macOS
```bash
# Option 1 : Installation système (nécessite sudo)
sudo mv dabara-*/dabara /usr/local/bin/

# Option 2 : Installation utilisateur
mkdir -p ~/.local/bin
mv dabara-*/dabara ~/.local/bin/

# Assurez-vous que ~/.local/bin est dans votre PATH
echo 'export PATH="$HOME/.local/bin:$PATH"' >> ~/.bashrc
source ~/.bashrc
```

#### Sur Windows
1. Copiez `dabara.exe` dans un dossier de votre choix (ex: `C:\\dabara\\`)
2. Ajoutez ce dossier à votre variable d'environnement `PATH` :
   - Ouvrez "Paramètres système avancés"
   - Cliquez sur "Variables d'environnement"
   - Modifiez la variable `PATH` et ajoutez le chemin vers dabara

### Étape 4 : Vérification

Ouvrez un nouveau terminal et testez :

```bash
dabara --version
```

Vous devriez voir :
```
dabara 0.0.1
```

## Option 2 : Compilation depuis les sources

### Prérequis
- [Rust](https://rustup.rs/) 1.70 ou plus récent
- Git

### Installation de Rust
```bash
# Installation de Rust via rustup
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source ~/.cargo/env
```

### Compilation
```bash
# Cloner le repository
git clone https://github.com/votre-username/dabara.git
cd dabara

# Compiler en mode release
cargo build --release

# Installer dans ~/.cargo/bin
cargo install --path .
```

### Vérification
```bash
dabara --version
```

## Option 3 : Installation via Cargo

```bash
# Installation directe depuis crates.io (quand disponible)
cargo install dabara

# Ou depuis GitHub
cargo install --git https://github.com/votre-username/dabara.git
```

## Configuration post-installation

### Éditeur de texte

Pour une meilleure expérience de développement, configurez votre éditeur :

#### VS Code
1. Installez l'extension "Dabara Language Support" (quand disponible)
2. Ou configurez la coloration syntaxique pour les fichiers `.ha`

#### Vim/Neovim
```vim
" Ajoutez dans votre .vimrc
autocmd BufNewFile,BufRead *.ha set filetype=hausa
```

#### Emacs
```elisp
;; Ajoutez dans votre configuration
(add-to-list 'auto-mode-alist '("\\.ha\\'" . text-mode))
```

### Variables d'environnement

```bash
# Pour activer le mode debug
export DABARA_DEBUG=1

# Pour personnaliser les messages d'erreur
export DABARA_LANG=ha  # ha pour haoussa, fr pour français
```

## Vérification de l'installation

Créez un fichier test `sannu.ha` :

```hausa
fara
  rubuta "Sannu duniya!"
ƙare
```

Exécutez-le :

```bash
dabara sannu.ha
```

Sortie attendue :
```
Sannu duniya!
```

## Résolution des problèmes

### Erreur : "command not found"
- Vérifiez que dabara est dans votre PATH
- Redémarrez votre terminal
- Sur Linux/macOS : `which dabara`
- Sur Windows : `where dabara`

### Erreur de permissions
- Sur Linux/macOS : utilisez `chmod +x dabara`
- Sur Windows : exécutez en tant qu'administrateur

### Problèmes de dépendances
- Sur Linux : installez `libc6-dev` ou `musl-dev`
- Utilisez la version musl si vous avez des problèmes avec glibc

### Support
- [Issues GitHub](https://github.com/votre-username/dabara/issues)
- [Discussions](https://github.com/votre-username/dabara/discussions)
- [Documentation complète](./quick-start.md)

---

**Prêt pour la prochaine étape ?** → [Démarrage rapide](./quick-start.md)