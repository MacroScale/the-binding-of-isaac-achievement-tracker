$(document).ready(function(){
    // change footer img
    $(".footer-img").attr("src", "/static/news/News_Footer.png");


    $("body").on("click", ".tv", function(){
        console.log("thumbnail clicked");
        let url = $(this).data("url");
        console.log(url);
        window.open(url, "_blank");
    });
    
    youtubeSearch();
    replaceFooter();
})

function replaceFooter(){
    let footer_html = $(".footer").prop("outerHTML");
    console.log(footer_html)
    let footer = $(".footer");
    footer.remove();
    $("#end").append(footer_html);
}
