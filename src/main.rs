use yew::prelude::*;
use serde::{Serialize,Deserialize};
use gloo::net::http::Request;
use web_sys::wasm_bindgen::JsCast;
use wasm_bindgen_futures::spawn_local;
use std::fmt;
use std::str::FromStr;

const ADDRESS: &str = "http://172.20.10.7:8080";

#[derive(Deserialize,Clone)]
pub enum ServerResponse {
    Awake,
    InProgress,
    Done,
    Idle,
    ENoFirmware,
    ENoRead,
    ENoWrite,
    EOpen,
    EUnkown
}

impl ServerResponse {
    fn from_i32(n: i32) -> ServerResponse {
        match n {
            0 => Self::Idle,
            1 => Self::Awake,
            2 => Self::InProgress,
            3 => Self::Done,
            -1 => Self::ENoFirmware,
            -2 => Self::ENoRead,
            -3 => Self::ENoWrite,
            -4 => Self::EOpen,
            _ => Self:: EUnkown,
        }
    }
    fn code(&self) -> i32 {
        match self {
            Self::Idle => 0,
            Self::Awake => 1,
            Self::InProgress => 2,
            Self::Done => 3,
            Self::ENoFirmware=> -1,
            Self::ENoRead=> -2,
            Self::ENoWrite=> -3,
            Self::EOpen=> -4,
            Self::EUnkown=> -5,

        }
    }

    fn message(&self) -> &str{
        match self {
            Self::Idle => "Awaiting input",
            Self::Awake => "Server is up and has loaded microcontroller firmware",
            Self::InProgress => "Microcontroller has began testing",
            Self::Done => "Test results are available",
            Self::ENoFirmware => "No Firmware was found for the selected device",
            Self::ENoRead => "There was an error reading data from the microcontroller",
            Self::ENoWrite => "There was an error writing data to the microcontroller",
            Self::EOpen => "There was an error trying to communicate to the microcontroller",
            Self::EUnkown => "Something bad went wrong",
        }
    }
}

#[derive(Serialize,Deserialize,Clone,PartialEq)]
pub enum Device {
    BST,
    CWS,
    PrS,
    ESCM,
    None,
}

impl FromStr for Device {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Break Signal Transmitter" =>Ok(Device::BST),
            "Continuious Wear Sensor" => Ok(Device::CWS),
            "Pressure Sensor" => Ok(Device::PrS),
            "Electronic Stability Control Module" => Ok(Device::ESCM),
            "None" => Ok(Device::None),
            _ =>Err(())
        }
    }
}

impl fmt::Display for Device {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Device::BST => write!(f, "Brake Signal Transmitter"),
            Device::CWS => write!(f, "Continuous Wear Sensor"),
            Device::PrS => write!(f, "Pressure Sensor"),
            Device::ESCM=> write!(f, "Electronic Stability Control Module"),
            Device::None=> write!(f, "No Device Selected"),
        }
    }
}

#[derive(Default,Serialize,Clone,PartialEq)]
struct TestData {
    device: String,
    check: bool,
}

pub enum Msg {
    ShowDevices,
    UpdateChosenDevice(Device),
    UpdateStringPot(bool),
    StartTest,
    UpdateStatus(ServerResponse),
}
pub struct App {
    chosen_dev: Device,
    show_devices: bool,
    bst_chosen: bool,
    use_str_pot: bool,
    test_data: TestData,
    status: ServerResponse,
}

impl Component for App{
    type Message = Msg; 
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self { 
        Self {
            test_data: TestData::default(), 
            chosen_dev: Device::None,
            bst_chosen: false,
            use_str_pot: false,
            show_devices: false,
            status: ServerResponse::Idle,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::ShowDevices => {
                self.show_devices = !self.show_devices;
                true
            }
            Msg::UpdateChosenDevice(device) => {
                self.chosen_dev = device;
                self.test_data.device = self.chosen_dev.to_string();
                if self.chosen_dev == Device::BST {
                    self.bst_chosen = true;
                } else {
                    self.bst_chosen = false
                }
                true
            }
            Msg::UpdateStringPot(check) => {
                self.use_str_pot = check;
                self.test_data.check = self.use_str_pot;
                true
            }
            Msg::StartTest => {
                    let test = self.test_data.clone();
                    let link = ctx.link().clone();
                    spawn_local(async move {
                        let response = Request::post(ADDRESS)
                            .json(&test)
                            .unwrap()
                            .send()
                            .await;
                        let new_status = match response.unwrap().json::<i32>().await{
                            Ok(code) => { ServerResponse::from_i32(code) }
                            Err(_) => { ServerResponse::EUnkown }
                        };
                        link.send_message(Msg::UpdateStatus(new_status));
                    });
                true
            }
            Msg::UpdateStatus(response) => {
                self.status = response;
                true
            }
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
                    <strong>{"Server State:"}</strong>{self.status.code()}<br/>
                    {self.status.message()}
                </div>
                    <button onclick={link.callback(|_| Msg::StartTest)}> {"Start Test"}</button>

            </div>

        }
    }
}

impl App {
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
