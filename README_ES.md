# System Monitor 📊

[![Rust](https://img.shields.io/badge/Rust-1.70+-orange.svg)](https://www.rust-lang.org/)
[![Licencia](https://img.shields.io/badge/Licencia-MIT-blue.svg)](LICENSE)
[![Versión](https://img.shields.io/badge/Versión-0.3.0-green.svg)](https://github.com/SoftDryzz/system_monitor/releases)

**System Monitor** es una herramienta CLI ligera y multiplataforma para monitoreo del sistema en tiempo real con escalado inteligente. Construida con Rust para máximo rendimiento y mínimo uso de recursos.

[English](README.md) | [Español](README_ES.md)

---

## ✨ Características

* 🖥️ **Monitoreo de CPU** - Visualización inteligente con modos compacto/detallado
* 💾 **Seguimiento de Memoria** - Uso de RAM con indicadores visuales
* 💽 **Uso de Disco** - Espacio usado en todas las unidades
* 📊 **Monitoreo de Procesos** - Top procesos por CPU y memoria
* ⏱️ **Tiempo de Actividad** - Rastrea el tiempo de ejecución del sistema
* 🔄 **Modo Watch** - Actualizaciones continuas en tiempo real
* 🎨 **CLI Hermosa** - Formateada con barras de progreso
* 🚀 **Súper Rápido** - Rendimiento nativo de Rust
* 🌐 **Multiplataforma** - Windows, Linux y macOS

---

## 🚀 Inicio Rápido

### Instalación

```bash
# Clonar y compilar
git clone https://github.com/SoftDryzz/system_monitor.git
cd system_monitor
cargo build --release

# Ejecutar
./target/release/sysmon
```

O descarga binarios precompilados desde [Releases](https://github.com/SoftDryzz/system_monitor/releases).

### Uso Básico

```bash
# Instantánea única (modo compacto)
sysmon

# Vista detallada (todos los núcleos, más procesos)
sysmon --detailed

# Modo watch (actualizaciones continuas)
sysmon --watch

# Modo watch con detalles
sysmon --watch --detailed --interval 2
```

---

## 📊 Ejemplo de Salida

### Modo Compacto (Por Defecto)
```
╭─────────────────────────────────────────────────────────╮
│   System Monitor v0.3.0                                 │
╰─────────────────────────────────────────────────────────╯

CPU:  34.5% (8 núcleos)  [██████░░░░░░░░░░░░░░]
  Top 3: Núcleo 0 (52%) Núcleo 4 (48%) Núcleo 7 (46%)

Memoria:  8.34/16.00 GB (52.1%)  [██████████░░░░░░░░░░]

Uso de Disco:
  C:\       450.0/1000.0 GB ( 45.0%)  [███████░░░░░░░░]

Top 5 Procesos (por CPU):
   1. firefox.exe        PID  1234   25.2%    2.5 GB
   2. chrome.exe         PID  5678   15.1%    1.8 GB
   3. Code.exe           PID  9012   10.4%    1.2 GB

Top 3 Procesos (por Memoria):
   1. chrome.exe         PID  5678   15.1%    2.8 GB
   2. firefox.exe        PID  1234   25.2%    2.5 GB

Tiempo Activo: 2 días, 0 horas, 32 minutos
```

---

## 💻 Comandos

| Comando | Descripción |
|---------|-------------|
| `sysmon` | Vista compacta (por defecto) |
| `sysmon --detailed` | Vista detallada (todos los núcleos) |
| `sysmon --watch` | Actualizaciones continuas |
| `sysmon -w -i 3` | Watch con intervalo de 3s |
| `sysmon --help` | Mostrar ayuda |
| `sysmon --version` | Mostrar versión |

---

## 📦 Novedades en v0.3.0

### 🎉 Características Principales
- **Monitoreo de Procesos**: Top procesos por CPU y memoria
- **Visualización Inteligente de CPU**: Modo compacto para sistemas con muchos núcleos
- **Modo Detallado**: Salida verbose opcional con flag `--detailed`

### 🔧 Mejoras
- Escalado inteligente para sistemas con 4 a 128+ núcleos
- Mejor formato de memoria (conversión automática MB/GB)
- Argumentos CLI mejorados

[Changelog Completo](CHANGELOG.md) | [Guía de Usuario](USER_GUIDE.md)

---

## 🗺️ Historial de Versiones

- **v0.3.0** (Actual) - Monitoreo de procesos y escalado inteligente
- **v0.2.1** - Monitoreo de uso de disco
- **v0.2.0** - Modo watch y actualizaciones continuas
- **v0.1.0** - Lanzamiento inicial

---

## 🛠️ Desarrollo

### Compilar
```bash
cargo build --release
```

### Probar
```bash
cargo test
cargo clippy
cargo fmt
```

### Estructura del Proyecto
```
src/
├── main.rs              # Punto de entrada
├── cli.rs               # Análisis CLI
├── monitor/             # Monitoreo del sistema
│   ├── cpu.rs
│   ├── memory.rs
│   ├── disk.rs
│   ├── process.rs
│   └── system.rs
└── display/
    └── formatter.rs     # Formato de salida
```

---

## 🤝 Contribuir

¡Contribuciones bienvenidas! Ver [USER_GUIDE.md](USER_GUIDE.md) para detalles.

### Guía Rápida
1. Fork el repositorio
2. Crear rama: `git checkout -b feature/increible`
3. Commit cambios: `git commit -m 'feat: añadir característica increíble'`
4. Push y crear Pull Request

---

## 📄 Licencia

Licencia MIT - ver archivo [LICENSE](LICENSE).

---

## 👤 Autor

**SoftDryzz**
- GitHub: [@SoftDryzz](https://github.com/SoftDryzz)
- Proyecto: [system_monitor](https://github.com/SoftDryzz/system_monitor)

---

## 🙏 Agradecimientos

- [sysinfo](https://github.com/GuillaumeGomez/sysinfo) - Información del sistema
- [clap](https://github.com/clap-rs/clap) - Análisis CLI
- [crossterm](https://github.com/crossterm-rs/crossterm) - Manipulación de terminal
- [ctrlc](https://github.com/Detegr/rust-ctrlc) - Manejo de señales

---

**⭐ ¡Dale una estrella si te resulta útil!**
