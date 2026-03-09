# 📚 INDASOCIAL — Biblioteca de Artículos

> Un espacio de conocimiento colectivo construido sobre Solana, para y por la comunidad Inda.

---

## 🎯 Objetivo

La **Biblioteca de Artículos de IndaSOCIAL** es un programa on-chain desplegado en la red Solana que permite a la comunidad Inda acceder a contenido curado, organizado por categorías, directamente desde la plataforma.

El objetivo principal es crear un **repositorio descentralizado de artículos** donde:

- 📖 Cualquier miembro de la comunidad puede **leer artículos** sobre temas relevantes como Web3, DeFi, Blockchain, IA y más.
- 🗂️ El contenido está **organizado por categorías** para facilitar la navegación y el descubrimiento.
- 🪙 Los usuarios pueden **ganar INDA Tokens** (tokens en la red Solana) por su participación activa — leer, compartir y contribuir con artículos a la comunidad.

---

## ✨ Funcionalidades del Programa

| Instrucción | Descripción |
|---|---|
| `crear_biblioteca` | Inicializa la biblioteca con nombre y owner |
| `agregar_articulo` | Agrega un artículo con título, autor, categoría, resumen y URL |
| `ver_articulos` | Lista todos los artículos disponibles |
| `desactivar_articulo` | Oculta un artículo sin eliminarlo permanentemente |
| `eliminar_articulo` | Elimina un artículo del registro on-chain |

---

## 🏗️ Estructura del Proyecto

```
programs/
└── indasocial-biblioteca/
    └── lib.rs       ← Programa principal (Anchor / Rust)
```

---

## 🔧 Tecnologías

- **Solana** — Blockchain de alta velocidad y bajos costos
- **Anchor Framework** — Framework de desarrollo para programas Solana en Rust
- **Rust** — Lenguaje de programación del programa on-chain

---

## 🚀 Despliegue

Este programa está pensado para desplegarse en **Solana Devnet** durante la fase de desarrollo, y posteriormente en **Mainnet** para uso en producción dentro de la plataforma IndaSOCIAL.

Para compilar y desplegar, puedes usar [Solana Playground](https://beta.solpg.io) directamente desde el navegador, sin necesidad de configuración local.

---

## 🪙 INDA TOKENS

La biblioteca forma parte del ecosistema de incentivos de INDASOCIAL. La participación activa de los usuarios — leer artículos, contribuir contenido y compartir conocimiento — es la base para la distribución de INDATOKENS, el token utilitario de la comunidad dentro de la plataforma.

> *El conocimiento compartido es la base de una comunidad fuerte.*

---

## 🤝 Comunidad

INDASOCIAL es una plataforma social descentralizada construida para su comunidad. Este repositorio es parte del esfuerzo colectivo por construir herramientas útiles, transparentes y accesibles para todos los miembros de Inda.

---

Construido con ❤️ para la comunidad Indasocial — sobre Solana.
