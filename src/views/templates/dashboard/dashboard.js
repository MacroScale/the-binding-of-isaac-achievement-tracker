var completion_mark_ids = {
    "heart": "#heart",
    "isaac": "#isaac",
    "polaroid": "#polaroid",
    "cent": "#cent",
    "satan": "#satan",
    "negative": "#negative",
    "brimstone": "#brimstone",
    "star": "#star",
    "hush": "#hush",
    "knife": "#knife",
    "dadsnote": "#dadsnote",
};

$(document).ready(function(){

    $("body").on("click", "#search-profile", async function(){
        let req = await fetch("/api/profile-search", {
            method: "POST",
            headers: {
                "Content-Type": "application/json"
            },
            body: JSON.stringify({
                "steam_id": $("#profile-search-input").val()
            })
        })
        let res = await req.json()
        if (req.status === 200) {
            location.reload();
        }
        else{
            alert(res.message)
        }
    })

    $("body").on("click", "#next-char", async function(){
        let req = await fetch("/api/next-char", {
            method: "GET",
            headers: {
                "Content-Type": "application/json"
            },
        })
        let res = await req.json()
        if (req.status === 200) {
            console.log(res)
        }
        else{
            alert(res.message)
        }
    })

    $("body").on("click", "#prev-char", async function(){
        let req = await fetch("/api/prev-char", {
            method: "GET",
            headers: {
                "Content-Type": "application/json"
            },
        })
        let res = await req.json()
        if (req.status === 200) {
            console.log(res)
        }
        else{
            alert(res.message)
        }
    })


    $("body").on("click", "#next-achievement", async function(){
        let req = await fetch("/api/next-achievement", {
            method: "GET",
            headers: {
                "Content-Type": "application/json"
            },
        })
        let res = await req.json()
        if (req.status === 200) {
            console.log(res)
        }
        else{
            alert(res.message)
        }
    })

    $("body").on("click", "#prev-achievement", async function(){
        let req = await fetch("/api/prev-achievement", {
            method: "GET",
            headers: {
                "Content-Type": "application/json"
            },
        })
        let res = await req.json()
        if (req.status === 200) {
            console.log(res)
        }
        else{
            alert(res.message)
        }
    })


})

async function getCookieData(){
    let req = await fetch("/api/get-cookie-data", {
        method: "GET",
        headers: {
            "Content-Type": "application/json"
        },
    })
    let res = await req.json()
    console.log(res);
}
