$(document).ready(function(){
    function init(){
        $(".footer-img").attr("src", "/static/news/News_Footer.png");
    }
    
    init();

    animate();
});




function animate(){
    let count = 0;

    setInterval(() => {
        let count_str = count.toString();
        if (count_str.length < 2){
            count_str = "0" + count_str;
        }

        $(".top").attr("src", `/static/devlog/TOP_BG/TOP_BG_00${count_str}.png`);
        $(".mid").attr("src", `/static/devlog/MID_BG/MID_BG_00${count_str}.png`);
        $(".bot").attr("src", `/static/devlog/BOT_BG/BOT_BG_00${count_str}.png`);

        if (count == 11) count = 0;
        count++;
    }, 100)

}

