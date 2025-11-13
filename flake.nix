{
  inputs.nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
  outputs =
    inputs:
    let
      systems = [
        "aarch64-linux"
        "aarch64-darwin"
        "x86_64-darwin"
        "x86_64-linux"
      ];
      forSystems = inputs.nixpkgs.lib.genAttrs systems;
      pkgsFor = system: (import inputs.nixpkgs { inherit system; });
      genPkgs = func: (forSystems (system: func (pkgsFor system)));
    in
    {
      packages = genPkgs (pkgs: {
        my-package = pkgs.rustPlatform.buildRustPackage {
          pname = "my-package";
          version = "0.1.0";
          # nativeBuildInputs = with pkgs; [];
          # buildInputs = with pkgs; [ ];
          src = ./.;
          hash = pkgs.lib.fakeHash;
          cargoHash = pkgs.lib.fakeHash;
        };
        download-input = pkgs.writeShellApplication {
          name = "download-input";
          runtimeInputs = with pkgs; [
            xh
            jq
            xxd
            openssl
          ];
          text = ''
            COOKIE_CACHE_DIR="''${XDG_CACHE_HOME:-$HOME/.cache}/everybody-codes"
            mkdir -p "$COOKIE_CACHE_DIR"

            event="$1"; shift
            quest="$1"; shift
            echo "Event $event, Quest $quest" >&2

            # TODO: inform user if cookie isn't working
            # load cookie
            cookie_cache_path="$COOKIE_CACHE_DIR/cookie.txt"
            if [[ ! -f $cookie_cache_path ]]; then
              nix run nixpkgs#bitwarden-cli -- config server https://bw.lyte.dev || true
              nix run nixpkgs#bitwarden-cli -- sync
              nix run nixpkgs#bitwarden-cli -- get password everybody.codes > "$cookie_cache_path.wip"
              mv "$cookie_cache_path.wip" "$cookie_cache_path"
            fi
            cookie="$(echo "$cookie_cache_path" | xargs cat | xargs)"

            # load seed
            if [[ ! -f $COOKIE_CACHE_DIR/seed.txt ]] || [[ $(cat "$COOKIE_CACHE_DIR/seed.txt") == 0 ]]; then
              xh "https://everybody.codes/api/user/me" "Cookie:everybody-codes=$cookie" \
                | jq -r .seed > "$COOKIE_CACHE_DIR/seed.txt"
            fi
            seed="$(xargs < "$COOKIE_CACHE_DIR/seed.txt")"
            echo "Seed: $seed" >&2

            # ensure directories are setup
            dir="$COOKIE_CACHE_DIR/$event/$quest"
            echo "dir: $dir"
            mkdir -p "$dir/key" "$dir/input"

            # retrieve inputs if needed
            if [[ ! -f "$dir/inputs.json" ]] || ! jq . "$dir/inputs.json"; then
              xh "https://everybody-codes.b-cdn.net/assets/$event/$quest/input/$seed.json" "Cookie:everybody-codes=$cookie" > "$dir/inputs.json"
            fi
            cat "$dir/inputs.json"
            for p in 1 2 3; do
              jq -r ".\"$p\"" "$dir/inputs.json" > "$dir/input/$p.encrypted.txt"
            done

            has_downloaded_keys_file="false"
            function download_keys {
              if [[ $has_downloaded_keys_file == false ]]; then
                # retrieve keys
                xh "https://everybody.codes/api/event/$event/quest/$quest" "Cookie:everybody-codes=$cookie" > "$dir/keys.json"
                has_downloaded_keys_file="true"
              fi
            }

            # decrypt inputs to plaintext where possible
            # TODO: only download keys if we don't already have them all?
            download_keys
            for p in 1 2 3; do
              if jq -r ".key$p" "$dir/keys.json" > "$dir/key/$p.txt"; then
                # we have the key, so decrypt and store the input
                key="$(cat "$dir/key/$p.txt")"
                if [[ $key == null ]]; then
                  rm "$dir/key/$p.txt"
                  continue
                fi
                echo "Event $event, Quest $quest, Part $p, Key: $key"
                iv="''${key:0:16}"
                echo "IV: $iv"
                xxd -r -p < "$dir/input/$p.encrypted.txt" \
                  | openssl enc -aes-256-cbc -d \
                      -K "$(echo -n "$key" | xxd -p | tr -d '\n ')" \
                      -iv "$(echo -n "$iv" | xxd -p | tr -d '\n ')" \
                  > "$dir/input/$p.txt"
              fi
            done
          '';
        };
        default = inputs.self.packages.${pkgs.system}.my-package;
      });
      devShells = genPkgs (pkgs: {
        default = pkgs.mkShell {
          inputsFrom = [ inputs.self.packages.${pkgs.system}.default ];
          packages =
            with pkgs;
            [
              rustPackages.clippy
              rust-analyzer
              rustfmt
            ]
            ++ (with inputs.self.outputs.packages.${pkgs.system}; [
              download-input
            ]);
        };
      });
    };
}
