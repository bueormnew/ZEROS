# Instalación de zerOS

1. Compila el ISO con `./scripts/build_iso.sh`.
2. Arranca una VM (QEMU/VirtualBox) con `zerOS.iso`.
3. En hardware real, flashea el ISO a USB con `dd` o Rufus.
4. Sigue el asistente de particionado (objetivo: GPT + partición de sistema zerFS).
