export interface ChatMessage {
  role: "user" | "assistant" | "system"
  content: string
  timestamp: string
  provider?: string
  model?: string
  responseTime?: number
  tokens?: number
  rawResponse?: any
}

export interface ChatRequest {
  messages: ChatMessage[]
  model: string
  temperature?: number
  top_p?: number
  max_tokens?: number
  stream?: boolean
}
