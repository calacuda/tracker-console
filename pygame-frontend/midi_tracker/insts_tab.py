class InstsTab:
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

        self.draw_tab_lable(right_most, height)

        self.tmp_view(right_most, self.screen_height - height)

    def draw_tab_lable(self, right_most: float, height: float):
        middle_x = right_most * 0.5
        middle_y = height * 0.5
        color = self.pg_state.config.colors.text
        n = self.state.screen._0.name

        display = self.pg_state.fonts[0].render(
            f"Instrument {n:02X}", True, color)
        textRect = display.get_rect()

        textRect.center = (middle_x, middle_y)

        self.pg_state.screen.blit(display, textRect)

    def tmp_view(self, right_most: float, height: float):
        middle_x = right_most * 0.5
        middle_y = height * 0.5
        color = self.pg_state.config.colors.text

        display = self.pg_state.fonts[0].render(
            "Under Construction", True, color)
        textRect = display.get_rect()

        textRect.center = (middle_x, middle_y)

        self.pg_state.screen.blit(display, textRect)
