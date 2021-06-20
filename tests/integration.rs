use raygun::render;
use tempfile::tempfile;

#[test]
fn render_test() {
    // confirms render successsful writes to a some output

    let test_img = tempfile().unwrap();

    let render_op = render(test_img, |mut img_writer| {
        img_writer.header(1, 2)?;
        img_writer.append("1 2 3")
    });
    render_op.unwrap();

    // confirm ability to write to different interfaces
    let test_vec = Vec::new();

    let vec_render = render(test_vec, |mut img_writer| {
        img_writer.header(1, 2)?;
        img_writer.append("1 2 3")
    });

    vec_render.unwrap();
}
