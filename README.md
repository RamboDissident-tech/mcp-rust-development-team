# MCP Rust Development Team

Écosystème complet en Rust et DeepSeek Harness composé de :
1. **Un serveur MCP haute performance** écrit en Rust (mcp-agent-tools) exposant des outils sécurisés pour la recherche, l'automatisation et l'édition de fichiers
2. **Une équipe de 7 agents** prête à être utilisée dans DeepSeek Harness avec votre LLM local

## Structure du projet

- Cargo.toml - Configuration du projet Rust
- src/main.rs - Serveur MCP principal en Rust
- dsh-config/ - Configuration complète pour DeepSeek Harness avec les 7 agents

## Agents configurés

1. **Orchestrator** - Planification et coordination des tâches
2. **Web Explorer** - Collecte de documentation technique via web_fetch_and_extract
3. **Technical Analyst** - Analyse et synthèse des données brutes
4. **Rust Developer** - Écriture sécurisée de code Rust
5. **Rust Tester** - Vérifications Cargo (check, clippy, test)
6. **Automation Engineer** - Scripts d'automatisation
7. **Quality Assurance Controller** - Contrôle qualité final

## Outils MCP disponibles

- safe_write_file - Écriture sécurisée avec sauvegarde Git
- estore_backup - Restauration depuis backups
- web_fetch_and_extract - Extraction web
- un_cargo_check - Vérifications Cargo
- un_automation_task - Exécution de scripts d'automatisation

## Contraintes et sécurité

Le système respecte :
- **24 Go VRAM** : Maximum 2 agents actifs en parallèle
- **Sauvegardes automatiques** : Toutes les modifications passent par safe_write_file
- **Git intégré** : Toutes les écritures sont sauvegardées dans Git
- **Contrôle qualité** : Les livraisons sont auditées

## Utilisation

1. Compiler le projet : cargo build --release
2. Démarrez DeepSeek Harness avec la configuration fournie
3. Les 7 agents seront automatiquement configurés et prêts à l'emploi

## Compilation

Pour compiler le serveur MCP en mode release :

`ash
cargo build --release
`

Le binaire compilé sera disponible dans 	arget/release/mcp-agent-tools.exe.

## Licence

Ce projet est sous licence MIT.
