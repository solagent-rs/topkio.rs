"use client"

import { useState } from "react"
import { useProviders } from "./use-providers"
import { useAdvancedSettings } from "./use-advanced-settings"
import type { ChatMessage } from "@/types/chat"

export function useChatCompletion() {
  const [messages, setMessages] = useState<ChatMessage[]>([])
  const [isLoading, setIsLoading] = useState(false)
  const [isStreaming, setIsStreaming] = useState(true)
  const { selectedProvider, selectedModel } = useProviders()
  const { temperature, topP, maxTokens, systemMessage } = useAdvancedSettings()

  const sendMessage = async (content: string) => {
    // Add user message
    const userMessage: ChatMessage = {
      role: "user",
      content,
      timestamp: new Date().toISOString(),
    }

    setMessages((prev) => [...prev, userMessage])
    setIsLoading(true)

    // Simulate API call
    const startTime = Date.now()

    try {
      // In a real implementation, this would be an actual API call
      await new Promise((resolve) => setTimeout(resolve, 1000))

      const responseTime = Date.now() - startTime

      // Mock response
      const assistantMessage: ChatMessage = {
        role: "assistant",
        content: generateMockResponse(content, selectedModel),
        timestamp: new Date().toISOString(),
        provider: selectedProvider,
        model: selectedModel,
        responseTime,
        tokens: Math.floor(Math.random() * 500) + 100,
        rawResponse: {
          id: `chatcmpl-${Math.random().toString(36).substring(2, 12)}`,
          object: "chat.completion",
          created: Math.floor(Date.now() / 1000),
          model: selectedModel,
          choices: [
            {
              index: 0,
              message: {
                role: "assistant",
                content: generateMockResponse(content, selectedModel),
              },
              finish_reason: "stop",
            },
          ],
          usage: {
            prompt_tokens: content.length / 4,
            completion_tokens: Math.floor(Math.random() * 500) + 100,
            total_tokens: content.length / 4 + Math.floor(Math.random() * 500) + 100,
          },
        },
      }

      setMessages((prev) => [...prev, assistantMessage])
    } catch (error) {
      console.error("Error sending message:", error)
      // Handle error
    } finally {
      setIsLoading(false)
    }
  }

  const resetChat = () => {
    setMessages([])
  }

  return {
    messages,
    isLoading,
    isStreaming,
    setIsStreaming,
    sendMessage,
    resetChat,
  }
}

// Helper function to generate mock responses
function generateMockResponse(prompt: string, model: string): string {
  const responses = [
    `I've processed your request about "${prompt.substring(0, 20)}...". Here's what I found:\n\n` +
      '```json\n{\n  "status": "success",\n  "data": {\n    "result": "Analysis complete"\n  }\n}\n```\n\n' +
      "Would you like me to explain this in more detail?",

    `Based on your query "${prompt.substring(0, 20)}...", I can provide the following information:\n\n` +
      "The AI Gateway has successfully routed your request through the selected model. The response latency was within expected parameters.\n\n" +
      "Is there anything specific about this response you'd like me to elaborate on?",

    `I've analyzed your input: "${prompt.substring(0, 20)}..."\n\n` +
      "Here are some key observations:\n" +
      "1. The request was properly formatted\n" +
      "2. All parameters were within acceptable ranges\n" +
      "3. The model responded with high confidence\n\n" +
      "Let me know if you need any clarification or have follow-up questions.",
  ]

  return responses[Math.floor(Math.random() * responses.length)]
}
