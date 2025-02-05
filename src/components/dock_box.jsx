import { invoke } from "@tauri-apps/api/core";
import { useEffect, useState } from "react";

import { emit, listen } from "@tauri-apps/api/event";
import { Window, getAllWindows, getCurrentWindow } from "@tauri-apps/api/window";

function DockBox() {
    const [quickMenu, setQuickMenu] = useState(false);
    const [isHovered, setIsHovered] = useState(false);

    useEffect(() => {
        let unlisten;
        async function setupDragDropListener() {
            unlisten = await getCurrentWindow().onDragDropEvent(event => {
                if (event.payload.type === "over") {
                    // console.log(event.payload.paths);
                    setIsHovered(true);
                }
                if (event.payload.type === "drop" || event.payload.type === "leave") {
                    if (event.payload.type === "drop") {
                        // send data to backend
                        // emit an event from the frontend
                        emit('quick_menu_data', event.payload.paths);
                    }

                    // console.log(event.payload);
                    setIsHovered(false);
                }
            });
        }
        setupDragDropListener();
        return () => {
            if (unlisten) unlisten();
        };
    }, []);

    const toggleQuickMenu = () => {
        if (quickMenu == true) {
            emit("toggle_quick_menu", "hide")
            setQuickMenu(false)
        } else {
            emit("toggle_quick_menu", "show")
            setQuickMenu(true)
        }
    }

    return (
        <div
            className={`absolute bg-[#ed1b76] pl-2 pr-2 text-black flex justify-center items-center
                transition-all duration-100 ease-in-out
                ${isHovered 
                    ? "w-full h-full" 
                    : "w-0 h-0"}
                `}
        >
        {isHovered ?  
            <p>Drop Here</p> :
            <button onClick={toggleQuickMenu} className="fixed left-1 text-white p-2 flex items-center justify-center">
                {quickMenu == true ? "❌" : "➕"}
            </button>
        }

        </div>
    );
}

export default DockBox;
