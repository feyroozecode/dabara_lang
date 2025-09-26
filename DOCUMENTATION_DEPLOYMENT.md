# Configuration de déploiement automatique

Ce document explique comment configurer le déploiement automatique de la documentation.

## GitHub Pages

### Configuration requise

1. **Activer GitHub Pages** dans les paramètres du repository
2. **Source** : GitHub Actions
3. **Branche** : gh-pages (créée automatiquement)

### URLs de déploiement

Une fois configuré, la documentation sera disponible à :
- **Documentation technique** : `https://votre-username.github.io/dabara/docs/`
- **eBook d'apprentissage** : `https://votre-username.github.io/dabara/ebook/`

## Workflow automatique

Le fichier `.github/workflows/docs.yml` :

### Déclencheurs
- Push sur la branche main avec modifications dans `docs/` ou `ebook/`
- Déclenchement manuel via l'interface GitHub

### Étapes
1. **Build** : Génération avec mdBook
2. **Deploy** : Publication sur GitHub Pages
3. **Artifacts** : Sauvegarde des fichiers générés
4. **EPUB** : Génération du format livre électronique

## Génération locale

### Script build-docs.sh

```bash
# Générer tout
./build-docs.sh

# Générer uniquement la documentation
./build-docs.sh docs

# Générer uniquement l'eBook
./build-docs.sh ebook

# Servir localement pour prévisualisation
./build-docs.sh serve

# Générer l'EPUB
./build-docs.sh epub

# Nettoyer les fichiers générés
./build-docs.sh clean
```

### Prévisualisation locale

```bash
# Démarrer les serveurs de développement
./build-docs.sh serve
```

Accès :
- Documentation : http://localhost:3000
- eBook : http://localhost:3001

## Structure des fichiers

```
dabara/
├── docs/                      # Documentation technique
│   ├── src/
│   │   ├── SUMMARY.md        # Table des matières
│   │   ├── introduction.md   # Introduction
│   │   ├── architecture/     # Architecture du code
│   │   ├── language/         # Référence du langage
│   │   └── ...
│   ├── book.toml            # Configuration mdBook
│   └── book/                # Fichiers générés
│
├── ebook/                    # eBook d'apprentissage
│   ├── src/
│   │   ├── SUMMARY.md       # Table des matières
│   │   ├── introduction.md  # Introduction
│   │   ├── part1/           # Partie 1 : Fondamentaux
│   │   ├── part2/           # Partie 2 : Variables
│   │   ├── part3/           # Partie 3 : Programmation
│   │   ├── part4/           # Partie 4 : Avancé
│   │   └── annexes/         # Annexes
│   ├── book.toml           # Configuration mdBook
│   └── book/               # Fichiers générés
│
└── .github/workflows/docs.yml # CI/CD pour documentation
```

## Formats de sortie

### HTML (Web)
- Navigation interactive
- Recherche intégrée
- Responsive design
- GitHub Pages ready

### EPUB (eBook)
- Compatible avec la plupart des lecteurs
- Téléchargeable pour lecture offline
- Métadonnées complètes
- Table des matières interactive

### PDF (bientôt)
- Format d'impression
- Mise en page optimisée
- Génération via Pandoc

## Personnalisation

### Thèmes et styles

Modifiez `book.toml` pour personnaliser :
- Couleurs
- Polices
- Layout
- Fonctionnalités

### Plugins mdBook

Plugins disponibles :
- `mdbook-mermaid` : Diagrammes
- `mdbook-plantuml` : UML
- `mdbook-katex` : Formules mathématiques
- `mdbook-linkcheck` : Vérification des liens

## Maintenance

### Mise à jour
- mdBook : `cargo install mdbook --force`
- Plugins : Selon les besoins

### Surveillance
- GitHub Actions : Logs de build
- GitHub Pages : Statistiques de visite
- Issues : Retours utilisateurs

## Automatisation complète

Une fois configuré, le workflow est entièrement automatique :

1. **Modification** : Édition des fichiers markdown
2. **Commit** : Push vers GitHub
3. **Build** : GitHub Actions génère automatiquement
4. **Deploy** : Publication automatique
5. **Notification** : Disponibilité immédiate

Aucune intervention manuelle requise ! 🚀# Configuration de déploiement automatique

Ce document explique comment configurer le déploiement automatique de la documentation.

## GitHub Pages

### Configuration requise

1. **Activer GitHub Pages** dans les paramètres du repository
2. **Source** : GitHub Actions
3. **Branche** : gh-pages (créée automatiquement)

### URLs de déploiement

Une fois configuré, la documentation sera disponible à :
- **Documentation technique** : `https://votre-username.github.io/dabara/docs/`
- **eBook d'apprentissage** : `https://votre-username.github.io/dabara/ebook/`

## Workflow automatique

Le fichier `.github/workflows/docs.yml` :

### Déclencheurs
- Push sur la branche main avec modifications dans `docs/` ou `ebook/`
- Déclenchement manuel via l'interface GitHub

### Étapes
1. **Build** : Génération avec mdBook
2. **Deploy** : Publication sur GitHub Pages
3. **Artifacts** : Sauvegarde des fichiers générés
4. **EPUB** : Génération du format livre électronique

## Génération locale

### Script build-docs.sh

```bash
# Générer tout
./build-docs.sh

# Générer uniquement la documentation
./build-docs.sh docs

# Générer uniquement l'eBook
./build-docs.sh ebook

# Servir localement pour prévisualisation
./build-docs.sh serve

# Générer l'EPUB
./build-docs.sh epub

# Nettoyer les fichiers générés
./build-docs.sh clean
```

### Prévisualisation locale

```bash
# Démarrer les serveurs de développement
./build-docs.sh serve
```

Accès :
- Documentation : http://localhost:3000
- eBook : http://localhost:3001

## Structure des fichiers

```
dabara/
├── docs/                      # Documentation technique
│   ├── src/
│   │   ├── SUMMARY.md        # Table des matières
│   │   ├── introduction.md   # Introduction
│   │   ├── architecture/     # Architecture du code
│   │   ├── language/         # Référence du langage
│   │   └── ...
│   ├── book.toml            # Configuration mdBook
│   └── book/                # Fichiers générés
│
├── ebook/                    # eBook d'apprentissage
│   ├── src/
│   │   ├── SUMMARY.md       # Table des matières
│   │   ├── introduction.md  # Introduction
│   │   ├── part1/           # Partie 1 : Fondamentaux
│   │   ├── part2/           # Partie 2 : Variables
│   │   ├── part3/           # Partie 3 : Programmation
│   │   ├── part4/           # Partie 4 : Avancé
│   │   └── annexes/         # Annexes
│   ├── book.toml           # Configuration mdBook
│   └── book/               # Fichiers générés
│
└── .github/workflows/docs.yml # CI/CD pour documentation
```

## Formats de sortie

### HTML (Web)
- Navigation interactive
- Recherche intégrée
- Responsive design
- GitHub Pages ready

### EPUB (eBook)
- Compatible avec la plupart des lecteurs
- Téléchargeable pour lecture offline
- Métadonnées complètes
- Table des matières interactive

### PDF (bientôt)
- Format d'impression
- Mise en page optimisée
- Génération via Pandoc

## Personnalisation

### Thèmes et styles

Modifiez `book.toml` pour personnaliser :
- Couleurs
- Polices
- Layout
- Fonctionnalités

### Plugins mdBook

Plugins disponibles :
- `mdbook-mermaid` : Diagrammes
- `mdbook-plantuml` : UML
- `mdbook-katex` : Formules mathématiques
- `mdbook-linkcheck` : Vérification des liens

## Maintenance

### Mise à jour
- mdBook : `cargo install mdbook --force`
- Plugins : Selon les besoins

### Surveillance
- GitHub Actions : Logs de build
- GitHub Pages : Statistiques de visite
- Issues : Retours utilisateurs

## Automatisation complète

Une fois configuré, le workflow est entièrement automatique :

1. **Modification** : Édition des fichiers markdown
2. **Commit** : Push vers GitHub
3. **Build** : GitHub Actions génère automatiquement
4. **Deploy** : Publication automatique
5. **Notification** : Disponibilité immédiate

Aucune intervention manuelle requise ! 🚀