use clap::Parser;
use std::fs::File;
use std::io::Cursor;
use std::path::Path;
use unreal_asset::exports::Export;
use unreal_asset::properties::Property;
use unreal_asset::types::PackageIndex;
use unreal_asset::Asset;

const PRESET_TH1_UASSET: &[u8] = include_bytes!("Preset_th1.uasset");
const PRESET_TH1_UEXP: &[u8] = include_bytes!("Preset_th1.uexp");

/// Generate game presets for Pseudoregalia custom maps
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Filename stem for output preset uasset file (without extension)
    #[arg(short, long)]
    output: String,

    /// Title of game preset
    #[arg(long)]
    title: String,

    /// Author of game preset
    #[arg(long)]
    author: String,

    /// Name of level asset
    #[arg(long)]
    level: String,

    /// PlayerStartTag of spawn point 
    #[arg(long, default_value_t = String::from("gameStart"))]
    tag: String,

    /// Enable upgrade (attack)
    #[arg(long, default_value_t = false)]
    dream_breaker: bool,

    /// Enable upgrade (slide)
    #[arg(long, default_value_t = false)]
    slide: bool,

    /// Enable upgrade (power boost)
    #[arg(long, default_value_t = false)]
    indignation: bool,

    /// Enable upgrade (air kick)
    #[arg(long, default_value_t = false)]
    sun_greaves: bool,

    /// Enable upgrade (plunge)
    #[arg(long, default_value_t = false)]
    sunsetter: bool,

    /// Enable upgrade (slide jump)
    #[arg(long, default_value_t = false)]
    solar_wind: bool,

    /// Enable upgrade (wall ride)
    #[arg(long, default_value_t = false)]
    cling_gem: bool,

    /// Enable upgrade (bounce attack)
    #[arg(long, default_value_t = false)]
    ascendant_light: bool,

    /// Enable upgrade (charge attack)
    #[arg(long, default_value_t = false)]
    strikebreak: bool,

    /// Enable upgrade (projectile)
    #[arg(long, default_value_t = false)]
    soul_cutter: bool,

    /// Enable upgrade (extra kick)
    #[arg(long, default_value_t = false)]
    heliacal_power: bool,
}

fn main() -> Result<(), String> {
    let args = Args::parse();

    let mut asset = Asset::new(
        Cursor::new(PRESET_TH1_UASSET),
        Some(Cursor::new(PRESET_TH1_UEXP)),
        unreal_asset::engine_version::EngineVersion::VER_UE5_1,
        None,
    )
    .unwrap();

    let export = asset.get_export_mut(PackageIndex::new(1)).unwrap();
    let Export::DataTableExport(export) = export else {
        return err("Export is not a DataTableExport");
    };
    let row = &mut export.table.data[0];
    for prop in &mut row.value {
        match prop {
            Property::MapProperty(prop) => {
                let name = prop.name.get_owned_content();
                assert!(name == "Upgrades_12_339EDA2D4B022358B32C3984E9FAE5F1");
                for (_, k, v) in prop.value.iter_mut() {
                    let Property::NameProperty(k) = k else {
                        return err("key is not a NameProperty");
                    };
                    let Property::IntProperty(v) = v else {
                        return err("value is not an IntProperty");
                    };
                    let key_name = k.value.get_owned_content();
                    match key_name.as_str() {
                        "attack" => {
                            v.value = bool2int(args.dream_breaker);
                        }
                        "slide" => {
                            v.value = bool2int(args.slide);
                        }
                        "SlideJump" => {
                            v.value = bool2int(args.solar_wind);
                        }
                        "Light" => {
                            v.value = bool2int(args.ascendant_light);
                        }
                        "airKick" => {
                            v.value = bool2int(args.sun_greaves);
                        }
                        "projectile" => {
                            v.value = bool2int(args.soul_cutter);
                        }
                        "plunge" => {
                            v.value = bool2int(args.sunsetter);
                        }
                        "powerBoost" => {
                            v.value = bool2int(args.indignation);
                        }
                        "extraKick" => {
                            v.value = bool2int(args.heliacal_power);
                        }
                        "chargeAttack" => {
                            v.value = bool2int(args.strikebreak);
                        }
                        "wallRide" => {
                            v.value = bool2int(args.cling_gem);
                        }
                        _ => {
                            return Err(format!("Unrecognized key name: {}", key_name));
                        }
                    }
                }
            }
            Property::StrProperty(prop) => {
                let name = prop.name.get_owned_content();
                match name.as_str() {
                    "Title_18_8D403C334BBBCC29B73D3CACCDAF0A08" => {
                        prop.value.replace(args.title.clone());
                    }
                    "Author_6_5E436BFF41A27B8B13653A8CEC5D15A6" => {
                        prop.value.replace(args.author.clone());
                    }
                    "LevelName_2_392769FD4066EFFA0CC1F99E8D749886" => {
                        prop.value.replace(args.level.clone());
                    }
                    "PlayerStartTag_5_7797C3C742DE3A0B8EEE189EDBEF3683" => {
                        prop.value.replace(args.tag.clone());
                    }
                    _ => {
                        return Err(format!("Unrecognized field name for StrProperty: {}", name));
                    }
                };
            }
            _ => {
                return err("Unrecognized export type");
            }
        };
    }

    let output_uasset_path = Path::new(&args.output);
    let mut output_uasset_file = File::create(output_uasset_path.with_extension("uasset")).unwrap();
    let output_uexp_path = output_uasset_path.with_extension("uexp");
    let mut output_uexp_file = File::create(output_uexp_path).unwrap();

    asset
        .write_data(&mut output_uasset_file, Some(&mut output_uexp_file))
        .unwrap();

    Ok(())
}

fn err(s: &str) -> Result<(), String> {
    Err(String::from(s))
}

fn bool2int(b: bool) -> i32 {
    if b {
        1
    } else {
        0
    }
}
