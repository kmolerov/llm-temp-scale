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

## Getting Started


---

## Dependencies
The only create this library uses is log create, which is a logging facade that works with different logging libraries. If consumer of the llm-temp-scale library does not have logging configured this library will not cause any problems. The log crate has a no-op logger by default.

---

## Contribution

Contributions, issues, and feature requests are welcome! Feel free to open pull requests or issues in your preferred language implementation.

---

## License

MIT
