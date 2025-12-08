{pkgs, toolchain, ...}:

pkgs.mkShell {
  buildInputs = [
    toolchain
  ];

  shellHook = ''
    echo "Cargo: $(cargo --version | head -n1)"
    echo "Rust: $(rustc --version)"
  '';
}
