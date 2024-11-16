from tracker_backend import SongRow


class SongTab:
    def __init__(self, state, pg_state) -> None:
        # self.screen = pg_state.screen
        self.state = state
        self.log = pg_state.log
        # self.config = config
        (self.screen_width, self.screen_height) = pg_state.screen_size
        self.pg_state = pg_state

    def draw(self):
        right_most = (self.screen_width * self.pg_state.config.ui.tab.width)
        height = (self.screen_height * self.pg_state.config.ui.tab.row_height)
        col_width = right_most * self.pg_state.config.ui.tab.row_elm_width

        self.draw_tab_lable(right_most, height)
        self.draw_col_lable(height, col_width)
        # print(dir(self.state.screen._0.rows))
        self.draw_rows(self.state.screen._0.rows, height, col_width)

    def draw_tab_lable(self, right_most: float, height: float):
        middle_x = right_most * 0.5
        middle_y = height * 0.5
        color = self.pg_state.config.colors.text

        display = self.pg_state.fonts[0].render(
            f"Song", True, color)
        textRect = display.get_rect()

        textRect.center = (middle_x, middle_y)

        self.pg_state.screen.blit(display, textRect)

    def draw_col_lable(self, height: float, col_width: float):
        color = self.pg_state.config.colors.text
        middle_y = (height * 3.0) * 0.5

        for i, lable in enumerate(["", "LD-1", "LD-2", "BASS", "PERC"]):
            middle_x = ((col_width * 0.5) + (col_width * i))
            display = self.pg_state.fonts[1].render(
                lable, True, color)
            textRect = display.get_rect()

            textRect.center = (middle_x, middle_y)

            self.pg_state.screen.blit(display, textRect)

    def draw_rows(self, rows: list[SongRow], height: float, col_width: float):
        color = self.pg_state.config.colors.text

        for row_i, row in enumerate(rows):
            bottom = (height * 3.0) + height * row_i
            middle_y = bottom - height * 0.5

            for col_i, lable in enumerate([row_i, row.lead_1, row.lead_2, row.bass, row.perc]):
                text = f"{lable:02X}" if lable is not None else "--"

                middle_x = ((col_width * 0.5) + (col_width * col_i))
                display = self.pg_state.fonts[1].render(
                    text, True, color)
                textRect = display.get_rect()

                textRect.center = (middle_x, middle_y)

                if row_i == self.state.display_cursor.row and col_i - 1 == self.state.display_cursor.col and self.state.display_cursor.selected:
                    self.pg_state.draw_rect(
                        (middle_x, middle_y), (col_width, height), self.pg_state.config.colors.cursor)
                elif row_i == self.state.display_cursor.row and col_i - 1 == self.state.display_cursor.col:
                    self.pg_state.draw_rect(
                        (middle_x, middle_y), (col_width, height), self.pg_state.config.colors.cursor)
                    self.pg_state.draw_rect(
                        (middle_x, middle_y), (col_width - 5, height - 5), self.pg_state.config.colors.back_ground)

                self.pg_state.screen.blit(display, textRect)
