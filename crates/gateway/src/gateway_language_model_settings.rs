use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumString};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum GatewayModelId {
    Alibaba(AlibabaModel),
    Amazon(AmazonModel),
    Anthropic(AnthropicModel),
    ArceeAi(ArceeAiModel),
    Cohere(CohereModel),
    Deepseek(DeepseekModel),
    Google(GoogleModel),
    Inception(InceptionModel),
    Meituan(MeituanModel),
    Meta(MetaModel),
    Minimax(MinimaxModel),
    Mistral(MistralModel),
    MoonshotAi(MoonshotAiModel),
    Morph(MorphModel),
    OpenAi(OpenAiModel),
    Perplexity(PerplexityModel),
    PrimeIntellect(PrimeIntellectModel),
    Stealth(StealthModel),
    Vercel(VercelModel),
    Xai(XaiModel),
    Zai(ZaiModel),
    Custom(String),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, EnumString, Display)]
pub enum AlibabaModel {
    #[strum(serialize = "alibaba/qwen-3-14b")]
    Qwen3_14b,
    #[strum(serialize = "alibaba/qwen-3-235b")]
    Qwen3_235b,
    #[strum(serialize = "alibaba/qwen-3-30b")]
    Qwen3_30b,
    #[strum(serialize = "alibaba/qwen-3-32b")]
    Qwen3_32b,
    #[strum(serialize = "alibaba/qwen3-235b-a22b-thinking")]
    Qwen3_235bA22bThinking,
    #[strum(serialize = "alibaba/qwen3-coder")]
    Qwen3Coder,
    #[strum(serialize = "alibaba/qwen3-coder-30b-a3b")]
    Qwen3Coder30bA3b,
    #[strum(serialize = "alibaba/qwen3-coder-plus")]
    Qwen3CoderPlus,
    #[strum(serialize = "alibaba/qwen3-max")]
    Qwen3Max,
    #[strum(serialize = "alibaba/qwen3-max-preview")]
    Qwen3MaxPreview,
    #[strum(serialize = "alibaba/qwen3-next-80b-a3b-instruct")]
    Qwen3Next80bA3bInstruct,
    #[strum(serialize = "alibaba/qwen3-next-80b-a3b-thinking")]
    Qwen3Next80bA3bThinking,
    #[strum(serialize = "alibaba/qwen3-vl-instruct")]
    Qwen3VlInstruct,
    #[strum(serialize = "alibaba/qwen3-vl-thinking")]
    Qwen3VlThinking,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, EnumString, Display)]
pub enum AmazonModel {
    #[strum(serialize = "amazon/nova-lite")]
    NovaLite,
    #[strum(serialize = "amazon/nova-micro")]
    NovaMicro,
    #[strum(serialize = "amazon/nova-pro")]
    NovaPro,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, EnumString, Display)]
pub enum AnthropicModel {
    #[strum(serialize = "anthropic/claude-3-haiku")]
    Claude3Haiku,
    #[strum(serialize = "anthropic/claude-3-opus")]
    Claude3Opus,
    #[strum(serialize = "anthropic/claude-3.5-haiku")]
    Claude35Haiku,
    #[strum(serialize = "anthropic/claude-3.5-sonnet")]
    Claude35Sonnet,
    #[strum(serialize = "anthropic/claude-3.5-sonnet-20240620")]
    Claude35Sonnet20240620,
    #[strum(serialize = "anthropic/claude-3.7-sonnet")]
    Claude37Sonnet,
    #[strum(serialize = "anthropic/claude-haiku-4.5")]
    ClaudeHaiku45,
    #[strum(serialize = "anthropic/claude-opus-4")]
    ClaudeOpus4,
    #[strum(serialize = "anthropic/claude-opus-4.1")]
    ClaudeOpus41,
    #[strum(serialize = "anthropic/claude-opus-4.5")]
    ClaudeOpus45,
    #[strum(serialize = "anthropic/claude-sonnet-4")]
    ClaudeSonnet4,
    #[strum(serialize = "anthropic/claude-sonnet-4.5")]
    ClaudeSonnet45,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, EnumString, Display)]
pub enum ArceeAiModel {
    #[strum(serialize = "arcee-ai/trinity-mini")]
    TrinityMini,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, EnumString, Display)]
pub enum CohereModel {
    #[strum(serialize = "cohere/command-a")]
    CommandA,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, EnumString, Display)]
pub enum DeepseekModel {
    #[strum(serialize = "deepseek/deepseek-r1")]
    DeepseekR1,
    #[strum(serialize = "deepseek/deepseek-v3")]
    DeepseekV3,
    #[strum(serialize = "deepseek/deepseek-v3.1")]
    DeepseekV31,
    #[strum(serialize = "deepseek/deepseek-v3.1-terminus")]
    DeepseekV31Terminus,
    #[strum(serialize = "deepseek/deepseek-v3.2-exp")]
    DeepseekV32Exp,
    #[strum(serialize = "deepseek/deepseek-v3.2-exp-thinking")]
    DeepseekV32ExpThinking,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, EnumString, Display)]
pub enum GoogleModel {
    #[strum(serialize = "google/gemini-2.0-flash")]
    Gemini20Flash,
    #[strum(serialize = "google/gemini-2.0-flash-lite")]
    Gemini20FlashLite,
    #[strum(serialize = "google/gemini-2.5-flash")]
    Gemini25Flash,
    #[strum(serialize = "google/gemini-2.5-flash-image")]
    Gemini25FlashImage,
    #[strum(serialize = "google/gemini-2.5-flash-image-preview")]
    Gemini25FlashImagePreview,
    #[strum(serialize = "google/gemini-2.5-flash-lite")]
    Gemini25FlashLite,
    #[strum(serialize = "google/gemini-2.5-flash-lite-preview-09-2025")]
    Gemini25FlashLitePreview092025,
    #[strum(serialize = "google/gemini-2.5-flash-preview-09-2025")]
    Gemini25FlashPreview092025,
    #[strum(serialize = "google/gemini-2.5-pro")]
    Gemini25Pro,
    #[strum(serialize = "google/gemini-3-pro-preview")]
    Gemini3ProPreview,
    #[strum(serialize = "google/gemini-3-pro-image")]
    Gemini3ProImage,
    #[strum(serialize = "google/gemini-3-flash")]
    Gemini3Flash,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, EnumString, Display)]
pub enum InceptionModel {
    #[strum(serialize = "inception/mercury-coder-small")]
    MercuryCoderSmall,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, EnumString, Display)]
pub enum MeituanModel {
    #[strum(serialize = "meituan/longcat-flash-chat")]
    LongcatFlashChat,
    #[strum(serialize = "meituan/longcat-flash-thinking")]
    LongcatFlashThinking,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, EnumString, Display)]
pub enum MetaModel {
    #[strum(serialize = "meta/llama-3.1-70b")]
    Llama31_70b,
    #[strum(serialize = "meta/llama-3.1-8b")]
    Llama31_8b,
    #[strum(serialize = "meta/llama-3.2-11b")]
    Llama32_11b,
    #[strum(serialize = "meta/llama-3.2-1b")]
    Llama32_1b,
    #[strum(serialize = "meta/llama-3.2-3b")]
    Llama32_3b,
    #[strum(serialize = "meta/llama-3.2-90b")]
    Llama32_90b,
    #[strum(serialize = "meta/llama-3.3-70b")]
    Llama33_70b,
    #[strum(serialize = "meta/llama-4-maverick")]
    Llama4Maverick,
    #[strum(serialize = "meta/llama-4-scout")]
    Llama4Scout,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, EnumString, Display)]
pub enum MinimaxModel {
    #[strum(serialize = "minimax/minimax-m2")]
    MinimaxM2,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, EnumString, Display)]
pub enum MistralModel {
    #[strum(serialize = "mistral/codestral")]
    Codestral,
    #[strum(serialize = "mistral/devstral-small")]
    DevstralSmall,
    #[strum(serialize = "mistral/magistral-medium")]
    MagistralMedium,
    #[strum(serialize = "mistral/magistral-medium-2506")]
    MagistralMedium2506,
    #[strum(serialize = "mistral/magistral-small")]
    MagistralSmall,
    #[strum(serialize = "mistral/magistral-small-2506")]
    MagistralSmall2506,
    #[strum(serialize = "mistral/ministral-3b")]
    Ministral3b,
    #[strum(serialize = "mistral/ministral-8b")]
    Ministral8b,
    #[strum(serialize = "mistral/mistral-large")]
    MistralLarge,
    #[strum(serialize = "mistral/mistral-medium")]
    MistralMedium,
    #[strum(serialize = "mistral/mistral-small")]
    MistralSmall,
    #[strum(serialize = "mistral/mixtral-8x22b-instruct")]
    Mixtral8x22bInstruct,
    #[strum(serialize = "mistral/pixtral-12b")]
    Pixtral12b,
    #[strum(serialize = "mistral/pixtral-large")]
    PixtralLarge,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, EnumString, Display)]
pub enum MoonshotAiModel {
    #[strum(serialize = "moonshotai/kimi-k2")]
    KimiK2,
    #[strum(serialize = "moonshotai/kimi-k2-0905")]
    KimiK20905,
    #[strum(serialize = "moonshotai/kimi-k2-thinking")]
    KimiK2Thinking,
    #[strum(serialize = "moonshotai/kimi-k2-thinking-turbo")]
    KimiK2ThinkingTurbo,
    #[strum(serialize = "moonshotai/kimi-k2-turbo")]
    KimiK2Turbo,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, EnumString, Display)]
pub enum MorphModel {
    #[strum(serialize = "morph/morph-v3-fast")]
    MorphV3Fast,
    #[strum(serialize = "morph/morph-v3-large")]
    MorphV3Large,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, EnumString, Display)]
pub enum OpenAiModel {
    #[strum(serialize = "openai/gpt-3.5-turbo")]
    Gpt35Turbo,
    #[strum(serialize = "openai/gpt-3.5-turbo-instruct")]
    Gpt35TurboInstruct,
    #[strum(serialize = "openai/gpt-4-turbo")]
    Gpt4Turbo,
    #[strum(serialize = "openai/gpt-4.1")]
    Gpt41,
    #[strum(serialize = "openai/gpt-4.1-mini")]
    Gpt41Mini,
    #[strum(serialize = "openai/gpt-4.1-nano")]
    Gpt41Nano,
    #[strum(serialize = "openai/gpt-4o")]
    Gpt4o,
    #[strum(serialize = "openai/gpt-4o-mini")]
    Gpt4oMini,
    #[strum(serialize = "openai/gpt-5")]
    Gpt5,
    #[strum(serialize = "openai/gpt-5-chat")]
    Gpt5Chat,
    #[strum(serialize = "openai/gpt-5-codex")]
    Gpt5Codex,
    #[strum(serialize = "openai/gpt-5-mini")]
    Gpt5Mini,
    #[strum(serialize = "openai/gpt-5-nano")]
    Gpt5Nano,
    #[strum(serialize = "openai/gpt-5-pro")]
    Gpt5Pro,
    #[strum(serialize = "openai/gpt-5.1-codex")]
    Gpt51Codex,
    #[strum(serialize = "openai/gpt-5.1-codex-mini")]
    Gpt51CodexMini,
    #[strum(serialize = "openai/gpt-5.1-instant")]
    Gpt51Instant,
    #[strum(serialize = "openai/gpt-5.1-thinking")]
    Gpt51Thinking,
    #[strum(serialize = "openai/gpt-5.2")]
    Gpt52,
    #[strum(serialize = "openai/gpt-5.2-chat-latest")]
    Gpt52ChatLatest,
    #[strum(serialize = "openai/gpt-5.2-pro")]
    Gpt52Pro,
    #[strum(serialize = "openai/gpt-oss-120b")]
    GptOss120b,
    #[strum(serialize = "openai/gpt-oss-20b")]
    GptOss20b,
    #[strum(serialize = "openai/gpt-oss-safeguard-20b")]
    GptOssSafeguard20b,
    #[strum(serialize = "openai/o1")]
    O1,
    #[strum(serialize = "openai/o3")]
    O3,
    #[strum(serialize = "openai/o3-deep-research")]
    O3DeepResearch,
    #[strum(serialize = "openai/o3-mini")]
    O3Mini,
    #[strum(serialize = "openai/o4-mini")]
    O4Mini,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, EnumString, Display)]
pub enum PerplexityModel {
    #[strum(serialize = "perplexity/sonar")]
    Sonar,
    #[strum(serialize = "perplexity/sonar-pro")]
    SonarPro,
    #[strum(serialize = "perplexity/sonar-reasoning")]
    SonarReasoning,
    #[strum(serialize = "perplexity/sonar-reasoning-pro")]
    SonarReasoningPro,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, EnumString, Display)]
pub enum PrimeIntellectModel {
    #[strum(serialize = "prime-intellect/intellect-3")]
    Intellect3,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, EnumString, Display)]
pub enum StealthModel {
    #[strum(serialize = "stealth/sonoma-dusk-alpha")]
    SonomaDuskAlpha,
    #[strum(serialize = "stealth/sonoma-sky-alpha")]
    SonomaSkyAlpha,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, EnumString, Display)]
pub enum VercelModel {
    #[strum(serialize = "vercel/v0-1.0-md")]
    V0_1_0Md,
    #[strum(serialize = "vercel/v0-1.5-md")]
    V0_1_5Md,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, EnumString, Display)]
pub enum XaiModel {
    #[strum(serialize = "xai/grok-2")]
    Grok2,
    #[strum(serialize = "xai/grok-2-vision")]
    Grok2Vision,
    #[strum(serialize = "xai/grok-3")]
    Grok3,
    #[strum(serialize = "xai/grok-3-fast")]
    Grok3Fast,
    #[strum(serialize = "xai/grok-3-mini")]
    Grok3Mini,
    #[strum(serialize = "xai/grok-3-mini-fast")]
    Grok3MiniFast,
    #[strum(serialize = "xai/grok-4")]
    Grok4,
    #[strum(serialize = "xai/grok-4-fast-non-reasoning")]
    Grok4FastNonReasoning,
    #[strum(serialize = "xai/grok-4-fast-reasoning")]
    Grok4FastReasoning,
    #[strum(serialize = "xai/grok-4.1-fast-reasoning")]
    Grok41FastReasoning,
    #[strum(serialize = "xai/grok-4.1-fast-non-reasoning")]
    Grok41FastNonReasoning,
    #[strum(serialize = "xai/grok-code-fast-1")]
    GrokCodeFast1,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, EnumString, Display)]
pub enum ZaiModel {
    #[strum(serialize = "zai/glm-4.5")]
    Glm45,
    #[strum(serialize = "zai/glm-4.5-air")]
    Glm45Air,
    #[strum(serialize = "zai/glm-4.5v")]
    Glm45v,
    #[strum(serialize = "zai/glm-4.6")]
    Glm46,
    #[strum(serialize = "zai/glm-4.7")]
    Glm47,
}
