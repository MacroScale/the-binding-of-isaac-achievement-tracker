
$(document).ready(function(){
    // change footer img
    $(".footer-img").attr("src", "/static/news/News_Footer.png");

    
    init();
})

function init(){
    let footer_html = $(".footer").prop("outerHTML");
    console.log(footer_html)

    let footer = $(".footer");
    footer.remove();

    $("#end").append(footer_html);
}
