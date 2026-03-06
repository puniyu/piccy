import { useState, useEffect } from "react";
import { image_rotate } from "@/utils/image.ts";
import { download_file } from "@/utils/file.ts";
import { useToaster } from "@/components/ui/toaster";
import { Modal } from "./Modal";
import { ChevronLeft, ChevronRight } from "lucide-react";

export const ImageRotateModal = ({
	images,
	onClose,
}: {
	images: File[];
	onClose: () => void;
}) => {
	const toaster = useToaster();
	const [isProcessing, setIsProcessing] = useState(false);
	const [selectedIndex, setSelectedIndex] = useState(0);
	const [angles, setAngles] = useState<number[]>(images.map(() => 0));
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

	const handleAngleChange = (index: number, newAngle: number) => {
		setAngles((prev) => {
			const updated = [...prev];
			updated[index] = newAngle;
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
					const result = await image_rotate(images[i], angles[i]);
					await download_file(result, toaster);
					successCount++;
				} catch (error) {
					console.error(`旋转第 ${i + 1} 张图片失败:`, error);
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
					旋转图片
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
										style={{
											transform: `rotate(${angles[index]}deg)`,
										}}
									/>
									<div className="absolute bottom-0 left-0 right-0 bg-black/50 text-white text-xs py-0.5 text-center">
										{angles[index]}°
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
								<img
									src={imgSrcs[selectedIndex]}
									alt="预览"
									className="max-h-100 max-w-full object-contain transition-transform duration-300"
									style={{
										transform: `rotate(${angles[selectedIndex]}deg)`,
									}}
								/>
							)}
						</div>

						{/* 角度控制 */}
						<div className="space-y-3">
							<div className="flex items-center gap-3">
								<label className="text-sm font-medium text-gray-700 dark:text-gray-300 w-16">
									角度
								</label>
								<input
									type="range"
									min="0"
									max="360"
									step="1"
									value={angles[selectedIndex]}
									onChange={(e) =>
										handleAngleChange(selectedIndex, Number(e.target.value))
									}
									className="flex-1"
								/>
								<input
									type="number"
									min="0"
									max="360"
									value={angles[selectedIndex]}
									onChange={(e) =>
										handleAngleChange(selectedIndex, Number(e.target.value))
									}
									className="w-20 px-2 py-1 border border-gray-300 dark:border-gray-600 rounded bg-white dark:bg-gray-700 text-gray-900 dark:text-white text-center"
								/>
								<span className="text-sm text-gray-500">°</span>
							</div>

							{/* 快捷按钮 */}
							<div className="flex gap-2">
								{[0, 90, 180, 270].map((deg) => (
									<button
										key={deg}
										onClick={() => handleAngleChange(selectedIndex, deg)}
										className={`flex-1 px-3 py-2 rounded-lg font-medium transition-colors ${
											angles[selectedIndex] === deg
												? "bg-pink-400/90 dark:bg-pink-300/80 text-white"
												: "bg-gray-100 dark:bg-gray-700 text-gray-700 dark:text-gray-300 hover:bg-gray-200 dark:hover:bg-gray-600"
										}`}
									>
										{deg}°
									</button>
								))}
							</div>

							{images.length > 1 && (
								<button
									onClick={() => {
										const currentAngle = angles[selectedIndex];
										setAngles(images.map(() => currentAngle));
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
						{isProcessing ? "处理中..." : "确认旋转"}
					</button>
				</div>
			</div>
		</Modal>
	);
};
