async function youtubeSearch(){
    let req = await fetch("/api/isaacyoutubers", {
        method: "GET",
        headers: {
            "Content-Type": "application/json"
        },
    });
    youtubeSearchRedeemer(req);
}





