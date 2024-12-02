$(document).ready( async function(){

    $("body").on("click", "#search-profile", async function(){
        //window.location.href = "/dashboard?steam_id=" + $("#profile-search-input").val()
        postProfileSearch($("#profile-search-input").val());
        window.history.pushState({}, "Dashboard", "/dashboard?steam_id=" + $("#profile-search-input").val());
    })

    $("body").on("keyup", "#modal-search-profile", async function(e){
        if (e.key === "Enter"){
            postProfileSearch($("#modal-search-profile").val());
            window.history.pushState({}, "Dashboard", "/dashboard?steam_id=" + $("#modal-search-profile").val());
        }
    })

    $("body").on("click", "#next-char", function(){

        if (_app_state.is_tainted) {

            if (_app_state.current_character_id === 33) _app_state.current_character_id = 17;
            else _app_state.current_character_id++;

            updateCharacterOverlay();
            updateCompletionMarks();

            return;
        }

        if (_app_state.current_character_id === 16) _app_state.current_character_id = 0;
        else _app_state.current_character_id++;

        updateCharacterOverlay();
        updateCompletionMarks();
    })

    $("body").on("click", "#prev-char", function(){

        if (_app_state.is_tainted) {

            if (_app_state.current_character_id === 17) _app_state.current_character_id = 33;
            else _app_state.current_character_id--;

            updateCharacterOverlay();
            updateCompletionMarks();

            return;
        }

        if (_app_state.current_character_id === 0) _app_state.current_character_id = 16;
        else _app_state.current_character_id--;

        updateCharacterOverlay();
        updateCompletionMarks();
    });


    $("body").on("click", "#switch", function(){
        _app_state.is_tainted = !_app_state.is_tainted;

        if (_app_state.is_tainted) {
            _app_state.current_character_id += 17;
            $("#who-am-i-bg").attr("src", "/static/character wheel/character wheel/t_who am i.png")
            $("#wrinkled-paper").attr("src", "/static/completion marks/completion marks/alt_post_it.png")
        }
        else {
            _app_state.current_character_id -= 17;
            $("#who-am-i-bg").attr("src", "/static/character wheel/character wheel/who am i.png")
            $("#wrinkled-paper").attr("src", "/static/completion marks/completion marks/post_it.png")
        }

        updateCharacterOverlay();
        updateCompletionMarks();
    })


    $("body").on("click", "#prev-achievement", function(){
        if (_app_state.current_achievement_id === 0) _app_state.current_achievement_id = _steam_data.achievements.length - 1;
        else _app_state.current_achievement_id--;
        updateAchievements();
    })

    $("body").on("click", "#next-achievement", function(){
        if (_app_state.current_achievement_id === _steam_data.achievements.length - 1) _app_state.current_achievement_id = 0;
        else _app_state.current_achievement_id++;
        updateAchievements();
    });

    $("body").on("change", "#achievement-page-search-input", function(){
        try{
            let val = parseInt($(this).val());
            if (val < 1) val = 1;
            if (val > 640) val = 640;

            _app_state.current_achievement_id = val - 1;
            updateAchievements();
        }
        catch(e){
            console.log(e);
        }
    });


    $("body").on("keyup", "#achievement-search-input", function(){
       let query = $(this).val().toLowerCase();

       $.each($("#achievements-listbox-table tr"), function(index, row){
           let text = $(row).text().toLowerCase();
           if (text.includes(query)) $(row).removeClass("hidden-query"); 
           else $(row).addClass("hidden-query");
       });
    });

    $("body").on("click", "#toggle-all", function(){
        $(this).hide();
        $("#toggle-missing").show();

        $.each($("#achievements-listbox-table tr"), function(index, row){
            $(row).removeClass("hidden-toggle");

            let achieved = $(row).data("achieved");
            if (achieved) $(row).addClass("hidden-toggle");
        });
    });

    $("body").on("click", "#toggle-missing", function(){
        $(this).hide();
        $("#toggle-all").show();

        $.each($("#achievements-listbox-table tr"), function(index, row){
            $(row).removeClass("hidden-toggle");
        });

    });

    $("body").on("dblclick", "#achievements-listbox-table tr", function(){
        let new_id = parseInt($(this).data("id")) - 1;
        if (_app_state.current_achievement_id === new_id) return;
        _app_state.current_achievement_id = parseInt($(this).data("id")) - 1;
        updateAchievements();
    });

    $("body").on("mouseenter", "#achievements-listbox-table tr", function(){
        $(this).addClass("hovering");
    });

    $("body").on("mouseleave", "#achievements-listbox-table tr", function(){
        $(this).removeClass("hovering");
    });

    await init();
})

async function init(){
    let params = new URLSearchParams(window.location.search);
    let steam_id = params.get("steam_id");

    if (steam_id){
        postProfileSearch(steam_id);
    }
    else{
        console.log("No steam id")
    }
}
