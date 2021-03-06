/*
 * This file is part of the userbot-rs (github.com/sabbyX/userbot-rs).
 * Copyright (c) 2021 Sabby.
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, version 3.
 *
 * This program is distributed in the hope that it will be useful, but
 * WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU
 * General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program. If not, see <http://www.gnu.org/licenses/>.
 */

use super::{Config, Telegram, load_config, utils::get_config_path};
use std::path::PathBuf;

/// High level implementation to do operations with config file
pub struct ConfigControl {
    pub config: ini::Ini
}

impl ConfigControl {

    /// Creates new `ConfigControl` instance
    pub fn get_config(path: Option<PathBuf>) -> Option<Self> {
        Some(
            Self {
                config: load_config(path)?
            }
        )
    }

    /// returns the saved configurations at the moment
    pub fn get_config_schema(&self) -> anyhow::Result<Config> {
        let telegram_conf = &self.config.section(Some("telegram")).ok_or_else(|| anyhow::anyhow!("Failed to get `telegram` section in configuration file"))?;
        Ok(Config {
            telegram: Telegram {
                api_id: telegram_conf.get("api_id").ok_or_else(|| anyhow::anyhow!("Failed to fetch api id"))?.parse()?,
                api_hash: telegram_conf.get("api_hash").ok_or_else(|| anyhow::anyhow!("Failed to fetch api hash"))?.parse()?
            }
        })
    }

    /// Overwrites given _telegram_ configuration (`API_ID` and `API_HASH`) into configuration file
    pub fn write_telegram_conf(&mut self, api_id: i32, api_hash: String) -> anyhow::Result<&Self> {
        let ini = Self::__gen_telegram_section(api_id, api_hash, Some(self.config.clone()));
        ini.write_to_file(get_config_path(true))?;
        // well, its not intended that reloading should give `Option`, we have already written config to the path
        self.reload().unwrap();
        Ok(self)
    }

    fn __gen_telegram_section(api_id: i32, api_hash: String, conf: Option<ini::Ini>) -> ini::Ini {
        let mut ini = conf.unwrap_or_default();
        ini.with_section(Some("telegram"))
            .set("api_id", api_id.to_string())
            .set("api_hash", api_hash);
        ini
    }

    #[allow(dead_code)]
    pub fn write_to_conf(&self, _: ini::SectionSetter) -> anyhow::Result<&Self> {
        unimplemented!()
    }

    /// Consumes `self` and returns reloaded [ConfigControl](./struct.ConfigControl.html)
    pub fn reload(&mut self) -> Option<()> {
        // TODO: Support for using user-defined configuration file path
        self.config = load_config(None)?;
        Some(())
    }

    /// Check whether a section exist in configuration file
    /// # Note
    /// This would return `false` even when configuration file is not found in desired path.
    pub fn check_section_exists(section_name: &str, path: Option<PathBuf>) -> bool {
        let config = load_config(path);
        if config.is_none() { return false; }
        let config = config.unwrap();
        let section = config.section(Some(section_name));
        if section.is_none() { return false; }
        true
    }
}
