use anchor_lang::prelude::*;

declare_id!("59FujaiRkNmPiZTZGZRNuTNKeaDgaTFjWkvkr1QgN6Mz");

#[program]
pub mod indasocial_biblioteca {
    use super::*;

    // ──────────────────────────────────────────────
    // 1. Crear la biblioteca (una por owner/wallet)
    // ──────────────────────────────────────────────
    pub fn crear_biblioteca(ctx: Context<CrearBiblioteca>, nombre: String) -> Result<()> {
        let biblioteca = &mut ctx.accounts.biblioteca;
        biblioteca.nombre = nombre;
        biblioteca.owner = *ctx.accounts.owner.key;
        biblioteca.articulos = Vec::new();
        msg!("✅ Biblioteca '{}' creada correctamente.", biblioteca.nombre);
        Ok(())
    }

    // ──────────────────────────────────────────────
    // 2. Agregar un artículo
    // ──────────────────────────────────────────────
    pub fn agregar_articulo(
        ctx: Context<AgregarArticulo>,
        titulo: String,
        autor: String,
        categoria: String,
        resumen: String,
        url: String,
    ) -> Result<()> {
        let biblioteca = &mut ctx.accounts.biblioteca;

        require!(
            biblioteca.articulos.len() < 10,
            ErrorCode::LimitArticulosAlcanzado
        );

        let nuevo_articulo = Articulo {
            titulo: titulo.clone(),
            autor,
            categoria,
            resumen,
            url,
            activo: true,
        };

        biblioteca.articulos.push(nuevo_articulo);
        msg!("📰 Artículo '{}' agregado correctamente.", titulo);
        Ok(())
    }

    // ──────────────────────────────────────────────
    // 3. Eliminar un artículo por título
    // ──────────────────────────────────────────────
    pub fn eliminar_articulo(
        ctx: Context<EliminarArticulo>,
        titulo_articulo: String,
    ) -> Result<()> {
        let biblioteca = &mut ctx.accounts.biblioteca;

        if let Some(pos) = biblioteca
            .articulos
            .iter()
            .position(|a| a.titulo == titulo_articulo)
        {
            biblioteca.articulos.swap_remove(pos);
            msg!("🗑️ Artículo '{}' eliminado.", titulo_articulo);
            Ok(())
        } else {
            Err(ErrorCode::ArticuloNoEncontrado.into())
        }
    }

    // ──────────────────────────────────────────────
    // 4. Ver todos los artículos (en logs)
    // ──────────────────────────────────────────────
    pub fn ver_articulos(ctx: Context<VerArticulos>) -> Result<()> {
        let biblioteca = &ctx.accounts.biblioteca;

        if biblioteca.articulos.is_empty() {
            msg!("📭 No hay artículos registrados en la biblioteca.");
            return Ok(());
        }

        msg!(
            "📚 Biblioteca '{}' — {} artículo(s):",
            biblioteca.nombre,
            biblioteca.articulos.len()
        );

        for (i, articulo) in biblioteca.articulos.iter().enumerate() {
            msg!(
                "#{} | Título: {} | Autor: {} | Categoría: {} | Activo: {} | URL: {}",
                i + 1,
                articulo.titulo,
                articulo.autor,
                articulo.categoria,
                articulo.activo,
                articulo.url,
            );
        }

        Ok(())
    }

    // ──────────────────────────────────────────────
    // 5. Desactivar un artículo (sin eliminarlo)
    // ──────────────────────────────────────────────
    pub fn desactivar_articulo(
        ctx: Context<DesactivarArticulo>,
        titulo_articulo: String,
    ) -> Result<()> {
        let biblioteca = &mut ctx.accounts.biblioteca;

        if let Some(articulo) = biblioteca
            .articulos
            .iter_mut()
            .find(|a| a.titulo == titulo_articulo)
        {
            articulo.activo = false;
            msg!("🔒 Artículo '{}' desactivado.", titulo_articulo);
            Ok(())
        } else {
            Err(ErrorCode::ArticuloNoEncontrado.into())
        }
    }
}

// ────────────────────────────────────────
// ERRORES
// ────────────────────────────────────────
#[error_code]
pub enum ErrorCode {
    #[msg("Artículo no encontrado en la biblioteca.")]
    ArticuloNoEncontrado,

    #[msg("Se alcanzó el límite de 10 artículos.")]
    LimitArticulosAlcanzado,
}

// ────────────────────────────────────────
// ACCOUNTS
// ────────────────────────────────────────
#[derive(Accounts)]
pub struct CrearBiblioteca<'info> {
    #[account(
        init,
        payer = owner,
        space = 8 + Biblioteca::INIT_SPACE,
        seeds = [b"biblioteca", owner.key().as_ref()],
        bump
    )]
    pub biblioteca: Account<'info, Biblioteca>,

    #[account(mut)]
    pub owner: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct AgregarArticulo<'info> {
    #[account(
        mut,
        seeds = [b"biblioteca", owner.key().as_ref()],
        bump,
        has_one = owner
    )]
    pub biblioteca: Account<'info, Biblioteca>,

    pub owner: Signer<'info>,
}

#[derive(Accounts)]
pub struct EliminarArticulo<'info> {
    #[account(
        mut,
        seeds = [b"biblioteca", owner.key().as_ref()],
        bump,
        has_one = owner
    )]
    pub biblioteca: Account<'info, Biblioteca>,

    pub owner: Signer<'info>,
}

#[derive(Accounts)]
pub struct VerArticulos<'info> {
    pub biblioteca: Account<'info, Biblioteca>,
}

#[derive(Accounts)]
pub struct DesactivarArticulo<'info> {
    #[account(
        mut,
        seeds = [b"biblioteca", owner.key().as_ref()],
        bump,
        has_one = owner
    )]
    pub biblioteca: Account<'info, Biblioteca>,

    pub owner: Signer<'info>,
}

// ────────────────────────────────────────
// STRUCTS / MODELOS
// ────────────────────────────────────────
#[account]
#[derive(InitSpace)]
pub struct Biblioteca {
    #[max_len(100)]
    pub nombre: String,           // Nombre de la biblioteca

    pub owner: Pubkey,            // Wallet del administrador

    #[max_len(10)]
    pub articulos: Vec<Articulo>, // Máx. 10 artículos
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, InitSpace)]
pub struct Articulo {
    #[max_len(120)]
    pub titulo: String,    // Título del artículo

    #[max_len(80)]
    pub autor: String,     // Autor

    #[max_len(60)]
    pub categoria: String, // Ej: "DeFi", "Web3", "IA", "Blockchain"

    #[max_len(280)]
    pub resumen: String,   // Resumen corto (estilo tweet)

    #[max_len(200)]
    pub url: String,       // URL del artículo completo

    pub activo: bool,      // true = visible, false = oculto
}
