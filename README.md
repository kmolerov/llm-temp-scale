# llm-temp-scale

`llm-temp-scale` is a multiplatform library for normalizing and converting a base, normalized temperature value into provider-specific temperature values for large language models (LLMs).

---

## Overview

Different LLM providers (OpenAI, Anthropic, Gemini, Mistral, and others) interpret and use temperature parameters with different valid ranges and scales. This library allows you to work with a consistent normalized temperature value (default range 0.0 to 1.0) and easily convert it to the appropriate temperature scale for each LLM provider.

By standardizing temperature handling across models, `llm-temp-scale` simplifies integration with multiple LLM APIs and helps maintain consistent behavior when switching or comparing models.

---

## Features

- Define a base normalized temperature (0.0 - 1.0)
- Convert the normalized temperature to provider-specific temperature ranges
- Easily extend or customize ranges for new or custom providers
- Lightweight, zero dependencies core logic
- Consistent behavior across platforms

---

## Multiplatform Support

This repository provides implementations in multiple programming languages to support cross-ecosystem development:

- Rust
- Go
- JavaScript / TypeScript (npm)
- Python (PyPI)
- Java (Maven)

This monorepo structure enables shared design, testing, and documentation to keep all versions aligned.

---

## Getting Started

See each language-specific folder for usage examples and installation instructions.

---

## Contribution

Contributions, issues, and feature requests are welcome! Feel free to open pull requests or issues in your preferred language implementation.

---

## License

MIT
