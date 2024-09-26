use signal_hook::consts::SIGUSR1;
use signal_hook::low_level;
use std::net::UdpSocket;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use std::io::Error;

// Função chamada quando o sinal é recebido
fn signal_handler(message_storage: Arc<Mutex<Option<String>>>) {
    // Inicia a recepção da mensagem e atualiza a variável compartilhada
    receive_message(message_storage);
}

// Função para receber a mensagem via UDP e atualizar a variável compartilhada
fn receive_message(message_storage: Arc<Mutex<Option<String>>>) {
    let socket = UdpSocket::bind("127.0.0.1:3001").expect("Erro ao bindar socket");

    let mut buf = [0; 1024];  // Buffer para a mensagem
    match socket.recv_from(&mut buf) {
        Ok((amt, _src)) => {
            let received_msg = String::from_utf8_lossy(&buf[..amt]).to_string();

            // Atualiza a variável compartilhada com a mensagem recebida
            let mut message = message_storage.lock().unwrap();
            *message = Some(received_msg);
        },
        Err(e) => {
            eprintln!("Erro ao receber mensagem: {}", e);
        }
    }
}

fn main() -> Result<(), Error> {
    // Cria uma variável compartilhada que será atualizada pelo sinal handler
    let message_storage = Arc::new(Mutex::new(Some("Mensagem Inicial".to_string())));

    // Clona o Arc para passar uma referência segura para o signal_handler
    let message_storage_clone = Arc::clone(&message_storage);

    // Registra o manipulador de sinal
    unsafe {
        low_level::register(SIGUSR1, move || {
            signal_handler(Arc::clone(&message_storage_clone))
        }).unwrap();
    }

    // Laço principal simulando trabalho de computação
    loop {
        // Imprime a mensagem atual
        let message = message_storage.lock().unwrap();
        println!("Mensagem atual: {:?}", *message);

        // Simula um trabalho sendo realizado
        thread::sleep(Duration::from_secs(1));
    }
}
