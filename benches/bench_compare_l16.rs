use fast_image_resize::pixels::U16;
use resize::Pixel::Gray16;
use rgb::FromSlice;
use utils::testing::PixelTestingExt;

mod utils;

pub fn bench_downscale_l16(bench_group: &mut utils::BenchGroup) {
    type P = U16;
    let src_image = P::load_big_image();
    utils::image_resize(bench_group, &src_image);
    utils::resize_resize(
        bench_group,
        Gray16,
        src_image.as_raw().as_gray(),
        src_image.width(),
        src_image.height(),
    );
    utils::libvips_resize::<P>(bench_group, false);
    utils::fir_resize::<P>(bench_group, false);
}

fn main() {
    let res = utils::run_bench(bench_downscale_l16, "Compare resize of U16 image");
    utils::print_and_write_compare_result(&res);
}
