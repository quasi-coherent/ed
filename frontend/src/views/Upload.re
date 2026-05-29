[@mel.module "./Upload.module.css"] external styles: Js.t({..}) = "default";

type submitState =
  | Idle
  | Submitting
  | Failed(string)
  | Warned(int);

[@react.component]
let make = () => {
  let corpus = CorpusContext.useCorpus();
  let (text, setText) = React.useState(() => "");
  let (hint, setHint) = React.useState(() => "unknown");
  let (state, setState) = React.useState(() => Idle);

  let onFile = e => {
    let target = React.Event.Form.target(e);
    let files: Js.t({..}) = target##files;
    let len: int = files##length;
    if (len > 0) {
      let file: Js.t({..}) = files##item(0);
      let p: Js.Promise.t(string) = file##text();
      let _ =
        p
        |> Js.Promise.then_(content => {
             setText(_ => content);
             Js.Promise.resolve();
           });
      ();
    };
  };

  let onSubmit = _ =>
    if (String.length(String.trim(text)) == 0) {
      setState(_ => Failed("Paste some text or upload a file first."));
    } else {
      setState(_ => Submitting);
      let upload: Api.corpusUpload = {
        text,
        audienceHint: Api.audienceOfString(hint),
      };
      let _ =
        Client.postData(upload)
        |> Js.Promise.then_(res => {
             switch (res) {
             | Ok(ingest) =>
               let ingest: Api.ingestResult = ingest;
               corpus.setFingerprint(Some(ingest.fingerprint));
               if (ingest.messageCount < 20) {
                 setState(_ => Warned(ingest.messageCount));
                 /* Still navigate so the user can see the profile, but
                    only after they dismiss / acknowledge the warning. */
                 let _ =
                   Js.Global.setTimeout(~f=() => Router.push(Profile), 2500);
                 ();
               } else {
                 setState(_ => Idle);
                 Router.push(Profile);
               };
             | Error(e) => setState(_ => Failed(Client.errorToString(e)))
             };
             Js.Promise.resolve();
           });
      ();
    };

  let disabled =
    switch (state) {
    | Submitting => true
    | _ => false
    };

  <div className={styles##container}>
    <h1 className={styles##title}> {React.string("Upload corpus")} </h1>
    <p className={styles##subtitle}>
      {React.string(
         "Paste raw writing or upload a file. The more text, the more reliable the style profile.",
       )}
    </p>
    <div className={styles##field}>
      <label className={styles##label}> {React.string("Text")} </label>
      <textarea
        className={styles##textarea}
        value=text
        placeholder="Paste emails, Slack messages, texts..."
        onChange={e => {
          let v = React.Event.Form.target(e)##value;
          setText(_ => v);
        }}
      />
    </div>
    <div className={styles##row}>
      <div className={styles##field}>
        <label className={styles##label}>
          {React.string("Or upload a file")}
        </label>
        <input
          className={styles##file}
          type_="file"
          accept=".txt,.md,.csv"
          onChange=onFile
        />
      </div>
      <div className={styles##field}>
        <label className={styles##label}>
          {React.string("Audience hint (optional)")}
        </label>
        <select
          className={styles##select}
          value=hint
          onChange={e => {
            let v = React.Event.Form.target(e)##value;
            setHint(_ => v);
          }}>
          <option value="unknown"> {React.string("Mixed / unknown")} </option>
          <option value="email"> {React.string("Email")} </option>
          <option value="slack"> {React.string("Slack")} </option>
          <option value="sms"> {React.string("SMS")} </option>
        </select>
      </div>
    </div>
    <button className={styles##button} disabled onClick=onSubmit>
      {React.string(
         switch (state) {
         | Submitting => "Ingesting..."
         | _ => "Ingest corpus"
         },
       )}
    </button>
    {switch (state) {
     | Failed(msg) =>
       <div className={styles##error}> {React.string(msg)} </div>
     | Warned(n) =>
       <div className={styles##warn}>
         {React.string(
            "Only "
            ++ string_of_int(n)
            ++ " messages parsed — style analysis may be unreliable with fewer than 20. Loading profile...",
          )}
       </div>
     | _ => React.null
     }}
  </div>;
};
