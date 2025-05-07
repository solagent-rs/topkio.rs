import type React from "react"
import type { Metadata } from "next"
import { Inter, JetBrains_Mono } from "next/font/google"
import "./globals.css"
import { ThemeProvider } from "@/components/theme-provider"
import { ProvidersProvider } from "@/hooks/use-providers"
import { AdvancedSettingsProvider } from "@/hooks/use-advanced-settings"

const inter = Inter({
  subsets: ["latin"],
  variable: "--font-sans",
})

const jetbrainsMono = JetBrains_Mono({
  subsets: ["latin"],
  variable: "--font-mono",
})

export const metadata: Metadata = {
  title: "AI Gateway Client",
  description: "A modern UI for interacting with AI Gateway",
    generator: 'v0.dev'
}

export default function RootLayout({
  children,
}: Readonly<{
  children: React.ReactNode
}>) {
  return (
    <html lang="en" suppressHydrationWarning>
      <body className={`${inter.variable} ${jetbrainsMono.variable} font-sans`}>
        <ThemeProvider attribute="class" defaultTheme="light" enableSystem={false} disableTransitionOnChange>
          <ProvidersProvider>
            <AdvancedSettingsProvider>{children}</AdvancedSettingsProvider>
          </ProvidersProvider>
        </ThemeProvider>
      </body>
    </html>
  )
}
