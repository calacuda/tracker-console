class SideBar:
    def __init__(self, state, pg_state) -> None:
        # self.screen = pg_state.screen
        self.state = state
        self.log = pg_state.log
        # self.config = config
        (self.screen_width, self.screen_height) = pg_state.screen_size
        self.pg_state = pg_state

    def draw(self, i):
        """renders the side bar to the screen"""
        left_most = self.screen_width - \
            (self.screen_width * self.pg_state.config.ui.menu.width)

        bottom = self.draw_tempo(left_most)
        bottom = self.draw_notes(left_most, bottom)
        bottom = self.draw_osciloscope(left_most, bottom)
        self.draw_menu_map(i, left_most, bottom)

    def draw_tempo(self, left_most: float) -> float:
        """draws tempo display to the screen"""
        self.log.info("drawing tempo")
        bottom = self.screen_height * self.pg_state.config.ui.menu.tempo
        middle_x = (left_most + self.screen_width) * 0.5
        middle_y = bottom * 0.5
        color = self.pg_state.config.colors.text

        display = self.pg_state.fonts[0].render(
            f"TEMPO: {self.state.tempo}", True, color)
        textRect = display.get_rect()

        textRect.center = (middle_x, middle_y)

        self.pg_state.screen.blit(display, textRect)

        return bottom

    def draw_notes(self, left_most: float, top: float) -> float:
        """draws now playing display to the screen"""
        self.log.info("drawing note display")
        bottom = self.screen_height * self.pg_state.config.ui.menu.note_display
        middle_y = (top + bottom) * 0.5
        color = self.pg_state.config.colors.text
        width = self.screen_width - left_most
        note_width = width * 0.25

        for i, note in enumerate(self.state.playing):
            middle_x = (left_most + (note_width * 0.5) + (note_width * i))
            display = self.pg_state.fonts[1].render(
                f"{self.pg_state.display_note(note)}", True, color)
            textRect = display.get_rect()

            textRect.center = (middle_x, middle_y)

            self.pg_state.screen.blit(display, textRect)

        return bottom

    def draw_osciloscope(self, left_most: float, top: float) -> float:
        """draws osciloscope to the screen"""
        self.log.info("drawing osciloscope")
        bottom = self.screen_height * self.pg_state.config.ui.menu.osciloscope

        self.log.debug("osciloscope not implemented")

        return bottom

    def draw_menu_map(self, i, left_most: float, top: float):
        """draws menu map to the screen"""
        self.log.info("drawing menu map")
        self.pg_state.draw_tap_map(i, left_most, top)
