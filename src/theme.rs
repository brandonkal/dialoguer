//! Customizes the rendering of the elements.
use std::fmt;
use std::io;

use console::{Style, StyledObject, Term};

/// Rendering style for a selected item
#[derive(Debug, Clone, Copy)]
pub enum SelectionStyle {
    /// Renders an unchecked but selected checkbox
    CheckboxUncheckedSelected,
    /// Renders an unchecked and unselected checkbox
    CheckboxUncheckedUnselected,
    /// Renders a checked but selected checkbox
    CheckboxCheckedSelected,
    /// Renders a checked and unselected checkbox
    CheckboxCheckedUnselected,
    /// Renders a selected menu item
    MenuSelected,
    /// Renders un unselected menu item
    MenuUnselected,
}

/// Implements a theme for dialoguer.
pub trait Theme {
    /// Given a prompt this formats out what the prompt should look like (multiline).
    fn format_prompt(&self, f: &mut dyn fmt::Write, prompt: &str) -> fmt::Result {
        write!(f, "{}:", prompt)
    }

    /// Given a prompt this formats out what the prompt should look like (singleline).
    fn format_singleline_prompt(
        &self,
        f: &mut dyn fmt::Write,
        prompt: &str,
        default: Option<&str>,
    ) -> fmt::Result {
        match default {
            Some(default) => write!(f, "{} [{}]: ", prompt, default),
            None => write!(f, "{}: ", prompt),
        }
    }

    /// Formats out an error.
    fn format_error(&self, f: &mut dyn fmt::Write, err: &str) -> fmt::Result {
        write!(f, "error: {}", err)
    }

    /// Formats a confirmation prompt.
    fn format_confirmation_prompt(
        &self,
        f: &mut dyn fmt::Write,
        prompt: &str,
        default: Option<bool>,
    ) -> fmt::Result {
        write!(f, "{}", &prompt)?;
        match default {
            None => {}
            Some(true) => write!(f, " [Y/n] ")?,
            Some(false) => write!(f, " [y/N] ")?,
        }
        Ok(())
    }

    /// Formats a key prompt.
    fn format_key_prompt(
        &self,
        f: &mut dyn fmt::Write,
        prompt: &str,
        default: Option<u8>,
        choices: &Vec<char>,
    ) -> fmt::Result {
        write!(f, "{}", &prompt)?;
        let strs = self._format_key_prompt(default, choices);
        write!(f, " [{}] ", strs)?;
        Ok(())
    }

    fn _format_key_prompt(&self, default: Option<u8>, choices: &Vec<char>) -> String {
        let num = default.unwrap_or(100) as usize;
        let choices = choices.clone();
        let mut strs = "".to_string();
        for (pos, choice) in choices.iter().enumerate() {
            if pos == num {
                strs.push(choice.to_ascii_uppercase());
            } else {
                strs.push(*choice);
            }
            if pos != choices.len() - 1 {
                strs.push('/');
            }
        }
        strs
    }

    /// Formats a confirmation prompt.
    fn format_confirmation_prompt_selection(
        &self,
        f: &mut dyn fmt::Write,
        prompt: &str,
        selection: bool,
    ) -> fmt::Result {
        write!(f, "{} {}", &prompt, if selection { "yes" } else { "no" })
    }

    /// Renders a prompt and a single selection made.
    fn format_single_prompt_selection(
        &self,
        f: &mut dyn fmt::Write,
        prompt: &str,
        sel: &str,
    ) -> fmt::Result {
        write!(f, "{}: {}", prompt, sel)
    }

    /// Renders a prompt and multiple selections,
    fn format_multi_prompt_selection(
        &self,
        f: &mut dyn fmt::Write,
        prompt: &str,
        selections: &[&str],
    ) -> fmt::Result {
        write!(f, "{}: ", prompt)?;
        for (idx, sel) in selections.iter().enumerate() {
            write!(f, "{}{}", if idx == 0 { "" } else { ", " }, sel)?;
        }
        Ok(())
    }

    /// Renders a prompt and multiple selections,
    fn format_password_prompt_selection(
        &self,
        f: &mut dyn fmt::Write,
        prompt: &str,
    ) -> fmt::Result {
        self.format_single_prompt_selection(f, prompt, "[hidden]")
    }

    /// Formats a selection.
    fn format_selection(
        &self,
        f: &mut dyn fmt::Write,
        text: &str,
        style: SelectionStyle,
    ) -> fmt::Result {
        write!(
            f,
            "{}{}",
            match style {
                SelectionStyle::CheckboxUncheckedSelected => "> [ ] ",
                SelectionStyle::CheckboxUncheckedUnselected => "  [ ] ",
                SelectionStyle::CheckboxCheckedSelected => "> [x] ",
                SelectionStyle::CheckboxCheckedUnselected => "  [x] ",
                SelectionStyle::MenuSelected => "> ",
                SelectionStyle::MenuUnselected => "  ",
            },
            text
        )
    }
}

/// The default theme.
pub struct SimpleTheme;

impl Theme for SimpleTheme {}
/// The default theme, with a custom prompt character in place of `:`
pub struct CustomPromptCharacterTheme {
    prompt_character: char,
}
impl CustomPromptCharacterTheme {
    /// Creates a theme, the prompt character for which is customized
    pub fn new(character: char) -> CustomPromptCharacterTheme {
        CustomPromptCharacterTheme {
            prompt_character: character,
        }
    }
}
impl Default for CustomPromptCharacterTheme {
    fn default() -> Self {
        CustomPromptCharacterTheme {
            prompt_character: ':',
        }
    }
}
impl Theme for CustomPromptCharacterTheme {
    /// Given a prompt this formats out what the prompt should look like (multiline).
    fn format_prompt(&self, f: &mut dyn fmt::Write, prompt: &str) -> fmt::Result {
        write!(f, "{}{}", prompt, self.prompt_character)
    }

    /// Given a prompt this formats out what the prompt should look like (singleline).
    fn format_singleline_prompt(
        &self,
        f: &mut dyn fmt::Write,
        prompt: &str,
        default: Option<&str>,
    ) -> fmt::Result {
        match default {
            Some(default) => write!(f, "{} [{}]{} ", prompt, default, self.prompt_character),
            None => write!(f, "{}{} ", prompt, self.prompt_character),
        }
    }
    /// Renders a prompt and a single selection made.
    fn format_single_prompt_selection(
        &self,
        f: &mut dyn fmt::Write,
        prompt: &str,
        sel: &str,
    ) -> fmt::Result {
        write!(f, "{}{} {}", prompt, self.prompt_character, sel)
    }

    /// Renders a prompt and multiple selections,
    fn format_multi_prompt_selection(
        &self,
        f: &mut dyn fmt::Write,
        prompt: &str,
        selections: &[&str],
    ) -> fmt::Result {
        write!(f, "{}{} ", prompt, self.prompt_character)?;
        for (idx, sel) in selections.iter().enumerate() {
            write!(f, "{}{}", if idx == 0 { "" } else { ", " }, sel)?;
        }
        Ok(())
    }
}
/// A colorful theme
pub struct ColorfulTheme {
    /// The style for default values in prompts and similar
    pub defaults_style: Style,
    /// The style for errors indicators
    pub error_style: Style,
    /// The style for user interface indicators
    pub indicator_style: Style,
    /// The style for inactive elements
    pub inactive_style: Style,
    /// The style for active elements
    pub active_style: Style,
    /// The style for values indicating "yes"
    pub yes_style: Style,
    /// The style for values indicating "no"
    pub no_style: Style,
    /// The style for values embedded in prompts
    pub values_style: Style,
}

impl Default for ColorfulTheme {
    fn default() -> ColorfulTheme {
        ColorfulTheme {
            defaults_style: Style::new().dim(),
            error_style: Style::new().red(),
            indicator_style: Style::new().cyan().bold(),
            inactive_style: Style::new().dim(),
            active_style: Style::new(),
            yes_style: Style::new().green(),
            no_style: Style::new().green(),
            values_style: Style::new().cyan(),
        }
    }
}

impl Theme for ColorfulTheme {
    fn format_prompt(&self, f: &mut dyn fmt::Write, prompt: &str) -> fmt::Result {
        write!(f, "{}:", prompt)
    }

    fn format_singleline_prompt(
        &self,
        f: &mut dyn fmt::Write,
        prompt: &str,
        default: Option<&str>,
    ) -> fmt::Result {
        match default {
            Some(default) => write!(
                f,
                "{} [{}]: ",
                prompt,
                self.defaults_style.apply_to(default)
            ),
            None => write!(f, "{}: ", prompt),
        }
    }

    fn format_error(&self, f: &mut dyn fmt::Write, err: &str) -> fmt::Result {
        write!(f, "{}: {}", self.error_style.apply_to("error"), err)
    }

    fn format_confirmation_prompt(
        &self,
        f: &mut dyn fmt::Write,
        prompt: &str,
        default: Option<bool>,
    ) -> fmt::Result {
        write!(f, "{}", &prompt)?;
        match default {
            None => {}
            Some(true) => write!(f, " {} ", self.defaults_style.apply_to("[Y/n]"))?,
            Some(false) => write!(f, " {} ", self.defaults_style.apply_to("[y/N]"))?,
        }
        Ok(())
    }

    fn format_confirmation_prompt_selection(
        &self,
        f: &mut dyn fmt::Write,
        prompt: &str,
        selection: bool,
    ) -> fmt::Result {
        write!(
            f,
            "{} {}",
            &prompt,
            if selection {
                self.yes_style.apply_to("yes")
            } else {
                self.no_style.apply_to("no")
            }
        )
    }

    fn format_single_prompt_selection(
        &self,
        f: &mut dyn fmt::Write,
        prompt: &str,
        sel: &str,
    ) -> fmt::Result {
        write!(f, "{}: {}", prompt, self.values_style.apply_to(sel))
    }

    fn format_multi_prompt_selection(
        &self,
        f: &mut dyn fmt::Write,
        prompt: &str,
        selections: &[&str],
    ) -> fmt::Result {
        write!(f, "{}: ", prompt)?;
        for (idx, sel) in selections.iter().enumerate() {
            write!(
                f,
                "{}{}",
                if idx == 0 { "" } else { ", " },
                self.values_style.apply_to(sel)
            )?;
        }
        Ok(())
    }

    fn format_selection(
        &self,
        f: &mut dyn fmt::Write,
        text: &str,
        st: SelectionStyle,
    ) -> fmt::Result {
        match st {
            SelectionStyle::CheckboxUncheckedSelected => write!(
                f,
                "{} [ ] {}",
                self.indicator_style.apply_to(">"),
                self.active_style.apply_to(text)
            ),
            SelectionStyle::CheckboxUncheckedUnselected => {
                write!(f, "  [ ] {}", self.inactive_style.apply_to(text))
            }
            SelectionStyle::CheckboxCheckedSelected => write!(
                f,
                "{} [{}] {}",
                self.indicator_style.apply_to(">"),
                self.indicator_style.apply_to("x"),
                self.active_style.apply_to(text),
            ),
            SelectionStyle::CheckboxCheckedUnselected => write!(
                f,
                "  [{}] {}",
                self.indicator_style.apply_to("x"),
                self.inactive_style.apply_to(text)
            ),
            SelectionStyle::MenuSelected => write!(
                f,
                "{} {}",
                self.indicator_style.apply_to(">"),
                self.active_style.apply_to(text)
            ),
            SelectionStyle::MenuUnselected => write!(f, "  {}", self.inactive_style.apply_to(text)),
        }
    }
}

/// Helper struct to conveniently render a theme to a term.
pub(crate) struct TermThemeRenderer<'a> {
    term: &'a Term,
    theme: &'a dyn Theme,
    height: usize,
    prompt_height: usize,
    prompts_reset_height: bool,
}

impl<'a> TermThemeRenderer<'a> {
    pub fn new(term: &'a Term, theme: &'a dyn Theme) -> TermThemeRenderer<'a> {
        TermThemeRenderer {
            term,
            theme,
            height: 0,
            prompt_height: 0,
            prompts_reset_height: true,
        }
    }

    pub fn set_prompts_reset_height(&mut self, val: bool) {
        self.prompts_reset_height = val;
    }

    pub fn term(&self) -> &Term {
        self.term
    }

    pub fn add_line(&mut self) {
        self.height += 1;
    }

    fn write_formatted_str<
        F: FnOnce(&mut TermThemeRenderer, &mut dyn fmt::Write) -> fmt::Result,
    >(
        &mut self,
        f: F,
    ) -> io::Result<()> {
        let mut buf = String::new();
        f(self, &mut buf).map_err(|err| io::Error::new(io::ErrorKind::Other, err))?;
        self.height += buf.chars().filter(|&x| x == '\n').count();
        self.term.write_str(&buf)
    }

    fn write_formatted_line<
        F: FnOnce(&mut TermThemeRenderer, &mut dyn fmt::Write) -> fmt::Result,
    >(
        &mut self,
        f: F,
    ) -> io::Result<()> {
        let mut buf = String::new();
        f(self, &mut buf).map_err(|err| io::Error::new(io::ErrorKind::Other, err))?;
        self.height += buf.chars().filter(|&x| x == '\n').count() + 1;
        self.term.write_line(&buf)
    }

    fn write_formatted_prompt<
        F: FnOnce(&mut TermThemeRenderer, &mut dyn fmt::Write) -> fmt::Result,
    >(
        &mut self,
        f: F,
    ) -> io::Result<()> {
        self.write_formatted_line(f)?;
        if self.prompts_reset_height {
            self.prompt_height = self.height;
            self.height = 0;
        }
        Ok(())
    }

    pub fn error(&mut self, err: &str) -> io::Result<()> {
        self.write_formatted_line(|this, buf| this.theme.format_error(buf, err))
    }

    pub fn prompt(&mut self, prompt: &str) -> io::Result<()> {
        self.write_formatted_prompt(|this, buf| this.theme.format_prompt(buf, prompt))
    }

    pub fn input_prompt(&mut self, prompt: &str, default: Option<&str>) -> io::Result<()> {
        self.write_formatted_str(|this, buf| {
            this.theme.format_singleline_prompt(buf, prompt, default)
        })
    }

    pub fn password_prompt(&mut self, prompt: &str) -> io::Result<()> {
        self.write_formatted_str(|this, buf| {
            write!(buf, "\r")?;
            this.theme.format_singleline_prompt(buf, prompt, None)
        })
    }

    pub fn confirmation_prompt(&mut self, prompt: &str, default: Option<bool>) -> io::Result<()> {
        self.write_formatted_str(|this, buf| {
            this.theme.format_confirmation_prompt(buf, prompt, default)
        })
    }

    pub fn key_prompt(
        &mut self,
        prompt: &str,
        default: Option<u8>,
        choices: &Vec<char>,
    ) -> io::Result<()> {
        self.write_formatted_str(|this, buf| {
            this.theme.format_key_prompt(buf, prompt, default, &choices)
        })
    }

    pub fn confirmation_prompt_selection(&mut self, prompt: &str, sel: bool) -> io::Result<()> {
        self.write_formatted_prompt(|this, buf| {
            this.theme
                .format_confirmation_prompt_selection(buf, prompt, sel)
        })
    }

    pub fn key_prompt_selection(&mut self, prompt: &str, sel: char) -> io::Result<()> {
        self.write_formatted_prompt(|this, buf| {
            this.theme
                .format_single_prompt_selection(buf, prompt, &sel.to_string())
        })
    }

    pub fn single_prompt_selection(&mut self, prompt: &str, sel: &str) -> io::Result<()> {
        self.write_formatted_prompt(|this, buf| {
            this.theme.format_single_prompt_selection(buf, prompt, sel)
        })
    }

    pub fn multi_prompt_selection(&mut self, prompt: &str, selections: &[&str]) -> io::Result<()> {
        self.write_formatted_prompt(|this, buf| {
            this.theme
                .format_multi_prompt_selection(buf, prompt, selections)
        })
    }

    pub fn password_prompt_selection(&mut self, prompt: &str) -> io::Result<()> {
        self.write_formatted_prompt(|this, buf| {
            this.theme.format_password_prompt_selection(buf, prompt)
        })
    }

    pub fn selection(&mut self, text: &str, style: SelectionStyle) -> io::Result<()> {
        self.write_formatted_line(|this, buf| this.theme.format_selection(buf, text, style))
    }

    pub fn clear(&mut self) -> io::Result<()> {
        self.term
            .clear_last_lines(self.height + self.prompt_height)?;
        self.height = 0;
        Ok(())
    }

    pub fn clear_preserve_prompt(&mut self, size_vec: &[usize]) -> io::Result<()> {
        let mut new_height = self.height;
        //Check each item size, increment on finding an overflow
        for size in size_vec {
            if *size > self.term.size().1 as usize {
                new_height += 1;
            }
        }
        self.term.clear_last_lines(new_height)?;
        self.height = 0;
        Ok(())
    }
}

//=== START CUSTOM COLORED THEME ===
#[allow(clippy::needless_doctest_main)]
/// Provides a colored theme for dialoguer
///
/// # Examples
///
/// ```
/// use dialoguer::Confirmation;
/// use enquirer::ColoredTheme;
///
/// fn main() {
///     let prompt = Confirmation::with_theme(&ColoredTheme::default())
///         .with_text("Do you want to continue?")
///         .with_default(true);
///
///     if prompt.interact()? {
///         println!("Looks like you want to continue");
///     } else {
///         println!("nevermind then :(");
///     }
/// }
/// ```
pub struct ColoredTheme {
    pub defaults_style: Style,
    pub prompts_style: Style,
    pub prefixes_style: Style,
    pub values_style: Style,
    pub errors_style: Style,
    pub selected_style: Style,
    pub unselected_style: Style,
    /// Defaults to `true`
    pub inline_selections: bool,
    /// Defaults to `false`
    pub is_sort: bool,
}

impl Default for ColoredTheme {
    fn default() -> Self {
        ColoredTheme {
            defaults_style: Style::new().yellow().bold(),
            prompts_style: Style::new().bold(),
            prefixes_style: Style::new().cyan(),
            values_style: Style::new().green(),
            errors_style: Style::new().red(),
            selected_style: Style::new().cyan().bold(),
            unselected_style: Style::new(),
            inline_selections: true,
            is_sort: true,
        }
    }
}

impl ColoredTheme {
    /// Checkboxes print the selected values on the prompt line.
    /// This option allows the user to customize whether
    /// those will be printed on the prompts line or not.
    ///
    /// # Examples
    ///
    /// ```
    /// use enquirer::ColoredTheme;
    ///
    /// let theme = ColoredTheme::default().inline_selections(false);
    /// ```
    pub fn inline_selections(mut self, val: bool) -> Self {
        self.inline_selections = val;
        self
    }

    /// OrderList by default prints like Checkboxes. This function
    /// allows the user to specify that the theme needs to use
    /// a different style for sort.
    ///
    /// # Examples
    ///
    /// ```
    /// use enquirer::ColoredTheme;
    ///
    /// let theme = ColoredTheme::default().set_sort(true);
    /// ```
    pub fn set_sort(mut self, val: bool) -> Self {
        self.is_sort = val;
        self
    }

    fn empty(&self) -> (StyledObject<&str>, StyledObject<&str>) {
        (
            self.prompts_style.apply_to(""),
            self.prompts_style.apply_to(""),
        )
    }
}

impl Theme for ColoredTheme {
    // Error
    fn format_error(&self, f: &mut dyn fmt::Write, err: &str) -> fmt::Result {
        write!(
            f,
            "{} {}",
            self.errors_style.apply_to("✘"),
            self.errors_style.apply_to(err)
        )?;

        Ok(())
    }

    // Prompt
    fn format_prompt(&self, f: &mut dyn fmt::Write, prompt: &str) -> fmt::Result {
        write!(
            f,
            "{} {} {}",
            self.prefixes_style.apply_to("?"),
            self.prompts_style.apply_to(prompt),
            self.defaults_style.apply_to("›")
        )?;

        Ok(())
    }

    // Input
    fn format_singleline_prompt(
        &self,
        f: &mut dyn fmt::Write,
        prompt: &str,
        default: Option<&str>,
    ) -> fmt::Result {
        let details = match default {
            Some(default) => format!(" ({})", default),
            None => "".to_string(),
        };

        write!(
            f,
            "{} {}{} {} ",
            self.prefixes_style.apply_to("?"),
            self.prompts_style.apply_to(prompt),
            self.defaults_style.apply_to(details),
            self.defaults_style.apply_to("›"),
        )?;

        Ok(())
    }

    // Input Selection
    fn format_single_prompt_selection(
        &self,
        f: &mut dyn fmt::Write,
        prompt: &str,
        selection: &str,
    ) -> fmt::Result {
        write!(
            f,
            "{} {} {} {}",
            self.values_style.apply_to("✔"),
            self.prompts_style.apply_to(prompt),
            self.defaults_style.apply_to("·"),
            self.values_style.apply_to(selection),
        )?;

        Ok(())
    }

    // Confirm
    fn format_confirmation_prompt(
        &self,
        f: &mut dyn fmt::Write,
        prompt: &str,
        default: Option<bool>,
    ) -> fmt::Result {
        let details = match default {
            None => self.empty(),
            Some(true) => (
                self.defaults_style.apply_to("(Y/n)"),
                self.prefixes_style.apply_to("true"),
            ),
            Some(false) => (
                self.defaults_style.apply_to("(y/N)"),
                self.prefixes_style.apply_to("false"),
            ),
        };

        write!(
            f,
            "{} {} {} {} {} ",
            self.prefixes_style.apply_to("?"),
            self.prompts_style.apply_to(prompt),
            details.0,
            self.defaults_style.apply_to("›"),
            details.1,
        )?;

        Ok(())
    }

    /// Formats a key prompt.
    fn format_key_prompt(
        &self,
        f: &mut dyn fmt::Write,
        prompt: &str,
        default: Option<u8>,
        choices: &Vec<char>,
    ) -> fmt::Result {
        let mut strs = self._format_key_prompt(default, &choices);
        strs.insert(0, '(');
        strs.push(')');
        let keys = self.defaults_style.apply_to(strs);

        write!(
            f,
            "{} {} {} {} ",
            self.prefixes_style.apply_to("?"),
            self.prompts_style.apply_to(prompt),
            keys,
            self.defaults_style.apply_to("›"),
        )?;
        Ok(())
    }

    // Confirm Selection
    fn format_confirmation_prompt_selection(
        &self,
        f: &mut dyn fmt::Write,
        prompt: &str,
        selection: bool,
    ) -> fmt::Result {
        write!(
            f,
            "{} {} {} {}",
            self.values_style.apply_to("✔"),
            self.prompts_style.apply_to(prompt),
            self.defaults_style.apply_to("·"),
            self.values_style
                .apply_to(if selection { "true" } else { "false" }),
        )?;

        Ok(())
    }

    // Password Selection
    fn format_password_prompt_selection(
        &self,
        f: &mut dyn fmt::Write,
        prompt: &str,
    ) -> fmt::Result {
        self.format_single_prompt_selection(f, prompt, "********")
    }

    // Selection
    fn format_selection(
        &self,
        f: &mut dyn fmt::Write,
        text: &str,
        style: SelectionStyle,
    ) -> fmt::Result {
        let strings = match style {
            SelectionStyle::CheckboxCheckedSelected => (
                self.values_style
                    .apply_to(if self.is_sort { "❯" } else { "✔" }),
                self.selected_style.apply_to(text),
            ),
            SelectionStyle::CheckboxCheckedUnselected => (
                self.values_style.apply_to("✔"),
                self.unselected_style.apply_to(text),
            ),
            SelectionStyle::CheckboxUncheckedSelected => (
                if self.is_sort {
                    self.defaults_style.apply_to(" ")
                } else {
                    self.defaults_style.apply_to("✔")
                },
                self.selected_style.apply_to(text),
            ),
            SelectionStyle::CheckboxUncheckedUnselected => (
                if self.is_sort {
                    self.defaults_style.apply_to(" ")
                } else {
                    self.defaults_style.apply_to("✔")
                },
                self.unselected_style.apply_to(text),
            ),
            SelectionStyle::MenuSelected => (
                self.values_style.apply_to("❯"),
                self.selected_style.apply_to(text),
            ),
            SelectionStyle::MenuUnselected => (
                self.defaults_style.apply_to(" "),
                self.unselected_style.apply_to(text),
            ),
        };

        write!(f, "{} {}", strings.0, strings.1)?;

        Ok(())
    }

    // Multi Prompt Selection
    fn format_multi_prompt_selection(
        &self,
        f: &mut dyn fmt::Write,
        prompt: &str,
        selections: &[&str],
    ) -> fmt::Result {
        write!(
            f,
            "{} {} {}",
            self.values_style.apply_to("✔"),
            self.prompts_style.apply_to(prompt),
            self.defaults_style.apply_to("·"),
        )?;

        if self.inline_selections {
            let selections_last_index = selections.len() - 1;

            for (i, v) in selections.iter().enumerate() {
                if i == selections_last_index {
                    write!(f, " {}", self.values_style.apply_to(v))?;
                } else {
                    write!(f, " {},", self.values_style.apply_to(v))?;
                }
            }
        }

        Ok(())
    }
}
//=== END CUSTOM COLORED THEME ===

/// Returns the default theme.
///
/// (This returns the simple theme)
pub(crate) fn get_default_theme() -> &'static dyn Theme {
    &SimpleTheme
}
