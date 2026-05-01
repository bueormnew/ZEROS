# Ficha técnica 100% técnica - zerOS

- Nombre: zerOS
- Arquitectura objetivo: x86_64 (64 bits)
- Modelo de kernel: monolítico modular
- Lenguaje base del kernel: Rust `no_std`
- Modo de UI: consola de texto
- Entrada: controlador PS/2
- Salida: framebuffer/VGA texto
- ABI de apps: BAP-ABI v1
- Formato ejecutable: `.bap` (Bueorm Application Package)
- FS objetivo: zerFS
- Boot: UEFI (fase actual), BIOS planeado
- Estado: MVP funcional expandible
