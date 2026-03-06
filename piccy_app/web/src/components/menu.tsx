import { Info, Crop } from "lucide-react";
import { useState } from "react";
import { image_info, ImageInfo } from "@/utils/image.ts";
import { ImageCropCard, ImageInfoCard } from "@/components/image.tsx";

export const ImageMenu = ({ image }: { image: Array<File> }) => {
	const [imageInfo, setImageInfo] = useState<ImageInfo | null>(null);
	const [showImageInfo, setShowImageInfo] = useState(false);
	const [showImageCrop, setShowImageCrop] = useState(false);

	return (
		<>
			<div className="flex flex-wrap gap-3 justify-center">
				<button
					onClick={async () => {
						const info = await image_info(image[0]);
						setImageInfo(info);
						setShowImageInfo(true);
					}}
					className="px-6 h-12 bg-gray-100 dark:bg-gray-700 text-gray-700 dark:text-gray-200 hover:bg-gray-200 dark:hover:bg-gray-600 rounded-xl transition-colors flex items-center justify-center gap-2 font-medium"
				>
					<Info size={18} />
					查看信息
				</button>

				<button
					onClick={() => setShowImageCrop(true)}
					className="px-6 h-12 bg-blue-500 text-white hover:bg-blue-600 rounded-xl transition-colors flex items-center justify-center gap-2 font-medium shadow-sm"
				>
					<Crop size={18} />
					裁剪图片
				</button>
			</div>

			{showImageInfo && imageInfo && (
				<ImageInfoCard
					imageInfo={imageInfo}
					onClose={() => setShowImageInfo(false)}
				/>
			)}

			{showImageCrop && (
				<ImageCropCard
					image_data={image[0]}
					onClose={() => setShowImageCrop(false)}
				/>
			)}
		</>
	);
};
