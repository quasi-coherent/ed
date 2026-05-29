[@mel.module "./Messages.module.css"] external styles: Js.t({..}) = "default";

type loadState =
  | Loading
  | Loaded(Api.messageList)
  | Failed(string);

let pageSize = 25;

[@react.component]
let make = () => {
  let (state, setState) = React.useState(() => Loading);
  let (offset, setOffset) = React.useState(() => 0);
  let (filter, setFilter) = React.useState(() => "all");

  React.useEffect2(
    () => {
      setState(_ => Loading);
      let audience =
        switch (filter) {
        | "all" => None
        | s => Api.audienceOfString(s)
        };
      let _ =
        Client.getMessages(~limit=pageSize, ~offset, ~audience?, ())
        |> Js.Promise.then_(res => {
             switch (res) {
             | Ok(ml) => setState(_ => Loaded(ml))
             | Error(e) => setState(_ => Failed(Client.errorToString(e)))
             };
             Js.Promise.resolve();
           });
      None;
    },
    (offset, filter),
  );

  let body =
    switch (state) {
    | Loading =>
      <div className={styles##loading}> {React.string("Loading...")} </div>
    | Failed(msg) =>
      <div className={styles##error}> {React.string(msg)} </div>
    | Loaded(ml) when Array.length(ml.messages) == 0 =>
      <div className={styles##empty}>
        {React.string("No messages match this filter.")}
      </div>
    | Loaded(ml) =>
      let items =
        ml.messages
        |> Array.map((m: Api.storedMessage) =>
             <div className={styles##item} key={m.messageId}>
               <div className={styles##text}> {React.string(m.text)} </div>
               <AudienceBadge audience={m.audience} />
             </div>
           )
        |> React.array;
      let hasPrev = offset > 0;
      let hasNext = offset + pageSize < ml.total;
      <>
        <div className={styles##controls}>
          <select
            className={styles##select}
            value=filter
            onChange={e => {
              let v = React.Event.Form.target(e)##value;
              setOffset(_ => 0);
              setFilter(_ => v);
            }}>
            <option value="all"> {React.string("All audiences")} </option>
            <option value="email"> {React.string("Email")} </option>
            <option value="slack"> {React.string("Slack")} </option>
            <option value="sms"> {React.string("SMS")} </option>
            <option value="unknown"> {React.string("Unknown")} </option>
          </select>
          <span className={styles##count}>
            {React.string(
               "Showing "
               ++ string_of_int(offset + 1)
               ++ "–"
               ++ string_of_int(min(offset + pageSize, ml.total))
               ++ " of "
               ++ string_of_int(ml.total),
             )}
          </span>
        </div>
        items
        <div className={styles##pager}>
          <button
            disabled={!hasPrev}
            onClick={_ => setOffset(o => max(0, o - pageSize))}>
            {React.string("← Prev")}
          </button>
          <button
            disabled={!hasNext} onClick={_ => setOffset(o => o + pageSize)}>
            {React.string("Next →")}
          </button>
        </div>
      </>;
    };

  <div className={styles##container}>
    <h1 className={styles##title}> {React.string("Corpus messages")} </h1>
    body
  </div>;
};
