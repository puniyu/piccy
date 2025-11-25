import { ChakraProvider, defaultSystem } from '@chakra-ui/react'
import { createRoot } from 'react-dom/client'
import App from "@/app"
import React from "react"
import "@/styles/tailwind.css"
import { ColorModeProvider } from "@/components/ui/color-mode"
import { Toaster } from "@/components/ui/toaster"

createRoot(document.getElementById('root')!).render(
    <React.StrictMode>
        <ChakraProvider value={defaultSystem}>
            <ColorModeProvider>
                <App />
                <Toaster />
            </ColorModeProvider>
        </ChakraProvider>
    </React.StrictMode>,
)