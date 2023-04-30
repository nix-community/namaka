{ root, lib, haumea }:

let
  inherit (builtins)
    attrNames
    concatStringsSep
    filter
    mapAttrs
    pathExists
    readFile
    storeDir
    tail
    toJSON
    trace
    tryEval
    ;
  inherit (lib)
    filterAttrs
    flip
    hasPrefix
    id
    pipe
    removePrefix
    splitString
    warn
    ;
in

args:

let
  src = toString (
    args.src or (warn
      "namaka.load: `flake` and `dir` have been deprecated, use `src` directly instead"
      (args.flake + "/${args.dir or "tests"}"))
  );

  tests = haumea.load (removeAttrs args [ "flake" "dir" ] // {
    inherit src;
  });

  results = flip mapAttrs tests (name: { format ? "json", expr }:
    assert hasPrefix "." name
      -> throw "invalid snapshot '${name}', names should not start with '.'";

    let
      path = "${src}/_snapshots/${name}";
      old = pathExists path;
      snap = readFile path;
      prefix = "#${format}\n";
      f = root.formats.${format};
      value = (f.serialize or id) expr;
      expected = (f.parse or id) (removePrefix prefix snap);
    in

    if old && hasPrefix prefix snap
      && tryEval expected == { inherit value; success = true; } then
      true
    else {
      inherit format value old;
    });

  msg = {
    dir = pipe src [
      (removePrefix storeDir)
      (splitString "/")
      (filter (x: x != ""))
      tail
      (concatStringsSep "/")
    ];
    inherit results;
  };

  failures = attrNames (filterAttrs (_: res: res ? value) results);
in

assert trace "namaka=${toJSON msg}" true;

if failures == [ ] then
  { }
else
  throw "the following tests failed: ${concatStringsSep "," failures}"
