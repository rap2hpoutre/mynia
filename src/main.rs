extern crate rand;

use rand::Rng;

fn main() {
    let x = [
        "Abi", "Ai", "Aba", "Agi", "Ava", "Cami", "Centi", 
        "Cogi", "Demi", "Diva", "Dyna", "Ea", "Ei", "Fa", 
        "Fi", "Ge", "Gi", "Gle", "Ja", "Je", "I", "Io", "Ka", 
        "Kay", "Ki", "Kwi", "La", "Lee", "Mee", "Mi", "Mu", 
        "My", "Oo", "O", "Oyo", "Pixo", "Pla", "Ple", "Qua", 
        "Qui", "Roo", "Rhy", "Ska", "Sky", "Ski", "Ta", 
        "Tri", "Twi", "Tru", "Vi", "Voo", "Wa", "Wiki", "Ya", 
        "Yaki", "Yo", "Za", "Zoo"
    ];
    let y = [
        "ba", "bi", "ble", "blee", "boo", "box", "by", "car", 
        "cero", "deo", "del", "do", "doo", "gen", "go", "jo", 
        "kwi", "lane", "lia", "lith",  "loo", "lium", "mba", 
        "mbee", "mbo", "mbu", "mia", "mm", "nder", "ndo", 
        "ndu", "noodle", "nix", "nte", "nti", "nu", "nyx", 
        "pe", "re", "ta", "tri", "tz", "va", "vee", "veo", 
        "via", "vu", "wee", "wi", "xo", "yo", "zz", "zzy", 
        "zio", "zoo", "zu", "zix"
    ];

    println!(
        "Your unicorn name will be: {}{}", 
        x[rand::thread_rng().gen_range(0, x.len())], 
        y[rand::thread_rng().gen_range(0, y.len())], 
    );
}