-- UNSTABLE SPECIFICATION FOR ALEPH Configuration
-- works without the return but my lsp will complain
return {
  description = "Minimal Lua";

  buckets = {
    main = "https://github.com/ScoopInstaller/Main/archive/68898739d017dfed8fcf7c096c48fe5d829c7bbd.zip",
    extras = "https://github.com/ScoopInstaller/Extras/archive/8c3c91d622775f8bafc9ee6051c7ccc016fe7ec7.zip",
  };

  -- could complicate this more later and follow the flake spec without the .${SYSTEM}.
  -- thingy maybe
  -- list of package names, can specify the bucket by adding 
  -- using "<bucketName>.packageName"
  packages = {
    "cowsay",
    "eza",
    "notepadplusplus",
    
  };
}
