use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use ringbuf::HeapProd;
use ringbuf::{traits::*, HeapRb};
use std::sync::Mutex;
use std::{mem, sync::OnceLock};

pub static ENGINE: OnceLock<Engine> = OnceLock::new();

pub fn get() -> Option<&'static Engine> {
    match ENGINE.get() {
        Some(engine) => Some(engine),
        None => {
            let engine = Engine::new()?;
            ENGINE.set(engine).ok()?;
            ENGINE.get()
        }
    }
}

pub struct Engine {
    // sender: mpsc::Sender<f32>,
    // data: Arc<Mutex<Vec<f32>>>,
    // data: HeapRb<f32>,
    config: cpal::StreamConfig,
    // producer: Caching<Arc<SharedRb<f32>>, true, false>,
    // producer: dyn Producer<Item = f32>,
    producer: Mutex<HeapProd<f32>>,
}

impl Engine {
    pub fn new() -> Option<Self> {
        let host = cpal::default_host();
        let device = host.default_output_device()?;
        let mut supported_configs_range = device.supported_output_configs().ok()?;
        let supported_config = supported_configs_range.next()?.with_max_sample_rate();

        // let (sender, receiver) = mpsc::channel();
        // let data = Arc::new(Mutex::new(Vec::new()));
        let rb = HeapRb::<f32>::new(48_000);
        let (prod, mut cons) = rb.split();

        let stream = device
            .build_output_stream(
                &supported_config.clone().into(),
                move |d: &mut [f32], _: &cpal::OutputCallbackInfo| {
                    let count = cons.pop_slice(d);
                    if count < d.len() {
                        // d[count..] = [0.0; d.len() - count];
                        for i in count..d.len() {
                            d[i] = 0.0;
                        }
                    }
                    // // let mut data = data_clone.lock().unwrap();
                    // let mut last = 0.0;
                    // for d in d.iter_mut() {
                    //     // let sample = match data.get(0) {
                    //     //     Some(s) => {data.remove(0)s,
                    //     //     Err(_) => 0.0,
                    //     // };
                    //     let sample = if data.len() > 0 {
                    //         data.remove(0)
                    //     } else {
                    //         last * 0.9
                    //     };
                    //     last = sample;
                    //     *d = sample;
                    // }
                    // drop(data);
                },
                move |err| {
                    // react to errors here.
                    eprintln!("{:?}", err);
                },
                None,
                // None, // None=blocking, Some(Duration)=timeout
                // Some(Duration::from_millis(1))
            )
            .ok()?;

        stream.play().ok()?;
        mem::forget(stream);

        Some(Engine {
            config: supported_config.into(),
            producer: Mutex::new(prod),
        })
    }

    pub fn play_mono(&self, data: Vec<f32>) {
        let data = self.interleave(&data);

        let mut prod = self.producer.lock().unwrap();
        prod.push_slice(&data);
        // let mut d = self.data.lock().unwrap();
        // d.append(&mut data);
    }

    fn interleave(&self, input: &[f32]) -> Vec<f32> {
        let mut buffer = Vec::new();
        let n = self.config.channels;
        for i in input {
            for _ in 0..n {
                buffer.push(*i);
            }
        }

        buffer
    }
}
