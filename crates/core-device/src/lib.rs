//! Núcleo de visibilidad del dispositivo.
//!
//! Librería pura, sin dependencias de interfaz ni de red, para poder
//! reutilizarla desde el CLI ahora y desde un servidor más adelante.

/// Versión del núcleo, útil para comprobar que el CLI enlaza bien.
pub fn version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn version_no_esta_vacia() {
        assert!(!version().is_empty());
    }
}
