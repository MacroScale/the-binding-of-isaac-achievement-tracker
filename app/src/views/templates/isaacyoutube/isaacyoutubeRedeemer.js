async function youtubeSearchRedeemer(res){
    let data = await res.json();
    console.log(data);
    setYoutuberData(data);
}

function setYoutuberData(data){
    $("#slayxc-slot").data("url", data.slayxc.latest_video_url);
    $("#slayxc-slot").find(".thumbnail").attr("src", data.slayxc.latest_video_thumbnail);

    $("#isaacguru-slot").data("url", data.isaacguru.latest_video_url);
    $("#isaacguru-slot").find(".thumbnail").attr("src", data.isaacguru.latest_video_thumbnail);

    $("#mattman-slot").data("url", data.mattman.latest_video_url);
    $("#mattman-slot").find(".thumbnail").attr("src", data.mattman.latest_video_thumbnail);

    $("#northern-slot").data("url", data.northernlion.latest_video_url);
    $("#northern-slot").find(".thumbnail").attr("src", data.northernlion.latest_video_thumbnail);

    $("#hutts-slot").data("url", data.hutts.latest_video_url);
    $("#hutts-slot").find(".thumbnail").attr("src", data.hutts.latest_video_thumbnail);

    $("#nyantuber-slot").data("url", data.nyantuber.latest_video_url);
    $("#nyantuber-slot").find(".thumbnail").attr("src", data.nyantuber.latest_video_thumbnail);
}
