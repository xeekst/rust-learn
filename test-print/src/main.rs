use base64::{engine::general_purpose, Engine as _};

fn main() {
    let mut text = r#" 
    upstream ftp_pasv-port {    
        server 10.176.60.55:-port max_fails=2 fail_timeout=3s weight=1;
    }
    server {              
        listen -port;
        proxy_pass ftp_pasv-port;
    }"#;
    for i in 50602..50701 {
        let replace_text = text.replace("-port", i.to_string().as_str());
        println!("{}", replace_text);
        // let str_text = format!("{0} \r\n {1}", text, replace_text);
        // text = str_text.as_str();
    }

    let orig = "21BNSIT209_PF3KXVHL***123456***7753-DE23-B948-FB0G";
    let encoded: String = general_purpose::STANDARD.encode(orig);
    println!("base64:{}", encoded);
}
