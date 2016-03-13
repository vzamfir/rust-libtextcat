extern crate libtextcat;

use libtextcat::Textcat;

#[test]
/// Test Textcat initialization.
fn test_load_handle() {
	let textcat = Textcat::new();
}

#[test]
/// Test language detection for random English text.
fn test_language_english_short() {
	let textcat = Textcat::new();
	let text = "The pen is mightier than the sword.";
	let result = textcat.get_language(text);
	assert_eq!(result, ["english"]);
}

#[test]
/// Test language detection for random (longer) English text.
fn test_language_english_long() {
	let textcat = Textcat::new();
	let text = "Game of as rest time eyes with of this it. 
		Add was music merry any truth since going. 
		Happiness she ham but instantly put departure propriety. 
		She amiable all without say spirits shy clothes morning. 
		Frankness in extensive to belonging improving so certainty. 
		Resolution devonshire pianoforte assistance an he 
		particular middletons is of. Explain ten man uncivil engaged 
		conduct. Am likewise betrayed as declared absolute do. Taste 
		oh spoke about no solid of hills up shade. Occasion so bachelor 
		humoured striking by attended doubtful be it. 
		Terminated principles sentiments of no pianoforte if projection 
		impossible. Horses pulled nature favour number yet highly his has 
		old. Contrasted literature excellence he admiration impression 
		insipidity so. Scale ought who terms after own quick since. 
		Servants margaret husbands to screened in throwing. Imprudence 
		oh an collecting partiality. Admiration gay difficulty unaffected how. ";
	let result = textcat.get_language(text);
	assert_eq!(result, ["english"]);
}

#[test]
/// Test language detection for random Spanish text.
fn test_language_spanish() {
	let textcat = Textcat::new();
	let m = " Los puntos de 
		acceso inalámbricos son relativamente poco 
		costosos y se implementan fácilmente.";
	let result = textcat.get_language(m);
	assert_eq!(result, ["spanish", "portuguese"]);
}

#[test]
/// Test language detection for random Romanian text.
fn test_language_romanian() {
	let textcat = Textcat::new();
	let text = "Fie matricea A. Calculati minorul elementului 3!";
	let result = textcat.get_language(text);
	assert_eq!(result, ["romanian"]);
}
