$(document).ready( async function(){

    $("body").on("click", "#search-profile", async function(){
        window.location.href = "/dashboard?steam_id=" + $("#profile-search-input").val()
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
        }
        else {
            _app_state.current_character_id -= 17;
        }

        updateCharacterOverlay();
        updateCompletionMarks();
    })

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
