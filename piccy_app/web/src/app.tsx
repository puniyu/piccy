import {Box, Button, FileUpload, Heading,} from "@chakra-ui/react"
import {LuFileImage} from "react-icons/lu"
import {useCallback, useEffect, useState} from "react";
import {ImageMenu} from "@/components/menu.tsx";
import {FileUploadList} from "@/components/file";


export default function App() {

    useEffect(() => {
        if (!import.meta.env.DEV) {
            document.addEventListener('contextmenu', function (event) {
                event.preventDefault();
            });
        }
    }, []);

    const [uploadedFile, setUploadedFile] = useState<File[] | null>(null);


    const onFileUpload = useCallback(async (file: File) => {
        setUploadedFile([file]);
    }, []);

    const onFileClear = useCallback(() => {
        setUploadedFile(null);
    }, []);

    return (
        <Box className="!w-full !h-screen flex"
             alignItems="center"
             justifyContent="center">
            <Box className="!p-7"
                 width={{base: "90%", md: "400px"}}
                 borderWidth={1}
                 boxShadow="2xl"
                 backdropFilter="blur(10px)"
                 borderRadius="lg">
                <Heading className="text-center">piccy - 图片小工具</Heading>

                <FileUpload.Root accept="image/*" maxFiles={5} className="!pt-5 ">
                    <FileUpload.HiddenInput/>
                    <FileUpload.Trigger asChild>
                        <Button variant="outline" size="sm" mx="auto">
                            <LuFileImage/> 上传图片
                        </Button>
                    </FileUpload.Trigger>
                    <FileUploadList onFileUpload={onFileUpload} onFileClear={onFileClear}/>


                    {uploadedFile && (
                        <ImageMenu image={uploadedFile}>
                        </ImageMenu>
                    )}

                </FileUpload.Root>

            </Box>
        </Box>
    )
}





