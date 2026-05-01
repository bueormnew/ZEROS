# zerOS

zerOS es un sistema operativo de consola x86_64 diseñado para ser extensible.

## Características actuales
- Kernel 64 bits (`no_std`) con shell de texto.
- Mensaje de inicio: `Bienvenido a ZerOS, escribe help para empezar`.
- Entrada de teclado PS/2 (polling).
- Salida de texto por framebuffer con driver simple.
- Cargador de aplicaciones `.bap` con registro interno.
- 3 aplicaciones preinstaladas en formato `.bap`: calculadora, reloj y bloc de notas.
- Documentación técnica completa en `docs/`.

## Estructura
- `kernel/`: código del kernel.
- `apps/`: aplicaciones de usuario en `.bap` (fuente + empaquetado).
- `tools/bap-pack/`: herramienta para empaquetar apps `.bap`.
- `scripts/`: scripts de compilación ISO e instalación.
- `docs/`: documentación funcional, técnica y guía `.bap`.
