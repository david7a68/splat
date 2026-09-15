# D&D Beyond API references

Research date: 2026-09-15. This report separates source-code evidence from endpoint access that still needs a live test. This document contains no credentials or real character responses. Source inspection was supplemented by a live SCDS check reported by the implementing agent.

## Main findings

1. D&D Beyond has a **simplified character data service (SCDS)** that returns calculated sheet values. Avrae calls it at `POST https://character-service-scds.dndbeyond.com/v2/characters`. This is the best new lead for data parity. A live read with the application's existing ordinary browser bearer succeeded for all three configured campaign characters. [A1–A4]
2. Character update events exist: Avrae handles **`character-sheet/character-update/fulfilled`** and then fetches current character data. This corrects the earlier assumption that only dice events were evidenced. Avrae receives these events through internal Redis pubsub; that does **not** prove the same events are available through a normal browser WebSocket. [A5–A6]
3. Avrae's full character import uses a separate computation endpoint whose host is supplied through deployment configuration. Its public source does not disclose that host. Do not confuse its computed response with the raw v5 character response. [A1, A7]
4. DDB Importer has extensive, readable normalization logic. Beyond20 mainly reads the rendered sheet and observes DOM changes, so it is more useful as a parity reference than as evidence of another character API. [D1–D7, B1–B2]

## Endpoint inventory

| Service | Verified source contract | Limits and next check |
| --- | --- | --- |
| Raw character | `GET https://character-service.dndbeyond.com/character/v5/character/{id}` | Already used by this application. DDB Importer also constructs this URL. Raw values still need rules evaluation. [D1] |
| Simplified computed character | `POST https://character-service-scds.dndbeyond.com/v2/characters`, JSON `{"characterIds":[123]}`, `Authorization: Bearer …` | Read operation despite POST. Response contains `foundCharacters`, `queuedIds`, `notFoundIds`. Avrae obtains its user with `auth_v1=True`. The application's existing ordinary browser bearer also worked in the live test. [A1–A5] |
| Avrae full computation | `GET ${CHARACTER_COMPUTATION_ENDPOINT}${characterId}`, optional bearer header, no body | Environment variable has no public default. Public characters can be requested without a linked user in Avrae's code. Errors include 403, 404, 429. No endpoint host was verified. [A1, A7] |
| Character writes | `PUT` routes under `https://character-service.dndbeyond.com/character/v5` | Avrae lists `/action/limited-use`, `/life/hp/damage-taken`, `/life/death-saves`, `/spell/pact-magic`, `/spell/slots`, `/inventory/currency`. Reference only; this stage must remain read-only toward Beyond. [A8] |
| DDB Importer proxy | `POST ${parsingApi}/proxy/v5/character` | This is the integration's parsing service, not another first-party raw character endpoint. Its enriched output must not be assumed to exist in raw v5. [D1] |
| First-party game data | Official sheet bundle defines GET routes `game-data/classes`, `subclasses`, `races`, `feats`, `backgrounds`, `spells`, `always-known-spells`, `always-prepared-spells`, `items`, `monsters`, `racial-trait/collection`, `class-feature/collection` | Its `HACK__CHARACTER_SERVICE_GAME_DATA` route builder uses the character-service host and version helper. Items explicitly use `v5.1`. Required query parameters come from current rules-engine state; do not assume bare route access or reuse one version for all routes. [O1] |

## Direct live check

The implementing agent successfully called SCDS with the application's existing ordinary browser bearer for all three configured campaign characters. The response returned `foundCharacters` with computed core totals, skills (including proficiency and advantage flags), attacks, and HP. Basic party stats matched the existing observations. This establishes access for the current session and campaign; it is not a complete field-by-field parity test or a guarantee of future API stability. No character details are reproduced here.

The chosen implementation combines SCDS calculated values with raw v5 source details, persists the normalized creature/entity, and uses the campaign event stream while a sheet is open, with polling fallback. The normal-session WebSocket handshake has now succeeded; delivery of all real edit categories remains to be verified.

## SCDS wire shape

Avrae's models use a snake-case to lower-camel-case alias generator. The following names are therefore wire names, not Python attribute names. The response is parsed directly, with no `data` wrapper. [A2–A4]

```text
response
  foundCharacters: SimplifiedCharacterData[]
  queuedIds: integer[]
  notFoundIds: integer[]

SimplifiedCharacterData
  characterId, userId, name, level, readOnlyUrl
  abilities[]: { name, label, score, modifier, save }
  armorClass, proficiencyBonus, initiativeBonus
  skills[]: { name, modifier, isProficient, isHalfProficient,
              isExpert, isAdvantage, isDisadvantage }
  passivePerception, passiveInsight, passiveInvestigation
  hitPointInfo: { current, maximum, temp }
  deathSaveInfo: { failCount, successCount }
  attacks[]: { name, range, damage, saveInfo?, toHit? }
    range: { label, longRangeValue?, value?, origin? }
    damage: { type?, value?: dice-object | integer }
    dice-object: { diceCount, diceValue, fixedValue,
                   diceString?, diceMultiplier? }
    saveInfo: { value, label }
  castingInfo: { modifiers[], saveDcs[], spellAttacks[] }
    each casting value: { value, sources: string[] }
  conditions[]: { name, level? }
  speeds[]: { name, distance }
  senses[]: { name, distance: string }
  resistances[], immunities[], vulnerabilities[]: { name, type }
  attunedItems[]: { name, definitionKey, type, avatarUrl }
  proficiencyGroups[]: { group, values: string }
  classes[]: { name, level, subclassName? }
  race: { name }
  campaign?: { name, id, url, dmUserId }
  currentXp?, inspiration
```

The declared SCDS model does not include full inventory, spell lists, slot usage, or feature resources. Keep raw v5 for these. The existence of `queuedIds` means a successful HTTP response is not proof that every requested character is ready. A queued/missing result must not erase an existing local entity. No documented processing delay, ETag contract, rate limit, or polling interval was established from these sources. [A2–A3]

Avrae's full computation response is richer: its parser reads calculated `stats`, `skills`, `initiative`, `ac`, `hp`, `spellbook` (per-spell attack/DC/modifier, prepared state, ordinary and pact slots), `consumables`, and attacks/actions. This is useful model guidance even though its endpoint is not publicly configured. [A7]

## Live updates

Avrae's handler accepts a small event body with `characterId`, calls SCDS, and updates HP and death saves. It is feature-flagged and respects an inbound-sync setting. Its public code does not establish exhaustive coverage of equipment, level, spell preparation, custom adjustments, or every other edit. [A3, A5]

The general event envelope includes `id`, `gameId`, `userId`, `eventType`, `source`, `data`, `entityId`, `entityType`, `messageScope`, `messageTarget`, `dateTime`, `persist`, and optionally `connectionId`. Avrae ignores its own events and unknown event types. It consumes a Redis `game-log` channel and retries connection after five seconds. This is an internal transport implementation, not a browser WebSocket recipe. [A6, A9]

Recommended design:

- Use events to invalidate the cached character and start a fresh read, not as a complete ordered change log.
- Subscribe while a sheet is open. Share one campaign connection across open sheets, if the transport permits it. Coalesce repeated invalidations and permit only one fetch per character at a time.
- Refresh on open and reconnect. Use a slower polling fallback while the sheet is visible until event coverage is verified. Stop polling when it closes; handle expired login, network failures, and rate limits with backoff.
- Keep the last successful cache on refresh failure, with a visible saved/updated time and stale state. Guard against older requests replacing newer data.
- Keep imported source state separate from future encounter state. A source HP update must not overwrite an encounter's independent simulation or working state without an explicit future policy. Avrae already contains special HP logic to avoid overwriting certain local combat changes. [A5]

These are application recommendations, not promises from D&D Beyond about event delivery or allowed request rates.

## Normalization references

| Area | What the integration source establishes |
| --- | --- |
| Selected modifiers | Match both component ID and component type; choice option IDs can collide with unrelated feature IDs. Filter replaced class features and resolve selected options. `isGranted` or a flat sum of every modifier is insufficient. [D2] |
| Item effects | Check equipped/attuned state, whether an item can equip/attune, and consumable status. DDB Importer separates item effects from other modifiers to avoid double application in Foundry. That separation is implementation-specific. [D2] |
| Skills | Supports expertise, proficiency, and half proficiency. `characterValues` type 26 changes proficiency, 27 changes ability, 24/25 contribute custom bonuses. Explicit zero must not be lost through truthiness checks. [D3] |
| AC | Has custom AC override type 1, armor alternatives, shields, magic bonuses, and dual-wielding checks. Treat the module as a list of cases to verify against Beyond, not proof every formula is correct for this application. [D4] |
| Spellcasting | Tracks class/subclass casting abilities, multiclass slot divisors, pact casting and per-spell data. Its Foundry summary selects one primary casting ability; our viewer should preserve each source instead of collapsing them. [D5, A7] |
| Resources | Uses `limitedUse.maxUses`, `statModifierUsesId`, `operator`, `useProficiencyBonus`, `proficiencyBonusOperator`, `numberUsed`, and `resetType`. Max uses can be ability/proficiency-based; remaining uses is derived from maximum minus used. [D6] |
| Other sheet values | Separate character modules cover abilities, HP, initiative, senses, speed, proficiencies, inventory, item spells, effects, and conditions. Typed raw-source declarations offer a useful field inventory. [D7] |
| Rendered parity | Beyond20 reads visible sheet values, caches some stable values and attack bonuses, and observes DOM mutations. Use the real rendered sheet to check parity; Beyond20 does not establish an alternative first-party calculated API. [B1–B2] |

## Browser event transport: follow-up inspection

The official message-broker bundle establishes the browser connection protocol. It reads `connecturl`, `getmessagesurl`, `gameid`, `userid`, `source`, `config`, and `environment` from the `#message-broker-client` element. The previously inspected campaign page supplies `wss://game-log-api-live.dndbeyond.com/v1` and `https://game-log-rest-live.dndbeyond.com/v1/getmessages`. [O2; local `docs/character-viewer.md` page observation]

```text
WebSocket connect:
  wss://game-log-api-live.dndbeyond.com/v1
    ?gameId=<campaign-id>&userId=<user-id>&stt=<short-term-token>

Token retrieval:
  POST https://auth-service.dndbeyond.com/v1/cobalt-token
  credentials: include
  response: { token, ttl }

Keepalive text frame, every 300000 ms:
  {"data":"ping"}

Keepalive response:
  pong
```

`stt` is the ordinary short-term token, encoded with `URLSearchParams`. The token helper refreshes 30 seconds before its reported TTL. Do not log the connection URL: its query contains authentication. There is no additional subscription frame in this client: `subscribe(callback)` registers a local handler and calls `connect()`. On socket open, the client flushes queued outbound messages and starts the keepalive timer. A read-only client needs no outbound application events. [O2]

The broker ignores literal `pong`; all other frames are parsed as JSON and passed to subscribers, with message-ID deduplication. It does not filter by event type. Its reconnect delay is `min(100 * 2^attempt, 30000) + random(0, 250)` milliseconds, with up to nine attempts before relying on focus, visibility, or online events to retry. It resets on offline and unload. [O2]

The REST history request uses query parameters `gameId`, `userId`, and optionally `lastEvaluatedKey`, with a bearer header and `X-Source-Url`. The response shape is `{data: Message[], lastKey}`. This is history for game-log messages; it does not establish that transient character updates can be replayed. A fresh sheet read after reconnect is still required. [O2]

The official character application's `MessageManager` subscribes to the general broker, then calls a handler registered for `eventType`. It uses `data.messageManagerGuid` and `data.shouldUpdate` to avoid redundant refreshes from its own messages. In this inspected bundle, the only declared message-manager event is `character-sheet/item-shared/fulfilled`; `PartyManager` handles it by requesting party inventory. The literal `character-sheet/character-update/fulfilled` is absent from this main bundle, the broker bundle, and the game-log UI bundle. Thus the browser transport is capable of passing the event, but the server's delivery of this Avrae-observed event to ordinary user sockets is still a separate live-test question. [O1–O3, A5]

Beyond20 confirms the same in-page broker interface: `window[Symbol.for("@dndbeyond/message-broker-lib")].subscribe(callback)`. Its bridge forwards dice events, which should not be mistaken for the full set the broker can carry. [B3]

Implementation implication: prefer the stream for immediate invalidation if live access succeeds. Retain a slower reconciliation read until character-event coverage is proved. Treat `character-sheet/character-update/fulfilled` as a refresh hint for `data.characterId` (or a validated character `entityId`). Treat item-shared events as potentially affecting the party inventory and related open sheets. Dice events alone are not proof a sheet changed.

## Local cache and existing entity schema

Recommended implementation boundary:

1. A D&D Beyond adapter owns raw v5/SCDS input, external IDs, source links, read time, schema version, and normalization coverage.
2. Persist a normalized creature/entity using the existing schema. Add typed fields for character features that the current schema lacks. Keep raw source data in a separate versioned import record only where needed for repeatable normalization and future unsupported fields. Do not store authentication in either record.
3. Preserve stable `(provider, characterId)` identity and source feature/item identifiers; names alone are unsafe keys. Preserve per-source spellcasting, resource reset semantics, and conditional text that cannot yet be executed.
4. Prefer verified computed service values for display when available. Do not add local modifiers to an already calculated total. Store the value's origin and whether it was fetched, locally derived, or unsupported.
5. Verify the configured party against the rendered sheet, including current and maximum HP, AC, abilities/saves/skills, passive scores, speeds, attacks, spellcasting, inventory, and resources. Report field differences and unsupported cases separately.

If SCDS access fails, the official sheet bundle has a rules engine and selectors such as `getSkills`, `getSpeeds`, `getSpellSlots`, `getSpellCasterInfo`, `getResistanceData`, `getSenseInfo`, and `getStaticInitiative`. This is a useful diagnostic reference, but coupling the application to a minified, hash-versioned bundle needs an explicit update strategy. [O1]

## Primary sources

All GitHub links below are pinned to the commits inspected. These are source references, not claims of stable or officially supported external API contracts.

- [A1: Avrae service configuration](https://github.com/avrae/avrae/blob/12b146fb3773b1c0de2e3650138d4af4a13c00f5/utils/config.py#L55-L76)
- [A2: Avrae SCDS client](https://github.com/avrae/avrae/blob/12b146fb3773b1c0de2e3650138d4af4a13c00f5/ddb/character/scds.py)
- [A3: Avrae SCDS response and event types](https://github.com/avrae/avrae/blob/12b146fb3773b1c0de2e3650138d4af4a13c00f5/ddb/character/scds_types.py)
- [A4: Avrae bearer-authenticated base client](https://github.com/avrae/avrae/blob/12b146fb3773b1c0de2e3650138d4af4a13c00f5/ddb/baseclient.py) and [wire-name aliases](https://github.com/avrae/avrae/blob/12b146fb3773b1c0de2e3650138d4af4a13c00f5/ddb/utils.py#L57-L71)
- [A5: Avrae character-update event handler](https://github.com/avrae/avrae/blob/12b146fb3773b1c0de2e3650138d4af4a13c00f5/cogs5e/gamelog/character.py)
- [A6: Avrae game-log client and Redis event loop](https://github.com/avrae/avrae/blob/12b146fb3773b1c0de2e3650138d4af4a13c00f5/ddb/gamelog/client.py)
- [A7: Avrae computed character importer](https://github.com/avrae/avrae/blob/12b146fb3773b1c0de2e3650138d4af4a13c00f5/cogs5e/sheets/beyond.py)
- [A8: Avrae character mutation routes](https://github.com/avrae/avrae/blob/12b146fb3773b1c0de2e3650138d4af4a13c00f5/ddb/character/client.py)
- [A9: Avrae game-log event envelope](https://github.com/avrae/avrae/blob/12b146fb3773b1c0de2e3650138d4af4a13c00f5/ddb/gamelog/event.py)
- [D1: DDB Importer character fetch and proxy](https://github.com/MrPrimate/ddb-importer/blob/9bb4cd53036055875a694192384c771240b292c8/src/parser/DDBCharacter.ts#L397-L470)
- [D2: DDB Importer modifier selection](https://github.com/MrPrimate/ddb-importer/blob/9bb4cd53036055875a694192384c771240b292c8/src/parser/lib/DDBModifiers.ts)
- [D3: DDB Importer skill calculation](https://github.com/MrPrimate/ddb-importer/blob/9bb4cd53036055875a694192384c771240b292c8/src/parser/character/skills.ts)
- [D4: DDB Importer AC calculation](https://github.com/MrPrimate/ddb-importer/blob/9bb4cd53036055875a694192384c771240b292c8/src/parser/character/ac.ts)
- [D5: DDB Importer spellcasting](https://github.com/MrPrimate/ddb-importer/blob/9bb4cd53036055875a694192384c771240b292c8/src/parser/character/spellCasting.ts)
- [D6: DDB Importer resources](https://github.com/MrPrimate/ddb-importer/blob/9bb4cd53036055875a694192384c771240b292c8/src/parser/character/resources.ts)
- [D7: DDB Importer character modules](https://github.com/MrPrimate/ddb-importer/tree/9bb4cd53036055875a694192384c771240b292c8/src/parser/character) and [source types](https://github.com/MrPrimate/ddb-importer/blob/9bb4cd53036055875a694192384c771240b292c8/src/types/ddb-character-source.d.ts)
- [B1: Beyond20 rendered character reader](https://github.com/kakaroto/Beyond20/blob/15a7f915a881a87f5f87152ec797bb06d9e123e0/src/dndbeyond/base/character.js)
- [B2: Beyond20 DOM observer](https://github.com/kakaroto/Beyond20/blob/15a7f915a881a87f5f87152ec797bb06d9e123e0/src/dndbeyond/content-scripts/character.js#L3654-L3668)
- [O1: Official D&D Beyond character application bundle](https://media.dndbeyond.com/character-app/static/js/main.e74b450e.js), inspected from `/tmp/splat-ddb-main.js`; search for `HACK__CHARACTER_SERVICE_GAME_DATA`, `game-data/always-prepared-spells`, `getRequiredGameDataServiceParams`, and `getSpellCasterInfo`.

- [O2: Official message-broker client bundle](https://media.dndbeyond.com/message-broker-client/_dndbeyond_message_broker_client.a0a06b68cf2ec42dfbc5.bundle.js), search `new WebSocket`, `stt`, `const F=3e5`, `subscribe(t)`, and `getMessages(t)`.
- [O3: Official game-log UI bundle](https://media.dndbeyond.com/game-log-client/_dndbeyond_game_log_client.cfdd861d040c0e140af1.bundle.js).
- [B3: Beyond20 broker adapter](https://github.com/kakaroto/Beyond20/blob/15a7f915a881a87f5f87152ec797bb06d9e123e0/src/dndbeyond/base/message-broker.js).

## Live event verification

During implementation, the user changed temporary HP and restored it on a party
character. The normal-session campaign WebSocket received both events as
`character-sheet/character-update/fulfilled`, with the expected campaign and a
`data.characterId` field. This verifies browser-stream delivery for that edit
category and corrects the earlier transport uncertainty. No remote edit was made
by the application or agent. The stream is now the primary invalidation path;
five-minute reconciliation and disconnected polling cover missed or unsupported
events. Other edit categories remain to be tested individually.
