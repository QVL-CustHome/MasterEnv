# MasterEnv
[![GitHub release (latest by date)](https://img.shields.io/github/v/release/QVL-CustHome/masterenv)](https://github.com/QVL-CustHome/masterenv/releases/latest)

---

**MasterEnv** is a Rust-based tool designed to synchronize environment variables across multiple microservices or directories in a monorepo. It ensures that specific variables defined in a central `.masterenv` file are propagated to all local configuration files (like `.env`, `.toml`), enforcing consistency across your architecture.

---

## 🇬🇧 English Version - Installation & Usage

### Prerequisites

* **Rust & Cargo**: Make sure you have Rust installed. If not, install it via [rustup.rs]().
* **Git**: To clone the repository.

### Installation via GitHub

1. **Clone the repository:**
Open your terminal and run the following command:
```bash
git clone https://github.com/your-username/master-env.git
cd master-env

```


2. **Build the project:**
Compile the project in release mode for better performance:
```bash
cargo build --release

```


The binary will be available in `./target/release/master_env`.

### Configuration Structure

For the tool to work correctly based on the hardcoded paths (`../.masterenv` and `../app-config`), your project structure should look like this:

```text
Project_Root/
├── .masterenv           # The source of truth for variables
├── app-config.toml      # Configuration for the tool (ignored dirs, extensions)
├── bin/                 # Or any subfolder where the executable is placed
│   └── master_env       # The compiled executable
├── Service A/
│   └── .env             # Target file to update
└── Service B/
    └── .env             # Target file to update

```

### Usage

1. **Define your Master Variables** in `.masterenv` at the project root:
```properties
PORT=8080
DATABASE_URL=postgres://user:pass@localhost:5432/db

```


2. **Configure the Tool** in `app-config.toml` (at the root):
```toml
config_files = [".env", ".toml"]
ignored_directories = ["target", ".git", "node_modules"]

```


3. **Run the Tool:**
Navigate to the folder containing the executable and run it:
```bash
./master_env

```


The tool will recursively scan the parent directories, find matching config files, and update lines where keys match those in `.masterenv`.

---

## 🇫🇷 Version Française - Explication du Code

Cette section détaille le fonctionnement interne du projet `MasterEnv`. Le code est divisé en deux modules principaux : la logique métier (`main.rs`) et la gestion de la configuration (`config.rs`).

### 1. Architecture Globale

L'outil fonctionne selon un principe de **"Source de Vérité"**. Il charge un dictionnaire de variables depuis un fichier maître, puis parcourt récursivement l'arborescence du projet pour forcer ces valeurs dans les fichiers enfants.

### 2. Le Module `config.rs` (Gestion de la Configuration)

Ce module utilise le pattern **Singleton** via `std::sync::OnceLock` pour charger la configuration une seule fois et la rendre accessible partout sans la passer en paramètre.

* **`AppSettings` struct** : Définit la structure du fichier `app-config` (liste des fichiers à scanner et dossiers à ignorer).
* **`OnceLock<AppSettings>`** : C'est une primitive de synchronisation de Rust. Elle garantit que la configuration est initialisée de manière thread-safe (sécurisée pour les threads) à la première demande, et stockée en cache mémoire pour les appels suivants.
* **`Configuration::get_instance()`** : Charge le fichier `../app-config` en utilisant la librairie `config` et le désérialise. Si le fichier est absent ou malformé, le programme panic (s'arrête).
* **`is_config_file` & `is_ignored**` : Méthodes utilitaires statiques qui vérifient si un fichier doit être traité ou un dossier ignoré, en se basant sur la configuration chargée.

### 3. Le Module `main.rs` (Logique Métier)

#### Initialisation

* **`get_masterenv_path`** : Localise le fichier `.masterenv` en remontant d'un cran (`../`) par rapport à l'exécutable.
* **`load_masterenv_file`** : Lit le fichier maître ligne par ligne. Il utilise `split_var_name_value` pour découper `CLE=VALEUR` et stocke le résultat dans une `HashMap`. C'est cette Map qui sert de référence pour les remplacements.

#### Parcours Récursif (`check_dir_recursive`)

La fonction parcourt l'arborescence de fichiers :

1. Elle ignore les dossiers définis dans la configuration (ex: `node_modules`, `.git`).
2. Si elle trouve un dossier, elle s'appelle elle-même (récursion).
3. Si elle trouve un fichier dont l'extension correspond à la configuration (ex: `.env`), elle lance `check_file`.

#### Mise à jour des Fichiers (`check_file`)

C'est le cœur du script. Plutôt que de remplacer aveuglément le fichier :

1. Il lit le fichier cible entièrement en mémoire.
2. Il itère sur chaque ligne.
3. **`get_line_expected`** : Analyse la ligne. Si la clé (ex: `PORT`) existe dans la `HashMap` du `.masterenv`, il reformate la ligne avec la valeur du maître (`PORT=8080`). Sinon, il garde la ligne originale.
4. **Écriture conditionnelle** : Le fichier n'est réécrit sur le disque **que si** une modification a été détectée (`has_wrong_line`). Cela évite des écritures inutiles et préserve les timestamps des fichiers non modifiés.

#### Parsing (`split_var_name_value`)

Une fonction utilitaire robuste qui :

* Ignore les commentaires (`#`).
* Ignore les lignes vides.
* Sépare proprement la clé et la valeur au premier signe `=`.

### Résumé Technique

Le code privilégie la **sécurité** (gestion des erreurs avec `Result`, pas de `unwrap` sauvages sauf à l'init de la config) et la **performance** (lecture bufferisée, écriture conditionnelle, singleton pour la config).
