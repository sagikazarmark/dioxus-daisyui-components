{ pkgs, ... }:

{
  dotenv.enable = true;

  dagger.enable = true;
  env.DAGGER_X_RELEASE = "v1.0.0-beta.11";

  # The Preview's syntax highlighting compiles arborium's tree-sitter runtime
  # for wasm32, and `cc` needs a clang that targets it. Nix's wrapped clang
  # adds host flags a wasm target rejects, so this is the unwrapped one
  # (ADR-0033).
  env.CC_wasm32_unknown_unknown = "${pkgs.llvmPackages.clang-unwrapped}/bin/clang";

  packages = with pkgs; [
    lld
    cargo-audit
    cargo-deny
    cargo-dist
    cargo-release
    cargo-watch

    dioxus-cli
  ];

  languages = {
    rust = {
      enable = true;
      channel = "stable";
      targets = [ "wasm32-unknown-unknown" ];
    };

    javascript = {
      enable = true;

      npm.enable = true;
    };
  };
}
