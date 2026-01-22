# System Monitor 📊

[![Rust](https://img.shields.io/badge/Rust-1.70+-orange.svg)](https://www.rust-lang.org/)
[![Licencia](https://img.shields.io/badge/Licencia-MIT-blue.svg)](LICENSE)
[![Versión](https://img.shields.io/badge/Versión-0.4.0-green.svg)](https://github.com/SoftDryzz/system_monitor/releases)

**System Monitor** es una herramienta CLI ligera y multiplataforma para monitoreo del sistema en tiempo real con escalado inteligente y feedback visual con colores. Construida con Rust para máximo rendimiento y mínimo uso de recursos.

[English](README.md) | [Español](README_ES.md)

---

## ✨ Características

* 🖥️ **Monitoreo de CPU** - Visualización inteligente con modos compacto/detallado
* 💾 **Seguimiento de Memoria** - Uso de RAM con indicadores visuales
* 💽 **Uso de Disco** - Espacio usado en todas las unidades
* 📊 **Monitoreo de Procesos** - Top procesos por CPU y memoria
* 🌐 **Estadísticas de Red** - Velocidad de descarga/subida en tiempo real
* 🎨 **UI con Colores** - Verde/Amarillo/Rojo según nivel de uso
* ⏱️ **Tiempo de Actividad** - Rastrea el tiempo de ejecución del sistema
* 🔄 **Modo Watch** - Actualizaciones continuas en tiempo real
* 🚀 **Súper Rápido** - Rendimiento nativo de Rust
* 🌍 **Multiplataforma** - Windows, Linux y macOS

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
# Instantánea única con colores
sysmon

# Vista detallada (todos los núcleos, más procesos)
sysmon --detailed

# Modo watch (actualizaciones continuas)
sysmon --watch

# Modo watch con detalles e intervalo personalizado
sysmon --watch --detailed --interval 2
```

---

## 📊 Ejemplo de Salida

```
╭─────────────────────────────────────────────────────────╮
│   System Monitor v0.4.0                                 │
╰─────────────────────────────────────────────────────────╯

CPU:  34.5% (8 núcleos)  [██████░░░░░░░░░░░░░░]    ← Amarillo (moderado)
  Top 3: Núcleo 0 (52%) Núcleo 4 (48%) Núcleo 7 (46%)

Memoria:  8.34/16.00 GB (52.1%)  [██████████░░░░░░░░░░]  ← Amarillo

Uso de Disco:
  C:\       450.0/1000.0 GB ( 45.0%)  [███████░░░░░░░░]  ← Amarillo
  D:\       200.0/ 500.0 GB ( 40.0%)  [██████░░░░░░░░░]  ← Amarillo

Red:
  ↓ Descarga: 5.2 MB/s    ← Verde (activo)
  ↑ Subida:   1.3 MB/s    ← Verde (activo)
  Total RX:   2.5 GB
  Total TX:   850 MB

Top 5 Procesos (por CPU):
   1. firefox.exe        PID  1234   25.2%    2.5 GB  ← Rojo (alto CPU)
   2. chrome.exe         PID  5678   15.1%    1.8 GB
   3. Code.exe           PID  9012   10.4%    1.2 GB

Top 3 Procesos (por Memoria):
   1. chrome.exe         PID  5678   15.1%    2.8 GB  ← Rojo (alta memoria)
   2. firefox.exe        PID  1234   25.2%    2.5 GB

Tiempo Activo: 2 días, 0 horas, 32 minutos
```

---

## 🎨 Código de Colores

| Color | Nivel de Uso | Significado |
|-------|--------------|-------------|
| 🟢 Verde | 0-30% | Saludable, uso bajo |
| 🟡 Amarillo | 30-70% | Normal, uso moderado |
| 🔴 Rojo | 70-100% | Uso alto, atención necesaria |

**Aplicado a:** CPU, Memoria, Disco y métricas de Procesos

---

## 💻 Comandos

| Comando | Descripción |
|---------|-------------|
| `sysmon` | Vista compacta con colores |
| `sysmon --detailed` | Vista detallada (todos los núcleos) |
| `sysmon --watch` | Actualizaciones continuas |
| `sysmon -w -d -i 3` | Watch detallado, intervalo 3s |
| `sysmon --help` | Mostrar ayuda |
| `sysmon --version` | Mostrar versión |

---

## 📦 Novedades en v0.4.0

### 🌐 Estadísticas de Red
- Velocidad de descarga/subida en tiempo real
- Conversión automática de unidades (B/s → GB/s)
- Total de bytes recibidos/transmitidos

### 🎨 UI con Colores
- Indicadores visuales de salud del sistema
- Feedback instantáneo del estado
- Apariencia profesional en terminal

[Changelog Completo](CHANGELOG.md) | [Guía de Usuario](USER_GUIDE.md)

---

## 🗺️ Historial de Versiones

| Versión | Características |
|---------|-----------------|
| **v0.4.0** | Stats de red, UI con colores |
| v0.3.0 | Monitoreo de procesos, Escalado inteligente |
| v0.2.1 | Monitoreo de disco |
| v0.2.0 | Modo watch, Actualizaciones continuas |
| v0.1.0 | Lanzamiento inicial |

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
├── monitor/
│   ├── cpu.rs           # Monitoreo de CPU
│   ├── memory.rs        # Monitoreo de memoria
│   ├── disk.rs          # Monitoreo de disco
│   ├── network.rs       # Monitoreo de red
│   ├── process.rs       # Monitoreo de procesos
│   └── system.rs        # Fachada del sistema
└── display/
    └── formatter.rs     # Formato de salida + colores
```

---

## 🤝 Contribuir

¡Contribuciones bienvenidas! Ver [USER_GUIDE.md](USER_GUIDE.md) para detalles.

1. Fork el repositorio
2. Crear rama: `git checkout -b feature/increible`
3. Commit cambios: `git commit -m 'feat: añadir feature increíble'`
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
- [colored](https://github.com/colored-rs/colored) - Colores en terminal
- [ctrlc](https://github.com/Detegr/rust-ctrlc) - Manejo de señales

---

**⭐ ¡Dale una estrella si te resulta útil!**