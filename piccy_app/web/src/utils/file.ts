import { invoke } from "@tauri-apps/api/core";

interface ToasterType {
  create: (options: {
    title: string;
    description?: string;
    type: "success" | "error" | "warning" | "info";
    duration: number;
  }) => void;
}

export const download_file = async (
  image_data: ArrayBuffer,
  toaster: ToasterType,
): Promise<void> => {
  const uint8Array = new Uint8Array(image_data);
  try {
    const path = await invoke<string>("download_file", { data: uint8Array });
    toaster.create({
      title: "保存成功",
      description: `文件已保存至 ${path}`,
      type: "success",
      duration: 4000,
    });
  } catch (error) {
    toaster.create({
      title: "保存失败",
      description: String(error),
      type: "error",
      duration: 4000,
    });
    throw error;
  }
};