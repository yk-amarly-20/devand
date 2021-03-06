use crate::app::components::AffinitiesTable;
use crate::app::components::{Alert, BusyIndicator};
use crate::app::workers::{main_worker, main_worker::MainWorker};
use crate::app::{AppRoute, RouterAnchor};
use devand_core::UserAffinity;
use devand_text::Text;
use yew::prelude::*;
use yewtil::NeqAssign;

#[derive(Default)]
pub struct State {
    affinities: Option<Vec<UserAffinity>>,
}

pub enum Msg {
    MainWorkerRes(main_worker::Response),
}

pub struct AffinitiesPage {
    props: Props,
    state: State,
    _main_worker: Box<dyn Bridge<MainWorker>>,
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {}

impl Component for AffinitiesPage {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let state = State::default();

        let mut main_worker = MainWorker::bridge(link.callback(Msg::MainWorkerRes));
        main_worker.send(main_worker::Request::LoadAffinities);

        Self {
            props,
            state,
            _main_worker: main_worker,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::MainWorkerRes(res) => {
                use main_worker::Response;

                match res {
                    Response::AffinitiesFetched(mut affinities) => {
                        affinities.sort_unstable_by_key(|x| x.affinity);
                        self.state.affinities = Some(affinities);
                        true
                    }

                    _ => false,
                }
            }
        }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props.neq_assign(props)
    }

    fn view(&self) -> Html {
        html! {
        <>
        <h1>{ Text::Affinities }</h1>
        {
            if let Some(affinities) = &self.state.affinities {
                view_affinities(affinities)
            } else {
            html! { <BusyIndicator /> }
            }
        }
        </>
        }
    }
}

fn view_affinities(affinities: &[UserAffinity]) -> Html {
    if affinities.is_empty() {
        view_no_affinities()
    } else {
        // TODO [optimization] Avoid cloning
        let affinities = Vec::from(affinities);
        html! {
        <>
            <p>{ Text::AffinitiesTableDescription }</p>
            <AffinitiesTable affinities=affinities />
        </>
        }
    }
}

fn view_no_affinities() -> Html {
    html! {
    <Alert>
        { Text::NoMatchingUsersFound } <RouterAnchor route=AppRoute::Settings >{ Text::ExtendYourLanguageSelection }</RouterAnchor>
    </Alert>
    }
}
