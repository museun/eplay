use std::collections::HashMap;

pub struct Runner {
    map: HashMap<&'static str, fn()>,
    main: fn(),
}

pub fn register(name: &'static str, f: fn()) -> Runner {
    Runner {
        map: HashMap::default(),
        main: move || {},
    }
    .register(name, f)
}

impl Runner {
    pub fn register(mut self, name: &'static str, f: fn()) -> Self {
        self.map.insert(name, f);
        self
    }

    pub fn main(self, main: fn()) -> Self {
        Self { main, ..self }
    }

    pub fn run(self) {
        std::env::args()
            .nth(1)
            .and_then(|arg| match &*arg {
                "--list" | "-l" => self.print_tests(),
                arg => self.map.get(arg).copied(),
            })
            .unwrap_or(self.main)()
    }

    fn print_tests(&self) -> ! {
        let pad = self.map.keys().map(|s| s.len()).max().unwrap_or(4);

        for (name, f) in &self.map {
            println!("{name: <pad$} -> {f:p}", pad = pad)
        }

        println!(
            "{main: <pad$} -> {f:p}",
            main = "main",
            f = self.main,
            pad = pad
        );

        std::process::exit(0)
    }
}
