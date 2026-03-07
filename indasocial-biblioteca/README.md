# 📚 IndaSOCIAL — Biblioteca de Artículos

> Un espacio de conocimiento colectivo construido sobre Solana, para y por la comunidad Inda.

---

## 🎯 Objetivo

La **Biblioteca de Artículos de IndaSOCIAL** es un programa on-chain desplegado en la red Solana que permite a la comunidad Inda acceder a contenido curado, organizado por categorías, directamente desde la plataforma.

El objetivo principal es crear un **repositorio descentralizado de artículos** donde:

- 📖 Cualquier miembro de la comunidad puede **leer artículos** sobre temas relevantes como Web3, DeFi, Blockchain, IA y más.
- 🗂️ El contenido está **organizado por categorías** para facilitar la navegación y el descubrimiento.
- 🪙 Los usuarios pueden **ganar INDA Coins** (tokens en la red Solana) por su participación activa — leer, compartir y contribuir con artículos a la comunidad.

---

## 🏗️ Estructura del Proyecto

```
indasocial-biblioteca/
├── programs/
│   └── indasocial-biblioteca/
│       ├── src/
│       │   └── lib.rs          ← Programa principal (Rust / Anchor)
│       └── Cargo.toml
├── tests/
│   └── indasocial-biblioteca.ts  ← Tests en TypeScript
├── Anchor.toml                   ← Configuración de Anchor
├── Cargo.toml                    ← Workspace Rust
├── package.json
├── tsconfig.json
└── .gitignore
```

---

## ✨ Instrucciones del Programa

| Instrucción | Descripción |
|---|---|
| `crear_biblioteca` | Inicializa la biblioteca con nombre y owner |
| `agregar_articulo` | Agrega un artículo con título, autor, categoría, resumen y URL |
| `ver_articulos` | Lista todos los artículos disponibles |
| `desactivar_articulo` | Oculta un artículo sin eliminarlo permanentemente |
| `eliminar_articulo` | Elimina un artículo del registro on-chain |

---

## 🔧 Tecnologías

- **Solana** — Blockchain de alta velocidad y bajos costos
- **Anchor Framework** `0.30.1` — Framework de desarrollo para programas Solana en Rust
- **Rust** — Lenguaje del programa on-chain
- **TypeScript** — Tests e integración con el frontend

---

## 🚀 Despliegue rápido (Solana Playground)

La forma más sencilla de compilar y desplegar sin configuración local:

1. Ve a [beta.solpg.io](https://beta.solpg.io)
2. Crea un proyecto nuevo → **Anchor (Rust)**
3. Pega el contenido de `programs/indasocial-biblioteca/src/lib.rs`
4. Click en **Build** 🔨 → **Deploy** 🚀
5. Solicita SOL de prueba con el botón **Airdrop**

---

## 🛠️ Desarrollo local

```bash
# Instalar dependencias
yarn install

# Compilar el programa
anchor build

# Ejecutar tests
anchor test
```

> Requiere: [Rust](https://rustup.rs), [Solana CLI](https://docs.solana.com/cli/install-solana-cli-tools), [Anchor CLI](https://www.anchor-lang.com/docs/installation)

---

## 🪙 INDA Coins

La biblioteca forma parte del ecosistema de incentivos de IndaSOCIAL. La participación activa de los usuarios — leer artículos, contribuir contenido y compartir conocimiento — es la base para la distribución de **INDA Coins**, el token utilitario de la comunidad dentro de la plataforma.

> *El conocimiento compartido es la base de una comunidad fuerte.*

---

*Construido con ❤️ para la comunidad Inda — sobre Solana.*
