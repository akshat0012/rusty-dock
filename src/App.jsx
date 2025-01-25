import { invoke } from "@tauri-apps/api/core";
import { emit, listen } from "@tauri-apps/api/event";
import { useEffect, useState } from "react";
import "./App.css"

import { useSelector, useDispatch } from 'react-redux'
import { set_color, set_radius } from './slices/dock_settings'

function App() {
    const dispatch = useDispatch();
    let [manojis, setManojis] = useState(["💀", "☠️", "⚰️", "🔪", "💪", "🗿", "🌚", "🩸", "🕳️", "👻", "🕷️", "🕸️", "🌑", "🖤", "🪓", "⛓️", "🩹", "🥀", "🔥"]);
    
    let settings = useSelector(state => state.dock_settings);

    useEffect(() => {
        let unlisten;
        
        async function init_backend() {
            try {
                const init_result = await invoke("init");
                if (init_result == true) {
                    unlisten = await listen("send_config_data", (event) => {
                        console.log(event.payload);
                        dispatch(set_color(event.payload.dock_settings.bg_color))
                        dispatch(set_radius(event.payload.dock_settings.border_radius))
                    })
                    // Emit for first update
                    emit('frontend_ready', "frontend_ready");
                }
            }
            catch (err) {
                console.log("ERROR:: init() \n", err);
            }
        }
        init_backend();
        
    }, []);

  return (
    <div style={{ 
        backgroundColor: settings.bg_color,
        borderRadius: `${settings.border_radius}px`,
    }}
      className={`select-none flex items-center justify-center rounded-[16px] text-white text-xl h-screen w-screen border border-white/20 overflow-hidden`}>
        <div style={{
            backgroundColor: "#ed1b76",
            borderRadius: `${settings.border_radius}px`,
        }}
          className={`p-[2px] flex flex-row gap-2 text-md`}>
            {manojis.map((emoji, index) => (
                <div key={index}>
                    {emoji}
                </div>
            ))}
        </div>
    </div>
  );
}

export default App;
