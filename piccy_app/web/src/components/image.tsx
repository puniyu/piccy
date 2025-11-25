import { CloseButton, Dialog, Portal, Button } from "@chakra-ui/react";
import { image_crop, ImageInfo } from "@/utils/image.ts";
import { useState, useRef, useEffect } from "react";
import { download_file } from "@/utils/file.ts";
import ReactCrop, {
  type Crop,
  type PixelCrop,
  centerCrop,
} from "react-image-crop";
import "react-image-crop/dist/ReactCrop.css";

export const ImageInfoCard = ({
  imageInfo,
  onClose,
}: {
  imageInfo: ImageInfo;
  onClose: () => void;
}) => {
  return (
    <Dialog.Root open={true}>
      <Portal>
        <Dialog.Backdrop className="bg-black/60 backdrop-blur-sm" />
        <Dialog.Positioner>
          <Dialog.Content className="!rounded-xl !shadow-2xl !bg-white dark:!bg-gray-800 !border !border-gray-200 dark:!border-gray-700">
            <Dialog.Header className="pb-3">
              <Dialog.Title className="text-xl font-bold text-gray-900 dark:text-white">
                图片信息
              </Dialog.Title>
            </Dialog.Header>
            <Dialog.Body className="py-4">
              <div className="grid gap-3 bg-gray-50 dark:bg-gray-900 p-4 rounded-lg">
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
            </Dialog.Body>
            <Dialog.Footer className="pt-3">
              <Button
                onClick={onClose}
                className="!w-full !bg-blue-500 !text-white hover:!bg-blue-600 !rounded-lg"
              >
                确定
              </Button>
            </Dialog.Footer>
            <Dialog.CloseTrigger asChild>
              <CloseButton
                size="sm"
                onClick={onClose}
                className="absolute top-3 right-3 text-gray-500 hover:bg-gray-100 dark:hover:bg-gray-700 !rounded-full"
              />
            </Dialog.CloseTrigger>
          </Dialog.Content>
        </Dialog.Positioner>
      </Portal>
    </Dialog.Root>
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
    <Dialog.Root open={true}>
      <Portal>
        <Dialog.Backdrop className="bg-black/60 backdrop-blur-sm" />
        <Dialog.Positioner>
          <Dialog.Content className="!rounded-xl !shadow-2xl !bg-white dark:!bg-gray-800 !border !border-gray-200 dark:!border-gray-700 !max-w-4xl">
            <Dialog.Header className="pb-3">
              <Dialog.Title className="text-xl font-bold text-gray-900 dark:text-white">
                图片裁剪
              </Dialog.Title>
            </Dialog.Header>
            <Dialog.Body className="py-4">
              <div className="flex justify-center items-center overflow-hidden max-h-[70vh] bg-gray-50 dark:bg-gray-900 rounded-lg p-4">
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
            </Dialog.Body>
            <Dialog.Footer className="pt-3 gap-2 flex">
              <Button
                variant="outline"
                onClick={onClose}
                className="flex-1 !border-gray-300 hover:!bg-gray-50 dark:!border-gray-600 dark:hover:!bg-gray-700"
              >
                取消
              </Button>
              <Button
                onClick={handleCrop}
                className="flex-1 !bg-blue-500 !text-white hover:!bg-blue-600"
                loading={isProcessing}
                disabled={!completedCrop}
              >
                {isProcessing ? "处理中..." : "确认裁剪"}
              </Button>
            </Dialog.Footer>
            <Dialog.CloseTrigger asChild>
              <CloseButton
                size="sm"
                onClick={onClose}
                className="absolute top-3 right-3 text-gray-500 hover:bg-gray-100 dark:hover:bg-gray-700 !rounded-full"
              />
            </Dialog.CloseTrigger>
          </Dialog.Content>
        </Dialog.Positioner>
      </Portal>
    </Dialog.Root>
  );
};
