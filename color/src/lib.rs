#![no_std]
use bytemuck::{Pod, Zeroable};

#[derive(Debug, Clone, Copy, Zeroable, Pod)]
#[repr(C)]
pub struct RGB {
    pub r: f32,
    pub g: f32,
    pub b: f32,
}

#[derive(Debug, Clone, Copy, Zeroable, Pod)]
#[repr(C)]
pub struct RGBA {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

fn clamp(f: f32) -> f32 {
    let f = f.clamp(0.0, 1.0);
    if f.is_nan() {
        0.0
    } else {
        f
    }
}

impl RGB {
    const fn new_unchecked(r: f32, g: f32, b: f32) -> Self {
        Self { r, g, b }
    }

    pub fn new(r: f32, g: f32, b: f32) -> Self {
        Self {
            r: clamp(r),
            g: clamp(g),
            b: clamp(b),
        }
    }

    pub fn rgba(&self, a: f32) -> RGBA {
        RGBA::new_unchecked(self.r, self.g, self.b, clamp(a))
    }
}

impl RGBA {
    const fn new_unchecked(r: f32, g: f32, b: f32, a: f32) -> Self {
        Self { r, g, b, a }
    }

    pub fn new(r: f32, g: f32, b: f32, a: f32) -> Self {
        Self {
            r: clamp(r),
            g: clamp(g),
            b: clamp(b),
            a: clamp(a),
        }
    }

    pub const fn rbg(&self) -> RGB {
        RGB::new_unchecked(self.r, self.g, self.b)
    }
}

/// data from https://www.colordic.org/
/// generate by gen_color.py
#[rustfmt::skip]
mod color_swatch {
    #![allow(clippy::excessive_precision)]
    use super::{RGB, RGBA};

    macro_rules! impl_color {
        ($name:ident, $r:expr, $g:expr, $b:expr) => {
            impl RGB {
                pub const fn $name() -> RGB {
                    RGB::new_unchecked($r, $g, $b)
                }
            }
            impl RGBA {
                pub const fn $name() -> RGBA {
                    RGBA::new_unchecked($r, $g, $b, 1.0)
                }
            }
        };
    }
    impl_color!(black, 0.0, 0.0, 0.0);
    impl_color!(aliceblue, 0.9411764705882353, 0.9725490196078431, 1.0);
    impl_color!(darkcyan, 0.0, 0.5450980392156862, 0.5450980392156862);
    impl_color!(lightyellow, 1.0, 1.0, 0.8784313725490196);
    impl_color!(coral, 1.0, 0.4980392156862745, 0.3137254901960784);
    impl_color!(dimgray, 0.4117647058823529, 0.4117647058823529, 0.4117647058823529);
    impl_color!(lavender, 0.9019607843137255, 0.9019607843137255, 0.9803921568627451);
    impl_color!(teal, 0.0, 0.5019607843137255, 0.5019607843137255);
    impl_color!(lightgoldenrodyellow, 0.9803921568627451, 0.9803921568627451, 0.8235294117647058);
    impl_color!(tomato, 1.0, 0.38823529411764707, 0.2784313725490196);
    impl_color!(gray, 0.5019607843137255, 0.5019607843137255, 0.5019607843137255);
    impl_color!(lightsteelblue, 0.6901960784313725, 0.7686274509803922, 0.8705882352941177);
    impl_color!(darkslategray, 0.1843137254901961, 0.30980392156862746, 0.30980392156862746);
    impl_color!(lemonchiffon, 1.0, 0.9803921568627451, 0.803921568627451);
    impl_color!(orangered, 1.0, 0.27058823529411763, 0.0);
    impl_color!(darkgray, 0.6627450980392157, 0.6627450980392157, 0.6627450980392157);
    impl_color!(lightslategray, 0.4666666666666667, 0.5333333333333333, 0.6);
    impl_color!(darkgreen, 0.0, 0.39215686274509803, 0.0);
    impl_color!(wheat, 0.9607843137254902, 0.8705882352941177, 0.7019607843137254);
    impl_color!(red, 1.0, 0.0, 0.0);
    impl_color!(silver, 0.7529411764705882, 0.7529411764705882, 0.7529411764705882);
    impl_color!(slategray, 0.4392156862745098, 0.5019607843137255, 0.5647058823529412);
    impl_color!(green, 0.0, 0.5019607843137255, 0.0);
    impl_color!(burlywood, 0.8705882352941177, 0.7215686274509804, 0.5294117647058824);
    impl_color!(crimson, 0.8627450980392157, 0.0784313725490196, 0.23529411764705882);
    impl_color!(lightgray, 0.8274509803921568, 0.8274509803921568, 0.8274509803921568);
    impl_color!(steelblue, 0.27450980392156865, 0.5098039215686274, 0.7058823529411765);
    impl_color!(forestgreen, 0.13333333333333333, 0.5450980392156862, 0.13333333333333333);
    impl_color!(tan, 0.8235294117647058, 0.7058823529411765, 0.5490196078431373);
    impl_color!(mediumvioletred, 0.7803921568627451, 0.08235294117647059, 0.5215686274509804);
    impl_color!(gainsboro, 0.8627450980392157, 0.8627450980392157, 0.8627450980392157);
    impl_color!(royalblue, 0.2549019607843137, 0.4117647058823529, 0.8823529411764706);
    impl_color!(seagreen, 0.1803921568627451, 0.5450980392156862, 0.3411764705882353);
    impl_color!(khaki, 0.9411764705882353, 0.9019607843137255, 0.5490196078431373);
    impl_color!(deeppink, 1.0, 0.0784313725490196, 0.5764705882352941);
    impl_color!(whitesmoke, 0.9607843137254902, 0.9607843137254902, 0.9607843137254902);
    impl_color!(midnightblue, 0.09803921568627451, 0.09803921568627451, 0.4392156862745098);
    impl_color!(mediumseagreen, 0.23529411764705882, 0.7019607843137254, 0.44313725490196076);
    impl_color!(yellow, 1.0, 1.0, 0.0);
    impl_color!(hotpink, 1.0, 0.4117647058823529, 0.7058823529411765);
    impl_color!(white, 1.0, 1.0, 1.0);
    impl_color!(navy, 0.0, 0.0, 0.5019607843137255);
    impl_color!(mediumaquamarine, 0.4, 0.803921568627451, 0.6666666666666666);
    impl_color!(gold, 1.0, 0.8431372549019608, 0.0);
    impl_color!(palevioletred, 0.8588235294117647, 0.4392156862745098, 0.5764705882352941);
    impl_color!(snow, 1.0, 0.9803921568627451, 0.9803921568627451);
    impl_color!(darkblue, 0.0, 0.0, 0.5450980392156862);
    impl_color!(darkseagreen, 0.5607843137254902, 0.7372549019607844, 0.5607843137254902);
    impl_color!(orange, 1.0, 0.6470588235294118, 0.0);
    impl_color!(pink, 1.0, 0.7529411764705882, 0.796078431372549);
    impl_color!(ghostwhite, 0.9725490196078431, 0.9725490196078431, 1.0);
    impl_color!(mediumblue, 0.0, 0.0, 0.803921568627451);
    impl_color!(aquamarine, 0.4980392156862745, 1.0, 0.8313725490196079);
    impl_color!(sandybrown, 0.9568627450980393, 0.6431372549019608, 0.3764705882352941);
    impl_color!(lightpink, 1.0, 0.7137254901960784, 0.7568627450980392);
    impl_color!(floralwhite, 1.0, 0.9803921568627451, 0.9411764705882353);
    impl_color!(blue, 0.0, 0.0, 1.0);
    impl_color!(palegreen, 0.596078431372549, 0.984313725490196, 0.596078431372549);
    impl_color!(darkorange, 1.0, 0.5490196078431373, 0.0);
    impl_color!(thistle, 0.8470588235294118, 0.7490196078431373, 0.8470588235294118);
    impl_color!(linen, 0.9803921568627451, 0.9411764705882353, 0.9019607843137255);
    impl_color!(dodgerblue, 0.11764705882352941, 0.5647058823529412, 1.0);
    impl_color!(lightgreen, 0.5647058823529412, 0.9333333333333333, 0.5647058823529412);
    impl_color!(goldenrod, 0.8549019607843137, 0.6470588235294118, 0.12549019607843137);
    impl_color!(magenta, 1.0, 0.0, 1.0);
    impl_color!(antiquewhite, 0.9803921568627451, 0.9215686274509803, 0.8431372549019608);
    impl_color!(cornflowerblue, 0.39215686274509803, 0.5843137254901961, 0.9294117647058824);
    impl_color!(springgreen, 0.0, 1.0, 0.4980392156862745);
    impl_color!(peru, 0.803921568627451, 0.5215686274509804, 0.24705882352941178);
    impl_color!(fuchsia, 1.0, 0.0, 1.0);
    impl_color!(papayawhip, 1.0, 0.9372549019607843, 0.8352941176470589);
    impl_color!(deepskyblue, 0.0, 0.7490196078431373, 1.0);
    impl_color!(mediumspringgreen, 0.0, 0.9803921568627451, 0.6039215686274509);
    impl_color!(darkgoldenrod, 0.7215686274509804, 0.5254901960784314, 0.043137254901960784);
    impl_color!(violet, 0.9333333333333333, 0.5098039215686274, 0.9333333333333333);
    impl_color!(blanchedalmond, 1.0, 0.9215686274509803, 0.803921568627451);
    impl_color!(lightskyblue, 0.5294117647058824, 0.807843137254902, 0.9803921568627451);
    impl_color!(lawngreen, 0.48627450980392156, 0.9882352941176471, 0.0);
    impl_color!(chocolate, 0.8235294117647058, 0.4117647058823529, 0.11764705882352941);
    impl_color!(plum, 0.8666666666666667, 0.6274509803921569, 0.8666666666666667);
    impl_color!(bisque, 1.0, 0.8941176470588236, 0.7686274509803922);
    impl_color!(skyblue, 0.5294117647058824, 0.807843137254902, 0.9215686274509803);
    impl_color!(chartreuse, 0.4980392156862745, 1.0, 0.0);
    impl_color!(sienna, 0.6274509803921569, 0.3215686274509804, 0.17647058823529413);
    impl_color!(orchid, 0.8549019607843137, 0.4392156862745098, 0.8392156862745098);
    impl_color!(moccasin, 1.0, 0.8941176470588236, 0.7098039215686275);
    impl_color!(lightblue, 0.6784313725490196, 0.8470588235294118, 0.9019607843137255);
    impl_color!(greenyellow, 0.6784313725490196, 1.0, 0.1843137254901961);
    impl_color!(saddlebrown, 0.5450980392156862, 0.27058823529411763, 0.07450980392156863);
    impl_color!(mediumorchid, 0.7294117647058823, 0.3333333333333333, 0.8274509803921568);
    impl_color!(navajowhite, 1.0, 0.8705882352941177, 0.6784313725490196);
    impl_color!(powderblue, 0.6901960784313725, 0.8784313725490196, 0.9019607843137255);
    impl_color!(lime, 0.0, 1.0, 0.0);
    impl_color!(maroon, 0.5019607843137255, 0.0, 0.0);
    impl_color!(darkorchid, 0.6, 0.19607843137254902, 0.8);
    impl_color!(peachpuff, 1.0, 0.8549019607843137, 0.7254901960784313);
    impl_color!(paleturquoise, 0.6862745098039216, 0.9333333333333333, 0.9333333333333333);
    impl_color!(limegreen, 0.19607843137254902, 0.803921568627451, 0.19607843137254902);
    impl_color!(darkred, 0.5450980392156862, 0.0, 0.0);
    impl_color!(darkviolet, 0.5803921568627451, 0.0, 0.8274509803921568);
    impl_color!(mistyrose, 1.0, 0.8941176470588236, 0.8823529411764706);
    impl_color!(lightcyan, 0.8784313725490196, 1.0, 1.0);
    impl_color!(yellowgreen, 0.6039215686274509, 0.803921568627451, 0.19607843137254902);
    impl_color!(brown, 0.6470588235294118, 0.16470588235294117, 0.16470588235294117);
    impl_color!(darkmagenta, 0.5450980392156862, 0.0, 0.5450980392156862);
    impl_color!(lavenderblush, 1.0, 0.9411764705882353, 0.9607843137254902);
    impl_color!(cyan, 0.0, 1.0, 1.0);
    impl_color!(darkolivegreen, 0.3333333333333333, 0.4196078431372549, 0.1843137254901961);
    impl_color!(firebrick, 0.6980392156862745, 0.13333333333333333, 0.13333333333333333);
    impl_color!(purple, 0.5019607843137255, 0.0, 0.5019607843137255);
    impl_color!(seashell, 1.0, 0.9607843137254902, 0.9333333333333333);
    impl_color!(aqua, 0.0, 1.0, 1.0);
    impl_color!(olivedrab, 0.4196078431372549, 0.5568627450980392, 0.13725490196078433);
    impl_color!(indianred, 0.803921568627451, 0.3607843137254902, 0.3607843137254902);
    impl_color!(indigo, 0.29411764705882354, 0.0, 0.5098039215686274);
    impl_color!(oldlace, 0.9921568627450981, 0.9607843137254902, 0.9019607843137255);
    impl_color!(turquoise, 0.25098039215686274, 0.8784313725490196, 0.8156862745098039);
    impl_color!(olive, 0.5019607843137255, 0.5019607843137255, 0.0);
    impl_color!(rosybrown, 0.7372549019607844, 0.5607843137254902, 0.5607843137254902);
    impl_color!(darkslateblue, 0.2823529411764706, 0.23921568627450981, 0.5450980392156862);
    impl_color!(ivory, 1.0, 1.0, 0.9411764705882353);
    impl_color!(mediumturquoise, 0.2823529411764706, 0.8196078431372549, 0.8);
    impl_color!(darkkhaki, 0.7411764705882353, 0.7176470588235294, 0.4196078431372549);
    impl_color!(darksalmon, 0.9137254901960784, 0.5882352941176471, 0.47843137254901963);
    impl_color!(blueviolet, 0.5411764705882353, 0.16862745098039217, 0.8862745098039215);
    impl_color!(honeydew, 0.9411764705882353, 1.0, 0.9411764705882353);
    impl_color!(darkturquoise, 0.0, 0.807843137254902, 0.8196078431372549);
    impl_color!(palegoldenrod, 0.9333333333333333, 0.9098039215686274, 0.6666666666666666);
    impl_color!(lightcoral, 0.9411764705882353, 0.5019607843137255, 0.5019607843137255);
    impl_color!(mediumpurple, 0.5764705882352941, 0.4392156862745098, 0.8588235294117647);
    impl_color!(mintcream, 0.9607843137254902, 1.0, 0.9803921568627451);
    impl_color!(lightseagreen, 0.12549019607843137, 0.6980392156862745, 0.6666666666666666);
    impl_color!(cornsilk, 1.0, 0.9725490196078431, 0.8627450980392157);
    impl_color!(salmon, 0.9803921568627451, 0.5019607843137255, 0.4470588235294118);
    impl_color!(slateblue, 0.41568627450980394, 0.35294117647058826, 0.803921568627451);
    impl_color!(azure, 0.9411764705882353, 1.0, 1.0);
    impl_color!(cadetblue, 0.37254901960784315, 0.6196078431372549, 0.6274509803921569);
    impl_color!(beige, 0.9607843137254902, 0.9607843137254902, 0.8627450980392157);
    impl_color!(lightsalmon, 1.0, 0.6274509803921569, 0.47843137254901963);
    impl_color!(mediumslateblue, 0.4823529411764706, 0.40784313725490196, 0.9333333333333333);
}
