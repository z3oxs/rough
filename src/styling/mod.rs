use gtk::prelude::*;
use gtk::CssProvider;
use crate::utils::types::Config;

pub struct Provider {
    pub config: Config
}

impl Provider {
    pub fn new(&self) -> CssProvider {
        let mut css: String = Self::create_default_css(&self.config);

        if self.config.window.opacity < 1.0 {
            css = Self::create_transparent_css(&self.config);
        
        }

        let provider = gtk::CssProvider::new();
        
        provider.load_from_data(css.as_bytes()).expect("Failed to read custom CSS");
        
        provider
    }

    fn create_transparent_css(config: &Config) -> String {
        let mut css = format!("
            * {{
                color: {};
            }}

            window {{
                border: 1px solid {};
            }}

            entry {{
                border: none;
                font-weight: 500;
                border-radius: 0;
                padding: 0;
                background: transparent;
                font-size: 14pt;
                color: white;
            }}

            list {{
                background: transparent;
            }}

            .title {{
                font-size: 11pt;
                font-weight: 600;
                color: white;
            }}

            .generic {{
                font-size: 9pt;
                font-weight: 500;
                color: #EEE;
            }}
        ",
            config.window.font_color,
            config.window.border_color,
        );

        if config.window.font != "" {
            css += &format!("
                * {{
                    font-family: {};
                }}
            ",
                config.window.font
            );
        }

        css
    }

    fn create_default_css(config: &Config) -> String {
        let mut css = format!("
            * {{
                color: {};
            }}

            window {{
                border: 1px solid {};
            }}

            entry {{
                border: none;
                font-weight: 600;
                border-radius: 0;
                padding: 0;
                font-size: 11pt;
                color: white;
            }}

            .title {{
                font-size: 11pt;
                font-weight: 600;
                color: white;
            }}

            .generic {{
                font-size: 9pt;
                font-weight: 500;
                color: #EEE;
            }}
        ",
            config.window.font_color,
            config.window.border_color,
        );

        if config.window.font != "" {
            css += &format!("
                * {{
                    font-family: {};
                }}
            ",
                config.window.font
            );
        }

        css
    }
}
