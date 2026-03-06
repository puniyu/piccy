import { useState, useRef, useEffect } from "react";
import { image_crop } from "@/utils/image.ts";
import { download_file } from "@/utils/file.ts";
import { useToaster } from "@/components/ui/toaster";
import ReactCrop, {
  type Crop,
  type PixelCrop,
  centerCrop,
  makeAspectCrop,
} from "react-image-crop";
import "react-image-crop/dist/ReactCrop.css";
import { Modal } from "./Modal";
import { ChevronLeft, ChevronRight, Copy } from "lucide-react";

const ASPECT_RATIOS = [
  { label: "自由", value: undefined },
  { label: "1:1", value: 1 },
  { label: "16:9", value: 16 / 9 },
  { label: "4:3", value: 4 / 3 },
  { label: "3:2", value: 3 / 2 },
];

interface CropData {
  crop: Crop;
  completedCrop: PixelCrop;
}

export const ImageCropModal = ({
  images,
  onClose,
}: {
  images: File[];
  onClose: () => void;
}) => {
  const toaster = useToaster();
  const [selectedIndex, setSelectedIndex] = useState(0);
  const [imgSrcs, setImgSrcs] = useState<string[]>([]);
  const imgRef = useRef<HTMLImageElement>(null);
  const [crop, setCrop] = useState<Crop>();
  const [completedCrop, setCompletedCrop] = useState<PixelCrop>();
  const [isProcessing, setIsProcessing] = useState(false);
  const [aspectRatio, setAspectRatio] = useState<number | undefined>(undefined);

  const [cropSettings, setCropSettings] = useState<Map<number, CropData>>(
    new Map(),
  );

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

  useEffect(() => {
    const savedCrop = cropSettings.get(selectedIndex);
    if (savedCrop) {
      setCrop(savedCrop.crop);
      setCompletedCrop(savedCrop.completedCrop);
    }
  }, [selectedIndex, cropSettings]);

  useEffect(() => {
    const handleKeyDown = (e: KeyboardEvent) => {
      if (e.key === "ArrowLeft" && selectedIndex > 0) {
        e.preventDefault();
        goToPrevious();
      } else if (e.key === "ArrowRight" && selectedIndex < images.length - 1) {
        e.preventDefault();
        goToNext();
      }
    };

    window.addEventListener("keydown", handleKeyDown);
    return () => window.removeEventListener("keydown", handleKeyDown);
  }, [selectedIndex, images.length, crop, completedCrop]);

  function onImageLoad(e: React.SyntheticEvent<HTMLImageElement>) {
    const { width, height } = e.currentTarget;

    // 如果已有保存的裁剪设置，不需要重新初始化
    const savedCrop = cropSettings.get(selectedIndex);
    if (savedCrop && crop) {
      return;
    }

    // 创建默认裁剪区域
    const newCrop = aspectRatio
      ? makeAspectCrop(
          {
            unit: "%",
            width: 90,
          },
          aspectRatio,
          width,
          height,
        )
      : centerCrop(
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
    setCrop(newCrop);
  }

  const handleAspectRatioChange = (ratio: number | undefined) => {
    setAspectRatio(ratio);
    if (imgRef.current && ratio) {
      const { width, height } = imgRef.current;
      const newCrop = makeAspectCrop(
        {
          unit: "%",
          width: 90,
        },
        ratio,
        width,
        height,
      );
      setCrop(newCrop);
    }
  };

  // 保存当前图片的裁剪设置
  const saveCropSetting = () => {
    if (crop && completedCrop) {
      setCropSettings((prev) => {
        const newMap = new Map(prev);
        newMap.set(selectedIndex, { crop, completedCrop });
        return newMap;
      });
    }
  };

  // 导航到上一张
  const goToPrevious = () => {
    if (selectedIndex > 0) {
      if (crop && completedCrop) {
        saveCropSetting();
      }
      setSelectedIndex(selectedIndex - 1);
    }
  };

  // 导航到下一张
  const goToNext = () => {
    if (selectedIndex < images.length - 1) {
      if (crop && completedCrop) {
        saveCropSetting();
      }
      setSelectedIndex(selectedIndex + 1);
    }
  };

  // 跳到下一张未设置的图片
  const goToNextUnconfigured = () => {
    if (crop && completedCrop) {
      saveCropSetting();
    }

    for (let i = selectedIndex + 1; i < images.length; i++) {
      if (!cropSettings.has(i)) {
        setSelectedIndex(i);
        return;
      }
    }

    // 如果后面没有未设置的，从头开始找
    for (let i = 0; i < selectedIndex; i++) {
      if (!cropSettings.has(i)) {
        setSelectedIndex(i);
        return;
      }
    }

    toaster.create({
      title: "所有图片已设置",
      description: "所有图片都已设置裁剪区域",
      type: "info",
      duration: 2000,
    });
  };

  // 应用当前裁剪到所有图片
  const applyToAll = () => {
    if (!crop || !completedCrop) {
      toaster.create({
        title: "请先设置裁剪区域",
        type: "warning",
        duration: 2000,
      });
      return;
    }

    const newSettings = new Map<number, CropData>();
    for (let i = 0; i < images.length; i++) {
      newSettings.set(i, { crop, completedCrop });
    }
    setCropSettings(newSettings);

    toaster.create({
      title: "已应用到所有图片",
      description: `${images.length} 张图片已设置相同裁剪区域`,
      type: "success",
      duration: 2000,
    });
  };

  // 处理所有图片
  const handleProcessAll = async () => {
    // 构建完整的裁剪设置 Map，包含当前正在编辑的图片
    const finalCropSettings = new Map(cropSettings);
    if (crop && completedCrop) {
      finalCropSettings.set(selectedIndex, { crop, completedCrop });
    }

    if (finalCropSettings.size === 0) {
      toaster.create({
        title: "没有可处理的图片",
        description: "请至少为一张图片设置裁剪区域",
        type: "warning",
        duration: 3000,
      });
      return;
    }

    setIsProcessing(true);
    let successCount = 0;
    let failCount = 0;

    try {
      for (let i = 0; i < images.length; i++) {
        const cropData = finalCropSettings.get(i);
        if (!cropData) {
          continue;
        }

        try {
          const img = new Image();
          img.src = imgSrcs[i];
          await new Promise((resolve) => {
            img.onload = resolve;
          });

          const scaleX = img.naturalWidth / img.width;
          const scaleY = img.naturalHeight / img.height;

          const actualX = cropData.completedCrop.x * scaleX;
          const actualY = cropData.completedCrop.y * scaleY;
          const actualWidth = cropData.completedCrop.width * scaleX;
          const actualHeight = cropData.completedCrop.height * scaleY;

          const result = await image_crop(images[i], {
            left: Math.round(actualX),
            top: Math.round(actualY),
            width: Math.round(actualWidth),
            height: Math.round(actualHeight),
          });
          await download_file(result, toaster);
          successCount++;
        } catch (error) {
          console.error(`裁剪第 ${i + 1} 张图片失败:`, error);
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
      } else {
        toaster.create({
          title: "裁剪完成",
          description: `已处理 ${successCount} 张图片`,
          type: "success",
          duration: 3000,
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

  const cropInfo = completedCrop
    ? `${Math.round(completedCrop.width)} × ${Math.round(completedCrop.height)} px`
    : "";

  const configuredCount = cropSettings.size + (crop && completedCrop ? 1 : 0);
  const unconfiguredCount = images.length - cropSettings.size;

  return (
    <Modal isOpen={true} onClose={onClose} size="4xl">
      <div className="p-6">
        <h2 className="text-xl font-bold text-gray-900 dark:text-white mb-4">
          图片裁剪
          {images.length > 1 && (
            <span className="text-sm font-normal text-gray-500 ml-2">
              ({configuredCount}/{images.length} 张已设置)
            </span>
          )}
        </h2>

        {/* 裁剪比例选择 */}
        <div className="mb-4 space-y-3">
          <div className="flex items-center gap-2 flex-wrap">
            <span className="text-sm font-medium text-gray-700 dark:text-gray-300">
              裁剪比例:
            </span>
            {ASPECT_RATIOS.map((ratio) => (
              <button
                key={ratio.label}
                onClick={() => handleAspectRatioChange(ratio.value)}
                className={`px-3 py-1 text-sm rounded-lg transition-colors ${
                  aspectRatio === ratio.value
                    ? "bg-pink-400/90 dark:bg-pink-300/80 text-white"
                    : "bg-gray-100 dark:bg-gray-700 text-gray-700 dark:text-gray-300 hover:bg-gray-200 dark:hover:bg-gray-600"
                }`}
              >
                {ratio.label}
              </button>
            ))}
          </div>

          {images.length > 1 && (
            <div className="flex gap-2">
              <button
                onClick={applyToAll}
                disabled={!crop || !completedCrop}
                className="flex items-center gap-1 px-3 py-1.5 text-sm bg-purple-300/80 dark:bg-purple-400/70 text-white hover:bg-purple-400/80 dark:hover:bg-purple-500/70 disabled:bg-gray-300 disabled:cursor-not-allowed rounded-lg transition-colors"
              >
                <Copy size={14} />
                应用到所有图片
              </button>
              {unconfiguredCount > 1 && (
                <button
                  onClick={goToNextUnconfigured}
                  className="px-3 py-1.5 text-sm bg-pink-300/80 dark:bg-pink-400/70 text-white hover:bg-pink-400/80 dark:hover:bg-pink-500/70 rounded-lg transition-colors"
                >
                  跳到下一张未设置 ({unconfiguredCount - 1} 张)
                </button>
              )}
            </div>
          )}
        </div>

        <div className="flex gap-4 mb-4">
          {/* 缩略图列表 */}
          {images.length > 1 && (
            <div className="flex flex-col gap-2 overflow-y-auto max-h-[60vh] w-24 shrink-0">
              {images.map((_, index) => (
                <button
                  key={index}
                  onClick={() => {
                    if (crop && completedCrop && selectedIndex !== index) {
                      saveCropSetting();
                    }
                    setSelectedIndex(index);
                  }}
                  className={`relative aspect-square rounded-lg overflow-hidden border-2 transition-all ${
                    selectedIndex === index
                      ? "border-pink-400/90 dark:border-pink-300/80 ring-2 ring-pink-200/60 dark:ring-pink-800/60"
                      : cropSettings.has(index)
                        ? "border-pink-300/80 dark:border-pink-400/70"
                        : "border-gray-200 dark:border-gray-600 hover:border-pink-300/80"
                  }`}
                >
                  <img
                    src={imgSrcs[index]}
                    alt={`缩略图 ${index + 1}`}
                    className="w-full h-full object-cover"
                  />
                  <div className="absolute bottom-0 left-0 right-0 bg-black/50 text-white text-xs py-0.5 text-center">
                    {cropSettings.has(index) ? "✓" : index + 1}
                  </div>
                </button>
              ))}
            </div>
          )}

          {/* 裁剪区域 */}
          <div className="flex-1 space-y-2">
            {/* 导航按钮 */}
            {images.length > 1 && (
              <div className="flex items-center justify-between mb-2">
                <button
                  onClick={goToPrevious}
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
                  onClick={goToNext}
                  disabled={selectedIndex === images.length - 1}
                  className="flex items-center gap-1 px-3 py-1.5 text-sm bg-gray-100 dark:bg-gray-700 hover:bg-gray-200 dark:hover:bg-gray-600 disabled:opacity-50 disabled:cursor-not-allowed rounded-lg transition-colors"
                >
                  下一张
                  <ChevronRight size={16} />
                </button>
              </div>
            )}

            <div className="flex justify-center items-center overflow-hidden max-h-[55vh] bg-gray-50 dark:bg-gray-900 rounded-lg p-4">
              {imgSrcs[selectedIndex] && (
                <ReactCrop
                  crop={crop}
                  onChange={(_, percentCrop) => {
                    setCrop(percentCrop);
                  }}
                  onComplete={(c) => {
                    setCompletedCrop(c);
                  }}
                  aspect={aspectRatio}
                >
                  <img
                    ref={imgRef}
                    alt="Crop preview"
                    src={imgSrcs[selectedIndex]}
                    style={{
                      maxHeight: "50vh",
                      objectFit: "contain",
                      borderRadius: "8px",
                    }}
                    onLoad={onImageLoad}
                  />
                </ReactCrop>
              )}
            </div>
            {cropInfo && (
              <p className="text-sm text-center text-gray-600 dark:text-gray-400">
                裁剪尺寸: {cropInfo}
              </p>
            )}
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
            onClick={handleProcessAll}
            disabled={configuredCount === 0 || isProcessing}
            className="flex-1 bg-pink-400/90 dark:bg-pink-300/80 text-white hover:bg-pink-500/90 dark:hover:bg-pink-400/80 disabled:bg-gray-300 disabled:cursor-not-allowed rounded-lg py-2.5 font-medium transition-colors"
          >
            {isProcessing
              ? `处理中...`
              : `确认裁剪 (${configuredCount}/${images.length})`}
          </button>
        </div>
      </div>
    </Modal>
  );
};
