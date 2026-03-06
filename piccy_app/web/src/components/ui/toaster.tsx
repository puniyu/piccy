import { motion, AnimatePresence } from "motion/react";
import { X, CheckCircle, AlertCircle, Info, AlertTriangle } from "lucide-react";
import { createContext, useContext, useState, useCallback } from "react";

interface Toast {
	id: string;
	title: string;
	description?: string;
	type: "success" | "error" | "warning" | "info";
	duration: number;
}

interface ToasterContextType {
	toasts: Toast[];
	create: (toast: Omit<Toast, "id">) => void;
	dismiss: (id: string) => void;
}

const ToasterContext = createContext<ToasterContextType | null>(null);

export function ToasterProvider({ children }: { children: React.ReactNode }) {
	const [toasts, setToasts] = useState<Toast[]>([]);

	const create = useCallback(
		(toast: Omit<Toast, "id">) => {
			const id = Math.random().toString(36).substring(7);
			const newToast = { ...toast, id };
			setToasts((prev) => [...prev, newToast]);

			setTimeout(() => {
				setToasts((prev) => prev.filter((t) => t.id !== id));
			}, toast.duration);
		},
		[],
	);

	const dismiss = useCallback((id: string) => {
		setToasts((prev) => prev.filter((t) => t.id !== id));
	}, []);

	return (
		<ToasterContext.Provider value={{ toasts, create, dismiss }}>
			{children}
			<div className="fixed top-4 right-4 z-9999 flex flex-col gap-2 pointer-events-none">
				<AnimatePresence>
					{toasts.map((toast) => (
						<ToastItem key={toast.id} toast={toast} onDismiss={dismiss} />
					))}
				</AnimatePresence>
			</div>
		</ToasterContext.Provider>
	);
}

function ToastItem({
	toast,
	onDismiss,
}: {
	toast: Toast;
	onDismiss: (id: string) => void;
}) {
	const icons = {
		success: <CheckCircle size={20} className="text-green-500" />,
		error: <AlertCircle size={20} className="text-red-500" />,
		warning: <AlertTriangle size={20} className="text-yellow-500" />,
		info: <Info size={20} className="text-blue-500" />,
	};

	const bgColors = {
		success: "bg-green-50 dark:bg-green-900/20 border-green-200 dark:border-green-800",
		error: "bg-red-50 dark:bg-red-900/20 border-red-200 dark:border-red-800",
		warning: "bg-yellow-50 dark:bg-yellow-900/20 border-yellow-200 dark:border-yellow-800",
		info: "bg-blue-50 dark:bg-blue-900/20 border-blue-200 dark:border-blue-800",
	};

	return (
		<motion.div
			initial={{ opacity: 0, x: 100, scale: 0.8 }}
			animate={{ opacity: 1, x: 0, scale: 1 }}
			exit={{ opacity: 0, x: 100, scale: 0.8 }}
			transition={{ type: "spring", damping: 20, stiffness: 300 }}
			className={`pointer-events-auto min-w-[320px] max-w-md p-4 rounded-lg shadow-lg border ${bgColors[toast.type]} backdrop-blur-sm`}
		>
			<div className="flex items-start gap-3">
				<div className="shrink-0 mt-0.5">{icons[toast.type]}</div>
				<div className="flex-1 min-w-0">
					<p className="font-semibold text-gray-900 dark:text-white">
						{toast.title}
					</p>
					{toast.description && (
						<p className="text-sm text-gray-600 dark:text-gray-300 mt-1">
							{toast.description}
						</p>
					)}
				</div>
				<button
					onClick={() => onDismiss(toast.id)}
					className="shrink-0 text-gray-400 hover:text-gray-600 dark:hover:text-gray-200 transition-colors"
				>
					<X size={18} />
				</button>
			</div>
		</motion.div>
	);
}

export function useToaster() {
	const context = useContext(ToasterContext);
	if (!context) {
		throw new Error("useToaster must be used within ToasterProvider");
	}
	return context;
}

export const toaster = {
	create: (_toast: Omit<Toast, "id">) => {
		// This is a placeholder that will be replaced by the actual implementation
		// when used within ToasterProvider
		console.warn("Toaster not initialized. Wrap your app with ToasterProvider.");
	},
};
