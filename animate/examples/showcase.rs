#[cfg(feature = "ratatui")]
mod enabled {
    use animate::animate;
    use ratatui::Frame;
    use ratatui::{
        layout::{Constraint, Layout, Rect},
        style::{Color, Style, Stylize},
        symbols::Marker,
        widgets::{
            Axis, Bar, BarChart, BarGroup, Block, BorderType, Borders, Chart, Dataset, Gauge,
            GraphType, LineGauge, Paragraph, Sparkline,
            canvas::{Canvas, Points},
        },
    };
    use std::f64::consts::{PI, TAU};
    use std::time::{Duration, Instant};

    #[animate]
    struct Gauges {
        #[alternate(duration = 2500, easing = cubic_in_out)]
        g1: f64,
        #[alternate(duration = 2500, easing = quad_in_out)]
        g2: f64,
    }

    #[animate]
    struct Colors {
        #[once(duration = 2500, easing = quad_in_out)]
        color: Color,
    }

    #[animate]
    struct Bars {
        #[alternate(duration = 1250, easing = cubic_in_out)]
        m1: u64,
        #[alternate(duration = 2500, easing = quad_in_out)]
        m2: u64,
        #[alternate(duration = 1666, easing = cubic_in_out)]
        m3: u64,
        #[alternate(duration = 1250, easing = cubic_in_out)]
        m4: u64,
        #[alternate(duration = 1000, easing = quad_in_out)]
        m5: u64,
        #[alternate(duration = 833,  easing = cubic_in_out)]
        m6: u64,
        #[alternate(duration = 625,  easing = quad_in_out)]
        m7: u64,
        #[alternate(duration = 500,  easing = cubic_in_out)]
        m8: u64,
        #[alternate(duration = 400,  easing = linear)]
        m9: u64,
        #[alternate(duration = 312,  easing = quad_in_out)]
        m10: u64,
        #[alternate(duration = 2500, easing = cubic_in_out)]
        m11: u64,
        #[alternate(duration = 1250, easing = quad_in_out)]
        m12: u64,
    }

    #[animate]
    struct Spark {
        #[alternate(duration = 1000, easing = linear)]
        value: u64,
    }

    #[animate]
    struct Wave {
        #[cycle(duration = 2500, easing = linear)]
        offset: f64,
    }

    #[animate]
    struct Grid {
        #[alternate(duration = 5000, easing = cubic_in_out)]
        horizontal: u16,
        #[alternate(duration = 2500, easing = quad_in_out)]
        top: u16,
        #[alternate(duration = 1250, easing = cubic_in_out)]
        bottom: u16,
    }

    #[animate]
    struct Txt {
        #[alternate(duration = 5000, easing = quad_in_out)]
        text: String,
    }

    #[animate]
    struct PingPong {
        #[alternate(duration = 2500, easing = quad_in_out)]
        percent: f64,
    }

    #[animate]
    struct Circle {
        #[cycle(duration = 5000, easing = linear)]
        rotation: f64,
    }

    pub struct App {
        gauges: Gauges,
        colorscheme: Colors,
        bars: Bars,
        spark: Spark,
        wave: Wave,
        grid: Grid,
        text: Txt,
        pingpong: PingPong,
        circle: Circle,
        spark_data: Vec<u64>,
        colors: Vec<Color>,
        color_index: usize,
        color_timer: Instant,
    }

    impl App {
        pub fn new() -> Self {
            let mut app = Self {
                gauges: Gauges::new(0.0, 1.0),
                colorscheme: Colors::new(Color::Rgb(255, 80, 80)),
                bars: Bars::new(20, 30, 10, 40, 15, 25, 45, 10, 35, 20, 15, 30),
                spark: Spark::new(0),
                wave: Wave::new(0.0),
                grid: Grid::new(30, 30, 30),
                text: Txt::new(String::new()),
                pingpong: PingPong::new(0.0),
                circle: Circle::new(0.0),
                spark_data: Vec::with_capacity(1000),
                colors: vec![
                    Color::Rgb(255, 80, 80),
                    Color::Rgb(255, 180, 0),
                    Color::Rgb(80, 220, 120),
                    Color::Rgb(80, 160, 255),
                    Color::Rgb(180, 80, 255),
                ],
                color_index: 0,
                color_timer: Instant::now(),
            };

            app.gauges.g1.set(1.0);
            app.gauges.g2.set(0.0);
            app.colorscheme.color.set(app.colors[1]);

            app.bars.m1.set(100);
            app.bars.m2.set(85);
            app.bars.m3.set(95);
            app.bars.m4.set(70);
            app.bars.m5.set(90);
            app.bars.m6.set(60);
            app.bars.m7.set(80);
            app.bars.m8.set(110);
            app.bars.m9.set(50);
            app.bars.m10.set(75);
            app.bars.m11.set(95);
            app.bars.m12.set(65);

            app.spark.value.set(100);
            app.wave.offset.set(1.0);
            app.grid.horizontal.set(70);
            app.grid.top.set(70);
            app.grid.bottom.set(70);

            app.text.text.set(
            "Lorem ipsum dolor sit amet consectetur adipiscing elit. Amet consectetur adipiscing elit quisque faucibus ex sapien. Quisque faucibus ex sapien vitae pellentesque sem placerat."
                .into(),
        );
            app.pingpong.percent.set(1.0);
            app.circle.rotation.set(1.0);

            app
        }

        pub fn update(&mut self) {
            self.gauges.animate();
            self.colorscheme.animate();
            self.bars.animate();
            self.spark.animate();
            self.wave.animate();
            self.grid.animate();
            self.text.animate();
            self.pingpong.animate();
            self.circle.animate();

            if self.color_timer.elapsed() >= Duration::from_millis(2500) {
                self.color_timer = Instant::now();
                self.color_index = (self.color_index + 1) % self.colors.len();
                self.colorscheme.color.set(self.colors[self.color_index]);
            }

            self.spark_data.push(*self.spark.value);
            if self.spark_data.len() > 1000 {
                self.spark_data.remove(0);
            }
        }

        pub fn draw(&mut self, frame: &mut Frame) {
            let [top, bottom] =
                Layout::vertical([Constraint::Percentage(60), Constraint::Percentage(40)])
                    .areas(frame.area());
            let [r1, r2, r3, r4] = Layout::vertical([
                Constraint::Length(3),
                Constraint::Length(3),
                Constraint::Length(3),
                Constraint::Min(5),
            ])
            .areas(top);

            draw_text(frame, r1, &self.text, &self.colorscheme);

            let [left, right] =
                Layout::horizontal([Constraint::Fill(1), Constraint::Fill(1)]).areas(r2);
            draw_stat_gauge(frame, left, *self.gauges.g1, &self.colorscheme);
            draw_stat_line(frame, right, *self.gauges.g2, &self.colorscheme);

            draw_pingpong(frame, r3, &self.pingpong, &self.colorscheme);

            let [bar, chart, illusion, spark] = Layout::horizontal([
                Constraint::Percentage(12),
                Constraint::Fill(1),
                Constraint::Length(50),
                Constraint::Fill(2),
            ])
            .areas(r4);

            draw_bars(frame, bar, &self.bars, &self.colorscheme);
            draw_chart(frame, chart, &self.wave, &self.colorscheme);
            draw_circle(frame, illusion, &self.circle, &self.colorscheme);
            draw_spark(frame, spark, &self.spark_data, &self.colorscheme);

            draw_grid(frame, bottom, &self.grid, &self.colorscheme);
        }
    }

    fn draw_text(frame: &mut Frame, area: Rect, typewriter: &Txt, colors: &Colors) {
        let block = Block::new()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded);

        let text = Paragraph::new(format!("> {}█", *typewriter.text))
            .fg(*colors.color)
            .block(block);
        frame.render_widget(text, area);
    }

    fn draw_stat_gauge(frame: &mut Frame, area: Rect, val: f64, colors: &Colors) {
        let gauge = Gauge::default()
            .block(
                Block::new()
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded),
            )
            .gauge_style(Style::new().fg(*colors.color))
            .ratio(val);
        frame.render_widget(gauge, area);
    }

    fn draw_stat_line(frame: &mut Frame, area: Rect, val: f64, colors: &Colors) {
        let block = Block::new()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded);
        let inner = block.inner(area);
        frame.render_widget(block, area);

        let [_, center, _] = Layout::vertical([
            Constraint::Min(0),
            Constraint::Length(1),
            Constraint::Min(0),
        ])
        .areas(inner);
        let line = LineGauge::default()
            .filled_style(Style::new().fg(*colors.color))
            .unfilled_style(Style::new().fg(Color::DarkGray))
            .ratio(val);
        frame.render_widget(line, center);
    }

    fn draw_pingpong(frame: &mut Frame, area: Rect, pingpong: &PingPong, colors: &Colors) {
        let block = Block::new()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded);
        let inner = block.inner(area);
        frame.render_widget(block, area);

        if inner.width > 0 {
            let x = (*pingpong.percent * inner.width.saturating_sub(1) as f64) as usize;
            let [_, center, _] = Layout::vertical([
                Constraint::Length(inner.height / 2),
                Constraint::Length(1),
                Constraint::Min(0),
            ])
            .areas(inner);
            frame.render_widget(
                Paragraph::new(" ".repeat(x) + "●").fg(*colors.color).bold(),
                center,
            );
        }
    }

    fn draw_bars(frame: &mut Frame, area: Rect, metrics: &Bars, colors: &Colors) {
        let data = [
            ("a", *metrics.m1),
            ("b", *metrics.m2),
            ("c", *metrics.m3),
            ("d", *metrics.m4),
            ("e", *metrics.m5),
            ("f", *metrics.m6),
            ("g", *metrics.m7),
            ("h", *metrics.m8),
            ("i", *metrics.m9),
            ("j", *metrics.m10),
            ("k", *metrics.m11),
            ("l", *metrics.m12),
        ];

        let bars: Vec<Bar> = data
            .iter()
            .map(|(l, v)| Bar::default().value(*v).label(*l).text_value(""))
            .collect();

        let chart = BarChart::default()
            .block(
                Block::new()
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded),
            )
            .bar_width(3)
            .bar_gap(1)
            .max(110)
            .bar_style(Style::new().fg(*colors.color))
            .data(BarGroup::default().bars(&bars));
        frame.render_widget(chart, area);
    }

    fn draw_chart(frame: &mut Frame, area: Rect, wave: &Wave, colors: &Colors) {
        let offset = *wave.offset * TAU * 2.0;
        let pts: Vec<(f64, f64)> = (0..100)
            .map(|x| (x as f64, ((x as f64 / 8.0) + offset).sin() * 10.0 + 10.0))
            .collect();
        let dataset = Dataset::default()
            .marker(Marker::Braille)
            .graph_type(GraphType::Line)
            .style(Style::default().fg(*colors.color))
            .data(&pts);
        let chart = Chart::new(vec![dataset])
            .block(
                Block::new()
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded),
            )
            .x_axis(Axis::default().bounds([0.0, 100.0]))
            .y_axis(Axis::default().bounds([0.0, 20.0]));
        frame.render_widget(chart, area);
    }

    fn draw_circle(frame: &mut Frame, area: Rect, illusion: &Circle, colors: &Colors) {
        let rot = *illusion.rotation * TAU;
        let color = *colors.color;
        let canvas = Canvas::default()
            .block(
                Block::new()
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded),
            )
            .marker(Marker::Braille)
            .x_bounds([-1.2, 1.2])
            .y_bounds([-1.2, 1.2])
            .paint(move |ctx| {
                for i in 0..8 {
                    let angle = (i as f64) * PI / 8.0;
                    let r = (rot - angle).cos();
                    let x = r * angle.cos();
                    let y = r * angle.sin();
                    ctx.draw(&Points {
                        coords: &[(x, y), (x + 0.01, y + 0.01), (x - 0.01, y - 0.01)],
                        color,
                    });
                }
            });
        frame.render_widget(canvas, area);
    }

    fn draw_spark(frame: &mut Frame, area: Rect, data: &[u64], colors: &Colors) {
        let block = Block::new()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded);
        let width = block.inner(area).width as usize;
        let slice = &data[data.len().saturating_sub(width)..];
        frame.render_widget(
            Sparkline::default()
                .block(block)
                .max(100)
                .fg(*colors.color)
                .data(slice),
            area,
        );
    }

    fn draw_grid(frame: &mut Frame, area: Rect, state: &Grid, colors: &Colors) {
        let [t_row, b_row] = Layout::vertical([
            Constraint::Percentage(*state.horizontal),
            Constraint::Percentage(100u16.saturating_sub(*state.horizontal)),
        ])
        .areas(area);
        let [q1, q2] = Layout::horizontal([
            Constraint::Percentage(*state.top),
            Constraint::Percentage(100u16.saturating_sub(*state.top)),
        ])
        .areas(t_row);
        let [q3, q4] = Layout::horizontal([
            Constraint::Percentage(*state.bottom),
            Constraint::Percentage(100u16.saturating_sub(*state.bottom)),
        ])
        .areas(b_row);

        let color = *colors.color;
        frame.render_widget(
            Block::new()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .fg(color),
            q1,
        );
        frame.render_widget(
            Block::new()
                .borders(Borders::ALL)
                .border_type(BorderType::Double)
                .fg(Color::DarkGray),
            q2,
        );
        frame.render_widget(
            Block::new()
                .borders(Borders::ALL)
                .border_type(BorderType::Thick)
                .fg(Color::DarkGray),
            q3,
        );
        frame.render_widget(
            Block::new()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .fg(color),
            q4,
        );
    }

    pub fn run() -> Result<(), Box<dyn std::error::Error>> {
        let mut app = App::new();
        let frame_time = Duration::from_micros(16666);

        ratatui::run(|terminal| {
            for _ in 0..600 {
                animate::tick(16);
                app.update();
                terminal.draw(|f| app.draw(f)).unwrap();
                std::thread::sleep(frame_time);
            }
        });
        Ok(())
    }
}

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    #[cfg(feature = "ratatui")]
    enabled::run().ok();

    #[cfg(not(feature = "ratatui"))]
    {
        println!("this example requires the ratatui feature");
    }

    Ok(())
}
