import { Heart, Sparkles } from "lucide-react";
import { useEffect } from "react";
import { ImageMenu } from "@/components/menu.tsx";
import { FileUploadList } from "@/components/file";
import { ColorModeButton } from "@/components/ui/color-mode";
import { motion, AnimatePresence } from "motion/react";
import { useFileUpload } from "@/hooks/useFileUpload";
import { TitleBar } from "@/components/TitleBar";

export default function App() {
  const {
    uploadedFiles,
    isDragging,
    removeFile,
    dropZoneProps,
    fileInputProps,
  } = useFileUpload();

  useEffect(() => {
    if (!import.meta.env.DEV) {
      document.addEventListener("contextmenu", (e) => e.preventDefault());
    }
  }, []);

  return (
    <div className="w-full h-screen flex flex-col relative overflow-hidden bg-linear-to-br from-pink-50 via-purple-50/30 to-pink-50 dark:from-gray-950 dark:via-pink-950/20 dark:to-gray-950">
      <TitleBar />

      {/* 柔和背景装饰 */}
      <div className="absolute inset-0 overflow-hidden pointer-events-none">
        <div className="absolute -top-20 -left-20 w-96 h-96 bg-linear-to-br from-pink-200/30 to-purple-200/30 dark:from-pink-500/8 dark:to-purple-500/8 rounded-full blur-3xl" />
        <div className="absolute -bottom-32 -right-32 w-125 h-125 bg-linear-to-tl from-purple-200/30 to-pink-200/30 dark:from-purple-500/8 dark:to-pink-500/8 rounded-full blur-3xl" />
      </div>

      <div className="flex-1 flex flex-col relative z-10 overflow-hidden">
        <div className="absolute top-4 right-4 z-10">
          <ColorModeButton />
        </div>

        {/* 内容区域 */}
        <div className="flex-1 px-8 py-8 overflow-y-auto min-h-0">
          <input
            type="file"
            id="file-upload"
            multiple
            accept="image/*"
            {...fileInputProps}
            className="hidden"
          />

          <AnimatePresence mode="wait">
            {uploadedFiles.length === 0 ? (
              <motion.div
                key="upload-trigger"
                initial={{ opacity: 0, scale: 0.9 }}
                animate={{ opacity: 1, scale: 1 }}
                exit={{ opacity: 0, scale: 0.9 }}
                transition={{ duration: 0.4, ease: "easeOut" }}
                className="h-full flex flex-col items-center justify-center gap-8"
              >
                {/* 标题 */}
                <div className="text-center space-y-3 select-none">
                  <div className="flex items-center justify-center gap-3">
                    <Heart size={32} className="text-pink-300/80 dark:text-pink-300/70 fill-pink-300/80 dark:fill-pink-300/70" />
                    <h1 className="text-5xl font-bold text-pink-400/90 dark:text-pink-300/80">
                      Piccy
                    </h1>
                    <Sparkles size={20} className="text-purple-300/80 dark:text-purple-300/70" />
                  </div>
                  <p className="text-pink-300/80 dark:text-pink-300/70 text-base font-medium">
                    可爱的图片处理工具 ♡
                  </p>
                </div>

                {/* 上传区域 */}
                <label htmlFor="file-upload" className="w-full max-w-2xl">
                  <div
                    {...dropZoneProps}
                    className={`w-full py-28 border-3 border-dashed rounded-[2rem] cursor-pointer transition-all duration-200 backdrop-blur-lg shadow-lg ${
                      isDragging
                        ? "border-pink-300/80 dark:border-pink-400/60 bg-white/70 dark:bg-pink-900/30 shadow-xl"
                        : "border-pink-300/50 dark:border-pink-600/40 bg-white/50 dark:bg-pink-950/20 hover:border-pink-300/70 dark:hover:border-pink-500/50 hover:bg-white/60 dark:hover:bg-pink-900/25"
                    }`}
                  >
                    <div className="flex flex-col items-center gap-7">
                      <div className="p-8 bg-linear-to-br from-pink-200/60 to-purple-200/60 dark:from-pink-400/20 dark:to-purple-400/20 rounded-full shadow-lg backdrop-blur-sm">
                        <Heart size={64} className="text-pink-400/90 dark:text-pink-300/80" strokeWidth={1.5} />
                      </div>
                      <div className="space-y-3">
                        <p className="text-2xl font-bold text-pink-400/90 dark:text-pink-300/80">
                          点击或拖拽上传图片
                        </p>
                        <p className="text-sm text-pink-300/90 dark:text-pink-400/70 text-center font-medium">
                          支持 JPG、PNG、GIF 等格式 · 最多 20 张
                        </p>
                      </div>
                    </div>
                  </div>
                </label>
              </motion.div>
            ) : (
              <motion.div
                key="upload-content"
                initial={{ opacity: 0, y: 20 }}
                animate={{ opacity: 1, y: 0 }}
                exit={{ opacity: 0, y: -20 }}
                transition={{ duration: 0.5, ease: "easeOut" }}
                className="flex flex-col gap-6 max-w-6xl mx-auto"
              >
                {/* 标题 */}
                <div className="text-center space-y-2 pt-4 select-none">
                  <div className="flex items-center justify-center gap-2.5">
                    <Heart size={24} className="text-pink-300/80 dark:text-pink-300/70 fill-pink-300/80 dark:fill-pink-300/70" />
                    <h1 className="text-3xl font-bold text-pink-400/90 dark:text-pink-300/80">
                      Piccy
                    </h1>
                    <Sparkles size={16} className="text-purple-300/80 dark:text-purple-300/70" />
                  </div>
                  <p className="text-pink-300/80 dark:text-pink-300/70 text-sm font-medium">
                    可爱的图片处理工具 ♡
                  </p>
                </div>

                <FileUploadList
                  files={uploadedFiles}
                  onRemove={removeFile}
                  onAddMore={() =>
                    document.getElementById("file-upload")?.click()
                  }
                  showAddMore={uploadedFiles.length < 20}
                />
                <div className="flex items-center justify-end gap-2 px-2">
                  <Sparkles size={14} className="text-pink-300/80 dark:text-pink-300/70" />
                  <p className="text-sm text-pink-400/90 dark:text-pink-300/80 font-medium">
                    已选择 {uploadedFiles.length} / 20 张图片
                  </p>
                </div>
                <ImageMenu image={uploadedFiles} />
              </motion.div>
            )}
          </AnimatePresence>
        </div>
      </div>
    </div>
  );
}
