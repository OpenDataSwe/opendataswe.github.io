use anyhow::Result;
use std::fmt::Write;

#[derive(Debug)]
pub struct GenInfo {
    request_name: String,
    enum_tag_name: String,
    display_name: String,
}

pub fn generate_municipalities(municipalities: Vec<String>) -> Result<String> {
    let to_pascal = convert_case::Converter::new().to_case(convert_case::Case::Pascal);
    let mut infos = Vec::with_capacity(municipalities.len());
    for m in municipalities {
        let pascal = to_pascal.convert(&m);
        let disp = orig_to_display(&m, &to_pascal);
        let tag = replace_swedish(&pascal);
        let info = GenInfo {
            request_name: m,
            enum_tag_name: tag,
            display_name: disp,
        };
        infos.push(info);
    }
    write_enum(&infos)
}

fn replace_swedish(input: &str) -> String {
    input
        .replace('ä', "ae")
        .replace('Ä', "Ae")
        .replace('å', "ao")
        .replace('Å', "Ao")
        .replace('ö', "oe")
        .replace('Ö', "Oe")
}

fn orig_to_display(input: &str, pascal: &convert_case::Converter) -> String {
    let parts = input.split(' ');
    let mut complete = String::new();
    for (ind, w) in parts.enumerate() {
        let w = w.to_lowercase();
        if ind == 0 {
            complete.push_str(&pascal.convert(&w));
        } else {
            let _ = complete.write_fmt(format_args!(" {w}"));
        }
    }
    complete
}

fn write_enum(infos: &[GenInfo]) -> Result<String> {
    let mut enum_def = String::new();
    enum_def.push_str(
        "#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]\npub enum Municipalities {\n",
    );
    let mut to_req_impl = String::new();
    to_req_impl.push_str(
        "    #[must_use]\n#[expect(clippy::too_many_lines)]\npub fn to_request(self) -> &'static str {\n        match self {\n",
    );
    let mut display_name_impl = String::new();
    display_name_impl.push_str(
        "    #[must_use]\n#[expect(clippy::too_many_lines)]\npub fn display_name(self) -> &'static str {\n        match self {\n",
    );
    let mut all_values = format!("    pub const VALUES: [Self; {}] = [", infos.len());
    for info in infos {
        enum_def.write_fmt(format_args!("    {},\n", info.enum_tag_name))?;
        to_req_impl.write_fmt(format_args!(
            "            Self::{} => \"{}\",\n",
            info.enum_tag_name, info.request_name
        ))?;
        display_name_impl.write_fmt(format_args!(
            "            Self::{} => \"{}\",\n",
            info.enum_tag_name, info.display_name
        ))?;
        all_values.write_fmt(format_args!("Self::{}, ", info.enum_tag_name))?;
    }
    all_values.push_str("];\n\n");
    to_req_impl.push_str("        }\n    }\n\n");
    display_name_impl.push_str("        }\n    }\n\n");
    enum_def.push_str("}\nimpl Municipalities {\n");
    enum_def.push_str(&all_values);
    enum_def.push_str(&to_req_impl);
    enum_def.push_str(&display_name_impl);
    enum_def.push_str("}\n");
    enum_def.push_str(
        "impl core::fmt::Display for Municipalities {\n    \
        #[inline]\n    \
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> std::fmt::Result {\n        \
            f.write_str(self.display_name())\n    \
        }\n\
    }\n\n",
    );

    Ok(enum_def)
}
