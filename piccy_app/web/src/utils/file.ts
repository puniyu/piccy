import {invoke} from "@tauri-apps/api/core";

export const download_file = async (
    image_data: ArrayBuffer,
): Promise<void> => {
    const uint8Array = new Uint8Array(image_data);
    return await invoke('download_file', { data: uint8Array});
}