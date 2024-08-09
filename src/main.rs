use colorful::{Color, Colorful};
use eye_oxide::Tools;

fn main() {
    let ey_ox = format!(
        "{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n",
        r#"MM""""""""`M                   MMP"""""YMM          oo       dP          "#,
        r#"MM  mmmmmmmM                   M' .mmm. `M                   88          "#,
        r#"M`      MMMM dP    dP .d8888b. M  MMMMM  M dP.  .dP dP .d888b88 .d8888b. "#,
        r#"MM  MMMMMMMM 88    88 88ooood8 M  MMMMM  M  `8bd8'  88 88'  `88 88ooood8 "#,
        r#"MM  MMMMMMMM 88.  .88 88.  ... M. `MMM' .M  .d88b.  88 88.  .88 88.  ... "#,
        r#"MM        .M `8888P88 `88888P' MMb     dMM dP'  `dP dP `88888P8 `88888P' "#,
        r#"MMMMMMMMMMMM      .88          MMMMMMMMMMM                               "#,
        r#"              d8888P                                                     "#,
    );

    let creds = format!(
        "{}\n{}\n{}\n",
        r#" -----------------------------------"#,
        r#"|https://github.com/rootprt/EyeOxide|"#,
        r#" -----------------------------------"#,
    );

    print!("{}", ey_ox.gradient(Color::Red));
    print!("\n{}", creds.gradient(Color::Red3a));
    println!("");

    Tools::cli();
}
