use common::models::transports::Departure;
use yew::{html, Html, Component, Context, Properties};

pub struct Timing;

#[derive(Properties)]
pub struct Props {
    pub departures: Vec<Departure>
}

// Implement partial equality for Props
impl PartialEq for Props {
    fn eq(&self, other: &Self) -> bool {
        self.departures.len() == other.departures.len() && self.departures.iter().zip(other.departures.iter()).all(|(a, b)| a.display == b.display)
    }
}

impl Component for Timing {
    type Message = ();
    type Properties = Props;

    fn create(_ctx: &Context<Self>) -> Self {
        Timing
    }

    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
        false
    }

    fn view(&self, ctx: &Context<Self>) -> yew::Html {
        let mut transport_modes: Vec<String> = Vec::new();
        for departure in &ctx.props().departures {
            if !transport_modes.contains(&departure.line.transport_mode) {
                transport_modes.push(departure.line.transport_mode.clone());
            }
        }

        html! {
            { transport_modes.iter().map(|mode| {
                let departures = &ctx.props().departures.iter().filter(|dep| &dep.line.transport_mode == mode).collect::<Vec<&Departure>>();
                
                html! {
                    <div>
                        <h4>{ mode }</h4>
                        { departures.iter().map(|departure| {
                            let dep_display = format!("{} - {} - {}", departure.line.id, departure.destination, departure.display);
                            html! {
                                <div>
                                    { dep_display }
                                </div>
                            }
                        }).collect::<Html>() }
                    </div>
                }
            }).collect::<Html>() }
        }
    }
}