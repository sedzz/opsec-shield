# opsec-shield

Herramienta open source de OPSEC para tu propio dispositivo. No es un antivirus:
responde a "¿qué pasa en mi máquina, quién tiene acceso y qué ha cambiado?".

## Estructura

- `crates/core-device`: lógica (puertos, conexiones, privilegios, persistencia).
- `crates/cli`: binario `opsec`, capa fina sobre el núcleo.

## Hoja de ruta

1. Esqueleto del workspace (hecho)
2. `opsec ports`
3. `opsec connections`
4. `opsec privileges` y `opsec startup`
5. `opsec snapshot` y `opsec diff`

## Uso

```powershell
cargo run -p opsec-cli
```

Licencia: AGPL-3.0-or-later.
