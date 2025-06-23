pkgs:
pkgs.mkShell {
  name = "Poincare";
  packages = with pkgs; let
    valeWithStyles =
      pkgs.vale.withStyles
      (s: with s; [readability proselint write-good]);

    valeConfig = pkgs.writeText ".vale.ini" ''
      # .vale.ini
      # Style configuration for Markdown notes

      # Core settings
      MinAlertLevel = suggestion  # Show errors (most severe) and warnings

      # Global rules (applies to all files)
      [*]
      # Base styles combining technical writing and readability:
      BasedOnStyles = Readability, proselint, write-good, Vale

      # Enable/disable specific rules
      Vale.Spelling = YES  # Requires a .dic file in "styles/Vale/Spelling"
      Vale.Terminology = YES  # Enforce consistent terms (configure terms below)
      Vale.HeadingCapitalization = YES  # Title-case headings
      Vale.Passive = YES  # Flag passive voice
      Vale.Weasel = YES  # Vague language ("many believe...")
      write-good.TooWordy = YES  # Wordy phrases
      write-good.SentenceLength = YES  # Max ~40 words
      proselint.Typography.Symbols = YES  # Proper symbols (e.g., en-dash)

      # Markdown-specific rules
      [*.md]
      # Syntax validation
      Vale.Markdown = YES  # Validate MD syntax
      Vale.RawHTML = NO  # Allow HTML if needed
      Vale.ListIndentation = YES  # Consistent list nesting

      # Headings
      Vale.HeadingLength = YES  # Max 60 chars
      Vale.HeadingSequencing = YES  # Check hierarchy (h1 -> h2 -> h3)

      # Readability
      Vale.Exclamation = YES  # Max 1 exclamation per doc
      Vale.QuoteStyle = YES  # Smart quotes
      Vale.LineLength = YES  # Max 100 chars (adjustable)
      MaxLineLength = 100

      # Terminology enforcement (example terms)
      [vale.Terminology]
      # Add your preferred terms here
      terms = TODO, FIXME, Note:, Warning:
    '';
  in [
    nixd
    alejandra
    statix
    deadnix
    npins
    cargo
    rustToolchains.nightly
    bacon
    (pkgs.symlinkJoin {
      name = "vale";
      paths = [valeWithStyles];
      buildInputs = [pkgs.makeWrapper];
      postBuild = ''
        wrapProgram $out/bin/vale \
          --add-flags "--config='${valeConfig}'"
      '';
    })
  ];
}
