BOOL SystemParametersInfoA(
  [in]      UINT  uiAction,
  [in]      UINT  uiParam,
  [in, out] PVOID pvParam,
  [in]      UINT  fWinIni
);


// Rust Code
        
    // Create a Structure
    struct pvParam {
        Left: ,
        Right: ,
        Top: ,
        Bottom: ,
    }

    unsafe {
        SystemParametersInfoA(SPI_SETWORKAREA, 0, &work_area as *const _ as *mut _, 0);
                              // uiAction
    }
