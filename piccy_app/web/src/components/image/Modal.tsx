import { X } from "lucide-react";
import { motion, AnimatePresence } from "motion/react";

interface ModalProps {
	isOpen: boolean;
	onClose: () => void;
	children: React.ReactNode;
	size?: "md" | "4xl";
}

export const Modal = ({ isOpen, onClose, children, size = "md" }: ModalProps) => {
	if (!isOpen) return null;

	const sizeClasses = {
		md: "max-w-md",
		"4xl": "max-w-4xl",
	};

	return (
		<AnimatePresence>
			{isOpen && (
				<div className="fixed inset-0 z-50 flex items-center justify-center p-4">
					<motion.div
						initial={{ opacity: 0 }}
						animate={{ opacity: 1 }}
						exit={{ opacity: 0 }}
						className="fixed inset-0 bg-pink-900/20 backdrop-blur-md"
						onClick={onClose}
					/>
					<motion.div
						initial={{ opacity: 0, scale: 0.95, y: 20 }}
						animate={{ opacity: 1, scale: 1, y: 0 }}
						exit={{ opacity: 0, scale: 0.95, y: 20 }}
						transition={{ type: "spring", damping: 25, stiffness: 300 }}
						className={`relative rounded-3xl shadow-[0_10px_60px_rgba(255,117,165,0.35)] border-2 border-pink-200/60 dark:border-pink-800/60 w-full ${sizeClasses[size]} max-h-[90vh] overflow-auto glass-strong animate-pulse-glow`}
					>
						<button
							onClick={onClose}
							className="absolute top-4 right-4 z-10 w-9 h-9 rounded-full flex items-center justify-center text-pink-500 hover:bg-pink-100 dark:hover:bg-pink-900/50 transition-all duration-300 hover:rotate-90 hover:scale-110 shadow-lg"
						>
							<X size={20} strokeWidth={2.5} />
						</button>
						{children}
					</motion.div>
				</div>
			)}
		</AnimatePresence>
	);
};
