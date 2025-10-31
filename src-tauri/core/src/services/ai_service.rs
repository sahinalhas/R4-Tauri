use reqwest::Client;
use serde::{Deserialize, Serialize};
use crate::error::AppError;
use super::config_service::AiProviderConfig;

/// AI Analysis Response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AiAnalysisResponse {
    pub analysis: String,
    pub recommendations: Vec<String>,
    pub confidence: f32,
}

/// Chat message
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatMessage {
    pub role: String,
    pub content: String,
}

pub struct AiService {
    client: Client,
    config: AiProviderConfig,
}

impl AiService {
    pub fn new(config: AiProviderConfig) -> Self {
        Self {
            client: Client::new(),
            config,
        }
    }

    /// Analyze student profile using AI
    pub async fn analyze_student_profile(
        &self,
        student_data: &str,
    ) -> Result<AiAnalysisResponse, AppError> {
        match self.config.provider.as_str() {
            "openai" => self.call_openai(student_data).await,
            "gemini" => self.call_gemini(student_data).await,
            "ollama" => self.call_ollama(student_data).await,
            _ => Err(AppError::Internal(format!(
                "Unsupported AI provider: {}",
                self.config.provider
            ))),
        }
    }

    /// Generate counseling recommendations
    pub async fn generate_recommendations(
        &self,
        student_data: &str,
    ) -> Result<Vec<String>, AppError> {
        let analysis = self.analyze_student_profile(student_data).await?;
        Ok(analysis.recommendations)
    }

    /// Chat with AI assistant
    pub async fn chat(&self, messages: Vec<ChatMessage>) -> Result<String, AppError> {
        match self.config.provider.as_str() {
            "openai" => self.chat_openai(messages).await,
            "gemini" => self.chat_gemini(messages).await,
            "ollama" => self.chat_ollama(messages).await,
            _ => Err(AppError::Internal(format!(
                "Unsupported AI provider: {}",
                self.config.provider
            ))),
        }
    }

    // ==================== OpenAI Implementation ====================

    async fn call_openai(&self, prompt: &str) -> Result<AiAnalysisResponse, AppError> {
        let api_key = self.config.api_key.as_ref().ok_or_else(|| {
            AppError::Internal("OpenAI API key not configured".to_string())
        })?;

        #[derive(Serialize)]
        struct OpenAIRequest {
            model: String,
            messages: Vec<ChatMessage>,
            temperature: f32,
        }

        #[derive(Deserialize)]
        struct OpenAIResponse {
            choices: Vec<OpenAIChoice>,
        }

        #[derive(Deserialize)]
        struct OpenAIChoice {
            message: ChatMessage,
        }

        let request = OpenAIRequest {
            model: self.config.model.clone(),
            messages: vec![
                ChatMessage {
                    role: "system".to_string(),
                    content: "You are a professional school counselor analyzing student data. Provide insights and recommendations in Turkish.".to_string(),
                },
                ChatMessage {
                    role: "user".to_string(),
                    content: prompt.to_string(),
                },
            ],
            temperature: 0.7,
        };

        let response = self
            .client
            .post("https://api.openai.com/v1/chat/completions")
            .header("Authorization", format!("Bearer {}", api_key))
            .header("Content-Type", "application/json")
            .json(&request)
            .send()
            .await?;

        if !response.status().is_success() {
            let error_text = response.text().await?;
            return Err(AppError::Internal(format!("OpenAI API error: {}", error_text)));
        }

        let result: OpenAIResponse = response.json().await?;
        let analysis = result.choices.first()
            .map(|choice| choice.message.content.clone())
            .unwrap_or_default();

        Ok(AiAnalysisResponse {
            analysis: analysis.clone(),
            recommendations: vec![analysis],
            confidence: 0.8,
        })
    }

    async fn chat_openai(&self, messages: Vec<ChatMessage>) -> Result<String, AppError> {
        let api_key = self.config.api_key.as_ref().ok_or_else(|| {
            AppError::Internal("OpenAI API key not configured".to_string())
        })?;

        #[derive(Serialize)]
        struct OpenAIRequest {
            model: String,
            messages: Vec<ChatMessage>,
        }

        #[derive(Deserialize)]
        struct OpenAIResponse {
            choices: Vec<OpenAIChoice>,
        }

        #[derive(Deserialize)]
        struct OpenAIChoice {
            message: ChatMessage,
        }

        let request = OpenAIRequest {
            model: self.config.model.clone(),
            messages,
        };

        let response = self
            .client
            .post("https://api.openai.com/v1/chat/completions")
            .header("Authorization", format!("Bearer {}", api_key))
            .header("Content-Type", "application/json")
            .json(&request)
            .send()
            .await?;

        if !response.status().is_success() {
            let error_text = response.text().await?;
            return Err(AppError::Internal(format!("OpenAI API error: {}", error_text)));
        }

        let result: OpenAIResponse = response.json().await?;
        Ok(result.choices.first()
            .map(|choice| choice.message.content.clone())
            .unwrap_or_default())
    }

    // ==================== Gemini Implementation ====================

    async fn call_gemini(&self, prompt: &str) -> Result<AiAnalysisResponse, AppError> {
        let api_key = self.config.api_key.as_ref().ok_or_else(|| {
            AppError::Internal("Gemini API key not configured".to_string())
        })?;

        #[derive(Serialize)]
        struct GeminiRequest {
            contents: Vec<GeminiContent>,
        }

        #[derive(Serialize)]
        struct GeminiContent {
            parts: Vec<GeminiPart>,
        }

        #[derive(Serialize)]
        struct GeminiPart {
            text: String,
        }

        #[derive(Deserialize)]
        struct GeminiResponse {
            candidates: Vec<GeminiCandidate>,
        }

        #[derive(Deserialize)]
        struct GeminiCandidate {
            content: GeminiContent,
        }

        let request = GeminiRequest {
            contents: vec![GeminiContent {
                parts: vec![GeminiPart {
                    text: format!("Analyze this student data and provide recommendations in Turkish: {}", prompt),
                }],
            }],
        };

        let url = format!(
            "https://generativelanguage.googleapis.com/v1beta/models/{}:generateContent?key={}",
            self.config.model, api_key
        );

        let response = self
            .client
            .post(&url)
            .header("Content-Type", "application/json")
            .json(&request)
            .send()
            .await?;

        if !response.status().is_success() {
            let error_text = response.text().await?;
            return Err(AppError::Internal(format!("Gemini API error: {}", error_text)));
        }

        let result: GeminiResponse = response.json().await?;
        let analysis = result.candidates.first()
            .and_then(|c| c.content.parts.first())
            .map(|p| p.text.clone())
            .unwrap_or_default();

        Ok(AiAnalysisResponse {
            analysis: analysis.clone(),
            recommendations: vec![analysis],
            confidence: 0.8,
        })
    }

    async fn chat_gemini(&self, messages: Vec<ChatMessage>) -> Result<String, AppError> {
        let api_key = self.config.api_key.as_ref().ok_or_else(|| {
            AppError::Internal("Gemini API key not configured".to_string())
        })?;

        #[derive(Serialize)]
        struct GeminiRequest {
            contents: Vec<GeminiContent>,
        }

        #[derive(Serialize)]
        struct GeminiContent {
            parts: Vec<GeminiPart>,
        }

        #[derive(Serialize)]
        struct GeminiPart {
            text: String,
        }

        #[derive(Deserialize)]
        struct GeminiResponse {
            candidates: Vec<GeminiCandidate>,
        }

        #[derive(Deserialize)]
        struct GeminiCandidate {
            content: GeminiContent,
        }

        let request = GeminiRequest {
            contents: messages
                .iter()
                .map(|msg| GeminiContent {
                    parts: vec![GeminiPart {
                        text: msg.content.clone(),
                    }],
                })
                .collect(),
        };

        let url = format!(
            "https://generativelanguage.googleapis.com/v1beta/models/{}:generateContent?key={}",
            self.config.model, api_key
        );

        let response = self
            .client
            .post(&url)
            .header("Content-Type", "application/json")
            .json(&request)
            .send()
            .await?;

        if !response.status().is_success() {
            let error_text = response.text().await?;
            return Err(AppError::Internal(format!("Gemini API error: {}", error_text)));
        }

        let result: GeminiResponse = response.json().await?;
        Ok(result.candidates.first()
            .and_then(|c| c.content.parts.first())
            .map(|p| p.text.clone())
            .unwrap_or_default())
    }

    // ==================== Ollama Implementation ====================

    async fn call_ollama(&self, prompt: &str) -> Result<AiAnalysisResponse, AppError> {
        let api_url = self.config.api_url.as_ref().ok_or_else(|| {
            AppError::Internal("Ollama API URL not configured".to_string())
        })?;

        #[derive(Serialize)]
        struct OllamaRequest {
            model: String,
            prompt: String,
            stream: bool,
        }

        #[derive(Deserialize)]
        struct OllamaResponse {
            response: String,
        }

        let request = OllamaRequest {
            model: self.config.model.clone(),
            prompt: format!("Analyze this student data and provide recommendations in Turkish: {}", prompt),
            stream: false,
        };

        let url = format!("{}/api/generate", api_url);
        let response = self
            .client
            .post(&url)
            .header("Content-Type", "application/json")
            .json(&request)
            .send()
            .await?;

        if !response.status().is_success() {
            let error_text = response.text().await?;
            return Err(AppError::Internal(format!("Ollama API error: {}", error_text)));
        }

        let result: OllamaResponse = response.json().await?;

        Ok(AiAnalysisResponse {
            analysis: result.response.clone(),
            recommendations: vec![result.response],
            confidence: 0.7,
        })
    }

    async fn chat_ollama(&self, messages: Vec<ChatMessage>) -> Result<String, AppError> {
        let api_url = self.config.api_url.as_ref().ok_or_else(|| {
            AppError::Internal("Ollama API URL not configured".to_string())
        })?;

        #[derive(Serialize)]
        struct OllamaRequest {
            model: String,
            messages: Vec<ChatMessage>,
            stream: bool,
        }

        #[derive(Deserialize)]
        struct OllamaResponse {
            message: ChatMessage,
        }

        let request = OllamaRequest {
            model: self.config.model.clone(),
            messages,
            stream: false,
        };

        let url = format!("{}/api/chat", api_url);
        let response = self
            .client
            .post(&url)
            .header("Content-Type", "application/json")
            .json(&request)
            .send()
            .await?;

        if !response.status().is_success() {
            let error_text = response.text().await?;
            return Err(AppError::Internal(format!("Ollama API error: {}", error_text)));
        }

        let result: OllamaResponse = response.json().await?;
        Ok(result.message.content)
    }
}

/// Test AI connection
pub async fn test_ai_connection(config: AiProviderConfig) -> Result<bool, AppError> {
    let service = AiService::new(config);
    
    // Simple test prompt
    let result = service
        .analyze_student_profile("Test connection")
        .await;

    match result {
        Ok(_) => Ok(true),
        Err(e) => {
            log::error!("AI connection test failed: {}", e);
            Ok(false)
        }
    }
}
