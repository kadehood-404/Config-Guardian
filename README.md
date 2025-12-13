# Config Guardian

A fast, strict, and opinionated configuration validator for YAML, JSON, and TOML.

Built for CI/CD pipelines that refuse to accept broken, sloppy, or insecure configuration files.

---

## Why Config Guardian Exists

Most configuration validation tools are either permissive, slow, or vague. They allow undocumented fields, fail with unhelpful error messages, or quietly accept configurations that will break at runtime.

Config Guardian takes a different approach:

* **Strict by default**: undocumented fields are rejected.
* **Explicit schemas**: JSON Schema is the source of truth.
* **Actionable errors**: failures explain exactly what went wrong and where.
* **Fast execution**: designed to run in milliseconds for CI/CD usage.

If a configuration passes Config Guardian, it is structurally sound, schema-compliant, and ready to ship.

---

## Features

* ⚡ **Zero-latency validation** using Rust
* 📄 **Multi-format support**: YAML, JSON, and TOML
* 🧱 **Strict JSON Schema enforcement** (`additionalProperties: false` by default)
* 🧠 **Descriptive, opinionated error messages** with file, line, and JSON path
* ✨ **Automatic default injection** from schema definitions
* 🔐 **Basic security heuristics** for unsafe or suspicious configuration patterns
* 🤖 **CI/CD friendly** with non-zero exit codes on failure
* 🧪 **Recursive validation** for large config trees

---

## Quick Start

### Validate a Configuration File

```bash
guardian validate config.yaml schema.json
```

* **Exit code 0**: configuration is valid
* **Non-zero exit code**: validation failed

### Example Error Output

```text
config.yaml:42
Path: services[3].ports.host
Expected integer, found string
Stop using words for numbers. Computers are literal.
```

---

## Supported Formats

Config Guardian natively supports:

* YAML (`.yaml`, `.yml`)
* JSON (`.json`)
* TOML (`.toml`)

All formats are internally normalized before validation, ensuring consistent behavior regardless of input type.

---

## Strict Schema Enforcement

By default, Config Guardian enforces:

* Required fields
* Correct data types
* Proper array and object structure
* No undocumented fields (`additionalProperties: false` is automatically applied unless explicitly overridden)

This prevents configuration drift and accidental misuse.

---

## Default Injection and Cleaning

If a schema defines default values, Config Guardian can automatically inject missing fields and output a cleaned configuration file.

```bash
guardian validate config.yaml schema.json --inject-defaults
```

This allows teams to maintain minimal user-facing configs while guaranteeing complete runtime configurations.

---

## Schema Generation

Generate a draft JSON Schema directly from a valid configuration file:

```bash
guardian generate-schema config.yaml > schema.json
```

This feature accelerates initial setup and provides a strong starting point for schema authoring.

---

## Security Heuristics (Optional)

Config Guardian can perform simple security checks during validation:

* Detects empty or weak secrets (e.g., `""`, `null`, `"12345"`)
* Flags services running as root
* Warns about privileged ports (<1024) without explicit approval

```bash
guardian validate config.yaml schema.json --security
```

Security checks emit warnings by default and can be made fatal with strict flags.

---

## CI/CD and GitHub Actions

Config Guardian is designed for automation and continuous integration.

A prebuilt GitHub Action allows validation to run automatically on pull requests and merges, blocking invalid configurations before deployment.

---

## Non-Goals

Config Guardian is intentionally focused. It is **not**:

* A secrets manager
* A configuration linter or formatter
* A runtime configuration loader
* A replacement for application-level validation

It validates structure and intent, nothing more.

---

## License

This project is licensed under the MIT License.

---

## Status

Config Guardian is under active development. The interface is stabilizing, and breaking changes may occur prior to v1.0.

Contributions, feedback, and issues are welcome.
