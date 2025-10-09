import { ChakraProvider, defaultSystem } from '@chakra-ui/react'
import { createRoot } from 'react-dom/client'
import App from "@/app"
import React from "react"
import "@/styles/index.scss"

createRoot(document.getElementById('root')!).render(
    <React.StrictMode>
    <ChakraProvider value={defaultSystem}>
        <App />
    </ChakraProvider>,
    </React.StrictMode>,
)