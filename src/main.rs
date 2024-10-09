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

async fn server_status() -> Result<State, Box<dyn Error>> {
    let response = Request::get(SERVER_IS_UP)
        .send()
        .await;

    match response?.json::<State>().await {
        Ok(state) => Ok(state),
        Err(_) => Ok(State::EUnknown)
    } 
}
                                
pub enum Msg {
    CheckServerUp,
    ShowDevices,
    UpdateChosenDevice(Device),
    UpdateStringPot(bool),
    StartTest,
    UpdateStatus(State),
}
pub struct App {
    chosen_dev: Device,
    show_devices: bool,
    bst_chosen: bool,
    use_str_pot: bool,
    test_data: TestData,
    state: State,
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
            state: State::Idle,
            _delta: Some(delta),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::CheckServerUp => {
                if matches!(self.state,State::Idle) {
                    ctx.link().send_future(async {
                        let state = server_status().await.unwrap_or(State::EOffline);
                        Msg::UpdateStatus(state)
                    });
                }
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
                true
            },
            
            Msg::UpdateStatus(response) => {
                self.state = response;
                true
            },
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();
        let check = self.use_str_pot.clone();
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
                    <strong>{"Server State:"}</strong>{self.state.code()}<br/>
                    {self.state.message()}
                </div>
                    <button onclick={link.callback(|_| Msg::StartTest)}> {"Start Test"}</button>

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
            State::Awake => {
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
                        .json::<i32>()
                        .await{
                        Ok(code) => { State::from_i32(code) }
                        Err(_) => { State::EUnknown }
                    };
                    link.send_message(Msg::UpdateStatus(new_status));
                });
            }
            /*-------------------------------------------------------*/
            // server is awake, M4 is running firmware
            /*-------------------------------------------------------*/

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
