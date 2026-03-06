import { FileImage, ImageIcon } from "lucide-react";
import { useCallback, useEffect, useState } from "react";
import { ImageMenu } from "@/components/menu.tsx";
import { FileUploadList } from "@/components/file";
import { ColorModeButton } from "@/components/ui/color-mode";
import { useToaster } from "@/components/ui/toaster";
import { motion, AnimatePresence } from "motion/react";

export default function App() {
	const toaster = useToaster();

	useEffect(() => {
		if (!import.meta.env.DEV) {
			document.addEventListener("contextmenu", function (event) {
				event.preventDefault();
			});
		}
	}, []);

	const [uploadedFiles, setUploadedFiles] = useState<File[]>([]);
	const [isDragging, setIsDragging] = useState(false);

	const handleFileChange = useCallback(
		(files: File[]) => {
			const maxFiles = 20;
			const acceptedFiles = files.slice(0, maxFiles);
			const rejectedCount = files.length - acceptedFiles.length;

			setUploadedFiles(acceptedFiles);

			if (rejectedCount > 0) {
				toaster.create({
					title: "部分图片未能添加",
					description: `已达到最大数量限制（${maxFiles}张），${rejectedCount} 张图片被忽略`,
					type: "warning",
					duration: 4000,
				});
			}
		},
		[toaster],
	);

	const handleDrop = useCallback(
		(e: React.DragEvent) => {
			e.preventDefault();
			setIsDragging(false);

			const files = Array.from(e.dataTransfer.files).filter((file) =>
				file.type.startsWith("image/"),
			);
			if (files.length > 0) {
				handleFileChange(files);
			}
		},
		[handleFileChange],
	);

	const handleDragOver = useCallback((e: React.DragEvent) => {
		e.preventDefault();
		setIsDragging(true);
	}, []);

	const handleDragLeave = useCallback((e: React.DragEvent) => {
		e.preventDefault();
		setIsDragging(false);
	}, []);

	const handleFileInput = useCallback(
		(e: React.ChangeEvent<HTMLInputElement>) => {
			const files = Array.from(e.target.files || []);
			if (files.length > 0) {
				handleFileChange(files);
			}
		},
		[handleFileChange],
	);

	const removeFile = useCallback((index: number) => {
		setUploadedFiles((prev) => prev.filter((_, i) => i !== index));
	}, []);

	return (
		<div className="w-full h-screen flex items-center justify-center bg-gray-50 dark:bg-gray-900 relative overflow-hidden">
			<motion.div
				layout
				initial={{ opacity: 0, y: 20, scale: 0.95 }}
				animate={{ opacity: 1, y: 0, scale: 1 }}
				transition={{
					layout: { duration: 0.4, ease: "easeOut" },
					opacity: { duration: 0.4 },
					y: { duration: 0.4 },
				}}
				data-has-files={uploadedFiles.length > 0}
				className="relative px-8 py-8 bg-white dark:bg-gray-800 rounded-2xl shadow-xl border border-gray-200 dark:border-gray-700 flex flex-col gap-8 w-[90%] max-w-2xl data-[has-files=true]:w-[95%] data-[has-files=true]:max-w-5xl"
			>
				<div className="absolute top-4 right-4 z-10">
					<ColorModeButton />
				</div>

				<div className="text-center space-y-3">
					<div className="flex items-center justify-center gap-3 text-gray-900 dark:text-white">
						<ImageIcon size={36} className="text-blue-500" />
						<h1 className="text-4xl font-bold">Piccy</h1>
					</div>
					<p className="text-gray-500 dark:text-gray-400 text-lg">
						简洁高效的图片处理工具
					</p>
				</div>

				<div className="w-full">
					<input
						type="file"
						id="file-upload"
						multiple
						accept="image/*"
						onChange={handleFileInput}
						className="hidden"
					/>

					<AnimatePresence mode="wait">
						{uploadedFiles.length === 0 ? (
							<motion.div
								key="upload-trigger"
								initial={{ opacity: 0 }}
								animate={{ opacity: 1 }}
								exit={{ opacity: 0 }}
								transition={{ duration: 0.2 }}
								className="w-full"
							>
								<label htmlFor="file-upload">
									<div
										onDrop={handleDrop}
										onDragOver={handleDragOver}
										onDragLeave={handleDragLeave}
										className={`w-full h-auto py-20 border-2 border-dashed rounded-xl cursor-pointer transition-all duration-200 group ${
											isDragging
												? "border-blue-500 bg-blue-50 dark:bg-blue-950/30"
												: "border-gray-300 dark:border-gray-600 hover:border-blue-400 hover:bg-blue-50 dark:hover:bg-blue-950/30"
										}`}
									>
										<div className="flex flex-col items-center gap-4 group-hover:scale-105 transition-transform duration-200">
											<div className="p-5 bg-gray-100 dark:bg-gray-700 rounded-full text-gray-500 dark:text-gray-400 group-hover:text-blue-500 group-hover:bg-blue-100 dark:group-hover:bg-blue-900/50 transition-colors">
												<FileImage size={48} />
											</div>
											<div className="space-y-2">
												<p className="text-xl font-medium text-gray-700 dark:text-gray-200">
													点击或拖拽上传图片
												</p>
												<p className="text-sm text-gray-400 dark:text-gray-500 whitespace-normal wrap-break-word text-center">
													支持 JPG、PNG、GIF 等格式，单次最多上传 20 张图片
												</p>
											</div>
										</div>
									</div>
								</label>
							</motion.div>
						) : (
							<motion.div
								key="upload-content"
								initial={{ opacity: 0, y: 10 }}
								animate={{ opacity: 1, y: 0 }}
								exit={{ opacity: 0, y: -10 }}
								transition={{ duration: 0.3 }}
								className="flex flex-col gap-6"
							>
								<div className="flex flex-col gap-3">
									<FileUploadList files={uploadedFiles} onRemove={removeFile} />
									<p className="text-xs text-gray-400 dark:text-gray-500 text-right">
										已选择 {uploadedFiles.length} / 20 张图片
									</p>
								</div>
								<ImageMenu image={uploadedFiles} />
							</motion.div>
						)}
					</AnimatePresence>
				</div>
			</motion.div>
		</div>
	);
}
