import { Cpu } from "lucide-react"

interface ProviderIconProps {
  provider: string
  className?: string
}

export function ProviderIcon({ provider, className = "h-8 w-8" }: ProviderIconProps) {
  // In a real implementation, you would import actual SVG logos
  const getColorForProvider = (provider: string) => {
    switch (provider.toLowerCase()) {
      case "openai":
        return "text-green-500"
      case "gemini":
        return "text-blue-500"
      case "ollama":
        return "text-purple-500"
      case "deepseek":
        return "text-yellow-500"
      default:
        return "text-gray-500"
    }
  }

  return (
    <div className={`flex items-center justify-center rounded-md bg-gray-800 p-1.5 ${getColorForProvider(provider)}`}>
      <Cpu className={className} />
    </div>
  )
}
