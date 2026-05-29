[@mel.module "./ResultView.module.css"]
external styles: Js.t({..}) = "default";

let scoreClass = (s: float) =>
  if (s >= 0.75) {
    styles##high;
  } else if (s >= 0.5) {
    styles##mid;
  } else {
    styles##low;
  };

let scoreLabel = (s: float) =>
  if (s >= 0.85) {
    "Very close match";
  } else if (s >= 0.7) {
    "Close match";
  } else if (s >= 0.5) {
    "Loose match";
  } else {
    "Off-style";
  };

let fmt = (f: float) => Js.Float.toFixed(f, ~digits=2);

[@react.component]
let make =
    (
      ~response: Api.generateResponse,
      ~corpusFingerprint: option(Api.styleFingerprint),
      ~onRegenerate: option(unit => unit)=?,
      ~regenerating: bool=false,
      ~onTweak: option(unit => unit)=?,
    ) => {
  let (showDetails, setShowDetails) = React.useState(() => false);

  /* Prefer a freshly computed local score against the *current* corpus
     fingerprint; fall back to the server-side confidence if the corpus
     fingerprint isn't available (e.g. expanded history entry). */
  let confidence =
    switch (corpusFingerprint) {
    | Some(fp) =>
      Confidence.score(~fingerprint=fp, ~text=response.generatedText)
    | None => response.confidence
    };

  let percent = int_of_float(confidence.overall *. 100.0);

  <div className={styles##layout}>
    <div className={styles##main}>
      <p className={styles##text}> {React.string(response.generatedText)} </p>
      <div className={styles##scoreBox}>
        <div
          className={
            styles##scoreNum ++ " " ++ scoreClass(confidence.overall)
          }>
          {React.string(string_of_int(percent) ++ "%")}
        </div>
        <div>
          <div className={styles##scoreLabel}>
            {React.string(scoreLabel(confidence.overall))}
          </div>
          <button
            className={styles##detailsToggle}
            onClick={_ => setShowDetails(s => !s)}>
            {React.string(
               showDetails ? "Hide breakdown" : "Show per-dimension breakdown",
             )}
          </button>
        </div>
      </div>
      {showDetails
         ? <div className={styles##details}>
             <table className={styles##dimTable}>
               <thead>
                 <tr>
                   <th> {React.string("Dimension")} </th>
                   <th className={styles##num}> {React.string("Corpus")} </th>
                   <th className={styles##num}> {React.string("Output")} </th>
                   <th className={styles##num}> {React.string("Score")} </th>
                 </tr>
               </thead>
               <tbody>
                 {confidence.dimensions
                  |> Array.map((d: Api.dimensionScore) =>
                       <tr key={d.dimension}>
                         <td> {React.string(d.dimension)} </td>
                         <td className={styles##num}>
                           {React.string(fmt(d.corpusValue))}
                         </td>
                         <td className={styles##num}>
                           {React.string(fmt(d.generatedValue))}
                         </td>
                         <td
                           className={
                             styles##num ++ " " ++ scoreClass(d.score)
                           }>
                           {React.string(fmt(d.score))}
                         </td>
                       </tr>
                     )
                  |> React.array}
               </tbody>
             </table>
           </div>
         : React.null}
      <div className={styles##actions}>
        {switch (onRegenerate) {
         | Some(f) =>
           <button
             className={styles##btn ++ " " ++ styles##primary}
             disabled=regenerating
             onClick={_ => f()}>
             {React.string(regenerating ? "Regenerating..." : "Regenerate")}
           </button>
         | None => React.null
         }}
        {switch (onTweak) {
         | Some(f) =>
           <button className={styles##btn} onClick={_ => f()}>
             {React.string("Tweak & regenerate")}
           </button>
         | None => React.null
         }}
      </div>
    </div>
    <aside className={styles##aside}>
      <h3> {React.string("Retrieved examples")} </h3>
      {Array.length(response.retrievedExamples) == 0
         ? <div> {React.string("(none)")} </div>
         : response.retrievedExamples
           |> Array.mapi((i, ex: Api.retrievedMessage) =>
                <div className={styles##example} key={string_of_int(i)}>
                  <p className={styles##exampleText}>
                    {React.string(ex.text)}
                  </p>
                  <div className={styles##exampleMeta}>
                    <AudienceBadge audience={ex.audience} />
                    <span>
                      {React.string(
                         "sim "
                         ++ Js.Float.toFixed(ex.similarityScore, ~digits=2),
                       )}
                    </span>
                  </div>
                </div>
              )
           |> React.array}
    </aside>
  </div>;
};
