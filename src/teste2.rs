use std::net::UdpSocket;

fn main() {
    let socket = UdpSocket::bind("127.0.0.1:3002").expect("Erro ao bindar socket");

    // Envia uma mensagem para o processo principal
    socket.send_to(b"Hello from process 2", "127.0.0.1:3001").expect("Erro ao enviar mensagem");

    // Envia um sinal SIGUSR1 para o processo principal (PID deve ser definido)
    let pid: i32 = 12345; // Substitua pelo PID do processo principal
    unsafe { libc::kill(pid, libc::SIGUSR1) };

    println!("Mensagem enviada e sinal enviado para o PID {}", pid);
}
