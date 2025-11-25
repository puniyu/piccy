// oxlint-disable arrow-body-style
import { Button } from "@chakra-ui/react";
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
        <Button
          className="!px-6 !h-12 !bg-gray-100 dark:!bg-gray-700 !text-gray-700 dark:!text-gray-200 hover:!bg-gray-200 dark:hover:!bg-gray-600 !rounded-xl transition-colors flex items-center justify-center gap-2 font-medium"
          onClick={async () => {
            const info = await image_info(image[0]);
            setImageInfo(info);
            setShowImageInfo(true);
          }}
        >
          <Info size={18} />
          查看信息
        </Button>

        <Button
          className="!px-6 !h-12 !bg-gray-900 dark:!bg-white !text-white dark:!text-gray-900 hover:!bg-gray-800 dark:hover:!bg-gray-100 !rounded-xl transition-colors flex items-center justify-center gap-2 font-medium shadow-sm"
          onClick={() => setShowImageCrop(true)}
        >
          <Crop size={18} />
          裁剪图片
        </Button>
      </div>

      {/* 渲染ImageInfoCard组件 */}
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
