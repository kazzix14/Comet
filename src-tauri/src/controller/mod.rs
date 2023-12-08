mod command;
mod notification;

pub use command::Command;
pub use notification::Notification;

use crate::frontend;

use std::thread::sleep;
use std::{cell::RefCell, rc::Rc};

use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use tokio::sync::mpsc;

pub struct Controller {
    notification_dispatcher: mpsc::UnboundedSender<frontend::Notification>,
    command: mpsc::UnboundedReceiver<Command>,
    is_playing: bool,
    cpal_host: Rc<cpal::Host>,
    current_output_device: Option<Rc<cpal::Device>>,
    current_output_device_config: Option<Rc<cpal::StreamConfig>>,
    current_stream: Option<Box<cpal::Stream>>,
}

impl Controller {
    pub fn new(
        notification_dispatcher: mpsc::UnboundedSender<frontend::Notification>,
        command: mpsc::UnboundedReceiver<Command>,
    ) -> Self {
        Self {
            notification_dispatcher,
            command,
            is_playing: false,
            cpal_host: Rc::new(cpal::default_host()),
            current_output_device: None,
            current_output_device_config: None,
            current_stream: None,
        }
    }

    pub fn set_output_device(&mut self) {
        let device = self
            .cpal_host
            .default_output_device()
            .expect("failed to get default output device");

        let mut supported_configs = device
            .supported_output_configs()
            .expect("error while querying configs");
        let supported_config_range = supported_configs.next().expect("no supported config?!");
        dbg!(supported_config_range.clone());
        let config = supported_config_range
            .with_sample_rate(cpal::SampleRate(44100))
            .config();

        dbg!(config.clone());
        dbg!(device.name());
        self.current_output_device_config =
            Some(Rc::new(device.default_output_config().unwrap().config()));

        let stream = device
            .build_output_stream(
                &self
                    .current_output_device_config
                    .clone()
                    .expect("no config?!"),
                move |data: &mut [f32], info: &cpal::OutputCallbackInfo| {
                    // react to stream events and read or write stream data here.
                    tracing::debug!("writing audio");

                    tracing::debug!("{:?}", info);
                    let mut i = 0f32;
                    for sample in data.iter_mut() {
                        *sample = i.sin();
                        i = i + 1.0 / 44100.0 * 440.0;
                    }
                },
                move |err| {
                    // react to errors here.
                    tracing::error!("error while writing audio: {}", err);
                },
                None, // None=blocking, Some(Duration)=timeout
            )
            .expect("failed to build output stream");
        stream.pause().expect("failed to pause stream");

        self.current_stream = Some(Box::new(stream));
        self.current_output_device = Some(Rc::new(device));
    }

    pub fn run(mut self) {
        loop {
            //tracing::debug!("Controller::run");

            self.process_commands();

            if let Some(device) = self.current_output_device.clone() {
                //tracing::debug!("device: {:?}", device.fm);
                if self.is_playing {
                    //tracing::debug!("is_playing");
                    // write audio to buffer

                    // send buffer to IO
                    //device.as_inner().to_owned();
                    //tracing::debug!("{:?}", device.as_inner());

                    //tracing::debug!("built stream");
                }
            }

            //sleep(std::time::Duration::from_millis(100));
        }
    }

    fn process_commands(&mut self) {
        //tracing::debug!("Controller::process_commands");

        while let Ok(command) = self.command.try_recv() {
            //tracing::debug!("processing command: {:?}", command);

            match command {
                Command::HealthCheck => {
                    self.notification_dispatcher
                        .send(Notification::HealthCheck.into())
                        .unwrap();
                    println!("HealthCheck");
                }
                Command::Play => {
                    self.is_playing = true;
                    if let Some(stream) = &self.current_stream {
                        stream.play().expect("failed to play stream");
                    }
                    self.notification_dispatcher
                        .send(Notification::Playing.into())
                        .unwrap();
                }
                Command::Pause => {
                    self.is_playing = false;
                    if let Some(stream) = &self.current_stream {
                        stream.pause().expect("failed to pause stream");
                    }
                    self.notification_dispatcher
                        .send(Notification::Pausing.into())
                        .unwrap();
                }
            }
        }
    }
}
