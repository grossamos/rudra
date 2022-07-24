{
  description = "An openapi based integration test coverage tool.";
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs";
  };

  outputs = { self, nixpkgs }: 
    let
      # geberare a version string for each build
      lastModifiedDate = self.lastModifiedDate or self.lastModified or "19700101";
      version = builtins.substring 0 8 lastModifiedDate;

      # Magix to make it work on different build systems
      supportedSystems = [ "x86_64-linux" "x86_64-darwin" "aarch64-linux" "aarch64-darwin" ];
      forAllSystems = nixpkgs.lib.genAttrs supportedSystems;
      nixpkgsFor = forAllSystems (system: import nixpkgs { inherit system; });
    in {
      packages = forAllSystems (system:
        let
          pkgs = nixpkgsFor.${system};
        in {
          # Go build flake
          rudra = pkgs.rustPlatform.buildRustPackage {
            pname = "rudra";
            inherit version;
            src = ./.;
            cargoSha256 = "q5OoAOFHmcMbDYG/mf/BO9vR+Y2K5s56tVCQVGzlopw=";
          };
        }
      );
      #dockerImage = pkgs.dockerTools.buildImage {
        #name = "rudra";
        #tage = "latest";

        #contents = [ 
          #nixpkgsFor.${system}.bash  
          #nixpkgsFor.${system}.nginx
        #];

        #config = { 
          #WorkingDir = "/app";
          #Cmd = [ "${rudra}/bin/rudra" ]; 
        #};
      #};
      #devShell = forAllSystems(system: 
        #nixpkgsFor.${system}.mkShell {
          #buildInputs = [ 
            #nixpkgsFor.${system}.cargo
            #nixpkgsFor.${system}.rustc
            #nixpkgsFor.${system}.rustfmt
            #nixpkgsFor.${system}.rust-analyzer
            #nixpkgsFor.${system}.openssl
          #];
        #}
      #);
      defaultPackage = forAllSystems (system: self.packages.${system}.rudra);
    };
}
