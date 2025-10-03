# Documentation Dabara 📚

Bienvenue dans la documentation officielle du langage de programmation **Dabara** ! Cette documentation est construite avec [mdBook](https://rust-lang.github.io/mdBook/) et est optimisée pour le déploiement en ligne.

## 🚀 Déploiement rapide

### Option 1: Vercel (Recommandé)

[![Deploy with Vercel](https://vercel.com/button)](https://vercel.com/new/clone?repository-url=https://github.com/ahmad-dabara/dabara&project-name=dabara-docs&repository-name=dabara-docs)

1. Cliquez sur le bouton ci-dessus
2. Connectez votre compte GitHub
3. Configurez le build :
   - **Build Command**: `cd docs && mdbook build`
   - **Output Directory**: `docs/book`
4. Déployez !

### Option 2: Script automatique

```bash
# Clonez le repository
git clone https://github.com/ahmad-dabara/dabara.git
cd dabara/docs

# Rendez le script exécutable
chmod +x build-and-deploy.sh

# Déployez sur votre plateforme préférée
./build-and-deploy.sh vercel    # Vercel
./build-and-deploy.sh netlify   # Netlify
./build-and-deploy.sh github    # GitHub Pages
./build-and-deploy.sh local     # Serveur local
```

### Option 3: Build manuel

```bash
# Installez mdBook
cargo install mdbook

# Construisez la documentation
cd docs
mdbook build

# Serveur de développement
mdbook serve --open
```

## 📋 Structure de la documentation

```
docs/
├── src/                    # Sources Markdown
│   ├── SUMMARY.md         # Table des matières
│   ├── introduction.md    # Introduction
│   ├── installation.md    # Guide d'installation
│   ├── quick-start.md     # Démarrage rapide
│   ├── language/          # Référence du langage
│   │   ├── syntax.md      # Syntaxe de base
│   │   ├── types.md       # Types de données
│   │   ├── variables.md   # Variables
│   │   ├── operators.md   # Opérateurs
│   │   └── ...
│   ├── examples/          # Exemples pratiques
│   ├── architecture/      # Architecture interne
│   ├── development/       # Guide de contribution
│   ├── tools/            # Outils et API
│   └── appendix/         # Annexes
├── book/                  # Site généré (après build)
├── book.toml             # Configuration mdBook
├── build-and-deploy.sh   # Script de déploiement
├── DEPLOYMENT.md         # Guide de déploiement détaillé
└── README.md             # Ce fichier
```

## 🌍 Déploiement sur différentes plateformes

### Vercel
- **URL de production**: `https://dabara-docs.vercel.app`
- **Configuration**: Automatique avec `vercel.json`
- **Domaine personnalisé**: Configurable dans le dashboard

### Netlify
- **URL de production**: `https://dabara-docs.netlify.app`
- **Configuration**: Via `netlify.toml`
- **Fonctionnalités**: Forms, Functions, Split testing

### GitHub Pages
- **URL de production**: `https://username.github.io/dabara`
- **Configuration**: Via GitHub Actions (`.github/workflows/deploy-docs.yml`)
- **Déclenchement**: Push sur `main`

### CloudFlare Pages
- **URL de production**: `https://dabara-docs.pages.dev`
- **Configuration**: Via dashboard CloudFlare
- **Performance**: CDN global intégré

## ⚙️ Configuration pour la production

### Variables d'environnement recommandées

```bash
# Vercel
MDBOOK_VERSION=0.4.36
SITE_URL=https://dabara-docs.vercel.app

# Netlify
MDBOOK_VERSION=0.4.36
NODE_VERSION=18

# GitHub Actions
# Aucune variable nécessaire (configuré dans le workflow)
```

### Configuration du domaine personnalisé

1. **Achetez un domaine** (ex: `docs.dabara.dev`)
2. **Configurez les DNS** selon votre plateforme :
   - **Vercel**: CNAME vers `cname.vercel-dns.com`
   - **Netlify**: CNAME vers `xxx.netlify.app`
   - **GitHub**: CNAME vers `username.github.io`
3. **Mettez à jour book.toml** avec la nouvelle URL

## 🔧 Développement local

### Prérequis
- [Rust](https://rustup.rs/) (pour installer mdBook)
- [mdBook](https://rust-lang.github.io/mdBook/guide/installation.html)

### Commands utiles

```bash
# Installation de mdBook
cargo install mdbook

# Développement avec rechargement automatique
mdbook serve --open

# Build de production
mdbook build

# Vérification des liens
mdbook test

# Nettoyage
rm -rf book
```

### Ajout de contenu

1. **Nouveau chapitre** :
   ```bash
   # Créez le fichier markdown
   touch src/nouveau-chapitre.md
   
   # Ajoutez-le à SUMMARY.md
   echo "- [Nouveau Chapitre](./nouveau-chapitre.md)" >> src/SUMMARY.md
   ```

2. **Nouvelle section** :
   ```bash
   mkdir src/nouvelle-section
   touch src/nouvelle-section/index.md
   ```

3. **Test local** :
   ```bash
   mdbook serve
   # Ouvrez http://localhost:3000
   ```

## 🎨 Personnalisation

### Thèmes et styles

```toml
# book.toml
[output.html]
default-theme = "light"
preferred-dark-theme = "navy"
additional-css = ["custom.css"]
```

### Ajout de CSS personnalisé

```bash
# Créez un dossier de thème
mkdir theme
echo '.content { font-size: 16px; }' > theme/custom.css
```

### JavaScript personnalisé

```toml
# book.toml
[output.html]
additional-js = ["custom.js"]
```

## 📊 Analytics et SEO

### Google Analytics

1. Obtenez votre ID de suivi Google Analytics
2. Ajoutez dans `theme/head.hbs` :

```html
<!-- Google Analytics -->
<script async src="https://www.googletagmanager.com/gtag/js?id=GA_MEASUREMENT_ID"></script>
<script>
  window.dataLayer = window.dataLayer || [];
  function gtag(){dataLayer.push(arguments);}
  gtag('js', new Date());
  gtag('config', 'GA_MEASUREMENT_ID');
</script>
```

### Optimisation SEO

- **Sitemap**: Généré automatiquement
- **Meta tags**: Configurés dans `book.toml`
- **Open Graph**: Ajouté via `theme/head.hbs`
- **Structured data**: Support JSON-LD

## 🚨 Dépannage

### Problèmes courants

1. **mdBook non trouvé**
   ```bash
   # Solution 1: Installation via Cargo
   cargo install mdbook
   
   # Solution 2: Téléchargement direct
   curl -L https://github.com/rust-lang/mdBook/releases/download/v0.4.36/mdbook-v0.4.36-x86_64-unknown-linux-gnu.tar.gz | tar xz
   ```

2. **Build qui échoue**
   ```bash
   # Vérifiez la syntaxe Markdown
   mdbook test
   
   # Nettoyez et reconstruisez
   rm -rf book && mdbook build
   ```

3. **CSS/JS non chargé**
   - Vérifiez `site-url` dans `book.toml`
   - Assurez-vous que les chemins sont corrects

### Logs de déploiement

- **Vercel**: https://vercel.com/dashboard
- **Netlify**: https://app.netlify.com/
- **GitHub**: Actions tab du repository

## 🤝 Contribution

### Workflow de contribution

1. **Fork** le repository
2. **Créez** une branche pour vos modifications
3. **Testez** localement avec `mdbook serve`
4. **Commitez** vos changements
5. **Créez** une Pull Request

### Standards de documentation

- **Langue principale**: Français
- **Code examples**: En Dabara (haoussa)
- **Format**: Markdown avec extensions mdBook
- **Images**: Optimisées et accessibles
- **Liens**: Relatifs quand possible

## 📞 Support

- **Issues GitHub**: Pour les bugs et demandes de fonctionnalités
- **Discussions**: Pour les questions générales
- **Email**: Pour le support direct

## 📄 Licence

Cette documentation est sous licence MIT. Voir [LICENSE](../LICENSE) pour plus de détails.

---

**Made with ❤️ for the Hausa programming community**

🌟 **Star le repository** si cette documentation vous aide !
🐛 **Reportez les bugs** via GitHub Issues
🤝 **Contribuez** pour améliorer la documentation

## URLs de déploiement

Une fois déployé, votre documentation sera accessible sur :

- **🔗 Vercel**: `https://dabara-docs.vercel.app`
- **🔗 Netlify**: `https://dabara-docs.netlify.app`
- **🔗 GitHub Pages**: `https://username.github.io/dabara`
- **🔗 CloudFlare**: `https://dabara-docs.pages.dev`

Choisissez la plateforme qui convient le mieux à vos besoins ! 🚀