extern crate memchr;
extern crate memmap;

use std::path::Path;
use std::fs::File;
use std::cmp::{max, min};
use std::fmt;
use std::io::prelude::*;

// use memmap::{Mmap};
use memchr::memchr;

// Why is it called partialEq?
#[derive(Debug, PartialEq, Default, Clone)]
pub struct Count {
    pub code: u32,
    pub comment: u32,
    pub blank: u32,
    pub lines: u32,
}

impl Count {
    pub fn merge(&mut self, o: &Count) {
        self.code += o.code;
        self.comment += o.comment;
        self.blank += o.blank;
        self.lines += o.lines;
    }
}

pub struct LangTotal {
    pub files: u32,
    pub count: Count,
}

pub enum LineConfig<'a> {
    Normal {
        single: Option<&'a str>,
        multi: Option<(&'a str, &'a str)>,
    },
    Everything {
        singles: Vec<&'a str>,
        multies: Vec<(&'a str, &'a str)>,
    },
}

// Do any languages actually use utf8 chars as comment chars?
// We can probably do something with the encoding crate where we decode
// as ascii, and then use unsafe_from_utf8. If decoding fails,
// we catch it and just use the safe from_utf8 as we're doing now.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Copy, Clone)]
pub enum Lang {
    ActionScript,
    Ada,
    Agda,
    Asp,
    AspNet,
    Assembly,
    Autoconf,
    Awk,
    Batch,
    BourneShell,
    C,
    CCppHeader,
    CSharp,
    CShell,
    Clojure,
    CoffeeScript,
    ColdFusion,
    ColdFusionScript,
    Coq,
    Cpp,
    Css,
    CUDA,
    CUDAHeader,
    D,
    Dart,
    DeviceTree,
    Elixir,
    Elm,
    Erlang,
    Forth,
    FortranLegacy,
    FortranModern,
    FSharp,
    Gherkin,
    Glsl,
    Go,
    Groovy,
    Handlebars,
    Haskell,
    Hex,
    Html,
    INI,
    Idris,
    IntelHex,
    Isabelle,
    Jai,
    Java,
    JavaScript,
    Json,
    Jsx,
    Julia,
    Kotlin,
    Less,
    LinkerScript,
    Lean,
    Lisp,
    Lua,
    Make,
    Makefile,
    Markdown,
    Mustache,
    Nim,
    Nix,
    OCaml,
    ObjectiveC,
    ObjectiveCpp,
    Oz,
    Pascal,
    Perl,
    Php,
    Polly,
    Prolog,
    Protobuf,
    PureScript,
    Pyret,
    Python,
    Qcl,
    Qml,
    R,
    Razor,
    ReStructuredText,
    Ruby,
    RubyHtml,
    Rust,
    SaltStack,
    Sass,
    Scala,
    Sml,
    Sql,
    Stylus,
    Swift,
    Tcl,
    Terraform,
    Tex,
    Text,
    Toml,
    TypeScript,
    Tsx,
    UnrealScript,
    VimScript,
    Wolfram,
    XML,
    Yacc,
    Yaml,
    Zig,
    Zsh,
    Haxe,
    Unrecognized,
}
use self::Lang::*;

impl Lang {
    pub fn to_s(&self) -> &str {
        match *self {
            ActionScript => "ActionScript",
            Ada => "Ada",
            Agda => "Agda",
            Asp => "ASP",
            AspNet => "ASP.NET",
            Assembly => "Assembly",
            Autoconf => "Autoconf",
            Awk => "Awk",
            Batch => "Batch",
            BourneShell => "Bourne Shell",
            C => "C",
            CCppHeader => "C/C++ Header",
            CSharp => "C#",
            CShell => "C Shell",
            Clojure => "Clojure",
            CoffeeScript => "CoffeeScript",
            ColdFusion => "ColdFusion",
            ColdFusionScript => "ColdFusionScript",
            Coq => "Coq",
            Cpp => "C++",
            Css => "CSS",
            CUDA => "CUDA",
            CUDAHeader => "CUDA Header",
            D => "D",
            Dart => "Dart",
            DeviceTree => "DeviceTree",
            Elixir => "Elixir",
            Elm => "Elm",
            Erlang => "Erlang",
            Forth => "Forth",
            FortranLegacy => "FORTRAN Legacy",
            FortranModern => "FORTRAN Modern",
            FSharp => "F#",
            Gherkin => "Gherkin",
            Glsl => "GLSL",
            Go => "Go",
            Groovy => "Groovy",
            Handlebars => "Handlebars",
            Haskell => "Haskell",
            Hex => "Hex",
            Html => "HTML",
            INI => "INI",
            Idris => "Idris",
            IntelHex => "Intel Hex",
            Isabelle => "Isabelle",
            Jai => "Jai",
            Java => "Java",
            JavaScript => "JavaScript",
            Json => "JSON",
            Jsx => "Jsx",
            Julia => "Julia",
            Kotlin => "Kotlin",
            Less => "Less",
            LinkerScript => "LinkerScript",
            Lean => "Lean",
            Lisp => "Lisp",
            Lua => "Lua",
            Make => "Make",
            Makefile => "Makefile",
            Markdown => "Markdown",
            Mustache => "Mustache",
            Nim => "Nim",
            Nix => "Nix",
            OCaml => "OCaml",
            ObjectiveC => "Objective-C",
            ObjectiveCpp => "Objective-C++",
            Oz => "Oz",
            Pascal => "Pascal",
            Perl => "Perl",
            Php => "PHP",
            Polly => "Polly",
            Prolog => "Prolog",
            Protobuf => "Protobuf",
            PureScript => "PureScript",
            Pyret => "Pyret",
            Python => "Python",
            Qcl => "Qcl",
            Qml => "Qml",
            R => "R",
            Razor => "Razor",
            ReStructuredText => "reStructuredText",
            Ruby => "Ruby",
            RubyHtml => "RubyHtml",
            Rust => "Rust",
            SaltStack => "SaltStack",
            Sass => "Sass",
            Scala => "Scala",
            Sml => "SML",
            Sql => "SQL",
            Stylus => "Stylus",
            Swift => "Swift",
            Tcl => "Tcl",
            Terraform => "Terraform",
            Tex => "TeX",
            Text => "Plain Text",
            Toml => "Toml",
            TypeScript => "TypeScript",
            Tsx => "Typescript JSX",
            UnrealScript => "UnrealScript",
            VimScript => "VimL",
            Wolfram => "Wolfram",
            XML => "XML",
            Yacc => "Yacc",
            Yaml => "YAML",
            Zig => "Zig",
            Zsh => "Z Shell",
            Haxe => "Haxe",
            Unrecognized => "Unrecognized",
        }
    }
}

impl fmt::Display for Lang {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.pad(self.to_s())
    }
}

pub fn lang_from_ext(filepath: &str) -> Lang {
    let path = Path::new(filepath);
    let file_name_lower = path.file_name()
        .expect("no filename?")
        .to_str()
        .expect("to_str")
        .to_lowercase();

    let ext = if file_name_lower.contains("makefile") {
        String::from("makefile")
    } else {
        match path.extension() {
            Some(os_str) => os_str.to_str().expect("path to_str").to_lowercase(),
            None => file_name_lower,
        }
    };

    // NOTE(cgag): while we lifted most of this from tokei, we support a few
    // more extensions in some places, can't just assume it's the same.
    match &*ext {
        "4th" | "forth" | "fr" | "frt" | "fth" | "f83" | "fb" | "fpm" | "e4" | "rx" | "ft" => Forth,
        "ada" | "adb" | "ads" | "pad" => Ada,
        "agda" => Agda,
        "as" => ActionScript,
        "awk" => Awk,
        "bat" | "btm" | "cmd" => Batch,
        "c" | "ec" | "pgc" => C,
        "cc" | "cpp" | "cxx" | "c++" | "pcc" => Cpp,
        "cfc" => ColdFusionScript,
        "coffee" => CoffeeScript,
        "cs" => CSharp,
        "csh" => CShell,
        "css" | "postcss" => Css,
        "cu" => CUDA,
        "cuh" => CUDAHeader,
        "d" => D,
        "dart" => Dart,
        "dts" | "dtsi" => DeviceTree,
        "el" | "lisp" | "lsp" | "scm" | "ss" | "rkt" => Lisp,
        "ex" | "exs" => Elixir,
        "elm" => Elm,
        "erl" | "hrl" => Erlang,
        "feature" => Gherkin,
        "fs" | "fsx" => FSharp,
        "vert" | "tesc" | "tese" | "geom" | "frag" | "comp" => Glsl,
        "go" => Go,
        "groovy" => Groovy,
        "h" | "hh" | "hpp" | "hxx" => CCppHeader,
        "hbs" | "handlebars" => Handlebars,
        "hs" => Haskell,
        "html" => Html,
        "idr" | "lidr" => Idris,
        "ini" => INI,
        "jai" => Jai,
        "java" => Java,
        "jl" => Julia,
        "js" => JavaScript,
        "jsx" => Jsx,
        "kt" | "kts" => Kotlin,
        "lds" => LinkerScript,
        "lean" | "hlean" => Lean,
        "less" => Less,
        "lua" => Lua,
        "m" => ObjectiveC,
        "ml" | "mli" => OCaml,
        "nb" | "wl" => Wolfram,
        "sh" => BourneShell,
        "asa" | "asp" => Asp,
        "asax" | "ascx" | "asmx" | "aspx" | "master" | "sitemap" | "webinfo" => AspNet,
        "in" => Autoconf,
        "clj" | "cljs" | "cljc" => Clojure,

        "f" | "for" | "ftn" | "f77" | "pfo" => FortranLegacy,
        "f03" | "f08" | "f90" | "f95" => FortranModern,
        "makefile" | "mk" => Makefile,
        "mm" => ObjectiveCpp,
        "nim" => Nim,
        "nix" => Nix,
        "php" => Php,
        "pl" | "pm" => Perl,
        "qcl" => Qcl,
        "qml" => Qml,
        "cshtml" => Razor,
        "mustache" => Mustache,
        "oz" => Oz,
        "p" | "pro" => Prolog,
        "pas" => Pascal,
        "hex" => Hex,
        "ihex" => IntelHex,
        "json" => Json,
        "markdown" | "md" => Markdown,
        "rst" => ReStructuredText,
        "text" | "txt" => Text,

        "polly" => Polly,
        "proto" => Protobuf,
        "purs" => PureScript,
        "arr" => Pyret,
        "py" => Python,
        "r" => R,
        "rake" | "rb" => Ruby,
        "rhtml" => RubyHtml,
        "rs" => Rust,
        "s" | "asm" => Assembly,
        "sass" | "scss" => Sass,
        "sc" | "scala" => Scala,
        "sls" => SaltStack,
        "sml" => Sml,
        "sql" => Sql,
        "styl" => Stylus,
        "swift" => Swift,
        "tcl" => Tcl,
        "tf" => Terraform,
        "tex" | "sty" => Tex,
        "toml" => Toml,
        "ts" => TypeScript,
        "tsx" => Tsx,
        "thy" => Isabelle,
        "uc" | "uci" | "upkg" => UnrealScript,
        "v" => Coq,
        "vim" => VimScript,
        "xml" => XML,
        "yaml" | "yml" => Yaml,
        "y" => Yacc,
        "zig" => Zig,
        "zsh" => Zsh,
        "hx" => Haxe,
        // Probably dumb to just default to C.
        _ => Unrecognized,
    }
}

enum ConfigTuple<'a> {
    // Normal (terrible name), anything without multiple syntaxes
    N(Option<&'a str>, Option<(&'a str, &'a str)>),
    // Everything (multiple singles, multiple multiline)
    EV(Vec<&'a str>, Vec<(&'a str, &'a str)>),
}
use self::ConfigTuple::*;

pub fn counter_config_for_lang<'a>(lang: &Lang) -> LineConfig<'a> {
    let c_style = N(Some("//"), Some(("/*", "*/")));
    let html_style = N(None, Some(("<!--", "-->")));
    let ml_style = N(None, Some(("(*", "*)")));
    let no_comments = N(None, None);
    let prolog_style = N(Some("%"), Some(("/*", "*/")));
    let sh_style = N(Some("#"), None);

    let ctuple = match *lang {
        Ada => N(Some("--"), None),
        Batch => N(Some("REM"), None),
        Erlang | Tex => N(Some("%"), None),
        FortranModern => N(Some("!"), None),
        INI => N(Some(";"), None),
        Protobuf | Zig => N(Some("//"), None),
        VimScript => N(Some("\""), None),
        Terraform => N(Some("#"), Some(("/*", "*/"))),
        Nix => N(Some("#"), Some(("/*", "*/"))),

        // TODO(cgag): Well, some architectures use ;, @, |, etc.  Figure out something
        // better?
        Assembly => N(Some("#"), Some(("/*", "*/"))),
        CoffeeScript => N(Some("#"), Some(("###", "###"))),
        D => N(Some("//"), Some(("/*", "*/"))),
        Forth => N(Some("\\"), Some(("(", ")"))),
        FSharp => N(Some("//"), Some(("(*", "*)"))),
        Julia => N(Some("#"), Some(("#=", "=#"))),
        Lisp => N(Some(";"), Some(("#|", "|#"))),
        Lean => N(Some("--"), Some(("/-", "-/"))),
        Lua => N(Some("--"), Some(("--[[", "]]"))),
        // which one is right? = or =pod?
        // Perl => SM("#""=", "=cut"),
        Perl => N(Some("#"), Some(("=pod", "=cut"))),
        Pyret => N(Some("#"), Some(("#|", "|#"))),
        Python => N(Some("#"), Some(("'''", "'''"))),
        Ruby => N(Some("#"), Some(("=begin", "=end"))),
        Sql => N(Some("--"), Some(("/*", "*/"))),
        Haskell | Idris | Agda | PureScript | Elm => N(Some("--"), Some(("{-", "-}"))),

        ColdFusion => N(None, Some(("<!---", "--->"))),
        Mustache => N(None, Some(("{{!", "}}"))),

        Asp => EV(vec!["'", "REM"], vec![]),
        AspNet => EV(vec![], vec![("<!--", "-->"), ("<%--", "-->")]),
        Autoconf => EV(vec!["#", "dnl"], vec![]),
        Clojure => EV(vec![";", "#"], vec![]),
        FortranLegacy => EV(vec!["c", "C", "!", "*"], vec![]),
        Handlebars => EV(vec![], vec![("<!--", "-->"), ("{{!", "}}")]),
        Php => EV(vec!["#", "//"], vec![("/*", "*/")]),
        Isabelle => {
            EV(
                vec!["--"],
                // Is that angle bracket utf8?  What's going to happen with that?
                vec![
                    ("{*", "*}"),
                    ("(*", "*)"),
                    ("‹", "›"),
                    ("\\<open>", "\\<close>"),
                ],
            )
        }
        Razor => EV(vec![], vec![("<!--", "-->"), ("@*", "*@")]),
        Pascal => EV(vec!["//", "(*"], vec![("{", "}")]),
        Text | Markdown | Json | IntelHex | Hex | ReStructuredText => no_comments,

        Oz | Prolog => prolog_style,

        Coq | Sml | Wolfram | OCaml => ml_style,

        Html | Polly | RubyHtml | XML => html_style,

        BourneShell | Make | Awk | CShell | Gherkin | Makefile | Nim | R | SaltStack | Tcl
        | Toml | Yaml | Zsh | Elixir => sh_style,

        // TODO(cgag): not 100% sure that yacc belongs here.
        C | CCppHeader | Rust | Yacc | ActionScript | ColdFusionScript | Css | Cpp | CUDA
        | CUDAHeader | CSharp | Dart | DeviceTree | Glsl | Go | Jai | Java | JavaScript | Jsx
        | Kotlin | Less | LinkerScript | ObjectiveC | ObjectiveCpp | Qcl | Sass | Scala | Swift
        | TypeScript | Tsx | UnrealScript | Stylus | Qml | Haxe | Groovy => c_style,

        Unrecognized => unreachable!(),
    };

    match ctuple {
        N(single, multi) => LineConfig::Normal {
            single: single,
            multi: multi,
        },
        EV(singles, multies) => LineConfig::Everything {
            singles: singles,
            multies: multies,
        },
    }
}

struct ByteLinesState<'a> {
    buf: &'a [u8],
    pos: usize,
}

struct ByteLines<'a>(&'a [u8]);

impl<'a> ByteLines<'a> {
    fn lines(&self) -> ByteLinesState {
        ByteLinesState {
            buf: self.0,
            pos: 0,
        }
    }
}

impl<'a> Iterator for ByteLinesState<'a> {
    type Item = &'a [u8];

    fn next(&mut self) -> Option<&'a [u8]> {
        match memchr(b'\n', &self.buf[self.pos..self.buf.len()]) {
            Some(n) => {
                let start = self.pos;
                self.pos = self.pos + n + 1;
                // - 1 to drop \n char
                Some(&self.buf[start..(self.pos - 1)])
            }
            None => {
                if self.pos == self.buf.len() {
                    return None;
                }
                let start = self.pos;
                self.pos = self.buf.len();
                Some(&self.buf[start..self.pos])
            }
        }
    }
}

pub fn count(filepath: &str) -> Count {
    let lang = lang_from_ext(filepath);
    let config = counter_config_for_lang(&lang);
    match config {
        LineConfig::Normal { single, multi } => {
            // TODO(cgag):get rid of this once we unify count_normal and count_everything
            let singles = match single {
                Some(s) => vec![s],
                None => vec![],
            };
            let multies = match multi {
                Some(m) => vec![m],
                None => vec![],
            };
            count_normal(filepath, &singles, multies)
        }
        // TODO(cgag): get rid of this or normal
        LineConfig::Everything { singles, multies } => {
            count_normal(filepath, &singles, multies)
        }
    }
}

pub fn count_normal(
    filepath: &str,
    singles: &[&str],
    multies: Vec<(&str, &str)>,
) -> Count {
    let mfile = File::open(filepath);
    let mut file = match mfile {
        Ok(file) => file,
        Err(_) => {
            return Count::default();
        }
    };
    let mut bytes = vec![];
    file.read_to_end(&mut bytes).expect("nani?!");

    let mut c = Count::default();
    let mut multi_stack: Vec<(&str, &str)> = vec![];

    'line: for byte_line in ByteLines(&bytes).lines() {
        let line = match std::str::from_utf8(byte_line) {
            Ok(s) => s,
            // TODO(cgag): should we report when this happens?
            Err(_) => return Count::default(),
        };
        c.lines += 1;

        let line = line.trim_left();
        // should blanks within a comment count as blank or comment? This counts them as blank.
        if line.is_empty() {
            c.blank += 1;
            continue;
        };

        // if we match a single line comment, count it and go onto next line
        // TODO(cgag): is the multiline comment start symbol ever the shorter one?
        // if multi_stack.is_empty, then we're not currently in a multiline comment
        if multi_stack.is_empty() {
            for single_start in singles.iter() {
                if line.starts_with(single_start) {
                    // if this single_start is a prefix of a multi_start,
                    // make sure that the line doesn't actually start with the multi_start
                    // TODO(cgag): donm't do this check here
                    // TODO(cgag): this assumption that the multi-line comment is always the longer one
                    //             may well be a terrible one
                    if multies.iter().clone().any(|(m_start, _)| line.starts_with(m_start)) {
                        break;
                    }

                    c.comment += 1;
                    continue 'line;
                }
            }

            if multies.len() == 0 {
                c.code += 1;
                continue 'line;
            }
        }

        if multi_stack.is_empty() && !multies.iter().any(|(start, end)| line.contains(start) || line.contains(end)) {
            c.code += 1;
            continue 'line;
        }

        let mut pos = 0;
        let mut found_code = 0;
        let line_len = line.len();
        let contains_utf8 = (0..line_len).any(|i| !line.is_char_boundary(i));

        'outer: while pos < line_len {

            // TODO(cgag):  If we're not in a comment yet, we need to be searching for all possible
            // multi-line starts, and if we fin one, add it to the stack.  If there's one on the
            // stack, we then need to both be searching for new starts of any kind, and the end
            // marker of the one on top of the stack.  If we ever hit any non-whitespace while the
            // stack is empty, then we found some code on that line and it gets counted as code.

            // TODO(cgag): merge the representation of in_comment with the multi_stack
            { // new version
                // TODO(cgag): figure out how to remove all these clones
                for multi in multies.iter() {
                    let (start, end) = multi;
                    let start_len    = start.len();
                    let end_len      = end.len();

                    // TODO(cgag): this is almost ceratinly giving us incorrect results.  Say the
                    // first multi is the longest.  If we advance position because the final byte
                    // position of that multi hits unicode, we might have skipped over a perfectly
                    // valid comment start that was unaffected by the unicode.
                    if contains_utf8 {
                        // TODO(cgag): was: for i in pos..pos + min(max(start_len, end_len) + 1, line_len - pos) {
                        // ensure the next N bytes are true characters, where N is the largest thing we
                        // might be looking for.
                        // TODO(cgag): Now that we're looking for multiple possible things, we've got
                        // problems.  Really not sure what this should look like.
                        for i in pos..pos + min(max(start_len, end_len) + 1, line_len - pos) {
                            if !line.is_char_boundary(i) {
                                pos += 1;
                                continue 'outer;
                            }
                        }
                    }

                    if pos + start_len <= line_len && &line[pos..pos + start_len] == *start {
                        pos += start_len;
                        multi_stack.push(*multi);
                        continue;
                    }

                    if multi_stack.len() > 0 {
                        // TODO(cgag): clone, bad
                        let (_, end) = multi_stack.last().expect("stack clone").clone();
                        if pos+end.len() <= line_len && &line[pos..pos+end.len()] == end {
                            let _ = multi_stack.pop();
                            pos += end.len();
                        }
                    } else if multi_stack.is_empty() && pos+1 <= line_len && !&line[pos..pos + 1].chars().next().expect("whitespace check").is_whitespace() {
                        found_code += 1;
                    }
                }
                pos += 1;
            }
        }

        if found_code >= multies.len() {
            c.code += 1;
        } else {
            c.comment += 1;
        }

    }

    c
}
