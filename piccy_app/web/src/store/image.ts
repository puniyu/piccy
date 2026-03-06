import { create } from "zustand";

interface ImageStore {
	uploadedFiles: File[];
	isDragging: boolean;
	maxFiles: number;

	addFiles: (files: File[]) => { success: boolean; rejectedCount: number };
	removeFile: (index: number) => void;
	clearFiles: () => void;
	setDragging: (isDragging: boolean) => void;
}

export const useImageStore = create<ImageStore>((set, get) => ({
	uploadedFiles: [],
	isDragging: false,
	maxFiles: 20,

	addFiles: (files) => {
		const { uploadedFiles, maxFiles } = get();
		const availableSlots = maxFiles - uploadedFiles.length;

		if (availableSlots <= 0) {
			return { success: false, rejectedCount: files.length };
		}

		const acceptedFiles = files.slice(0, availableSlots);
		const rejectedCount = files.length - acceptedFiles.length;

		set({ uploadedFiles: [...uploadedFiles, ...acceptedFiles] });

		return { success: true, rejectedCount };
	},

	removeFile: (index) =>
		set((state) => ({
			uploadedFiles: state.uploadedFiles.filter((_, i) => i !== index),
		})),

	clearFiles: () => set({ uploadedFiles: [] }),

	setDragging: (isDragging) => set({ isDragging }),
}));
