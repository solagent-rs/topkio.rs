"use client"

import { useState, createContext, useContext, type ReactNode } from "react"

interface Provider {
  url: string
  models: string[]
  retries: number
  retryDelay: number
  status: "online" | "offline"
  latency: number
  successRate: number
}

interface ProvidersContextType {
  providers: Record<string, Provider>
  selectedProvider: string
  selectedModel: string
  setProvider: (provider: string) => void
  setModel: (model: string) => void
}

// Mock data - in a real app this would come from your API
const mockProviders: Record<string, Provider> = {
  openai: {
    url: "https://api.openai.com/v1",
    models: ["gpt-4o", "gpt-4-turbo", "gpt-3.5-turbo"],
    retries: 3,
    retryDelay: 1000,
    status: "online",
    latency: 245,
    successRate: 99.8,
  },
  gemini: {
    url: "https://generativelanguage.googleapis.com",
    models: ["gemini-1.5-pro", "gemini-1.5-flash"],
    retries: 2,
    retryDelay: 500,
    status: "online",
    latency: 320,
    successRate: 99.5,
  },
  ollama: {
    url: "http://localhost:11434",
    models: ["llama3.2", "mistral", "codellama"],
    retries: 1,
    retryDelay: 200,
    status: "online",
    latency: 150,
    successRate: 98.7,
  },
  deepseek: {
    url: "https://api.deepseek.com",
    models: ["deepseek-coder", "deepseek-chat"],
    retries: 2,
    retryDelay: 800,
    status: "offline",
    latency: 400,
    successRate: 97.2,
  },
}

const ProvidersContext = createContext<ProvidersContextType | undefined>(undefined)

export function ProvidersProvider({ children }: { children: ReactNode }) {
  const [providers] = useState(mockProviders)
  const [selectedProvider, setSelectedProvider] = useState("openai")
  const [selectedModel, setSelectedModel] = useState("gpt-4o")

  return (
    <ProvidersContext.Provider
      value={{
        providers,
        selectedProvider,
        selectedModel,
        setProvider: setSelectedProvider,
        setModel: setSelectedModel,
      }}
    >
      {children}
    </ProvidersContext.Provider>
  )
}

export function useProviders() {
  const context = useContext(ProvidersContext)
  if (context === undefined) {
    throw new Error("useProviders must be used within a ProvidersProvider")
  }
  return context
}
