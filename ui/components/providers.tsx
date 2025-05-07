"use client"

import { useState } from "react"
import { Card, CardContent } from "@/components/ui/card"
import { Check, ChevronDown, ExternalLink } from "lucide-react"
import { DropdownMenu, DropdownMenuContent, DropdownMenuItem, DropdownMenuTrigger } from "@/components/ui/dropdown-menu"
import { Badge } from "@/components/ui/badge"
import { Button } from "@/components/ui/button"
import { useProviders } from "@/hooks/use-providers"
import { ProviderIcon } from "./provider-icon"

export function Providers() {
  const { providers, selectedProvider, selectedModel, setProvider, setModel } = useProviders()
  const [expanded, setExpanded] = useState<string | null>(null)

  return (
    <div className="space-y-4">
      <div className="flex items-center justify-between">
        <h2 className="text-xl font-mono font-semibold">Providers</h2>
        <Badge variant="outline" className="font-mono text-xs text-white">
          {Object.keys(providers).length} Available
        </Badge>
      </div>

      <div className="grid gap-3">
        {Object.entries(providers).map(([key, config]) => (
          <Card
            key={key}
            className={`border transition-all cursor-pointer ${
              selectedProvider === key
                ? "border-green-500 bg-gray-900"
                : "border-gray-800 hover:border-gray-700 bg-gray-950"
            }`}
            onClick={() => {
              setProvider(key)
              setExpanded(expanded === key ? null : key)
            }}
          >
            <CardContent className="p-4">
              <div className="flex items-center justify-between">
                <div className="flex items-center gap-3">
                  <ProviderIcon provider={key} />
                  <h3 className="font-mono font-bold text-white">{key.toUpperCase()}</h3>
                </div>
                <div className="flex items-center gap-2">
                  <Badge variant={config.status === "online" ? "success" : "destructive"} className="font-mono text-xs text-white">
                    {config.status}
                  </Badge>
                  <Button
                    variant="ghost"
                    size="icon"
                    className="h-6 w-6"
                    onClick={(e) => {
                      e.stopPropagation()
                      setExpanded(expanded === key ? null : key)
                    }}
                  >
                    <ChevronDown className={`h-4 w-4 transition-transform ${expanded === key ? "rotate-180" : ""}`} />
                  </Button>
                </div>
              </div>

              {expanded === key && (
                <div className="mt-4 pt-4 border-t border-gray-800 text-sm">
                  <div className="grid gap-3">
                    <div className="flex justify-between">
                      <span className="text-gray-400">Endpoint:</span>
                      <span className="font-mono text-gray-300 flex items-center gap-1">
                        {config.url}
                        <ExternalLink className="h-3 w-3 text-gray-500" />
                      </span>
                    </div>
                    <div className="flex justify-between">
                      <span className="text-gray-400">Retries:</span>
                      <span className="font-mono text-gray-300">{config.retries}</span>
                    </div>
                    <div className="flex justify-between">
                      <span className="text-gray-400">Retry Delay:</span>
                      <span className="font-mono text-gray-300">{config.retryDelay}ms</span>
                    </div>
                    <div className="flex justify-between">
                      <span className="text-gray-400">Latency:</span>
                      <span className="font-mono text-green-400">{config.latency}ms</span>
                    </div>
                    <div className="flex justify-between">
                      <span className="text-gray-400">Success Rate:</span>
                      <span className="font-mono text-green-400">{config.successRate}%</span>
                    </div>

                    <div className="mt-2">
                      <span className="text-gray-400 block mb-2">Models:</span>
                      <div className="flex flex-wrap gap-2">
                        {config.models.map((model) => (
                          <DropdownMenu key={model}>
                            <DropdownMenuTrigger asChild>
                              <Button
                                variant={selectedProvider === key && selectedModel === model ? "default" : "outline"}
                                size="sm"
                                className={`font-mono text-xs ${
                                  selectedProvider === key && selectedModel === model
                                    ? "bg-green-900 hover:bg-green-800 text-green-100"
                                    : "bg-gray-900 hover:bg-gray-800"
                                }`}
                                onClick={(e) => {
                                  e.stopPropagation()
                                  setProvider(key)
                                  setModel(model)
                                }}
                              >
                                {model}
                                {selectedProvider === key && selectedModel === model && (
                                  <Check className="ml-1 h-3 w-3" />
                                )}
                              </Button>
                            </DropdownMenuTrigger>
                            <DropdownMenuContent className="bg-gray-900 border-gray-800">
                              <DropdownMenuItem
                                className="font-mono text-xs cursor-pointer"
                                onClick={() => {
                                  setProvider(key)
                                  setModel(model)
                                }}
                              >
                                Select Model
                              </DropdownMenuItem>
                              <DropdownMenuItem className="font-mono text-xs cursor-pointer">
                                View Model Info
                              </DropdownMenuItem>
                            </DropdownMenuContent>
                          </DropdownMenu>
                        ))}
                      </div>
                    </div>
                  </div>
                </div>
              )}
            </CardContent>
          </Card>
        ))}
      </div>
    </div>
  )
}
