import { X } from "lucide-react";
import { motion, AnimatePresence } from "motion/react";

interface FileUploadListProps {
	files: File[];
	onRemove: (index: number) => void;
}

export const FileUploadList = ({ files, onRemove }: FileUploadListProps) => {
	if (files.length === 0) return null;

	return (
		<div
			className={
				"grid w-full gap-2 md:gap-3 max-h-[50vh] overflow-y-auto p-1 " +
				"grid-cols-[repeat(auto-fit,minmax(120px,1fr))] " +
				"sm:grid-cols-[repeat(auto-fit,minmax(140px,1fr))] " +
				"md:grid-cols-[repeat(auto-fit,minmax(160px,1fr))]"
			}
		>
			<AnimatePresence>
				{files.map((file, index) => (
					<motion.div
						key={file.name}
						initial={{ opacity: 0, scale: 0.8 }}
						animate={{ opacity: 1, scale: 1 }}
						exit={{ opacity: 0, scale: 0.8 }}
						transition={{ type: "spring", damping: 20, stiffness: 300 }}
						layout
						className="w-full"
					>
						<div
							className={`w-full ${files.length === 1 ? "h-64 md:h-80" : "aspect-square"} p-0 rounded-xl overflow-hidden border-2 border-gray-200 dark:border-gray-600 relative bg-gray-50 hover:border-blue-400 hover:shadow-md transition-all duration-200 group`}
						>
							<img
								src={URL.createObjectURL(file)}
								alt="preview"
								className="w-full h-full object-cover block"
							/>

							<div className="absolute top-2 right-2 opacity-0 group-hover:opacity-100 transition-opacity duration-200 z-10">
								<button
									onClick={() => onRemove(index)}
									className="w-7 h-7 rounded-full bg-red-500 text-white flex items-center justify-center cursor-pointer transition-transform hover:bg-red-600 hover:scale-110 shadow-sm"
								>
									<X size={16} />
								</button>
							</div>
						</div>
					</motion.div>
				))}
			</AnimatePresence>
		</div>
	);
};
