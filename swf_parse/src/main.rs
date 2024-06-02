use std::fs::read;

use swf_parse::display_object::TDisplayObject;
use swf_parse::{
    context::UpdateContext,
    display_object::{container::TDisplayObjectContainer, movie_clip::MovieClip},
    library::MovieLibrary,
};
fn main() {
    let data = if cfg!(target_os = "windows") {
        read("D:\\Code\\Rust\\swf_animation\\desktop\\swf_files\\spirit2471src.swf").unwrap()
    } else {
        read("/home/intasect/study/Rust/swf_animation/desktop/swf_files/spirit2471src.swf").unwrap()
    };
    let swf_buf = swf::decompress_swf(&data[..]).unwrap();
    let parse_swf = swf::parse_swf(&swf_buf).unwrap();
    println!("{:?}", parse_swf.header.swf_header().frame_rate);
    println!("{:?}", parse_swf.header.swf_header().num_frames);
    println!(
        "width:{:?},height:{:?}",
        parse_swf.header.swf_header().stage_size.width().to_pixels(),
        parse_swf
            .header
            .swf_header()
            .stage_size
            .height()
            .to_pixels()
    );
    let player_version = parse_swf.header.swf_header().version;
    let mut movie_library = MovieLibrary::new();
    let mut update_context = UpdateContext::new(player_version, &mut movie_library);
    let mut movie_clip = MovieClip::empty(parse_swf.header.swf_header().version);
    movie_clip.parse_tag(parse_swf.tags, &mut update_context);
    movie_clip.run_frame(swf::parse_swf(&swf_buf).unwrap().tags, &mut update_context);
    dbg!(update_context.library.length());
}
