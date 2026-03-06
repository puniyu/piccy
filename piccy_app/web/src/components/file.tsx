import { Button } from "@heroui/react";
import { X, Plus } from "lucide-react";
import { motion, AnimatePresence } from "motion/react";

interface FileUploadListProps {
	files: File[];
	onRemove: (index: number) => void;
	onAddMore?: () => void;
	showAddMore?: boolean;
}

export const FileUploadList = ({
	files,
	onRemove,
	onAddMore,
	showAddMore = false,
}: FileUploadListProps) => {
	if (files.length === 0) return null;

	return (
		<div className="grid grid-cols-2 sm:grid-cols-3 md:grid-cols-4 lg:grid-cols-5 gap-4">
			<AnimatePresence>
				{files.map((file, index) => (
					<motion.div
						key={file.name}
						initial={{ opacity: 0, scale: 0.95 }}
						animate={{ opacity: 1, scale: 1 }}
						exit={{ opacity: 0, scale: 0.95 }}
						transition={{ duration: 0.2 }}
						className="relative group aspect-square rounded-3xl overflow-hidden border-3 border-pink-200/50 dark:border-pink-800/40 hover:border-pink-300/70 dark:hover:border-pink-700/60 transition-all duration-200 shadow-md hover:shadow-lg bg-white/30 dark:bg-pink-950/20 backdrop-blur-sm"
					>
						<div className="absolute inset-0 p-2">
							<img
								src={URL.createObjectURL(file)}
								alt={file.name}
								className="w-full h-full object-cover rounded-2xl"
							/>
						</div>
						<div className="absolute inset-0 bg-linear-to-t from-pink-400/60 via-transparent to-transparent opacity-0 group-hover:opacity-100 transition-opacity duration-200 backdrop-blur-[2px]" />
						<div className="absolute bottom-0 left-0 right-0 p-3 text-white text-xs truncate opacity-0 group-hover:opacity-100 transition-opacity duration-200 font-medium">
							{file.name}
						</div>
						<div className="absolute top-3 right-3">
							<Button
								isIconOnly
								size="sm"
								onPress={() => onRemove(index)}
								className="bg-pink-500 hover:bg-pink-600 text-white min-w-8 w-8 h-8 shadow-lg"
							>
								<X size={16} strokeWidth={2.5} />
							</Button>
						</div>
						<div className="absolute top-3 left-3">
							<div className="bg-pink-400/90 backdrop-blur-md text-white text-xs px-2.5 py-1 rounded-full font-semibold shadow-md">
								{index + 1}
							</div>
						</div>
					</motion.div>
				))}

				{showAddMore && onAddMore && (
					<motion.button
						key="add-more"
						initial={{ opacity: 0, scale: 0.95 }}
						animate={{ opacity: 1, scale: 1 }}
						exit={{ opacity: 0, scale: 0.95 }}
						transition={{ duration: 0.2 }}
						onClick={onAddMore}
						className="aspect-square rounded-3xl border-3 border-dashed border-pink-300/50 dark:border-pink-700/50 hover:border-pink-400/70 dark:hover:border-pink-600/70 bg-white/50 dark:bg-pink-950/20 hover:bg-white/70 dark:hover:bg-pink-950/30 backdrop-blur-md transition-all duration-200 flex flex-col items-center justify-center gap-3 shadow-md hover:shadow-lg"
					>
						<div className="p-4 rounded-full bg-pink-300/60 dark:bg-pink-700/40 backdrop-blur-sm">
							<Plus size={28} className="text-pink-500 dark:text-pink-300" strokeWidth={2.5} />
						</div>
						<span className="text-sm font-semibold text-pink-400/90 dark:text-pink-300/90">
							继续添加
						</span>
					</motion.button>
				)}
			</AnimatePresence>
		</div>
	);
};
