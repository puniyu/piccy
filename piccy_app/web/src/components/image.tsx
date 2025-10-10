import {CloseButton, Dialog, InputGroup, Portal, Text, Button, VStack, NumberInput} from "@chakra-ui/react";
import {image_crop, ImageInfo} from "@/utils/image.ts";
import {useState} from "react";
import {download_file} from "@/utils/file.ts";

export const ImageInfoCard = ({imageInfo, onClose}: { imageInfo: ImageInfo; onClose: () => void }) => {
    return (
        <Dialog.Root open={true}>
            <Portal>
                <Dialog.Backdrop/>
                <Dialog.Positioner>
                    <Dialog.Content>
                        <Dialog.Header>
                            <Dialog.Title>图片信息</Dialog.Title>
                        </Dialog.Header>
                        <Dialog.Body>
                            <Text>图片长度：{imageInfo.width}</Text>
                            <Text>图片高度：{imageInfo.height}</Text>
                            <Text>是否动图：{imageInfo.is_multi_frame ? "是" : "否"}</Text>
                            {imageInfo.is_multi_frame && (
                                <>
                                    <Text>动图帧数：{imageInfo.frame_count}</Text>
                                    <Text>平均帧间隔：{imageInfo.average_duration}</Text>
                                </>
                            )}
                        </Dialog.Body>
                        <Dialog.Footer>
                        </Dialog.Footer>
                        <Dialog.CloseTrigger asChild>
                            <CloseButton size="sm" onClick={onClose}/>
                        </Dialog.CloseTrigger>
                    </Dialog.Content>
                </Dialog.Positioner>
            </Portal>
        </Dialog.Root>
    )
}



export const ImageCropCard = ({ image_data, onClose }: { image_data: File; onClose: () => void }) => {
    const [left, setLeft] = useState<number>(0);
    const [top, setTop] = useState<number>(0);
    const [width, setWidth] = useState<number>(100);
    const [height, setHeight] = useState<number>(100);


    return (
        <>
        <Dialog.Root open={true}>
            <Portal>
                <Dialog.Backdrop />
                <Dialog.Positioner>
                    <Dialog.Content>
                        <Dialog.Header>
                            <Dialog.Title>图片裁剪</Dialog.Title>
                        </Dialog.Header>
                        <Dialog.Body>
                            <VStack style={{ gap: '1rem' }}>
                                <Text textAlign="left" width="100%" display="block">
                                    左上角x坐标
                                </Text>
                                <InputGroup>
                                    <NumberInput.Root defaultValue="0" min={0} onValueChange={(e) => setLeft(e.valueAsNumber)}>
                                        <NumberInput.Control />
                                        <NumberInput.Input />
                                    </NumberInput.Root>
                                </InputGroup>
                                <Text textAlign="left" width="100%" display="block">
                                    左上角y坐标
                                </Text>
                                <InputGroup>
                                    <NumberInput.Root defaultValue="0" min={0} onValueChange={(e) => setTop(e.valueAsNumber)}>
                                        <NumberInput.Control />
                                        <NumberInput.Input />
                                    </NumberInput.Root>
                                </InputGroup>
                                <Text textAlign="left" width="100%" display="block">
                                    图片的宽度
                                </Text>
                                <InputGroup>
                                    <NumberInput.Root defaultValue="0" min={0} onValueChange={(e) => setWidth(e.valueAsNumber)}>
                                        <NumberInput.Control />
                                        <NumberInput.Input />
                                    </NumberInput.Root>
                                </InputGroup>
                                <Text textAlign="left" width="100%" display="block">
                                    图片的高度
                                </Text>
                                <InputGroup>
                                    <NumberInput.Root defaultValue="0" min={0} onValueChange={(e) => setHeight(e.valueAsNumber)}>
                                        <NumberInput.Control />
                                        <NumberInput.Input />
                                    </NumberInput.Root>
                                </InputGroup>
                                </VStack>
                        </Dialog.Body>
                        <Dialog.Footer>
                            <Button onClick={async () => {
                                const image = await image_crop(image_data, { left, top, width, height });
                                await download_file(image);
                            }}>处理</Button>

                        </Dialog.Footer>
                        <Dialog.CloseTrigger asChild>
                            <CloseButton size="sm" onClick={onClose}/>
                        </Dialog.CloseTrigger>
                    </Dialog.Content>
                </Dialog.Positioner>
            </Portal>
        </Dialog.Root>
        </>
    );
}
