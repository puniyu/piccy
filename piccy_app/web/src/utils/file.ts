import { invoke } from "@tauri-apps/api/core";
import { toaster } from "@/components/ui/toaster";

export const download_file = async (image_data: ArrayBuffer): Promise<void> => {
  const uint8Array = new Uint8Array(image_data);
  try {
    const path = await invoke<string>("download_file", { data: uint8Array });
    toaster.create({
      title: "保存成功",
      description: `文件已保存至 ${path}`,
      type: "success",
      duration: 4000,
      closable: true,
    });
  } catch (error) {
    toaster.create({
      title: "保存失败",
      description: String(error),
      type: "error",
      duration: 4000,
      closable: true,
    });
    throw error;
  }
};