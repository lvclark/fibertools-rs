use super::PbChem;
use spin;
use std::fs;
use tch;

// make sure file exists for cargo
static INIT_PT: spin::Once<tch::CModule> = spin::Once::new();
static PT: &[u8] = include_bytes!("../models/2.0_torch.pt");
static PT_2_2: &[u8] = include_bytes!("../models/2.0_torch.pt");

pub fn get_saved_pytorch_model(polymerase: &PbChem) -> &'static tch::CModule {
    INIT_PT.call_once(|| {
        let d = tch::Device::cuda_if_available();
        log::warn!("Using {:?} for Torch device.", d);
        let model_str = match polymerase {
            PbChem::Two => {
                log::warn!("Using model for 2.0 chemistry");
                PT
            }
            PbChem::TwoPointTwo => {
                log::warn!("Using model for 2.2 chemistry");
                PT_2_2
            }
        };
        let temp_file_name = "ft.tmp.model.json";
        fs::write(temp_file_name, model_str).expect("Unable to write file");
        let model = tch::CModule::load(temp_file_name).expect("Unable to load PyTorch model");
        fs::remove_file(temp_file_name).expect("Unable to remove temp model file");
        log::info!("Model from PyTorch loaded");
        //tch::nn::VarStore::set_device(&mut self, device)
        model
    })
}

pub fn predict_with_cnn(windows: &[f32], count: usize, polyermase: &PbChem) -> Vec<f32> {
    let model = get_saved_pytorch_model(polyermase);
    let ts = tch::Tensor::of_slice(windows).to_device(tch::Device::cuda_if_available());
    let ts = ts.reshape(&[count.try_into().unwrap(), 6, 15]);
    log::warn!("Made other gpu? tensor");
    let x = model.forward_ts(&[ts]).unwrap();
    let w: Vec<f32> = x.try_into().unwrap();
    let z: Vec<f32> = w.chunks(2).map(|c| c[0]).collect();
    log::trace!(
        "{:?} {} {} {}",
        z.len(),
        count,
        z.iter().sum::<f32>() / z.len() as f32,
        w.chunks(2).map(|c| c[1]).sum::<f32>() / z.len() as f32
    );
    z
}
