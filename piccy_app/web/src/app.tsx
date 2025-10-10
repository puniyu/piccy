import {
	Box,
	Button, CloseButton,
	Dialog,
	FileUpload,
	Float,
	Heading,
	Portal,
	Text,
	useFileUploadContext,
} from "@chakra-ui/react"
import {LuFileImage, LuX} from "react-icons/lu"
import {invoke} from '@tauri-apps/api/core';
import {useEffect, useState} from "react";

const image_info = async (image_data: number[]): Promise<ImageInfo> => {
	return await invoke('image_info', { imageData: image_data });
};
export default function App() {

	useEffect(() => {
		if (!import.meta.env.DEV) {
			document.addEventListener('contextmenu', function(event) {
				event.preventDefault();
			});
		}
	}, []);

	const [uploadedFile, setUploadedFile] = useState<{buffer: ArrayBuffer} | null>(null)
	const [imageInfo, setImageInfo] = useState<ImageInfo | null>(null);
	const [showImageInfo, setShowImageInfo] = useState(false);



	return (
		<Box className="w-full h-screen flex "
		     alignItems="center"
		     justifyContent="center">
			<Box className="!p-7"
			     width={{ base: "90%", md: "400px" }}
			     borderWidth={1}
			     boxShadow="2xl"
			     backdropFilter="blur(10px)"
			     borderRadius="lg">
				<Heading className="text-center">piccy - 图片小工具</Heading>

				<FileUpload.Root accept="image/*" maxFiles={1} className="!pt-5 ">
					<FileUpload.HiddenInput />
					<FileUpload.Trigger asChild>
						<Button variant="outline" size="sm" mx="auto">
							<LuFileImage /> 上传图片
						</Button>
					</FileUpload.Trigger>
					<FileUploadList onFileUpload={async (file) => {
						const arrayBuffer = await file.arrayBuffer()
						setUploadedFile({ buffer: arrayBuffer })
					}} onFileClear={()=> {
						setUploadedFile(null)
					}} />



					<Button className={"!bg-gradient-to-r from-pink-300 to-blue-300 mt-10"}
					        mx="auto"
					        display="block"
					        border={"none"}
					        mt={5}
					        onClick={async () => {
						if (uploadedFile) {

							const image_buffer = Array.from(new Uint8Array(uploadedFile.buffer))
							const info = await image_info(image_buffer)
							setImageInfo(info);
							setShowImageInfo(true)
						}

					}}>
						<Text>查看图片信息</Text>
					</Button>


					{showImageInfo && imageInfo && (
						<ImageInfoCard
							imageInfo={imageInfo}
							onClose={() => setShowImageInfo(false)}
						/>
					)}




				</FileUpload.Root>



			</Box>
		</Box>
	)
}

const FileUploadList = ({ onFileUpload, onFileClear }: {
	onFileUpload: (file: File) => void,
	onFileClear: () => void
}) => {
	const fileUpload = useFileUploadContext()
	const files = fileUpload.acceptedFiles

	useEffect(() => {
		if (files.length > 0) {
			onFileUpload(files[0])
		} else {
			onFileClear()
		}
	}, [files, onFileUpload, onFileClear])


	if (files.length === 0) return null
	return (
		<FileUpload.ItemGroup>
			{files.map((file) => (
				<FileUpload.Item
					w="auto"
					boxSize="30"
					p="2"
					file={file}
					key={file.name}
				>
					<FileUpload.ItemPreviewImage />
					<Float placement="top-end">
						<FileUpload.ItemDeleteTrigger boxSize="4" layerStyle="fill.solid">
							<LuX />
						</FileUpload.ItemDeleteTrigger>
					</Float>
				</FileUpload.Item>
			))}
		</FileUpload.ItemGroup>
	)
}


interface ImageInfo {
	width: number;
	height: number;
	is_multi_frame: boolean;
	frame_count: number | null;
	average_duration: number | null;
}



const ImageInfoCard = ({ imageInfo, onClose }: { imageInfo: ImageInfo; onClose: () => void }) => {
	return (
		<Dialog.Root open={true}>
			<Portal>
				<Dialog.Backdrop />
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
							<CloseButton size="sm" onClick={onClose} />
						</Dialog.CloseTrigger>
					</Dialog.Content>
				</Dialog.Positioner>
			</Portal>
		</Dialog.Root>
	)
}
