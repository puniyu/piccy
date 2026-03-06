import { Button } from "@heroui/react";
import { Info, Crop, RotateCw, Maximize2, FlipHorizontal, Palette, Contrast } from "lucide-react";
import { useState } from "react";
import {
	ImageInfoListModal,
	ImageCropModal,
	ImageRotateModal,
	ImageResizeModal,
	ImageFlipModal,
	ImageFilterModal,
} from "@/components/image";

type ModalType = "info" | "crop" | "rotate" | "resize" | "flip" | "grayscale" | "invert" | null;

export const ImageMenu = ({ image }: { image: File[] }) => {
	const [activeModal, setActiveModal] = useState<ModalType>(null);

	const menuItems = [
		{ type: "info" as const, label: "查看信息", icon: Info, color: "bg-pink-300/70" },
		{ type: "crop" as const, label: "裁剪", icon: Crop, color: "bg-pink-300/80" },
		{ type: "rotate" as const, label: "旋转", icon: RotateCw, color: "bg-pink-300/70" },
		{ type: "resize" as const, label: "缩放", icon: Maximize2, color: "bg-pink-300/70" },
		{ type: "flip" as const, label: "翻转", icon: FlipHorizontal, color: "bg-pink-300/70" },
		{ type: "grayscale" as const, label: "灰度化", icon: Palette, color: "bg-pink-300/80" },
		{ type: "invert" as const, label: "反色", icon: Contrast, color: "bg-pink-300/70" },
	];

	return (
		<>
			<div className="grid grid-cols-[repeat(auto-fit,minmax(240px,1fr))] gap-3">
				{menuItems.map((item) => {
					const Icon = item.icon;
					return (
						<Button
							key={item.type}
							onPress={() => setActiveModal(item.type)}
							className="h-auto py-7 px-7 bg-white/60 dark:bg-pink-950/30 backdrop-blur-md border-2 border-pink-200/50 dark:border-pink-800/40 hover:border-pink-300/80 dark:hover:border-pink-700/60 hover:bg-white/80 dark:hover:bg-pink-950/40 hover:scale-[1.02] transition-all duration-200 shadow-sm hover:shadow-lg rounded-2xl"
						>
							<div className="flex flex-row items-center gap-5 w-full">
								<div className={`p-5 rounded-xl ${item.color} shadow-md shrink-0`}>
									<Icon size={32} className="text-white" strokeWidth={2} />
								</div>
								<span className="text-lg font-semibold text-pink-400/90 dark:text-pink-300/90">
									{item.label}
								</span>
							</div>
						</Button>
					);
				})}
			</div>

			{activeModal === "info" && (
				<ImageInfoListModal images={image} onClose={() => setActiveModal(null)} />
			)}
			{activeModal === "crop" && (
				<ImageCropModal images={image} onClose={() => setActiveModal(null)} />
			)}
			{activeModal === "rotate" && (
				<ImageRotateModal images={image} onClose={() => setActiveModal(null)} />
			)}
			{activeModal === "resize" && (
				<ImageResizeModal images={image} onClose={() => setActiveModal(null)} />
			)}
			{activeModal === "flip" && (
				<ImageFlipModal images={image} onClose={() => setActiveModal(null)} />
			)}
			{(activeModal === "grayscale" || activeModal === "invert") && (
				<ImageFilterModal
					images={image}
					filterType={activeModal}
					onClose={() => setActiveModal(null)}
				/>
			)}
		</>
	);
};
