use std::{fs, path::PathBuf};

use base64::{engine::general_purpose, Engine as _};

use std::path::{Path};

mod nfs {
    #[derive(Clone)]
    pub(crate) struct AuthInfo(String); // NFS session management omitted
}

mod bootp {
    pub(crate) struct AuthInfo(); // no authentication in bootp
}

// private module, lest outside users invent their own protocol kinds!
mod proto_trait {
    use std::path::{Path, PathBuf};
    use super::{bootp, nfs};

    pub(crate) trait ProtoKind {
        type AuthInfo;
        fn auth_info(&self) -> Self::AuthInfo;
    }

    pub struct Nfs {
        auth: nfs::AuthInfo,
        mount_point: PathBuf,
    }

    impl Nfs {
        pub fn mount_point(&self) -> &Path {
            &self.mount_point
        }
    }

    impl ProtoKind for Nfs {
        type AuthInfo = nfs::AuthInfo;
        fn auth_info(&self) -> Self::AuthInfo {
            self.auth.clone()
        }
    }

    pub struct Bootp(); // no additional metadata

    impl ProtoKind for Bootp {
        type AuthInfo = bootp::AuthInfo;
        fn auth_info(&self) -> Self::AuthInfo {
            bootp::AuthInfo()
        }
    }
}

use proto_trait::ProtoKind; // keep internal to prevent impls
pub use proto_trait::{Nfs, Bootp}; // re-export so callers can see them

struct FileDownloadRequest<P: ProtoKind> {
    file_name: PathBuf,
    protocol: P,
}

// all common API parts go into a generic impl block
impl<P: ProtoKind> FileDownloadRequest<P> {
    fn file_path(&self) -> &Path {
        &self.file_name
    }

    fn auth_info(&self) -> P::AuthInfo {
        self.protocol.auth_info()
    }
}

// all protocol-specific impls go into their own block
impl FileDownloadRequest<Nfs> {
    fn mount_point(&self) -> &Path {
        self.protocol.mount_point()
    }
}



fn main() {
    let mut text = r#" 
    upstream ftp_pasv-port {    
        server 10.176.60.55:-port max_fails=2 fail_timeout=3s weight=1;
    }
    server {              
        listen -port;
        proxy_pass ftp_pasv-port;
    }"#;
    let mut nginx_conf = "".to_string();
    for i in 50602..50801 {
        let replace_text = text.replace("-port", i.to_string().as_str());
        println!("{}", replace_text);
        nginx_conf = format!("{0} \r\n {1}", nginx_conf, replace_text);
        // let str_text = format!("{0} \r\n {1}", text, replace_text);
        // text = str_text.as_str();
    }
    let f = fs::File::create("n.conf").unwrap();
    fs::write("n.conf", nginx_conf).unwrap();

    let orig = "21BNSIT209_PF3KXVHL***123456***7753-DE23-B948-FB0G";
    let encoded: String = general_purpose::STANDARD.encode(orig);
    println!("base64:{}", encoded);

    for i in 5..1 {
        println!("{}", i);
    }

    // let cur_build_number_path = PathBuf::from("cur_build_number_path");
    // let mut cur_build_number = -1;
    // if cur_build_number_path.exists() {
    //     cur_build_number = match fs::read_to_string(&cur_build_number_path)
    //         .unwrap()
    //         .parse::<i32>()
    //     {
    //         std::result::Result::Ok(r) => r,
    //         Err(err) => -1,
    //     }
    // }
    // println!("cur_build_number:{cur_build_number}");

    // let hec_package_dir = "C:/Packages/windowsAgent/plugin";
    // println!("{:?}", PathBuf::from(hec_package_dir).join("dir3"));
    // if !PathBuf::from(hec_package_dir).join("dir3").exists() {
        
    //     fs::create_dir_all(PathBuf::from(hec_package_dir).join("dir3")).unwrap();
    // }

   
}


