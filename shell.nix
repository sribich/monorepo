{
  pkgs,
  self,
  system,
}:
{
  default = pkgs.mkShell (
    let
      rustToolchain = pkgs.rust-bin.selectLatestNightlyWith (
        toolchain:
        toolchain.default.override {
          extensions = [
            "cargo"
            "clippy"
            "rust-src"
            "rustc"
            "rustfmt"
            "llvm-tools-preview"
            "rustc-codegen-cranelift-preview"
            "miri"
          ];
          # targets = [ "arm-unknown-linux-gnueabihf" ];
        }
      );
      rustPlatform = pkgs.makeRustPlatform {
        cargo = rustToolchain;
        rustc = rustToolchain;
      };

      mdx-language-server = (
        pkgs.buildNpmPackage rec {
          pname = "mdx-language-server";
          version = "0.6.3";
          src = pkgs.fetchurl {
            url = "https://registry.npmjs.org/@mdx-js/language-server/-/language-server-${version}.tgz";
            hash = "sha256-rNYJYQjnA7u02nP4a7EL/yJbjGdwP0RLQpAhr/I9xLs";
          };
          postPatch = ''
            ln -s ${./patches/mdx-language-server-package-lock.json} package-lock.json
          '';
          npmDepsHash = "sha256-1P5JMUOZiXtiue5C+vIEMGKt/wNBy8qf9Bo6Mfo8+Rg";
          dontNpmBuild = true;
        }
      );
    in
    {
      NIX_LD = "${pkgs.stdenv.cc.libc}/lib/ld-linux-x86-64.so.2";

      packages = [
        rustToolchain
      ]
      ++ (with pkgs; [
        moon
        nodejs_24

        # nodePackages.pnpm
        pnpm
        mdx-language-server

        gst_all_1.gstreamer
        # Common plugins like "filesrc" to combine within e.g. gst-launch
        gst_all_1.gst-plugins-base
        # Specialized plugins separated by quality
        gst_all_1.gst-plugins-good
        gst_all_1.gst-plugins-bad

        # rust-analyzer-nightly
        cargo-binstall
        cargo-llvm-cov
        cargo-nextest
        cargo-mutants
        cargo-machete
        cargo-xbuild

        cargo-watch
        unstable.bacon

        cargo-audit
        cargo-deny
        # cargo-geiger
        cargo-outdated
        cargo-insta
        cargo-hack
        grcov
        bunyan-rs
        valgrind
        cargo-valgrind

        mecab
        curl
        wget
        pkg-config
        dbus
        # openssl_3
        openssl
        glib
        gtk3

        # webkitgtk
        webkitgtk_4_1
        librsvg
        gmp
        gnum4
        alsa-lib
        lld
        binutils
        mold
        mpv
        # Needed to build whisper
        gcc13
        clang
        libclang
        cmake
        ffmpeg_7-full

        playwright-driver.browsers
      ]);

      buildInputs = with pkgs; [
        curl
        wget
        pkg-config
        dbus
        # openssl_3
        openssl
        glib
        gtk3

        # webkitgtk
        webkitgtk_4_1
        librsvg
        gcc13

        ffmpeg_7-full

      ];

      shellHook =
        let
          libraries = with pkgs; [
            libsoup_3

            ffmpeg_7-full

            # webkitgtk
            webkitgtk_4_1
            gtk3
            cairo
            gdk-pixbuf
            glib
            dbus
            # openssl_3
            openssl
            librsvg
            binutils
            mpv

            # Needed to run cargo-interactive-update
            curl

            cmake
            clang
            libclang
            # gcc-unwrapped
            gcc13

            # versoview
            xorg.libXcursor
            xorg.libXrandr
            xorg.libXi
            libxkbcommon
            libGL
          ];
        in
        ''
          export GIO_MODULE_DIR="${pkgs.glib-networking}/lib/gio/modules/"

          export LD_LIBRARY_PATH=${pkgs.lib.makeLibraryPath libraries}:$LD_LIBRARY_PATH
          export XDG_DATA_DIRS=${pkgs.gsettings-desktop-schemas}/share/gsettings-schemas/${pkgs.gsettings-desktop-schemas.name}:${pkgs.gtk3}/share/gsettings-schemas/${pkgs.gtk3.name}:$XDG_DATA_DIRS

          # webkitgtk 2.42 has problems with with nvidia drivers
          export WEBKIT_DISABLE_DMABUF_RENDERER=1

          export MOON_TOOLCHAIN_FORCE_GLOBALS=true


          export PLAYWRIGHT_BROWSERS_PATH=${pkgs.playwright-driver.browsers}
          export PLAYWRIGHT_SKIP_VALIDATE_HOST_REQUIREMENTS=true
          export PLAYWRIGHT_HOST_PLATFORM_OVERRIDE="ubuntu-24.04"
        '';
    }
  );
}
