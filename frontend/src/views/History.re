[@mel.module "./History.module.css"] external styles: Js.t({..}) = "default";

type loadState =
  | Loading
  | Loaded(Api.historyList)
  | Failed(string);

let pageSize = 20;

let formatTimestamp = (iso: string): string => {
  let d: Js.Date.t = Js.Date.fromString(iso);
  Js.Date.toLocaleString(d);
};

let scoreClass = (s: float) =>
  if (s >= 0.75) {
    styles##high;
  } else if (s >= 0.5) {
    styles##mid;
  } else {
    styles##low;
  };

[@react.component]
let make = () => {
  let corpus = CorpusContext.useCorpus();
  let (state, setState) = React.useState(() => Loading);
  let (offset, setOffset) = React.useState(() => 0);
  let (expandedId, setExpandedId) = React.useState(() => None);

  let corpusFingerprint =
    switch (corpus.status) {
    | Ready(Some(fp)) => Some(fp)
    | _ => None
    };

  React.useEffect1(
    () => {
      setState(_ => Loading);
      let _ =
        Client.getHistory(~limit=pageSize, ~offset, ())
        |> Js.Promise.then_(res => {
             switch (res) {
             | Ok(hl) => setState(_ => Loaded(hl))
             | Error(e) => setState(_ => Failed(Client.errorToString(e)))
             };
             Js.Promise.resolve();
           });
      None;
    },
    [|offset|],
  );

  let body =
    switch (state) {
    | Loading =>
      <div className={styles##loading}> {React.string("Loading...")} </div>
    | Failed(msg) =>
      <div className={styles##error}> {React.string(msg)} </div>
    | Loaded(hl) when Array.length(hl.entries) == 0 =>
      <div className={styles##empty}>
        {React.string("No generations yet.")}
      </div>
    | Loaded(hl) =>
      let entries =
        hl.entries
        |> Array.map((entry: Api.historyEntry) => {
             let expanded =
               switch (expandedId) {
               | Some(id) => id == entry.simulationId
               | None => false
               };
             let pct = int_of_float(entry.result.confidence.overall *. 100.0);
             <div className={styles##entry} key={entry.simulationId}>
               <button
                 className={styles##entryHead}
                 onClick={_ =>
                   setExpandedId(cur =>
                     switch (cur) {
                     | Some(id) when id == entry.simulationId => None
                     | _ => Some(entry.simulationId)
                     }
                   )
                 }>
                 <div>
                   <div className={styles##prompt}>
                     {React.string(entry.prompt)}
                   </div>
                   <div className={styles##preview}>
                     {React.string(entry.result.generatedText)}
                   </div>
                 </div>
                 <AudienceBadge audience={entry.audience} />
                 <span
                   className={
                     styles##score
                     ++ " "
                     ++ scoreClass(entry.result.confidence.overall)
                   }>
                   {React.string(string_of_int(pct) ++ "%")}
                 </span>
                 <span className={styles##timestamp}>
                   {React.string(formatTimestamp(entry.createdAt))}
                 </span>
               </button>
               {expanded
                  ? <div className={styles##expanded}>
                      <ResultView response={entry.result} corpusFingerprint />
                    </div>
                  : React.null}
             </div>;
           })
        |> React.array;
      let hasPrev = offset > 0;
      let hasNext = offset + pageSize < hl.total;
      <>
        entries
        <div className={styles##pager}>
          <button
            disabled={!hasPrev}
            onClick={_ => setOffset(o => max(0, o - pageSize))}>
            {React.string("← Newer")}
          </button>
          <button
            disabled={!hasNext} onClick={_ => setOffset(o => o + pageSize)}>
            {React.string("Older →")}
          </button>
        </div>
      </>;
    };

  <div className={styles##container}>
    <h1 className={styles##title}> {React.string("History")} </h1>
    body
  </div>;
};
