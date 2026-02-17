# MasterEnv
- Windows Latest Release : [![GitHub release (latest by date)](https://img.shields.io/github/v/release/QVL-CustHome/masterenv)](https://github.com/QVL-CustHome/masterenv/releases/latest)

---

🇬🇧 **MasterEnv** is a Rust-based tool designed to synchronize environment variables across multiple microservices or directories in a monorepo. It ensures that specific variables defined in a central `.masterenv` file are propagated to all local configuration files, enforcing consistency across your architecture.

🇫🇷 **MasterEnv** est un outil écrit en Rust conçu pour synchroniser les variables d'environnement à travers plusieurs microservices ou dossiers dans un monorepo. Il garantit que les variables définies dans un fichier central `.masterenv` sont propagées dans tous les fichiers de configuration locaux, assurant une cohérence dans votre architecture.
Voici le fichier `README.md` complet, assemblé selon tes instructions : **Sommaire** en haut, suivi de la **version Anglaise**, puis de la **version Française**.

---

## Table of Contents / Sommaire

- [🇬🇧 English Version](#-english-version)
  - [Part 1: Installation](#part-1--installation)
  - [Part 2: Usage](#part-2--usage)
  - [Part 3: Technical Explanation](#part-3--technical-explanation)
- [🇫🇷 Version Française](#-version-française)
  - [Partie 1 : Installation](#partie-1--installation)
  - [Partie 2 : Utilisation](#partie-2--utilisation)
  - [Partie 3 : Explication Technique](#partie-3--explication-technique)

---

## 🇬🇧 English Version

### Part 1 : Installation

#### Quick Install

- Download the latest version: [![GitHub release (latest by date)](https://img.shields.io/github/v/release/QVL-CustHome/masterenv)](https://github.com/QVL-CustHome/masterenv/releases/latest)
- Extract the folder and place it at the root of your project.

#### Project Structure with MasterEnv

```text
Project_Root/
├── masterenv/            # Extracted folder placed in your project
│   ├── masterenv/
│   │    └──masterenv.exe # Executable file
│   ├── app-config.toml   # MasterEnv configuration file
│   └── .masterenv        # MasterEnv environment file
├── Service A/
│   └── .env              # File updated by MasterEnv
└── Service B/
    └── .env              # File updated by MasterEnv

```

#### Installation via GitHub (Dev)

##### Prerequisites

* **Rust & Cargo**: Ensure Rust is installed via [rustup.rs]().
* **Git**: To clone the repository.

1. **Clone the repository:**
Open your terminal and run:
```bash
git clone [https://github.com/your-username/master-env.git](https://github.com/your-username/master-env.git)
cd master-env

```


2. **Build the project:**
Compile the project in "release" mode for better performance:
```bash
cargo build --release

```


The executable will be located in `./target/release/master_env`.

#### Configuration

**Example `app-config.toml`:**

```toml
config_files = [".env", ".toml"]
ignored_directories = ["target", ".git", "node_modules"]

```

#### Running the Tool

Navigate to the folder containing the executable and run it:

```bash
./master_env

```

---

### Part 2 : Usage

#### Adding an Environment Variable

The `.masterenv` file acts as the source of truth. To propagate a new value:

1. Open the `.masterenv` file located at the project root.
2. Add or modify your variable on a new line (format `KEY=VALUE`).
```properties
# Example in .masterenv
API_PORT=3000
DB_HOST=localhost

```


3. Run the `master_env` executable.
4. The tool will scan all eligible configuration files. If a line starts with `API_PORT=` in a child file, its value will be automatically replaced by `3000`.

> **Important Note:** The tool works by **synchronization**, not injection. It will not create the variable in the child file if the key does not already exist there. It only updates the values of existing keys to ensure they match the Master.

#### Adding a Directory to Ignore

To optimize performance or avoid modifying sensitive files, you can exclude entire directories from the scan.

1. Open the `app-config` configuration file (located at the same level as `.masterenv`).
2. Locate the `ignored_directories` key.
3. Add the directory name to the list (TOML format).
```toml
# Example in app-config
ignored_directories = ["target", ".git", "node_modules", "temp_build", "legacy_service"]

```


4. The listed directories and their contents will be completely ignored during the next run.

The tool will automatically scan parent directories, find matching configuration files, and update lines where keys match those in `.masterenv`.

---

### Part 3 : Technical Explanation

The code is divided into two main modules: business logic (`main.rs`) and configuration management (`config.rs`).

#### 1. Configuration Management (`config.rs`)

This module uses the **Singleton pattern** via `std::sync::OnceLock` to load the configuration once and make it accessible everywhere without passing it as a parameter.

* **`AppSettings` struct**: Defines the structure of the `app-config` file (list of files to scan and directories to ignore).
* **`OnceLock<AppSettings>`**: A Rust synchronization primitive. It ensures the configuration is initialized in a thread-safe manner upon the first request and cached for subsequent calls.
* **`Configuration::get_instance()`**: Loads the `../app-config` file via the `config` crate and deserializes it. If the file is missing or malformed, the program panics.
* **`is_config_file` & `is_ignored**`: Static helper methods that check if a file should be processed or a directory ignored, based on the loaded configuration.

#### 2. Business Logic (`main.rs`)

**Initialization**

* **`get_masterenv_path`**: Locates the `.masterenv` file by going up one level (`../`) relative to the executable.
* **`load_masterenv_file`**: Reads the master file line by line. It uses `split_var_name_value` to parse `KEY=VALUE` pairs and stores them in a `HashMap`. This Map serves as the reference for replacements.

**Recursive Scanning (`check_dir_recursive`)**
The function traverses the file tree:

1. Ignores directories defined in the configuration (e.g., `node_modules`).
2. Recursively calls itself for subdirectories.
3. Calls `check_file` if it finds a file matching the configured extensions.

**File Update (`check_file`)**
This is the core of the system. Instead of blindly replacing the file:

1. It reads the entire target file into memory.
2. It iterates through each line.
3. **`get_line_expected`**: Parses the line. If the key (e.g., `PORT`) exists in the `.masterenv` HashMap, it reformats the line with the master value (`PORT=8080`). Otherwise, it keeps the original line.
4. **Conditional Writing**: The file is rewritten to disk **only if** a modification was detected (`has_wrong_line`). This prevents unnecessary writes and preserves metadata (timestamps).

---

---

## 🇫🇷 Version Française

### Partie 1 : Installation

#### Installation rapide

* Télécharger la dernière version : []()
* Extraire le dossier et le placer à la racine de votre projet.

#### Structure de votre projet avec masterenv

```text
Racine_Projet/
├── masterenv/            # Dossier extrait à placer dans votre projet
│   ├── masterenv/
│   │    └──masterenv.exe # Fichier exécutable
│   ├── app-config.toml   # Fichier de configuration de masterenv
│   └── .masterenv        # Fichier d'environnement masterenv
├── Service A/
│   └── .env              # Fichier mis à jour par masterenv
└── Service B/
    └── .env              # Fichier mis à jour par masterenv

```

#### Installation via GitHub (Dev)

##### Prérequis

* **Rust & Cargo** : Assurez-vous que Rust est installé via [rustup.rs]().
* **Git** : Pour cloner le dépôt.

1. **Cloner le dépôt :**
Ouvrez votre terminal et lancez :
```bash
git clone [https://github.com/votre-username/master-env.git](https://github.com/votre-username/master-env.git)
cd master-env

```


2. **Compiler le projet :**
Compilez le projet en mode "release" pour de meilleures performances :
```bash
cargo build --release

```


L'exécutable se trouvera dans `./target/release/master_env`.

#### Configuration

**Exemple `app-config.toml` :**

```toml
config_files = [".env", ".toml"]
ignored_directories = ["target", ".git", "node_modules"]

```

#### Lancer l'outil

Naviguez vers le dossier contenant l'exécutable et lancez-le :

```bash
./master_env

```

---

### Partie 2 : Utilisation

#### Ajout de variable d'environnement

Le fichier `.masterenv` agit comme la source de vérité. Pour propager une nouvelle valeur :

1. Ouvrez le fichier `.masterenv` situé à la racine du projet.
2. Ajoutez ou modifiez votre variable sur une nouvelle ligne (format `CLÉ=VALEUR`).
```properties
# Exemple dans .masterenv
API_PORT=3000
DB_HOST=localhost

```


3. Lancez l'exécutable `master_env`.
4. L'outil va parcourir tous les fichiers de configuration éligibles. Si une ligne commence par `API_PORT=` dans un fichier enfant, sa valeur sera automatiquement remplacée par `3000`.

> **Note importante :** L'outil fonctionne par **synchronisation**, pas par injection. Il ne créera pas la variable dans le fichier enfant si la clé n'y existe pas déjà. Il met uniquement à jour les valeurs des clés existantes pour garantir qu'elles correspondent au Master.

#### Ajout de dossier à ignorer

Pour optimiser les performances ou éviter de modifier des fichiers sensibles, vous pouvez exclure des dossiers entiers du scan.

1. Ouvrez le fichier de configuration `app-config` (situé au même niveau que `.masterenv`).
2. Localisez la clé `ignored_directories`.
3. Ajoutez le nom du dossier à la liste (format TOML).
```toml
# Exemple dans app-config
ignored_directories = ["target", ".git", "node_modules", "temp_build", "legacy_service"]

```


4. Les dossiers listés et leur contenu seront totalement ignorés lors de la prochaine exécution.

L'outil scannera automatiquement les dossiers parents, trouvera les fichiers de configuration correspondants et mettra à jour les lignes où les clés correspondent à celles du `.masterenv`.

---

### Partie 3 : Explication Technique

Le code est divisé en deux modules principaux : la logique métier (`main.rs`) et la gestion de la configuration (`config.rs`).

#### 1. Gestion de la Configuration (`config.rs`)

Ce module utilise le **pattern Singleton** via `std::sync::OnceLock` pour charger la configuration une seule fois et la rendre accessible partout sans la passer en paramètre.

* **`AppSettings` struct** : Définit la structure du fichier `app-config` (liste des fichiers à scanner et dossiers à ignorer).
* **`OnceLock<AppSettings>`** : Une primitive de synchronisation Rust. Elle garantit que la configuration est initialisée de manière thread-safe (sécurisée) à la première demande et mise en cache pour les appels suivants.
* **`Configuration::get_instance()`** : Charge le fichier `../app-config` via la librairie `config` et le désérialise. Si le fichier est absent ou malformé, le programme s'arrête (panic).
* **`is_config_file` & `is_ignored**` : Méthodes utilitaires statiques qui vérifient si un fichier doit être traité ou un dossier ignoré, selon la configuration chargée.

#### 2. Logique Métier (`main.rs`)

**Initialisation**

* **`get_masterenv_path`** : Localise le fichier `.masterenv` en remontant d'un niveau (`../`) par rapport à l'exécutable.
* **`load_masterenv_file`** : Lit le fichier maître ligne par ligne. Il utilise `split_var_name_value` pour découper les paires `CLÉ=VALEUR` et les stocke dans une `HashMap`. Cette Map sert de référence pour les remplacements.

**Parcours Récursif (`check_dir_recursive`)**
La fonction traverse l'arborescence de fichiers :

1. Ignore les dossiers définis dans la configuration (ex: `node_modules`).
2. S'appelle récursivement pour les sous-dossiers.
3. Appelle `check_file` si elle trouve un fichier correspondant aux extensions configurées.

**Mise à jour de Fichier (`check_file`)**
C'est le cœur du système. Plutôt que de remplacer aveuglément le fichier :

1. Il lit le fichier cible entièrement en mémoire.
2. Il itère sur chaque ligne.
3. **`get_line_expected`** : Analyse la ligne. Si la clé (ex: `PORT`) existe dans la HashMap du `.masterenv`, il reformate la ligne avec la valeur maître (`PORT=8080`). Sinon, il garde la ligne originale.
4. **Écriture Conditionnelle** : Le fichier n'est réécrit sur le disque **que si** une modification a été détectée (`has_wrong_line`). Cela évite des écritures inutiles et préserve les métadonnées (timestamps).
