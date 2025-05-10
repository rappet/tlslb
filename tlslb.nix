{
  pkgs ? import <nixpkgs> { },
  lib,
  stdenv ? pkgs.stdenv,
  # A set providing `buildRustPackage :: attrsets -> derivation`
  rustPlatform ? pkgs.rustPlatform,
  fetchFromGitHub ? pkgs.fetchFromGitHub,
  pkg-config ? pkgs.pkg-config,
  installShellFiles ? pkgs.installShellFiles,
  libiconv,
}:

rustPlatform.buildRustPackage rec {
  pname = "tlslb";
  version = "0.0.1";

  src = ./.;

  cargoLock = {
    lockFile = ./Cargo.lock;
  };

  nativeBuildInputs = [
    pkg-config
    installShellFiles
  ];

  buildInputs = lib.optionals stdenv.isDarwin [ libiconv ];

  postInstall = ''
    installManPage man/${pname}.1
    installManPage man/${pname}.conf.5
    installShellCompletion completions/${pname}.{bash,fish}
    installShellCompletion completions/_${pname}
  '';

  meta = with lib; {
    homepage = "https://rappet.xyz/";
    description = "TCP/TLS load balancer";
    license = licenses.mit;
  };
}
