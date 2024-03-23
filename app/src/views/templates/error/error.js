$(document).ready(function(){
    $(".footer-img").attr("src", "/static/news/News_Footer.png");
    animate();
});

function animate(){
    let count = 0;

    setInterval(() => {
        let count_str = count.toString();
        if (count_str.length < 2){
            count_str = "0" + count_str;
        }

        $(".bg").attr("src", `/static/error/BG/BG_00${count_str}.png`);

        if (count == 11) count = 0;
        count++;
    }, 100)

}

