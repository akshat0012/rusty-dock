import { listen } from "@tauri-apps/api/event";
import { useEffect } from 'react'

import { useSelector, useDispatch } from 'react-redux'
import { insert_data} from '../slices/quick_window_data_slice'
import { set_color, set_radius } from '../slices/dock_settings_slice'


function QuickWindow() {
    let settings = useSelector(state => state.dock_settings);
    let data = useSelector(state => state.quick_window_data.data);
    const dispatch = useDispatch();


    useEffect(() => {
        async function listen_for_events() {
            await listen("send_config_data_to_quick_window", (event) => {
                dispatch(set_color(event.payload.dock_settings.bg_color))
                dispatch(set_radius(event.payload.dock_settings.border_radius))
            })
            await listen("send_config_data", (event) => {
                dispatch(set_color(event.payload.dock_settings.bg_color))
                dispatch(set_radius(event.payload.dock_settings.border_radius))
            })
            await listen("quick_menu_data", (event) => {
                try {
                    console.log("Raw payload:", event.payload);
                        dispatch(insert_data(event.payload));
                } catch (error) {
                    console.error("Error parsing payload:", error);
                }
            });
        }
        listen_for_events();
    }, [dispatch] );


    return (
        <div 
            style={{ 
                backgroundColor: settings.bg_color,
                borderRadius: `${settings.border_radius}px`,
            }}
            className={`relative select-none flex items-start justify-center pt-3 pb-3 rounded-[16px] text-white text-xl h-screen w-screen border border-white/20 overflow-auto no-scrollbar`}
        >

        {data.length > 0 ? <div className="grid grid-cols-5 gap-4">
                {data.map((item, index) => (
                    <div key={index} className="bg-[#ed1b76] p-4 rounded-lg shadow-md text-center overflow-hidden">
                        📄
                    </div>
                ))}
            </div>
            : "Add Some file here for Quick Access 🗃"
        }

        </div>
    )
}

export default QuickWindow;
