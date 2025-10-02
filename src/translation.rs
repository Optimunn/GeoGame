use serde::Deserialize;
use serde_json::Result;
use std::path::PathBuf;
use std::fs;
use slint::ToSharedString;
use crate::slint_generatedMainWindow::Translation;
use crate::to_ss;

#[derive(Deserialize)]
pub struct TranslationRs {
    cont_e: String,
    play: String,
    restart: String,
    settings: String,
    about: String,

    end: String,
    close: String,
    name: String,
    capital: String,
    continent: String,
    code: String,
    answer: String,
    info: String,
    help: String,
    exit_o_p: String,
    exit: String,
    pause: String,

    eur: String,
    asi: String,
    afr: String,
    nam: String,
    sam: String,
    oce: String,

    flags: String,
    capitals: String,
    flag_and_c: String,
    sel_pref_reg: String,
    sel_mode: String,
    p_10_n: String,
    p_25_n: String,
    p_hard: String,

    sel_b_color: String,
    sel_lang: String,

    time_out: String,
}

pub struct LocalTranslation {
    pub time_out: String,
}

impl TranslationRs {
    pub fn load_new(patch: &PathBuf) -> Result<Self> {
        let data: String = match fs::read_to_string(patch) {
            Ok(data) => data,
            Err(_) => String::from(""),
        };
        let result: TranslationRs = serde_json::from_str(&data)?;
        Ok(result)
    }

    pub fn local_translation(&self) -> LocalTranslation {
        LocalTranslation {
            time_out: self.time_out.clone(),
        }
    }

    pub fn to_translation(&self) -> Translation {
        Translation {
            cont_e: to_ss!(self.cont_e),
            play: to_ss!(self.play),
            restart: to_ss!(self.restart),
            settings: to_ss!(self.settings),
            about: to_ss!(self.about),

            end: to_ss!(self.end),
            close: to_ss!(self.close),
            name: to_ss!(self.name),
            capital: to_ss!(self.capital),
            continent: to_ss!(self.continent),
            code: to_ss!(self.code),
            answer: to_ss!(self.answer),
            info: to_ss!(self.info),
            help: to_ss!(self.help),
            exit_o_p: to_ss!(self.exit_o_p),
            exit: to_ss!(self.exit),
            pause: to_ss!(self.pause),

            eur: to_ss!(self.eur),
            asi: to_ss!(self.asi),
            afr: to_ss!(self.afr),
            nam: to_ss!(self.nam),
            sam: to_ss!(self.sam),
            oce: to_ss!(self.oce),

            flags: to_ss!(self.flags),
            capitals: to_ss!(self.capitals),
            flag_and_c: to_ss!(self.flag_and_c),
            sel_pref_reg: to_ss!(self.sel_pref_reg),
            sel_mode: to_ss!(self.sel_mode),
            p_10_n: to_ss!(self.p_10_n),
            p_25_n: to_ss!(self.p_25_n),
            p_hard: to_ss!(self.p_hard),

            sel_b_color: to_ss!(self.sel_b_color),
            sel_lang: to_ss!(self.sel_lang),
        }
    }
}