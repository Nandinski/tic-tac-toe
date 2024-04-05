
use ratatui::{layout::{Alignment, Constraint, Direction, Layout, Rect}, style::{Style, Styled, Stylize}, widgets::{Block, BorderType, Borders, Padding, Paragraph}, Frame};

use crate::app::App;

pub fn render(f: &mut Frame, app: &App) {
    let title = Block::new()
        .title(" Tic-Tac-Toe ").title_alignment(Alignment::Left)
        .borders(Borders::ALL )
        .border_type(BorderType::Rounded)
        .style(Style::new().white().on_black());
    f.render_widget(title, f.size());

    // Create the layout sections.
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(1), // Buffer title
            Constraint::Length(5), // Win message
            Constraint::Min(9),    // Grid size
            Constraint::Length(1), // Footer
        ])
        .margin(1)
        .split(f.size());
    
    let centered_chunk = center_chunk(CELL_WIDTH*3, CELL_HEIGHT*3, chunks[2]);
    render_grid(f, &app, centered_chunk);

    let quit_message = Paragraph::new("Press 'q' to exit, 'r' to restart")
        .alignment(Alignment::Right)
        .block(Block::default().padding(Padding::horizontal(1)))
        .style(Style::new().white().on_black());
    f.render_widget(quit_message, chunks[3]);
    
    if app.game_over {
        let popup_area = center_chunk(40, 5, chunks[1]);
        render_game_over_popup(f, app, popup_area);
        return;
    }

    let player = app.get_current_player();
    let player_turn = Paragraph::new(format!("Player {} turn. Press SPACE to play on selection", player.play_symbol))
        .alignment(Alignment::Left)
        .block(Block::default().padding(Padding::horizontal(1)))
        .style(Style::new().white().on_black());
    f.render_widget(player_turn, chunks[3]);
}

const CELL_WIDTH: u16 = 5;
const CELL_HEIGHT: u16 = 3;
fn render_grid(f: &mut Frame, app: &App, grid_area: Rect) {
    let rows = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(CELL_HEIGHT), 
            Constraint::Length(CELL_HEIGHT), 
            Constraint::Length(CELL_HEIGHT), 
            Constraint::Min(0), // Discard the rest 
        ])
        .split(grid_area);

    let grid: Vec<Vec<Rect>> = rows.iter().take(3) // ignore Min(0)
            .map(|area| {
                Layout::default()
                    .direction(Direction::Horizontal)
                    .constraints([
                        Constraint::Length(CELL_WIDTH), 
                        Constraint::Length(CELL_WIDTH), 
                        Constraint::Length(CELL_WIDTH), 
                        Constraint::Min(0), // Discard the rest 
                    ])
                    .split(*area)
                    .iter()
                    .take(3) // ignore Min(0)
                    .copied()
                    .collect::<Vec<Rect>>()
            }).collect();
    
    let mut cell_id = 0;
    for row in grid.iter() {
        for cell_area in row.iter() {
            render_cell(f, app, cell_id, cell_area);
            cell_id += 1;
        }
    }
}

fn render_cell(f: &mut Frame, app: &App, cell_id: u8, area: &Rect) {
    let player = app.get_current_player();
    let cell_selected = app.selected_cell_id == cell_id;
    let border_style = if cell_selected { Style::new().fg(player.symbol_color) } else {Style::new().white()};
    
    let cell_block = Block::default()
        .borders(Borders::ALL )
        .border_type(BorderType::Rounded)
        .border_style(border_style);
    
    let (play_symbol, symbol_color) = app.get_play(cell_id);
    let cell = Paragraph::new(play_symbol).fg(symbol_color).block(cell_block).alignment(Alignment::Center);
    f.render_widget(cell, *area);
}

fn render_game_over_popup(f: &mut Frame, app: &App, popup_area: Rect) {
    let popup = Block::new()
                                .title(" Game over ").title_alignment(Alignment::Center)
                                .borders(Borders::ALL )
                                .border_type(BorderType::Rounded)
                                .style(Style::new().black().on_light_yellow());

    f.render_widget(popup, popup_area);

    let game_over_msg_area = Layout::default()
                                        .direction(Direction::Vertical)
                                        .constraints([
                                            Constraint::Percentage(33),
                                            Constraint::Percentage(34),
                                            Constraint::Percentage(33),    
                                        ])
                                        .split(popup_area);

    let game_over_msg = if let Some(winner) = app.get_winner() {
        format!("Player {} won!", winner.play_symbol)
    } else {
        "It's a tie!".to_string()
    };

    let popup_msg = Paragraph::new(game_over_msg)
                                        .alignment(Alignment::Center)
                                        .style(Style::new().black().on_light_yellow());

    f.render_widget(popup_msg, game_over_msg_area[1]);

}
/// helper function to create a centered rect using up certain size of the available rect `r`
fn center_chunk(size_x: u16, size_y: u16, r: Rect) -> Rect {
    let x_margin = r.width.saturating_sub(size_x) / 2;
    let y_margin = r.height.saturating_sub(size_y) / 2;

    let y_centered = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(y_margin),
            Constraint::Length(size_y),
            Constraint::Length(y_margin),
        ])
        .split(r)[1];

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Length(x_margin),
            Constraint::Length(size_x),
            Constraint::Length(x_margin),
        ])
        .split(y_centered)[1]
}