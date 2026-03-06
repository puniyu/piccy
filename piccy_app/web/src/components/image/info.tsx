import { useState, useEffect } from "react";
import { image_info, ImageInfo } from "@/utils/image.ts";
import { Modal } from "./Modal";

export const ImageInfoListModal = ({
	images,
	onClose,
}: {
	images: File[];
	onClose: () => void;
}) => {
	const [selectedIndex, setSelectedIndex] = useState(0);
	const [imageInfos, setImageInfos] = useState<(ImageInfo | null)[]>([]);
	const [loading, setLoading] = useState(true);

	useEffect(() => {
		const loadInfos = async () => {
			setLoading(true);
			const infos = await Promise.all(
				images.map(async (img) => {
					try {
						return await image_info(img);
					} catch {
						return null;
					}
				}),
			);
			setImageInfos(infos);
			setLoading(false);
		};
		loadInfos();
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

	const currentInfo = imageInfos[selectedIndex];

	return (
		<Modal isOpen={true} onClose={onClose} size="4xl">
			<div className="p-6">
				<h2 className="text-xl font-bold text-gray-900 dark:text-white mb-4">
					图片信息 ({images.length} 张)
				</h2>

				<div className="flex gap-4 mb-4">
					{/* 缩略图列表 */}
					<div className="flex flex-col gap-2 overflow-y-auto max-h-[60vh] w-32 shrink-0">
						{images.map((img, index) => (
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
									src={URL.createObjectURL(img)}
									alt={`缩略图 ${index + 1}`}
									className="w-full h-full object-cover"
								/>
								<div className="absolute bottom-0 left-0 right-0 bg-black/50 text-white text-xs py-0.5 text-center">
									{index + 1}
								</div>
							</button>
						))}
					</div>

					{/* 信息显示区域 */}
					<div className="flex-1">
						{loading ? (
							<div className="flex items-center justify-center h-64">
								<p className="text-gray-500">加载中...</p>
							</div>
						) : currentInfo ? (
							<div className="space-y-4">
								<div className="bg-gray-50 dark:bg-gray-900 p-4 rounded-lg">
									<div className="grid gap-3">
										<div className="flex justify-between items-center">
											<span className="font-medium text-gray-600 dark:text-gray-400">
												文件名
											</span>
											<span className="font-semibold text-gray-900 dark:text-gray-100 truncate max-w-xs">
												{images[selectedIndex].name}
											</span>
										</div>
										<div className="flex justify-between items-center">
											<span className="font-medium text-gray-600 dark:text-gray-400">
												宽度
											</span>
											<span className="font-semibold text-gray-900 dark:text-gray-100">
												{currentInfo.width} px
											</span>
										</div>
										<div className="flex justify-between items-center">
											<span className="font-medium text-gray-600 dark:text-gray-400">
												高度
											</span>
											<span className="font-semibold text-gray-900 dark:text-gray-100">
												{currentInfo.height} px
											</span>
										</div>
										<div className="flex justify-between items-center">
											<span className="font-medium text-gray-600 dark:text-gray-400">
												文件大小
											</span>
											<span className="font-semibold text-gray-900 dark:text-gray-100">
												{(images[selectedIndex].size / 1024).toFixed(2)} KB
											</span>
										</div>
										<div className="flex justify-between items-center">
											<span className="font-medium text-gray-600 dark:text-gray-400">
												动图
											</span>
											<span
												className={`font-semibold ${currentInfo.is_multi_frame ? "text-pink-400/90 dark:text-pink-300/80" : "text-gray-500"}`}
											>
												{currentInfo.is_multi_frame ? "是" : "否"}
											</span>
										</div>
										{currentInfo.is_multi_frame && (
											<>
												<div className="flex justify-between items-center">
													<span className="font-medium text-gray-600 dark:text-gray-400">
														帧数
													</span>
													<span className="font-semibold text-gray-900 dark:text-gray-100">
														{currentInfo.frame_count}
													</span>
												</div>
												<div className="flex justify-between items-center">
													<span className="font-medium text-gray-600 dark:text-gray-400">
														平均帧间隔
													</span>
													<span className="font-semibold text-gray-900 dark:text-gray-100">
														{currentInfo.average_duration} ms
													</span>
												</div>
											</>
										)}
									</div>
								</div>
							</div>
						) : (
							<div className="flex items-center justify-center h-64">
								<p className="text-red-500">加载失败</p>
							</div>
						)}
					</div>
				</div>

				<div className="flex justify-end">
					<button
						onClick={onClose}
						className="px-6 py-2 bg-pink-400/90 dark:bg-pink-300/80 text-white hover:bg-pink-500/90 dark:hover:bg-pink-400/80 rounded-lg font-medium transition-colors"
					>
						关闭
					</button>
				</div>
			</div>
		</Modal>
	);
};
