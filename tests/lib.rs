extern crate libtextcat;

use libtextcat::Textcat;

#[test]
fn test_load_handle() {
	let textcat = Textcat::new();
}

#[test]
fn test_language_english_short() {
	let textcat = Textcat::new();
	let s = "Knock Knock! Who's there?";
	let result = textcat.get_language(s);
	assert_eq!("[english]", result);
}

#[test]
fn test_language_english_long() {
	let textcat = Textcat::new();
	let s = "The quick brown fox jumps over the lazy dog";
	let result = textcat.get_language(s);
	assert_eq!("[english]", result);
}

#[test]
fn test_language_spanish() {
	let textcat = Textcat::new();
	let m = " Los puntos de 
acceso inalámbricos son relativamente poco co
stosos y se implementan fácilmente. Un 
equipo de asesores bienintencionado que tr
abaja en una sala de
 conferencias podría 
instalar un punto de acceso inalámbrico para 
compartir un solo puerto cableado en la sala. 
Un 
hacker
 malicioso puede sentarse en un café c
on un ordenador portátil habilitado para 
uso inalámbrico buscando tráfico no cifrado o cifrado con WEP. En ambos casos, se 
presentan riesgos inaceptables. Independiente
mente de si hay un intento malicioso, la 
introducción de 
hardware
 no autorizado puede comprometer la confidencialidad e 
integridad del tráfico de red. Los dispositivos inalámbricos no autorizados pueden 
detectarse al examinar físicamente las in
stalaciones (práctica 
conocida como “guerra 
móvil”), al utilizar escáneres de radiofrecuen
cia (RF) para determinar la ubicación de los 
dispositivos inalámbricos o al usar sistemas di
señados para analizar el
 tráfico de red para 
detectar dispositivos no autorizados. ";
	let result = textcat.get_language(m);
	assert_eq!("[spanish]", result);
}

#[test]
fn test_language_romanian() {
	let textcat = Textcat::new();
	let m = "Fie matricea A. Calculati minorul elementului 3!";
	let result = textcat.get_language(m);
	assert_eq!("[romanian]", result);
}
