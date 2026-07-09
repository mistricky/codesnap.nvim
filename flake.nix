{
  description = "CodeSnap.nvim - pretty code snapshots for Neovim";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
  };

  outputs = {
    self,
    nixpkgs,
  }: let
    # Systems for which the generator is compiled and the plugin is provided.
    systems = [
      "x86_64-linux"
      "aarch64-linux"
      "x86_64-darwin"
      "aarch64-darwin"
    ];

    forAllSystems = f:
      nixpkgs.lib.genAttrs systems (system:
        f {
          inherit system;
          pkgs = import nixpkgs {inherit system;};
        });

    # Plugin version, read from project.toml so it stays in sync with releases.
    version = let
      toml = builtins.readFile ./project.toml;
      m = builtins.match ".*version[[:space:]]*=[[:space:]]*\"([^\"]+)\".*" toml;
    in
      if m == null
      then "0.0.0"
      else builtins.head m;
  in {
    # The Rust `generator` crate, built as a native Lua module (cdylib).
    packages = forAllSystems ({
      pkgs,
      system,
    }: let
      inherit (pkgs) lib stdenv;

      generator = pkgs.rustPlatform.buildRustPackage {
        pname = "codesnap-generator";
        inherit version;
        src = self;
        sourceRoot = "${self.sourceInfo.name or "source"}/generator";
        cargoLock.lockFile = ./generator/Cargo.lock;

        nativeBuildInputs = [
          pkgs.pkg-config
          pkgs.rustPlatform.bindgenHook
        ];

        buildInputs = [
          pkgs.libuv.dev
          pkgs.openssl
        ];

        env = {
          # Use the system OpenSSL rather than the vendored copy pinned in
          # generator/Cargo.toml (vendoring is only needed for cross-compiled
          # release binaries, not for a native Nix build).
          OPENSSL_NO_VENDOR = 1;

          # On Darwin, undefined symbols are resolved at load time against
          # Neovim's LuaJIT runtime.
          RUSTFLAGS = lib.optionalString stdenv.hostPlatform.isDarwin "-C link-arg=-undefined -C link-arg=dynamic_lookup";
        };

        doCheck = false;
      };

      libExt =
        if stdenv.hostPlatform.isDarwin
        then "dylib"
        else "so";

      # The <os>-<arch>_generator.<ext> name that lua/codesnap/fetch.lua
      # (get_platform_lib_name) looks for in the libs directory.
      libName = let
        osArch =
          {
            "x86_64-linux" = "linux-x86_64";
            "aarch64-linux" = "linux-aarch64";
            "x86_64-darwin" = "mac-x86_64";
            "aarch64-darwin" = "mac-aarch64";
          }.${
            system
          };
      in "${osArch}_generator.${libExt}";

      plugin = pkgs.vimUtils.buildVimPlugin {
        pname = "codesnap.nvim";
        inherit version;
        src = self;
        doCheck = false;

        # Provision the Nix-built generator where the runtime loader expects
        # a pre-built library (lua/libs/<os>-<arch>_generator.<ext>), plus the
        # matching .version marker. With both present, lua/codesnap/fetch.lua
        # uses the library directly and never attempts a GitHub release
        # download at runtime.
        postInstall = ''
          mkdir -p $out/lua/libs
          ln -s ${generator}/lib/libgenerator.${libExt} $out/lua/libs/${libName}
          printf '%s' "${version}" > $out/lua/libs/.version
        '';
      };
    in {
      inherit generator;
      default = plugin;
    });

    # `nix flake check` builds the plugin and asserts that Neovim can require
    # the plugin and successfully load the native generator module.
    checks = forAllSystems ({
      pkgs,
      system,
    }: {
      plugin-loads = let
        plugin = self.packages.${system}.default;
        nvim = pkgs.neovim;
      in
        pkgs.runCommand "codesnap-plugin-loads" {
          nativeBuildInputs = [nvim];
        } ''
          export HOME=$TMPDIR
          nvim --headless --clean \
            --cmd "set runtimepath^=${plugin}" \
            -c "lua require('codesnap').setup({})" \
            -c "lua assert(require('codesnap.module').load_generator() ~= nil, 'generator failed to load')" \
            -c "qa!" 2> $TMPDIR/err || (cat $TMPDIR/err; exit 1)
          echo "codesnap.nvim loaded and generator module resolved" > $out
        '';
    });

    devShells = forAllSystems ({
      pkgs,
      system,
    }: {
      default = pkgs.mkShell {
        inputsFrom = [self.packages.${system}.generator];
        packages = with pkgs; [
          cargo
          rustc
          rust-analyzer
          rustfmt
          clippy
          stylua
        ];
      };
    });

    formatter = forAllSystems ({pkgs, ...}: pkgs.alejandra);
  };
}
