use std::env;
use clap::Parser;
use std::path::PathBuf;


#[derive(Parser)]
#[command(name = "route")]
#[command(author = "Thierry P. <thierry.probst@free.fr>")]
#[command(version = "1.1.0")]
#[command(about = "Some things for routing", long_about = None)]
#[command(help_template = "{author-with-newline} {about-section} \nVersion: {version} \n {usage-heading} {usage} \n\n {all-args} {tab}")]
#[command(propagate_version = true)]
pub struct Cli {
    /// Optional file name to operate on. default is "St_Brieuc-Loudéac"
    #[arg(short,long)]
    pub filename: Option<PathBuf>,

    /// Optional input file type in ["osm", "osm.pbf"]. default is "osm.pbf"
    #[arg(short,long)]
    pub itype: Option<String>,
}


#[derive(Debug)]
pub struct Datafiles {
    pub input_file: PathBuf,
}


impl Datafiles {
    pub fn new(f: Option<PathBuf>, e: Option<String>) -> Self {
        let mut fpath = env::current_dir().unwrap();
        fpath.push( "data" );
        if let Some(filename) = f { fpath.push( filename ) } else { fpath.push("St_Brieuc-Loudéac") };
        if let Some(ext) = e { fpath.set_extension( ext.as_str() ) } else { fpath.set_extension( "osm.pbf" ) };
        Self {
            input_file: fpath
        }
    }

    pub fn get_ifilepath(&self) -> &PathBuf {
        &self.input_file
    }

    pub fn get_ifile_str(& self) -> String {
        self.input_file.as_os_str().to_str().expect("all must be right").to_string()
    }

}

pub fn get_input_filename() -> String {
    let cli = Cli::parse();
    let df = Datafiles::new( cli.filename, cli.itype );
    df.get_ifile_str()
}


#[cfg(test)]
mod cli_tests {
    use super::*;

    #[test]
    fn create_df() {
        let df = Datafiles::new( Some( PathBuf::from("StBrieuc")), Some( "osm.pbf".to_string()) );
        let r = PathBuf::from("/mnt/vg1-data/Code/Rust/route/data/StBrieuc.osm.pbf");
        assert_eq!( &r, df.get_ifilepath() );
        assert_eq!( "/mnt/vg1-data/Code/Rust/route/data/StBrieuc.osm.pbf".to_string(), df.get_ifile_str() );
    }
}
