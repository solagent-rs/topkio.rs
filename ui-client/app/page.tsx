import { Providers } from "@/components/providers"
import { ChatInterface } from "@/components/chat-interface"
import { AdvancedControls } from "@/components/advanced-controls"

export default function Home() {
  return (
    <main className="min-h-screen bg-gray-950 text-gray-100 p-4 md:p-6">
      <div className="max-w-7xl mx-auto">
        <header className="mb-8">
          <h1 className="text-3xl font-mono font-bold text-green-400 mb-2">AI Gateway Client</h1>
          <p className="text-gray-400 font-mono text-sm">
            Connect to multiple AI providers through a unified interface
          </p>
        </header>

        <div className="grid grid-cols-1 lg:grid-cols-4 gap-6">
          <div className="lg:col-span-1">
            <Providers />
            <div className="mt-6">
              <AdvancedControls />
            </div>
          </div>
          <div className="lg:col-span-3">
            <ChatInterface />
          </div>
        </div>
      </div>
    </main>
  )
}
