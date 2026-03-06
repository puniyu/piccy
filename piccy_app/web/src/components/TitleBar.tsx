import { Minus, Square, X, ImageIcon } from "lucide-react";
import { getCurrentWindow } from "@tauri-apps/api/window";
import { useEffect, useState } from "react";

export const TitleBar = () => {
  const appWindow = getCurrentWindow();

  const [isMaximized, setIsMaximized] = useState(false);

  useEffect(() => {
    appWindow.isMaximized().then(setIsMaximized);
    const unlisten = appWindow.listen("tauri://resize", async () => {
      setIsMaximized(await appWindow.isMaximized());
    });

    return () => {
      unlisten.then((fn) => fn());
    };
  }, []);

  return (
    <div className="h-11 flex items-center bg-pink-50/98 dark:bg-gray-900/98 backdrop-blur-xl border-b border-pink-100/60 dark:border-gray-800/60 shrink-0">
      <div
        data-tauri-drag-region
        className="flex items-center gap-2 flex-1 h-full px-4"
      >
        <ImageIcon size={16} className="text-pink-400 dark:text-gray-400" />
        <span className="text-sm font-medium text-pink-500 dark:text-gray-300">
          Piccy
        </span>
      </div>

      <div className="flex items-center gap-0.5 px-2">
        <button
          type="button"
          onClick={async () => await appWindow.minimize()}
          className="w-10 h-8 rounded-md flex items-center justify-center text-pink-400 dark:text-gray-400 hover:bg-pink-100/70 dark:hover:bg-gray-800 transition-colors"
        >
          <Minus size={16} strokeWidth={2.5} />
        </button>
        <button
          type="button"
          onClick={async () => await appWindow.toggleMaximize()}
          className="w-10 h-8 rounded-md flex items-center justify-center text-pink-400 dark:text-gray-400 hover:bg-pink-100/70 dark:hover:bg-gray-800 transition-colors"
        >
          {isMaximized ? (
            <div className="w-3 h-3 border-2 border-current" />
          ) : (
            <Square size={13} strokeWidth={2.5} />
          )}
        </button>
        <button
          type="button"
          onClick={async () => await appWindow.close()}
          className="w-10 h-8 rounded-md flex items-center justify-center text-pink-400 dark:text-gray-400 hover:bg-red-500 hover:text-white transition-colors"
        >
          <X size={16} strokeWidth={2.5} />
        </button>
      </div>
    </div>
  );
};
