// oxlint-disable arrow-body-style
import { FileUpload, useFileUploadContext } from "@chakra-ui/react";
import { X } from "lucide-react";
import { motion, AnimatePresence } from "motion/react";

export const FileUploadList = () => {
  const fileUpload = useFileUploadContext();
  const files = fileUpload.acceptedFiles;

  if (files.length === 0) return null;
  return (
    <FileUpload.ItemGroup
      className={
        "grid w-full gap-2 md:gap-3 max-h-[50vh] overflow-y-auto p-1 " +
        "[grid-template-columns:repeat(auto-fit,minmax(120px,1fr))] " +
        "sm:[grid-template-columns:repeat(auto-fit,minmax(140px,1fr))] " +
        "md:[grid-template-columns:repeat(auto-fit,minmax(160px,1fr))]"
      }
    >
      <AnimatePresence>
        {files.map((file) => (
          <motion.div
            key={file.name}
            initial={{ opacity: 0, scale: 0.8 }}
            animate={{ opacity: 1, scale: 1 }}
            exit={{ opacity: 0, scale: 0.8 }}
            transition={{ type: "spring", damping: 20, stiffness: 300 }}
            layout
            className="w-full"
          >
            <FileUpload.Item
              file={file}
              className={`w-full ${files.length === 1 ? "h-64 md:h-80" : "aspect-square"} p-0 rounded-xl overflow-hidden border-2 border-gray-200 dark:border-gray-600 relative bg-gray-50 hover:border-blue-400 hover:shadow-md transition-all duration-200 group`}
            >
              <FileUpload.ItemPreviewImage asChild>
                <img
                  src={URL.createObjectURL(file)}
                  alt="preview"
                  className="w-full h-full object-cover block"
                />
              </FileUpload.ItemPreviewImage>

              <div className="absolute top-2 right-2 opacity-0 group-hover:opacity-100 transition-opacity duration-200 z-10">
                <FileUpload.ItemDeleteTrigger asChild>
                  <button className="w-7 h-7 rounded-full bg-red-500 text-white flex items-center justify-center cursor-pointer transition-transform hover:bg-red-600 hover:scale-110 shadow-sm">
                    <X size={16} />
                  </button>
                </FileUpload.ItemDeleteTrigger>
              </div>
            </FileUpload.Item>
          </motion.div>
        ))}
      </AnimatePresence>
    </FileUpload.ItemGroup>
  );
};
