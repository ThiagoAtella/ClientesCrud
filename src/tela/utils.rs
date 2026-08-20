use std::thread::sleep;
use std::time::Duration;

pub fn limpar_tela(){
    clearscreen::clear().expect("Erro ao limpar a tella");
}
pub fn esperar(tempo: u64){
    sleep(Duration::from_secs(tempo));
}
