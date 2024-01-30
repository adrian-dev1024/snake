use crate::game_context::{GameContext, GameState, Point};
use crate::GRID_X_SIZE;
use crate::GRID_Y_SIZE;
use crate::{DOT_SIZE_IN_PXS, FONT_PATH};
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::WindowCanvas;
use sdl2::ttf::Sdl2TtfContext;
use sdl2::video::Window;

pub struct Renderer {
    canvas: WindowCanvas,
    ttf_context: Sdl2TtfContext,
}

impl Renderer {
    pub fn new(window: Window) -> Result<Renderer, String> {
        let canvas = window.into_canvas().build().map_err(|e| e.to_string())?;
        let ttf_context = sdl2::ttf::init().map_err(|e| e.to_string())?;

        Ok(Renderer {
            canvas,
            ttf_context,
        })
    }

    fn draw_dot(&mut self, point: &Point) -> Result<(), String> {
        let Point(x, y) = point;
        self.canvas.fill_rect(Rect::new(
            x * DOT_SIZE_IN_PXS,
            y * DOT_SIZE_IN_PXS,
            DOT_SIZE_IN_PXS.try_into().unwrap(),
            DOT_SIZE_IN_PXS.try_into().unwrap(),
        ))?;

        Ok(())
    }

    pub fn draw(&mut self, context: &GameContext) -> Result<(), String> {
        if let GameState::OverYes | GameState::OverNo = context.state {
            self.draw_game_over_box(context)?;
        } else {
            self.draw_background(context);
            self.draw_player(context)?;
            self.draw_food(context)?;
            self.canvas.present();
        }

        Ok(())
    }

    fn draw_background(&mut self, context: &GameContext) {
        let color = match context.state {
            GameState::Playing => Color::BLACK,
            GameState::Paused => Color::GRAY,
            _ => Color::MAGENTA,
        };
        self.canvas.set_draw_color(color);
        self.canvas.clear();
    }

    fn draw_player(&mut self, context: &GameContext) -> Result<(), String> {
        self.canvas.set_draw_color(Color::GREEN);
        for point in &context.player_position {
            self.draw_dot(point)?;
        }

        Ok(())
    }

    fn draw_food(&mut self, context: &GameContext) -> Result<(), String> {
        self.canvas.set_draw_color(Color::RED);
        self.draw_dot(&context.food)?;
        Ok(())
    }

    fn draw_game_over_box(&mut self, context: &GameContext) -> Result<(), String> {
        self.canvas.set_draw_color(Color::BLUE);

        let center = sdl2::rect::Point::new(
            (GRID_X_SIZE / 2) * DOT_SIZE_IN_PXS,
            (GRID_Y_SIZE / 2) * DOT_SIZE_IN_PXS,
        );

        let bg_rect = Rect::from_center(
            center,
            (8 * DOT_SIZE_IN_PXS).try_into().unwrap(),
            (7 * DOT_SIZE_IN_PXS).try_into().unwrap(),
        );

        self.canvas.fill_rect(bg_rect)?;

        let go_rect = Rect::from_center(
            center - sdl2::rect::Point::new(0, 2 * DOT_SIZE_IN_PXS),
            (8 * DOT_SIZE_IN_PXS).try_into().unwrap(),
            (4 * DOT_SIZE_IN_PXS).try_into().unwrap(),
        );

        let go_text_box =
            TextBox::new(String::from("Game Over!"), go_rect, Color::BLUE, Color::RED);
        self.draw_text_box(&go_text_box)?;

        let pa_rect = Rect::from_center(
            center + sdl2::rect::Point::new(0, 1 * DOT_SIZE_IN_PXS),
            (6 * DOT_SIZE_IN_PXS).try_into().unwrap(),
            (2 * DOT_SIZE_IN_PXS).try_into().unwrap(),
        );

        let pa_text_box =
            TextBox::new(String::from("Play Again"), pa_rect, Color::BLUE, Color::RED);
        self.draw_text_box(&pa_text_box)?;

        let yn_rect = Rect::from_center(
            center + sdl2::rect::Point::new(0, 3 * DOT_SIZE_IN_PXS),
            (4 * DOT_SIZE_IN_PXS).try_into().unwrap(),
            (1 * DOT_SIZE_IN_PXS).try_into().unwrap(),
        );

        let yn_text = if let GameState::OverNo = context.state {
            String::from(" Yes  [No]")
        } else {
            String::from("[Yes]  No ")
        };

        let yn_text_box = TextBox::new_bold(yn_text, yn_rect, Color::BLUE, Color::RED);
        self.draw_text_box(&yn_text_box)?;

        self.canvas.present();
        Ok(())
    }

    fn draw_text_box(&mut self, text_box: &TextBox) -> Result<(), String> {
        let texture_creator = self.canvas.texture_creator();
        let mut font = self.ttf_context.load_font(FONT_PATH, 64)?;

        self.canvas.set_draw_color(text_box.background_color);
        self.canvas.fill_rect(text_box.rect)?;

        if text_box.bold {
            font.set_style(sdl2::ttf::FontStyle::BOLD);
        }

        let surface = font
            .render(&text_box.text)
            .blended(Color::RGBA(255, 0, 0, 255))
            .map_err(|e| e.to_string())?;

        let texture = texture_creator
            .create_texture_from_surface(&surface)
            .map_err(|e| e.to_string())?;

        self.canvas.set_draw_color(text_box.text_color);
        self.canvas.copy(&texture, None, Some(text_box.rect))?;
        Ok(())
    }
}

struct TextBox {
    text: String,
    rect: Rect,
    background_color: Color,
    text_color: Color,
    bold: bool,
}

impl TextBox {
    fn new(text: String, rect: Rect, background_color: Color, text_color: Color) -> TextBox {
        TextBox {
            text,
            rect,
            background_color,
            text_color,
            bold: false,
        }
    }

    fn new_bold(text: String, rect: Rect, background_color: Color, text_color: Color) -> TextBox {
        TextBox {
            text,
            rect,
            background_color,
            text_color,
            bold: true,
        }
    }
}
