use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LLMProvider {
    OpenAI,
    Anthropic,
    Gemini,
    Mistral,
    Custom(&'static str),
}

#[derive(Debug, Clone)]
pub struct TemperatureRange {
    pub min: f32,
    pub max: f32,
}

// T_mapped = T_min + base_normalized × (T_max - T_min)
// ToDo: in future custom base range will be supported that will use the formula:
//  mapped = target_min + ((value - base_min) / (base_max - base_min)) * (target_max - target_min)
impl TemperatureRange {
    pub fn remap(&self, normalized: f32) -> f32 {
        let clamped = normalized.clamp(0.0, 1.0);
        self.min + clamped * (self.max - self.min)
    }
}

#[derive(Debug, Clone)]
pub struct TemperatureScaler {
    /// Normalized temperature in range 0.0 - 1.0
    pub base_normalized_temperature: f32,

    /// Actual temperature range per provider
    pub provider_ranges: HashMap<LLMProvider, TemperatureRange>,
}

impl TemperatureScaler {
    pub fn new(base_normalized_temperature: f32) -> Self {
        let mut provider_ranges = HashMap::new();
        provider_ranges.insert(LLMProvider::OpenAI, TemperatureRange { min: 0.0, max: 2.0 });
        provider_ranges.insert(LLMProvider::Anthropic, TemperatureRange { min: 0.0, max: 1.0 });
        provider_ranges.insert(LLMProvider::Gemini, TemperatureRange { min: 0.0, max: 1.2 });
        provider_ranges.insert(LLMProvider::Mistral, TemperatureRange { min: 0.0, max: 1.0 });

        Self {
            base_normalized_temperature: base_normalized_temperature.clamp(0.0, 1.0),
            provider_ranges,
        }
    }

    pub fn for_provider(&self, provider: LLMProvider) -> f32 {
        let range = self
            .provider_ranges
            .get(&provider)
            .unwrap_or(&TemperatureRange { min: 0.0, max: 1.0 });
        range.remap(self.base_normalized_temperature)
    }

    pub fn set_range(&mut self, provider: LLMProvider, min: f32, max: f32) {
        self.provider_ranges.insert(provider, TemperatureRange { min, max });
    }
}

pub fn example() {
    let scaler = TemperatureScaler::new(0.7); // normalized

    println!("OpenAI temp: {:.2}", scaler.for_provider(LLMProvider::OpenAI));      // 1.40
    println!("Anthropic temp: {:.2}", scaler.for_provider(LLMProvider::Anthropic)); // 0.70
    println!("Gemini temp: {:.2}", scaler.for_provider(LLMProvider::Gemini));       // 0.84
    println!("Mistral temp: {:.2}", scaler.for_provider(LLMProvider::Mistral));     // 0.70
}
