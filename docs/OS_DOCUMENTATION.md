# Documentación completa de zerOS

## Arranque
- Bootloader UEFI carga kernel x86_64.
- Kernel inicializa consola, teclado PS/2 y shell.
- Mensaje inicial mostrado: `Bienvenido a ZerOS, escribe help para empezar`.

## Shell
Comandos base:
- `help`
- `apps`
- `calc`
- `clock`
- `notepad`
- `clear`

## Aplicaciones preinstaladas
- `calculator.bap`: calculadora básica.
- `clock.bap`: muestra hora RTC.
- `notepad.bap`: crear, abrir y guardar notas.

## Almacenamiento
zerOS define `zerFS` (diseño modular):
- Superbloque
- Tabla de inodos
- Bloques de datos
- Journal básico

## Extensión futura
- Scheduler con multitarea preventiva
- MMU avanzada
- Drivers SATA/NVMe y red
- Seguridad por capacidades
