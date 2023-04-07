{ root, lib, haumea }:

let
  inherit (builtins)
    attrNames
    concatStringsSep
    mapAttrs
    pathExists
    readFile
    toJSON
    trace
    tryEval
    ;
  inherit (lib)
    flip
    filterAttrs
    hasPrefix
    id
    removePrefix
    ;
in

{ flake, dir ? "tests", ... }@args:

let
  tests = haumea.load (removeAttrs args [ "flake" "dir" ] // {
    src = flake + "/${dir}";
  });

  results = flip mapAttrs tests (name: { format ? "json", expr }:
    assert hasPrefix "." name
      -> throw "invalid snapshot '${name}', names should not start with '.'";

    let
      path = flake + "/${dir}/_snapshots/${name}";
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

  failures = attrNames (filterAttrs (_: res: res ? value) results);
in

assert trace "namaka=${toJSON { inherit dir results; }}" true;

if failures == [ ] then
  { }
else
  throw "the following tests failed: ${concatStringsSep "," failures}"
