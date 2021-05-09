#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

pub mod renderer;
pub mod render_target;
pub mod shader;
pub mod vertex_buffer;
pub mod com_util;
