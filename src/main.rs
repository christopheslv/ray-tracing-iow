fn main() {
    // Image
    let img_width = 256;
    let img_height = 256;

    // Render
    println!("P3\n{} {}\n255", img_width, img_height);

    let mut j = img_height-1;
    
    while j >= 0 {
        let mut i = 0;

        while i < img_width{
            let r = i as f64 / (img_width-1) as f64;
            let g = j as f64 / (img_height-1) as f64;
            let b = 0.25;

            let ir = (255.999 * r) as u32;
            let ig = (255.999 * g) as u32;
            let ib = (255.999 * b) as u32;

            println!("{} {} {}", ir, ig, ib);
            i += 1;
        }

        j = j-1;
    }
}
