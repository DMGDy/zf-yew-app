use yew::prelude::*;
use gloo::net::http::Request;
use gloo_timers::callback::Interval;
use wasm_bindgen_futures::spawn_local;
use std::error::Error;

use crate::{states::State,
    device::{Device, TestData}
};

pub mod states;
pub mod device;

const SERVER: &str = "http://172.20.10.7:8080";
const SERVER_IS_UP: &str = "http://172.20.10.7:8080/up";
const SERVER_RESULT: &str = "http://172.20.10.7:8080/result";

async
fn get_test_data(dev: &str) ->Result<(), Box<dyn Error>> {
    let server_path = format!("{SERVER}/data/{dev}-test.csv");
    let result = Request::get(&server_path)
        .send()
        .await;
    match result {
        Ok(_) => {},
        Err(e) => {
            gloo::console::error!(
                format!("Error fetching file: {e}"));
        }
    }

    Ok(())
}

async 
fn server_get(path: &str) -> Result<State, Box<dyn Error>> {
    let response = Request::get(path)
        .send()
        .await;

    match response?.json::<State>().await {
        Ok(state) => Ok(state),
        Err(_) => Ok(State::EUnknown)
    } 
}
                                
pub enum Msg {
    NoOp,
    CheckServerUp,
    ShowDevices,
    UpdateChosenDevice(Device),
    UpdateStringPot(bool),
    StartTest,
    DownloadFile(Device),
    UpdateStatus(State),
}

pub struct App {
    chosen_dev: Device,
    show_devices: bool,
    bst_chosen: bool,
    use_str_pot: bool,
    test_data: TestData,
    state: State,
    finished: bool,
    _delta: Option<Interval>,
}

impl Component for App{
    type Message = Msg; 
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self { 
        let link = ctx.link().clone();
        let delta = Interval::
            new(1000, move || link.send_message(Msg::CheckServerUp));

        Self {
            test_data: TestData::default(), 
            chosen_dev: Device::None,
            bst_chosen: false,
            use_str_pot: false,
            show_devices: false,
            state: State::Offline,
            finished: false,
            _delta: Some(delta),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::NoOp => false,
            Msg::CheckServerUp => {
                let current_state = self.state.clone();
                ctx.link().send_future(async move {
                    let state = server_get(SERVER_IS_UP)
                        .await
                        .unwrap_or(State::Offline);

                    match state {
                        State::Online => {
                            match current_state {
                                State::Offline => { Msg::UpdateStatus(state)},
                                _ => { Msg::NoOp }
                            }
                        }
                        State::Offline => { Msg::UpdateStatus(state) },
                        _ => {Msg::NoOp},
                    }
                });
                false
            },

            Msg::ShowDevices => {
                self.show_devices = !self.show_devices;
                true
            },


            Msg::UpdateChosenDevice(device) => {
                self.chosen_dev = device;
                self.test_data.device = self.chosen_dev.to_string();
                if self.chosen_dev == Device::BST {
                    self.bst_chosen = true;
                } else {
                    self.bst_chosen = false
                }
                true
            },

            Msg::UpdateStringPot(check) => {
                self.use_str_pot = check;
                self.test_data.check = self.use_str_pot;
                true
            },

            Msg::StartTest => {
                self.test(&ctx);
                false
            },
            
            Msg::DownloadFile(dev) => {
                spawn_local(async move {
                    let _ = get_test_data(dev.abbrev()).await;
                });
                false
            }

            Msg::UpdateStatus(state) => {
                self.state = state;
                match self.state {
                    State::InProgress => {self.test(&ctx);},
                    State::Pass => {self.finished = true;}
                    State::Fail => {self.finished = true;}
                    _ => {}
                };
                true
            },
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();
        let check = self.use_str_pot.clone();
        let finished = self.finished.clone();

        html! {
            <div align="center">
            <h1> {"ZF Device Test Web Application"} </h1>
                <p>
                    <strong>{"Chosen Device: "}</strong>{self.chosen_dev.clone()}<br/>
                    <button onclick={link.callback(|_| Msg::ShowDevices)}> {"Show Devices:"}</button>
                </p>
                    if self.show_devices {
                        <div>
                            {self.show_devices(&ctx)}
                            if self.bst_chosen {
                                <input 
                                    type="checkbox"
                                    checked={check}
                                    onchange={link.callback(move |_|Msg::UpdateStringPot(!check))}
                                />
                                {" Use String Potentiometer"}
                            }
                        </div>
                    }
                <div style="margin:25px">
                    <strong>{"Server State: "}</strong>{self.state.code()}<br/>
                    {self.state.message()}
                </div>
                    <button onclick={link.callback(|_| Msg::StartTest)}> {"Start Test"}</button>

                <div style="margin:20px">
                    if finished {
                        <button onclick={
                            let dev = self.chosen_dev.clone();
                            link.callback(move |_| Msg::DownloadFile(dev.clone()))}>
                            {"Download Results in CSV"}</button>
                    }
                </div>
            </div>

        }
    }
}

impl App {
    fn test(&self, ctx: &Context<Self>) {
        let status = self.state.clone();
        match status {
            /*-------------------------------------------------------*/
            // from Idle to starting
            /*-------------------------------------------------------*/
            State::Online => {
                let test = self.test_data.clone();
                let link = ctx.link().clone();
                spawn_local(async move {
                    let response = Request::post(SERVER)
                        .json(&test)
                        .unwrap()
                        .send()
                        .await;
                    let new_status = match response
                        .unwrap()
                        .json::<State>()
                        .await{
                        Ok(code) => { code }
                        Err(_) => { State::EUnknown }
                    };
                    link.send_message(Msg::UpdateStatus(new_status));
                });
            },
            /*-------------------------------------------------------*/
            // server is awake, M4 is running firmware
            /*-------------------------------------------------------*/
            State::InProgress => {
                let link = ctx.link().clone();
                spawn_local(async  move {
                    let result = match server_get(SERVER_RESULT).await {
                        Ok(State::InProgress) => {State::InProgress},
                        Ok(State::Pass) => { State::Pass },
                        Ok(State::Fail) => { State::Fail },
                        Err(_) => {State::EUnknown},
                        _ => {State::EUnknown},
                    };
                    link.send_message(Msg::UpdateStatus(result));
                });

            }

            _ => {}
        }
    }

    fn show_devices(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();
        html! {
            <>
                <label style="cursor: pointer;" 
                onclick={link.callback(|_| 
                    Msg::UpdateChosenDevice(Device::BST))}> 
                    <strong>{"Brake Signal Transmitter"}</strong>
                </label>

                <br/>

                <label style="cursor: pointer;"
                onclick={link.callback(|_|
                    Msg::UpdateChosenDevice(Device::CWS))}> 
                    <strong>{"Continuous Wear Sensor"}</strong>
                </label>

                <br/>

                <label style="cursor: pointer;"
                onclick={link.callback(|_|
                    Msg::UpdateChosenDevice(Device::PrS))}> 
                    <strong>{"Pressure Sensor"}</strong>
                </label>

                <br/>

                <label style="cursor: pointer;"
                onclick={link.callback(|_|
                    Msg::UpdateChosenDevice(Device::ESCM))}> 
                    <strong>{"Electronic Stability Control Module"}</strong>
                </label>

                <br/>
            </>
        }
    }
}

#[function_component]
fn MainApp() -> Html {
    html! { <App/> }
}

fn main() {
    yew::Renderer::<MainApp>::new().render();
}
