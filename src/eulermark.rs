extern crate extra;
extern crate serialize;
extern crate time;

use extra::stats::Stats;
use serialize::{Decodable,Encodable,json};
use std::f64::{ceil,log10,sqrt};
use std::io::fs;
use std::io::process::Process;
use std::io::stdio;
use std::io::{Append,File,Truncate,Write};
use std::iter::range_inclusive;
use std::num::pow;
use std::os;
use std::str;
use std::vec;
use time::precise_time_ns;

static max_bench_time: u64 = 3_000_000_000;

fn format_time(ns: u64) -> ~str {
    if ns < 1_000 {
        format!("{:4} ns", ns)
    } else {
        let digits = ceil(log10(ns as f64)) as uint;
        let one_zeros = pow(10u64, digits - 3);
        let ns = (ns / one_zeros) * one_zeros;

        if ns < 1_000_000 {
            format!("{:4} us", ns as f64 / 1_000.0)
        } else if ns < 1_000_000_000 {
            format!("{:4} ms", ns as f64 / 1_000_000.0)
        } else {
            format!("{:4} s", ns as f64 / 1_000_000_000.0)
        }
    }
}

fn specialize(template: &str, source: &Path) -> Path {
    os::getcwd().join(template.replace("*", source.with_extension("").filename_str().unwrap()))
}

struct Symlink(Path);

impl Drop for Symlink {
    fn drop(&mut self) {
        let &Symlink(ref path) = self;

        match fs::unlink(path) {
            Err(_) => fail!("failed to remove symlink"),
            Ok(_)  => {}
        }
    }
}

#[deriving(Decodable)]
struct Compiler {
    byproduct: ~[~str],
    command: ~str,
    flags: ~[~str],
    output: ~str,
}

impl Compiler {
    fn compile(&self, source: &Path) -> Option<CompilerOutput> {
        let mut args = self.flags.clone();
        let prog = self.command.as_slice();
        args.push(source.as_str().unwrap().to_owned());

        match Process::output(prog, args) {
            Err(_) => {
                println!("compiler not found!\n");
                None
            } Ok(output) => {
                if output.status.success() {
                    Some(CompilerOutput {
                        executable: specialize(self.output, source),
                        byproduct: self.byproduct.iter().map(|t| specialize(t.as_slice(), source)).to_owned_vec()
                    })
                } else {
                    println!("compilation failed! {} stderr:\n{}",
                        prog,
                        str::from_utf8_owned(output.error).unwrap());
                    None
                }
            }
        }
    }
}

struct CompilerOutput {
    executable: Path,
    byproduct: ~[Path],
}

impl Drop for CompilerOutput {
    fn drop(&mut self) {
        match fs::unlink(&self.executable) {
            Err(_) => fail!("Couldn't delete executable"),
            Ok(_)  => {}
        }

        for file in self.byproduct.iter() {
            match fs::unlink(file) {
                Err(_) => fail!("Couldn't delete byproduct"),
                Ok(_)  => {}
            }
        }
    }
}

impl CompilerOutput {
    fn run(&self) -> Option<(~str, u64)> {
        let filename = str::from_utf8(self.executable.filename().unwrap()).unwrap();
        let prog = self.executable.as_str().unwrap();

        let now = precise_time_ns();
        let out = Process::output(prog, []);
        let elapsed = precise_time_ns() - now;

        match out {
            Err(e)     => fail!("failed to run executable: {}", e),
            Ok(output) => if output.status.success() {
                let stdout = str::from_utf8_owned(output.output).unwrap();

                Some((stdout, elapsed))
            } else {
                println!("runtime error! {} stderr:\n{}",
                    filename,
                    str::from_utf8_owned(output.error).unwrap())

                    None
            }
        }
    }
}

#[deriving(Decodable)]
struct Interpreter {
    command: ~str,
    flags: ~[~str],
}

impl Interpreter {
    fn execute(&self, bytecode: &Path) -> Option<(~str, u64)> {
        let mut args = self.flags.clone();
        let prog = self.command.as_slice();
        args.push(bytecode.as_str().unwrap().to_owned());

        let now = precise_time_ns();
        let out = Process::output(prog, args);
        let elapsed = precise_time_ns() - now;

        match out {
            Err(_) => {
                println!("interpreter not found!");
                None
            } Ok(output) => if output.status.success() {
                let stdout = str::from_utf8_owned(output.output).unwrap();

                Some((stdout, elapsed))
            } else {
                print!("execution failed! {} stderr:\n{}",
                    prog,
                    str::from_utf8_owned(output.error).unwrap());

                None
            }
        }
    }
}

#[deriving(Decodable)]
struct Language {
    comment: ~str,
    compiler: Option<Compiler>,
    extension: ~str,
    interpreter: Option<Interpreter>,
    name: ~str,
}

struct Problem {
    answer: ~str,
    path: Path,
    title: ~str,
}

impl Problem {
    fn new(n: uint) -> Option<Problem> {
        let id = format!("{:03}", n);
        let path = problems_directory().join(format!("{}/{}/{}/{}",
            id.char_at(0),
            id.char_at(1),
            id.char_at(2),
            id,
        ));
        let ans = match File::open(&path.with_extension("ans")) {
            Err(_) => {
                println!("answer file (.ans) not found!");
                return None;
            } Ok(mut f) => match f.read_to_str() {
                Err(e) => {
                    println!("failed to read answer file: {}", e);
                    return None;
                } Ok(s) => s,
            }
        };
        let title = match File::open(&path.with_extension("md")) {
            Err(_) => {
                println!("problem statement file (.md) not found!");
                return None;
            } Ok(mut f) => match f.read_to_str() {
                Err(e) => {
                    println!("failed to read  file: {}", e);
                    return None;
                } Ok(s) => s.lines().take(2).to_owned_vec().connect("\n"),
            }
        };

        Some(Problem { answer: ans, path: path, title: title })
    }
}

struct Benchmark<'a,'b> {
    language: &'a Language,
    loc: uint,
    problem: &'b Problem,
    source: Path,
}

impl<'a,'b> Benchmark<'a,'b> {
    fn new(language: &'a Language, problem: &'b Problem) -> Option<Benchmark<'a,'b>> {
        let source = problem.path.with_extension(language.extension.as_slice());

        if source.exists() {
            let loc = match File::open(&source) {
                Err(_) => fail!("failed to open source file"),
                Ok(mut file) => match file.read_to_str() {
                    Err(_) => fail!("failed to read source file"),
                    Ok(code) => code.lines().fold(0, |loc, line| {
                        let line = line.trim();
                        if line.char_len() == 0 || line.starts_with(language.comment) {
                            loc
                        } else {
                            loc + 1
                        }
                    })
                }
            };
            Some(Benchmark {
                language: language, problem: problem, source: source, loc: loc
            })
        } else {
            None
        }
    }

    fn compile(&self) -> Option<CompilerOutput> {
        self.language.compiler.get_ref().compile(&self.source)
    }

    fn execute(&self, compiler_output: &Option<CompilerOutput>) -> Option<(bool, u64)> {
        match self.language.interpreter {
            None => compiler_output.get_ref().run(),
            Some(ref interpreter) => {
                let bytecode = match *compiler_output {
                    None => &self.source,
                    Some(ref compiler_output) => &compiler_output.executable,
                };

                interpreter.execute(bytecode)
            }
        }.map(|(solution, time)| (solution == self.problem.answer, time))
    }

    fn benchmark(&self, base_results: &Option<&[BenchmarkResult]>) -> Option<BenchmarkResult> {
        print!("* COMPILING... ");
        stdio::flush();
        let compiler_output = match self.language.compiler {
            None => {
                println!("Nothing to compile");
                None
            } Some(_) => {
                match self.compile() {
                    None => {
                        return None;
                    } Some(out) => {
                        println!("Done")
                        Some(out)
                    }
                }
            }
        };

        print!("* VALIDATING... ");
        stdio::flush();
        let estimate = match self.execute(&compiler_output) {
            None => {
                return None;
            } Some((correct, time)) => {
                if correct {
                    println!("Correct!");
                    time
                } else {
                    println!("Incorrect\n")
                    return None;
                }
            }
        };

        let runs = max_bench_time / estimate;

        print!("* BENCHMARKING... ");
        stdio::flush();
        let mut samples = vec::with_capacity(runs as uint);
        for _ in range(0, runs) {
            samples.push(self.execute(&compiler_output).unwrap().val1() as f64)
        }

        let mu = samples.mean() as u64;
        let sigma = samples.std_dev() as u64;

        print!("abs: {} ns/iter (+/- {})", mu, sigma);

        let absolute = Measurement { mu: mu, sigma: sigma };

        let relative = match base_results {
            &None => None,
            &Some(results) => match results.iter().find(|result| result.language == self.language.name) {
                None => None,
                Some(result) => {
                    Some(absolute - result.absolute)
                }
            }

        };

        match relative {
            None => println!(" rel: None"),
            Some(ref measurement) => println!(" rel: {} ns/iter (+/- {})", measurement.mu ,measurement.sigma)
        }

        Some(BenchmarkResult {
            language: self.language.name.clone(),
            loc: self.loc,
            absolute: absolute,
            relative: relative,
        })
    }
}

#[deriving(Decodable,Encodable)]
struct Measurement {
    mu: u64,
    sigma: u64,
}

impl Sub<Measurement, Measurement> for Measurement {
    fn sub(&self, rhs: &Measurement) -> Measurement {
        Measurement {
            mu: if self.mu > rhs.mu { self.mu - rhs.mu } else { 0 },
            sigma: sqrt((pow(self.sigma, 2) + pow(rhs.sigma, 2)) as f64) as u64
        }
    }
}

#[deriving(Decodable,Encodable)]
struct BenchmarkResult {
    language: ~str,
    loc: uint,
    absolute: Measurement,
    relative: Option<Measurement>,
}

impl BenchmarkResult {
    fn show(&self) {
        println!("{} - {}", format_time(self.absolute.mu), self.language);
    }
}

fn eulermark_directory() -> Path {
    Path::new(os::args()[0]).dir_path().dir_path()
}

fn languages_directory() -> Path {
    Path::new(os::args()[0]).dir_path().dir_path().join("languages")
}

fn problems_directory() -> Path {
    Path::new(os::args()[0]).dir_path().dir_path().join("problems")
}

fn supported_languages() -> ~[Language] {
    match fs::readdir(&languages_directory()) {
        Err(e) => fail!("languages directory not found: {}", e),
        Ok(paths) => paths.move_iter().filter(|path| match path.extension_str() {
            Some(extension) => extension == "json",
            None => false,
        }).map(|path| {
            let filename = path.filename_str().unwrap();
            match File::open(&path) {
                Err(e) => fail!("failed to open {}: {}", filename, e),
                Ok(mut file) => match file.read_to_str() {
                    Err(e) => fail!("failed to read {}: {}", filename, e),
                    Ok(s) => match json::from_str(s) {
                        Err(e) => fail!("failed to parse {}: {}", filename, e),
                        Ok(json) => {
                            // FIXME this should be an one-liner
                            let t: Language = Decodable::decode(&mut json::Decoder::new(json));
                            t
                        },
                    },
                },
            }
        }).filter(|language| {
            if language.compiler.is_none() && language.interpreter.is_none() {
                fail!("{}.json must have a compiler, an interpreter or both", language.name);
            } else {
                true
            }
        }).to_owned_vec(),
    }
}

fn read_results(json_path: &Path) -> Option<~[BenchmarkResult]> {
    match File::open(json_path) {
        Err(_) => {
            println!("failed to open {}", json_path.as_str().unwrap());
            None
        } Ok(mut file) => match file.read_to_str() {
            Err(_) => {
                println!("failed to read {}", json_path.as_str().unwrap());
                None
            } Ok(s) => match json::from_str(s) {
                Err(_) => {
                    println!("failed to parse {}", json_path.as_str().unwrap());
                    None
                } Ok(json) => {
                    Some(Decodable::decode(&mut json::Decoder::new(json)))
                }
            }
        }
    }
}

fn write_results(results: &[BenchmarkResult], json_path: &Path) {
    match File::open_mode(json_path, Truncate, Write) {
        Err(_) => fail!("couldn't open json file"),
        Ok(mut file) => match file.write_str(json::Encoder::str_encode(&results)) {
            Err(_) => fail!("failed to write json file"),
            Ok(_) => {}
        }
    }
}

fn update_readme(results: &mut [BenchmarkResult], problem_path: &Path) {
    let readme_path = problem_path.dir_path().join("README.md");

    match fs::copy(&problem_path.with_extension("md"), &readme_path) {
        Err(_) => fail!("failed to copy md file"),
        Ok(_) => {}
    }

    match File::open_mode(&readme_path, Append, Write) {
        Err(_) => fail!("failed to open README.md"),
        Ok(mut file) => {
            let mut buf = ~"";

            buf.push_str("Language | LoC\n--- | :---:\n");

            results.sort_by(|x, y| x.loc.cmp(&y.loc));

            buf.push_str(results.iter().map(|r| {
                format!("{} | {}", r.language, r.loc)
            }).to_owned_vec().connect("\n"));

            buf.push_str("\n\nLanguage | aTime | aTime\n--- | :---: | :---:\n");

            results.sort_by(|x, y| x.absolute.mu.cmp(&y.absolute.mu));

            let min_atime = results[0].absolute.mu;

            buf.push_str(results.iter().map(|r| {
                format!("{} | {} | {}%",
                    r.language,
                    format_time(r.absolute.mu),
                    r.absolute.mu * 100 / min_atime)
            }).to_owned_vec().connect("\n"));

            if results.iter().any(|r| {
                r.relative.is_some() &&
                r.relative.get_ref().mu > r.relative.get_ref().sigma
            }) {
                let mut results = results.iter().filter(|r| {
                    r.relative.get_ref().mu > r.relative.get_ref().sigma
                }).to_owned_vec();

                results.sort_by(|x, y| x.relative.get_ref().mu.cmp(&y.relative.get_ref().mu));

                let min_rtime = results[0].relative.get_ref().mu;

                buf.push_str("\n\nLanguage | rTime | rTime\n--- | :---: | :---:\n");

                buf.push_str(results.iter().map(|r| {
                    format!("{} | {} | {}%",
                        r.language,
                        format_time(r.relative.get_ref().mu),
                        r.relative.get_ref().mu * 100 / min_rtime)
                }).to_owned_vec().connect("\n"))
            }

            buf.push_str("\n");

            match file.write_str(buf) {
                Err(_) => fail!("failed to write README.md"),
                Ok(_) => {}
            }
        }
    }
}

fn benchmark(languages: &[Language], pid: uint, base_results: &Option<&[BenchmarkResult]>) {
    match Problem::new(pid) {
        None => return,
        Some(problem) => {
            println!("{}\n", problem.title);

            let input_data = problem.path.with_extension("in");

            let _symlink = if input_data.exists() {
                let dst = Path::new(input_data.filename().unwrap());

                match fs::symlink(&input_data, &dst) {
                    Err(_) => fail!("failed to symlink input data"),
                    Ok(_) => Some(Symlink(dst)),
                }
            } else {
                None
            };

            let mut results = ~[];
            for language in languages.iter() {
                match Benchmark::new(language, &problem) {
                    None => println!("{} not found\n", language.name),
                    Some(benchmark) => {
                        println!("Found {}", language.name);

                        match benchmark.benchmark(base_results) {
                            None => {},
                            Some(bench) =>{
                                println!("");
                                results.push(bench);
                            }
                        }
                    }

                }
            }

            if results.len() == 0 {
                return;
            }

            results.sort_by(|x, y| x.absolute.mu.cmp(&y.absolute.mu));

            println!("SUMMARY:");
            for result in results.iter() {
               result.show();
            }

            print!("\nJSON... ");
            stdio::flush();
            write_results(results, &problem.path.with_extension("json"));
            println!("Done");

            print!("README... ");
            stdio::flush();
            update_readme(results, &problem.path);
            println!("Done\n");
        }
    };
}

fn update_table() {
    let mut buf = ~"";

    buf.push_str("PID | aTime | rTime | Loc\n");
    buf.push_str(":--:| :---: | :---: | :---:\n");
    buf.push_str(match fs::walk_dir(&problems_directory()) {
        Err(_)    => fail!("problems directory does not exist"),
        Ok(paths) => paths.filter_map(|path| {
            if path.extension_str().map_or(false, |ext| ext == "json") {
                let results = read_results(&path).unwrap();

                let link = path.dir_path();

                let filename = path.filename_str().unwrap();

                let pid = filename.split('.').next().unwrap();

                let atime = results.iter()
                                   .min_by(|x| x.absolute.mu)
                                   .unwrap()
                                   .language.as_slice();

                let rtime = if results.iter().any(|r| {
                    r.relative.is_none()
                        || r.relative.get_ref().sigma > r.relative.get_ref().mu
                }) {
                    "None"
                } else {
                    results.iter().min_by(|x| x.relative.get_ref().mu)
                           .unwrap()
                           .language.as_slice()
                };

                let loc = results.iter()
                                 .min_by(|x| x.loc)
                                 .unwrap()
                                 .language.as_slice();

                Some(format!("[{}]({}) | {} | {} | {}",
                        pid,
                        link.as_str().unwrap(),
                        atime,
                        rtime,
                        loc))
            } else {
                None
            }
        })
    }.to_owned_vec().connect("\n"));

    buf.push_str("\n");

    match File::open_mode(&eulermark_directory().join("README.md"), Truncate, Write) {
        Err(_)   => fail!("failed to open README.md"),
        Ok(mut file) => match file.write_str(buf) {
            Err(_) => fail!("failed to write README.md"),
            Ok(_)  => {}
        }
    }
}

fn help() {
    fail!("bad input");
}

fn main() {
    let args = os::args();

    if args.len() < 2 {
        help();
    }

    match args[1] {
        ~"bench" => {
            let (low, high): (uint, uint) = match args.len() {
                3 => {
                    let low: uint = match from_str(args[2]) {
                        None => fail!("bad input"),
                        Some(n) => n,
                    };

                    (low, low)
                } 4 => {
                    let low = match from_str(args[2]) {
                        None => fail!("bad input"),
                        Some(n) => n,
                    };

                    let high = match from_str(args[3]) {
                        None => fail!("bad input"),
                        Some(n) => n,
                    };

                    (low, high)
                } _ => fail!("bad input")
            };

            let languages = supported_languages();
            match Problem::new(0) {
                None => fail!("problem 0 must exist"),
                Some(problem) => match read_results(&problem.path.with_extension("json")) {
                    None => {
                        if low != 0 || high != 0 {
                            fail!("problem 0 must be benchmarked first");
                        }
                        benchmark(languages, 0, &None);
                    } Some(results) => {
                        for i in range_inclusive(low, high) {
                            let base_results = if i == 0 { None } else { Some(results.as_slice()) };
                            benchmark(languages, i, &base_results);
                        }
                    }
                }
            };
        } ~"table" => update_table(),
        _ => help(),
    }
}

