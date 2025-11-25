import { Button, FileUpload, Heading, Text } from "@chakra-ui/react";
import { FileImage, ImageIcon } from "lucide-react";
import { useCallback, useEffect, useState } from "react";
import { ImageMenu } from "@/components/menu.tsx";
import { FileUploadList } from "@/components/file";
import { ColorModeButton } from "@/components/ui/color-mode";
import { toaster } from "@/components/ui/toaster";
import { motion, AnimatePresence } from "motion/react";


export default function App() {
  useEffect(() => {
    if (!import.meta.env.DEV) {
      document.addEventListener("contextmenu", function (event) {
        event.preventDefault();
      });
    }
  }, []);

  const [uploadedFiles, setUploadedFiles] = useState<File[]>([]);

  const handleFileChange = useCallback(
    (details: { acceptedFiles: File[]; rejectedFiles: { file: File }[] }) => {
      setUploadedFiles(details.acceptedFiles);
      if (details.rejectedFiles.length > 0) {
        toaster.create({
          title: "部分图片未能添加",
          description: `已达到最大数量限制（20张），${details.rejectedFiles.length} 张图片被忽略`,
          type: "warning",
          duration: 4000,
        });
      }
    },
    [],
  );

  return (
    <div className="w-full h-screen flex items-center justify-center bg-gray-50 dark:bg-gray-900 relative overflow-hidden">
      <div className="absolute top-5 right-5 z-10">
        <ColorModeButton />
      </div>

      <motion.div
        layout
        initial={{ opacity: 0, y: 20, scale: 0.95 }}
        animate={{ opacity: 1, y: 0, scale: 1 }}
        transition={{
          layout: { duration: 0.4, ease: "backOut" },
          opacity: { duration: 0.4 },
          y: { duration: 0.4 },
        }}
        className={`!px-8 !py-8 bg-white dark:bg-gray-800 rounded-2xl shadow-xl border border-gray-200 dark:border-gray-700 flex flex-col gap-8 ${
          uploadedFiles.length > 0 ? "w-[95%] max-w-5xl" : "w-[90%] max-w-2xl"
        }`}
      >
        {/* 标题区域 */}
        <div className="text-center space-y-3">
          <div className="flex items-center justify-center gap-3 text-gray-900 dark:text-white">
            <ImageIcon size={36} className="text-blue-500" />
            <Heading size="2xl" fontWeight="bold">
              Piccy
            </Heading>
          </div>
          <Text className="text-gray-500 dark:text-gray-400 text-lg">
            简洁高效的图片处理工具
          </Text>
        </div>

        {/* 文件上传区域 */}
        <FileUpload.Root
          accept="image/*"
          maxFiles={20}
          className="w-full"
          onFileChange={handleFileChange}
        >
          <FileUpload.HiddenInput />

          <AnimatePresence mode="wait">
            {uploadedFiles.length === 0 ? (
              <motion.div
                key="upload-trigger"
                initial={{ opacity: 0 }}
                animate={{ opacity: 1 }}
                exit={{ opacity: 0 }}
                transition={{ duration: 0.2 }}
                className="w-full"
              >
                <FileUpload.Trigger asChild>
                  <Button
                    variant="outline"
                    className="!w-full !h-auto !py-20 !border-2 !border-dashed !border-gray-300 dark:!border-gray-600 hover:!border-blue-400 hover:!bg-blue-50 dark:hover:!bg-blue-950/30 dark:hover:!border-blue-500 transition-all duration-200 group rounded-xl"
                  >
                    <div className="flex flex-col items-center gap-4 group-hover:scale-105 transition-transform duration-200">
                      <div className="p-5 bg-gray-100 dark:bg-gray-700 rounded-full text-gray-500 dark:text-gray-400 group-hover:text-blue-500 group-hover:bg-blue-100 dark:group-hover:bg-blue-900/50 transition-colors">
                        <FileImage size={48} />
                      </div>
                      <div className="space-y-2">
                        <Text
                          fontWeight="medium"
                          className="text-xl text-gray-700 dark:text-gray-200"
                        >
                          点击或拖拽上传图片
                        </Text>
                        <Text
                          fontSize="sm"
                          className="text-gray-400 dark:text-gray-500"
                        >
                          支持 JPG、PNG、GIF 等格式，单次最多上传 20 张图片
                        </Text>
                      </div>
                    </div>
                  </Button>
                </FileUpload.Trigger>
              </motion.div>
            ) : (
              <motion.div
                key="upload-content"
                initial={{ opacity: 0, y: 10 }}
                animate={{ opacity: 1, y: 0 }}
                exit={{ opacity: 0, y: -10 }}
                transition={{ duration: 0.3 }}
                className="flex flex-col gap-6"
              >
                <div className="flex flex-col gap-3">
                  <FileUploadList />
                  <Text className="text-xs text-gray-400 dark:text-gray-500 text-right">
                    已选择 {uploadedFiles.length} / 20 张图片
                  </Text>
                </div>
                <ImageMenu image={uploadedFiles} />
              </motion.div>
            )}
          </AnimatePresence>
        </FileUpload.Root>
      </motion.div>
    </div>
  );
}
