import { useSelector, useDispatch } from 'react-redux'

function QuickWindow() {
    let settings = useSelector(state => state.dock_settings);
    return (
        <div 
            style={{ 
                backgroundColor: settings.bg_color,
                borderRadius: `${settings.border_radius}px`,
            }}
            className={`relative select-none flex items-center justify-center rounded-[16px] text-white text-xl h-screen w-screen border border-white/20 overflow-hidden`}>
            Quick Menu here
        </div>
    )
}

export default QuickWindow;
