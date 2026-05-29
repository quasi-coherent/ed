/* App-wide knowledge of whether a corpus has been ingested and the current
   StyleFingerprint. Views read via `useCorpus`; Upload pushes a new
   fingerprint via `setFingerprint` after a successful ingest. */

type status =
  | Loading
  | Ready(option(Api.styleFingerprint))
  | Errored(string);

type t = {
  status,
  setFingerprint: option(Api.styleFingerprint) => unit,
  refresh: unit => unit,
};

let defaultValue: t = {
  status: Loading,
  setFingerprint: _ => (),
  refresh: () => (),
};

let context = React.createContext(defaultValue);

module ContextProvider = {
  let make = React.Context.provider(context);
  let makeProps = React.Context.makeProps;
};

[@react.component]
let make = (~children) => {
  let (status, setStatus) = React.useState(() => Loading);

  let load =
    React.useCallback0(() => {
      setStatus(_ => Loading);
      let _ =
        Client.getFingerprint()
        |> Js.Promise.then_(res => {
             switch (res) {
             | Ok(fp) => setStatus(_ => Ready(fp))
             | Error(e) => setStatus(_ => Errored(Client.errorToString(e)))
             };
             Js.Promise.resolve();
           });
      ();
    });

  React.useEffect0(() => {
    load();
    None;
  });

  let setFingerprint = React.useCallback0(fp => setStatus(_ => Ready(fp)));

  let value = {
    status,
    setFingerprint,
    refresh: load,
  };

  <ContextProvider value> children </ContextProvider>;
};

let useCorpus = () => React.useContext(context);

let hasCorpus = (c: t): bool =>
  switch (c.status) {
  | Ready(Some(_)) => true
  | _ => false
  };
