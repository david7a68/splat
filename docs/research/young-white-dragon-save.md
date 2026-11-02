# Young White Dragon: unsigned Intelligence save

**Replaced on 2026-11-01.** The user now rules that the unsigned `2` is a printing
error, so the save is −2. The rest of this note records the earlier +2 ruling.

Checked online on 2026-09-15.

Use **+2 as the complete base Intelligence saving-throw modifier** for the
SRD 5.2.1 Young White Dragon, as directed by the user. Its Intelligence score
remains 6 and its ability modifier remains −2. The save is `d20 + 2`, followed
by applicable additional modifiers. Do not add the ability modifier or
proficiency bonus again, and do not replace the d20 result with 2.

The online sources do not agree. There is direct evidence of another project
using +2, but two established structured implementations use −2. This is not
evidence of a community consensus or an official correction.

## Publisher evidence

- The [English SRD 5.2.1, page 340][srd] prints `Int 6 −2 2` under the ability
  score, MOD, and SAVE columns. The SAVE cell has neither a plus nor a minus
  sign. A fresh download has SHA-256
  `8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87`,
  which matches the bundled source PDF.
- The same document's page 255, “Ability Scores,” identifies these cells as
  ability modifiers and saving-throw modifiers. The official
  [conversion guide, “Ability Scores”][guide] also says all monster saving-throw
  modifiers are now listed beside ability modifiers. Thus the SAVE cell is a
  modifier to a roll, not a fixed roll result.
- The [French SRD 5.2.1, printed page 286][french], also prints
  `Int 6 −2 2` for “Jeune dragon blanc.” This confirms another publisher edition
  retains the unsigned value. It does not explain the intended sign.

The usual calculation for Intelligence 6 without saving-throw proficiency is
−2. The stat block has PB +3, so adding proficiency once would produce +1,
not +2. This arithmetic makes the printed cell unusual; it does not authorize
silently changing the source value.

## How other implementations handle it

| Source | Observed handling | What this establishes |
| --- | --- | --- |
| [Kinetic Vanguard maintainer comment][kinetic] | “Young White Dragon Intelligence prints modifier −2 and final save +2.” The owner lists it under “Source-printed exceptions are preserved, not silently corrected.” | An explicit interpretation that agrees with the user's ruling. The comment is dated 2026-08-11 and cites the same PDF SHA-256. |
| [Roll20 D&D 2024 compendium][roll20] | Displays `INT 6 (-2)` and `Saving Throws Str +4, Dex +3, Con +4, Int 2, Wis +3, Cha +1`. | Retains the unsigned printed save. The page alone does not prove which modifier its character-sheet automation uses. |
| [Open5e SRD-2024 source record][open5e-source] | `ability_score_intelligence: 6` and `saving_throw_intelligence: -2`. The [live API][open5e-api] also returns Intelligence −2 in both `saving_throws` and `saving_throws_all`. | Explicitly records −2. No explanation of this choice was found in the inspected record. |
| [Foundry dnd5e 2024 actor data][foundry-data] | Intelligence `value: 6`, `proficient: 0`, and an empty save bonus. Its [ability preparation code][foundry-code] computes the save from the ability modifier, bonus, and proficiency. | Produces a base save of −2 under the normal calculation. The record does not explain the unsigned SRD cell. |

The Open5e source was read at commit
`4b314adb19b52ae6caf705f6620311d90ed10a74`; Foundry was read at commit
`8c10c1a180cac7d292867c920fc9c13277a48670`. Both files were fetched from
upstream during this check. The Roll20 page, API response, and maintainer
comment are live sources and may change.

## Conversion decision

Preserve the raw SAVE text `2` in source provenance and encode numeric **2** as
the supplied save-total override. Mark the sign interpretation as a resolved
project ruling. The value is no longer a conversion blocker. Keep the differing
external values in this note so future updates cannot silently replace the
chosen interpretation with a recalculated −2.

No official erratum for this cell was found in the publisher materials checked.
The evidence supports keeping the distinction between what the PDF prints,
what the default calculation produces, and what this project chooses to run.

[srd]: https://media.dndbeyond.com/compendium-images/srd/5.2/SRD_CC_v5.2.1.pdf#page=340
[guide]: https://media.dndbeyond.com/compendium-images/srd/guide/converting-to-srd-5.2.1.pdf
[french]: https://media.dndbeyond.com/compendium-images/srd/5.2/FR_SRD_CC_v5.2.1.pdf#page=286
[kinetic]: https://github.com/kmart01123/kinetic-vanguard/issues/55#issuecomment-5251754933
[roll20]: https://roll20.net/compendium/dnd5e/Monsters:Young%20White%20Dragon?expansion=33335
[open5e-source]: https://github.com/open5e/open5e-api/blob/4b314adb19b52ae6caf705f6620311d90ed10a74/data/v2/wizards-of-the-coast/srd-2024/Creature.json
[open5e-api]: https://api.open5e.com/v2/creatures/?name=Young%20White%20Dragon
[foundry-data]: https://github.com/foundryvtt/dnd5e/blob/8c10c1a180cac7d292867c920fc9c13277a48670/packs/_source/actors24/dragon/young-white-dragon.yml
[foundry-code]: https://github.com/foundryvtt/dnd5e/blob/8c10c1a180cac7d292867c920fc9c13277a48670/module/data/actor/templates/common.mjs#L357-L365
