import { useState, useEffect } from "react";

export const useMultiImageState = <T>(images: File[], initialValue: (index: number) => T) => {
	const [selectedIndex, setSelectedIndex] = useState(0);
	const [values, setValues] = useState<T[]>(() => images.map((_, i) => initialValue(i)));

	// 键盘导航
	useEffect(() => {
		const handleKeyDown = (e: KeyboardEvent) => {
			if (e.key === "ArrowLeft" && selectedIndex > 0) {
				e.preventDefault();
				setSelectedIndex(selectedIndex - 1);
			} else if (e.key === "ArrowRight" && selectedIndex < images.length - 1) {
				e.preventDefault();
				setSelectedIndex(selectedIndex + 1);
			}
		};

		window.addEventListener("keydown", handleKeyDown);
		return () => window.removeEventListener("keydown", handleKeyDown);
	}, [selectedIndex, images.length]);

	const updateValue = (index: number, value: T) => {
		setValues((prev) => {
			const updated = [...prev];
			updated[index] = value;
			return updated;
		});
	};

	const applyToAll = (value: T) => {
		setValues(images.map(() => value));
	};

	return {
		selectedIndex,
		setSelectedIndex,
		currentValue: values[selectedIndex],
		values,
		updateValue,
		applyToAll,
	};
};
