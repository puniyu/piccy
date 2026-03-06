import { useImageStore } from "@/store/image";
import { useToaster } from "@/components/ui/toaster";

export const useFileUpload = () => {
	const toaster = useToaster();
	const { uploadedFiles, isDragging, maxFiles, addFiles, removeFile, setDragging } = useImageStore();

	const handleFiles = (files: File[]) => {
		const { success, rejectedCount } = addFiles(files);

		if (!success) {
			toaster.create({
				title: "已达到最大数量",
				description: `最多只能上传 ${maxFiles} 张图片`,
				type: "warning",
				duration: 4000,
			});
			return;
		}

		if (rejectedCount > 0) {
			toaster.create({
				title: "部分图片未能添加",
				description: `已达到最大数量限制（${maxFiles}张），${rejectedCount} 张图片被忽略`,
				type: "warning",
				duration: 4000,
			});
		}
	};

	// 返回可以直接展开到元素上的 props
	const dropZoneProps = {
		onDrop: (e: React.DragEvent) => {
			e.preventDefault();
			setDragging(false);
			const files = Array.from(e.dataTransfer.files).filter((f) => f.type.startsWith("image/"));
			if (files.length > 0) handleFiles(files);
		},
		onDragOver: (e: React.DragEvent) => {
			e.preventDefault();
			setDragging(true);
		},
		onDragLeave: (e: React.DragEvent) => {
			e.preventDefault();
			setDragging(false);
		},
	};

	const fileInputProps = {
		onChange: (e: React.ChangeEvent<HTMLInputElement>) => {
			const files = Array.from(e.target.files || []);
			if (files.length > 0) handleFiles(files);
		},
	};

	return {
		uploadedFiles,
		isDragging,
		removeFile,
		dropZoneProps,
		fileInputProps,
	};
};
