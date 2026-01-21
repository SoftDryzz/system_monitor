# System Monitor 📊

[![Rust](https://img.shields.io/badge/Rust-1.70+-orange.svg)](https://www.rust-lang.org/)
[![Licencia](https://img.shields.io/badge/Licencia-MIT-blue.svg)](LICENSE)
[![Estado](https://img.shields.io/badge/Estado-En%20Desarrollo-yellow.svg)]()

**System Monitor** es una herramienta CLI ligera y multiplataforma para monitoreo del sistema en tiempo real. Construida con Rust para máximo rendimiento y mínimo uso de recursos.

[English](README.md) | [Español](README_ES.md)

> ⚠️ **En Desarrollo Activo:** Este proyecto está en construcción. Las características y APIs pueden cambiar.

---

## ✨ Características

* 🖥️ **Monitoreo de CPU** - Uso de CPU en tiempo real por núcleo con barras visuales
* 💾 **Seguimiento de Memoria** - Uso de RAM con estadísticas detalladas
* ⏱️ **Tiempo de Actividad** - Rastrea cuánto tiempo ha estado funcionando tu sistema
* 🎨 **Salida CLI Hermosa** - Formateada con colores y barras de progreso
* 🚀 **Súper Rápido** - Escrito en Rust para rendimiento nativo
* 🌐 **Multiplataforma** - Funciona en Windows, Linux y macOS

---

## 📋 Requisitos

* Rust 1.70 o superior
* Cargo (viene con Rust)

---

## 🚀 Instalación

### Desde el Código Fuente

```bash
# Clonar el repositorio
git clone https://github.com/SoftDryzz/system_monitor.git
cd system_monitor

# Compilar el proyecto
cargo build --release

# El binario estará en target/release/sysmon
```

### Instalación Rápida (Linux/macOS)

```bash
# Compilar e instalar en ~/.cargo/bin
cargo install --path .

# Ahora puedes ejecutarlo desde cualquier lugar
sysmon
```

### Windows

```powershell
# Compilar
cargo build --release

# El ejecutable estará en target\release\sysmon.exe
# Añadir al PATH o copiar a la ubicación deseada
```

---

## 💻 Uso

### Uso Básico

```bash
# Mostrar métricas actuales del sistema
sysmon

# Ejemplo de salida:
╭─────────────────────────────────────╮
│      System Monitor v0.1.0          │
╰─────────────────────────────────────╯

CPU Usage:  34.5%  [██████░░░░░░░░░░░░░░]
  Core  0:   45.2%  [██████░░░░░░░░░]
  Core  1:   28.3%  [████░░░░░░░░░░░]
  Core  2:   31.1%  [████░░░░░░░░░░░]
  Core  3:   33.8%  [█████░░░░░░░░░░]

Memory:     8.34/16.00 GB (52.1%)
            [██████████░░░░░░░░░░]

Uptime:     2 días, 0 horas, 32 minutos
```

### Comandos Disponibles

| Comando | Descripción |
|---------|-------------|
| `sysmon` | Mostrar métricas actuales del sistema |
| `sysmon --help` | Mostrar información de ayuda |
| `sysmon --version` | Mostrar versión |

---

## 🏗️ Estructura del Proyecto

```
system_monitor/
├── Cargo.toml              # Configuración del proyecto
├── README.md               # Documentación en inglés
├── README_ES.md            # Este archivo
├── LICENSE                 # Licencia MIT
├── docs/
│   └── USER_GUIDE.md       # Guía de usuario detallada
└── src/
    ├── main.rs             # Punto de entrada
    ├── cli.rs              # Parseo de argumentos CLI
    ├── monitor/            # Lógica de monitoreo
    │   ├── mod.rs
    │   ├── cpu.rs          # Monitoreo de CPU
    │   ├── memory.rs       # Monitoreo de memoria
    │   └── system.rs       # Info del sistema
    └── display/            # Formateo de salida
        ├── mod.rs
        └── formatter.rs    # Utilidades de visualización
```

---

## 🛠️ Desarrollo

### Compilar

```bash
# Compilación de depuración
cargo build

# Compilación de release (optimizada)
cargo build --release
```

### Ejecutar

```bash
# Ejecutar en modo desarrollo
cargo run

# Ejecutar versión release
cargo run --release
```

### Verificar

```bash
# Verificar errores sin compilar
cargo check

# Ejecutar con advertencias
cargo clippy
```

---

## 🗺️ Hoja de Ruta

### ✅ Versión 0.1.0 (Actual)
- [x] Monitoreo básico de CPU
- [x] Seguimiento de uso de memoria
- [x] Tiempo de actividad del sistema
- [x] Salida CLI formateada
- [x] Soporte multiplataforma

### 🔨 Versión 0.2.0 (Próxima)
- [ ] Monitoreo de uso de disco
- [ ] Estadísticas de red
- [ ] Lista de procesos (top N por CPU/RAM)
- [ ] Modo watch (actualizaciones continuas)
- [ ] Argumentos CLI con clap

### 🚀 Versión 0.3.0 (Futuro)
- [ ] TUI interactivo (Terminal UI)
- [ ] Exportar métricas a JSON/CSV
- [ ] Seguimiento de datos históricos
- [ ] Alertas y notificaciones
- [ ] Intervalos de actualización personalizados

---

## 🤝 Contribuir

¡Las contribuciones son bienvenidas! Por favor, sigue estos pasos:

1. Haz un fork del repositorio
2. Crea una rama de característica (`git checkout -b feature/caracteristica-increible`)
3. Haz commit de tus cambios (`git commit -m 'feat: añadir característica increíble'`)
4. Haz push a la rama (`git push origin feature/caracteristica-increible`)
5. Abre un Pull Request

### Convención de Commits

Seguimos [Conventional Commits](https://www.conventionalcommits.org/):

- `feat:` Nuevas características
- `fix:` Corrección de bugs
- `docs:` Cambios en documentación
- `refactor:` Refactorización de código
- `test:` Añadir tests
- `chore:` Tareas de mantenimiento

---

## 📄 Licencia

Este proyecto está licenciado bajo la Licencia MIT - ver el archivo [LICENSE](LICENSE) para más detalles.

---

## 👤 Autor

**SoftDryzz**

* GitHub: [@SoftDryzz](https://github.com/SoftDryzz)
* Portafolio: [Más proyectos](https://github.com/SoftDryzz?tab=repositories)

---

## 🙏 Agradecimientos

* [sysinfo](https://github.com/GuillaumeGomez/sysinfo) - Biblioteca de información del sistema
* [clap](https://github.com/clap-rs/clap) - Parser de argumentos de línea de comandos
* La comunidad de Rust por su excelente documentación y herramientas

---

## 📚 Aprende Más

* [El Libro de Rust](https://doc.rust-lang.org/book/) - Aprende Rust
* [Guía de Usuario](docs/USER_GUIDE.md) - Guía de uso detallada
* [Rust by Example](https://doc.rust-lang.org/rust-by-example/) - Ejemplos de Rust

---

**⭐ ¡Si te gusta este proyecto, dale una estrella en GitHub!**
