"use client"

import { useState } from "react"
import { Card } from "@/components/ui/card"
import { Button } from "@/components/ui/button"
import { Copy, Code, Check, Clock, Zap } from "lucide-react"
import { ProviderIcon } from "./provider-icon"
import { JsonViewer } from "./json-viewer"
import type { ChatMessage } from "@/types/chat"

interface MessageBubbleProps {
  message: ChatMessage
  onViewJson: () => void
  showJson: boolean
}

export function MessageBubble({ message, onViewJson, showJson }: MessageBubbleProps) {
  const [copied, setCopied] = useState(false)

  const copyToClipboard = () => {
    navigator.clipboard.writeText(message.content)
    setCopied(true)
    setTimeout(() => setCopied(false), 2000)
  }

  return (
    <div className={`flex ${message.role === "user" ? "justify-end" : "justify-start"} text-white`}>
      <Card
        className={`max-w-[85%] ${
          message.role === "user" ? "bg-gray-800 border-gray-700" : "bg-gray-900 border-green-900/30"
        }`}
      >
        <div className="p-4">
          <div className="flex items-center gap-2 mb-2">
            {message.role === "user" ? (
              <div className="bg-gray-700 rounded-full h-6 w-6 flex items-center justify-center text-xs font-mono">
                U
              </div>
            ) : (
              <ProviderIcon provider={message.provider || "openai"} className="h-4 w-4" />
            )}
            <div className="text-xs font-mono text-white">
              {message.role === "user" ? "You" : message.provider?.toUpperCase() || "ASSISTANT"}
            </div>
            <div className="ml-auto flex items-center gap-2">
              {message.role === "assistant" && (
                <>
                  <Button
                    variant="ghost"
                    size="icon"
                    className="h-6 w-6 text-white hover:text-gray-300"
                    onClick={onViewJson}
                  >
                    <Code className="h-3.5 w-3.5" />
                  </Button>
                  <Button
                    variant="ghost"
                    size="icon"
                    className="h-6 w-6 text-white hover:text-gray-300"
                    onClick={copyToClipboard}
                  >
                    {copied ? <Check className="h-3.5 w-3.5 text-green-500" /> : <Copy className="h-3.5 w-3.5" />}
                  </Button>
                </>
              )}
            </div>
          </div>

          <div className="font-mono text-sm whitespace-pre-wrap text-white">{message.content}</div>

          {message.role === "assistant" && (
            <>
              {showJson && (
                <div className="mt-4 border-t border-gray-800 pt-4 text-white">
                  <div className="text-xs font-mono text-white mb-2">Raw Response:</div>
                  <JsonViewer data={message.rawResponse || {}} />
                </div>
              )}

              <div className="mt-4 pt-2 border-t border-gray-800 flex items-center justify-between text-xs text-white font-mono">
                <div className="flex items-center gap-1">
                  <Clock className="h-3 w-3" />
                  <span>{message.responseTime || 0}ms</span>
                </div>
                <div className="flex items-center gap-1">
                  <Zap className="h-3 w-3" />
                  <span>{message.tokens || 0} tokens</span>
                </div>
              </div>
            </>
          )}
        </div>
      </Card>
    </div>
  )
}
