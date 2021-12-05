

// called when we are passed a markdown file via command line
fn parse_markdown_file(){

}
// this will output the title, version, and description
fn print_short_banner(){
    println!("{}", get_title());
}

//this will output the short banner plus a "written by" plus "usage"
fn print_long_banner(){
    print_short_banner();
      println!("Written by: {}\nHomepage: {}\nUsage: tinymd <somefile>.md\n",
    env!("CARGO_PKG_AUTHORS"),
    env!("CARGO_PKG_HOMEPAGE")
  );
}

fn get_title() -> String{
    let mut the_title = String::from(env!("CARGO_PKG_NAME"));
    the_title.push_str(" (v)");
    the_title.push_str(env!("CARGO_PKG_VERSION"));
    the_title.push_str(")");
    the_title.push_str(env!("CARGO_PKG_DESCRIPTION"));
    return the_title;
}
fn main() {
   print_short_banner(); 
}

