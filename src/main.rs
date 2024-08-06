mod enums;

use colored::Colorize;

fn main() {
    let ey_ox = r#"MM""""""""`M                   MMP"""""YMM          oo       dP          
MM  mmmmmmmM                   M' .mmm. `M                   88         
M`      MMMM dP    dP .d8888b. M  MMMMM  M dP.  .dP dP .d888b88 .d8888b.     
MM  MMMMMMMM 88    88 88ooood8 M  MMMMM  M  `8bd8'  88 88'  `88 88ooood8 
MM  MMMMMMMM 88.  .88 88.  ... M. `MMM' .M  .d88b.  88 88.  .88 88.  ...     
MM        .M `8888P88 `88888P' MMb     dMM dP'  `dP dP `88888P8 `88888P' 
MMMMMMMMMMMM      .88          MMMMMMMMMMM                               
              d8888P                                                     
"#.red();

    print!("{}", ey_ox);
    enums::Tools::cli();
}
