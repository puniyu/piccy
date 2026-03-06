import { useState } from "react";
import { image_grayscale, image_invert } from "@/utils/image.ts";
import { download_file } from "@/utils/file.ts";
import { useToaster } from "@/components/ui/toaster";
import { Modal } from "./Modal";

export const ImageFilterModal = ({
	images,
	filterType,
	onClose,
}: {
	images: File[];
	filterType: "grayscale" | "invert";
	onClose: () => void;
}) => {
	const toaster = useToaster();
	const [isProcessing, setIsProcessing] = useState(false);
	const [processedCount, setProcessedCount] = useState(0);

	const handleProcess = async () => {
		setIsProcessing(true);
		setProcessedCount(0);

		try {
			let successCount = 0;
			let failCount = 0;

			for (let i = 0; i < images.length; i++) {
				try {
					const result =
						filterType === "grayscale"
							? await image_grayscale(images[i])
							: await image_invert(images[i]);

					await download_file(result, toaster);
					successCount++;
					setProcessedCount(i + 1);
				} catch (error) {
					console.error(`处理第 ${i + 1} 张图片失败:`, error);
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

	const titles = {
		grayscale: "灰度化",
		invert: "反色",
	};

	return (
		<Modal isOpen={true} onClose={onClose} size="md">
			<div className="p-6">
				<h2 className="text-xl font-bold text-gray-900 dark:text-white mb-4">
					{titles[filterType]}
					{images.length > 1 && (
						<span className="text-sm font-normal text-gray-500 ml-2">
							({images.length} 张图片)
						</span>
					)}
				</h2>

				{isProcessing && images.length > 1 && (
					<div className="mb-4 p-3 bg-pink-50/60 dark:bg-pink-900/30 rounded-lg">
						<p className="text-sm text-pink-400/90 dark:text-pink-300/90">
							正在处理: {processedCount} / {images.length}
						</p>
					</div>
				)}

				<div className="mb-6">
					<p className="text-sm text-gray-600 dark:text-gray-400">
						点击确认按钮即可处理图片
					</p>
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
						{isProcessing ? "处理中..." : "确认"}
					</button>
				</div>
			</div>
		</Modal>
	);
};
