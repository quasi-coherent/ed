/* Thin fetch wrapper + typed endpoints. */

type response;

[@mel.send]
external responseJson: response => Js.Promise.t(Js.Json.t) = "json";
[@mel.send] external responseText: response => Js.Promise.t(string) = "text";
[@mel.get] external responseOk: response => bool = "ok";
[@mel.get] external responseStatus: response => int = "status";

external fetch: string => Js.Promise.t(response) = "fetch";
external fetchWith: (string, Js.t({..})) => Js.Promise.t(response) = "fetch";

/* Build a request init object. */
[@mel.obj]
external makeInit:
  (~method: string, ~headers: Js.Dict.t(string)=?, ~body: string=?, unit) => _;

/* API base URL from Vite's build-time env. Falls back to same-origin. */
let baseUrl: string = [%mel.raw {| import.meta.env.VITE_API_BASE_URL || "" |}];

/* Current user_id threaded into spec-scoped paths. Defaults to the nil UUID
   as a stub; will be overwritten by `setUserId` once OAuth lands and the
   session/me endpoint resolves the real user. */
let currentUserId: ref(string) = ref("00000000-0000-0000-0000-000000000000");

let setUserId = (uid: string): unit => currentUserId := uid;

let getUserId = (): string => currentUserId^;

let url = (path: string): string => baseUrl ++ path;

let dataPath = (): string => "/data/" ++ currentUserId^;
let simulatePath = (): string => "/simulate/" ++ currentUserId^;

/* Common error type for all API calls. */
type apiError =
  | NetworkError(string)
  | HttpError(int, string)
  | DecodeError(string);

let errorToString = (e: apiError): string =>
  switch (e) {
  | NetworkError(s) => "Network error: " ++ s
  | HttpError(code, body) => "HTTP " ++ string_of_int(code) ++ ": " ++ body
  | DecodeError(s) => "Decode error: " ++ s
  };

/* Try to pull an {"error":"..."} message out of a response body, falling
   back to the raw body. */
let extractError = (body: string): string =>
  switch (Js.Json.parseExn(body)) {
  | json =>
    switch (Api.decodeErrorBody(json)) {
    | s => s
    | exception _ => body
    }
  | exception _ => body
  };

let handleResponse =
    (decode: Js.Json.t => 'a, resp: response): Js.Promise.t('a) =>
  if (responseOk(resp)) {
    Js.Promise.then_(
      json =>
        switch (decode(json)) {
        | v => Js.Promise.resolve(v)
        | exception (Api.Decode_error(msg)) =>
          Js.Promise.reject(Failure("decode: " ++ msg))
        },
      responseJson(resp),
    );
  } else {
    Js.Promise.then_(
      text => {
        let msg = extractError(text);
        Js.Promise.reject(
          Failure(
            "http:" ++ string_of_int(responseStatus(resp)) ++ ":" ++ msg,
          ),
        );
      },
      responseText(resp),
    );
  };

/* Translate the internal Failure exceptions back into apiError values for
   the caller's `result`. */
let catchToResult =
    (p: Js.Promise.t('a)): Js.Promise.t(result('a, apiError)) =>
  p
  |> Js.Promise.then_(v => Js.Promise.resolve(Ok(v)))
  |> Js.Promise.catch(err => {
       let str = {
         let any: Js.t({..}) = Obj.magic(err);
         switch (Js.Undefined.toOption(any##message)) {
         | Some(m) => m
         | None => "unknown error"
         };
       };
       let parsed: apiError =
         if (String.length(str) > 5 && String.sub(str, 0, 5) == "http:") {
           let rest = String.sub(str, 5, String.length(str) - 5);
           switch (String.index_opt(rest, ':')) {
           | Some(i) =>
             let code = int_of_string(String.sub(rest, 0, i));
             let body = String.sub(rest, i + 1, String.length(rest) - i - 1);
             HttpError(code, body);
           | None => NetworkError(str)
           };
         } else if (String.length(str) > 7
                    && String.sub(str, 0, 7) == "decode:") {
           DecodeError(String.sub(str, 7, String.length(str) - 7));
         } else {
           NetworkError(str);
         };
       Js.Promise.resolve(Error(parsed));
     });

let jsonHeaders: Js.Dict.t(string) =
  Js.Dict.fromArray([|("Content-Type", "application/json")|]);

/* ----- Endpoints ----- */

let postData =
    (upload: Api.corpusUpload)
    : Js.Promise.t(result(Api.ingestResult, apiError)) =>
  fetchWith(
    url(dataPath()),
    makeInit(
      ~method="POST",
      ~headers=jsonHeaders,
      ~body=Js.Json.stringify(Api.encodeCorpusUpload(upload)),
      (),
    ),
  )
  |> Js.Promise.then_(handleResponse(Api.decodeIngestResult))
  |> catchToResult;

let getFingerprint =
    (): Js.Promise.t(result(option(Api.styleFingerprint), apiError)) =>
  fetch(url(dataPath() ++ "/fingerprint"))
  |> Js.Promise.then_(resp =>
       if (responseStatus(resp) == 404) {
         Js.Promise.resolve(None);
       } else {
         handleResponse(Api.decodeStyleFingerprint, resp)
         |> Js.Promise.then_(fp => Js.Promise.resolve(Some(fp)));
       }
     )
  |> catchToResult;

let getMessages =
    (~limit: int=50, ~offset: int=0, ~audience: option(Api.audience)=?, ())
    : Js.Promise.t(result(Api.messageList, apiError)) => {
  let qs =
    "?limit="
    ++ string_of_int(limit)
    ++ "&offset="
    ++ string_of_int(offset)
    ++ (
      switch (audience) {
      | Some(a) => "&audience=" ++ Api.audienceToString(a)
      | None => ""
      }
    );
  fetch(url(dataPath() ++ qs))
  |> Js.Promise.then_(handleResponse(Api.decodeMessageList))
  |> catchToResult;
};

let postGenerate =
    (req: Api.generateRequest)
    : Js.Promise.t(result(Api.generateResponse, apiError)) =>
  fetchWith(
    url(simulatePath()),
    makeInit(
      ~method="POST",
      ~headers=jsonHeaders,
      ~body=Js.Json.stringify(Api.encodeGenerateRequest(req)),
      (),
    ),
  )
  |> Js.Promise.then_(handleResponse(Api.decodeGenerateResponse))
  |> catchToResult;

let getHistory =
    (~limit: int=20, ~offset: int=0, ())
    : Js.Promise.t(result(Api.historyList, apiError)) => {
  let qs =
    "?limit=" ++ string_of_int(limit) ++ "&offset=" ++ string_of_int(offset);
  fetch(url(simulatePath() ++ qs))
  |> Js.Promise.then_(handleResponse(Api.decodeHistoryList))
  |> catchToResult;
};
