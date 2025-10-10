import {Button, Menu, Portal} from "@chakra-ui/react"
import {useState} from "react";
import {image_info, ImageInfo} from "@/utils/image.ts";
import {ImageCropCard, ImageInfoCard} from "@/components/image.tsx";

export const ImageMenu = ({image}: {image: Array<File>}) => {
    const [imageInfo, setImageInfo] = useState<ImageInfo | null>(null);
    const [showImageInfo, setShowImageInfo] = useState(false);
    const [showImageCrop, setShowImageCrop] = useState(false);

    return (
        <>
            <Menu.Root>
                <Menu.Trigger asChild>
                    <Button variant="outline" size="sm">
                        操作
                    </Button>
                </Menu.Trigger>
                <Portal>
                    <Menu.Positioner>
                        <Menu.Content>
                            <Menu.Item value="image-info" onSelect={async () => {
                                const info = await image_info(image[0]);
                                setImageInfo(info)
                                setShowImageInfo(true);
                            }}>图片信息</Menu.Item>

                            <Menu.Item value="image-crop" onSelect={async () => {
                                setShowImageCrop(true);
                            }}>图片裁剪</Menu.Item>

                        </Menu.Content>
                    </Menu.Positioner>
                </Portal>
            </Menu.Root>

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
    )
}