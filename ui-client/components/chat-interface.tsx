"use client"

import type React from "react"

import { useState, useRef, useEffect } from "react"
import { Card, CardContent } from "@/components/ui/card"
import { Button } from "@/components/ui/button"
import { Textarea } from "@/components/ui/textarea"
import { Switch } from "@/components/ui/switch"
import { Label } from "@/components/ui/label"
import { Send, Clock, RotateCcw, Download, Code, List, Sparkles, Loader2 } from "lucide-react"
import { useChatCompletion } from "@/hooks/use-chat-completion"
import { useProviders } from "@/hooks/use-providers"
import { MessageBubble } from "./message-bubble"

export function ChatInterface() {
  const { messages, isLoading, isStreaming, setIsStreaming, sendMessage, resetChat } = useChatCompletion()
  const { selectedProvider, selectedModel } = useProviders()
  const [input, setInput] = useState("")
  const messagesEndRef = useRef<HTMLDivElement>(null)
  const [showJson, setShowJson] = useState(false)

  useEffect(() => {
    messagesEndRef.current?.scrollIntoView({ behavior: "smooth" })
  }, [messages])

  const handleSubmit = () => {
    if (input.trim() && !isLoading) {
      sendMessage(input)
      setInput("")
    }
  }

  const handleKeyDown = (e: React.KeyboardEvent) => {
    if (e.key === "Enter" && !e.shiftKey) {
      e.preventDefault()
      handleSubmit()
    }
  }

  return (
    <Card className="border-gray-800 bg-gray-950">
      <CardContent className="p-0">
        <div className="flex flex-col h-[calc(100vh-180px)]">
          {/* Chat Header */}
          <div className="p-4 border-b border-gray-800 flex items-center justify-between bg-gray-900">
            <div className="flex items-center gap-2">
              <div className="font-mono text-sm">
                <span className="text-gray-400">Using:</span>{" "}
                <span className="text-green-400">
                  {selectedProvider}:{selectedModel}
                </span>
              </div>
              <div className="h-1.5 w-1.5 rounded-full bg-green-500 animate-pulse"></div>
            </div>
            <div className="flex items-center gap-4">
              <div className="flex items-center gap-2">
                <Switch id="streaming" checked={isStreaming} onCheckedChange={setIsStreaming} />
                <Label htmlFor="streaming" className="font-mono text-xs cursor-pointer">
                  Streaming
                </Label>
              </div>
              <Button
                variant="ghost"
                size="icon"
                className="h-8 w-8 text-gray-400 hover:text-gray-100"
                onClick={resetChat}
              >
                <RotateCcw className="h-4 w-4" />
              </Button>
              <Button variant="ghost" size="icon" className="h-8 w-8 text-gray-400 hover:text-gray-100">
                <Download className="h-4 w-4" />
              </Button>
            </div>
          </div>

          {/* Messages Area */}
          <div className="flex-1 overflow-y-auto p-4 space-y-4 bg-[linear-gradient(to_bottom,rgba(0,0,0,0)_0%,rgba(0,128,0,0.05)_100%)]">
            {messages.length === 0 ? (
              <div className="h-full flex flex-col items-center justify-center text-gray-500">
                <div className="font-mono text-center max-w-md">
                  <div className="text-green-500 text-xl mb-2">{">"} AI Gateway Terminal</div>
                  <p className="mb-4">
                    Select a provider and model, then start chatting to interact with the AI Gateway.
                  </p>
                  <div className="text-xs text-gray-600 border border-gray-800 bg-gray-900 p-2 rounded font-mono">
                    $ echo "Hello, AI Gateway!" | gateway-client --provider={selectedProvider} --model={selectedModel}
                  </div>
                </div>
              </div>
            ) : (
              <>
                {messages.map((message, index) => (
                  <MessageBubble
                    key={index}
                    message={message}
                    onViewJson={() => setShowJson(!showJson)}
                    showJson={showJson}
                  />
                ))}
                {isLoading && (
                  <div className="flex items-center gap-2 text-gray-400 font-mono text-sm">
                    <Loader2 className="h-4 w-4 animate-spin" />
                    <span>Processing request...</span>
                  </div>
                )}
              </>
            )}
            <div ref={messagesEndRef} />
          </div>

          {/* Input Area */}
          <div className="p-4 border-t border-gray-800 bg-gray-900">
            <div className="flex gap-2 mb-2">
              <Button variant="outline" size="sm" className="h-8 bg-gray-900 border-gray-700 hover:bg-gray-800">
                <Code className="h-4 w-4 mr-1" />
                <span className="text-xs">Code</span>
              </Button>
              <Button variant="outline" size="sm" className="h-8 bg-gray-900 border-gray-700 hover:bg-gray-800">
                <List className="h-4 w-4 mr-1" />
                <span className="text-xs">List</span>
              </Button>
              <Button variant="outline" size="sm" className="h-8 bg-gray-900 border-gray-700 hover:bg-gray-800">
                <Sparkles className="h-4 w-4 mr-1" />
                <span className="text-xs">Assist</span>
              </Button>
            </div>
            <div className="flex gap-2">
              <Textarea
                value={input}
                onChange={(e) => setInput(e.target.value)}
                onKeyDown={handleKeyDown}
                placeholder="Type your message here..."
                className="min-h-[80px] bg-gray-900 border-gray-700 font-mono resize-none"
              />
              <Button
                className="self-end bg-green-700 hover:bg-green-600"
                onClick={handleSubmit}
                disabled={isLoading || !input.trim()}
              >
                <Send className="h-4 w-4 mr-2" />
                Send
              </Button>
            </div>
            <div className="mt-2 text-xs text-gray-500 font-mono flex items-center justify-between">
              <div className="flex items-center gap-1">
                <Clock className="h-3 w-3" />
                <span>
                  Response time: {messages.length > 0 ? `${messages[messages.length - 1].responseTime || 0}ms` : "0ms"}
                </span>
              </div>
              <div>
                Tokens: <span className="text-green-500">0</span> / 4096
              </div>
            </div>
          </div>
        </div>
      </CardContent>
    </Card>
  )
}
