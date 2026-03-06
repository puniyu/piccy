import { createRoot } from "react-dom/client";
import App from "@/app";
import React from "react";
import "@/styles/global.css";
import { Provider } from "@/components/ui/provider";
import { ToasterProvider } from "@/components/ui/toaster";

createRoot(document.getElementById("root")!).render(
	<React.StrictMode>
		<Provider>
			<ToasterProvider>
				<App />
			</ToasterProvider>
		</Provider>
	</React.StrictMode>,
);
