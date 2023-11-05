export const upload_files = async (file: File): Promise<{url: string, id: string}> => {
    console.log(file)
    return new Promise((resolve, reject) => {
        const reader = new FileReader()
        reader.readAsDataURL(file)
        reader.onload = async () => {
        const b64str = reader.result.replace("data:", "").replace(/^.+,/, "")
        const res = await fetch(`/api/v1/uploads/`, {
            method: "POST",
            headers: {
                "Content-Type": "application/json"
            },
            body: JSON.stringify({
                file_name: file.name,
                mime_type: file.type,
                data: b64str
            })
        })
        if (res.ok) {
            const json = await res.json()
            console.log(json)
            resolve({url: `${window.location.origin}/api/v1/uploads/${json.id}`, id: json.id})
        }
    }
    })


}