mod command;
mod notification;

pub use command::Command;
pub use notification::Notification;

use crate::frontend;

use std::collections::VecDeque;
use std::sync::{Arc, Mutex};
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
    audio_buffer: Arc<Mutex<VecDeque<f64>>>,
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
            audio_buffer: Arc::new(Mutex::new(VecDeque::new())),
        }
    }

    pub fn set_output_device(&mut self) {
        let device = self
            .cpal_host
            .default_output_device()
            .expect("failed to get default output device");

        tracing::debug!("{:?}", device.name());
        self.current_output_device_config =
            Some(Rc::new(device.default_output_config().unwrap().config()));

        let audio_buffer = Arc::clone(&self.audio_buffer);
        let stream = device
            .build_output_stream(
                &self
                    .current_output_device_config
                    .clone()
                    .expect("no config?!"),
                move |data: &mut [f32], info: &cpal::OutputCallbackInfo| {
                    // react to stream events and read or write stream data here.
                    for sample in data.iter_mut() {
                        if let Some(audio) =
                            audio_buffer.lock().expect("failed to Lock").pop_front()
                        {
                            *sample = audio as f32;
                        } else {
                            tracing::debug!("no audio data");
                            *sample = 0f32;
                        }
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

    /*
     * イベントループでシートの内容を読み込んで、オーディオとして積んでいく
     */
    pub fn run(mut self) {
        loop {
            //tracing::debug!("Controller::run");

            self.process_commands();

            //tracing::debug!("device: {:?}", device.fm);
            if self.is_playing {
                //tracing::debug!("is_playing");
                // write audio to buffer
                let mut audio_buffer = self.audio_buffer.lock().expect("failed to Lock");

                const BUFFER_SIZE: usize = 40000;

                if audio_buffer.len() < BUFFER_SIZE {
                    // audio_bufferにデータが足りない場合は、シートの内容を読み込む
                    // TODO: シートの内容を読み込む
                    // TODO: シートの内容をオーディオに変換する
                    audio_buffer.push_back(rand::random::<f64>() * 2.0 - 1.0);
                }

                // send buffer to IO
                //device.as_inner().to_owned();
                //tracing::debug!("{:?}", device.as_inner());

                //tracing::debug!("built stream");
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
