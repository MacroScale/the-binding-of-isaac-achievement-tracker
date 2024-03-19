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



function updateCompletionMarks(data){
    $("#heart").attr("src", data.heart_url)
    $("#isaac").attr("src", data.isaac_url)
    $("#polaroid").attr("src", data.polaroid_url)
    $("#cent").attr("src", data.cent_url)
    $("#satan").attr("src", data.satan_url)
    $("#negative").attr("src", data.negative_url)
    $("#brimstone").attr("src", data.brimstone_url)
    $("#star").attr("src", data.star_url)
    $("#hush").attr("src", data.hush_url)
    $("#knife").attr("src", data.knife_url)
    $("#dadsnote").attr("src", data.dadsnote_url)
}
