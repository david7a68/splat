# D&D Beyond combat writes

Research date: 2026-09-15. This is source inspection, not a live write test. No credentials were inspected and no character was changed. The routes are internal APIs; this report does not establish a supported third-party API guarantee.

## Verified request contracts

All routes below use `https://character-service.dndbeyond.com/character/v5` as the base, JSON request bodies, and bearer authentication. Every body includes integer `characterId`. The official sheet's `im` helper issues PUT; `am` issues DELETE with a JSON body. Both merge the required character-service parameters, which are `{characterId}`. [O1, A1, A2]

| Operation | Method and route | Additional JSON body fields | Meaning |
| --- | --- | --- | --- |
| HP and temporary HP | PUT `/life/hp/damage-taken` | `removedHitPoints`, `temporaryHitPoints` | Absolute damage taken and current temporary HP, not damage/healing deltas. Avrae clamps removed HP to `[0, maxHp]` and temp HP to at least zero. [O1, A1, A3] |
| Death saves | PUT `/life/death-saves` | `successCount`, `failCount` | Absolute counts, each 0–3. [O1, A1, A3] |
| Add/change condition | PUT `/condition` | `id`, `level`, `totalHp` | Official add/set actions supply condition ID and nullable level; their handlers add calculated total HP. Do not substitute a condition name for ID. [O1] |
| Remove condition | DELETE `/condition` | `id` | JSON body, not a query parameter. [O1] |
| Ordinary spell slots | PUT `/spell/slots` | `level1` through `level9` | Absolute slots **used**, separately from pact slots. Avrae sends all nine counts. [O1, A1, A3] |
| Pact magic slots | PUT `/spell/pact-magic` | `level1` through `level5` | Absolute pact slots **used**. Avrae fills the current pact level and sends zero for the other levels. [O1, A1, A3] |
| Action/spell limited uses | PUT `/action/limited-use` | `id`, `entityTypeId`, `uses` | Source record identity and absolute uses **spent**. Official action and spell limited-use mutations share this endpoint. [O1, A1, A3] |

For example, setting current HP to 18 from a maximum of 30, with 4 temporary HP, means:

```json
{"characterId":123,"removedHitPoints":12,"temporaryHitPoints":4}
```

This body describes final values. Retrying it is not equivalent to applying damage twice, but it can still overwrite another user's more recent value. [A3; implementation consequence]

For ordinary and pact slots, preserve every untouched level from a fresh raw character record. Do not combine the two pools even when they have the same spell level. Avrae explicitly subtracts used pact slots before computing ordinary slots. For limited uses, retain both source ID and entity type; a display name or normalized local ID is not an API identity. [A3]

The generic action/spell contract above does not establish the correct contract for inventory charges. The official bundle has separate inventory handlers. Do not route every resource to `/action/limited-use`. No current-concentration mutation contract was verified; the inspected concentration references describe spell properties/filtering. [O1]

## Authentication and edit permission

The official API adapter gets authorization headers from the short-term token helper. Avrae sends `Authorization: Bearer <token>`. Splat already provides this through `Session::authorize` and refreshes the session through the cobalt-token endpoint. The first-party browser adapter also sets `withCredentials: true`; Avrae's server-side client uses bearer authentication without browser cookies. [O1, A2; local `src/auth.rs`]

The raw character's `canEdit` flag is the first-party permission signal: the official `getCanEdit` selector returns it, and the builder sets `isReadonly: !canEdit`. The same component records DM status separately by comparing the authenticated user ID with `campaign.dmUserId`. Campaign membership, DM status, or successful reads alone do not prove that a mutation is authorized. The inspected client does not disclose the server's complete owner/DM permission policy. [O1]

Implementation recommendation: require `canEdit == true` on a fresh authenticated raw read before offering writes, and still treat HTTP 401/403 as failure. Do not infer permission from locally cached campaign metadata, and do not silently ignore denied mutations. This is a conservative application policy, not a claimed server requirement.

## Success and failure responses

Avrae's character client expects a JSON envelope with `success`, `id`, `message`, and `data`. It rejects `success: false`, including after a successful HTTP status. Its base client first rejects non-2xx statuses, classifies 403 as forbidden, and rejects unreadable/non-JSON responses. [A1, A2]

The first-party adapter checks for an object response and extracts the envelope's `data`. Its condition handlers explicitly manage HTTP 400: they fetch the character again, replace the local condition list, and display “This character’s conditions changed elsewhere. We’ve refreshed them here.” They do not blindly retry the failed write. This establishes condition conflict recovery behavior; it does not establish that every HTTP 400 on every endpoint means a conflict. [O1: `Vm`, `TC`, `IC`, `MC`, `wC`, `NC`]

No ETag, revision token, conditional-write header, atomic multi-field transaction, or stable rate-limit contract was verified. No complete error-body catalogue was found. For safety and clarity, classify status failures locally and avoid exposing arbitrary response bodies. A transport timeout after sending a write is an uncertain result: read back before deciding whether to retry. [O1, A1, A2; recommendations]

## Read-back and two-way sync

Raw character read: `GET /character/{characterId}` under the same v5 base. It contains `removedHitPoints`, `temporaryHitPoints`, `deathSaves`, `conditions`, `spellSlots`, `pactMagic`, and action/spell `limitedUse.numberUsed` records. The first-party raw reducers and mutation actions use these fields. [O1]

SCDS is useful for computed HP and displays, but its declared response omits slots and limited-use pools; it also has `queuedIds`. Thus SCDS alone cannot verify these writes. The prior report documents its contract and verified browser-stream refresh event. [A4, local `dndbeyond-api.md`]

Recommended sequence:

1. Fetch current raw state. Confirm identity and `canEdit`.
2. Compare the touched field group against the edit's baseline. Stop on a conflict, retaining the user's pending values.
3. Send only the required mutation group. HP/temp HP and each slot pool are grouped endpoints, so preserve the unedited members of each group.
4. Require both a successful HTTP status and `success: true`.
5. Read raw state again and compare the exact target fields. Mark the write confirmed only when they match. Refresh SCDS for computed display values afterward.
6. Treat character-update events as refresh hints; serialize writes per character and prevent old background reads from overwriting a newer pending edit.

These steps reduce lost edits but cannot eliminate the race between the preflight read and mutation without server-side conditional writes. A multi-operation save can partly succeed. Record each confirmed group and report remaining groups as pending or failed; do not claim transaction atomicity or automatically roll back remote state.

Keep encounter-only state separate: initiative, round/turn, notes, arbitrary conditions, and concentration have no verified sheet write contract here. Only recognized character fields should be synchronized. A replayed saved run must not automatically overwrite a live character merely because it was opened.

## Source locators

- [O1: Official character application bundle](https://media.dndbeyond.com/character-app/static/js/main.e74b450e.js), inspected as `/tmp/splat-ddb-main.js`. Useful exact searches: `function im(`, `function am(`, `function va(e){return{characterId:e}}`, `life/hp/damage-taken`, `life/death-saves`, `spell/slots`, `spell/pact-magic`, `action/limited-use`, `putCharacterCondition`, `function*MC(`, `function*wC(`, `function*NC(`, `function*IC(`, `function*bE(`, `getCanEdit:`, `isReadonly:!p`.
- [A1: Avrae character service client](https://github.com/avrae/avrae/blob/12b146fb3773b1c0de2e3650138d4af4a13c00f5/ddb/character/client.py).
- [A2: Avrae base HTTP client](https://github.com/avrae/avrae/blob/12b146fb3773b1c0de2e3650138d4af4a13c00f5/ddb/baseclient.py).
- [A3: Avrae outbound sheet sync](https://github.com/avrae/avrae/blob/12b146fb3773b1c0de2e3650138d4af4a13c00f5/cogs5e/models/ddbsync.py).
- [A4: Avrae SCDS types](https://github.com/avrae/avrae/blob/12b146fb3773b1c0de2e3650138d4af4a13c00f5/ddb/character/scds_types.py).

All Avrae references are pinned to the inspected commit. The first-party asset is hash-versioned. No live write or actual DM authorization test was performed for this report.

## Exact raw identities and condition IDs

The official `ConditionIdEnum` provides the following IDs. Raw `conditions` entries have `{id, level}`; the condition utilities call these `getContractId` and `getContractLevel`. Standard condition add actions default `level` to JSON null. [O1]

| ID | Condition | Level |
| --- | --- | --- |
| 1 | Blinded | null |
| 2 | Charmed | null |
| 3 | Deafened | null |
| 4 | Exhaustion | 1–6 |
| 5 | Frightened | null |
| 6 | Grappled | null |
| 7 | Incapacitated | null |
| 8 | Invisible | null |
| 9 | Paralyzed | null |
| 10 | Petrified | null |
| 11 | Poisoned | null |
| 12 | Prone | null |
| 13 | Restrained | null |
| 14 | Stunned | null |
| 15 | Unconscious | null |

Exhaustion levels 1–6 are established in both official rules editions; level 6 means death, and level 0 means the condition ends. The official sheet builds its level controls from `ConditionUtils.getDefinitionLevels`, rather than a local constant. Its handler removes a special condition when the selected value is null. Therefore, represent no exhaustion by removing condition 4, not by writing an arbitrary standard condition or retaining an active level-zero entry. The 2014 and 2024 effects differ; this table specifies identity and level storage only. [O1, O4, O5]

Examples (including the required calculated HP value):

```json
{"characterId":123,"id":11,"level":null,"totalHp":30}
{"characterId":123,"id":4,"level":2,"totalHp":30}
```

Removing either condition uses DELETE with `{characterId, id}`. Condition add/remove can have other sheet effects, so refresh the complete raw record after the mutation. [O1]

Exact resource extraction from a raw character:

- Action resource: `actions.<group>[i].id`, `.entityTypeId`, `.limitedUse.numberUsed` (groups include class, race, feat, item, background). The official `ActionUtils` mapping getters read `.id` and `.entityTypeId` directly. Do not use `.componentId` or `.componentTypeId` for this payload. [O1]
- Spell resource: the spell **mapping** record's `.id`, `.entityTypeId`, and `.limitedUse.numberUsed`, whether it comes from a source spell list or `classSpells[].spells[]`. `SpellUtils.getMappingId` returns `.id`, and `getMappingEntityTypeId` returns `.entityTypeId`; `getId` separately returns `.definition.id`. The limited-use UI/dispatcher uses mapping IDs, not the spell definition ID. [O1]
- Both action and spell updates write `{characterId, id, entityTypeId, uses}`. `uses` corresponds to `limitedUse.numberUsed`. Preserve these identities in the local resource model; a label is not sufficient. [O1]
- Raw death saves use `deathSaves.successCount`, `deathSaves.failCount`, and `deathSaves.isStabilized`. The verified mutation controls the first two fields only; do not invent an `isStabilized` write parameter. The derived display defaults missing counts to zero, but that is not evidence that absent raw counts are safe to overwrite. [O1]
- Ordinary/pact slot records use `level` and `used`. The official reducer matches by `level` and writes `used`. These fields differ from the mutation body's `level1` ... `level9`/`level5`. [O1: `function Jy(`]

## Maximum HP caveat

Do not treat raw `baseHitPoints` as maximum HP. The inspected first-party `deriveHitPointInfo` implementation starts from base HP (or fixed class hit-die values when that preference is selected), adds Constitution times total level, global and class-specific HP-per-level modifiers, other HP modifiers, and `bonusHitPoints`. A non-null `overrideHitPoints` replaces that total; a rules-data minimum clamps the result. It then clamps removed HP to total HP for the displayed remainder. [O1: search `remainingHp:P-B`, inspect the preceding `de` function]

Thus `baseHitPoints + bonusHitPoints` is also insufficient. Use a fresh computed maximum for a current-HP-to-removed-HP conversion, or retain raw removed HP when editing only temporary HP. Do not silently convert an encounter's overridden maximum into the character's maximum. A stale computed maximum can produce an incorrect `removedHitPoints` even if the raw HP fields were freshly read; require consistent raw/computed observations or defer an edit when they do not agree. After a condition or other change that can affect maximum HP, fetch the resulting sheet state before the next dependent HP write. These are application recommendations based on the derived-value boundary, not server concurrency guarantees.

Additional official rules sources:

- [O4: 2024 Free Rules, Exhaustion](https://www.dndbeyond.com/sources/dnd/free-rules/rules-glossary#ExhaustionCondition): cumulative levels; death at six; ends at zero; per-level D20 and Speed penalties.
- [O5: 2014 Basic Rules, Exhaustion](https://www.dndbeyond.com/sources/dnd/basic-rules-2014/appendix-a-conditions#Exhaustion): six-level table, including maximum HP halved at level four and death at level six.

## Null counters are zero, not unavailable

Follow-up source check: explicit JSON null values for these counters are compatible with the official sheet's zero-state behavior. They need not disable combat controls. This refines the absence warning above: **explicit null and a missing/wrong-type field are different cases**. [O1]

- `getRemovedHp` returns raw `character.removedHitPoints`. The HP derivation then executes `B = Math.min(P, i)`, where `P` is calculated maximum and `i` is removed HP. JavaScript converts null to zero here, so null removed HP means no damage. Missing/undefined instead produces NaN; do not generalize this evidence to every absent or malformed input.
- `getTempHp` returns raw `character.temporaryHitPoints`. The official damage calculation uses `const i = e.tempHp ? e.tempHp : 0`, and the character mobile summary explicitly uses `null === e.tempHp ? 0 : e.tempHp`. Thus null temporary HP means no temporary HP. Some display state retains null, but combat arithmetic treats it as zero.
- The official death-save derivation is `failCount: null !== e && null !== e.failCount ? e.failCount : 0`, with the same expression for `successCount`. Both a null `deathSaves` object and explicit null counts normalize to zero. This expression does not deliberately normalize a missing count: undefined passes its null check.

Application recommendation: normalize explicit null to zero for `removedHitPoints`, `temporaryHitPoints`, and both death-save counts; a null death-save object means both counts are zero. Compare normalized values for conflict detection and read-back, so a server response of null matches a requested zero. Preserve strict validation for missing fields, wrong types, negative/fractional counters, and death-save counts above three. Outbound requests can use integer zero, which is already used by the verified mutation clients. [O1, A1, A3]

Exact bundle search locators: `xs=e=>e.character.removedHitPoints`, `ws=e=>e.character.temporaryHitPoints`, `remainingHp:P-B`, `const i=e.tempHp?e.tempHp:0`, `null===e.tempHp?0:e.tempHp`, and `function H(e,t){return{failCount`.

## More sheet values and sheet actions

Research date: 2026-11-10. Source: the official character bundle `main.b9f1c122.js`, and the public `GET /character/v5/rule-data`. This is source inspection. No live write was made. Splat reads every write back, as above.

Absolute values. Each body also has `characterId`. Splat sends these from `src/combat_sync.rs`:

| Value | Method and route | Body fields | Raw read field |
| --- | --- | --- | --- |
| Inspiration | PUT `/character/inspiration` | `inspiration` (bool) | `inspiration` |
| Maximum HP modifier | PUT `/life/hp/bonus` | `bonusHitPoints` (signed) | `bonusHitPoints`, null is 0 |
| One coin | PUT `/inventory/currency/{copper,silver,electrum,gold,platinum}` | `amount`, `destinationEntityId` (character ID), `destinationEntityTypeId` 1581111423 | `currencies.{cp,sp,ep,gp,pp}` |
| Item quantity | PUT `/inventory/item/quantity` | `id` (inventory mapping ID), `quantity` | `inventory[].quantity` |
| Item charges spent | PUT `/inventory/item/charge` | `id`, `charges` | `inventory[].limitedUse.numberUsed` |
| Companion HP | PUT `/creature/hp` | `id` (creature mapping ID), `removedHitPoints`, `temporaryHitPoints` | `creatures[]` |

The coin route's destination is the official sheet's character container key: entity type 1581111423 (`ContainerTypeEnum.CHARACTER`) and the character ID. Coins in other containers and the party inventory use other destinations. Splat does not write those. The item charge body's `charges` is the official action's `uses`, which is charges spent. A companion's maximum HP is computed by the sheet. `definition.averageHitPoints` is only the stat block's average, so Splat edits damage taken and uses the average only as a limit and a note.

Sheet actions. D&D Beyond applies these itself, so sending one twice applies it twice. Splat sends one only on the DM's request, between two reads, and never retries it:

| Action | Method and route | Body fields |
| --- | --- | --- |
| Short rest | POST `/character/rest/short` | `classHitDiceUsed` (object: class mapping ID to total hit dice used after the rest), `resetMaxHpModifier` |
| Long rest | POST `/character/rest/long` | `resetMaxHpModifier`, `adjustConditionLevel` (reduce Exhaustion) |
| Restore to life | PUT `/life/restore` | `totalHp` (computed maximum HP), `restoreType` |

`classHitDiceUsed` is absolute: the official dialog starts from each class's `hitDiceUsed` and adds the dice spent. The short rest body has no HP field, so it does not add the HP the dice give. Rule data lists `restoreTypes`: 1 is `Full`, 2 is `OneHP`.

D&D Beyond has no route for concentration, and the condition route accepts only the 15 standard IDs. Concentration and other effects stay in Splat.
