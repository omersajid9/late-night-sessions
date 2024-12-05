use uuid::Uuid;
use yew_router::prelude::*;
use yew::prelude::*;

use crate::editnote::EditNote;
use crate::notes::Notes;
use crate::products::Products;
use crate::form::Form;


#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Notes,
    #[at("/notes")]
    Home,
    #[at("/addproduct")]
    AddProduct,
    #[at("/about")]
    About,
    #[at("/edit_note/:note_id")]
    EditNote { note_id: Uuid },
    #[not_found]
    #[at("/404")]
    NotFound,
}
pub fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! { <Products /> },
        Route::AddProduct => html! { <Form />},
        Route::Notes => html!{ <Notes /> },
        Route::EditNote {note_id} => html!{ <EditNote note_id={note_id}/> },
        Route::About => html! { <h1>{ "About us " }</h1>},
        Route::NotFound => html! { <h1>{ "404" }</h1> },
    }
}