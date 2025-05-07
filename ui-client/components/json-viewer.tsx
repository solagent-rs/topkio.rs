"use client"

import { useState } from "react"
import { Button } from "@/components/ui/button"
import { ChevronDown, ChevronRight, Copy, Check } from "lucide-react"

interface JsonViewerProps {
  data: any
  initialCollapsed?: boolean
}

export function JsonViewer({ data, initialCollapsed = true }: JsonViewerProps) {
  const [collapsed, setCollapsed] = useState(initialCollapsed)
  const [copied, setCopied] = useState(false)

  const copyToClipboard = () => {
    navigator.clipboard.writeText(JSON.stringify(data, null, 2))
    setCopied(true)
    setTimeout(() => setCopied(false), 2000)
  }

  return (
    <div className="bg-gray-950 border border-gray-800 rounded-md overflow-hidden">
      <div className="flex items-center justify-between p-2 bg-gray-900 border-b border-gray-800">
        <Button variant="ghost" size="sm" className="h-6 text-xs font-mono" onClick={() => setCollapsed(!collapsed)}>
          {collapsed ? <ChevronRight className="h-3 w-3 mr-1" /> : <ChevronDown className="h-3 w-3 mr-1" />}
          {collapsed ? "Expand" : "Collapse"}
        </Button>
        <Button variant="ghost" size="icon" className="h-6 w-6" onClick={copyToClipboard}>
          {copied ? <Check className="h-3.5 w-3.5 text-green-500" /> : <Copy className="h-3.5 w-3.5" />}
        </Button>
      </div>
      <div className={`p-3 font-mono text-xs overflow-x-auto ${collapsed ? "max-h-32" : "max-h-96"}`}>
        <pre className="text-green-400">{JSON.stringify(data, null, 2)}</pre>
      </div>
    </div>
  )
}
