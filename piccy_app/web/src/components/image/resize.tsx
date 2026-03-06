import { useState, useEffect } from "react";
import { image_resize } from "@/utils/image.ts";
import { download_file } from "@/utils/file.ts";
import { useToaster } from "@/components/ui/toaster";
import { Modal } from "./Modal";
import { ChevronLeft, ChevronRight } from "lucide-react";

export const ImageResizeModal = ({
	images,
	onClose,
}: {
	images: File[];
	onClose: () => void;
}) => {
	const toaster = useToaster();
	const [isProcessing, setIsProcessing] = useState(false);
	const [selectedIndex, setSelectedIndex] = useState(0);
	const [dimensions, setDimensions] = useState<{ width: number; height: number }[]>(
		images.map(() => ({ width: 800, height: 600 })),
	);
	const [imgSrcs, setImgSrcs] = useState<string[]>([]);

	useEffect(() => {
		const loadImages = async () => {
			const srcs = await Promise.all(
				images.map((img) => {
					return new Promise<string>((resolve) => {
						const reader = new FileReader();
						reader.onload = () => resolve(reader.result as string);
						reader.readAsDataURL(img);
					});
				}),
			);
			setImgSrcs(srcs);
		};
		loadImages();
	}, [images]);

	// 键盘快捷键支持
	useEffect(() => {
		const handleKeyDown = (e: KeyboardEvent) => {
			if (e.key === "ArrowLeft" && selectedIndex > 0) {
				e.preventDefault();
				setSelectedIndex(selectedIndex - 1);
			} else if (e.key === "ArrowRight" && selectedIndex < images.length - 1) {
				e.preventDefault();
				setSelectedIndex(selectedIndex + 1);
			}
		};

		window.addEventListener("keydown", handleKeyDown);
		return () => window.removeEventListener("keydown", handleKeyDown);
	}, [selectedIndex, images.length]);

	const handleDimensionChange = (
		index: number,
		field: "width" | "height",
		value: number,
	) => {
		setDimensions((prev) => {
			const updated = [...prev];
			updated[index] = { ...updated[index], [field]: value };
			return updated;
		});
	};

	const handleProcess = async () => {
		setIsProcessing(true);
		try {
			let successCount = 0;
			let failCount = 0;

			for (let i = 0; i < images.length; i++) {
				try {
					const result = await image_resize(
						images[i],
						dimensions[i].width,
						dimensions[i].height,
					);
					await download_file(result, toaster);
					successCount++;
				} catch (error) {
					console.error(`缩放第 ${i + 1} 张图片失败:`, error);
					failCount++;
				}
			}

			if (failCount > 0) {
				toaster.create({
					title: "部分图片处理失败",
					description: `成功: ${successCount} 张，失败: ${failCount} 张`,
					type: "warning",
					duration: 4000,
				});
			}

			onClose();
		} catch (error) {
			console.error("处理失败:", error);
			toaster.create({
				title: "处理失败",
				description: String(error),
				type: "error",
				duration: 4000,
			});
		} finally {
			setIsProcessing(false);
		}
	};

	return (
		<Modal isOpen={true} onClose={onClose} size="4xl">
			<div className="p-6">
				<h2 className="text-xl font-bold text-gray-900 dark:text-white mb-4">
					缩放图片
					{images.length > 1 && (
						<span className="text-sm font-normal text-gray-500 ml-2">
							({images.length} 张图片)
						</span>
					)}
				</h2>

				<div className="flex gap-4 mb-4">
					{/* 缩略图列表 */}
					{images.length > 1 && (
						<div className="flex flex-col gap-2 overflow-y-auto max-h-[60vh] w-24 shrink-0">
							{images.map((_, index) => (
								<button
									key={index}
									onClick={() => setSelectedIndex(index)}
									className={`relative aspect-square rounded-lg overflow-hidden border-2 transition-all ${
										selectedIndex === index
											? "border-pink-400/90 dark:border-pink-300/80 ring-2 ring-pink-200/60 dark:ring-pink-800/60"
											: "border-gray-200 dark:border-gray-600 hover:border-pink-300/80"
									}`}
								>
									<img
										src={imgSrcs[index]}
										alt={`缩略图 ${index + 1}`}
										className="w-full h-full object-cover"
									/>
									<div className="absolute bottom-0 left-0 right-0 bg-black/50 text-white text-xs py-0.5 text-center">
										{dimensions[index].width}×{dimensions[index].height}
									</div>
								</button>
							))}
						</div>
					)}

					{/* 预览和控制区域 */}
					<div className="flex-1 space-y-4">
						{/* 导航按钮 */}
						{images.length > 1 && (
							<div className="flex items-center justify-between">
								<button
									onClick={() => setSelectedIndex(selectedIndex - 1)}
									disabled={selectedIndex === 0}
									className="flex items-center gap-1 px-3 py-1.5 text-sm bg-gray-100 dark:bg-gray-700 hover:bg-gray-200 dark:hover:bg-gray-600 disabled:opacity-50 disabled:cursor-not-allowed rounded-lg transition-colors"
								>
									<ChevronLeft size={16} />
									上一张
								</button>
								<span className="text-sm text-gray-600 dark:text-gray-400">
									{selectedIndex + 1} / {images.length}
								</span>
								<button
									onClick={() => setSelectedIndex(selectedIndex + 1)}
									disabled={selectedIndex === images.length - 1}
									className="flex items-center gap-1 px-3 py-1.5 text-sm bg-gray-100 dark:bg-gray-700 hover:bg-gray-200 dark:hover:bg-gray-600 disabled:opacity-50 disabled:cursor-not-allowed rounded-lg transition-colors"
								>
									下一张
									<ChevronRight size={16} />
								</button>
							</div>
						)}

						{/* 图片预览 */}
						<div className="flex justify-center items-center bg-gray-50 dark:bg-gray-900 rounded-lg p-4 min-h-75">
							{imgSrcs[selectedIndex] && (
								<div className="relative">
									<img
										src={imgSrcs[selectedIndex]}
										alt="预览"
										className="max-h-100 max-w-full object-contain"
									/>
									<div className="absolute top-2 right-2 bg-black/70 text-white text-xs px-2 py-1 rounded">
										目标: {dimensions[selectedIndex].width}×
										{dimensions[selectedIndex].height}
									</div>
								</div>
							)}
						</div>

						{/* 尺寸控制 */}
						<div className="space-y-3">
							<div>
								<label className="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2">
									宽度 (px)
								</label>
								<input
									type="number"
									min="1"
									value={dimensions[selectedIndex].width}
									onChange={(e) =>
										handleDimensionChange(
											selectedIndex,
											"width",
											Number(e.target.value),
										)
									}
									className="w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-lg bg-white dark:bg-gray-700 text-gray-900 dark:text-white focus:ring-2 focus:ring-pink-400/90 focus:border-transparent"
								/>
							</div>
							<div>
								<label className="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2">
									高度 (px)
								</label>
								<input
									type="number"
									min="1"
									value={dimensions[selectedIndex].height}
									onChange={(e) =>
										handleDimensionChange(
											selectedIndex,
											"height",
											Number(e.target.value),
										)
									}
									className="w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-lg bg-white dark:bg-gray-700 text-gray-900 dark:text-white focus:ring-2 focus:ring-pink-400/90 focus:border-transparent"
								/>
							</div>

							{images.length > 1 && (
								<button
									onClick={() => {
										const currentDim = dimensions[selectedIndex];
										setDimensions(images.map(() => ({ ...currentDim })));
									}}
									className="w-full px-3 py-2 text-sm bg-gray-100 dark:bg-gray-700 hover:bg-gray-200 dark:hover:bg-gray-600 rounded-lg transition-colors"
								>
									应用到所有图片
								</button>
							)}
						</div>
					</div>
				</div>

				<div className="flex gap-2">
					<button
						onClick={onClose}
						className="flex-1 bg-gray-100 dark:bg-gray-700 hover:bg-gray-200 dark:hover:bg-gray-600 rounded-lg py-2.5 font-medium transition-colors"
					>
						取消
					</button>
					<button
						onClick={handleProcess}
						disabled={isProcessing}
						className="flex-1 bg-pink-400/90 dark:bg-pink-300/80 text-white hover:bg-pink-500/90 dark:hover:bg-pink-400/80 disabled:bg-gray-300 disabled:cursor-not-allowed rounded-lg py-2.5 font-medium transition-colors"
					>
						{isProcessing ? "处理中..." : "确认缩放"}
					</button>
				</div>
			</div>
		</Modal>
	);
};
