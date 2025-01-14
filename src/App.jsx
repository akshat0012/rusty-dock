import { invoke } from "@tauri-apps/api/core";
import { useEffect, useState } from "react";
import "./App.css"

function App() {

    useEffect(() => {
        async function fetchAndSetWindowSize() {
            try {
                const size = await invoke("get_primary_monitor_info");
                console.log(size);
                if (size) {
                    try {
                        const result = await invoke("set_window_size", { width: size[0], height: 10 });
                        if (result == true) {
                            console.log("Successfully Set the Window Size");
                        }
                    }
                    catch (err) {
                        console.log("There is some error in setting the Window size", err);
                    }
                }
            } catch (error) {
                console.error("Error:", error);
            }
        }
    fetchAndSetWindowSize();
    }, []);

  return (
    <div className="select-none flex flex-col items-center justify-center bg-zinc-900 rounded-[1rem] text-white text-xs h-screen w-screen border border-white/20">
      Rusty Bar 💀 
    </div>
  );
}

export default App;
