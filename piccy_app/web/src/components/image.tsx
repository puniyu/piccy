import { X } from "lucide-react";
import { image_crop, ImageInfo } from "@/utils/image.ts";
import { useState, useRef, useEffect } from "react";
import { download_file } from "@/utils/file.ts";
import ReactCrop, {
	type Crop,
	type PixelCrop,
	centerCrop,
} from "react-image-crop";
import "react-image-crop/dist/ReactCrop.css";
import { motion, AnimatePresence } from "motion/react";

interface ModalProps {
	isOpen: boolean;
	onClose: () => void;
	children: React.ReactNode;
	size?: "md" | "4xl";
}

const Modal = ({ isOpen, onClose, children, size = "md" }: ModalProps) => {
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
						className="fixed inset-0 bg-black/60 backdrop-blur-sm"
						onClick={onClose}
					/>
					<motion.div
						initial={{ opacity: 0, scale: 0.95 }}
						animate={{ opacity: 1, scale: 1 }}
						exit={{ opacity: 0, scale: 0.95 }}
						transition={{ type: "spring", damping: 25, stiffness: 300 }}
						className={`relative bg-white dark:bg-gray-800 rounded-xl shadow-2xl border border-gray-200 dark:border-gray-700 w-full ${sizeClasses[size]} max-h-[90vh] overflow-hidden`}
					>
						<button
							onClick={onClose}
							className="absolute top-3 right-3 z-10 w-8 h-8 rounded-full flex items-center justify-center text-gray-500 hover:bg-gray-100 dark:hover:bg-gray-700 transition-colors"
						>
							<X size={18} />
						</button>
						{children}
					</motion.div>
				</div>
			)}
		</AnimatePresence>
	);
};

export const ImageInfoCard = ({
	imageInfo,
	onClose,
}: {
	imageInfo: ImageInfo;
	onClose: () => void;
}) => {
	return (
		<Modal isOpen={true} onClose={onClose} size="md">
			<div className="p-6">
				<h2 className="text-xl font-bold text-gray-900 dark:text-white mb-4">
					图片信息
				</h2>
				<div className="grid gap-3 bg-gray-50 dark:bg-gray-900 p-4 rounded-lg mb-4">
					<div className="flex justify-between items-center">
						<span className="font-medium text-gray-600 dark:text-gray-400">
							宽度
						</span>
						<span className="font-semibold text-gray-900 dark:text-gray-100">
							{imageInfo.width} px
						</span>
					</div>
					<div className="flex justify-between items-center">
						<span className="font-medium text-gray-600 dark:text-gray-400">
							高度
						</span>
						<span className="font-semibold text-gray-900 dark:text-gray-100">
							{imageInfo.height} px
						</span>
					</div>
					<div className="flex justify-between items-center">
						<span className="font-medium text-gray-600 dark:text-gray-400">
							动图
						</span>
						<span
							className={`font-semibold ${imageInfo.is_multi_frame ? "text-green-500" : "text-gray-500"}`}
						>
							{imageInfo.is_multi_frame ? "是" : "否"}
						</span>
					</div>
					{imageInfo.is_multi_frame && (
						<>
							<div className="flex justify-between items-center">
								<span className="font-medium text-gray-600 dark:text-gray-400">
									帧数
								</span>
								<span className="font-semibold text-gray-900 dark:text-gray-100">
									{imageInfo.frame_count}
								</span>
							</div>
							<div className="flex justify-between items-center">
								<span className="font-medium text-gray-600 dark:text-gray-400">
									平均帧间隔
								</span>
								<span className="font-semibold text-gray-900 dark:text-gray-100">
									{imageInfo.average_duration} ms
								</span>
							</div>
						</>
					)}
				</div>
				<button
					onClick={onClose}
					className="w-full bg-blue-500 text-white hover:bg-blue-600 rounded-lg py-2.5 font-medium transition-colors"
				>
					确定
				</button>
			</div>
		</Modal>
	);
};

export const ImageCropCard = ({
	image_data,
	onClose,
}: {
	image_data: File;
	onClose: () => void;
}) => {
	const [imgSrc, setImgSrc] = useState<string>("");
	const imgRef = useRef<HTMLImageElement>(null);
	const [crop, setCrop] = useState<Crop>();
	const [completedCrop, setCompletedCrop] = useState<PixelCrop>();
	const [isProcessing, setIsProcessing] = useState(false);

	useEffect(() => {
		const reader = new FileReader();
		reader.addEventListener("load", () => {
			setImgSrc(reader.result?.toString() || "");
		});
		reader.readAsDataURL(image_data);
	}, [image_data]);

	function onImageLoad(e: React.SyntheticEvent<HTMLImageElement>) {
		const { width, height } = e.currentTarget;
		const crop = centerCrop(
			{
				unit: "%",
				width: 90,
				height: 90,
				x: 0,
				y: 0,
			},
			width,
			height,
		);
		setCrop(crop);
	}

	const handleCrop = async () => {
		if (completedCrop && imgRef.current) {
			setIsProcessing(true);
			try {
				const image = imgRef.current;
				const scaleX = image.naturalWidth / image.width;
				const scaleY = image.naturalHeight / image.height;

				const actualX = completedCrop.x * scaleX;
				const actualY = completedCrop.y * scaleY;
				const actualWidth = completedCrop.width * scaleX;
				const actualHeight = completedCrop.height * scaleY;

				const result = await image_crop(image_data, {
					left: Math.round(actualX),
					top: Math.round(actualY),
					width: Math.round(actualWidth),
					height: Math.round(actualHeight),
				});
				await download_file(result);
				onClose();
			} catch (error) {
				console.error("裁剪失败:", error);
			} finally {
				setIsProcessing(false);
			}
		}
	};

	return (
		<Modal isOpen={true} onClose={onClose} size="4xl">
			<div className="p-6">
				<h2 className="text-xl font-bold text-gray-900 dark:text-white mb-4">
					图片裁剪
				</h2>
				<div className="flex justify-center items-center overflow-hidden max-h-[70vh] bg-gray-50 dark:bg-gray-900 rounded-lg p-4 mb-4">
					{imgSrc && (
						<ReactCrop
							crop={crop}
							onChange={(_, percentCrop) => {
								setCrop(percentCrop);
							}}
							onComplete={(c) => {
								setCompletedCrop(c);
							}}
						>
							<img
								ref={imgRef}
								alt="Crop me"
								src={imgSrc}
								style={{
									maxHeight: "65vh",
									objectFit: "contain",
									borderRadius: "8px",
								}}
								onLoad={onImageLoad}
							/>
						</ReactCrop>
					)}
				</div>
				<div className="flex gap-2">
					<button
						onClick={onClose}
						className="flex-1 border border-gray-300 dark:border-gray-600 hover:bg-gray-50 dark:hover:bg-gray-700 rounded-lg py-2.5 font-medium transition-colors"
					>
						取消
					</button>
					<button
						onClick={handleCrop}
						disabled={!completedCrop || isProcessing}
						className="flex-1 bg-blue-500 text-white hover:bg-blue-600 disabled:bg-gray-300 disabled:cursor-not-allowed rounded-lg py-2.5 font-medium transition-colors"
					>
						{isProcessing ? "处理中..." : "确认裁剪"}
					</button>
				</div>
			</div>
		</Modal>
	);
};
