use std::fs;
use std::path::{Path, PathBuf};

use dioxus_registry_preview::validation::{
    Diagnostic, DiagnosticCode, MarkdownOptions, load_site_from_catalog, readme_parts,
};
use regex::Regex;
use serde::Deserialize;
use serde_json::Value;

fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

#[test]
fn component_documentation_follows_registry_policy() {
    let mut validation = load_site_from_catalog(
        &root().join("src/preview/pages/mod.rs"),
        &MarkdownOptions::default(),
    );
    let state_bridging = Regex::new("Tier [12]|no state to bridge").unwrap();

    if let Some(site) = &validation.value {
        for readme in &site.readmes {
            for heading in [
                "State bridging",
                "Deviations",
                "daisyUI classes deliberately not used",
            ] {
                if readme_section(&readme.source, heading).is_none() {
                    validation.diagnostics.push(
                        Diagnostic::new(
                            DiagnosticCode::MissingRequiredSection,
                            format!("README has no '## {heading}' section"),
                        )
                        .at_path(&readme.path),
                    );
                }
            }

            let valid_state_bridging = readme_section(&readme.source, "State bridging")
                .is_some_and(|section| state_bridging.is_match(section));
            if !valid_state_bridging {
                validation.diagnostics.push(
                    Diagnostic::new(
                        DiagnosticCode::SectionPatternMismatch,
                        "README section '## State bridging' must name its Tier or say there is no state to bridge",
                    )
                    .at_path(&readme.path),
                );
            }

            // The Example is where the code lives (ADR-0009), so a README opens
            // on the links to it rather than on a synopsis nothing compiles.
            // Prose may still quote a fragment further down, where it is
            // illustrating the argument around it rather than standing in for
            // an Example.
            let expected = readme_links(&readme.member_path);
            if readme_parts(&readme.source)
                .is_none_or(|(_, body)| !body.trim_start().starts_with(&expected))
            {
                validation.diagnostics.push(
                    Diagnostic::new(
                        DiagnosticCode::SectionPatternMismatch,
                        format!(
                            "README must follow its introduction with the Preview and Example links, not a synopsis:\n{expected}"
                        ),
                    )
                    .at_path(&readme.path),
                );
            }
        }
    }

    assert_valid(validation.diagnostics);
}

/// The links a Component README carries between its introduction and its prose:
/// its page in the deployed Preview, and the Examples that page is built from.
fn readme_links(member_path: &Path) -> String {
    let component = member_path
        .file_name()
        .and_then(|name| name.to_str())
        .expect("every Registry member path names a Component directory");

    format!(
        "[Live examples](https://daisyui-components.dioxus.cc/components/{component}) ·\n[their sources](docs/examples/)"
    )
}

#[test]
fn components_declare_the_primitive_dependency_identically() {
    #[derive(Deserialize)]
    struct RegistryManifest {
        members: Vec<PathBuf>,
    }

    #[derive(Deserialize)]
    #[serde(rename_all = "camelCase")]
    struct ComponentManifest {
        cargo_dependencies: Vec<Value>,
    }

    let root = root();
    let registry: RegistryManifest = read_json(&root.join("component.json"));
    assert!(
        !registry.members.is_empty(),
        "the root manifest lists no members"
    );

    let declarations: Vec<_> = registry
        .members
        .iter()
        .map(|member| {
            let component: ComponentManifest = read_json(&root.join(member).join("component.json"));
            let matching: Vec<_> = component
                .cargo_dependencies
                .into_iter()
                .filter(|dependency| dependency_name(dependency) == Some("dioxus-primitives"))
                .collect();
            assert_eq!(
                matching.len(),
                1,
                "{} declares dioxus-primitives {} times",
                member.display(),
                matching.len()
            );
            (member, matching.into_iter().next().unwrap())
        })
        .collect();
    let expected = &declarations[0].1;
    let different: Vec<_> = declarations
        .iter()
        .filter(|(_, declaration)| declaration != expected)
        .collect();

    assert!(
        different.is_empty(),
        "components declare dioxus-primitives differently: {different:#?}"
    );
}

#[test]
fn field_aware_components_declare_the_field_dependency_identically() {
    #[derive(Deserialize)]
    #[serde(rename_all = "camelCase")]
    struct ComponentManifest {
        cargo_dependencies: Vec<Value>,
    }

    let root = root();
    let expected = serde_json::json!({
        "name": "dioxus-field",
        "version": "0.6.0"
    });

    for component in [
        "src/components/input",
        "src/components/checkbox",
        "src/components/combobox",
        "src/components/field",
        "src/components/otp",
        "src/components/radio_group",
        "src/components/switch",
        "src/components/textarea",
        "src/components/select",
        "src/components/slider",
    ] {
        let manifest: ComponentManifest = read_json(&root.join(component).join("component.json"));
        let matching: Vec<_> = manifest
            .cargo_dependencies
            .into_iter()
            .filter(|dependency| dependency_name(dependency) == Some("dioxus-field"))
            .collect();
        assert_eq!(
            matching.as_slice(),
            std::slice::from_ref(&expected),
            "{component} must declare the pinned dioxus-field dependency once"
        );
    }
}

fn read_json<T: for<'de> Deserialize<'de>>(path: &Path) -> T {
    serde_json::from_str(&fs::read_to_string(path).unwrap()).unwrap()
}

fn dependency_name(dependency: &Value) -> Option<&str> {
    match dependency {
        Value::String(name) => Some(name),
        Value::Object(fields) => fields.get("name").and_then(Value::as_str),
        _ => None,
    }
}

fn readme_section<'a>(readme: &'a str, heading: &str) -> Option<&'a str> {
    let marker = format!("## {heading}");
    let start = readme
        .match_indices(&marker)
        .find(|(index, _)| {
            let starts_line = *index == 0 || readme.as_bytes()[index - 1] == b'\n';
            let end = index + marker.len();
            let ends_heading =
                end == readme.len() || matches!(readme.as_bytes()[end], b'\n' | b'\r');
            starts_line && ends_heading
        })?
        .0;
    let body_start = readme[start + marker.len()..]
        .find('\n')
        .map(|offset| start + marker.len() + offset + 1)
        .unwrap_or(readme.len());
    let body = &readme[body_start..];
    let end = body
        .match_indices("\n## ")
        .next()
        .map_or(body.len(), |(index, _)| index);
    Some(&body[..end])
}

fn assert_valid(diagnostics: Vec<Diagnostic>) {
    if diagnostics.is_empty() {
        return;
    }

    let rendered = diagnostics
        .iter()
        .map(ToString::to_string)
        .collect::<Vec<_>>()
        .join("\n");
    panic!("Registry documentation is invalid:\n{rendered}");
}

#[test]
fn component_props_are_documented() {
    let manifest: Value = read_json(&root().join("component.json"));
    let mut undocumented = Vec::new();

    for member in manifest["members"].as_array().unwrap() {
        let path = root().join(member.as_str().unwrap()).join("component.rs");
        let source = fs::read_to_string(&path).unwrap();
        for (component, props) in component_props(&source) {
            for prop in props {
                if !prop.documented {
                    undocumented.push(format!("{}: {component}.{}", path.display(), prop.name));
                }
            }
        }
    }

    assert!(
        undocumented.is_empty(),
        "every prop except `attributes` and `children` needs a `///` comment (ADR-0029):\n{}",
        undocumented.join("\n")
    );
}

struct Prop {
    name: String,
    documented: bool,
}

/// Every `#[component]` function's props, read from its signature.
///
/// A prop is one comma-separated parameter, together with the doc comment and
/// attributes above it. Brackets are only counted on code lines, so a doc
/// comment's prose cannot unbalance them.
fn component_props(source: &str) -> Vec<(String, Vec<Prop>)> {
    let name_of = Regex::new(r"^pub fn (\w+)(?:<[^(]*>)?\s*\(").unwrap();
    let prop_name = Regex::new(r"(\w+)\s*:[^:]").unwrap();
    let mut components = Vec::new();
    let mut lines = source.lines().peekable();

    while let Some(line) = lines.next() {
        if line.trim() != "#[component]" {
            continue;
        }
        let Some(signature) = lines.find(|line| line.starts_with("pub fn ")) else {
            break;
        };
        let component = name_of.captures(signature).unwrap()[1].to_string();
        let mut props = Vec::new();
        let mut chunk: Vec<&str> = Vec::new();
        let mut depth = 0i32;

        if let Some(inline) = signature.split_once('(').map(|(_, rest)| rest)
            && let Some(params) = inline
                .split(") ->")
                .next()
                .filter(|_| inline.contains(") ->"))
            {
                for param in params.split(',') {
                    if let Some(name) = prop_name.captures(param) {
                        props.push(Prop {
                            name: name[1].to_string(),
                            documented: false,
                        });
                    }
                }
                components.push((component, skip_slots(props)));
                continue;
            }

        for line in lines.by_ref() {
            if line.starts_with(") ->") {
                break;
            }
            let code = line.trim();
            chunk.push(line);
            if code.starts_with("//") {
                continue;
            }
            for character in code.replace("->", "  ").chars() {
                match character {
                    '(' | '<' | '[' | '{' => depth += 1,
                    ')' | '>' | ']' | '}' => depth -= 1,
                    _ => {}
                }
            }
            if depth == 0 && code.ends_with(',') {
                props.push(prop_from(&chunk, &prop_name));
                chunk.clear();
            }
        }
        if chunk.iter().any(|line| !line.trim().is_empty()) {
            props.push(prop_from(&chunk, &prop_name));
        }
        components.push((component, skip_slots(props)));
    }

    components
}

fn prop_from(chunk: &[&str], prop_name: &Regex) -> Prop {
    let documented = chunk.iter().any(|line| line.trim().starts_with("///"));
    let code = chunk
        .iter()
        .filter(|line| !line.trim().starts_with("//"))
        .copied()
        .collect::<Vec<_>>()
        .join("\n");
    let name = prop_name
        .captures(&code)
        .map(|captures| captures[1].to_string())
        .unwrap_or_else(|| code.trim().to_string());
    Prop { name, documented }
}

fn skip_slots(props: Vec<Prop>) -> Vec<Prop> {
    props
        .into_iter()
        .filter(|prop| prop.name != "attributes" && prop.name != "children")
        .collect()
}

#[test]
fn focus_handlers_do_not_commit() {
    let manifest: Value = read_json(&root().join("component.json"));
    let mut committing = Vec::new();

    for member in manifest["members"].as_array().unwrap() {
        let path = root().join(member.as_str().unwrap()).join("component.rs");
        let source = fs::read_to_string(&path).unwrap();
        for (handler, line) in commits_in_focus_handlers(&source) {
            committing.push(format!("{}:{line}: inside {handler}", path.display()));
        }
    }

    assert!(
        committing.is_empty(),
        "Commit and Focus Exit are independent (ADR-0028), so a focus handler cannot Commit: \
         leaving an unchanged control would run Commit validation over a value no interaction \
         produced\n{}",
        committing.join("\n")
    );
}

/// Every Commit reached from inside a focus handler, as its handler and line.
///
/// A handler runs from its `onfocusin` or `onfocusout` line until its braces
/// balance again, so a Commit inside a nested closure or a spawned task counts
/// the same as one written directly in the body.
fn commits_in_focus_handlers(source: &str) -> Vec<(String, usize)> {
    let handler_name = Regex::new(r"^\s*(onfocus(?:in|out)):").unwrap();
    let commit = Regex::new(r"\.commit\(\)|on_commit").unwrap();
    let mut found = Vec::new();
    let mut open: Option<(String, isize)> = None;

    for (index, line) in source.lines().enumerate() {
        if open.is_none() {
            match handler_name.captures(line) {
                Some(captures) => open = Some((captures[1].to_string(), 0)),
                None => continue,
            }
        }
        let Some((name, depth)) = &mut open else {
            continue;
        };
        if commit.is_match(line) {
            found.push((name.clone(), index + 1));
        }
        *depth += line.matches('{').count() as isize;
        *depth -= line.matches('}').count() as isize;
        if *depth <= 0 {
            open = None;
        }
    }

    found
}
