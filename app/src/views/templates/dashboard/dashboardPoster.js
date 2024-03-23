async function postProfileSearch(steam_id){
    let req = await fetch("/api/profile-search", {
        method: "POST",
        headers: {
            "Content-Type": "application/json"
        },
        body: JSON.stringify({
            "steam_id": steam_id 
        })
    });
    ProfileSearchRedeemer(req);
}





