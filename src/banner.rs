use colored::Colorize;

pub fn display_banner() {
    let title = r#"
███████╗██╗████████╗██████╗  ██████╗      ██████╗██╗     ██╗
╚══███╔╝██║╚══██╔══╝██╔══██╗██╔═══██╗    ██╔════╝██║     ██║
  ███╔╝ ██║   ██║   ██████╔╝██║   ██║    ██║     ██║     ██║
 ███╔╝  ██║   ██║   ██╔══██╗██║   ██║    ██║     ██║     ██║
███████╗██║   ██║   ██║  ██║╚██████╔╝    ╚██████╔███████╗██║
╚══════╝╚═╝   ╚═╝   ╚═╝  ╚═╝ ╚═════╝      ╚═════╝╚══════╝╚═╝"#;

    let subtitle = "AUDITEUR     D'EMPREINTE      CARBONE      NUMERIQUE";

    println!("{}", title.green().bold());
    println!("{}", subtitle.green().bold());
    println!();
}

pub fn display_installation_success(version: &str) {
    println!();
    println!("Installation reussie avec succes.");
    println!("Version installee : {}", version);
    println!();
    println!("Commandes pour debuter :");
    println!("   zitro --help            Afficher le menu d'aide complet");
    println!("   zitro --version         Verifier la version de l'outil");
    println!("   zitro scan <URL>        Lancer un audit (ex: zitro scan http://localhost:3000)");
    println!("   zitro scan <URL> -c TG  Lancer un audit avec le mix energetique du Togo");
    println!();
    println!("Rendez votre code plus vert, un scan a la fois.");
}