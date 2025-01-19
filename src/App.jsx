import { invoke } from "@tauri-apps/api/core";
import { useEffect, useState } from "react";
import "./App.css"

function App() {

    useEffect(() => {
        async function init_backend() {
            try {
                const result = await invoke("init");
                if (result == true) {
                    console.log("Successfully Set the Window Size");
                }
            }
            catch (err) {
                console.log("There is some error in setting the Window size", err);
            }
        }
        init_backend();
    }, []);

  return (
    <div className="select-none flex flex-col items-center justify-center bg-zinc-900 rounded-[1rem] text-white text-xl h-screen w-screen border border-white/20 overflow-hidden">
     💀 ☠️  ⚰️  🔪 💪 🗿🌚 🩸 🕳️ 👻 🕷️ 🕸️ 🌑 🖤 🪓 ⛓️ 🩹 🥀 🔥 
    </div>
  );
}

export default App;
