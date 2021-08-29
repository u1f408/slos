let
  rust' = { channel = "nightly"; date = "2021-08-28"; };

in self: super: {
  rust-nightly-bin = (super.rustChannelOf {
    inherit (rust') channel date;
  }).rust;
}
