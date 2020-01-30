use std::fs::{self, File};
use std::io::{self, BufRead, BufReader};

pub fn gn_metadata(gn_reflexives: &[String], gn_verb_groups: &[String], gn_expletives: &[String], verbs_file: &str) {

    let mut ar = Vec::new();
    let mut dr = Vec::new();
    let mut nr = Vec::new();

    let mut allgemein = Vec::new();
    let mut besitz = Vec::new();
    let mut gefuehl = Vec::new();
    let mut gesellschaft = Vec::new();
    let mut koerperfunktion = Vec::new();
    let mut kognition = Vec::new();
    let mut kommunikation = Vec::new();
    let mut konkurrenz = Vec::new();
    let mut kontakt = Vec::new();
    let mut lokation = Vec::new();
    let mut nat_phaenomen = Vec::new();
    let mut perzeption = Vec::new();
    let mut schoepfung = Vec::new();
    let mut veraenderung = Vec::new();
    let mut verbrauch = Vec::new();

    let mut expl = Vec::new();
    let mut non_expl = Vec::new();

    for gn_reflexive in gn_reflexives {
        if gn_reflexive.ends_with("acc.txt") {
            let f = File::open(&gn_reflexive).expect("Could not open file");
            for line in BufReader::new(f).lines() {
                ar.push(line.unwrap());
            }
        } else if gn_reflexive.ends_with("dat.txt") {
            let f = File::open(&gn_reflexive).expect("Could not open file");
            for line in BufReader::new(f).lines() {
                dr.push(line.unwrap());
            }
        } else if gn_reflexive.ends_with("non-refl.txt") {
            let f = File::open(&gn_reflexive).expect("Could not open file");
            for line in BufReader::new(f).lines() {
                nr.push(line.unwrap());
            }
        }
    }

    for gn_verb_group in gn_verb_groups {
        if gn_verb_group.ends_with("allgemein.txt") {
            let f = File::open(&gn_verb_group).expect("Could not open file");
            for line in BufReader::new(f).lines() {
                let verb = line.unwrap().split(":").collect::<Vec<&str>>()[0].to_string();
                allgemein.push(verb);
            }

        } else if gn_verb_group.ends_with("besitz.txt") {
            let f = File::open(&gn_verb_group).expect("Could not open file");
            for line in BufReader::new(f).lines() {
                let verb = line.unwrap().split(":").collect::<Vec<&str>>()[0].to_string();
                besitz.push(verb);
            }
        } else if gn_verb_group.ends_with("gefuehl.txt") {
            let f = File::open(&gn_verb_group).expect("Could not open file");
            for line in BufReader::new(f).lines() {
                let verb = line.unwrap().split(":").collect::<Vec<&str>>()[0].to_string();
                gefuehl.push(verb);
            }
        } else if gn_verb_group.ends_with("gesellschaft.txt") {
            let f = File::open(&gn_verb_group).expect("Could not open file");
            for line in BufReader::new(f).lines() {
                let verb = line.unwrap().split(":").collect::<Vec<&str>>()[0].to_string();
                gesellschaft.push(verb);
            }
        } else if gn_verb_group.ends_with("koerperfunktion.txt") {
            let f = File::open(&gn_verb_group).expect("Could not open file");
            for line in BufReader::new(f).lines() {
                let verb = line.unwrap().split(":").collect::<Vec<&str>>()[0].to_string();
                koerperfunktion.push(verb);
            }
        } else if gn_verb_group.ends_with("kognition.txt") {
            let f = File::open(&gn_verb_group).expect("Could not open file");
            for line in BufReader::new(f).lines() {
                let verb = line.unwrap().split(":").collect::<Vec<&str>>()[0].to_string();
                kognition.push(verb);
            }
        } else if gn_verb_group.ends_with("kommunikation.txt") {
            let f = File::open(&gn_verb_group).expect("Could not open file");
            for line in BufReader::new(f).lines() {
                let verb = line.unwrap().split(":").collect::<Vec<&str>>()[0].to_string();
                kommunikation.push(verb);
            }
        } else if gn_verb_group.ends_with("konkurrenz.txt") {
            let f = File::open(&gn_verb_group).expect("Could not open file");
            for line in BufReader::new(f).lines() {
                let verb = line.unwrap().split(":").collect::<Vec<&str>>()[0].to_string();
                konkurrenz.push(verb);
            }
        } else if gn_verb_group.ends_with("kontakt.txt") {
            let f = File::open(&gn_verb_group).expect("Could not open file");
            for line in BufReader::new(f).lines() {
                let verb = line.unwrap().split(":").collect::<Vec<&str>>()[0].to_string();
                kontakt.push(verb);
            }
        } else if gn_verb_group.ends_with("lokation.txt") {
            let f = File::open(&gn_verb_group).expect("Could not open file");
            for line in BufReader::new(f).lines() {
                let verb = line.unwrap().split(":").collect::<Vec<&str>>()[0].to_string();
                lokation.push(verb);
            }
        } else if gn_verb_group.ends_with("natPhaenomen.txt") {
            let f = File::open(&gn_verb_group).expect("Could not open file");
            for line in BufReader::new(f).lines() {
                let verb = line.unwrap().split(":").collect::<Vec<&str>>()[0].to_string();
                nat_phaenomen.push(verb);
            }
        } else if gn_verb_group.ends_with("perzeption.txt") {
            let f = File::open(&gn_verb_group).expect("Could not open file");
            for line in BufReader::new(f).lines() {
                let verb = line.unwrap().split(":").collect::<Vec<&str>>()[0].to_string();
                perzeption.push(verb);
            }
        } else if gn_verb_group.ends_with("schoepfung.txt") {
            let f = File::open(&gn_verb_group).expect("Could not open file");
            for line in BufReader::new(f).lines() {
                let verb = line.unwrap().split(":").collect::<Vec<&str>>()[0].to_string();
                schoepfung.push(verb);
            }
        } else if gn_verb_group.ends_with("veraenderung.txt") {
            let f = File::open(&gn_verb_group).expect("Could not open file");
            for line in BufReader::new(f).lines() {
                let verb = line.unwrap().split(":").collect::<Vec<&str>>()[0].to_string();
                veraenderung.push(verb);
            }
        } else if gn_verb_group.ends_with("verbrauch.txt") {
            let f = File::open(&gn_verb_group).expect("Could not open file");
            for line in BufReader::new(f).lines() {
                let verb = line.unwrap().split(":").collect::<Vec<&str>>()[0].to_string();
                verbrauch.push(verb);
            }
        }
    }

    for gn_expletive in gn_expletives {
        if gn_expletive.ends_with("verbs_expl.txt") {
            let f = File::open(&gn_expletive).expect("Could not open file");
            for line in BufReader::new(f).lines() {
                expl.push(line.unwrap());
            }
        } else if gn_expletive.ends_with("verbs_non-expl.txt") {
            let f = File::open(&gn_expletive).expect("Could not open file");
            for line in BufReader::new(f).lines() {
                non_expl.push(line.unwrap());
            }
        }
    }

    // Read verbs and check to which verb group they belong.
    // Print verb with verb group.
    let file = File::open(&verbs_file).expect("Could not open file");
    for line in BufReader::new(file).lines() {

        let line = line.unwrap();
        let split_line = line.split("\t").collect::<Vec<&str>>();

        print!("{}\t", split_line[0].to_string().replace("\"", "").split("%").collect::<Vec<&str>>()[0]);
        for i in 1..split_line.len() {
            print!("{}\t", split_line[i]);
        }

        let mut verb = split_line[0].to_string().replace("#", "");
        verb = verb.replace("\"", "");
        verb = verb.split("%").collect::<Vec<&str>>()[0].to_string();

        if ar.contains(&verb) {
            print!("AR\t")
        } else if dr.contains(&verb) {
            print!("DR\t")
        } else if nr.contains(&verb) {
            print!("NR\t")
        } else {
            print!("XY\t")
        }

        if allgemein.contains(&verb) {
            print!("allgemein\t")
        } else if besitz.contains(&verb) {
            print!("besitz\t")
        } else if gefuehl.contains(&verb) {
            print!("gefuehl\t")
        } else if gesellschaft.contains(&verb) {
            print!("gesellschaft\t")
        } else if koerperfunktion.contains(&verb) {
            print!("koerperfunktion\t")
        } else if kognition.contains(&verb) {
            print!("kognition\t")
        } else if kommunikation.contains(&verb) {
            print!("kommunikation\t")
        } else if konkurrenz.contains(&verb) {
            print!("konkurrenz\t")
        } else if kontakt.contains(&verb) {
            print!("kontakt\t")
        } else if lokation.contains(&verb) {
            print!("lokation\t")
        } else if nat_phaenomen.contains(&verb) {
            print!("natPhaenomen\t")
        } else if perzeption.contains(&verb) {
            print!("perzeption\t")
        } else if schoepfung.contains(&verb) {
            print!("schoepfung\t")
        } else if veraenderung.contains(&verb) {
            print!("veraenderung\t")
        } else if verbrauch.contains(&verb) {
            print!("verbrauch\t")
        } else {
            print!("XY\t")
        }

        if expl.contains(&verb) {
            print!("EXPL\n")
        } else if non_expl.contains(&verb) {
            print!("NEXPL\n")
        } else {
            print!("XY\n")
        }
    }
}

/// Assign a verb group (one group per file from `germanet_verb_files'
/// to all verbs from `verbs_file'.
pub fn gn_verbgroups(germanet_verb_files: &[String], verbs_file: &str) {

    let mut allgemein = Vec::new();
    let mut besitz = Vec::new();
    let mut gefuehl = Vec::new();
    let mut gesellschaft = Vec::new();
    let mut koerperfunktion = Vec::new();
    let mut kognition = Vec::new();
    let mut kommunikation = Vec::new();
    let mut konkurrenz = Vec::new();
    let mut kontakt = Vec::new();
    let mut lokation = Vec::new();
    let mut nat_phaenomen = Vec::new();
    let mut perzeption = Vec::new();
    let mut schoepfung = Vec::new();
    let mut veraenderung = Vec::new();
    let mut verbrauch = Vec::new();

    for germanet_file in germanet_verb_files {
        if germanet_file.ends_with("allgemein.txt") {
            let f = File::open(&germanet_file).expect("Could not open file");
            for line in BufReader::new(f).lines() {
                let verb = line.unwrap().split(":").collect::<Vec<&str>>()[0].to_string();
                allgemein.push(verb);
            }

        } else if germanet_file.ends_with("besitz.txt") {
            let f = File::open(&germanet_file).expect("Could not open file");
            for line in BufReader::new(f).lines() {
                let verb = line.unwrap().split(":").collect::<Vec<&str>>()[0].to_string();
                besitz.push(verb);
            }
        } else if germanet_file.ends_with("gefuehl.txt") {
            let f = File::open(&germanet_file).expect("Could not open file");
            for line in BufReader::new(f).lines() {
                let verb = line.unwrap().split(":").collect::<Vec<&str>>()[0].to_string();
                gefuehl.push(verb);
            }
        } else if germanet_file.ends_with("gesellschaft.txt") {
            let f = File::open(&germanet_file).expect("Could not open file");
            for line in BufReader::new(f).lines() {
                let verb = line.unwrap().split(":").collect::<Vec<&str>>()[0].to_string();
                gesellschaft.push(verb);
            }
        } else if germanet_file.ends_with("koerperfunktion.txt") {
            let f = File::open(&germanet_file).expect("Could not open file");
            for line in BufReader::new(f).lines() {
                let verb = line.unwrap().split(":").collect::<Vec<&str>>()[0].to_string();
                koerperfunktion.push(verb);
            }
        } else if germanet_file.ends_with("kognition.txt") {
            let f = File::open(&germanet_file).expect("Could not open file");
            for line in BufReader::new(f).lines() {
                let verb = line.unwrap().split(":").collect::<Vec<&str>>()[0].to_string();
                kognition.push(verb);
            }
        } else if germanet_file.ends_with("kommunikation.txt") {
            let f = File::open(&germanet_file).expect("Could not open file");
            for line in BufReader::new(f).lines() {
                let verb = line.unwrap().split(":").collect::<Vec<&str>>()[0].to_string();
                kommunikation.push(verb);
            }
        } else if germanet_file.ends_with("konkurrenz.txt") {
            let f = File::open(&germanet_file).expect("Could not open file");
            for line in BufReader::new(f).lines() {
                let verb = line.unwrap().split(":").collect::<Vec<&str>>()[0].to_string();
                konkurrenz.push(verb);
            }
        } else if germanet_file.ends_with("kontakt.txt") {
            let f = File::open(&germanet_file).expect("Could not open file");
            for line in BufReader::new(f).lines() {
                let verb = line.unwrap().split(":").collect::<Vec<&str>>()[0].to_string();
                kontakt.push(verb);
            }
        } else if germanet_file.ends_with("lokation.txt") {
            let f = File::open(&germanet_file).expect("Could not open file");
            for line in BufReader::new(f).lines() {
                let verb = line.unwrap().split(":").collect::<Vec<&str>>()[0].to_string();
                lokation.push(verb);
            }
        } else if germanet_file.ends_with("natPhaenomen.txt") {
            let f = File::open(&germanet_file).expect("Could not open file");
            for line in BufReader::new(f).lines() {
                let verb = line.unwrap().split(":").collect::<Vec<&str>>()[0].to_string();
                nat_phaenomen.push(verb);
            }
        } else if germanet_file.ends_with("perzeption.txt") {
            let f = File::open(&germanet_file).expect("Could not open file");
            for line in BufReader::new(f).lines() {
                let verb = line.unwrap().split(":").collect::<Vec<&str>>()[0].to_string();
                perzeption.push(verb);
            }
        } else if germanet_file.ends_with("schoepfung.txt") {
            let f = File::open(&germanet_file).expect("Could not open file");
            for line in BufReader::new(f).lines() {
                let verb = line.unwrap().split(":").collect::<Vec<&str>>()[0].to_string();
                schoepfung.push(verb);
            }
        } else if germanet_file.ends_with("veraenderung.txt") {
            let f = File::open(&germanet_file).expect("Could not open file");
            for line in BufReader::new(f).lines() {
                let verb = line.unwrap().split(":").collect::<Vec<&str>>()[0].to_string();
                veraenderung.push(verb);
            }
        } else if germanet_file.ends_with("verbrauch.txt") {
            let f = File::open(&germanet_file).expect("Could not open file");
            for line in BufReader::new(f).lines() {
                let verb = line.unwrap().split(":").collect::<Vec<&str>>()[0].to_string();
                verbrauch.push(verb);
            }
        }
    }

    // Read verbs and check to which verb group they belong.
    // Print verb with verb group.
    let file = File::open(&verbs_file).expect("Could not open file");
    for line in BufReader::new(file).lines() {

        let line = line.unwrap();
        let split_line = line.split("\t").collect::<Vec<&str>>();

        print!("{}\t", split_line[0].to_string().replace("\"", "").split("%").collect::<Vec<&str>>()[0]);
        for i in 1..split_line.len() {
            print!("{}\t", split_line[i]);
        }

        let mut verb = split_line[0].to_string().replace("#", "");
        verb = verb.replace("\"", "");
        verb = verb.split("%").collect::<Vec<&str>>()[0].to_string();

        if allgemein.contains(&verb) {
            print!("allgemein\n")
        } else if besitz.contains(&verb) {
            print!("besitz\n")
        } else if gefuehl.contains(&verb) {
            print!("gefuehl\n")
        } else if gesellschaft.contains(&verb) {
            print!("gesellschaft\n")
        } else if koerperfunktion.contains(&verb) {
            print!("koerperfunktion\n")
        } else if kognition.contains(&verb) {
            print!("kognition\n")
        } else if kommunikation.contains(&verb) {
            print!("kommunikation\n")
        } else if konkurrenz.contains(&verb) {
            print!("konkurrenz\n")
        } else if kontakt.contains(&verb) {
            print!("kontakt\n")
        } else if lokation.contains(&verb) {
            print!("lokation\n")
        } else if nat_phaenomen.contains(&verb) {
            print!("natPhaenomen\n")
        } else if perzeption.contains(&verb) {
            print!("perzeption\n")
        } else if schoepfung.contains(&verb) {
            print!("schoepfung\n")
        } else if veraenderung.contains(&verb) {
            print!("veraenderung\n")
        } else if verbrauch.contains(&verb) {
            print!("verbrauch\n")
        } else {
            print!("XY\n")
        }
    }
}

/// Label verbs with accusative or dative reflexive markers if applicable.
pub fn gn_reflexives(germanet_reflverb_files: &[String], verbs_file: &str) {

    let mut ar = Vec::new();
    let mut dr = Vec::new();
    let mut nr = Vec::new();

    for germanet_file in germanet_reflverb_files {
        if germanet_file.ends_with("acc.txt") {
            let f = File::open(&germanet_file).expect("Could not open file");
            for line in BufReader::new(f).lines() {
                ar.push(line.unwrap());
            }
        } else if germanet_file.ends_with("dat.txt") {
            let f = File::open(&germanet_file).expect("Could not open file");
            for line in BufReader::new(f).lines() {
                dr.push(line.unwrap());
            }
        } else if germanet_file.ends_with("non-refl.txt") {
            let f = File::open(&germanet_file).expect("Could not open file");
            for line in BufReader::new(f).lines() {
                nr.push(line.unwrap());
            }
        }
    }
    // Read verbs and check if they are reflexive.
    // Print verb with reflexive marker "AR" for accusative, "DR" for dative reflexives,
    // "NR" for non-reflexive verbs and "XY" reflexives not in GermaNet.
    let file = File::open(&verbs_file).expect("Could not open file");
    for line in BufReader::new(file).lines() {

        let line = line.unwrap();
        let split_line = line.split("\t").collect::<Vec<&str>>();

        print!("{}\t", split_line[0].to_string().replace("\"", "").split("%").collect::<Vec<&str>>()[0]);
        for i in 1..split_line.len() {
            if i == 3 && ! (split_line[2] == "Inversion frequency") {
                let inv_ratio = split_line[2].trim().parse::<f32>().expect("Error parsing as f32") / split_line[1].trim().parse::<f32>().expect("Error parsing as f32");
                if inv_ratio > 0f32 && inv_ratio < 1f32 {
                    print!("{}\t", inv_ratio);
                } else {
                    print!("0.000\t", );
                }
            } else {
                print!("{}\t", split_line[i]);
            }
        }

        let mut verb = split_line[0].to_string().replace("#", "");
        verb = verb.replace("\"", "");
        verb = verb.split("%").collect::<Vec<&str>>()[0].to_string();
        if ar.contains(&verb) {
            print!("AR\n")
        } else if dr.contains(&verb) {
            print!("DR\n")
        } else if nr.contains(&verb) {
            print!("NR\n")
        } else {
            print!("XY\n")
        }
    }
}

/// Collect reflexive verbs from GermaNet.
///
/// Verb frames in GermaNet contain information about reflexivity
/// of the verb. Markers are "AR" and "DR" where "R" marks reflexives.
/// "A" and "D" identify accusative and dative objects.
pub fn gn_collect_reflexive(files: &[String]) {
    for f in files {
        let file = File::open(&f).expect("Could not open file");
        for line in BufReader::new(file).lines() {
            let line = line.unwrap();
            let split_line = line.split(":").collect::<Vec<&str>>();
            let verb = split_line[0];
            if split_line.len() > 1 {
                let frame = split_line[1].split(".").collect::<Vec<&str>>();
                if frame.len() > 1 && frame[1] == "DR" {
                    println!("{}", verb);
                }
            }
        }
    }
}

/// Label verbs with expletive or non-expletive markers if applicable.
pub fn gn_expletives(germanet_explverb_files: &[String], verbs_file: &str) {

    let mut expl = Vec::new();
    let mut non_expl = Vec::new();

    for germanet_file in germanet_explverb_files {
        if germanet_file.ends_with("verbs_expl.txt") {
            let f = File::open(&germanet_file).expect("Could not open file");
            for line in BufReader::new(f).lines() {
                expl.push(line.unwrap());
            }
        } else if germanet_file.ends_with("verbs_non-expl.txt") {
            let f = File::open(&germanet_file).expect("Could not open file");
            for line in BufReader::new(f).lines() {
                non_expl.push(line.unwrap());
            }
        }
    }
    // Read verbs and check if they occur with expletive.
    // Print verb with expletive marker "EXPL" for expletives, "NEXPL" for non-expletives
    // and "XY" if the verb is not in GermaNet.
    let file = File::open(&verbs_file).expect("Could not open file");
    for line in BufReader::new(file).lines() {

        let line = line.unwrap();
        let split_line = line.split("\t").collect::<Vec<&str>>();

        print!("{}\t", split_line[0].to_string().replace("\"", "").split("%").collect::<Vec<&str>>()[0]);
        for i in 1..split_line.len() {
            print!("{}\t", split_line[i]);
        }

        let mut verb = split_line[0].to_string().replace("#", "");
        verb = verb.replace("\"", "");
        verb = verb.split("%").collect::<Vec<&str>>()[0].to_string();
        if expl.contains(&verb) {
            print!("EXPL\n")
        } else if non_expl.contains(&verb) {
            print!("NEXPL\n")
        } else {
            print!("XY\n")
        }
    }
}