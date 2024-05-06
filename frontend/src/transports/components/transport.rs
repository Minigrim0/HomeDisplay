use common::models::transports::{Site, Departure};
use std::collections::HashMap;
use yew::{html, Html, Component, Context};
use futures::stream::StreamExt;
use chrono::prelude::Local;
use gloo_console::log;

use crate::transports::components::timing::Timing;

use super::super::services::{fetch_sites, fetch_departures, stream_time};

pub struct TransportsComponent {
    sites: Vec<Site>,
    departures: HashMap<String, Vec<Departure>>,
    site_errors: HashMap<String, String>,
    last_update: i64,
    time_since_last_update: i64,
    loading: bool,
    refreshing: bool,
    error: Option<String>,
}

pub enum Msg {
    ClockUpdate,
    LoadSitessData,
    LoadAllDepartures,
    SitesDataReceived(Result<Vec<Site>, String>),
    LoadDepartures(String),
    DeparturesDataReceived(Result<(String, Vec<Departure>), (String, String)>),
}

impl Component for TransportsComponent {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        ctx.link().send_message(Msg::LoadSitessData);

        // Clock update, used to update the time since last update
        let time_stream = stream_time();
        ctx.link().send_stream(time_stream.map(|_| Msg::ClockUpdate));

        TransportsComponent {
            sites: Vec::new(),
            departures: HashMap::new(),
            site_errors: HashMap::new(),
            last_update: Local::now().timestamp(),
            time_since_last_update: 0,
            loading: true,
            refreshing: false,
            error: None,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::ClockUpdate => {  // Update timestamps
                let now = Local::now().timestamp();
                self.time_since_last_update = now - self.last_update;
                if self.time_since_last_update > 60 {
                    ctx.link().send_message(Msg::LoadAllDepartures);
                }
                true
            }
            Msg::LoadSitessData => {  // Call the API to load the sites
                self.loading = true;
                self.error = None;
                fetch_sites(ctx.link().callback(Msg::SitesDataReceived));
                true
            }
            Msg::SitesDataReceived(result) => {  // Handle the sites data
                match result {
                    Ok(value) => {
                        self.error = None;
                        self.sites = value.clone();
                    }
                    Err(e) => {
                        self.error = Some(e);
                    }
                }
                self.loading = false;
                // Load the departures for each site
                true
            }
            Msg::LoadDepartures(site_id) => {  // Call the API to load the departures
                self.refreshing = true;
                self.error = None;
                fetch_departures(site_id, ctx.link().callback(Msg::DeparturesDataReceived));
                true
            }
            Msg::DeparturesDataReceived(result) => {  // Handle the departures data
                match result {
                    Ok((site_id, departures)) => {
                        log!("Loaded departures for site {}", &site_id);
                        self.error = None;
                        if let Some(_) = self.site_errors.remove(&site_id) {
                            log!("Site {} is now working again", &site_id);
                        }
                        self.departures.insert(site_id, departures);
                    }
                    Err((site_id, error)) => {
                        log!("Error loading departures for site {}: {}", &site_id, &error);
                        self.site_errors.insert(site_id, error);
                    }
                }
                self.refreshing = false;
                true
            }
            Msg::LoadAllDepartures => {  // Load all departures
                for site in &self.sites {
                    ctx.link().send_message(Msg::LoadDepartures(site.id.clone()));
                }
                self.last_update = Local::now().timestamp();
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let panel_title = html! {
            <h3 class="panel-title">
                { "🚂 Departures 🚂" }
                <button class="link-button" onclick={ctx.link().callback(|_| Msg::LoadAllDepartures)}>{ "🔁" }</button>
            </h3>
        };

        if let Some(error) = &self.error {
            html! {
                <div class="panel panel-div">
                    { panel_title }
                    <p style="color: red">{{ error }}</p>
                </div>
            }
        } else if self.loading {
            html! {
                <div class="panel panel-div">
                    { panel_title }
                    <div v-if="loading" class="ring">
                        <div class="ball-holder">
                            <div class="ball"></div>
                        </div>
                    </div>
                </div>
            }
        } else {
            let last_update = if self.refreshing {
                format!("{} seconds ago. (refreshing...)", self.time_since_last_update)
            } else {
                format!("{} seconds ago.", self.time_since_last_update)
            };

            html! {
                <div class="panel panel-div">
                    { panel_title }
                    { self.sites.iter().map(|site| {
                        let site_name = &site.name;
                        if let Some(error) = self.site_errors.get(&site.id) {
                            html! {
                                <div>
                                    <h3>{ site_name }</h3>
                                    <div style="color: red">{ error }</div>
                                </div>
                            }
                        } else if let Some(departures) = self.departures.get(&site.id) {
                            html! {
                                <div>
                                    <h3>{ site_name }</h3>
                                    <Timing departures={departures.clone()} />
                                </div>
                            }
                        } else {
                            html! {
                                <div>
                                    <h3>{ site_name }</h3>
                                    <div>{"Loading..."}</div>
                                </div>
                            }
                        }
                    }).collect::<Html>() }
                    <small>{ last_update }</small>
                </div>
            }
        }
    }
}