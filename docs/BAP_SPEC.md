# Especificación de archivos `.bap`

## Definición
`.bap` = **Bueorm Application Package**, ejecutable nativo de zerOS.

## Formato binario v1
Offset | Tamaño | Campo
---|---:|---
0x00 | 4 | Magic `BAP1`
0x04 | 4 | Tamaño payload (u32 LE)
0x08 | N | Payload (bytecode/script/ELF interno)

## Carga
1. Kernel valida magic y tamaño.
2. Reserva memoria de usuario.
3. Mapea payload y transfiere al runtime BAP.

## Seguridad
- Firma opcional Ed25519 (v2 planificado)
- Permisos declarativos: fs.read, fs.write, rtc.read
