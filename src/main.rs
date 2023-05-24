fn main() {
    // Exemplo de proteção de espaço de memória
    let mut buffer = [0u8; 10]; // Cria um buffer seguro de tamanho fixo

    // Exemplo de leitura segura de entrada do usuário
    println!("Digite seu nome:");
    let mut name = String::new();
    match std::io::stdin().read_line(&mut name) {
        Ok(_) => {
            // Truncar a entrada do usuário para caber no buffer
            let name = name.trim().as_bytes();
            let len = std::cmp::min(name.len(), buffer.len());
            buffer[..len].copy_from_slice(&name[..len]);

            // Exemplo de impressão segura do conteúdo do buffer
            println!("Olá, {}!", String::from_utf8_lossy(&buffer));
        }
        Err(error) => {
            eprintln!("Erro ao ler a entrada: {}", error);
        }
    }

    // Exemplo de prevenção de ataques de desreferência nula
    let ptr: *const i32 = std::ptr::null();
    unsafe {
        if !ptr.is_null() {
            println!("Valor: {}", *ptr);
        } else {
            println!("Ponteiro nulo!");
        }
    }

    // Exemplo de prevenção de ataques de desbordamento de vetor
    let arr = [1, 2, 3, 4, 5];
    let index = 10;
    if index < arr.len() {
        println!("Valor no índice {}: {}", index, arr[index]);
    } else {
        println!("Índice inválido!");
    }
}
