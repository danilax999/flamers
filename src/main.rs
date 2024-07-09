use clap::Parser;
use crossterm::{
    cursor, event,
    style::{self, Color},
    terminal,
};
use std::io::Write;

fn main() -> std::io::Result<()> {
    let Params {
        mut zoom,
        mut scale,
        mut detail,
        mut speed,
        mut offset,
        fps,
        gradient,
    } = Params::parse();
    let frame_rate = 10u128.pow(6) / fps as u128; // Frame rate in microseconds

    let mut stdout =
        std::io::BufWriter::with_capacity(1024 * 1024, std::io::stdout().lock());

    terminal::enable_raw_mode()?;
    crossterm::queue!(
        stdout,
        terminal::EnterAlternateScreen,
        terminal::Clear(terminal::ClearType::All),
        cursor::Hide,
        cursor::MoveTo(0, 0)
    )?;

    let (mut width, mut height) = terminal::size().unwrap();

    loop {
        crossterm::queue!(stdout, cursor::MoveTo(0, 0))?;
        let start = std::time::SystemTime::now();
        let t = start
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_millis();
        let ratio = width as f64 / height as f64;
        for i in 0..height {
            for j in 0..width {
                let x = j as f64 / (width - 1) as f64 * zoom * ratio;
                let y = (i as f64 / (height - 1) as f64) * zoom * 2.1;
                let yt = y + (t % u32::MAX as u128) as f64 * speed;
                let p = perlin_ext(x, yt, scale, detail);
                let value =
                    ((p + p.powi(2) + 0.03) * (y + offset) * 2.).clamp(0., 1.);
                let (r, g, b) = mix_rgb(1.0 - value, &gradient);
                let color = Color::Rgb { r, g, b };

                crossterm::queue!(
                    stdout,
                    style::SetBackgroundColor(color),
                    style::Print(' ')
                )?;
            }
            if i < height - 1 {
                write!(stdout, "\n\r")?;
            }
        }
        stdout.flush()?;

        let duration = std::time::SystemTime::now()
            .duration_since(start)
            .unwrap()
            .as_micros();
        let pause = frame_rate.saturating_sub(duration);
        if event::poll(std::time::Duration::from_micros(pause as u64))? {
            use event::KeyCode::{self, Char};
            match event::read()? {
                event::Event::Key(key) => match key.code {
                    Char('q') => break,
                    Char('-') => zoom *= 1.25,
                    Char('+') => zoom /= 1.25,
                    Char('s') => scale = (scale - 0.25).max(0.),
                    Char('S') => scale += 0.25,
                    Char('d') => detail += 1,
                    Char('D') => detail = (detail - 1).max(1),
                    Char('e') => speed += 1e-4f64,
                    Char('E') => speed = (speed - 1e-4f64).max(0.),
                    KeyCode::Up | Char('k') => offset -= 0.1,
                    KeyCode::Down | Char('j') => offset += 0.1,
                    _ => (),
                },
                event::Event::Resize(width1, height1) => {
                    width = width1;
                    height = height1;
                }
                _ => (),
            }
        }
    }

    crossterm::queue!(stdout, terminal::LeaveAlternateScreen, cursor::Show)?;
    terminal::disable_raw_mode()?;

    Ok(())
}

fn perlin_ext(x: f64, y: f64, scale: f64, octaves: usize) -> f64 {
    assert!(octaves > 0);
    (1..=octaves)
        .map(|i| scale * (1u32 << (i - 1)) as f64)
        .map(|i| perlin(x * i, y * i))
        .reduce(|a, b| a + b)
        .unwrap()
        / octaves as f64
}

fn perlin(x: f64, y: f64) -> f64 {
    let x0 = x as u64;
    let x1 = x0 + 1;
    let sx = x.fract();

    let y0 = y as u64;
    let y1 = y0 + 1;
    let sy = y.fract();

    let n0 = dot_gradient(x0, y0, x, y);
    let n1 = dot_gradient(x1, y0, x, y);
    let ix0 = smoothstep(n0, n1, sx);

    let n0 = dot_gradient(x0, y1, x, y);
    let n1 = dot_gradient(x1, y1, x, y);
    let ix1 = smoothstep(n0, n1, sx);

    smoothstep(ix0, ix1, sy) * 0.5 + 0.5
}

fn dot_gradient(ix: u64, iy: u64, x: f64, y: f64) -> f64 {
    let (gx, gy) = rand_gradient(ix, iy);
    let dx = x - ix as f64;
    let dy = y - iy as f64;
    gx * dx + gy * dy
}

fn rand_gradient(ix: u64, iy: u64) -> (f64, f64) {
    let s = 32;
    let mut a = ix;
    let mut b = iy;

    a = a.wrapping_mul(3284157443);
    b ^= a << s | a >> s;
    b = b.wrapping_mul(1911520717);
    a ^= b << s | b >> s;
    a = a.wrapping_mul(2048419325);

    let random = (a as f64 / (1u64 << 63) as f64) * std::f64::consts::PI;

    (random.cos(), random.sin())
}

fn smoothstep(a: f64, b: f64, w: f64) -> f64 {
    (b - a) * ((w * (w * 6.0 - 15.0) + 10.0) * w * w * w) + a
}

fn lerp(a: f64, b: f64, w: f64) -> f64 {
    (b - a) * w + a
}

type Rgb = (u8, u8, u8);

fn mix_rgb(n: f64, gradient: &[Rgb]) -> Rgb {
    let l = gradient.len();
    assert!(l >= 1);
    let a = n * (l - 1) as f64;
    let i = a as usize;
    let f = a.fract();
    let color1 = gradient[i];
    let color2 = gradient[(i + 1).min(l - 1)];
    (
        lerp(color1.0 as f64, color2.0 as f64, f) as u8,
        lerp(color1.1 as f64, color2.1 as f64, f) as u8,
        lerp(color1.2 as f64, color2.2 as f64, f) as u8,
    )
}

#[derive(clap::Parser)]
#[command(
    about = "Fire simulation in terminal",
    after_help = "\
Values:
  ZOOM      Integer > 0
  SCALE     Float >= 0
  DETAIL    Integer > 0
  SPEED     Float >= 0
  OFFSET    Float
  FPS       Integer > 0
  GRADIENT  Simicolon-separated string of hex colors in the format `#RRGGBB`.
            Example values:
              Orange: #FFFF64;#FFBE1E;#FF9600;#FF5000;#B45000;#503C28;#28281E;#000000
              Blue:   #64FFFF;#1EBEFF;#0096FF;#0050FF;#0050B4;#283C50;#1E2828;#000000
              Green:  #78FF96;#1EFF5A;#00C850;#00B446;#00963C;#285028;#1E281E;#000000

Available keymaps:
  q          Quit
  +          Zoom in
  -          Zoom out
  s          Decrease scale
  S          Increase scale
  d          Increase detialization level
  D          Decrease detialization level
  e          Increase speed
  E          Descrease speed
  <Up>, k    Decrease vertical offset (Scroll up)
  <Down>, j  Increase vertical offset (Scroll down)
"
)]
struct Params {
    #[arg(short, long,
        help = "Set zoom level",
        value_parser = parse_zoom_level,
        default_value = "0.28")]
    zoom: f64,

    #[arg(short, long,
        help = "Set fire simulation scale",
        value_parser = parse_scale,
        default_value = "3.0")]
    scale: f64,

    #[arg(short, long,
        help = "Set fire simulation detalization level",
        value_parser = parse_detail,
        default_value = "3")]
    detail: usize,

    #[arg(
        short = 'p',
        long,
        value_parser = parse_speed,
        help = "Set fire simulation speed",
        default_value = "0.0006",
    )]
    speed: f64,

    #[arg(short, long, help = "Set vertical offset", default_value = "0.0")]
    offset: f64,

    #[arg(short, long,
        help = "Set maximum frames per second",
        value_parser = parse_fps,
        default_value = "60")]
    fps: usize,

    #[arg(short, long,
        help="Set flame gradient",
        value_parser = parse_gradient,
        default_value =
        "#FFFF64;#FFBE1E;#FF9600;#FF5000;#B45000;#503C28;#28281E;#000000")]
    gradient: std::vec::Vec<Rgb>,
}

type ParseResult<T> = Result<T, String>;

fn parse_zoom_level(input: &str) -> ParseResult<f64> {
    let n = input.parse::<f64>().map_err(|e| e.to_string())?;
    if n < 0. {
        Err(format!("Zoom level expected to be > 0, got `{n}`"))
    } else {
        Ok(n)
    }
}

fn parse_scale(input: &str) -> ParseResult<f64> {
    let n = input.parse::<f64>().map_err(|e| e.to_string())?;
    if n < 0. {
        Err(format!("Scale expected to be > 0, got `{n}`"))
    } else {
        Ok(n)
    }
}

fn parse_detail(input: &str) -> ParseResult<usize> {
    let n = input.parse::<usize>().map_err(|e| e.to_string())?;
    if n == 0 {
        Err(format!("Detail expected to be > 0, got `{n}`"))
    } else {
        Ok(n)
    }
}

fn parse_speed(input: &str) -> ParseResult<f64> {
    let n = input.parse::<f64>().map_err(|e| e.to_string())?;
    if n < 0. {
        Err(format!("Speed expected to be >= 0, got `{n}`"))
    } else {
        Ok(n)
    }
}

fn parse_fps(input: &str) -> ParseResult<usize> {
    let n = input.parse::<usize>().map_err(|e| e.to_string())?;
    if n == 0 {
        Err(format!("FPS expected to be > 0, got `{n}`"))
    } else {
        Ok(n)
    }
}

fn parse_gradient(input: &str) -> ParseResult<Vec<Rgb>> {
    input
        .split(';')
        .map(|col| -> ParseResult<Rgb> {
            if let Some(col) = col.strip_prefix('#') {
                u32::from_str_radix(col, 16)
                    .map_err(|_| format!("Hex color parsing error: {col}"))
                    .map(|n| {
                        (
                            (n >> 16 & 0xff) as u8,
                            (n >> 8 & 0xff) as u8,
                            (n & 0xff) as u8,
                        )
                    })
            } else {
                Err(format!("Unsupported color format: {col}"))
            }
        })
        .collect::<ParseResult<Vec<Rgb>>>()
        .and_then(|gradient| {
            if gradient.is_empty() {
                Err("Gradient must contain at least 1 color".to_string())
            } else {
                Ok(gradient)
            }
        })
}
