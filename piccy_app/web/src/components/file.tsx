import {FileUpload, Float, useFileUploadContext} from "@chakra-ui/react";
import {useEffect, useMemo} from "react";
import {LuX} from "react-icons/lu";

export const FileUploadList = ({onFileUpload, onFileClear}: {
    onFileUpload: (file: File) => void,
    onFileClear: () => void
}) => {
    const fileUpload = useFileUploadContext()
    const files = fileUpload.acceptedFiles

    const fileCount = useMemo(() => files.length, [files]);

    useEffect(() => {
        if (fileCount > 0) {
            files.forEach(file => {
                onFileUpload(file);
            });
        } else {
            onFileClear();
        }
    }, [fileCount, onFileUpload, onFileClear]);


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
                    <FileUpload.ItemPreviewImage/>
                    <Float placement="top-end">
                        <FileUpload.ItemDeleteTrigger boxSize="4" layerStyle="fill.solid">
                            <LuX/>
                        </FileUpload.ItemDeleteTrigger>
                    </Float>
                </FileUpload.Item>
            ))}
        </FileUpload.ItemGroup>
    )
}