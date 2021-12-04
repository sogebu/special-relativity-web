use golem::*;
use golem::{
    blend::{
        BlendChannel, BlendEquation, BlendFactor, BlendFunction, BlendInput, BlendMode,
        BlendOperation,
    },
    depth::{DepthTestFunction, DepthTestMode},
    Dimension::*,
};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct GolemBackend {
    golem_ctx: Context,
    tex: Texture,
    shader: ShaderProgram,
    vb: VertexBuffer,
    eb: ElementBuffer,
}

#[wasm_bindgen]
impl GolemBackend {
    #[wasm_bindgen(constructor)]
    pub fn from_webgl(webgl_context: web_sys::WebGlRenderingContext) -> Self {
        let glow_context = glow::Context::from_webgl1_context(webgl_context);
        let golem_context = golem::Context::from_glow(glow_context).unwrap();
        GolemBackend::new(golem_context).unwrap()
    }

    pub fn render(&mut self) {
        self.draw().unwrap();
    }
}

impl GolemBackend {
    pub fn new(golem_ctx: Context) -> Result<Self, GolemError> {
        let blend_mode = BlendMode {
            equation: BlendEquation::Same(BlendOperation::Add),
            function: BlendFunction::Same {
                source: BlendFactor::Color {
                    input: BlendInput::Source,
                    channel: BlendChannel::Alpha,
                    is_inverse: false,
                },
                destination: BlendFactor::Color {
                    input: BlendInput::Source,
                    channel: BlendChannel::Alpha,
                    is_inverse: true,
                },
            },
            global_color: [0.0; 4],
        };
        let mut tex = Texture::new(&golem_ctx)?;
        tex.set_image(Some(&[255; 128 * 128 * 4]), 128, 128, ColorFormat::RGBA);
        golem_ctx.set_blend_mode(Some(blend_mode));

        let depth_test = DepthTestMode {
            function: DepthTestFunction::Always,
            range_near: 0.0,
            range_far: 1.0,
            depth_mask: false,
        };
        golem_ctx.set_depth_test_mode(Some(depth_test));

        let mut shader = ShaderProgram::new(
            &golem_ctx,
            ShaderDescription {
                vertex_input: &[Attribute::new("vert_position", AttributeType::Vector(D2))],
                fragment_input: &[],
                uniforms: &[Uniform::new("projection", UniformType::Matrix(D4))],
                vertex_shader: r#" void main() {
                    gl_Position = projection * vec4(vert_position, 0, 1);
                }"#,
                fragment_shader: r#" void main() {
                    gl_FragColor = vec4(1.0, 0.0, 0.0, 1.0);
                }"#,
            },
        )?;
        let vb = VertexBuffer::new(&golem_ctx)?;
        let eb = ElementBuffer::new(&golem_ctx)?;
        shader.bind();
        Ok(Self {
            golem_ctx,
            tex,
            shader,
            vb,
            eb,
        })
    }

    pub fn draw(&mut self) -> Result<(), GolemError> {
        #[rustfmt::skip]
        let projection = UniformValue::Matrix4([
            1.8106601238250732, 0.0, 0.0, 0.0,
            0.0, 2.4142136573791504, 0.0, 0.0,
            0.0, 0.0, -1.0020020008087158, -1.0,
            0.0, 0.0, 5.811811923980713, 6.0,
        ]);
        #[rustfmt::skip]
        let vertices = &[
            -1.0,  1.0,
            1.0,  1.0,
            -1.0, -1.0,
            1.0, -1.0,
        ];
        let indices = &[0, 1, 2, 3];
        self.vb.set_data(vertices);
        self.eb.set_data(indices);
        self.shader.prepare_draw(&self.vb, &self.eb)?;
        self.shader.set_uniform("projection", projection)?;
        // self.golem_ctx.set_viewport(0, 0, 10, 10);
        self.golem_ctx.set_clear_color(0.0, 1.0, 1.0, 0.8);
        self.tex.set_active(std::num::NonZeroU32::new(1).unwrap());
        self.golem_ctx.clear();
        unsafe {
            self.shader.draw_prepared(0..4, GeometryMode::TriangleStrip);
        }
        Ok(())
    }
}
