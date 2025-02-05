function error_prompt() {

    useEffect(() => {
        console.log("Error Component Rendered");
    })
    return (
        <div className='bg-[#ed1b76] p-[2px] flex flex-row gap-2 text-md rounded-[10px]'>
            Error ❌
        </div>
    )
}

export default error_prompt;
