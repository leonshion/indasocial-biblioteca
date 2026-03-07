import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { IndasocialBiblioteca } from "../target/types/indasocial_biblioteca";
import { assert } from "chai";

describe("indasocial-biblioteca", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace
    .IndasocialBiblioteca as Program<IndasocialBiblioteca>;

  const owner = provider.wallet;

  // PDA de la biblioteca
  const [bibliotecaPda] = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from("biblioteca"), owner.publicKey.toBuffer()],
    program.programId
  );

  // ─────────────────────────────────────
  // 1. Crear biblioteca
  // ─────────────────────────────────────
  it("✅ Crea la biblioteca correctamente", async () => {
    await program.methods
      .crearBiblioteca("IndaSOCIAL Articles")
      .accounts({
        biblioteca: bibliotecaPda,
        owner: owner.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .rpc();

    const cuenta = await program.account.biblioteca.fetch(bibliotecaPda);
    assert.equal(cuenta.nombre, "IndaSOCIAL Articles");
    assert.equal(cuenta.articulos.length, 0);
    console.log("📚 Biblioteca creada:", cuenta.nombre);
  });

  // ─────────────────────────────────────
  // 2. Agregar artículos
  // ─────────────────────────────────────
  it("📰 Agrega un artículo correctamente", async () => {
    await program.methods
      .agregarArticulo(
        "Qué es DeFi y por qué importa",
        "Equipo IndaSOCIAL",
        "DeFi",
        "Una introducción clara al mundo de las finanzas descentralizadas.",
        "https://indasocial.com/articulos/que-es-defi"
      )
      .accounts({
        biblioteca: bibliotecaPda,
        owner: owner.publicKey,
      })
      .rpc();

    const cuenta = await program.account.biblioteca.fetch(bibliotecaPda);
    assert.equal(cuenta.articulos.length, 1);
    assert.equal(cuenta.articulos[0].titulo, "Qué es DeFi y por qué importa");
    assert.equal(cuenta.articulos[0].activo, true);
    console.log("📰 Artículo agregado:", cuenta.articulos[0].titulo);
  });

  // ─────────────────────────────────────
  // 3. Ver artículos
  // ─────────────────────────────────────
  it("👀 Lista los artículos correctamente", async () => {
    await program.methods
      .verArticulos()
      .accounts({
        biblioteca: bibliotecaPda,
      })
      .rpc();

    const cuenta = await program.account.biblioteca.fetch(bibliotecaPda);
    assert.isAbove(cuenta.articulos.length, 0);
    console.log("📋 Total artículos:", cuenta.articulos.length);
  });

  // ─────────────────────────────────────
  // 4. Desactivar artículo
  // ─────────────────────────────────────
  it("🔒 Desactiva un artículo correctamente", async () => {
    await program.methods
      .desactivarArticulo("Qué es DeFi y por qué importa")
      .accounts({
        biblioteca: bibliotecaPda,
        owner: owner.publicKey,
      })
      .rpc();

    const cuenta = await program.account.biblioteca.fetch(bibliotecaPda);
    assert.equal(cuenta.articulos[0].activo, false);
    console.log("🔒 Artículo desactivado correctamente");
  });

  // ─────────────────────────────────────
  // 5. Eliminar artículo
  // ─────────────────────────────────────
  it("🗑️ Elimina un artículo correctamente", async () => {
    await program.methods
      .eliminarArticulo("Qué es DeFi y por qué importa")
      .accounts({
        biblioteca: bibliotecaPda,
        owner: owner.publicKey,
      })
      .rpc();

    const cuenta = await program.account.biblioteca.fetch(bibliotecaPda);
    assert.equal(cuenta.articulos.length, 0);
    console.log("🗑️ Artículo eliminado. Total:", cuenta.articulos.length);
  });
});
