$(document).ready(function(){

    $("body").on("click", ".nav-section", function(e){
        let el = $(e.currentTarget);
        let url = el.data("url"); 
        if (url) window.location.href = url;
    })


    $("body").on("mouseenter", ".nav-section", function(e){
        $(e.currentTarget).find(".nav-underline").show();
    })

    $("body").on("mouseleave", ".nav-section", function(e){
        $(e.currentTarget).find(".nav-underline").hide();
    })
});
