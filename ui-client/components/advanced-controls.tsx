"use client"

import { useState } from "react"
import { Card, CardContent } from "@/components/ui/card"
import { Slider } from "@/components/ui/slider"
import { Input } from "@/components/ui/input"
import { Textarea } from "@/components/ui/textarea"
import { Button } from "@/components/ui/button"
import { ChevronDown, ChevronUp, Save, RotateCcw } from "lucide-react"
import { useAdvancedSettings } from "@/hooks/use-advanced-settings"

export function AdvancedControls() {
  const {
    temperature,
    setTemperature,
    topP,
    setTopP,
    maxTokens,
    setMaxTokens,
    systemMessage,
    setSystemMessage,
    resetSettings,
  } = useAdvancedSettings()

  const [expanded, setExpanded] = useState(false)

  return (
    <Card className="border-gray-800 bg-gray-950">
      <CardContent className="p-4">
        <div className="flex items-center justify-between cursor-pointer" onClick={() => setExpanded(!expanded)}>
          <h3 className="font-mono font-semibold">Advanced Controls</h3>
          <Button variant="ghost" size="icon" className="h-6 w-6">
            {expanded ? <ChevronUp className="h-4 w-4" /> : <ChevronDown className="h-4 w-4" />}
          </Button>
        </div>

        {expanded && (
          <div className="mt-4 space-y-4">
            <div className="space-y-2">
              <div className="flex items-center justify-between">
                <label className="text-sm font-mono text-gray-400">Temperature</label>
                <span className="text-xs font-mono">{temperature.toFixed(2)}</span>
              </div>
              <Slider
                value={[temperature]}
                min={0}
                max={2}
                step={0.01}
                onValueChange={(value) => setTemperature(value[0])}
                className="py-1"
              />
              <p className="text-xs text-gray-500">Controls randomness: lower is more deterministic</p>
            </div>

            <div className="space-y-2">
              <div className="flex items-center justify-between">
                <label className="text-sm font-mono text-gray-400">Top P</label>
                <span className="text-xs font-mono">{topP.toFixed(2)}</span>
              </div>
              <Slider
                value={[topP]}
                min={0}
                max={1}
                step={0.01}
                onValueChange={(value) => setTopP(value[0])}
                className="py-1"
              />
              <p className="text-xs text-gray-500">Controls diversity via nucleus sampling</p>
            </div>

            <div className="space-y-2">
              <div className="flex items-center justify-between">
                <label className="text-sm font-mono text-gray-400">Max Tokens</label>
                <span className="text-xs font-mono">{maxTokens}</span>
              </div>
              <div className="flex items-center gap-2">
                <Slider
                  value={[maxTokens]}
                  min={1}
                  max={4096}
                  step={1}
                  onValueChange={(value) => setMaxTokens(value[0])}
                  className="py-1"
                />
                <Input
                  type="number"
                  value={maxTokens}
                  onChange={(e) => setMaxTokens(Number(e.target.value))}
                  className="w-16 h-8 bg-gray-900 border-gray-700 font-mono text-xs"
                />
              </div>
              <p className="text-xs text-gray-500">Maximum length of generated text</p>
            </div>

            <div className="space-y-2">
              <label className="text-sm font-mono text-gray-400">System Message</label>
              <Textarea
                value={systemMessage}
                onChange={(e) => setSystemMessage(e.target.value)}
                className="min-h-[100px] bg-gray-900 border-gray-700 font-mono text-xs resize-none"
                placeholder="You are a helpful AI assistant..."
              />
              <p className="text-xs text-gray-500">Instructions for the AI model</p>
            </div>

            <div className="flex items-center justify-between pt-2">
              <Button
                variant="outline"
                size="sm"
                className="text-xs bg-gray-900 border-gray-700 hover:bg-gray-800"
                onClick={resetSettings}
              >
                <RotateCcw className="h-3 w-3 mr-1" />
                Reset
              </Button>
              <Button size="sm" className="text-xs bg-green-700 hover:bg-green-600">
                <Save className="h-3 w-3 mr-1" />
                Save Preset
              </Button>
            </div>
          </div>
        )}
      </CardContent>
    </Card>
  )
}
