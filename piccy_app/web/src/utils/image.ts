import {invoke} from '@tauri-apps/api/core'

export interface ImageInfo {
    /// 图像宽度
    width: number
    /// 图像高度
    height: number
    // 是否为动图
    is_multi_frame: boolean
    // 动图帧数
    frame_count: number | null
    // 动图帧间隔
    average_duration: number | null
}

export const image_info = async (
    image_data: File,
): Promise<ImageInfo> => {
    const arrayBuffer = await image_data.arrayBuffer();
    const uint8Array = new Uint8Array(arrayBuffer);

    return await invoke('image_info', {imageData: uint8Array})
}

export const image_crop = async (
    image_data: File,
    crop_rect: { left: number, top: number, width: number, height: number }
): Promise<ArrayBuffer> => {
    const arrayBuffer = await image_data.arrayBuffer();
    const uint8Array = new Uint8Array(arrayBuffer);

    return await invoke<ArrayBuffer>('image_crop', {
        imageData: uint8Array,
        left: crop_rect.left,
        top: crop_rect.top,
        width: crop_rect.width,
        height: crop_rect.height
    });

}
