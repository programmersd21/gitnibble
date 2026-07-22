use std::collections::HashSet;
use std::path::Path;
use walkdir::WalkDir;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum DetectedRule {
    Language(&'static str),
    Framework(&'static str),
    Environment(&'static str),
    Os(&'static str),
}

impl DetectedRule {
    pub fn template_key(&self) -> &'static str {
        match self {
            DetectedRule::Language(s)
            | DetectedRule::Framework(s)
            | DetectedRule::Environment(s)
            | DetectedRule::Os(s) => s,
        }
    }

    pub fn category_label(&self) -> &'static str {
        match self {
            DetectedRule::Language(_) => "lang",
            DetectedRule::Framework(_) => "framework",
            DetectedRule::Environment(_) => "ide",
            DetectedRule::Os(_) => "os",
        }
    }
}

const SKIP_DIRS: &[&str] = &[".git", "node_modules", "target", "vendor", "dist", "build"];

pub struct WorkspaceScanner;

impl WorkspaceScanner {
    pub fn scan<P: AsRef<Path>>(root: P) -> Vec<DetectedRule> {
        let mut detected: HashSet<DetectedRule> = HashSet::new();

        let walker = WalkDir::new(root)
            .max_depth(3)
            .into_iter()
            .filter_entry(|e| {
                let name = e.file_name().to_string_lossy();
                !SKIP_DIRS.contains(&name.as_ref())
            })
            .filter_map(|e| e.ok());

        for entry in walker {
            let name = entry.file_name().to_string_lossy();
            match name.as_ref() {
                "Cargo.toml" => {
                    detected.insert(DetectedRule::Language("Rust"));
                }
                "package.json" => {
                    detected.insert(DetectedRule::Language("Node"));
                }
                "go.mod" => {
                    detected.insert(DetectedRule::Language("Go"));
                }
                "requirements.txt" | "pyproject.toml" | "Pipfile" => {
                    detected.insert(DetectedRule::Language("Python"));
                }
                "pom.xml" | "build.gradle" | "build.gradle.kts" => {
                    detected.insert(DetectedRule::Language("Java"));
                }
                "Gemfile" => {
                    detected.insert(DetectedRule::Language("Ruby"));
                }
                "CMakeLists.txt" => {
                    detected.insert(DetectedRule::Language("C++"));
                }
                "next.config.js" | "next.config.mjs" | "next.config.ts" => {
                    detected.insert(DetectedRule::Framework("Next"));
                }
                "vite.config.ts" | "vite.config.js" => {
                    detected.insert(DetectedRule::Framework("Vite"));
                }
                "bun.lockb" | "bunfig.toml" => {
                    detected.insert(DetectedRule::Framework("Bun"));
                }
                "deno.json" | "deno.jsonc" => {
                    detected.insert(DetectedRule::Framework("Deno"));
                }
                "svelte.config.js" | "svelte.config.ts" => {
                    detected.insert(DetectedRule::Framework("SvelteKit"));
                }
                "astro.config.mjs" | "astro.config.ts" | "astro.config.js" => {
                    detected.insert(DetectedRule::Framework("Astro"));
                }
                "remix.config.js" | "remix.config.ts" => {
                    detected.insert(DetectedRule::Framework("Remix"));
                }
                "expo.json" | "app.json" => {
                    detected.insert(DetectedRule::Framework("Expo"));
                }
                "pubspec.yaml" => {
                    detected.insert(DetectedRule::Language("Flutter"));
                }
                "ansible.cfg" => {
                    detected.insert(DetectedRule::Framework("Ansible"));
                }
                "main.tf" | ".terraform" => {
                    detected.insert(DetectedRule::Framework("Terraform"));
                }
                "Pulumi.yaml" => {
                    detected.insert(DetectedRule::Framework("Pulumi"));
                }
                "cdk.json" => {
                    detected.insert(DetectedRule::Framework("CDK"));
                }
                "serverless.yml" | "serverless.yaml" => {
                    detected.insert(DetectedRule::Framework("Serverless"));
                }
                "nx.json" => {
                    detected.insert(DetectedRule::Framework("Nx"));
                }
                "turbo.json" => {
                    detected.insert(DetectedRule::Framework("Turborepo"));
                }
                "tauri.conf.json" => {
                    detected.insert(DetectedRule::Framework("Tauri"));
                }
                "build.zig" | "build.zig.zon" => {
                    detected.insert(DetectedRule::Language("Zig"));
                }
                "gleam.toml" => {
                    detected.insert(DetectedRule::Language("Gleam"));
                }
                "flake.nix" | "default.nix" => {
                    detected.insert(DetectedRule::Environment("Nix"));
                }
                "BUILD" | "BUILD.bazel" | "WORKSPACE" | "WORKSPACE.bazel" => {
                    detected.insert(DetectedRule::Environment("Bazel"));
                }
                "Earthfile" => {
                    detected.insert(DetectedRule::Environment("Earthly"));
                }
                ".cursor" => {
                    detected.insert(DetectedRule::Environment("Cursor"));
                }
                "wasm-pack.toml" => {
                    detected.insert(DetectedRule::Framework("Wasm"));
                }
                "netlify.toml" => {
                    detected.insert(DetectedRule::Environment("Netlify"));
                }
                "vercel.json" | ".vercel" => {
                    detected.insert(DetectedRule::Environment("Vercel"));
                }
                "fly.toml" => {
                    detected.insert(DetectedRule::Environment("Fly"));
                }
                "Tiltfile" => {
                    detected.insert(DetectedRule::Environment("Tilt"));
                }
                "skaffold.yaml" | "skaffold.yml" => {
                    detected.insert(DetectedRule::Environment("Skaffold"));
                }
                "dagger.json" => {
                    detected.insert(DetectedRule::Environment("Dagger"));
                }
                "Chart.yaml" => {
                    detected.insert(DetectedRule::Environment("Helm"));
                }
                "Vagrantfile" => {
                    detected.insert(DetectedRule::Environment("Vagrant"));
                }
                "composer.json" => {
                    detected.insert(DetectedRule::Framework("Composer"));
                }
                "mix.exs" => {
                    detected.insert(DetectedRule::Language("Elixir"));
                }
                "stack.yaml" | "cabal.project" => {
                    detected.insert(DetectedRule::Language("Haskell"));
                }
                "manage.py" => {
                    detected.insert(DetectedRule::Framework("Django"));
                }
                "artisan" => {
                    detected.insert(DetectedRule::Framework("Laravel"));
                }
                "nuxt.config.ts" | "nuxt.config.js" => {
                    detected.insert(DetectedRule::Framework("Nuxt"));
                }
                "react-native.config.js" => {
                    detected.insert(DetectedRule::Framework("ReactNative"));
                }
                ".storybook" => {
                    detected.insert(DetectedRule::Environment("Storybook"));
                }
                "playwright.config.ts" | "playwright.config.js" => {
                    detected.insert(DetectedRule::Environment("Playwright"));
                }
                "cypress.config.ts" | "cypress.config.js" => {
                    detected.insert(DetectedRule::Environment("Cypress"));
                }
                "vitest.config.ts" | "vitest.config.js" => {
                    detected.insert(DetectedRule::Environment("Vitest"));
                }
                "jest.config.ts" | "jest.config.js" | "jest.config.cjs" => {
                    detected.insert(DetectedRule::Environment("Jest"));
                }
                "poetry.lock" => {
                    detected.insert(DetectedRule::Framework("Poetry"));
                }
                "uv.lock" => {
                    detected.insert(DetectedRule::Framework("UV"));
                }
                "environment.yml" | "conda.yml" => {
                    detected.insert(DetectedRule::Framework("Conda"));
                }
                "firebase.json" => {
                    detected.insert(DetectedRule::Framework("Firebase"));
                }
                "Dockerfile" => {
                    detected.insert(DetectedRule::Environment("Docker"));
                }
                ".vscode" => {
                    detected.insert(DetectedRule::Environment("VisualStudioCode"));
                }
                ".idea" => {
                    detected.insert(DetectedRule::Environment("JetBrains"));
                }
                ".DS_Store" => {
                    detected.insert(DetectedRule::Os("macOS"));
                }
                "Thumbs.db" => {
                    detected.insert(DetectedRule::Os("Windows"));
                }
                _ => {
                    if name.ends_with(".csproj") || name.ends_with(".sln") {
                        detected.insert(DetectedRule::Language("DotNet"));
                    } else if name.ends_with(".tf") {
                        detected.insert(DetectedRule::Framework("Terraform"));
                    }
                }
            }
        }

        let mut result: Vec<DetectedRule> = detected.into_iter().collect();
        result.sort_by_key(|r| r.template_key());
        result
    }
}
