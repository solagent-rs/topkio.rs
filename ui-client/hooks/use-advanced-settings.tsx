"use client"

import { useState, createContext, useContext, type ReactNode } from "react"

interface AdvancedSettingsContextType {
  temperature: number
  setTemperature: (value: number) => void
  topP: number
  setTopP: (value: number) => void
  maxTokens: number
  setMaxTokens: (value: number) => void
  systemMessage: string
  setSystemMessage: (value: string) => void
  resetSettings: () => void
}

const defaultSettings = {
  temperature: 0.7,
  topP: 0.9,
  maxTokens: 1024,
  systemMessage: "You are a helpful AI assistant.",
}

const AdvancedSettingsContext = createContext<AdvancedSettingsContextType | undefined>(undefined)

export function AdvancedSettingsProvider({ children }: { children: ReactNode }) {
  const [temperature, setTemperature] = useState(defaultSettings.temperature)
  const [topP, setTopP] = useState(defaultSettings.topP)
  const [maxTokens, setMaxTokens] = useState(defaultSettings.maxTokens)
  const [systemMessage, setSystemMessage] = useState(defaultSettings.systemMessage)

  const resetSettings = () => {
    setTemperature(defaultSettings.temperature)
    setTopP(defaultSettings.topP)
    setMaxTokens(defaultSettings.maxTokens)
    setSystemMessage(defaultSettings.systemMessage)
  }

  return (
    <AdvancedSettingsContext.Provider
      value={{
        temperature,
        setTemperature,
        topP,
        setTopP,
        maxTokens,
        setMaxTokens,
        systemMessage,
        setSystemMessage,
        resetSettings,
      }}
    >
      {children}
    </AdvancedSettingsContext.Provider>
  )
}

export function useAdvancedSettings() {
  const context = useContext(AdvancedSettingsContext)
  if (context === undefined) {
    throw new Error("useAdvancedSettings must be used within an AdvancedSettingsProvider")
  }
  return context
}
