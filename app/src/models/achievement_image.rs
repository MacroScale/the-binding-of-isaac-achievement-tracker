use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct AchievementImg{
    pub _id: i32,
    pub _text: String,
    pub _gfx: String,
}

impl AchievementImg{


    pub fn new() -> Vec<AchievementImg>{

        let mut achievements = Vec::new();

        achievements.push(AchievementImg{
            _id: 1,
            _text: "You unlocked \'Magdalene\'".to_string(),
            _gfx: "achievement_magdalene.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 2,
            _text: "You unlocked \'Cain\'".to_string(),
            _gfx: "achievement_cain.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 3,
            _text: "You unlocked \'Judas\'".to_string(),
            _gfx: "achievement_judas.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 4,
            _text: "You unlocked \'The Womb\' -Chapter 4- Mother sleeps".to_string(),
            _gfx: "achievement_thewomb.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 5,
            _text: "You unlocked \'The Harbingers\' The Horsemen are loose".to_string(),
            _gfx: "achievement_harbingers.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 6,
            _text: "\'A Cube of Meat\' has appeared in the basement".to_string(),
            _gfx: "achievement_cubeofmeat.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 7,
            _text: "\'The Book of Revelations\' has appeared in the basement".to_string(),
            _gfx: "achievement_bookofrevelations.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 8,
            _text: "\'A Noose\' has appeared in the basement".to_string(),
            _gfx: "achievement_transcendence.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 9,
            _text: "\'The Nail\' has appeared in the basement".to_string(),
            _gfx: "achievement_thenail.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 10,
            _text: "\'A Quarter\' has appeared in the basement".to_string(),
            _gfx: "achievement_aquarter.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 11,
            _text: "\'A Fetus in a Jar\' has appeared in the basement".to_string(),
            _gfx: "achievement_afetusinajar.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 12,
            _text: "\'A Small Rock\' has appeared in the basement".to_string(),
            _gfx: "achievement_asmallrock.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 13,
            _text: "\'Monstro's Tooth\' has appeared in the basement".to_string(),
            _gfx: "achievement_monstrostooth.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 14,
            _text: "\'Lil Chubby\' has appeared in the basement".to_string(),
            _gfx: "achievement_lilchubby.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 15,
            _text: "\'Loki's Horns\' has appeared in the basement".to_string(),
            _gfx: "achievement_lokishorns.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 16,
            _text: "You unlocked \'Something From The Future!\' in the basement".to_string(),
            _gfx: "achievement_steven.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 17,
            _text: "You unlocked \'Something Cute\' in the caves".to_string(),
            _gfx: "achievement_chad.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 18,
            _text: "You unlocked \'Something Sticky\' in the depths".to_string(),
            _gfx: "achievement_gishy.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 19,
            _text: "\'A Bandage\' has appeared in the basement".to_string(),
            _gfx: "achievement_superbandagegirl.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 20,
            _text: "\'A Cross\' has appeared in the basement".to_string(),
            _gfx: "achievement_therelic.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 21,
            _text: "\'A Bag of Pennies\' has appeared in the basement".to_string(),
            _gfx: "achievement_coinbag.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 22,
            _text: "\'The Book of Sin\' has appeared in the basement".to_string(),
            _gfx: "achievement_bookofsin.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 23,
            _text: "\'Little Gish\' has appeared in the basement".to_string(),
            _gfx: "achievement_littlegish.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 24,
            _text: "\'Little Steven\' has appeared in the basement".to_string(),
            _gfx: "achievement_littlesteven.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 25,
            _text: "\'Little C.H.A.D.\' has appeared in the basement".to_string(),
            _gfx: "achievement_littlechad.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 26,
            _text: "\'A Gamekid\' has appeared in the basement".to_string(),
            _gfx: "achievement_gamekid.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 27,
            _text: "\'A Halo\' has appeared in the basement".to_string(),
            _gfx: "achievement_thehalo.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 28,
            _text: "\'Mr. Mega!\' has appeared in the basement".to_string(),
            _gfx: "achievement_mrmega.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 29,
            _text: "Isaac now holds... \'The D6\'".to_string(),
            _gfx: "achievement_d6.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 30,
            _text: "\'The Scissors\' has appeared in the basement".to_string(),
            _gfx: "achievement_scissors.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 31,
            _text: "\'The Parasite\' has appeared in the basement".to_string(),
            _gfx: "achievement_theparasite.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 32,
            _text: "You unlocked \'???\'".to_string(),
            _gfx: "achievement_bluebaby.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 33,
            _text: "Everything is Terrible!!! The game just got harder!".to_string(),
            _gfx: "achievement_everythingisterrible.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 34,
            _text: "\'It Lives!\' Your futures past waits".to_string(),
            _gfx: "achievement_itlives.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 35,
            _text: "\'Mom's Contact\' has appeared in the basement".to_string(),
            _gfx: "achievement_momscontact.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 36,
            _text: "\'The Necronomicon\' has appeared in the basement".to_string(),
            _gfx: "achievement_necronomicon.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 37,
            _text: "\'Basement Boy\' achieved".to_string(),
            _gfx: "achievement_basementboy.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 38,
            _text: "\'Spelunker Boy\' achieved".to_string(),
            _gfx: "achievement_spelunkerboy.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 39,
            _text: "\'Dark Boy\' achieved".to_string(),
            _gfx: "achievement_darkboy.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 40,
            _text: "\'Mamas Boy\' achieved".to_string(),
            _gfx: "achievement_mamasboy.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 41,
            _text: "\'Golden God!\' achieved".to_string(),
            _gfx: "achievement_goldengod.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 42,
            _text: "You unlocked \'Eve\'".to_string(),
            _gfx: "achievement_eve.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 43,
            _text: "\'Mom's Knife\' has appeared in the basement".to_string(),
            _gfx: "achievement_momsknife.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 44,
            _text: "\'The Razor\' has appeared in the basement".to_string(),
            _gfx: "achievement_therazor.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 45,
            _text: "\'Guardian Angel\' has appeared in the basement".to_string(),
            _gfx: "achievement_guardianangel.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 46,
            _text: "\'A Bag of Bombs\' has appeared in the basement".to_string(),
            _gfx: "achievement_bombbag.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 47,
            _text: "\'A Demon Baby!\' has appeared in the basement".to_string(),
            _gfx: "achievement_demonbaby.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 48,
            _text: "\'A Forget Me Now\' has appeared in the basement".to_string(),
            _gfx: "achievement_forgetmenow.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 49,
            _text: "\'The D20\' has appeared in the basement".to_string(),
            _gfx: "achievement_d20.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 50,
            _text: "\'The Crucifix\' has appeared in the basement".to_string(),
            _gfx: "achievement_celticcross.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 51,
            _text: "\'Abel\' has appeared in the basement".to_string(),
            _gfx: "achievement_abel.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 52,
            _text: "\'Curved Horn\' has appeared in the basement".to_string(),
            _gfx: "achievement_curvedhorn.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 53,
            _text: "\'Sacrificial Dagger\' has appeared in the basement".to_string(),
            _gfx: "achievement_sacrificialknife.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 54,
            _text: "\'Blood Lust\' has appeared in the basement".to_string(),
            _gfx: "achievement_bloodylust.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 55,
            _text: "\'Blood Penny\' has appeared in the basement".to_string(),
            _gfx: "achievement_bloodypenny.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 56,
            _text: "\'Blood Rights\' has appeared in the basement".to_string(),
            _gfx: "achievement_bloodrights.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 57,
            _text: "\'The Polaroid\' has appeared in the basement".to_string(),
            _gfx: "achievement_thepolaroid.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 58,
            _text: "\'Dad's Key\' has appeared in the basement".to_string(),
            _gfx: "achievement_dadskey.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 59,
            _text: "\'The Blue Candle\' has appeared in the basement".to_string(),
            _gfx: "achievement_bluecandle.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 60,
            _text: "\'Burnt Penny\' has appeared in the basement".to_string(),
            _gfx: "achievement_burntpenny.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 61,
            _text: "\'Lucky Toe!\' has appeared in the basement".to_string(),
            _gfx: "achievement_luckytoe.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 62,
            _text: "\'Epic Fetus\' has appeared in the basement".to_string(),
            _gfx: "achievement_epicfetus.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 63,
            _text: "\'SMB Super Fan\' has appeared in the basement".to_string(),
            _gfx: "achievement_superfan.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 64,
            _text: "\'Counterfeit Coin\' has appeared in the basement".to_string(),
            _gfx: "achievement_counterfeitpenny.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 65,
            _text: "\'Guppy's Hairball\' has appeared in the basement".to_string(),
            _gfx: "achievement_guppyshairball.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 66,
            _text: "You unlocked \'A Forgotten Horsemen\' in the basement".to_string(),
            _gfx: "achievement_conquest.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 67,
            _text: "You unlocked \'Samson\'".to_string(),
            _gfx: "achievement_samson.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 68,
            _text: "You unlocked \'Something Icky!\' in the basement".to_string(),
            _gfx: "achievement_triachnid.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 69,
            _text: "!Platinum God! OMG!".to_string(),
            _gfx: "achievement_platinumgod.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 70,
            _text: "\'Isaac's Head\' has appeared in the basement".to_string(),
            _gfx: "achievement_isaacshead.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 71,
            _text: "\'Maggy's Faith\' has appeared in the basement".to_string(),
            _gfx: "achievement_maggysfaith.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 72,
            _text: "\'Judas' Tongue\' has appeared in the basement".to_string(),
            _gfx: "achievement_judastongue.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 73,
            _text: "\'???'s Soul\' has appeared in the basement".to_string(),
            _gfx: "achievement_bluebabyssoul.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 74,
            _text: "\'Samson's Lock\' has appeared in the basement".to_string(),
            _gfx: "achievement_samsonslock.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 75,
            _text: "\'Cain's Eye\' has appeared in the basement".to_string(),
            _gfx: "achievement_cainseye.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 76,
            _text: "\'Eve's Bird Foot\' has appeared in the basement".to_string(),
            _gfx: "achievement_evesbirdfoot.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 77,
            _text: "\'The Left Hand\' has appeared in the basement".to_string(),
            _gfx: "achievement_thelefthand.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 78,
            _text: "\'The Negative\' has appeared in the basement".to_string(),
            _gfx: "achievement_thenegative.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 79,
            _text: "You unlocked \'Demon Isaac\'".to_string(),
            _gfx: "achievement_azazel.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 80,
            _text: "You unlocked \'Lazarus\'".to_string(),
            _gfx: "achievement_lazarus.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 81,
            _text: "You unlocked \'Eden\'".to_string(),
            _gfx: "achievement_eden.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 82,
            _text: "You unlocked \'The Soul\'".to_string(),
            _gfx: "achievement_thelost.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 83,
            _text: "\'Dead Boy\' achieved".to_string(),
            _gfx: "achievement_deadboy.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 84,
            _text: "The Real Platinum God".to_string(),
            _gfx: "achievement_realplatinumgod.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 85,
            _text: "\'Lucky Rock\' has appeared in the basement".to_string(),
            _gfx: "achievement_luckyrock.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 86,
            _text: "The Cellar".to_string(),
            _gfx: "achievement_thecellar.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 87,
            _text: "The Catacombs".to_string(),
            _gfx: "achievement_thecatacombs.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 88,
            _text: "The Necropolis".to_string(),
            _gfx: "achievement_necropolis.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 89,
            _text: "\'Rune of Hagalaz\' has appeared in the basement".to_string(),
            _gfx: "achievement_runeofhagalaz.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 90,
            _text: "\'Rune of Jera\' has appeared in the basement".to_string(),
            _gfx: "achievement_runeofjera.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 91,
            _text: "\'Rune of Ehwaz\' has appeared in the basement".to_string(),
            _gfx: "achievement_runeofehwaz.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 92,
            _text: "\'Rune of Dagaz\' has appeared in the basement".to_string(),
            _gfx: "achievement_runeofdagaz.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 93,
            _text: "\'Rune of Ansuz\' has appeared in the basement".to_string(),
            _gfx: "achievement_runeofansuz.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 94,
            _text: "\'Rune of Perthro\' has appeared in the basement".to_string(),
            _gfx: "achievement_runeofperthro.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 95,
            _text: "\'Rune of Berkano\' has appeared in the basement".to_string(),
            _gfx: "achievement_runeofberkano.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 96,
            _text: "\'Rune of Algiz\' has appeared in the basement".to_string(),
            _gfx: "achievement_runeofalgiz.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 97,
            _text: "\'Chaos Card\' has appeared in the basement".to_string(),
            _gfx: "achievement_thechaoscard.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 98,
            _text: "\'Credit Card\' has appeared in the basement".to_string(),
            _gfx: "achievement_creditcard.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 99,
            _text: "\'Rules Card\' has appeared in the basement".to_string(),
            _gfx: "achievement_rulescard.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 100,
            _text: "\'Card Against Humanity\' has appeared in the basement".to_string(),
            _gfx: "achievement_cardagainsthumanity.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 101,
            _text: "\'Swallowed Penny\' has appeared in the basement".to_string(),
            _gfx: "achievement_swallowedpenny.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 102,
            _text: "\'Robo Baby 2.0\' has appeared in the basement".to_string(),
            _gfx: "achievement_robobaby20.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 103,
            _text: "\'Death's Touch\' has appeared in the basement".to_string(),
            _gfx: "achievement_deathstouch.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 104,
            _text: "\'Technology .5\' has appeared in the basement".to_string(),
            _gfx: "achievement_tech5.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 105,
            _text: "\'Missing No.\' has appeared in the basement".to_string(),
            _gfx: "achievement_missingno.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 106,
            _text: "\'Isaac's Tears\' has appeared in the basement".to_string(),
            _gfx: "achievement_isaacstears.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 107,
            _text: "\'Guillotine\' has appeared in the basement".to_string(),
            _gfx: "achievement_guillotine.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 108,
            _text: "\'Judas' Shadow\' has appeared in the basement".to_string(),
            _gfx: "achievement_judasshadow.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 109,
            _text: "\'Maggy's Bow\' has appeared in the basement".to_string(),
            _gfx: "achievement_maggysbow.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 110,
            _text: "\'Cain's Other Eye\' has appeared in the basement".to_string(),
            _gfx: "achievement_cainsothereye.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 111,
            _text: "\'Black Lipstick\' has appeared in the basement".to_string(),
            _gfx: "achievement_blacklipstick.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 112,
            _text: "\'Eve's Mascara\' has appeared in the basement".to_string(),
            _gfx: "achievement_evesmascara.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 113,
            _text: "\'Fate\' has appeared in the basement".to_string(),
            _gfx: "achievement_fate.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 114,
            _text: "\'???'s Only Friend\' has appeared in the basement".to_string(),
            _gfx: "achievement_bluebabysonlyfriend.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 115,
            _text: "\'Samson's Chains\' has appeared in the basement".to_string(),
            _gfx: "achievement_samsonschains.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 116,
            _text: "\'Lazarus' Rags\' has appeared in the basement".to_string(),
            _gfx: "achievement_lazarusrags.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 117,
            _text: "\'Broken Ankh\' has appeared in the basement".to_string(),
            _gfx: "achievement_brokenankh.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 118,
            _text: "\'Store Credit\' has appeared in the basement".to_string(),
            _gfx: "achievement_storecredit.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 119,
            _text: "\'Pandora's Box\' has appeared in the basement".to_string(),
            _gfx: "achievement_pandorasbox.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 120,
            _text: "\'Suicide King\' has appeared in the basement".to_string(),
            _gfx: "achievement_suicideking.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 121,
            _text: "\'Blank Card\' has appeared in the basement".to_string(),
            _gfx: "achievement_blankcard.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 122,
            _text: "\'Book of Secrets\' has appeared in the basement".to_string(),
            _gfx: "achievement_bookofsecrets.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 123,
            _text: "\'Mysterious Paper\' has appeared in the basement".to_string(),
            _gfx: "achievement_mysteriouspaper.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 124,
            _text: "\'Mystery Sack\' has appeared in the basement".to_string(),
            _gfx: "achievement_mysterysack.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 125,
            _text: "\'Undefined\' has appeared in the basement".to_string(),
            _gfx: "achievement_undefined.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 126,
            _text: "\'Satanic Bible\' has appeared in the basement".to_string(),
            _gfx: "achievement_satanicbible.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 127,
            _text: "\'Daemon's Tail\' has appeared in the basement".to_string(),
            _gfx: "achievement_demontail.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 128,
            _text: "\'Abaddon\' has appeared in the basement".to_string(),
            _gfx: "achievement_abaddon.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 129,
            _text: "\'Isaac's Heart\' has appeared in the basement".to_string(),
            _gfx: "achievement_isaacsheart.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 130,
            _text: "\'The Mind\' has appeared in the basement".to_string(),
            _gfx: "achievement_themind.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 131,
            _text: "\'The Body\' has appeared in the basement".to_string(),
            _gfx: "achievement_thebody.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 132,
            _text: "\'The Soul\' has appeared in the basement".to_string(),
            _gfx: "achievement_thesoul.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 133,
            _text: "\'A D100\' has appeared in the basement".to_string(),
            _gfx: "achievement_d100.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 134,
            _text: "\'Blue Map\' has appeared in the basement".to_string(),
            _gfx: "achievement_bluemap.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 135,
            _text: "\'There's Options\' has appeared in the basement".to_string(),
            _gfx: "achievement_theresoptions.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 136,
            _text: "\'Black Candle\' has appeared in the basement".to_string(),
            _gfx: "achievement_blackcandle.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 137,
            _text: "\'Red Candle\' has appeared in the basement".to_string(),
            _gfx: "achievement_redcandle.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 138,
            _text: "\'Stop Watch\' has appeared in the basement".to_string(),
            _gfx: "achievement_stopwatch.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 139,
            _text: "\'Wire Coat Hanger\' has appeared in the basement".to_string(),
            _gfx: "achievement_wirecoathanger.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 140,
            _text: "\'Ipecac\' has appeared in the basement".to_string(),
            _gfx: "achievement_ipecac.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 141,
            _text: "\'Experimental Treatment\' has appeared in the basement".to_string(),
            _gfx: "achievement_experimentaltreatment.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 142,
            _text: "Krampus unlocked".to_string(),
            _gfx: "achievement_krampus.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 143,
            _text: "\'Head of Krampus\' has appeared in the basement".to_string(),
            _gfx: "achievement_headofkrampus.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 144,
            _text: "You made a Super Meat Boy".to_string(),
            _gfx: "achievement_supermeatboy.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 145,
            _text: "\'Butter Bean\' has appeared in the basement".to_string(),
            _gfx: "achievement_butterbean.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 146,
            _text: "\'Little Baggy\' has appeared in the basement".to_string(),
            _gfx: "achievement_littlebaggy.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 147,
            _text: "\'Blood Bag\' has appeared in the basement".to_string(),
            _gfx: "achievement_bloodbag.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 148,
            _text: "\'A D4\' has appeared in the basement".to_string(),
            _gfx: "achievement_d4.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 149,
            _text: "\'The Lost Poster\' has appeared in the basement".to_string(),
            _gfx: "achievement_lostposter.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 150,
            _text: "\'Rubber Cement\' has appeared in the basement".to_string(),
            _gfx: "achievement_rubbercement.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 151,
            _text: "Store Upgrade lv.1".to_string(),
            _gfx: "achievement_storeupgrade1.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 152,
            _text: "Store Upgrade lv.2".to_string(),
            _gfx: "achievement_storeupgrade2.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 153,
            _text: "Store Upgrade lv.3".to_string(),
            _gfx: "achievement_storeupgrade3.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 154,
            _text: "Store Upgrade lv.4".to_string(),
            _gfx: "achievement_storeupgrade4.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 155,
            _text: "You unlocked the Angel bosses".to_string(),
            _gfx: "achievement_angels.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 156,
            _text: "\'Godhead\' has appeared in the basement".to_string(),
            _gfx: "achievement_godhead.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 157,
            _text: "You unlocked Challenge #4 Darkness Falls".to_string(),
            _gfx: "achievement_challenge04.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 158,
            _text: "You unlocked Challenge #5 The Tank".to_string(),
            _gfx: "achievement_challenge05.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 159,
            _text: "You unlocked Challenge #6 Solar System".to_string(),
            _gfx: "achievement_challenge06.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 160,
            _text: "You unlocked Challenge #7 Suicide King".to_string(),
            _gfx: "achievement_challenge07.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 161,
            _text: "You unlocked Challenge #8 Cat got your Tongue".to_string(),
            _gfx: "achievement_challenge08.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 162,
            _text: "You unlocked Challenge #9 Demo Man".to_string(),
            _gfx: "achievement_challenge09.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 163,
            _text: "You unlocked Challenge #10 Cursed!".to_string(),
            _gfx: "achievement_challenge10.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 164,
            _text: "You unlocked Challenge #11 Glass Cannon".to_string(),
            _gfx: "achievement_challenge11.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 165,
            _text: "You unlocked Challenge #19 The Family Man".to_string(),
            _gfx: "achievement_challenge19.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 166,
            _text: "You unlocked Challenge #20 Purist".to_string(),
            _gfx: "achievement_challenge20.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 167,
            _text: "You unlocked \'Lost Baby\'".to_string(),
            _gfx: "achievement_lostbaby.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 168,
            _text: "You unlocked \'Cute Baby\'".to_string(),
            _gfx: "achievement_cutebaby.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 169,
            _text: "You unlocked \'Crow Baby\'".to_string(),
            _gfx: "achievement_crowbaby.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 170,
            _text: "You unlocked \'Shadow Baby\'".to_string(),
            _gfx: "achievement_shadowbaby.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 171,
            _text: "You unlocked \'Glass Baby\'".to_string(),
            _gfx: "achievement_glassbaby.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 172,
            _text: "You unlocked \'Wrapped Baby\'".to_string(),
            _gfx: "achievement_wrappedbaby.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 173,
            _text: "You unlocked \'Begotten Baby\'".to_string(),
            _gfx: "achievement_begottenbaby.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 174,
            _text: "You unlocked \'Dead Baby\'".to_string(),
            _gfx: "achievement_deadbaby.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 175,
            _text: "You unlocked \'-0- Baby\'".to_string(),
            _gfx: "achievement_0baby.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 176,
            _text: "You unlocked \'Glitch Baby\'".to_string(),
            _gfx: "achievement_glitchbaby.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 177,
            _text: "You unlocked \'Fighting Baby\'".to_string(),
            _gfx: "achievement_fightingbaby.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 178,
            _text: "You became Lord of the Flies".to_string(),
            _gfx: "achievement_lordoftheflies.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 179,
            _text: "\'Fart Baby\' has appeared in the basement".to_string(),
            _gfx: "achievement_179_fartbaby.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 180,
            _text: "\'Purity\' has appeared in the basement".to_string(),
            _gfx: "achievement_180_purity.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 181,
            _text: "\'D12\' has appeared in the basement".to_string(),
            _gfx: "achievement_181_d12.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 182,
            _text: "\'Betrayal\' has appeared in the basement".to_string(),
            _gfx: "achievement_182_betrayal.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 183,
            _text: "\'Fate's Reward\' has appeared in the basement".to_string(),
            _gfx: "achievement_183_fatesreward.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 184,
            _text: "\'Athame\' has appeared in the basement".to_string(),
            _gfx: "achievement_184_athame.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 185,
            _text: "\'Blind Rage\' has appeared in the basement".to_string(),
            _gfx: "achievement_185_blindrage.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 186,
            _text: "\'Maw of the Void\' has appeared in the basement".to_string(),
            _gfx: "achievement_186_mawofthevoid.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 187,
            _text: "\'Empty Vessel\' has appeared in the basement".to_string(),
            _gfx: "achievement_187_emptyvessel.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 188,
            _text: "\'Eden's Blessing\' has appeared in the basement".to_string(),
            _gfx: "achievement_188_edensblessing.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 189,
            _text: "\'Sworn Protector\' has appeared in the basement".to_string(),
            _gfx: "achievement_189_swornprotector.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 190,
            _text: "\'Incubus\' has appeared in the basement".to_string(),
            _gfx: "achievement_190_incubus.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 191,
            _text: "Keeper now holds... \'A Penny!\'".to_string(),
            _gfx: "achievement_191_apenny.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 192,
            _text: "\'Lil' Chest\' has appeared in the basement".to_string(),
            _gfx: "achievement_192_lilchest.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 193,
            _text: "\'Censer\' has appeared in the basement".to_string(),
            _gfx: "achievement_193_censer.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 194,
            _text: "\'Evil Eye\' has appeared in the basement".to_string(),
            _gfx: "achievement_194_evileye.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 195,
            _text: "\'My Shadow\' has appeared in the basement".to_string(),
            _gfx: "achievement_195_myshadow.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 196,
            _text: "\'Cracked Dice\' has appeared in the basement".to_string(),
            _gfx: "achievement_196_crackeddice.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 197,
            _text: "\'Black Feather\' has appeared in the basement".to_string(),
            _gfx: "achievement_197_blackfeather.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 198,
            _text: "\'Lusty Blood\' has appeared in the basement".to_string(),
            _gfx: "achievement_198_lustyblood.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 199,
            _text: "You unlocked \'Lilith\'".to_string(),
            _gfx: "achievement_199_lilith.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 200,
            _text: "\'Key Bum\' has appeared in the basement".to_string(),
            _gfx: "achievement_200_keybum.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 201,
            _text: "\'GB Bug\' has appeared in the basement".to_string(),
            _gfx: "achievement_201_gbbug.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 202,
            _text: "\'Zodiac\' has appeared in the basement".to_string(),
            _gfx: "achievement_202_zodiac.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 203,
            _text: "\'Box of Friends\' has appeared in the basement".to_string(),
            _gfx: "achievement_203_boxoffriends.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 204,
            _text: "\'Rib of Greed\' has appeared in the basement".to_string(),
            _gfx: "achievement_204_ribofgreed.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 205,
            _text: "\'Cry Baby\' has appeared in the basement".to_string(),
            _gfx: "achievement_205_crybaby.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 206,
            _text: "\'Red Baby\' has appeared in the basement".to_string(),
            _gfx: "achievement_206_redbaby.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 207,
            _text: "\'Green Baby\' has appeared in the basement".to_string(),
            _gfx: "achievement_207_greenbaby.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 208,
            _text: "\'Brown Baby\' has appeared in the basement".to_string(),
            _gfx: "achievement_208_brownbaby.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 209,
            _text: "\'Blue Baby\' has appeared in the basement".to_string(),
            _gfx: "achievement_209_bluebaby.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 210,
            _text: "\'Lil' Baby\' has appeared in the basement".to_string(),
            _gfx: "achievement_210_lilbaby.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 211,
            _text: "\'Rage Baby\' has appeared in the basement".to_string(),
            _gfx: "achievement_211_ragebaby.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 212,
            _text: "\'Black Baby\' has appeared in the basement".to_string(),
            _gfx: "achievement_212_blackbaby.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 213,
            _text: "\'Long Baby\' has appeared in the basement".to_string(),
            _gfx: "achievement_213_longbaby.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 214,
            _text: "\'Yellow Baby\' has appeared in the basement".to_string(),
            _gfx: "achievement_214_yellowbaby.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 215,
            _text: "\'White Baby\' has appeared in the basement".to_string(),
            _gfx: "achievement_215_whitebaby.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 216,
            _text: "\'Big Baby\' has appeared in the basement".to_string(),
            _gfx: "achievement_216_bigbaby.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 217,
            _text: "\'Noose Baby\' has appeared in the basement".to_string(),
            _gfx: "achievement_217_noosebaby.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 218,
            _text: "\'Rune Bag\' has appeared in the basement".to_string(),
            _gfx: "achievement_218_runebag.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 219,
            _text: "\'Cambion Conception\' has appeared in the basement".to_string(),
            _gfx: "achievement_219_cambionconception.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 220,
            _text: "\'Serpent's Kiss\' has appeared in the basement".to_string(),
            _gfx: "achievement_220_serpentskiss.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 221,
            _text: "\'Succubus\' has appeared in the basement".to_string(),
            _gfx: "achievement_221_succubus.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 222,
            _text: "\'Immaculate Conception\' has appeared in the basement".to_string(),
            _gfx: "achievement_222_immaculateconception.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 223,
            _text: "\'Goat Head Baby\' has appeared in the basement".to_string(),
            _gfx: "achievement_223_goatheadbaby.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 224,
            _text: "\'Gold Heart\' has appeared in the basement".to_string(),
            _gfx: "achievement_224_goldheart.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 225,
            _text: "Get out of Jail Free Card has appeared in the basement".to_string(),
            _gfx: "achievement_225_getoutofjailfreecard.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 226,
            _text: "\'Gold Bomb\' has appeared in the basement".to_string(),
            _gfx: "achievement_226_goldbomb.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 227,
            _text: "2 new pills have appeared".to_string(),
            _gfx: "achievement_227_2newpills1.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 228,
            _text: "2 new pills have appeared".to_string(),
            _gfx: "achievement_228_2newpills2.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 229,
            _text: "\'Poker Chip\' has appeared in the basement".to_string(),
            _gfx: "achievement_229_pokerchip.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 230,
            _text: "\'Stud Finder\' has appeared in the basement".to_string(),
            _gfx: "achievement_230_studfinder.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 231,
            _text: "\'D8\' has appeared in the basement".to_string(),
            _gfx: "achievement_231_d8.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 232,
            _text: "\'Kidney Stone\' has appeared in the basement".to_string(),
            _gfx: "achievement_233_kidneystone.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 233,
            _text: "\'Blank Rune\' has appeared in the basement".to_string(),
            _gfx: "achievement_232_blankrune.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 234,
            _text: "You unlocked the Blue Womb!".to_string(),
            _gfx: "achievement_234_bluewomb.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 235,
            _text: "1001% (Nerd x 1000000)".to_string(),
            _gfx: "achievement_235_1000percent.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 236,
            _text: "Keeper now holds... \'Wooden Nickel\'".to_string(),
            _gfx: "achievement_236_keeperwoodennickel.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 237,
            _text: "Keeper now holds... \'Store Key\'".to_string(),
            _gfx: "achievement_237_keeperstorekey.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 238,
            _text: "\'Deep Pockets\' has appeared in the basement".to_string(),
            _gfx: "achievement_238_deeppockets.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 239,
            _text: "\'Karma\' has appeared in the basement".to_string(),
            _gfx: "achievement_239_karma.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 240,
            _text: "Sticky Nickels have appeared in the basement".to_string(),
            _gfx: "achievement_240_stickynickels.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 241,
            _text: "\'Super Greed Baby\' has appeared in the basement".to_string(),
            _gfx: "achievement_241_supergreedbaby.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 242,
            _text: "Lucky Pennies have appeared in the basement".to_string(),
            _gfx: "achievement_242_luckypenny.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 243,
            _text: "You unlocked Special Hanging Shopkeepers!".to_string(),
            _gfx: "achievement_243_specialhangingshopkeepers.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 244,
            _text: "\'Wooden Nickel\' has appeared in the basement".to_string(),
            _gfx: "achievement_244_woodennickel.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 245,
            _text: "Cain now holds... \'Paperclip\'".to_string(),
            _gfx: "achievement_245_cainpaperclip.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 246,
            _text: "Everything is terrible 2!!! Greed just got harder!".to_string(),
            _gfx: "achievement_246_everythingisterrible2.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 247,
            _text: "You unlocked Special Shopkeepers!".to_string(),
            _gfx: "achievement_247_specialshopkeepers.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 248,
            _text: "Eve now holds... \'Razor Blade\'".to_string(),
            _gfx: "achievement_248_everazorblade.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 249,
            _text: "\'Store Key\' has appeared in the basement".to_string(),
            _gfx: "achievement_249_storekey.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 250,
            _text: "Lost now holds... \'Holy Mantle\'".to_string(),
            _gfx: "achievement_250_lostholymantle.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 251,
            _text: "You unlocked \'Keeper\'".to_string(),
            _gfx: "achievement_251_keeper.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 252,
            _text: "\'Hive Baby\' has appeared in the basement".to_string(),
            _gfx: "achievement_252_hivebaby.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 253,
            _text: "\'Buddy Baby\' has appeared in the basement".to_string(),
            _gfx: "achievement_253_buddybaby.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 254,
            _text: "\'Colorful Baby\' has appeared in the basement".to_string(),
            _gfx: "achievement_254_colorfulbaby.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 255,
            _text: "\'Whore Baby\' has appeared in the basement".to_string(),
            _gfx: "achievement_255_whorebaby.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 256,
            _text: "\'Cracked Baby\' has appeared in the basement".to_string(),
            _gfx: "achievement_256_crackedbaby.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 257,
            _text: "\'Dripping Baby\' has appeared in the basement".to_string(),
            _gfx: "achievement_257_drippingbaby.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 258,
            _text: "\'Blinding Baby\' has appeared in the basement".to_string(),
            _gfx: "achievement_258_blindingbaby.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 259,
            _text: "\'Sucky Baby\' has appeared in the basement".to_string(),
            _gfx: "achievement_259_suckybaby.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 260,
            _text: "\'Dark Baby\' has appeared in the basement".to_string(),
            _gfx: "achievement_260_darkbaby.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 261,
            _text: "\'Picky Baby\' has appeared in the basement".to_string(),
            _gfx: "achievement_261_pickybaby.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 262,
            _text: "\'Revenge Baby\' has appeared in the basement".to_string(),
            _gfx: "achievement_262_revengebaby.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 263,
            _text: "\'Belial Baby\' has appeared in the basement".to_string(),
            _gfx: "achievement_263_belialbaby.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 264,
            _text: "\'Sale Baby\' has appeared in the basement".to_string(),
            _gfx: "achievement_264_salebaby.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 265,
            _text: "You unlocked Challenge #21 XXXXXXXXL".to_string(),
            _gfx: "achievement_challenge21.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 266,
            _text: "You unlocked Challenge #22 SPEED!".to_string(),
            _gfx: "achievement_challenge22.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 267,
            _text: "You unlocked Challenge #23 Blue Bomber".to_string(),
            _gfx: "achievement_challenge23.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 268,
            _text: "You unlocked Challenge #24 PAY TO PLAY".to_string(),
            _gfx: "achievement_challenge24.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 269,
            _text: "You unlocked Challenge #25 Have a heart".to_string(),
            _gfx: "achievement_challenge25.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 270,
            _text: "You unlocked Challenge #26 I RULE!".to_string(),
            _gfx: "achievement_challenge26.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 271,
            _text: "You unlocked Challenge #27 BRAINS!".to_string(),
            _gfx: "achievement_challenge27.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 272,
            _text: "You unlocked Challenge #28 PRIDE DAY!".to_string(),
            _gfx: "achievement_challenge28.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 273,
            _text: "You unlocked Challenge #29 Onan's Streak".to_string(),
            _gfx: "achievement_challenge29.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 274,
            _text: "You unlocked Challenge #30 The Guardian".to_string(),
            _gfx: "achievement_challenge30.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 275,
            _text: "If only everyone was as generous as you are...".to_string(),
            _gfx: "achievement_generosity.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 276,
            _text: "\'Mega Blast\' has appeared in the basement".to_string(),
            _gfx: "achievement_megablast.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 277,
            _text: "You unlocked Challenge #31 Backasswards".to_string(),
            _gfx: "achievement_challenge31.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 278,
            _text: "You unlocked Challenge #32 Aprils fool".to_string(),
            _gfx: "achievement_challenge32.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 279,
            _text: "You unlocked Challenge #33 Pokey mans".to_string(),
            _gfx: "achievement_challenge33.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 280,
            _text: "You unlocked Challenge #34 Ultra hard".to_string(),
            _gfx: "achievement_challenge34.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 281,
            _text: "You unlocked Challenge #35 PONG".to_string(),
            _gfx: "achievement_challenge35.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 282,
            _text: "\'D infinity\' has appeared in the basement".to_string(),
            _gfx: "achievement_dinf.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 283,
            _text: "\'Eucharist\' has appeared in the basement".to_string(),
            _gfx: "achievement_eucharist.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 284,
            _text: "\'Silver Dollar\' has appeared in the basement".to_string(),
            _gfx: "achievement_silverdollar.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 285,
            _text: "\'Shade\' has appeared in the basement".to_string(),
            _gfx: "achievement_shade.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 286,
            _text: "\'King Baby\' has appeared in the basement".to_string(),
            _gfx: "achievement_kingbaby.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 287,
            _text: "\'Bloody Crown\' has appeared in the basement".to_string(),
            _gfx: "achievement_bloodycrown.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 288,
            _text: "\'Dull Razor\' has appeared in the basement".to_string(),
            _gfx: "achievement_dullrazor.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 289,
            _text: "\'Eden's Soul\' has appeared in the basement".to_string(),
            _gfx: "achievement_edenssoul.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 290,
            _text: "\'Dark Prince's Crown\' has appeared in the basement".to_string(),
            _gfx: "achievement_darkprincescrown.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 291,
            _text: "\'Compound Fracture\' has appeared in the basement".to_string(),
            _gfx: "achievement_compoundfracture.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 292,
            _text: "\'Euthanasia\' has appeared in the basement".to_string(),
            _gfx: "achievement_euthanasia.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 293,
            _text: "\'Holy Card\' has appeared in the basement".to_string(),
            _gfx: "achievement_holycard.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 294,
            _text: "\'Crooked Penny\' has appeared in the basement".to_string(),
            _gfx: "achievement_crookedpenny.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 295,
            _text: "\'Void\' has appeared in the basement".to_string(),
            _gfx: "achievement_void.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 296,
            _text: "\'D1\' has appeared in the basement".to_string(),
            _gfx: "achievement_d1.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 297,
            _text: "\'Glyph of Balance\' has appeared in the basement".to_string(),
            _gfx: "achievement_glyphofbalance.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 298,
            _text: "\'Sack of Sacks\' has appeared in the basement".to_string(),
            _gfx: "achievement_sackofsacks.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 299,
            _text: "\'Eye of Belial\' has appeared in the basement".to_string(),
            _gfx: "achievement_eyeofbelial.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 300,
            _text: "\'Meconium\' has appeared in the basement".to_string(),
            _gfx: "achievement_meconium.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 301,
            _text: "\'Stem Cell\' has appeared in the basement".to_string(),
            _gfx: "achievement_stemcell.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 302,
            _text: "\'Crow Heart\' has appeared in the basement".to_string(),
            _gfx: "achievement_crowheart.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 303,
            _text: "\'Metronome\' has appeared in the basement".to_string(),
            _gfx: "achievement_metronome.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 304,
            _text: "\'Bat Wing\' has appeared in the basement".to_string(),
            _gfx: "achievement_batwing.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 305,
            _text: "\'Plan C\' has appeared in the basement".to_string(),
            _gfx: "achievement_planc.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 306,
            _text: "\'Duality\' has appeared in the basement".to_string(),
            _gfx: "achievement_duality.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 307,
            _text: "\'Dad's Lost Coin\' has appeared in the basement".to_string(),
            _gfx: "achievement_dadslostcoin.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 308,
            _text: "\'Eye of Greed\' has appeared in the basement".to_string(),
            _gfx: "achievement_eyeofgreed.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 309,
            _text: "\'Black Rune\' has appeared in the basement".to_string(),
            _gfx: "achievement_blackrune.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 310,
            _text: "\'Locust of Wrath\' has appeared in the basement".to_string(),
            _gfx: "achievement_locustofwrath.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 311,
            _text: "\'Locust of Pestilence\' has appeared in the basement".to_string(),
            _gfx: "achievement_locustofpestilence.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 312,
            _text: "\'Locust of Famine\' has appeared in the basement".to_string(),
            _gfx: "achievement_locustoffamine.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 313,
            _text: "\'Locust of Death\' has appeared in the basement".to_string(),
            _gfx: "achievement_locustofdeath.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 314,
            _text: "\'Locust of Conquest\' has appeared in the basement".to_string(),
            _gfx: "achievement_locustofconquest.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 315,
            _text: "\'Hushy\' has appeared in the basement".to_string(),
            _gfx: "achievement_hushy.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 316,
            _text: "\'Brown Nugget\' has appeared in the basement".to_string(),
            _gfx: "achievement_brownnugget.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 317,
            _text: "\'Mort Baby\' has appeared in the basement".to_string(),
            _gfx: "achievement_mortbaby.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 318,
            _text: "\'Smelter\' has appeared in the basement".to_string(),
            _gfx: "achievement_smelter.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 319,
            _text: "\'Apollyon Baby\' has appeared in the basement".to_string(),
            _gfx: "achievement_apollyonbaby.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 320,
            _text: "You unlocked a new area!".to_string(),
            _gfx: "achievement_thevoid.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 321,
            _text: "Once more with feeling!".to_string(),
            _gfx: "achievement_pillgulp.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 322,
            _text: "Hat trick!".to_string(),
            _gfx: "achievement_aceofclubs.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 323,
            _text: "5 nights at moms".to_string(),
            _gfx: "achievement_superspecialrocks.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 324,
            _text: "Sin collector".to_string(),
            _gfx: "achievement_pillsunshine.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 325,
            _text: "Dedication".to_string(),
            _gfx: "achievement_pillhorf.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 326,
            _text: "ZIP!".to_string(),
            _gfx: "achievement_aceofdiamonds.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 327,
            _text: "It's the key".to_string(),
            _gfx: "achievement_aceofspades.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 328,
            _text: "Mr. Resetter!".to_string(),
            _gfx: "achievement_scaredheart.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 329,
            _text: "Living on the edge".to_string(),
            _gfx: "achievement_aceofhearts.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 330,
            _text: "U broke it!".to_string(),
            _gfx: "achievement_pillvurp.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 331,
            _text: "Laz Bleeds More!".to_string(),
            _gfx: "achievement_lazbleedsmore.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 332,
            _text: "Maggy now hold a pill!".to_string(),
            _gfx: "achievement_maggynowholdsapill.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 333,
            _text: "\'Charged Key\' has appeared in the basement".to_string(),
            _gfx: "achievement_chargedkey.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 334,
            _text: "Samson feels healthy!".to_string(),
            _gfx: "achievement_samsonfeelshealthy.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 335,
            _text: "\'Greed's Gullet\' has appeared in the basement".to_string(),
            _gfx: "achievement_greedsgullet.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 336,
            _text: "\'Cracked Crown\' has appeared in the basement".to_string(),
            _gfx: "achievement_crackedcrown.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 337,
            _text: "You unlocked RERUN!".to_string(),
            _gfx: "achievement_rerun.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 338,
            _text: "\'Delirious\' has appeared in the basement".to_string(),
            _gfx: "achievement_delirious.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 339,
            _text: "1000000% Just Stop!".to_string(),
            _gfx: "achievement_1000000percent.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 340,
            _text: "You unlocked \'Apollyon\'".to_string(),
            _gfx: "achievement_apollyon.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 341,
            _text: "You just got Greedier!".to_string(),
            _gfx: "achievement_greedier.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 342,
            _text: "The Basement is burning!".to_string(),
            _gfx: "achievement_burningbasement.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 343,
            _text: "The Caves are flooded!".to_string(),
            _gfx: "achievement_floodedcaves.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 344,
            _text: "The Depths are dank!".to_string(),
            _gfx: "achievement_dankdepths.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 345,
            _text: "The Womb is scarred!".to_string(),
            _gfx: "achievement_scarredwomb.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 346,
            _text: "Something wicked this way comes!".to_string(),
            _gfx: "achievement_somethingwicked.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 347,
            _text: "Something wicked this way comes+!".to_string(),
            _gfx: "achievement_somethingwicked+.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 348,
            _text: "The gate is open!".to_string(),
            _gfx: "achievement_gateopen.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 349,
            _text: "\'Black Hole\' has appeared in the basement".to_string(),
            _gfx: "achievement_blackhole.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 350,
            _text: "\'Mystery Gift\' has appeared in the basement".to_string(),
            _gfx: "achievement_mysterygift.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 351,
            _text: "\'Sprinkler\' has appeared in the basement".to_string(),
            _gfx: "achievement_sprinkler.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 352,
            _text: "\'Angry Fly\' has appeared in the basement".to_string(),
            _gfx: "achievement_angryfly.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 353,
            _text: "\'Bozo\' has appeared in the basement".to_string(),
            _gfx: "achievement_bozo.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 354,
            _text: "\'Broken Modem\' has appeared in the basement".to_string(),
            _gfx: "achievement_brokenmodem.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 355,
            _text: "\'Buddy in a Box\' has appeared in the basement".to_string(),
            _gfx: "achievement_buddyinabox.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 356,
            _text: "\'Fast Bombs\' has appeared in the basement".to_string(),
            _gfx: "achievement_fastbombs.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 357,
            _text: "\'Lil Delirium\' has appeared in the basement".to_string(),
            _gfx: "achievement_lildelirium.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 358,
            _text: "\'Hairpin\' has appeared in the basement".to_string(),
            _gfx: "achievement_hairpin.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 359,
            _text: "\'Wooden Cross\' has appeared in the basement".to_string(),
            _gfx: "achievement_woodencross.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 360,
            _text: "\'Butter\' has appeared in the basement".to_string(),
            _gfx: "achievement_butter.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 361,
            _text: "\'Huge Growth\' has appeared in the basement".to_string(),
            _gfx: "achievement_hugegrowth.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 362,
            _text: "\'Ancient Recall\' has appeared in the basement".to_string(),
            _gfx: "achievement_ancientrecall.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 363,
            _text: "\'Era Walk\' has appeared in the basement".to_string(),
            _gfx: "achievement_erawalk.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 364,
            _text: "\'Coupon\' has appeared in the basement".to_string(),
            _gfx: "achievement_coupon.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 365,
            _text: "\'Telekinesis\' has appeared in the basement".to_string(),
            _gfx: "achievement_telekinesis.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 366,
            _text: "\'Moving Box\' has appeared in the basement".to_string(),
            _gfx: "achievement_movingbox.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 367,
            _text: "\'Jumper Cables\' has appeared in the basement".to_string(),
            _gfx: "achievement_jumpercables.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 368,
            _text: "\'Leprosy\' has appeared in the basement".to_string(),
            _gfx: "achievement_leprosy.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 369,
            _text: "\'Technology Zero\' has appeared in the basement".to_string(),
            _gfx: "achievement_technologyzero.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 370,
            _text: "\'Filigree Feather\' has appeared in the basement".to_string(),
            _gfx: "achievement_filigreefeather.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 371,
            _text: "\'Mr. Me\' has appeared in the basement".to_string(),
            _gfx: "achievement_mrme.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 372,
            _text: "\'7 Seals\' has appeared in the basement".to_string(),
            _gfx: "achievement_7seals.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 373,
            _text: "\'Angelic Prism\' has appeared in the basement".to_string(),
            _gfx: "achievement_angelicprism.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 374,
            _text: "\'Pop!\' has appeared in the basement".to_string(),
            _gfx: "achievement_pop.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 375,
            _text: "\'Door Stop\' has appeared in the basement".to_string(),
            _gfx: "achievement_doorstop.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 376,
            _text: "\'Death's List\' has appeared in the basement".to_string(),
            _gfx: "achievement_deathslist.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 377,
            _text: "\'Haemolacria\' has appeared in the basement".to_string(),
            _gfx: "achievement_haemolacria.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 378,
            _text: "\'Lachryphagy\' has appeared in the basement".to_string(),
            _gfx: "achievement_lachryphagy.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 379,
            _text: "\'Schoolbag\' has appeared in the basement".to_string(),
            _gfx: "achievement_schoolbag.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 380,
            _text: "\'Trisagion\' has appeared in the basement".to_string(),
            _gfx: "achievement_trisagion.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 381,
            _text: "\'Extension Cord\' has appeared in the basement".to_string(),
            _gfx: "achievement_extensioncord.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 382,
            _text: "\'Flat Stone\' has appeared in the basement".to_string(),
            _gfx: "achievement_flatstone.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 383,
            _text: "\'Sacrificial Altar\' has appeared in the basement".to_string(),
            _gfx: "achievement_sacrificialaltar.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 384,
            _text: "\'Lil Spewer\' has appeared in the basement".to_string(),
            _gfx: "achievement_lilspewer.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 385,
            _text: "\'Blanket\' has appeared in the basement".to_string(),
            _gfx: "achievement_blanket.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 386,
            _text: "\'Marbles\' has appeared in the basement".to_string(),
            _gfx: "achievement_marbles.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 387,
            _text: "\'Mystery Egg\' has appeared in the basement".to_string(),
            _gfx: "achievement_mysteryegg.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 388,
            _text: "\'Rotten Penny\' has appeared in the basement".to_string(),
            _gfx: "achievement_rottenpenny.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 389,
            _text: "\'Baby-Bender\' has appeared in the basement".to_string(),
            _gfx: "achievement_babybender.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 390,
            _text: "You unlocked \'The Forgotten\'".to_string(),
            _gfx: "achievement_theforgotten.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 391,
            _text: "\'Bone Heart\' has appeared in the basement".to_string(),
            _gfx: "achievement_boneheart.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 392,
            _text: "\'Marrow\' has appeared in the basement".to_string(),
            _gfx: "achievement_marrow.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 393,
            _text: "\'Slipped Rib\' has appeared in the basement".to_string(),
            _gfx: "achievement_slippedrib.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 394,
            _text: "\'Pointy Rib\' has appeared in the basement".to_string(),
            _gfx: "achievement_pointyrib.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 395,
            _text: "\'Jaw Bone\' has appeared in the basement".to_string(),
            _gfx: "achievement_jawbone.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 396,
            _text: "\'Brittle Bones\' has appeared in the basement".to_string(),
            _gfx: "achievement_brittlebones.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 397,
            _text: "\'Divorce Papers\' has appeared in the basement".to_string(),
            _gfx: "achievement_divorcepapers.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 398,
            _text: "\'Hallowed Ground\' has appeared in the basement".to_string(),
            _gfx: "achievement_hallowedground.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 399,
            _text: "\'Finger Bone\' has appeared in the basement".to_string(),
            _gfx: "achievement_fingerbone.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 400,
            _text: "\'Dad's Ring\' has appeared in the basement".to_string(),
            _gfx: "achievement_dadsring.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 401,
            _text: "\'Book of the Dead\' has appeared in the basement".to_string(),
            _gfx: "achievement_bookofthedead.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 402,
            _text: "\'Bone Baby\' has appeared in the basement".to_string(),
            _gfx: "achievement_bonebaby.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 403,
            _text: "\'Bound Baby\' has appeared in the basement".to_string(),
            _gfx: "achievement_boundbaby.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 404,
            _text: "You unlocked \'Bethany\'".to_string(),
            _gfx: "achievement_bethany.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 405,
            _text: "You unlocked \'Jacob&Esau\'".to_string(),
            _gfx: "achievement_jacobesau.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 406,
            _text: "The stars are calling...".to_string(),
            _gfx: "achievement_planetarium.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 407,
            _text: "You unlocked \'A Secret Exit\'".to_string(),
            _gfx: "achievement_secretexit.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 408,
            _text: "\'Forgotten Lullaby\' has appeared in the basement".to_string(),
            _gfx: "achievement_forgottenlullaby.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 409,
            _text: "\'Fruity Plum\' has appeared in the basement".to_string(),
            _gfx: "achievement_fruityplum.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 410,
            _text: "\'Plum Flute\' has appeared in the basement".to_string(),
            _gfx: "achievement_plumflute.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 411,
            _text: "\'Rotten Heart\' has appeared in the basement".to_string(),
            _gfx: "achievement_rottenheart.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 412,
            _text: "You unlocked \'Dross\'".to_string(),
            _gfx: "achievement_dross.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 413,
            _text: "You unlocked \'Ashpit\'".to_string(),
            _gfx: "achievement_ashpit.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 414,
            _text: "You unlocked \'Gehenna\'".to_string(),
            _gfx: "achievement_gehenna.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 415,
            _text: "\'Red Key\' has appeared in the basement".to_string(),
            _gfx: "achievement_redkey.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 416,
            _text: "You unlocked \'Wisp Baby\'".to_string(),
            _gfx: "achievement_wispbaby.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 417,
            _text: "\'Book of Virtues\' has appeared in the basement".to_string(),
            _gfx: "achievement_bookofvirtues.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 418,
            _text: "\'Urn of Souls\' has appeared in the basement".to_string(),
            _gfx: "achievement_urnofsouls.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 419,
            _text: "\'Blessed Penny\' has appeared in the basement".to_string(),
            _gfx: "achievement_blessedpenny.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 420,
            _text: "\'Alabaster Box\' has appeared in the basement".to_string(),
            _gfx: "achievement_alabasterbox.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 421,
            _text: "\'Beth's Faith\' has appeared in the basement".to_string(),
            _gfx: "achievement_bethsfaith.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 422,
            _text: "\'Soul Locket\' has appeared in the basement".to_string(),
            _gfx: "achievement_soullocket.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 423,
            _text: "\'Divine Intervention\' has appeared in the basement".to_string(),
            _gfx: "achievement_divineintervention.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 424,
            _text: "\'Vade Retro\' has appeared in the basement".to_string(),
            _gfx: "achievement_vaderetro.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 425,
            _text: "\'Star of Bethlehem\' has appeared in the basement".to_string(),
            _gfx: "achievement_starofbethlehem.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 426,
            _text: "You unlocked \'Hope Baby\'".to_string(),
            _gfx: "achievement_hopebaby.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 427,
            _text: "You unlocked \'Glowing Baby\'".to_string(),
            _gfx: "achievement_glowingbaby.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 428,
            _text: "You unlocked \'Double Baby\'".to_string(),
            _gfx: "achievement_doublebaby.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 429,
            _text: "\'The Stairway\' has appeared in the basement".to_string(),
            _gfx: "achievement_thestairway.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 430,
            _text: "\'Red Stew\' has appeared in the basement".to_string(),
            _gfx: "achievement_redstew.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 431,
            _text: "\'Birthright\' has appeared in the basement".to_string(),
            _gfx: "achievement_birthright.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 432,
            _text: "\'Damocles\' has appeared in the basement".to_string(),
            _gfx: "achievement_damocles.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 433,
            _text: "\'Rock Bottom\' has appeared in the basement".to_string(),
            _gfx: "achievement_rockbottom.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 434,
            _text: "\'Inner Child\' has appeared in the basement".to_string(),
            _gfx: "achievement_innerchild.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 435,
            _text: "\'Vanishing Twin\' has appeared in the basement".to_string(),
            _gfx: "achievement_vanishingtwin.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 436,
            _text: "\'Genesis\' has appeared in the basement".to_string(),
            _gfx: "achievement_genesis.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 437,
            _text: "\'Suplex!\' has appeared in the basement".to_string(),
            _gfx: "achievement_suplex.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 438,
            _text: "You unlocked \'Solomon's Baby\'".to_string(),
            _gfx: "achievement_solomonsbaby.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 439,
            _text: "You unlocked \'Illusion Baby\'".to_string(),
            _gfx: "achievement_illusionbaby.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 440,
            _text: "\'Meat Cleaver\' has appeared in the basement".to_string(),
            _gfx: "achievement_meatcleaver.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 441,
            _text: "\'Options?\' has appeared in the basement".to_string(),
            _gfx: "achievement_options.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 442,
            _text: "\'Yuck Heart\' has appeared in the basement".to_string(),
            _gfx: "achievement_yuckheart.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 443,
            _text: "\'Candy Heart\' has appeared in the basement".to_string(),
            _gfx: "achievement_candyheart.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 444,
            _text: "\'Guppy's Eye\' has appeared in the basement".to_string(),
            _gfx: "achievement_guppyseye.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 445,
            _text: "\'A Pound of Flesh\' has appeared in the basement".to_string(),
            _gfx: "achievement_poundofflesh.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 446,
            _text: "\'Akeldama\' has appeared in the basement".to_string(),
            _gfx: "achievement_akeldama.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 447,
            _text: "\'Redemption\' has appeared in the basement".to_string(),
            _gfx: "achievement_redemption.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 448,
            _text: "\'Eternal D6\' has appeared in the basement".to_string(),
            _gfx: "achievement_eternald6.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 449,
            _text: "\'Montezuma's Revenge\' has appeared in the basement".to_string(),
            _gfx: "achievement_montezumasrevenge.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 450,
            _text: "\'Bird Cage\' has appeared in the basement".to_string(),
            _gfx: "achievement_birdcage.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 451,
            _text: "\'Cracked Orb\' has appeared in the basement".to_string(),
            _gfx: "achievement_crackedorb.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 452,
            _text: "\'Bloody Gust\' has appeared in the basement".to_string(),
            _gfx: "achievement_bloodygust.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 453,
            _text: "\'Empty Heart\' has appeared in the basement".to_string(),
            _gfx: "achievement_emptyheart.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 454,
            _text: "\'Devil's Crown\' has appeared in the basement".to_string(),
            _gfx: "achievement_devilscrown.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 455,
            _text: "\'Lil Abaddon\' has appeared in the basement".to_string(),
            _gfx: "achievement_lilabaddon.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 456,
            _text: "\'Tinytoma\' has appeared in the basement".to_string(),
            _gfx: "achievement_tinytoma.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 457,
            _text: "\'Astral Projection\' has appeared in the basement".to_string(),
            _gfx: "achievement_astralprojection.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 458,
            _text: "\''M\' has appeared in the basement".to_string(),
            _gfx: "achievement_m.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 459,
            _text: "\'Everything Jar\' has appeared in the basement".to_string(),
            _gfx: "achievement_everythingjar.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 460,
            _text: "\'Lost Soul\' has appeared in the basement".to_string(),
            _gfx: "achievement_lostsoul.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 461,
            _text: "\'Hungry Soul\' has appeared in the basement".to_string(),
            _gfx: "achievement_hungrysoul.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 462,
            _text: "\'Blood Puppy\' has appeared in the basement".to_string(),
            _gfx: "achievement_bloodpuppy.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 463,
            _text: "\'C Section\' has appeared in the basement".to_string(),
            _gfx: "achievement_csection.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 464,
            _text: "\'Keeper's Sack\' has appeared in the basement".to_string(),
            _gfx: "achievement_keeperssack.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 465,
            _text: "\'Keeper's Box\' has appeared in the basement".to_string(),
            _gfx: "achievement_keepersbox.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 466,
            _text: "\'Lil Portal\' has appeared in the basement".to_string(),
            _gfx: "achievement_lilportal.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 467,
            _text: "\'Worm Friend\' has appeared in the basement".to_string(),
            _gfx: "achievement_wormfriend.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 468,
            _text: "\'Bone Spurs\' has appeared in the basement".to_string(),
            _gfx: "achievement_bonespurs.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 469,
            _text: "\'Spirit Shackles\' has appeared in the basement".to_string(),
            _gfx: "achievement_spiritshackles.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 470,
            _text: "\'Revelation\' has appeared in the basement".to_string(),
            _gfx: "achievement_revelation.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 471,
            _text: "\'Jar of Wisps\' has appeared in the basement".to_string(),
            _gfx: "achievement_jarofwisps.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 472,
            _text: "\'Magic Skin\' has appeared in the basement".to_string(),
            _gfx: "achievement_magicskin.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 473,
            _text: "\'Friend Finder\' has appeared in the basement".to_string(),
            _gfx: "achievement_friendfinder.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 474,
            _text: "You unlocked \'Isaac\'".to_string(),
            _gfx: "achievement_isaacb.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 475,
            _text: "You unlocked \'Magdalene\'".to_string(),
            _gfx: "achievement_magdaleneb.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 476,
            _text: "You unlocked \'Cain\'".to_string(),
            _gfx: "achievement_cainb.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 477,
            _text: "You unlocked \'Judas\'".to_string(),
            _gfx: "achievement_judasb.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 478,
            _text: "You unlocked \'???\'".to_string(),
            _gfx: "achievement_bluebabyb.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 479,
            _text: "You unlocked \'Eve\'".to_string(),
            _gfx: "achievement_eveb.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 480,
            _text: "You unlocked \'Samson\'".to_string(),
            _gfx: "achievement_samsonb.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 481,
            _text: "You unlocked \'Azazel\'".to_string(),
            _gfx: "achievement_azazelb.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 482,
            _text: "You unlocked \'Lazarus\'".to_string(),
            _gfx: "achievement_lazarusb.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 483,
            _text: "You unlocked \'Eden\'".to_string(),
            _gfx: "achievement_edenb.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 484,
            _text: "You unlocked \'The Lost\'".to_string(),
            _gfx: "achievement_thelostb.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 485,
            _text: "You unlocked \'Lilith\'".to_string(),
            _gfx: "achievement_lilithb.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 486,
            _text: "You unlocked \'Keeper\'".to_string(),
            _gfx: "achievement_keeperb.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 487,
            _text: "You unlocked \'Apollyon\'".to_string(),
            _gfx: "achievement_apollyonb.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 488,
            _text: "You unlocked \'The Forgotten\'".to_string(),
            _gfx: "achievement_theforgottenb.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 489,
            _text: "You unlocked \'Bethany\'".to_string(),
            _gfx: "achievement_bethanyb.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 490,
            _text: "You unlocked \'Jacob\'".to_string(),
            _gfx: "achievement_jacobb.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 491,
            _text: "\'Glitched Crown\' has appeared in the basement".to_string(),
            _gfx: "achievement_glitchedcrown.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 492,
            _text: "\'Belly Jelly\' has appeared in the basement".to_string(),
            _gfx: "achievement_bellyjelly.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 493,
            _text: "\'Blue Key\' has appeared in the basement".to_string(),
            _gfx: "achievement_bluekey.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 494,
            _text: "\'Sanguine Bond\' has appeared in the basement".to_string(),
            _gfx: "achievement_sanguinebond.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 495,
            _text: "\'The Swarm\' has appeared in the basement".to_string(),
            _gfx: "achievement_theswarm.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 496,
            _text: "\'Heartbreak\' has appeared in the basement".to_string(),
            _gfx: "achievement_heartbreak.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 497,
            _text: "\'Larynx\' has appeared in the basement".to_string(),
            _gfx: "achievement_larynx.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 498,
            _text: "\'Azazel's Rage\' has appeared in the basement".to_string(),
            _gfx: "achievement_azazelsrage.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 499,
            _text: "\'Salvation\' has appeared in the basement".to_string(),
            _gfx: "achievement_salvation.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 500,
            _text: "\'TMTRAINER\' has appeared in the basement".to_string(),
            _gfx: "achievement_tmtrainer.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 501,
            _text: "\'Sacred Orb\' has appeared in the basement".to_string(),
            _gfx: "achievement_sacredorb.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 502,
            _text: "\'Twisted Pair\' has appeared in the basement".to_string(),
            _gfx: "achievement_twistedpair.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 503,
            _text: "\'Strawman\' has appeared in the basement".to_string(),
            _gfx: "achievement_strawman.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 504,
            _text: "\'Echo Chamber\' has appeared in the basement".to_string(),
            _gfx: "achievement_echochamber.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 505,
            _text: "\'Isaac's Tomb\' has appeared in the basement".to_string(),
            _gfx: "achievement_isaacstomb.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 506,
            _text: "\'Vengeful Spirit\' has appeared in the basement".to_string(),
            _gfx: "achievement_vengefulspirit.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 507,
            _text: "\'Esau Jr.\' has appeared in the basement".to_string(),
            _gfx: "achievement_esaujr.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 508,
            _text: "You unlocked Challenge #37 Bloody Mary".to_string(),
            _gfx: "achievement_challenge37.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 509,
            _text: "You unlocked Challenge #38 Baptism by Fire".to_string(),
            _gfx: "achievement_challenge38.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 510,
            _text: "You unlocked Challenge #39 Isaac's Awakening".to_string(),
            _gfx: "achievement_challenge39.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 511,
            _text: "You unlocked Challenge #40 Seeing Double".to_string(),
            _gfx: "achievement_challenge40.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 512,
            _text: "You unlocked Challenge #41 Pica Run".to_string(),
            _gfx: "achievement_challenge41.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 513,
            _text: "You unlocked Challenge #42 Hot Potato".to_string(),
            _gfx: "achievement_challenge42.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 514,
            _text: "You unlocked Challenge #43 Cantripped!".to_string(),
            _gfx: "achievement_challenge43.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 515,
            _text: "You unlocked Challenge #44".to_string(),
            _gfx: "achievement_challenge44.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 516,
            _text: "You unlocked Challenge #45".to_string(),
            _gfx: "achievement_challenge45.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 517,
            _text: "\'Dirty Mind\' has appeared in the basement".to_string(),
            _gfx: "achievement_dirtymind.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 518,
            _text: "\'Sigil of Baphomet\' has appeared in the basement".to_string(),
            _gfx: "achievement_sigilofbaphomet.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 519,
            _text: "\'Purgatory\' has appeared in the basement".to_string(),
            _gfx: "achievement_purgatory.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 520,
            _text: "\'Spirit Sword\' has appeared in the basement".to_string(),
            _gfx: "achievement_spiritsword.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 521,
            _text: "\'Broken Glasses\' has appeared in the basement".to_string(),
            _gfx: "achievement_brokenglasses.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 522,
            _text: "\'Ice Cube\' has appeared in the basement".to_string(),
            _gfx: "achievement_icecube.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 523,
            _text: " for a small fee...\'".to_string(),
            _gfx: "achievement_chargedpenny.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 524,
            _text: "\'The Fool\' has appeared in the basement".to_string(),
            _gfx: "achievement_reversefool.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 525,
            _text: "\'The Magician\' has appeared in the basement".to_string(),
            _gfx: "achievement_reversemagician.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 526,
            _text: "\'The High Priestess\' has appeared in the basement".to_string(),
            _gfx: "achievement_reversehighpriestess.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 527,
            _text: "\'The Empress\' has appeared in the basement".to_string(),
            _gfx: "achievement_reverseempress.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 528,
            _text: "\'The Emperor\' has appeared in the basement".to_string(),
            _gfx: "achievement_reverseemperor.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 529,
            _text: "\'The Hierophant\' has appeared in the basement".to_string(),
            _gfx: "achievement_reversehierophant.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 530,
            _text: "\'The Lovers\' has appeared in the basement".to_string(),
            _gfx: "achievement_reverselovers.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 531,
            _text: "\'The Chariot\' has appeared in the basement".to_string(),
            _gfx: "achievement_reversechariot.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 532,
            _text: "\'Justice\' has appeared in the basement".to_string(),
            _gfx: "achievement_reversejustice.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 533,
            _text: "\'The Hermit\' has appeared in the basement".to_string(),
            _gfx: "achievement_reversehermit.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 534,
            _text: "\'Wheel of Fortune\' has appeared in the basement".to_string(),
            _gfx: "achievement_reversewheeloffortune.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 535,
            _text: "\'Strength\' has appeared in the basement".to_string(),
            _gfx: "achievement_reversestrength.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 536,
            _text: "\'The Hanged Man\' has appeared in the basement".to_string(),
            _gfx: "achievement_reversehangedman.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 537,
            _text: "\'Death\' has appeared in the basement".to_string(),
            _gfx: "achievement_reversedeath.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 538,
            _text: "\'Temperance\' has appeared in the basement".to_string(),
            _gfx: "achievement_reversetemperance.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 539,
            _text: "\'The Devil\' has appeared in the basement".to_string(),
            _gfx: "achievement_reversedevil.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 540,
            _text: "\'The Tower\' has appeared in the basement".to_string(),
            _gfx: "achievement_reversetower.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 541,
            _text: "\'The Stars\' has appeared in the basement".to_string(),
            _gfx: "achievement_reversestars.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 542,
            _text: "\'The Sun & The Moon\' has appeared in the basement".to_string(),
            _gfx: "achievement_reversesunandmoon.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 543,
            _text: "\'Judgement\' has appeared in the basement".to_string(),
            _gfx: "achievement_reversejudgement.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 544,
            _text: "\'The World\' has appeared in the basement".to_string(),
            _gfx: "achievement_reverseworld.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 545,
            _text: "\'Old Capacitor\' has appeared in the basement".to_string(),
            _gfx: "achievement_oldcapacitor.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 546,
            _text: "\'Brimstone Bombs\' has appeared in the basement".to_string(),
            _gfx: "achievement_brimstonebombs.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 547,
            _text: "\'Mega Mush\' has appeared in the basement".to_string(),
            _gfx: "achievement_megamush.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 548,
            _text: "\'Mom's Lock\' has appeared in the basement".to_string(),
            _gfx: "achievement_momslock.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 549,
            _text: "\'Dice Bag\' has appeared in the basement".to_string(),
            _gfx: "achievement_dicebag.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 550,
            _text: "\'Holy Crown\' has appeared in the basement".to_string(),
            _gfx: "achievement_holycrown.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 551,
            _text: "\'Mother's Kiss\' has appeared in the basement".to_string(),
            _gfx: "achievement_motherskiss.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 552,
            _text: "\'Gilded Key\' has appeared in the basement".to_string(),
            _gfx: "achievement_gildedkey.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 553,
            _text: "\'Lucky Sack\' has appeared in the basement".to_string(),
            _gfx: "achievement_luckysack.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 554,
            _text: "\'Your Soul\' has appeared in the basement".to_string(),
            _gfx: "achievement_yoursoul.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 555,
            _text: "\'Number Magnet\' has appeared in the basement".to_string(),
            _gfx: "achievement_numbermagnet.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 556,
            _text: "\'Dingle Berry\' has appeared in the basement".to_string(),
            _gfx: "achievement_dingleberry.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 557,
            _text: "\'Ring Cap\' has appeared in the basement".to_string(),
            _gfx: "achievement_ringcap.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 558,
            _text: "\'Strange Key\' has appeared in the basement".to_string(),
            _gfx: "achievement_strangekey.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 559,
            _text: "\'Lil Clot\' has appeared in the basement".to_string(),
            _gfx: "achievement_lilclot.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 560,
            _text: "\'Temporary Tattoo\' has appeared in the basement".to_string(),
            _gfx: "achievement_temporarytattoo.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 561,
            _text: "\'Swallowed M80\' has appeared in the basement".to_string(),
            _gfx: "achievement_swallowedm80.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 562,
            _text: "\'Wicked Crown\' has appeared in the basement".to_string(),
            _gfx: "achievement_wickedcrown.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 563,
            _text: "\'Azazel's Stump\' has appeared in the basement".to_string(),
            _gfx: "achievement_azazelsstump.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 564,
            _text: "\'Torn Pocket\' has appeared in the basement".to_string(),
            _gfx: "achievement_tornpocket.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 565,
            _text: "\'Torn Card\' has appeared in the basement".to_string(),
            _gfx: "achievement_torncard.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 566,
            _text: "\'Nuh Uh!\' has appeared in the basement".to_string(),
            _gfx: "achievement_nuhuh.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 567,
            _text: "\'Modeling Clay\' has appeared in the basement".to_string(),
            _gfx: "achievement_modelingclay.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 568,
            _text: "\'Kid's Drawing\' has appeared in the basement".to_string(),
            _gfx: "achievement_kidsdrawing.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 569,
            _text: "\'Crystal Key\' has appeared in the basement".to_string(),
            _gfx: "achievement_crystalkey.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 570,
            _text: "\'The Twins\' has appeared in the basement".to_string(),
            _gfx: "achievement_thetwins.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 571,
            _text: "\'Adoption Papers\' has appeared in the basement".to_string(),
            _gfx: "achievement_adoptionpapers.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 572,
            _text: "\'Keeper's Bargain\' has appeared in the basement".to_string(),
            _gfx: "achievement_keepersbargain.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 573,
            _text: "\'Cursed Penny\' has appeared in the basement".to_string(),
            _gfx: "achievement_cursedpenny.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 574,
            _text: "\'Cricket Leg\' has appeared in the basement".to_string(),
            _gfx: "achievement_cricketleg.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 575,
            _text: "\'Apollyon's Best Friend\' has appeared in the basement".to_string(),
            _gfx: "achievement_apollyonsbestfriend.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 576,
            _text: "\'Polished Bone\' has appeared in the basement".to_string(),
            _gfx: "achievement_polishedbone.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 577,
            _text: "\'Hollow Heart\' has appeared in the basement".to_string(),
            _gfx: "achievement_hollowheart.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 578,
            _text: "\'Expansion Pack\' has appeared in the basement".to_string(),
            _gfx: "achievement_expansionpack.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 579,
            _text: "\'Beth's Essence\' has appeared in the basement".to_string(),
            _gfx: "achievement_bethsessence.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 580,
            _text: "\'RC Remote\' has appeared in the basement".to_string(),
            _gfx: "achievement_rcremote.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 581,
            _text: "\'Found Soul\' has appeared in the basement".to_string(),
            _gfx: "achievement_foundsoul.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 582,
            _text: "\'Member Card\' has appeared in the basement".to_string(),
            _gfx: "achievement_membercard.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 583,
            _text: "\'Golden Razor\' has appeared in the basement".to_string(),
            _gfx: "achievement_goldenrazor.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 584,
            _text: "\'Spindown Dice\' has appeared in the basement".to_string(),
            _gfx: "achievement_spindowndice.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 585,
            _text: "\'Hypercoagulation\' has appeared in the basement".to_string(),
            _gfx: "achievement_hypercoagulation.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 586,
            _text: "\'Bag of Crafting\' has appeared in the basement".to_string(),
            _gfx: "achievement_bagofcrafting.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 587,
            _text: "\'Dark Arts\' has appeared in the basement".to_string(),
            _gfx: "achievement_darkarts.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 588,
            _text: "\'IBS\' has appeared in the basement".to_string(),
            _gfx: "achievement_ibs.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 589,
            _text: "\'Sumptorium\' has appeared in the basement".to_string(),
            _gfx: "achievement_sumptorium.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 590,
            _text: "\'Berserk!\' has appeared in the basement".to_string(),
            _gfx: "achievement_berserk.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 591,
            _text: "\'Hemoptysis\' has appeared in the basement".to_string(),
            _gfx: "achievement_hemoptysis.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 592,
            _text: "\'Flip\' has appeared in the basement".to_string(),
            _gfx: "achievement_flip.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 593,
            _text: "\'Corrupted Data\' has appeared in the basement".to_string(),
            _gfx: "achievement_corrupteddata.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 594,
            _text: "\'Ghost Bombs\' has appeared in the basement".to_string(),
            _gfx: "achievement_ghostbombs.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 595,
            _text: "\'Gello\' has appeared in the basement".to_string(),
            _gfx: "achievement_gello.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 596,
            _text: "\'Keeper's Kin\' has appeared in the basement".to_string(),
            _gfx: "achievement_keeperskin.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 597,
            _text: "\'Abyss\' has appeared in the basement".to_string(),
            _gfx: "achievement_abyss.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 598,
            _text: "\'Decap Attack\' has appeared in the basement".to_string(),
            _gfx: "achievement_decapattack.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 599,
            _text: "\'Lemegeton\' has appeared in the basement".to_string(),
            _gfx: "achievement_lemegeton.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 600,
            _text: "\'Anima Sola\' has appeared in the basement".to_string(),
            _gfx: "achievement_animasola.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 601,
            _text: "\'Mega Chest\' has appeared in the basement".to_string(),
            _gfx: "achievement_megachest.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 602,
            _text: "\'Queen of Hearts\' has appeared in the basement".to_string(),
            _gfx: "achievement_queenofhearts.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 603,
            _text: "\'Gold Pill\' has appeared in the basement".to_string(),
            _gfx: "achievement_goldpill.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 604,
            _text: "\'Black Sack\' has appeared in the basement".to_string(),
            _gfx: "achievement_blacksack.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 605,
            _text: "\'Charming Poop\' has appeared in the basement".to_string(),
            _gfx: "achievement_charmingpoop.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 606,
            _text: "\'Horse Pill\' has appeared in the basement".to_string(),
            _gfx: "achievement_horsepill.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 607,
            _text: "\'Crane Game\' has appeared in the basement".to_string(),
            _gfx: "achievement_cranegame.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 608,
            _text: "\'Hell Game\' has appeared in the basement".to_string(),
            _gfx: "achievement_hellgame.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 609,
            _text: "\'Wooden Chest\' has appeared in the basement".to_string(),
            _gfx: "achievement_woodenchest.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 610,
            _text: "\'Wild Card\' has appeared in the basement".to_string(),
            _gfx: "achievement_wildcard.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 611,
            _text: "\'Haunted Chest\' has appeared in the basement".to_string(),
            _gfx: "achievement_hauntedchest.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 612,
            _text: "\'Fool's Gold\' has appeared in the basement".to_string(),
            _gfx: "achievement_foolsgold.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 613,
            _text: "\'Golden Penny\' has appeared in the basement".to_string(),
            _gfx: "achievement_goldenpenny.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 614,
            _text: "\'Rotten Beggar\' has appeared in the basement".to_string(),
            _gfx: "achievement_rottenbeggar.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 615,
            _text: "\'Golden Battery\' has appeared in the basement".to_string(),
            _gfx: "achievement_goldenbattery.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 616,
            _text: "\'Confessional\' has appeared in the basement".to_string(),
            _gfx: "achievement_confessional.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 617,
            _text: "\'Golden Trinket\' has appeared in the basement".to_string(),
            _gfx: "achievement_goldentrinket.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 618,
            _text: "\'Soul of Isaac\' has appeared in the basement".to_string(),
            _gfx: "achievement_soulofisaac.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 619,
            _text: "\'Soul of Magdalene\' has appeared in the basement".to_string(),
            _gfx: "achievement_soulofmagdalene.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 620,
            _text: "\'Soul of Cain\' has appeared in the basement".to_string(),
            _gfx: "achievement_soulofcain.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 621,
            _text: "\'Soul of Judas\' has appeared in the basement".to_string(),
            _gfx: "achievement_soulofjudas.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 622,
            _text: "\'Soul of ???\' has appeared in the basement".to_string(),
            _gfx: "achievement_soulofbluebaby.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 623,
            _text: "\'Soul of Eve\' has appeared in the basement".to_string(),
            _gfx: "achievement_soulofeve.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 624,
            _text: "\'Soul of Samson\' has appeared in the basement".to_string(),
            _gfx: "achievement_soulofsamson.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 625,
            _text: "\'Soul of Azazel\' has appeared in the basement".to_string(),
            _gfx: "achievement_soulofazazel.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 626,
            _text: "\'Soul of Lazarus\' has appeared in the basement".to_string(),
            _gfx: "achievement_souloflazarus.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 627,
            _text: "\'Soul of Eden\' has appeared in the basement".to_string(),
            _gfx: "achievement_soulofeden.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 628,
            _text: "\'Soul of the Lost\' has appeared in the basement".to_string(),
            _gfx: "achievement_soulofthelost.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 629,
            _text: "\'Soul of Lilith\' has appeared in the basement".to_string(),
            _gfx: "achievement_souloflilith.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 630,
            _text: "\'Soul of the Keeper\' has appeared in the basement".to_string(),
            _gfx: "achievement_soulofthekeeper.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 631,
            _text: "\'Soul of Apollyon\' has appeared in the basement".to_string(),
            _gfx: "achievement_soulofapollyon.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 632,
            _text: "\'Soul of the Forgotten\' has appeared in the basement".to_string(),
            _gfx: "achievement_souloftheforgotten.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 633,
            _text: "\'Soul of Bethany\' has appeared in the basement".to_string(),
            _gfx: "achievement_soulofbethany.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 634,
            _text: "\'Soul of Jacob\' has appeared in the basement".to_string(),
            _gfx: "achievement_soulofjacob.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 635,
            _text: "\'A Strange Door\' has appeared in the depths".to_string(),
            _gfx: "achievement_strangedoor.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 636,
            _text: "\'Death Certificate\' has appeared in the basement".to_string(),
            _gfx: "achievement_deathcertificate.png".to_string(),
        });
        achievements.push(AchievementImg{
            _id: 637,
            _text: "Dead God!".to_string(),
            _gfx: "achievement_deadgod.png".to_string(),
        });


        log::info!("achievements image data successfully");

        achievements
    }

}

