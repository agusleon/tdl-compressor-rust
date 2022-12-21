use indicatif::{MultiProgress, ProgressStyle};

pub fn create_multiprogress_bar() -> (MultiProgress, ProgressStyle){
    let multiprogress_bar = MultiProgress::new();

    let style = ProgressStyle::with_template(
        "[{elapsed_precise}] {bar:40.cyan/blue} {pos:>7}/{len:7} {msg}",
    )
    .unwrap()
    .progress_chars("##-");

    return (multiprogress_bar, style)
}