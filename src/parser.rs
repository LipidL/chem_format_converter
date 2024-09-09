pub mod xyz_parser{
    use std::{fs::File, io::{self, BufRead}, path::Path};

    use arc_parser::modules::structures::{Atom, StructureBlock, Coordinate, CrystalInfo};
    use regex::Regex;
    struct AtomDataParser{
        re: Regex,
    }
    impl AtomDataParser{
        pub fn new() -> Self {
            let atom_data_regex = Regex::new(r"^(?P<s>\w+)\s+(?P<f1>-?\d+\.\d+)\s+(?P<f2>-?\d+\.\d+)\s+(?P<f3>-?\d+\.\d+).*");
            Self{re: atom_data_regex.unwrap()}
        }
        pub fn parse_atom_data(&self, input:&str) -> Option<(String, f64, f64, f64)> {
            if let Some(caps) = self.re.captures(input) {
                let s = caps.name("s").unwrap().as_str().to_string();
                let f1 = caps.name("f1").unwrap().as_str().parse::<f64>().unwrap();
                let f2 = caps.name("f2").unwrap().as_str().parse::<f64>().unwrap();
                let f3 = caps.name("f3").unwrap().as_str().parse::<f64>().unwrap();
                return Some((s, f1, f2, f3));
            }
            None
        }
    }

    pub fn read_file(filepath: String) -> io::Result<StructureBlock> {
        let path = Path::new(&filepath);
        let file = File::open(&path)?;
        let reader = io::BufReader::new(file);
        // initialize parser
        let atom_data_parser = AtomDataParser::new();
        let mut atoms: Vec<Atom> = Vec::new();
        for line in reader.lines(){
            let line = line?;
            if let Some((s, f1, f2, f3)) = atom_data_parser.parse_atom_data(&line){
                let coord = Coordinate(f1, f2, f3);
                let atom = Atom{
                    element: s,
                    coordinate: coord
                };
                atoms.push(atom);
            }
        }
        // find maxmum and minimum value of x, y and z
        let mut x_max = f64::MIN;
        let mut x_min = f64::MAX;
        let mut y_max = f64::MIN;
        let mut y_min = f64::MAX;
        let mut z_max = f64::MIN;
        let mut z_min = f64::MAX;
        for atom in atoms.iter(){
            let x = atom.coordinate.0;
            let y = atom.coordinate.1;
            let z = atom.coordinate.2;
            if x > x_max {
                x_max = x;
            }
            if x < x_min {
                x_min = x;
            }
            if y > y_max {
                y_max = y;
            }
            if y < y_min {
                y_min = y;
            }
            if z > z_max {
                z_max = z;
            }
            if z < z_min {
                z_min = z;
            }
        }
        // move all atoms if x, y or z is negative
        let mut x_shift = 0.0;
        let mut y_shift = 0.0;
        let mut z_shift = 0.0;
        if x_min < 0.0 {
            x_shift = -x_min + 10.0;
        }
        if y_min < 0.0 {
            y_shift = -y_min + 10.0;
        }
        if z_min < 0.0 {
            z_shift = -z_min + 10.0;
        }
        let mut new_atoms: Vec<Atom> = Vec::new();
        for atom in atoms.iter(){
            let x = atom.coordinate.0 + x_shift;
            let y = atom.coordinate.1 + y_shift;
            let z = atom.coordinate.2 + z_shift;
            let coord = Coordinate(x, y, z);
            let new_atom = Atom{
                element: atom.element.clone(),
                coordinate: coord
            };
            new_atoms.push(new_atom);
        }
        // update maximum and minimum value of x, y and z
        x_max += x_shift + 10.0;
        y_max += y_shift + 10.0;
        z_max += z_shift + 10.0;

        // construct a cell to contain all atoms
        let cell = CrystalInfo{
            x: x_max,
            y: y_max,
            z: z_max,
            alpha: 90.0,
            beta: 90.0,
            gamma: 90.0
        };

        // construct a structure block
        let structure_block = StructureBlock{
            number: 0,
            energy: 0.0,
            symmetry: "P1".to_string(),
            crystal: cell,
            atoms: new_atoms
        };
        Ok(structure_block)
    }
}