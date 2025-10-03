# 🚀 Guide de déploiement rapide - Documentation Dabara

## Déploiement en 5 minutes

### 1. Vercel (Recommandé - Gratuit)

```bash
# 1. Installez Vercel CLI
npm i -g vercel

# 2. Dans le dossier racine du projet
cd /path/to/dabara
vercel

# 3. Suivez les instructions (première fois)
# 4. Pour les déploiements suivants
vercel --prod
```

**✅ Votre documentation sera disponible sur une URL `.vercel.app`**

### 2. Netlify (Alternative gratuite)

```bash
# 1. Installez Netlify CLI
npm i -g netlify-cli

# 2. Connectez-vous à Netlify
netlify login

# 3. Déployez
netlify deploy --prod --dir=docs/book
```

### 3. GitHub Pages (Automatique)

```bash
# 1. Poussez votre code sur GitHub
git add .
git commit -m "📚 Documentation complète"
git push origin main

# 2. C'est tout ! GitHub Actions déploiera automatiquement
```

**✅ Disponible sur `username.github.io/dabara`**

## URLs de déploiement disponibles

Après déploiement, votre documentation sera accessible sur :

- **Vercel** : `https://dabara-docs.vercel.app`
- **Netlify** : `https://dabara-docs.netlify.app`
- **GitHub Pages** : `https://username.github.io/dabara`

## Test local rapide

```bash
cd docs
mdbook serve --open
# Ouvre http://localhost:3000
```

## Résolution de problèmes

### mdBook non installé
```bash
cargo install mdbook
```

### Erreur de build
```bash
cd docs
rm -rf book
mdbook build
```

---

**🎉 C'est tout ! Votre documentation est maintenant en ligne et accessible au monde entier !**