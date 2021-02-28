use serde::Deserialize;

pub type DataResult<T, E = reqwest::Error> = Result<T, E>;

const DATA_ENDPOINT: &str = "https://paste.myst.rs/api/v2/data/";

/// Get information on a specific language *supported by PasteMyst*.
/// You are recommened to only use the language names provided within
/// `pastemyst::data::language` to prevent panicking. This method is
/// synchronous.
///
/// This method does and will panic if a language is not found. The
/// simplest solution to this is to use the language name you know - as
/// mentioned earlier, it is recommended to use that specific module.
///
/// Some fields, namely `color and `ext` may not be provided and will
/// have the value of `None`. This is because they are not provided
/// by PasteMyst which means that they do not exist or specified anywhere.
///
/// ## Examples
/// ```rust
/// use pastemyst::data::*;
///
/// fn main() -> DataResult<()> {
///     let language: DataObject = get_language_by_name(language::DLANG)?;
///     println!("{:?}", language.color);
///     Ok(())
/// }
/// ```
pub fn get_language_by_name(language_name: &str) -> DataResult<DataObject, reqwest::Error> {
    Ok(reqwest::blocking::get(&parse_url(language_name, "name"))?.json()?)
}

/// Get information on a specific language *supported by PasteMyst*.
/// You are recommened to only use the language names provided within
/// `pastemyst::data::language` to prevent panicking. This method is
/// asynchronous.
///
/// This method does and will panic if a language is not found. The
/// simplest solution to this is to use the language name you know - as
/// mentioned earlier, it is recommended to use that specific module.
///
/// Some fields, namely `color` and `ext` may not be provided and will
/// have the value of `None`. This is because they are not provided
/// by PasteMyst which means that they do not exist or specified anywhere.
///
/// ## Examples
/// ```rust
/// use pastemyst::data::*;
///
/// #[tokio::main]
/// async fn main() -> DataResult<()> {
///     let language: DataObject = get_language_by_name_async(language::CLANG).await?;
///     println!("{:?}", language.name);
///     Ok(())
/// }
/// ```
pub async fn get_language_by_name_async(language_name: &str) -> DataResult<DataObject, reqwest::Error> {
    Ok(reqwest::get(&parse_url(language_name, "name")).await?.json().await?)
}

/// The same thing as getting a language by a name, except that it is by
/// extension, of a given language. This is a synchronous method.
///
/// This method will also panic if a language extension is not found. The
/// simplest solution to this is to use the language extension that PasteMyst
/// has. The easiest way to confirm so is to check if your desired language
/// exists in `pastemyst::data::langauge`.
///
/// This will return a `DataObject`. Some fields, namely `color` and `ext` may
/// not be provided and will have the value of `None`. This is because they are
/// not provided by PasteMyst which means that they do not exist or specified
/// anywhere.
///
/// ## Examples
/// ```rust
/// use pastemyst::data::*;
///
/// fn main() -> DataResult<()> {
///     let language: DataObject = get_language_by_extension("cs")?;
///     println!("{}", language.name);
///     Ok(())
/// }
/// ```
pub fn get_language_by_extension(lang_extension: &str) -> DataResult<DataObject, reqwest::Error> {
    Ok(reqwest::blocking::get(&parse_url(lang_extension, "ext"))?.json()?)
}

/// The same thing as getting a language by a name, except that it is by
/// extension, of a given language. This is an asynchronous method.
///
/// This method will also panic if a language extension is not found. The
/// simplest solution to this is to use the language extension that PasteMyst
/// has. The easiest way to confirm so is to check if your desired language
/// exists in `pastemyst::data::langauge`.
///
/// This will return a `DataObject`. Some fields, namely `color` and `ext` may
/// not be provided and will have the value of `None`. This is because they are
/// not provided by PasteMyst which means that they do not exist or specified
/// anywhere.
///
/// ## Examples
/// ```rust
/// use pastemyst::data::*;
///
/// #[tokio::main]
/// async fn main() -> DataResult<()> {
///     let language: DataObject = get_language_by_extension_async("c").await?;
///     println!("{:?}", language.color);
///     Ok(())
/// }
/// ```
pub async fn get_language_by_extension_async(lang_extension: &str) -> DataResult<DataObject, reqwest::Error> {
    Ok(reqwest::get(&parse_url(lang_extension, "ext")).await?.json().await?)
}

#[derive(Deserialize)]
#[allow(non_snake_case)]
pub struct DataObject {
    /// The name of the language.
    pub name: String,
    /// The mode of a language used
    /// in a pasty editor.
    pub mode: String,
    /// A vector of the data types
    /// of each language that is
    /// meant to be used by official
    /// standards.
    pub mimes: Vec<String>,
    /// The extension(s) of a language
    pub ext: Option<Vec<String>>,
    /// The color of a language used
    /// to identify a language.
    ///
    /// This field may, or may not be
    /// provided by PasteMyst.
    pub color: Option<String>,
    // /// An optional message provided
    // /// by PasteMyst if the language
    // /// has been found.
    // statusMessage: Option<String>
    // if result.statusMessage == None { println!("[pastemyst] The provided language does not exist!"); }
    // Note:
    // Might be implemented, might not. I'll let it stay here like so for now.
}

fn parse_url(value: &str, req_type: &str) -> String {
    let parsed_url: String;
    if req_type == "name" { parsed_url = format!("{}language?name={}", DATA_ENDPOINT, &value); }
    else if req_type == "ext" { parsed_url = format!("{}languageExt?extension={}", DATA_ENDPOINT, &value); }
    else { panic!("[pastemyst] Invalid valid: `req_type` provided. Report the developer about this."); }
    return parsed_url;
}

/// An enum of PasteMyt language constants.
//#[allow(non_camel_case_types)]
pub mod language {
    pub const AUTODETECT: &str = "Autodetect";
    pub const PLAIN: &str = "Plain Text";
    pub const APL: &str = "APL";
    pub const PGP: &str = "PGP";
    pub const ASN1: &str = "ASN.1";
    pub const ASTERISK: &str = "Asterisk";
    pub const BRAINFUCK: &str = "Brainfuck";
    pub const CLANG: &str = "C";
    pub const C: &str = "C";
    pub const CPP: &str = "C++";
    pub const COBOL: &str = "Cobol";
    pub const CSHARP: &str = "C#";
    pub const CLOJURE: &str = "Clojure";
    pub const CLOJURE_SCRIPT: &str = "ClojureScript";
    pub const GSS: &str = "Closure Stylesheets (GSS)";
    pub const CMAKE: &str = "CMake";
    pub const COFFEE_SCRIPT: &str = "CoffeeScript";
    pub const LISP: &str = "Common Lisp";
    pub const CYPHER: &str = "Cypher";
    pub const CYTHON: &str = "Cython";
    pub const CRYSTAL: &str = "Crystal";
    pub const CSS: &str = "CSS";
    pub const CQL: &str = "CQL";
    pub const DLANG: &'static str = "D";
    pub const D: &str = "D";
    pub const DART: &str = "Dart";
    pub const DIFF: &str = "diff";
    pub const DJANGO: &str = "Django";
    pub const DOCKER: &str = "Dockerfile";
    pub const DTD: &str = "DTD";
    pub const DYLAN: &str = "Dylan";
    pub const EBNF: &str = "EBNF";
    pub const ECL: &str = "ECL";
    pub const EDN: &str = "edn";
    pub const EIFFEL: &str = "Eiffel";
    pub const ELM: &str = "Elm";
    pub const EJS: &str = "Embedded Javascript";
    pub const ERB: &str = "Embedded Ruby";
    pub const ERLANG: &str = "Erlang";
    pub const ESPER: &str = "Esper";
    pub const FACTOR: &str = "Factor";
    pub const FCL: &str = "FCL";
    pub const FORTH: &str = "Forth";
    pub const FORTRAN: &str = "Fortran";
    pub const FSHARP: &str = "F#";
    pub const GAS: &str = "Gas";
    pub const GHERKIN: &str = "Gherkin";
    pub const GFM: &str = "GitHub Flavored Markdown";
    pub const GITHUB_MARKDOWN: &str = "GitHub Flavored Markdown";
    pub const GO: &str = "Go";
    pub const GROOVY: &str = "Groovy";
    pub const HAML: &str = "HAML";
    pub const HASKELL: &str = "Haskell";
    pub const HASKELL_LITERATE: &str = "Haskell (Literate)";
    pub const HAXE: &str = "Haxe";
    pub const HXML: &str = "HXML";
    pub const ASP_NET: &str = "ASP.NET";
    pub const HTML: &str = "HTML";
    pub const HTTP: &str = "HTTP";
    pub const IDL: &str = "IDL";
    pub const PUG: &str = "Pug";
    pub const JAVA: &str = "Java";
    pub const JSP: &str = "Java Server Pages";
    pub const JAVASCRIPT: &str = "JavaScript";
    pub const JSON: &str = "JSON";
    pub const JSON_LD: &str = "JSON-LD";
    pub const JSX: &str = "JSX";
    pub const JINJA2: &str = "Jinja2";
    pub const JULIA: &str = "Julia";
    pub const KOTLIN: &str = "Kotlin";
    pub const LESS: &str = "LESS";
    pub const LIVESCRIPT: &str = "LiveScript";
    pub const LUA: &str = "Lua";
    pub const MARKDOWN: &str = "Markdown";
    pub const MIRC: &str = "mIRC";
    pub const MARIA_DB: &str = "MariaDB SQL";
    pub const MATHEMATICA: &str = "Mathematica";
    pub const MODELICA: &str = "Modelica";
    pub const MUMPS: &str = "MUMPS";
    pub const MS_SQL: &str = "MS SQL";
    pub const MBOX: &str = "mbox";
    pub const MYSQL: &str = "MySQL";
    pub const NGINX: &str = "Nginx";
    pub const NSIS: &str = "NSIS";
    pub const NTRIPLES: &str = "NTriples";
    pub const OBJ_C: &str = "Objective-C";
    pub const OCAML: &str = "OCaml";
    pub const OCTAVE: &str = "Octave";
    pub const OZ: &str = "Oz";
    pub const PASCAL: &str = "Pascal";
    pub const PEG_JS: &str = "PEG.js";
    pub const PERL: &str = "Perl";
    pub const PHP: &str = "PHP";
    pub const PIG: &str = "Pig";
    pub const PLSQL: &str = "PLSQL";
    pub const POWERSHELL: &str = "PowerShell";
    pub const INI: &str = "Properties files";
    pub const PROTOBUF: &str = "ProtoBuf";
    pub const PYTHON: &str = "Python";
    pub const PUPPET: &str = "Puppet";
    pub const QLANG: &str = "Q";
    pub const RSCRIPT: &str = "R";
    pub const RST: &str = "reStructuredText";
    pub const RPM_CHANGES: &str = "RPM Changes";
    pub const RPM_SPEC: &str = "RPM Spec";
    pub const RUBY: &str = "Ruby";
    pub const RUST: &str = "Rust";
    pub const SAS: &str = "SAS";
    pub const SASS: &str = "Sass";
    pub const SCALA: &str = "Scala";
    pub const SCHEME: &str = "Scheme";
    pub const SCSS: &str = "SCSS";
    pub const SHELL: &str = "Shell";
    pub const SIEVE: &str = "Sieve";
    pub const SLIM: &str = "Slim";
    pub const SMALLTALK: &str = "Smalltalk";
    pub const SMARTY: &str = "Smarty";
    pub const SOLR: &str = "Solr";
    pub const SML: &str = "SML";
    pub const SOY: &str = "Soy";
    pub const SPARQL: &str = "SPARQL";
    pub const SPREADSHEET: &str = "Spreadsheet";
    pub const SQL: &str = "SQL";
    pub const SQLITE: &str = "SQLite";
    pub const SQUIRREL: &str = "Squirrel";
    pub const STYLUS: &str = "Stylus";
    pub const SWIFT: &str = "SWIFT";
    pub const STEX: &str = "sTeX";
    pub const LATEX: &str = "LaTeX";
    pub const SYSTEM_VERILOG: &str = "SystemVerilog";
    pub const TCL: &str = "Tcl";
    pub const TEXTILE: &str = "Textile";
    pub const TIDDLYWIKI: &str = "TiddlyWiki";
    pub const TIKI_WIKI: &str = "Tiki Wiki";
    pub const TOML: &str = "TOML";
    pub const TORNADO: &str = "Tornado";
    pub const TROFF: &str = "troff";
    pub const TTCN: &str = "TTCN";
    pub const TTCN_CFG: &str = "TTCN_CFG";
    pub const TURTLE: &str = "Turtle";
    pub const TYPESCRIPT: &str = "TypeScript";
    pub const TYPESCRIPT_JSX: &str = "TypeScript-JSX";
    pub const TWIG: &str = "Twig";
    pub const WEB_IDL: &str = "Web IDL";
    pub const VB_NET: &str = "VB.NET";
    pub const VBSCRIPT: &str = "VBScript";
    pub const VELOCITY: &str = "Velocity";
    pub const VERILOG: &str = "Verilog";
    pub const VHDL: &str = "VHDL";
    pub const VUE: &str = "Vue.js Component";
    pub const XML: &str = "XML";
    pub const XQUERY: &str = "XQuery";
    pub const YACAS: &str = "Yacas";
    pub const YAML: &str = "YAML";
    pub const Z80: &str = "Z80";
    pub const MSCGEN: &str = "mscgen";
    pub const XU: &str = "xu";
    pub const MSGENNY: &str = "msgenny";
}
